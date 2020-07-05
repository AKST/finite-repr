use finite_repr::{FiniteDecoding, FiniteEncoding, FiniteRepr};

fn main() {
  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  enum Has2Reprs {
    P1,
    P2,
  }

  for value in pairs(vec![Has2Reprs::P1, Has2Reprs::P2]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  enum HasSubReprs {
    S(bool),
  }

  for value in pairs(vec![HasSubReprs::S(false), HasSubReprs::S(true)]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  enum HasProducts {
    A(bool),
    B(bool, bool),
    C(bool),
  }

  for value in pairs(vec![
    HasProducts::A(false),
    HasProducts::A(true),
    HasProducts::B(false, false),
    HasProducts::B(true, false),
    HasProducts::B(false, true),
    HasProducts::B(true, true),
    HasProducts::C(false),
    HasProducts::C(true),
  ]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  enum HasRecord {
    A { a: bool, b: bool },
    B(bool),
  }

  for value in pairs(vec![
    HasRecord::A { a: false, b: false },
    HasRecord::A { a: true, b: false },
    HasRecord::A { a: false, b: true },
    HasRecord::A { a: true, b: true },
    HasRecord::B(false),
    HasRecord::B(true),
  ]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  struct IsRecord {
    pub a: bool,
    pub b: bool,
  }

  for value in pairs(vec![
    IsRecord { a: false, b: false },
    IsRecord { a: true, b: false },
    IsRecord { a: false, b: true },
    IsRecord { a: true, b: true },
  ]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  struct TupleType(bool, bool);

  for value in pairs(vec![
    TupleType(false, false),
    TupleType(true, false),
    TupleType(false, true),
    TupleType(true, true),
  ]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }

  #[derive(Clone, Copy, PartialEq, Debug, FiniteRepr, FiniteDecoding, FiniteEncoding)]
  struct VoidStruct;

  for value in pairs(vec![VoidStruct]) {
    assert_eq!(
      Some(value),
      value
        .into_finite::<u16>()
        .and_then(FiniteDecoding::from_finite)
    );
  }
}

fn pairs<T>(values: Vec<T>) -> <Vec<T> as IntoIterator>::IntoIter {
  values.into_iter()
}
