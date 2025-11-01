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

    let variant_data: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let discriminant = match &v.discriminant {
                Some((_, expr)) => expr,
                None => panic!("All variants must have explicit values! (e.g., Ready = 0)"),
            };
            (variant_name, discriminant)
        })
        .collect();

    let from_bit_arms = variant_data.iter().map(|(variant_name, discriminant)| {
        quote! {
            #discriminant => Some(#name::#variant_name),
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn bit_position(&self) -> u8 {
                *self as u8
            }

            pub fn from_bit(bit: u8) -> Option<Self> {
                match bit {
                    #(#from_bit_arms)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
