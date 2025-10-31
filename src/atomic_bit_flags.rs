use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::AtomicBitState;

pub struct AtomicFlagSet<T, U, V, F1, F2>
where
    T: AtomicBitState<T, U>,
    F1: Fn(&V),
    F2: Fn(&V),
{
    flags: HashMap<u8, V>,
    bit_state: T,
    up_event: F1,
    down_event: F2,
    _phantom: PhantomData<U>,
}

macro_rules! impl_atomic_flag_sets {
    ($($ty: ident: $tu: ident),+) => {
      $(
        impl <V, F1, F2> AtomicFlagSet<$ty,$tu,V, F1, F2>
        where
          $ty: AtomicBitState<$ty,$tu>,
          V: Clone,
          F1: Fn(&V),
          F2: Fn(&V),
        {
          pub fn new<const N: usize>(hm: [(u8,V);N],up_event: F1, down_event: F2) -> Self {
              Self {
                  flags: HashMap::from(hm),
                  bit_state: $ty::default(),
                  up_event,
                  down_event,
                  _phantom: PhantomData
              }
          }

          pub fn set_with_state(&mut self, new: $tu)
          {

            let mut old = self.bit_state.load(Ordering::Acquire);

            loop {
              if old == new {
                return;
              }

              match self.bit_state.compare_exchange_weak(
                old,
                new,
                Ordering::Relaxed,
                Ordering::Relaxed
                ) {
                    Ok(_) => break,
                    Err(x) => old = x, // Retry with updated value
                }
              }

              let up_bits = (old ^ new) & new;
              let down_bits = (old ^ new) & (!new);

              for (key, value) in &self.flags {
                if up_bits != 0 && ((1 << key) & up_bits) != 0 {
                  (self.up_event)(value);
                }
                if down_bits != 0 && ((1 << key) & down_bits) != 0 {
                  (self.down_event)(value);
                }
              }
          }
       }
      )+
    };
}

#[macro_export]
macro_rules! atomic_flag_set {
    ($ty:ident : $tu:ident, {$($bit:literal => $value:expr),+}, $up:expr, $down:expr) => {{
      $(
        const _: () = {
          const BIT_MAX: u8 = $tu::MAX_SIZE;
          const FLAG_BIT: u8 = $bit;
          assert!(FLAG_BIT < BIT_MAX);
        };
      )+

      AtomicFlagSet::<$ty,$tu,_,_,_>::new([
        $(($bit, $value)),+
      ],
      $up,
      $down)
    }};
}

impl_atomic_flag_sets!(AtomicU8 : u8, AtomicU16 : u16, AtomicU32 : u32, AtomicU64 : u64);

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::BitState;
    use crate::atomic_bit_flags::*;
    #[test]
    fn simple() {
        let up_events = RefCell::new(Vec::new());
        let down_events = RefCell::new(Vec::new());
        let mut result = atomic_flag_set!(
            AtomicU8 : u8,
            {
                0 => ("Flag0", "GG"),
                2 => ("Flag2", "GG"),
                4 => ("Flag4", "GG"),
                7 => ("Flag7", "GG"),
                6 => ("Flag6", "GG")
            },
            |a: &(&str, &str)| up_events.borrow_mut().push(a.0),
            |a: &(&str, &str)| down_events.borrow_mut().push(a.0)
        );

        result.set_with_state(0b_0001_0100);
        result.set_with_state(0b_1100_0001);

        let up = up_events.borrow();
        let down = down_events.borrow();

        assert_eq!(up.len(), 5);
        assert_eq!(down.len(), 2);

        assert!(up.contains(&"Flag0"));
        assert!(up.contains(&"Flag2"));
        assert!(up.contains(&"Flag4"));
        assert!(up.contains(&"Flag6"));
        assert!(up.contains(&"Flag7"));

        assert!(down.contains(&"Flag2"));
        assert!(down.contains(&"Flag4"));
    }

    #[test]
    fn test_set_with_changes() {
        let mut result = 0b_0001_1011 as u8;
        if let Some((ups, downs)) = result.set_with_changes(0b_1001_0101 as u8) {
            assert_eq!(ups, Vec::from([2, 7]));
            assert_eq!(downs, Vec::from([1, 3]));
            assert_eq!(result, 0b_1001_0101 as u8);
        }
    }
}
