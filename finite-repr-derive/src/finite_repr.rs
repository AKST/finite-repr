use crate::codegen::bounds::add_trait_bounds;
use crate::codegen::enum_codegen::EnumCodegen;
use crate::codegen::field_codegen::FieldCodegen;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Ident};

pub use error::*;

type ImplResult<T> = Result<T, DeriveFiniteRepr>;

pub fn impl_derive(ast: &syn::DeriveInput) -> ImplResult<TokenStream> {
  let name = &ast.ident;

  let bound = syn::parse_str("finite_repr::FiniteRepr")
    .map_err(|_| DeriveFiniteRepr::CouldNotAddTraitBounds(ast.ident.span()))?;

  let generics = add_trait_bounds(&ast.generics, &bound);
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let reprs = count_reprs(&ast.data)?;

  let implementation_gen = quote! {
    impl #impl_generics finite_repr::FiniteRepr for #name #ty_generics #where_clause {
      const REPRS: usize = #reprs;
    }
  };

  Ok(implementation_gen)
}

fn count_reprs(data: &Data) -> ImplResult<TokenStream> {
  match data {
    Data::Enum(d) => {
      let mut reprs = EnumCodegen::new();
      let mut parent: Option<Ident> = None;

      for variant in d.variants.iter() {
        reprs.insert_variant(&variant, parent.clone());
        parent = Some(variant.ident.clone());
      }

      Ok(reprs.size_expresssion())
    }
    Data::Struct(s) => {
      let reprs = FieldCodegen::from_struct(&s);
      Ok(reprs.calc_size())
    }
    Data::Union(u) => Err(DeriveFiniteRepr::UnionNotSupported(u.union_token.span)),
  }
}

mod error {
  use crate::impl_error::CompilerError;

  pub enum DeriveFiniteRepr {
    CouldNotAddTraitBounds(proc_macro2::Span),
    UnionNotSupported(proc_macro2::Span),
  }

  impl CompilerError for DeriveFiniteRepr {
    fn compile_error(&self) -> proc_macro2::TokenStream {
      match *self {
        DeriveFiniteRepr::CouldNotAddTraitBounds(span) => {
          quote::quote_spanned! {
            span => compile_error!("Tragic... Could not add a FiniteRepr trait bound.")
          }
        }
        DeriveFiniteRepr::UnionNotSupported(span) => {
          quote::quote_spanned! {
            span => compile_error!("union are not supported... yet(?)")
          }
        }
      }
    }
  }
}
