use typenum::UTerm;
use typenum::Unsigned;
use typenum::{Bit, B1, B0};
use typenum::UInt;

pub trait Norm: NormI<UTerm,UTerm> {
  type Output;
}

impl<N: Unsigned + NormI<UTerm,UTerm>> Norm for N
where <N as NormI<UTerm,UTerm>>::Output: Inv<UTerm> {
  type Output =
    <<N as NormI<UTerm,UTerm>>::Output
      as Inv<UTerm>>::Output;
}

pub trait NormI<A: Unsigned, B: Unsigned>: Unsigned {
  type Output;
}

impl<A: Unsigned, B: Unsigned> NormI<A,B> for UTerm {
  type Output = B;
}

impl<A: Unsigned, B: Unsigned, N: Unsigned>
NormI<A,B> for UInt<N, B0>
where N: NormI<UInt<A,B0>, B> {
  type Output =
    <N as NormI<UInt<A,B0>, B>>::Output;
}

impl<A: Unsigned, B: Unsigned, N: Unsigned>
NormI<A,B> for UInt<N, B1>
where N: NormI<UInt<A,B1>, UInt<A,B1>> {
  type Output =
    <N as NormI<UInt<A,B1>, UInt<A,B1>>>::Output;
}

pub trait Inv<Acc: Unsigned>: Unsigned {
  type Output: Unsigned;
}

impl<Acc: Unsigned> Inv<Acc> for UTerm {
  type Output = Acc;
}

impl<Acc: Unsigned, N, B: Bit> Inv<Acc> for UInt<N,B>
where N: Inv<UInt<Acc,B>> {
  type Output = <N as Inv<UInt<Acc,B>>>::Output;
}

