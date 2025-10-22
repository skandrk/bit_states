pub mod atomic_bit_state;
pub mod bit_state;

pub use bit_state::BitState;

#[cfg(test)]
mod tests {
    use crate::bit_state::BitState;
    use std::sync::atomic::AtomicU8;

    #[test]
    fn it_works() {
        let mut result = 0u8;
        result.set_bit(1);
        assert_eq!(result, 0b_0000_0010);
        result.set_bit(3);
        assert_eq!(result, 0b_0000_1010);
    }

    #[test]
    fn atomic_u8() {
        let mut result = AtomicU8::new(0);
        result.set_bit(2);
        assert_eq!(
            result.load(std::sync::atomic::Ordering::Acquire),
            0b_0000_0100
        );
        result.set_bit(3);
        result.clear_bit(2);
        assert_eq!(
            result.load(std::sync::atomic::Ordering::Acquire),
            0b_0000_1000
        );
    }
}
