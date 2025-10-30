pub trait BitState<T> {
    fn set(&mut self, new: T);
    fn set_with_changes(&mut self, new: T) -> Option<(Vec<u8>, Vec<u8>)>;
    fn set_bit(&mut self, n: u8);
    fn clear_bit(&mut self, n: u8);
    fn get_bit(&self, n: u8) -> bool;
    fn reset(&mut self);
}

macro_rules! bit_state_impl {
  ($($t: ident ),+) => {
       $(impl BitState<$t> for $t {

            #[inline]
            fn set(&mut self, new: $t) {
              *self = new;
            }

            fn set_with_changes(&mut self, new: $t) -> Option<(Vec<u8>,Vec<u8>)> {
              if *self == new {
                return None;
              };
              let mut up_bits = (*self ^ new) & new;
              let mut down_bits = (*self ^ new) & (!new);
              let mut pos = 0;

              let mut ups: Vec<u8> = Vec::new();
              let mut downs: Vec<u8> = Vec::new();

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
              *self = new;
              Some((ups,downs))
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

#[cfg(test)]
mod tests {
    use crate::BitState;

    #[test]
    fn it_works() {
        let mut result = 0u8;
        result.set_bit(1);
        assert_eq!(result, 0b_0000_0010);
        result.set_bit(3);
        assert_eq!(result, 0b_0000_1010);
    }
}
