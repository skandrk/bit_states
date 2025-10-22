use crate::BitState;
use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

macro_rules! bit_state_impl_atomics {
  ($($ty:ident : $fr:ident),+) => {
    $(
      impl BitState<$ty,$fr> for $ty{

        fn set(&mut self, new: $fr){
          self.swap(new,Ordering::SeqCst);
        }

        #[inline]
        fn set_bit(&mut self, n: u8) {
          self.fetch_or(1 << n, Ordering::SeqCst);
        }

        #[inline]
        fn clear_bit(&mut self, n: u8) {
          self.fetch_and(!(1 << n), Ordering::SeqCst);
        }

        #[inline]
        fn get_bit(&self, n: u8) -> bool {
          (self.load(Ordering::SeqCst) & (1 << n)) != 0
        }

        #[inline]
        fn reset(&mut self) {
          self.store(0, Ordering::SeqCst);
        }

      }
    )+
  };
}

bit_state_impl_atomics!(AtomicU8 : u8, AtomicU16 : u16, AtomicU32 : u32, AtomicU64 : u64);
