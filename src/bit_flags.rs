use std::collections::HashMap;

use crate::BitState;

pub struct FlagSet<T, V>
where
    T: BitState<T>,
{
    flags: HashMap<u8, V>,
    bit_state: T,
}

macro_rules! impl_flag_sets {
    ($($ty: ident),+) => {
      $(
        impl <V> FlagSet<$ty,V>
        where
          $ty: Default + BitState<$ty>,
          V: Clone,
        {
          pub fn new<const N: usize>(hm: [(u8,V);N]) -> Self {
              Self {
                  flags: HashMap::from(hm),
                  bit_state: $ty::default(),
              }
          }

          pub fn set_with_state<F>(&mut self, new: $ty, up_event: F, down_event: F)
          where
              F: Fn(&V),
          {
              let up_bits = (self.bit_state ^ new) & new;
              let down_bits = (self.bit_state ^ new) & (!new);

              if up_bits == 0 && down_bits == 0 {
                return;
              }

              for (key, value) in &self.flags {
                if up_bits != 0 && ((1 << key) & up_bits) != 0 {
                  up_event(value);
                }
                if down_bits != 0 && ((1 << key) & down_bits) != 0 {
                  down_event(value);
                }
              }

              self.bit_state = new;
          }
       }
      )+
    };
}

#[macro_export]
macro_rules! flag_set {
    ($ty:ident, {$($bit:literal => $value:expr),+}) => {{
      $(
        const _: () = {
          const BIT_MAX: u8 = $ty::MAX_SIZE;
          const FLAG_BIT: u8 = $bit;
          assert!(FLAG_BIT < BIT_MAX);
        };
      )+

      FlagSet::<$ty,_>::new([
        $(($bit, $value)),+
      ])
    }};
}

impl_flag_sets!(u8, u16, u32, u64, u128);
