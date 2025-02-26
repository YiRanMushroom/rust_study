use proc_macro::TokenStream;
use proc_macro_essentials::syn::parse::{Parse, ParseStream};
use proc_macro_essentials::{proc_macro2, quote, syn};
use quote::{format_ident, quote};

use syn::LitInt;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use proc_macro_essentials::utils::get_call_site_crate_name;

fn json_struct(input: DeriveInput) -> proc_macro2::TokenStream {
    let crate_name = get_call_site_crate_name("json");
    let name = &input.ident;
    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            return json_tuple(input);
        }
    } else {
        unimplemented!()
    };

    let to_json_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            json[stringify!(#field_name)] = self.#field_name.to_json();
        }
    });

    let from_json_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: #field_type::from_json(&json[stringify!(#field_name)]),
        }
    });

    let expanded: proc_macro2::TokenStream = quote! {
        impl #crate_name::FromAndToJson for #name {
            fn to_json(&self) -> #crate_name::JsonNode {
                let mut json = #crate_name::JsonNode::Object(std::collections::HashMap::new());
                #(#to_json_fields)*
                json
            }

            fn from_json(json: &#crate_name::JsonNode) -> Self {
                Self{#(#from_json_fields)*}
            }
        }
    };

    proc_macro2::TokenStream::from(expanded)
}

fn json_tuple(input: DeriveInput) -> proc_macro2::TokenStream {
    let crate_name = get_call_site_crate_name("json");
    let name = &input.ident;
    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            &fields_unnamed.unnamed
        } else {
            return json_struct(input);
        }
    } else {
        unimplemented!()
    };

    let to_json_fields = fields.iter().enumerate().map(|(idx, _)| {
        let idx_lit = LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
        quote! {
            json.push(self.#idx_lit.to_json());
        }
    });

    let from_json_fields = fields.iter().enumerate().map(|(idx, field)| {
        let field_type = &field.ty;
        let idx_lit = LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
        quote! {
            #idx_lit : #field_type::from_json(&json[#idx_lit]),
        }
    });

    let fields_len = fields.len();

    let expanded = quote! {
        impl #crate_name::FromAndToJson for #name {
            fn to_json(&self) -> #crate_name::JsonNode {
                let mut json = #crate_name::JsonNode::Array(std::vec::Vec::with_capacity(#fields_len));
                #(#to_json_fields)*
                json
            }

            fn from_json(json: &#crate_name::JsonNode) -> Self {
                Self{#(#from_json_fields)*}
            }
        }

    };

    proc_macro2::TokenStream::from(expanded)
}

fn json_enum(input: DeriveInput) -> proc_macro2::TokenStream {
    let crate_name = get_call_site_crate_name("json");
    let name = &input.ident;
    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("Only enum is supported");
    };

    let from_json_variants = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();

        let fields = if let Fields::Unnamed(fields_unnamed) = &variant.fields {
            &fields_unnamed.unnamed
        } else if let Fields::Unit = &variant.fields {
            return quote! {
                #variant_name_str => {#name::#variant_name}
            };
        } else if let Fields::Named(fields_named) = &variant.fields {
            let named_fields_init_quotes = fields_named.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! {
                    #field_name: #field_type::from_json(&json["value"][stringify!(#field_name)])
                }
            });
            return quote! {
                #variant_name_str => {#name::#variant_name{#(#named_fields_init_quotes),*}}
            };
        } else {
            panic!("Not supported because it is not unit, named or unnamed fields");
        };

        if fields.len() == 0 {
            quote! {
                #variant_name_str => {#name::#variant_name()}
            }
        } else if fields.len() == 1 {
            let field = &fields[0];
            let field_type = &field.ty;
            quote! {
                #variant_name_str => {#name::#variant_name(#field_type::from_json(&json["value"]))}
            }
        } else {
            let field_init_quotes = fields.iter().enumerate().map(|(idx, field)| {
                let field_type = &field.ty;
                let idx_lit = LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
                quote! {
                    #field_type::from_json(&json["value"][#idx_lit])
                }
            });
            quote! {
                #variant_name_str => {#name::#variant_name(#(#field_init_quotes),*)}
            }
        }
    });

    let to_json_variants = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();
        let fields = if let Fields::Unnamed(fields_unnamed) = &variant.fields {
            &fields_unnamed.unnamed
        } else if let Fields::Unit = &variant.fields {
            return quote! {
                #name::#variant_name => {
                    json["type"] = #crate_name::JsonNode::String(#variant_name_str.to_string());
                }
            };
        } else if let Fields::Named(fields_named) = &variant.fields {
            let quote_identifiers = fields_named.named.iter().map(|field| {
                field.ident.as_ref().unwrap()
            });
            let field_init_quotes = fields_named.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    json["value"][stringify!(#field_name)] = #field_name.to_json();
                }
            });
            return quote! {
                #name::#variant_name{#(#quote_identifiers),*} => {
                    json["type"] = #crate_name::JsonNode::String(#variant_name_str.to_string());
                    json["value"] = #crate_name::JsonNode::Object(std::collections::HashMap::new());
                    #(#field_init_quotes)*
                }
            };
        }  else {
            panic!("Only unit and unnamed fields are supported");
        };

        if fields.len() == 0 {
            quote! {
                #name::#variant_name() => {
                    json["type"] = #crate_name::JsonNode::String(#variant_name_str.to_string());
                }
            }
        } else if fields.len() == 1 {
            quote! {
                #name::#variant_name(v) => {
                    json["type"] = #crate_name::JsonNode::String(#variant_name_str.to_string());
                    json["value"] = v.to_json();
                }
            }
        } else {
            let idx_tokens = fields.iter().enumerate().map(|(idx, _)| {
                let var_name = format_ident!("v{}", idx);
                quote! {
                    #var_name
                }
            });
            let field_init_quotes = fields.iter().enumerate().map(|(idx, _)| {
                let var_name = format_ident!("v{}", idx);
                quote! {
                    json["value"].push(#var_name.to_json());
                }
            });

            let fields_len = fields.len();

            quote! {
                #name::#variant_name(#(#idx_tokens), *) => {
                    json["type"] = #crate_name::JsonNode::String(#variant_name_str.to_string());
                    json["value"] = #crate_name::JsonNode::Array(std::vec::Vec::with_capacity(#fields_len));
                    #(#field_init_quotes)*
                }
            }
        }
    });

    let expanded = quote! {
        impl #crate_name::FromAndToJson for #name {
            fn from_json(json: &#crate_name::JsonNode) -> Self {
                match &json["type"] {
                    #crate_name::JsonNode::String(s) => match s.as_str() {
                        #(#from_json_variants)*
                        _ => panic!("Invalid variant")
                    }
                    _ => panic!("Invalid variant")
                }
            }

            fn to_json(&self) -> #crate_name::JsonNode {
                let mut json = #crate_name::JsonNode::Object(std::collections::HashMap::new());
                match self {
                    #(#to_json_variants)*
                }
                json
            }
        }
    };

    proc_macro2::TokenStream::from(expanded)
}

