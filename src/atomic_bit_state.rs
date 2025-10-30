use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

pub trait AtomicBitState<T, U> {
    const MAX_SIZE: u8 = (std::mem::size_of::<U>() * 8) as u8;
    fn set(&self, new: U);
    fn set_with_changes(&self, new: U) -> Option<(Vec<u8>, Vec<u8>)>;
    fn set_bit(&self, n: u8);
    fn clear_bit(&self, n: u8);
    fn get_bit(&self, n: u8) -> bool;
    fn reset(&self);
}

macro_rules! bit_state_impl_atomics {
  ($($ty:ident : $fr:ident),+) => {
    $(
      impl AtomicBitState<$ty,$fr> for $ty {

        #[inline]
        fn set(&self, new: $fr) {
          self.store(new,Ordering::Relaxed);
        }

        fn set_with_changes(&self, new: $fr) -> Option<(Vec<u8>, Vec<u8>)>{

          let mut old = self.load(Ordering::Relaxed);

          loop{
            if old == new {
              return None;
            }

            match self.compare_exchange_weak(
              old,
              new,
              Ordering::Relaxed,
              Ordering::Relaxed
              ) {
                  Ok(_) => break,
                  Err(x) => old = x, // Retry with updated value
              }
          }

          let mut up_bits = (old ^ new) & new;
          let mut down_bits = (old ^ new) & (!new);
          let mut pos: u8 = 0;

          let mut ups = Vec::new();
          let mut downs = Vec::new();

          while (up_bits != 0 || down_bits !=0) {
            if(up_bits & 1 == 1){
              ups.push(pos);
            }
            if(down_bits & 1 == 1){
              downs.push(pos);
            }
            up_bits >>= 1;
            down_bits >>= 1;
            pos += 1;
          }

          Some((ups,downs))
        }

        #[inline]
        fn set_bit(&self, n: u8) {
          self.fetch_or(1 << n, Ordering::Relaxed);
        }

        #[inline]
        fn clear_bit(&self, n: u8) {
          self.fetch_and(!(1 << n), Ordering::Relaxed);
        }

        #[inline]
        fn get_bit(&self, n: u8) -> bool {
          (self.load(Ordering::Relaxed) & (1 << n)) != 0
        }

        #[inline]
        fn reset(&self) {
          self.store(0, Ordering::Relaxed);
        }
      }
    )+
  };
}

bit_state_impl_atomics!(AtomicU8 : u8, AtomicU16 : u16, AtomicU32 : u32, AtomicU64 : u64);

#[cfg(test)]
mod tests {
    use crate::AtomicBitState;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU8, AtomicU16};

    #[test]
    fn atomic_u8() {
        let result = Arc::new(AtomicU8::new(0));
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

    #[test]
    fn test_set_with_changes() {
        let result = AtomicU16::from(0b_0001_1011_0011_1010 as u16);
        if let Some((ups, downs)) = result.set_with_changes(0b_1001_0101_1100_0110 as u16) {
            assert_eq!(ups, Vec::from([2, 6, 7, 10, 15]));
            assert_eq!(downs, Vec::from([3, 4, 5, 9, 11]));
            assert_eq!(
                result.load(std::sync::atomic::Ordering::Relaxed),
                0b_1001_0101_1100_0110 as u16
            );
        }
    }
}
