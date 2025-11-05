//!  #BitStates
//!
//!  Reactive bit state management with event callbacks for Rust.

#[cfg(feature = "atomic")]
mod atomic_bit_state;

mod bit_state;

use proc_macro::TokenStream;

#[cfg(feature = "non-atomic")]
#[proc_macro_derive(BitStates)]
pub fn bit_flag_derive(input: TokenStream) -> TokenStream {
    bit_state::derive(input)
}

#[cfg(feature = "atomic")]
#[proc_macro_derive(AtomicBitStates)]
pub fn atomic_bit_flag_derive(input: TokenStream) -> TokenStream {
    atomic_bit_state::derive(input)
}

#[cfg(not(feature = "atomic"))]
#[proc_macro_derive(AtomicBitState)]
pub fn atomic_bit_state_derive(_input: TokenStream) -> TokenStream {
    // Return a compile error if someone tries to use it without the feature
    syn::Error::new(
        proc_macro2::Span::call_site(),
        "AtomicBitState requires the `atomic` feature to be enabled.\n\
         Add `features = [\"atomic\"]` to your dependency.",
    )
    .to_compile_error()
    .into()
}
