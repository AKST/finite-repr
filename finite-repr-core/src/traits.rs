use crate::finite_repr::FiniteRepr;
use crate::finite_value::FiniteValue;

/**
 * Encodes values that implement this trait into values
 * that implement `FiniteValue` (mostly numbers).
 */
pub trait FiniteEncoding: FiniteRepr {
  /**
   * The implementation of this method should be isomorphic
   * to the implementation of `FiniteDecoding::from_finite`.
   */
  fn into_finite<T: FiniteValue>(&self) -> Option<T>;
}

/**
 * Decodes values that implement `FiniteValue` (mostly numbers)
 * into types that implements this trait.
 */
pub trait FiniteDecoding: FiniteRepr + Sized {
  /**
   * The implementation of this method should be isomorphic
   * to the implementation of `FiniteEncoding::into_finite`.
   */
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self>;
}

impl FiniteEncoding for bool {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    Some(if *self { T::ONE } else { T::MIN })
  }
}

impl FiniteDecoding for bool {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    match number {
      v if v == T::ONE => Some(true),
      v if v == T::MIN => Some(false),
      _ => None,
    }
  }
}

impl FiniteEncoding for u8 {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    T::from_usize(u8::into_usize(*self)?)
  }
}

impl FiniteDecoding for u8 {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    u8::from_usize(T::into_usize(number)?)
  }
}

impl<A: FiniteEncoding> FiniteEncoding for Option<A> {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    match self {
      Some(a) => a.into_finite(),
      None => T::from_usize(A::REPRS).map(|v| v.inc()),
    }
  }
}

impl<A: FiniteDecoding> FiniteDecoding for Option<A> {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    let a_size = T::from_usize(A::REPRS)?;

    match number {
      value if value <= a_size => A::from_finite(value).map(Some),
      value if value == a_size.inc() => Some(None),
      _ => None,
    }
  }
}

impl<A: FiniteEncoding, B: FiniteEncoding> FiniteEncoding for (A, B) {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    let (a, b) = self;
    let a_num = a.into_finite::<T>()?;
    let b_num = T::from_usize(A::REPRS)? * b.into_finite::<T>()?;
    Some(a_num + b_num)
  }
}

impl<A: FiniteDecoding, B: FiniteDecoding> FiniteDecoding for (A, B) {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    let a_size = T::from_usize(A::REPRS)?;
    let a = A::from_finite(number % a_size)?;
    let b = B::from_finite(number / a_size)?;
    Some((a, b))
  }
}

impl<A: FiniteEncoding, B: FiniteEncoding, C: FiniteEncoding> FiniteEncoding for (A, B, C) {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    let (a, b, c) = self;
    let a_size = T::from_usize(A::REPRS)?;
    let b_size = T::from_usize(B::REPRS)?;

    let a_num = a.into_finite::<T>()?;
    let b_num = a_size * b.into_finite::<T>()?;
    let c_num = a_size * b_size * c.into_finite::<T>()?;
    Some(a_num + b_num + c_num)
  }
}

impl<A: FiniteDecoding, B: FiniteDecoding, C: FiniteDecoding> FiniteDecoding for (A, B, C) {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    let a_size = T::from_usize(A::REPRS)?;
    let b_size = T::from_usize(B::REPRS)?;

    let a = A::from_finite(number % a_size)?;
    let b = B::from_finite((number / a_size) % b_size)?;
    let c = C::from_finite(number / (a_size * b_size))?;
    Some((a, b, c))
  }
}

impl<A: FiniteEncoding, B: FiniteEncoding> FiniteEncoding for Result<A, B> {
  fn into_finite<T: FiniteValue>(&self) -> Option<T> {
    let a_size = T::from_usize(A::REPRS)?;
    match self {
      Ok(a) => a.into_finite(),
      Err(b) => Some(a_size.inc() + b.into_finite()?),
    }
  }
}

impl<A: FiniteDecoding, B: FiniteDecoding> FiniteDecoding for Result<A, B> {
  fn from_finite<T: FiniteValue>(number: T) -> Option<Self> {
    let usize_v = T::into_usize(number)?;
    let a_size = T::from_usize(A::REPRS)?;

    match number {
      value if usize_v <= A::REPRS => A::from_finite(value).map(Ok),
      value => B::from_finite(value.dec() - a_size).map(Err),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_option_impls() {
    for item in [None, Some(u8::MIN), Some(u8::MAX)].iter() {
      let encoded = item.into_finite::<u16>();
      let decoded = encoded.and_then(Option::<u8>::from_finite);
      assert_eq!(Some(*item), decoded);
    }
  }

  #[test]
  fn test_result_impls() {
    type ResultT = Result<u8, u8>;

    for item in [Ok(u8::MIN), Ok(u8::MAX), Err(u8::MIN), Err(u8::MAX)].iter() {
      let encoded = item.into_finite::<u16>();
      let decoded = encoded.and_then(ResultT::from_finite);
      assert_eq!(Some(*item), decoded);
    }
  }

  #[test]
  fn test_tuple_impls() {
    type TupleT = (u8, bool);

    for item in [
      (u8::MIN, false),
      (u8::MAX, false),
      (u8::MIN, true),
      (u8::MAX, true),
    ]
    .iter()
    {
      let encoded = item.into_finite::<u16>();
      let decoded = encoded.and_then(TupleT::from_finite);
      assert_eq!(Some(*item), decoded);
    }
  }

  #[test]
  fn test_tuple_tuple_impls() {
    type TupleTupleA = (u8, bool, bool);

    for item in [
      (u8::MIN, false, false),
      (u8::MAX, false, false),
      (u8::MIN, true, false),
      (u8::MAX, true, false),
      (u8::MIN, false, true),
      (u8::MAX, false, true),
      (u8::MIN, true, true),
      (u8::MAX, true, true),
    ]
    .iter()
    {
      let encoded = item.into_finite::<u16>();
      let decoded = encoded.and_then(TupleTupleA::from_finite);
      assert_eq!(Some(*item), decoded);
    }

    type TupleTupleB = (bool, u8, bool);

    for item in [
      (false, u8::MIN, false),
      (true, u8::MIN, false),
      (false, u8::MAX, false),
      (true, u8::MAX, false),
      (false, u8::MIN, true),
      (true, u8::MIN, true),
      (false, u8::MAX, true),
      (true, u8::MAX, true),
    ]
    .iter()
    {
      let encoded = item.into_finite::<u16>();
      let decoded = encoded.and_then(TupleTupleB::from_finite);
      assert_eq!(Some(*item), decoded);
    }
  }
}
