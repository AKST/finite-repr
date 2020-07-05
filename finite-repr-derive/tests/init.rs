use trybuild::TestCases;

#[test]
fn tests() {
  let t = TestCases::new();
  t.pass("tests/test_derive_repr.rs");
  t.pass("tests/test_derive_encoding.rs");
  t.pass("tests/test_derive_decoding.rs");
  t.pass("tests/test_isomorphism.rs");
}
