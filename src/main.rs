#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod conv;
pub use conv::{T8, T16, T32, T64};

pub(crate) mod test;

#[cfg(feature = "norm")]
pub(crate) mod norm;

fn main() {
  test::main1();
  test::main2();
  use typenum::{U4, Unsigned};
  type Four   = U4;
  type Eight  = T32<{Four::U32 * 2}>;
  type Twelve = T32<{Four::U32 + Eight::U32}>;
  println!("{} {} {}",
    Four::U32, Eight::U32, Twelve::to_u32());
}
