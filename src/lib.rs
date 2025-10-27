pub mod atomic_bit_state;
pub mod bit_state;

pub use bit_state::BitState;

#[cfg(feature = "atomic")]
pub use atomic_bit_state::AtomicBitState;
