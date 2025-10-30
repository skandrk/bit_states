#[cfg(feature = "non-atomic")]
pub mod bit_state;
#[cfg(feature = "non-atomic")]
pub use bit_state::BitState;

#[cfg(feature = "atomic")]
pub mod atomic_bit_state;
#[cfg(feature = "atomic")]
pub use atomic_bit_state::AtomicBitState;

#[cfg(feature = "bit-watcher")]
pub mod bit_state_watcher;
