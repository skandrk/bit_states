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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple() {
        #[derive(Debug, Clone, Copy, PrintInput)]
        #[repr(u8)]
        enum Status {
            Ready = 0,
            Active = 2,
        }

        let s = Status::Active;
        println!("Bit: {}", s.bit_position()); // 2

        let from_bit = Status::from_bit(0);
        println!("{:?}", from_bit); // Some(Complete)
    }
}
