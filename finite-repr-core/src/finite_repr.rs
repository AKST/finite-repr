pub trait FiniteRepr {
  /**
   * The possible permutations for this value.
   */
  const REPRS: usize;
}

impl FiniteRepr for u8 {
  const REPRS: usize = u8::MAX as usize + 1;
}

impl FiniteRepr for u16 {
  const REPRS: usize = u16::MAX as usize + 1;
}

impl FiniteRepr for i8 {
  const REPRS: usize = u8::MAX as usize + 1;
}

impl FiniteRepr for i16 {
  const REPRS: usize = u16::MAX as usize + 1;
}

impl FiniteRepr for bool {
  const REPRS: usize = 2;
}

impl<A: FiniteRepr, B: FiniteRepr> FiniteRepr for (A, B) {
  const REPRS: usize = A::REPRS * B::REPRS;
}

impl<A: FiniteRepr, B: FiniteRepr, C: FiniteRepr> FiniteRepr for (A, B, C) {
  const REPRS: usize = A::REPRS * B::REPRS * C::REPRS;
}

impl<A: FiniteRepr> FiniteRepr for Option<A> {
  const REPRS: usize = 1 + A::REPRS;
}

impl<A: FiniteRepr, B: FiniteRepr> FiniteRepr for Result<A, B> {
  const REPRS: usize = A::REPRS + B::REPRS;
}
