/**
 * This create exists to allow the encoding of data
 */
mod finite_repr;
mod finite_value;
mod traits;

pub use self::finite_repr::FiniteRepr;
pub use self::finite_value::FiniteValue;
pub use self::traits::{FiniteDecoding, FiniteEncoding};

// Re-export #[derive(FiniteRepr, FiniteEncoding, FiniteDecoding)].
#[cfg(feature = "finite_repr_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate finite_repr_derive;
#[cfg(feature = "finite_repr_derive")]
#[doc(hidden)]
pub use finite_repr_derive::*;
