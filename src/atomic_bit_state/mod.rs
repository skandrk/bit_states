use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let repr_type = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("repr") {
            attr.parse_args::<syn::Ident>().ok()
        } else {
            None
        }
    });

    match repr_type {
        Some(ty) => {
            if ty.to_string().as_str() != "u8" {
                return syn::Error::new(
                    input.ident.span(),
                    format!("BitStates requires repr to be u8, found '{}'", ty),
                )
                .to_compile_error()
                .into();
            }
        }
        None => {
            return syn::Error::new(input.ident.span(), "BitFlags requires #[repr(u8)]")
                .to_compile_error()
                .into();
        }
    };

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
                None => panic!("All variants/flags must have explicit values that are equal to or greater than Zero! (e.g., Ready = 0)"),
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
          fn from_flagbit(n: u8) ->  Option<Self>{
            match n {
              #(#branch_arms)*
              _ => None
            }
          }

          #[inline]
          pub const fn get_flagbit(&self) -> u8 {
            *self as u8
          }

          pub fn get_flagmask(&self) -> #primitive_type {
            return 1 << self.get_flagbit()
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
              if let Some(flag) = #enum_name::from_flagbit(rightmost_set_bit){
                (self.up_event)(flag);
              };
              up_bits &= up_bits - 1;
            }

            while down_bits != 0 {
              let rightmost_set_bit = down_bits.trailing_zeros() as u8;
              if let Some(flag) = #enum_name::from_flagbit(rightmost_set_bit){
                (self.down_event)(flag);
              };
              down_bits &= down_bits - 1;
            }
          }

          pub fn set_flag(&self, flag: #enum_name) {
            let bit_mask = flag.get_flagmask();
            let old = self.bit_state.fetch_or(bit_mask, core::sync::atomic::Ordering::Release);
            let was_already_set = (old & bit_mask) != 0;
            if !was_already_set {
              (self.up_event)(flag);
            }
          }

          pub fn reset_flag(&self, flag: #enum_name) {
            let bit_mask = flag.get_flagmask();
            let old = self.bit_state.fetch_and(!bit_mask, core::sync::atomic::Ordering::Release);
            let was_already_set = (old & bit_mask) != 0;
            if was_already_set {
              (self.down_event)(flag);
            }
          }

          pub fn get(&self) -> #primitive_type {
            self.bit_state.load(core::sync::atomic::Ordering::Acquire)
          }

          pub fn clear(&self) {
            self.bit_state.store(0, core::sync::atomic::Ordering::Release);
          }

          pub fn is_set(&self, flag: #enum_name) -> bool {
            let current = self.bit_state.load(core::sync::atomic::Ordering::Acquire);
            (current & flag.get_flagmask()) != 0
          }
        }
    };
    TokenStream::from(expanded)
}
