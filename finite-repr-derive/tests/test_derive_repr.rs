use finite_repr::FiniteRepr;

fn main() {
  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  enum Has2Reprs {
    P1,
    P2,
  }

  assert_eq!(Has2Reprs::REPRS, 2);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  enum HasSubPermutations {
    P1,
    P2(Has2Reprs),
  }

  assert_eq!(HasSubPermutations::REPRS, 3);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  enum WithBools {
    Single(bool),
  }

  assert_eq!(WithBools::REPRS, 2);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  struct With3Bools(bool, bool, bool);

  // the number of representations should be the product of the
  // possible reprs of `With3Bools` which in this case should be
  // just 2 * 2 * 2...
  assert_eq!(With3Bools::REPRS, 2 * 2 * 2);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  struct With5Bools(bool, bool, bool, bool, bool);

  assert_eq!(With5Bools::REPRS, 2 * 2 * 2 * 2 * 2);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  enum GenericPermutaions<T> {
    Single(T),
  }

  assert_eq!(GenericPermutaions::<Has2Reprs>::REPRS, 2);
  assert_eq!(GenericPermutaions::<HasSubPermutations>::REPRS, 3);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  struct ADaringStuct {
    foo: bool,
    boo: With3Bools,
  }

  assert_eq!(ADaringStuct::REPRS, 2 * 2 * 2 * 2);

  #[derive(FiniteRepr)]
  #[allow(dead_code)]
  struct AGenericField {
    foo: bool,
    boo: Option<bool>,
  }

  assert_eq!(AGenericField::REPRS, 2 * 3);
}
