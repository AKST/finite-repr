use self::enum_builder::EnumBuilder;
use crate::codegen::bounds::add_trait_bounds;
use crate::codegen::field_codegen::FieldCodegen;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Data;

pub use error::*;

type ImplResult<T> = Result<T, DeriveFiniteEncoding>;

pub fn impl_derive(ast: &syn::DeriveInput) -> ImplResult<TokenStream> {
  let name = &ast.ident;

  let bound = syn::parse_str("finite_repr::FiniteEncoding")
    .map_err(|_| DeriveFiniteEncoding::CouldNotAddTraitBounds(ast.ident.span()))?;

  let generics = add_trait_bounds(&ast.generics, &bound);
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  let generic_param = Ident::new("T", Span::call_site());
  let body = generate_body(&ast, &generic_param)?;

  let implementation_gen = quote! {
    impl #impl_generics finite_repr::FiniteEncoding for #name #ty_generics #where_clause {
      fn into_finite<#generic_param: finite_repr::FiniteValue>(&self) -> Option<#generic_param> {
        #body
      }
    }
  };

  Ok(implementation_gen)
}

fn generate_body(ast: &syn::DeriveInput, generic_param: &Ident) -> ImplResult<TokenStream> {
  match &ast.data {
    Data::Enum(data_enum) => {
      let mut builder = EnumBuilder::new(&ast.ident, &generic_param);

      for variant in data_enum.variants.iter() {
        builder.with_variant(variant);
      }

      Ok(builder.get_expression())
    }
    Data::Struct(struct_data) => {
      let codegen = FieldCodegen::from_struct(&struct_data);
      let type_name = &ast.ident;
      let destructor = codegen.field_destructor();
      let (branch_stmts, branch_size) = codegen.branch_size(generic_param);

      Ok(quote! {
        let #type_name #destructor = self;
        #branch_stmts
        return Some(#branch_size);
      })
    }
    Data::Union(u) => Err(DeriveFiniteEncoding::UnionNotSupported(u.union_token.span)),
  }
}

mod enum_builder {
  use crate::codegen::enum_codegen::EnumCodegen;
  use proc_macro2::{Ident, TokenStream};
  use quote::quote;
  use syn::Variant;

  pub struct EnumBuilder<'a> {
    body: TokenStream,
    generic_param: &'a Ident,
    codegen: EnumCodegen,
    type_name: &'a Ident,
  }

  impl<'a> EnumBuilder<'a> {
    pub fn new(type_name: &'a Ident, generic_param: &'a Ident) -> Self {
      EnumBuilder {
        body: quote! {},
        generic_param,
        codegen: EnumCodegen::new(),
        type_name,
      }
    }

    pub fn with_variant(&mut self, variant: &Variant) {
      let body_so_far = &self.body;
      let type_name = &self.type_name;
      let generic_param = &self.generic_param;
      let variant_name = &variant.ident;

      let base_size = self.codegen.size_expresssion();
      let variant = self.codegen.insert_variant(variant, None);
      let destructor = variant.repr.field_destructor();
      let (branch_stmts, branch_size) = variant.repr.branch_size(generic_param);

      self.body = quote! {
        #body_so_far
        if let #type_name::#variant_name #destructor = self {
          let base = #generic_param::from_usize(#base_size)?;
          #branch_stmts
          return Some(base + #branch_size);
        }
      };
    }

    pub fn get_expression(self) -> TokenStream {
      let body = self.body;
      quote! {
        #body
        return None;
      }
    }
  }
}

mod error {
  use crate::impl_error::CompilerError;

  pub enum DeriveFiniteEncoding {
    CouldNotAddTraitBounds(proc_macro2::Span),
    UnionNotSupported(proc_macro2::Span),
  }

  impl CompilerError for DeriveFiniteEncoding {
    fn compile_error(&self) -> proc_macro2::TokenStream {
      match *self {
        DeriveFiniteEncoding::CouldNotAddTraitBounds(span) => {
          quote::quote_spanned! {
            span => compile_error!("Tragic... Could not add a FiniteEncoding trait bound.")
          }
        }
        DeriveFiniteEncoding::UnionNotSupported(span) => {
          quote::quote_spanned! {
            span => compile_error!("union are not supported... yet(?)")
          }
        }
      }
    }
  }
}
