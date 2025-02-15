use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Pair::Punctuated;
use syn::LitInt;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn json_struct(input: DeriveInput) -> TokenStream {
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
            let #field_name = self.#field_name.to_json();
            json[stringify!(#field_name).to_string()] = #field_name;
        }
    });

    let from_json_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: #field_type::from_json(&json[stringify!(#field_name).to_string()]),
        }
    });

    let expanded = quote! {

        impl json::FromAndToJson for #name {
            fn to_json(&self) -> json::JsonNode {
                let mut json = json::JsonNode::Object(std::collections::HashMap::new());
                #(#to_json_fields)*
                json
            }

            fn from_json(json: &json::JsonNode) -> Self {
                Self{#(#from_json_fields)*}
            }
        }
    };

    TokenStream::from(expanded)
}

fn json_tuple(input: DeriveInput) -> TokenStream {
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

    let to_json_fields = fields.iter().enumerate().map(|(idx, field)| {
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

        impl json::FromAndToJson for #name {
            fn to_json(&self) -> json::JsonNode {
                let mut json = json::JsonNode::Array(std::vec::Vec::with_capacity(#fields_len));
                #(#to_json_fields)*
                json
            }

            fn from_json(json: &json::JsonNode) -> Self {
                Self{#(#from_json_fields)*}
            }
        }

    };

    TokenStream::from(expanded)
}

/*
enum TestEnum {
    Variant1(String),
    Variant2,
    Variant3(i32, f64),
}

impl FromAndToJson for TestEnum {
    fn from_json(json: &json::JsonNode) -> Self {
        match &json["type".to_string()] {
            json::JsonNode::String(s) => match s.as_str() {
                "Variant1" => TestEnum::Variant1(String::from_json(&json["v".to_string()])),
                "Variant2" => TestEnum::Variant2,
                "Variant3" => TestEnum::Variant3(
                    i32::from_json(&json["v".to_string()][0]),
                    f64::from_json(&json["v".to_string()][1]),
                ),
                _ => panic!("Invalid variant"),
            },
            _ => panic!("Invalid variant"),
        }
    }

    fn to_json(&self) -> json::JsonNode {
        let mut json = json::JsonNode::Object(std::collections::HashMap::new());
        match self {
            TestEnum::Variant1(p0) => {
                json["type".to_string()] = json::JsonNode::String("Variant1".to_string());
                json["v".to_string()] = p0.to_json();
            }
            TestEnum::Variant2 => {
                json["type".to_string()] = json::JsonNode::String("Variant2".to_string());
            }
            TestEnum::Variant3(v0, v1) => {
                json["type".to_string()] = json::JsonNode::String("Variant3".to_string());
                json["v".to_string()][0] = v0.to_json();
                json["v".to_string()][1] = v1.to_json();
            }
        }
        json
    }
}
*/

fn json_enum(input: DeriveInput) -> TokenStream {
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
                #variant_name_str => #name::#variant_name,
            };
        } else if let Fields::Named(fields_named) = &variant.fields {
            let named_fields_init_quotes = fields_named.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! {
                    #field_name: #field_type::from_json(&json["value".to_string()][stringify!(#field_name).to_string()]),
                }
            });
            return quote! {
                #variant_name_str => #name::#variant_name{#(#named_fields_init_quotes)*},
            };
        } else {
            panic!("Not supported because it is not unit, named or unnamed fields");
        };
        if fields.len() == 0 {
            quote! {
                #variant_name_str => #name::#variant_name,
            }
        } else if fields.len() == 1 {
            let field = &fields[0];
            let field_type = &field.ty;
            quote! {
                #variant_name_str => #name::#variant_name(#field_type::from_json(&json["value".to_string()])),
            }
        } else {
            let field_init_quotes = fields.iter().enumerate().map(|(idx, field)| {
                let field_type = &field.ty;
                let idx_lit = LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
                quote! {
                    #field_type::from_json(&json["value".to_string()][#idx_lit]),
                }
            });
            quote! {
                #variant_name_str => #name::#variant_name(#(#field_init_quotes)*),
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
                    json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
                }
            };
        } else if let Fields::Named(fields_named) = &variant.fields {
            // let named_fields_init_quotes = fields_named.named.iter().map(|field| {
            //     let field_name = field.ident.as_ref().unwrap();
            //     quote! {
            //         json["value".to_string()][stringify!(#field_name).to_string()] = #field_name.to_json();
            //     }
            // });
            // quote! {
            //     #name::#variant_name(#(#idx_tokens)*) => {
            //         json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
            //         json["value".to_string()] = json::JsonNode::Array(std::vec::Vec::with_capacity(#fields_len));
            //         #(#field_init_quotes)*
            //     }
            // }
            let quote_identifiers = fields_named.named.iter().map(|field| {
                field.ident.as_ref().unwrap()
            });
            let field_init_quotes = fields_named.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    json["value".to_string()][stringify!(#field_name).to_string()] = #field_name.to_json();
                }
            });
            return quote! {
                #name::#variant_name{#(#quote_identifiers),*} => {
                    json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
                    json["value".to_string()] = json::JsonNode::Object(std::collections::HashMap::new());
                    #(#field_init_quotes)*
                }
            };
        }  else {
            panic!("Only unit and unnamed fields are supported");
        };
        if fields.len() == 0 {
            quote! {
                #name::#variant_name => {
                    json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
                }
            }
        } else if fields.len() == 1 {
            quote! {
                #name::#variant_name(p) => {
                    json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
                    json["value".to_string()] = p.to_json();
                }
            }
        } else {
            let idx_tokens = fields.iter().enumerate().map(|(idx, _)| {
                LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
                let var_name = format_ident!("v{}", idx);
                if idx == fields.len() - 1 {
                    quote! {
                        #var_name
                    }
                } else {
                    quote! {
                        #var_name,
                    }
                }
            });
            let field_init_quotes = fields.iter().enumerate().map(|(idx, field)| {
                let var_name = format_ident!("v{}", idx);
                quote! {
                    json["value".to_string()].push(#var_name.to_json());
                }
            });

            let fields_len = fields.len();

            quote! {
                #name::#variant_name(#(#idx_tokens)*) => {
                    json["type".to_string()] = json::JsonNode::String(#variant_name_str.to_string());
                    json["value".to_string()] = json::JsonNode::Array(std::vec::Vec::with_capacity(#fields_len));
                    #(#field_init_quotes)*
                }
            }
        }
    });

    let expanded = quote! {
        impl json::FromAndToJson for #name {
            fn from_json(json: &json::JsonNode) -> Self {
                match &json["type".to_string()] {
                    json::JsonNode::String(s) => match s.as_str() {
                        #(#from_json_variants)*
                        _ => panic!("Invalid variant"),
                    },
                    _ => panic!("Invalid variant"),
                }
            }

            fn to_json(&self) -> json::JsonNode {
                let mut json = json::JsonNode::Object(std::collections::HashMap::new());
                match self {
                    #(#to_json_variants)*
                }
                json
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(JsonType)]
pub fn json_type(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    if let Data::Struct(_) = &input.data {
        return json_struct(input);
    } else if let Data::Enum(_) = &input.data {
        return json_enum(input);
    } else {
        panic!("Only struct and enum are supported");
    }
}
