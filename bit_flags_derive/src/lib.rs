use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(PrintInput)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("Not an Enum"),
    };

    let variant_count = variants.len();

    let variant_names = variants.iter().map(|v| &v.ident);

    let expanded = quote! {
        impl #name {
            pub fn variant_count() -> usize {
                #variant_count
            }

            pub fn variant_names() -> &'static [&'static str] {
                &[#(stringify!(#variant_names)),*]
            }
        }
    };

    TokenStream::from(expanded)
}
