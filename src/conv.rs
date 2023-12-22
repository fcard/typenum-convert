use typenum::UTerm;
use typenum::Unsigned;
use typenum::{Bit, B1, B0};
use typenum::UInt;
use crate::norm;

pub trait TypeNumConvert<Tail: Unsigned> {
  type TypeNum: Unsigned;
}

pub trait TypeNumDigitConvert {
  type TypeDigit: Bit;
}


pub const fn fbit(n: u8, p: u8) -> u8 {
  (n & (1 << p)) >> p
}

pub const fn _p1(n: u16) -> u8 {
  (n & 0x0000_00ff) as u8
}

pub const fn _p2(n: u16) -> u8 {
  ((n & 0x0000_ff00) >> 8) as u8
}

pub const fn _q1(n: u32) -> u16 {
  (n & 0x0000_ffff) as u16
}

pub const fn _q2(n: u32) -> u16 {
  ((n & 0xffff_0000) >> 16) as u16
}

pub const fn _r1(n: u64) -> u32 {
  (n & 0x0000_0000_ffff_ffff) as u32
}

pub const fn _r2(n: u64) -> u32 {
  ((n & 0xffff_ffff_0000_0000) >> 32) as u32
}

type D<const N: u8, const P: u8> =
  <Conv8<{fbit(N,P)}>
    as TypeNumDigitConvert>::TypeDigit;

pub type T8<const N: u8>   = Norm<T8p<N, UTerm>>;
pub type T16<const N: u16> = Norm<T16p<N, UTerm>>;
pub type T32<const N: u32> = Norm<T32p<N, UTerm>>;
pub type T64<const N: u64> = Norm<T64p<N, UTerm>>;

type UC8<
  B1, B2, B3, B4, B5, B6, B7, B8, Tail> =
  UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<Tail,
    B1>, B2>, B3>, B4>, B5>, B6>, B7>, B8>;

#[cfg(feature = "norm")]
pub type Norm<T> = <T as norm::TypeNumNormalize>::TypeNorm;

#[cfg(not(feature = "norm"))]
pub type Norm<T> = T;

pub type T8p<const N: u8, Tail> =
  UC8<
    D<N,7>,
    D<N,6>,
    D<N,5>,
    D<N,4>,
    D<N,3>,
    D<N,2>,
    D<N,1>,
    D<N,0>, Tail>;

pub type T16p<const N: u16, Tail> =
  T8p<{_p1(N)}, T8p<{_p2(N)}, Tail>>;

pub type T32p<const N: u32, Tail> =
  T16p<{_q1(N)}, T16p<{_q2(N)}, Tail>>;

pub type T64p<const N: u64, Tail> =
  T32p<{_r1(N)}, T32p<{_r2(N)}, Tail>>;


pub struct Conv8<const N: u8>;
pub struct Conv16<const N: u16>;

impl TypeNumDigitConvert for Conv8<1> {
  type TypeDigit = B1;
}

impl TypeNumDigitConvert for Conv8<0> {
  type TypeDigit = B0;
}

