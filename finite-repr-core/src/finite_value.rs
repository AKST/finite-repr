use std::convert::TryInto;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait FiniteValue:
  Copy
  + Add<Self, Output = Self>
  + Div<Self, Output = Self>
  + Mul<Self, Output = Self>
  + Rem<Self, Output = Self>
  + Sub<Self, Output = Self>
  + PartialOrd
{
  const MIN: Self;
  const ONE: Self;
  const ZERO: Self;

  fn inc(self) -> Self {
    self + Self::ONE
  }

  fn dec(self) -> Self {
    self - Self::ONE
  }

  fn from_usize(other: usize) -> Option<Self>;
  fn into_usize(other: Self) -> Option<usize>;
}

impl FiniteValue for u8 {
  const MIN: u8 = 0;
  const ONE: u8 = 1;
  const ZERO: u8 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for u16 {
  const MIN: u16 = 0;
  const ONE: u16 = 1;
  const ZERO: u16 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for u32 {
  const MIN: u32 = 0;
  const ONE: u32 = 1;
  const ZERO: u32 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for u64 {
  const MIN: u64 = 0;
  const ONE: u64 = 1;
  const ZERO: u64 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for u128 {
  const MIN: u128 = 0;
  const ONE: u128 = 1;
  const ZERO: u128 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for i8 {
  const MIN: i8 = i8::MIN;
  const ONE: i8 = 1;
  const ZERO: i8 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for i16 {
  const MIN: i16 = i16::MIN;
  const ONE: i16 = 1;
  const ZERO: i16 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for i32 {
  const MIN: i32 = i32::MIN;
  const ONE: i32 = 1;
  const ZERO: i32 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for i64 {
  const MIN: i64 = i64::MIN;
  const ONE: i64 = 1;
  const ZERO: i64 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}

impl FiniteValue for i128 {
  const MIN: i128 = i128::MIN;
  const ONE: i128 = 1;
  const ZERO: i128 = 0;

  fn from_usize(other: usize) -> Option<Self> {
    other.try_into().ok()
  }

  fn into_usize(other: Self) -> Option<usize> {
    other.try_into().ok()
  }
}
