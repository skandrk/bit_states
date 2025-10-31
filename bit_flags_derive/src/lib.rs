use proc_macro::TokenStream;
//use quote::{format_ident, quote};
//use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(_: TokenStream) -> TokenStream {
    // For now, just return empty - doesn't generate any code
    TokenStream::new()
}
