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
}
