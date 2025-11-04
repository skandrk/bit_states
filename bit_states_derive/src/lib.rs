mod atomic_bit_state;
mod bit_state;

use proc_macro::TokenStream;

#[proc_macro_derive(BitStates)]
pub fn bit_flag_derive(input: TokenStream) -> TokenStream {
    bit_state::derive(input)
}

#[proc_macro_derive(AtomicBitStates)]
pub fn atomic_bit_flag_derive(input: TokenStream) -> TokenStream {
    atomic_bit_state::derive(input)
}
