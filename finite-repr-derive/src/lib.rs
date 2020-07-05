extern crate proc_macro;

mod codegen;
mod finite_decoding;
mod finite_encoding;
mod finite_repr;
mod impl_error;

use impl_error::CompilerError;
use proc_macro::TokenStream;

#[proc_macro_derive(FiniteEncoding)]
pub fn derive_finite_encoding(input: TokenStream) -> TokenStream {
  match syn::parse(input) {
    Ok(data) => flatten_stream(finite_encoding::impl_derive(&data)),
    Err(e) => e.to_compile_error().into(),
  }
}

#[proc_macro_derive(FiniteDecoding)]
pub fn derive_finite_decoding(input: TokenStream) -> TokenStream {
  match syn::parse(input) {
    Ok(data) => flatten_stream(finite_decoding::impl_derive(&data)),
    Err(e) => e.to_compile_error().into(),
  }
}

#[proc_macro_derive(FiniteRepr)]
pub fn derive_finite_repr(input: TokenStream) -> TokenStream {
  match syn::parse(input) {
    Ok(data) => flatten_stream(finite_repr::impl_derive(&data)),
    Err(e) => e.to_compile_error().into(),
  }
}

fn flatten_stream<E: CompilerError>(r: Result<proc_macro2::TokenStream, E>) -> TokenStream {
  match r {
    Ok(s) => s.into(),
    Err(e) => e.compile_error().into(),
  }
}
