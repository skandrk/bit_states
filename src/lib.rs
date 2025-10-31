#[cfg(feature = "non-atomic")]
pub use bit_state::BitState;

#[cfg(feature = "non-atomic")]
pub mod bit_state;

#[cfg(feature = "non-atomic")]
pub mod bit_flags;

#[cfg(feature = "atomic")]
pub use atomic_bit_state::AtomicBitState;

#[cfg(feature = "atomic")]
pub mod atomic_bit_state;

#[cfg(feature = "atomic")]
pub mod atomic_bit_flags;

pub use bit_flags_derive::PrintInput;

#[derive(PrintInput)]
enum Status {
    Ready = 0,
    Active = 2,
}
