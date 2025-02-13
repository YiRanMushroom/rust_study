use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(JsonStruct)]
pub fn json_struct(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            unimplemented!()
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

    TokenStream::from(quote! {
        #expanded
    })
}
