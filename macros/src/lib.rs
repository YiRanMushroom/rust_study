use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn print_ident(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);

    let expanded = quote! {
        println!("The identifier is: {}", stringify!(#input));
    };

    TokenStream::from(expanded)
}