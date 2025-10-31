use proc_macro::TokenStream;
//use quote::{format_ident, quote};
//use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(PrintInput)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    println!("Received: {}", input.to_string());

    TokenStream::new()
}
