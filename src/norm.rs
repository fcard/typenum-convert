use typenum::UTerm;
use typenum::Unsigned;
use typenum::{Bit, B1, B0};
use typenum::UInt;

pub trait TypeNumNormalize: TypeNumNormalize2 {
  type TypeNorm: Unsigned +
                 TypeNumNormalize;
}

pub trait TypeNumNormalize2 {
  type TypeNorm2: Unsigned + TypeNumNormalize;
}

impl<N: Unsigned + TypeNumNormalize, Bx: Bit>
TypeNumNormalize for UInt<UInt<N, Bx>, B0>
where Self: Unsigned {
  type TypeNorm =
    <UInt<
      UInt<<N as TypeNumNormalize>::TypeNorm, Bx>, B0>
      as TypeNumNormalize2>::TypeNorm2;
}

impl<N: Unsigned + TypeNumNormalize, Bx: Bit>
TypeNumNormalize for UInt<UInt<N, Bx>, B1>
where Self: Unsigned {
  type TypeNorm =
    UInt<
      UInt<<N as TypeNumNormalize>::TypeNorm, Bx>, B1>;
}

impl TypeNumNormalize for UTerm
where Self: Unsigned {
  type TypeNorm = UTerm;
}

impl<N: Unsigned + TypeNumNormalize, Bx: Bit, By: Bit>
TypeNumNormalize2 for UInt<UInt<N, Bx>, By>
where Self: TypeNumNormalize {
  type TypeNorm2 = UInt<UInt<N, Bx>, By>;
}

impl TypeNumNormalize2 for UInt<UTerm, B0>
where Self: Unsigned {
  type TypeNorm2 = UTerm;
}

impl TypeNumNormalize2 for UTerm {
  type TypeNorm2 = UTerm;
}