#[proc_macro_derive(JsonType)]
pub fn json_type(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    if let Data::Struct(_) = &input.data {
        proc_macro::TokenStream::from(json_struct(input))
    } else if let Data::Enum(_) = &input.data {
        proc_macro::TokenStream::from(json_enum(input))
    } else {
        panic!("Unions are unsafe, please use enum instead")
    }
}

enum MacroJsonNode {
    Object(std::collections::HashMap<String, MacroJsonNode>),
    Array(std::vec::Vec<MacroJsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Parse for MacroJsonNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let mut obj = std::collections::HashMap::new();
            while !content.is_empty() {
                let key: syn::LitStr = content.parse()?;
                content.parse::<syn::Token![:]>()?;
                let value = MacroJsonNode::parse(&content)?;
                obj.insert(key.value(), value);
                if content.peek(syn::Token![,]) {
                    content.parse::<syn::Token![,]>()?;
                }
            }
            Ok(MacroJsonNode::Object(obj))
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut arr = std::vec::Vec::new();
            while !content.is_empty() {
                let value = MacroJsonNode::parse(&content)?;
                arr.push(value);
                if content.peek(syn::Token![,]) {
                    content.parse::<syn::Token![,]>()?;
                }
            }
            Ok(MacroJsonNode::Array(arr))
        } else if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            Ok(MacroJsonNode::String(lit.value()))
        } else if input.peek(syn::LitFloat) {
            let lit: syn::LitFloat = input.parse()?;
            Ok(MacroJsonNode::Number(lit.base10_parse()?))
        } else if input.peek(syn::LitInt) {
            let lit: syn::LitInt = input.parse()?;
            Ok(MacroJsonNode::Number(lit.base10_parse()?))
        } else if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;
            if ident == "null" {
                Ok(MacroJsonNode::Null)
            } else {
                Err(syn::Error::new(ident.span(), "Invalid identifier"))
            }
        } else if input.peek(syn::LitBool) { 
            let lit: syn::LitBool = input.parse()?;
            Ok(MacroJsonNode::Boolean(lit.value))
        }
        else {
            Err(syn::Error::new(input.span(), "Invalid token"))
        }
    }
}

fn generate_json_call_site(compiler_json: MacroJsonNode) -> proc_macro::TokenStream {
    let crate_name = get_call_site_crate_name("json");

    match compiler_json {
        MacroJsonNode::String(s) => {
            let expanded = quote! {
                #crate_name::JsonNode::String(#s.to_string())
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Number(n) => {
            let number_literal = proc_macro2::Literal::f64_unsuffixed(n);
            let expanded = quote! {
                #crate_name::JsonNode::Number(#number_literal)
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Boolean(b) => {
            let expanded = quote! {
                #crate_name::JsonNode::Boolean(#b)
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Null => {
            let expanded = quote! {
                #crate_name::JsonNode::Null
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Array(arr) => {
            let arr = arr
                .into_iter()
                .map(|node| proc_macro2::TokenStream::from(generate_json_call_site(node)))
                .collect::<Vec<_>>();
            let expanded = quote! {
                #crate_name::JsonNode::Array(std::vec::Vec::from([#(#arr),*]))
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Object(map) => {
            let arr = map
                .into_iter()
                .map(|(key, value)| {
                    let key = proc_macro2::Literal::string(&key);
                    let value = proc_macro2::TokenStream::from(generate_json_call_site(value));
                    quote! {
                        (#key.to_string(), #value)
                    }
                })
                .collect::<Vec<_>>();
            let expanded = quote! {
                #crate_name::JsonNode::Object(std::collections::HashMap::from([#(#arr),*]))
            };
            proc_macro::TokenStream::from(expanded)
        }
    }
}

#[proc_macro]
pub fn json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let compiler_json = parse_macro_input!(input as MacroJsonNode);
    generate_json_call_site(compiler_json)
}
