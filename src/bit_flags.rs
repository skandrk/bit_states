use std::collections::HashMap;

use crate::BitState;

pub struct FlagSet<T, V, F1, F2>
where
    T: BitState<T>,
    F1: Fn(&V),
    F2: Fn(&V),
{
    flags: HashMap<u8, V>,
    bit_state: T,
    up_event: F1,
    down_event: F2,
}

macro_rules! impl_flag_sets {
    ($($ty: ident),+) => {
      $(
        impl <V, F1, F2> FlagSet<$ty,V, F1, F2>
        where
          $ty: Default + BitState<$ty>,
          V: Clone,
          F1: Fn(&V),
          F2: Fn(&V),
        {
          pub fn new<const N: usize>(hm: [(u8,V);N], up_event: F1, down_event: F2) -> Self {
            Self {
              flags: HashMap::from(hm),
              bit_state: $ty::default(),
              up_event,
              down_event
            }
          }

          pub fn set_with_state(&mut self, new: $ty)
          {
              let up_bits = (self.bit_state ^ new) & new;
              let down_bits = (self.bit_state ^ new) & (!new);

              if up_bits == 0 && down_bits == 0 {
                return;
              }

              for (key, value) in &self.flags {
                if up_bits != 0 && ((1 << key) & up_bits) != 0 {
                  (self.up_event)(value);
                }
                if down_bits != 0 && ((1 << key) & down_bits) != 0 {
                  (self.down_event)(value);
                }
              }

              self.bit_state = new;
          }
       }
      )+
    };
}

impl_flag_sets!(u8, u16, u32, u64, u128);

#[macro_export]
macro_rules! flag_set {
    ($ty:ident, {$($bit:literal => $value:expr),+}, $up:expr, $down:expr) => {{
      $(
        const _: () = {
          const BIT_MAX: u8 = $ty::MAX_SIZE;
          const FLAG_BIT: u8 = $bit;
          assert!(FLAG_BIT < BIT_MAX, concat!("Bit Postion ", stringify!($bit)," is higher than maximum allowed for parameter type ", stringify!($ty)));
        };
      )+

      FlagSet::<$ty,_, _, _>::new([
        $(($bit, $value)),+
      ],
      $up,
      $down)
    }};
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::BitState;
    use crate::bit_flags::*;
    #[test]
    fn simple() {
        let up_events = RefCell::new(Vec::new());
        let down_events = RefCell::new(Vec::new());
        let mut result = flag_set!(
            u8,
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
