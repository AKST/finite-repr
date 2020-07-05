use super::field_codegen::FieldCodegen;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::Variant;

pub struct EnumCodegen(HashMap<Ident, VariantInfo>);

impl EnumCodegen {
  pub fn new() -> Self {
    EnumCodegen(HashMap::new())
  }

  pub fn insert_variant(&mut self, variant: &Variant, parent: Option<Ident>) -> &VariantInfo {
    let repr = FieldCodegen::from_enum_variant(variant);
    self
      .0
      .insert(variant.ident.clone(), VariantInfo { repr, parent });
    self.0.get(&variant.ident).unwrap()
  }

  pub fn size_expresssion(&self) -> TokenStream {
    let mut variants = self.0.values();
    let mut tokens = match variants.next() {
      None => return quote! { 0 },
      Some(v) => {
        let variant_size = v.repr.calc_size();
        quote! { #variant_size }
      }
    };

    for variant in variants {
      let variant_size = variant.repr.calc_size();
      tokens = quote! { #tokens + #variant_size };
    }

    tokens
  }
}

pub struct VariantInfo {
  #[allow(dead_code)]
  parent: Option<Ident>,
  pub repr: FieldCodegen,
}
