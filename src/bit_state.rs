pub trait BitState<T, F> {
    fn set(&mut self, new: F);
    fn set_bit(&mut self, n: u8);
    fn clear_bit(&mut self, n: u8);
    fn get_bit(&self, n: u8) -> bool;
    fn reset(&mut self);
}

macro_rules! bit_state_impl {
  ($($t: ident ),+) => {
       $( impl BitState<$t, $t> for $t {

            fn set(&mut self, new: $t){
              *self = new;
            }

            #[inline]
            fn set_bit(&mut self, n: u8) {
              *self |= 1 << n;
            }

            #[inline]
            fn clear_bit(&mut self, n: u8) {
              *self &= !(1 << n);
            }

            #[inline]
            fn get_bit(&self, n: u8) -> bool {
              (*self & (1 << n)) != 0
            }

            #[inline]
            fn reset(&mut self) {
              *self = 0;
            }
        })+
    };
}

bit_state_impl!(u8, u16, u32, u64, u128);
