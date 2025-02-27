use proc_macro_essentials::proc_macro2::Ident;
use proc_macro_essentials::syn::parse::{Parse, ParseStream};
use proc_macro_essentials::{proc_macro2, quote, syn};
use quote::{format_ident, quote};
use std::fmt::format;
use proc_macro_essentials::quote::ToTokens;
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

#[derive(Debug)]
enum StringLiteralOrTokenStream {
    StringLiteral(String),
    TokenStream(proc_macro2::TokenStream),
}

enum MacroJsonNode {
    Object(std::vec::Vec<(StringLiteralOrTokenStream, MacroJsonNode)>),
    Array(std::vec::Vec<MacroJsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    TokenStream(proc_macro2::TokenStream),
    Null,
}

impl Parse for MacroJsonNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let mut obj = std::vec::Vec::new();
            while !content.is_empty() {
                let key;
                if content.peek(syn::LitStr) {
                    key = StringLiteralOrTokenStream::StringLiteral(
                        content.parse::<syn::LitStr>()?.value(),
                    );
                } else if content.peek(syn::Token![#]) {
                    content.parse::<syn::Token![#]>()?;

                    let inner_content;

                    if content.peek(syn::token::Paren) {
                        syn::parenthesized!(inner_content in content);
                        let token_stream: proc_macro2::TokenStream = inner_content.parse()?;
                        key = StringLiteralOrTokenStream::TokenStream(token_stream);
                    } else if content.peek(syn::Ident) {
                        let ident: Ident = content.parse()?;
                        key = StringLiteralOrTokenStream::TokenStream(quote! {#ident});
                    } else {
                        return Err(syn::Error::new(
                            content.span(),
                            "identifier after # is not valid",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(content.span(), "Invalid key"));
                }

                if !content.peek(syn::Token![:]) {
                    return Err(syn::Error::new(content.span(), "Expects a colon"));
                }

                content.parse::<syn::Token![:]>()?;
                let value = MacroJsonNode::parse(&content)?;
                obj.push((key, value));

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
        } else if input.peek(syn::Token![#]) {
            // could be a single token or a token stream with ()
            input.parse::<syn::Token![#]>()?;

            let content;
            if input.peek(syn::token::Paren) {
                syn::parenthesized!(content in input);
                let token_stream = content.parse()?;
                Ok(MacroJsonNode::TokenStream(token_stream))
            } else if input.peek(syn::Ident) {
                let ident: Ident = input.parse()?;
                Ok(MacroJsonNode::TokenStream(quote! {#ident}))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Identifier after # is not valid",
                ))
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("Invalid token at: {:?}", input),
            ))
        }
    }
}

fn generate_json_call_site(compiler_json: MacroJsonNode) -> proc_macro::TokenStream {
    let crate_name = get_call_site_crate_name("json");

    match compiler_json {
        MacroJsonNode::String(s) => {
            let expanded = quote! {
                #s.to_json()
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Number(n) => {
            let number_literal = proc_macro2::Literal::f64_unsuffixed(n);
            let expanded = quote! {
                #number_literal.to_json()
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Boolean(b) => {
            let expanded = quote! {
                #b.to_json()
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
                std::vec::Vec::from([#(#arr),*]).to_json()
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::Object(map) => {
            let arr = map
                .into_iter()
                .map(|(key, value)| {
                    let key_token_stream = match key {
                        StringLiteralOrTokenStream::StringLiteral(s) => {
                            let key = proc_macro2::Literal::string(&s);
                            quote! {
                                #key.to_string()
                            }
                        }
                        StringLiteralOrTokenStream::TokenStream(token_stream) => {
                            quote! {
                                #token_stream.to_string()
                            }
                        }
                    };
                    let value = proc_macro2::TokenStream::from(generate_json_call_site(value));
                    quote! {
                        (#key_token_stream, #value)
                    }
                })
                .collect::<Vec<_>>();
            let expanded = quote! {
                std::collections::HashMap::from([#(#arr),*]).to_json()
            };
            proc_macro::TokenStream::from(expanded)
        }
        MacroJsonNode::TokenStream(token_stream) => {
            let expanded = quote! {
                (#token_stream).to_json()
            };
            proc_macro::TokenStream::from(expanded)
        }
    }
}

fn json_parse_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let compiler_json = syn::parse_macro_input!(input as MacroJsonNode);
    let result = proc_macro2::TokenStream::from(generate_json_call_site(compiler_json));
    let crate_name = get_call_site_crate_name("json");
    proc_macro::TokenStream::from(quote! {
        {
            use #crate_name::FromAndToJson;
            #result
        }
    })
}

#[proc_macro]
pub fn json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    json_parse_impl(input)
}

#[proc_macro]
pub fn json_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // add {} to the input
    let input_tokens = proc_macro2::TokenStream::from(input);
    let expanded = quote! {
        {
            #input_tokens
        }
    };
    json_parse_impl(proc_macro::TokenStream::from(expanded))
}

#[proc_macro]
pub fn json_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // add [] to the input
    let input_tokens = proc_macro2::TokenStream::from(input);
    let expanded = quote! {
        [
            #input_tokens
        ]
    };
    json_parse_impl(proc_macro::TokenStream::from(expanded))
}