use finite_repr::{FiniteDecoding, FiniteRepr};

fn main() {
  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  enum Has2Reprs {
    P1,
    P2,
  }

  for (d, value) in pairs(vec![(Has2Reprs::P1, 0), (Has2Reprs::P2, 1)]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  enum HasSubReprs {
    S(bool),
  }

  for (d, value) in pairs(vec![(HasSubReprs::S(false), 0), (HasSubReprs::S(true), 1)]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  enum HasProducts {
    A(bool),
    B(bool, bool),
    C(bool),
  }

  for (d, value) in pairs(vec![
    (HasProducts::A(false), 0),
    (HasProducts::A(true), 1),
    (HasProducts::B(false, false), 2),
    (HasProducts::B(true, false), 3),
    (HasProducts::B(false, true), 4),
    (HasProducts::B(true, true), 5),
    (HasProducts::C(false), 6),
    (HasProducts::C(true), 7),
  ]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  enum HasRecord {
    A { a: bool, b: bool },
    B(bool),
  }

  for (d, value) in pairs(vec![
    (HasRecord::A { a: false, b: false }, 0),
    (HasRecord::A { a: true, b: false }, 1),
    (HasRecord::A { a: false, b: true }, 2),
    (HasRecord::A { a: true, b: true }, 3),
    (HasRecord::B(false), 4),
    (HasRecord::B(true), 5),
  ]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  struct IsRecord {
    pub a: bool,
    pub b: bool,
  }

  for (d, value) in pairs(vec![
    (IsRecord { a: false, b: false }, 0),
    (IsRecord { a: true, b: false }, 1),
    (IsRecord { a: false, b: true }, 2),
    (IsRecord { a: true, b: true }, 3),
  ]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  struct TupleType(bool, bool);

  for (d, value) in pairs(vec![
    (Some(TupleType(false, false)), 0),
    (Some(TupleType(true, false)), 1),
    (Some(TupleType(false, true)), 2),
    (Some(TupleType(true, true)), 3),
    (None, 4),
  ]) {
    assert_eq!(d, FiniteDecoding::from_finite(value));
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding)]
  struct VoidStruct;

  for (d, value) in pairs(vec![(VoidStruct, 0)]) {
    assert_eq!(Some(d), FiniteDecoding::from_finite(value));
  }
}

fn pairs<T>(values: Vec<(T, u16)>) -> <Vec<(T, u16)> as IntoIterator>::IntoIter {
  values.into_iter()
}
