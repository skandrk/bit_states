use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let struct_name = format_ident!("{}AtomicStates", enum_name);

    let enums = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("Not an Enum"),
    };

    let enum_data: Vec<_> = enums
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let variant_value = match &v.discriminant {
                Some((_, expr)) => expr,
                None => panic!("All variants/flags must have explicit values that are greater than Zero! (e.g., Ready = 1)"),
            };
            (variant_name, variant_value)
        })
        .collect();

    let branch_arms = enum_data.iter().map(|(vn, vv)| {
        quote! {
          #vv => Some(#enum_name::#vn),
        }
    });

    let max_branch_value = enum_data
        .iter()
        .filter_map(|x| {
            if let syn::Expr::Lit(el) = x.1 {
                if let syn::Lit::Int(lit_int) = &el.lit {
                    return lit_int.base10_parse::<u8>().ok();
                }
            }
            None
        })
        .max()
        .unwrap_or(0);

    let (atomic_type, primitive_type) = match max_branch_value {
        0..=7 => (quote! { AtomicU8 }, quote! { u8 }),
        8..=15 => (quote! { AtomicU16 }, quote! { u16 }),
        16..=31 => (quote! { AtomicU32 }, quote! { u32 }),
        32..=63 => (quote! { AtomicU64 }, quote! { u64 }),

        n => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Bit position {} is too large (max 63)", n),
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {

        impl #enum_name {
          fn from_flagbit_atomic(n: u8) ->  Option<Self>{
            match n {
              #(#branch_arms)*
              _ => None
            }
          }
        }

        pub struct #struct_name<Fup, Fdown>
        where
          Fup: Fn(#enum_name),
          Fdown: Fn(#enum_name)
        {
          bit_state: core::sync::atomic::#atomic_type,
          up_event: Fup,
          down_event: Fdown
        }

        impl <Fup, Fdown> #struct_name<Fup, Fdown>
        where
          Fup: Fn(#enum_name) + Send + Sync,
          Fdown: Fn(#enum_name) + Send + Sync
        {
          pub fn new(up_event: Fup, down_event: Fdown) -> Self {
            Self {
              bit_state: core::sync::atomic::#atomic_type::new(0),
              up_event,
              down_event
            }
          }

          pub fn set(&self, new: #primitive_type ) {

            let old = self.bit_state.swap(new, core::sync::atomic::Ordering::AcqRel);

            let mut up_bits = (old ^ new) & new;
            let mut down_bits = (old ^ new) & (!new);

            if up_bits == 0 && down_bits == 0 {
              return;
            }

            while up_bits != 0 {
              let rightmost_set_bit = up_bits.trailing_zeros() as u8;
              if let Some(flag) = #enum_name::from_flagbit_atomic(rightmost_set_bit){
                (self.up_event)(flag);
              };
              up_bits &= up_bits - 1;
            }

            while down_bits != 0 {
              let rightmost_set_bit = down_bits.trailing_zeros() as u8;
              if let Some(flag) = #enum_name::from_flagbit_atomic(rightmost_set_bit){
                (self.down_event)(flag);
              };
              down_bits &= down_bits - 1;
            }
          }
        }
    };
    TokenStream::from(expanded)
}
