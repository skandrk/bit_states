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

use bit_flags_derive::AtomicBitState;
pub use bit_flags_derive::BitState;

#[cfg(test)]
mod tests {

    use crate::*;

    #[derive(Debug, AtomicBitState)]
    #[repr(u8)]
    enum Status {
        Zero = 0,
        Ready = 1,
        Active = 2,
    }

    #[test]
    fn simple() {
        let set_status = StatusAtomicSet::new(
            |a| println!("Up Event {:?}", a),
            |a| println!("Down Event {:?}", a),
        );

        set_status.set_with_state(0b_0000_0001 as u8);
        set_status.set_with_state(0b_0000_0010 as u8);
        set_status.set_with_state(0b_0000_0100 as u8);
    }
}
