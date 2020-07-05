use finite_repr::{FiniteEncoding, FiniteRepr};

fn main() {
  #[derive(FiniteRepr, FiniteEncoding)]
  enum Has2Reprs {
    P1,
    P2,
  }

  for (d, value) in pairs(&[(Has2Reprs::P1, 0), (Has2Reprs::P2, 1)]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  enum HasSubReprs {
    S(bool),
  }

  for (d, value) in pairs(&[(HasSubReprs::S(false), 0), (HasSubReprs::S(true), 1)]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  enum HasProducts {
    A(bool),
    B(bool, bool),
    C(bool),
  }

  for (d, value) in pairs(&[
    (HasProducts::A(false), 0),
    (HasProducts::A(true), 1),
    (HasProducts::B(false, false), 2),
    (HasProducts::B(true, false), 3),
    (HasProducts::B(false, true), 4),
    (HasProducts::B(true, true), 5),
    (HasProducts::C(false), 6),
    (HasProducts::C(true), 7),
  ]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  enum HasRecord {
    A { a: bool, b: bool },
    B(bool),
  }

  for (d, value) in pairs(&[
    (HasRecord::A { a: false, b: false }, 0),
    (HasRecord::A { a: true, b: false }, 1),
    (HasRecord::A { a: false, b: true }, 2),
    (HasRecord::A { a: true, b: true }, 3),
    (HasRecord::B(false), 4),
    (HasRecord::B(true), 5),
  ]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  struct IsRecord {
    pub a: bool,
    pub b: bool,
  }

  for (d, value) in pairs(&[
    (IsRecord { a: false, b: false }, 0),
    (IsRecord { a: true, b: false }, 1),
    (IsRecord { a: false, b: true }, 2),
    (IsRecord { a: true, b: true }, 3),
  ]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  struct TupleType(bool, bool);

  for (d, value) in pairs(&[
    (TupleType(false, false), 0),
    (TupleType(true, false), 1),
    (TupleType(false, true), 2),
    (TupleType(true, true), 3),
  ]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }

  #[derive(FiniteRepr, FiniteEncoding)]
  struct GenericTuple<A>(A);

  for (d, value) in pairs(&[
    (GenericTuple(TupleType(false, false)), 0),
    (GenericTuple(TupleType(true, false)), 1),
    (GenericTuple(TupleType(false, true)), 2),
    (GenericTuple(TupleType(true, true)), 3),
  ]) {
    assert_eq!(d.into_finite::<u16>(), Some(*value));
  }
}

fn pairs<T>(values: &[(T, u16)]) -> impl Iterator<Item = &(T, u16)> {
  values.iter()
}
