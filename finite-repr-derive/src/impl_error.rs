pub trait GetSpan {
  fn get_span(&self) -> proc_macro2::Span;
}

pub trait CompilerError {
  fn compile_error(&self) -> proc_macro2::TokenStream;
}
