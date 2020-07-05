use crate::codegen::bounds::add_trait_bounds;
use crate::codegen::field_codegen::FieldCodegen;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Data;

use self::enum_builder::EnumBuilder;
pub use self::error::*;

type ImplResult<T> = Result<T, DeriveFiniteEncoding>;

pub fn impl_derive(ast: &syn::DeriveInput) -> ImplResult<TokenStream> {
  let name = &ast.ident;

  let bound = syn::parse_str("finite_repr::FiniteDecoding")
    .map_err(|_| DeriveFiniteEncoding::CouldNotAddTraitBounds(ast.ident.span()))?;

  let generics = add_trait_bounds(&ast.generics, &bound);
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  let generic_param = Ident::new("T", Span::call_site());
  let value_param = Ident::new("value", Span::call_site());
  let body = generate_body(&ast, &value_param, &generic_param)?;

  let implementation_gen = quote! {
    impl #impl_generics finite_repr::FiniteDecoding for #name #ty_generics #where_clause {
      fn from_finite<#generic_param: finite_repr::FiniteValue>(#value_param: #generic_param) -> Option<Self> {
        #body
      }
    }
  };

  Ok(implementation_gen)
}

fn generate_body(
  ast: &syn::DeriveInput,
  value_param: &Ident,
  generic_param: &Ident,
) -> ImplResult<TokenStream> {
  match &ast.data {
    Data::Enum(data_enum) => {
      let mut builder = EnumBuilder::new(&ast.ident, value_param, generic_param);

      for variant in data_enum.variants.iter() {
        builder.with_variant(variant);
      }

      Ok(builder.get_expression())
    }
    Data::Struct(struct_data) => {
      let codegen = FieldCodegen::from_struct(&struct_data);
      let type_name = &ast.ident;
      let value = quote! { #value_param };
      let (constructor_stmts, constructor_arguments, condition) =
        codegen.branch_construction(&value, generic_param);

      let condition = match condition {
        Some(condition) => condition,
        None => {
          let ceil_size = codegen.calc_size();
          quote! { #value_param < #generic_param::from_usize(#ceil_size)? }
        }
      };

      Ok(quote! {
        if #condition {
          #constructor_stmts
          return Some(#type_name #constructor_arguments);
        }
        return None;
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
    value_idenfier: &'a Ident,
    generic_param: &'a Ident,
    codegen: EnumCodegen,
    type_name: &'a Ident,
  }

  impl<'a> EnumBuilder<'a> {
    pub fn new(type_name: &'a Ident, value_idenfier: &'a Ident, generic_param: &'a Ident) -> Self {
      EnumBuilder {
        body: quote! {},
        codegen: EnumCodegen::new(),
        type_name,
        generic_param,
        value_idenfier,
      }
    }

    pub fn with_variant(&mut self, variant: &Variant) {
      let generic_param = &self.generic_param;
      let value_idenfier = &self.value_idenfier;
      let variant_name = &variant.ident;

      let floor_size = self.codegen.size_expresssion();
      let variant = self.codegen.insert_variant(variant, None);

      let value = quote! {
        (#value_idenfier - #generic_param::from_usize(#floor_size)?)
      };

      let (constructor_stmts, constructor_arguments, condition) = variant
        .repr
        .branch_construction(&value, &self.generic_param);

      let body_so_far = &self.body;
      let type_name = &self.type_name;
      let condition = match condition {
        Some(condition) => condition,
        None => {
          let ceil_size = self.codegen.size_expresssion();
          quote! {
            #value_idenfier < #generic_param::from_usize(#ceil_size)?
          }
        }
      };

      self.body = quote! {
        #body_so_far
        if #condition {
          #constructor_stmts
          return Some(#type_name::#variant_name #constructor_arguments);
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
