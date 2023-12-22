use super::*;
use typenum::Unsigned;

#[allow(unused)]
mod short {
  use std::ops;
  pub type Add<A,B> = <A as ops::Add<B>>::Output;
  pub type Sub<A,B> = <A as ops::Sub<B>>::Output;
  pub type Mul<A,B> = <A as ops::Mul<B>>::Output;
}

macro_rules! def1 {
  ($test: ident, $main: ident;
   $($T: ident {$e: expr}),*) => {

    #[cfg(test)]
    #[test]
    pub fn $test() {
      $(
        assert_eq!(<$T<{$e}>>::to_u64(), $e as u64);
      )*
    }

    #[cfg_attr(test, allow(unused))]
    pub fn $main() {
      $(
        println!("{}", <$T<{$e}>>::to_u64());
      )*
    }
  }
}

macro_rules! def2 {
  ($test: ident, $main: ident;
   $($T: ty = $e: expr),*) => {

    #[cfg(test)]
    #[test]
    pub fn $test() {
      use self::short::*;
      $(
        assert_eq!(<$T>::to_u64(), $e as u64);
      )*
    }

    #[cfg_attr(test, allow(unused))]
    pub fn $main() {
      use self::short::*;
      $(
        println!("{} == {}",
          stringify!($T), <$T>::to_u64());
      )*
    }
  }
}

def1! {
  test1, main1;
  T8{3u8.pow(3)},
  T16{1 + 4 * 17},
  T32{[1,2,3,4,5].len() as u32},
  T32{u32::MAX},
  T64{3 * u32::MAX as u64},
  T64{u64::MAX}
}

def2! {
  test2, main2;
  Add<T8<8>, T8<16>> = 24,
  Add<T16<{1+4}>, T8<{5+5}>> = 15,
  Add<T32<{u32::MAX}>, T32<{u32::MAX}>> = 2 * (u32::MAX as u64)
}

