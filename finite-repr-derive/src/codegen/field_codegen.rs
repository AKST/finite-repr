use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, Type, Variant};

/**
 * Meta data associated with fieldset, along with codegen
 * methods for these types.
 */
pub enum FieldCodegen {
  Fieldless,
  Product(Vec<Type>),
  ProductNamed(Vec<(Ident, Type)>),
}

impl FieldCodegen {
  pub fn from_struct(struct_data: &DataStruct) -> Self {
    Self::from_fields(&struct_data.fields)
  }

  pub fn from_enum_variant(variant: &Variant) -> Self {
    Self::from_fields(&variant.fields)
  }

  fn from_fields(fields: &Fields) -> Self {
    match fields {
      Fields::Unit => FieldCodegen::Fieldless,
      Fields::Named(_) => {
        let tys = fields
          .iter()
          .map(|f| {
            let ident = f.ident.clone().unwrap();
            (ident, f.ty.clone())
          })
          .collect();
        FieldCodegen::ProductNamed(tys)
      }
      Fields::Unnamed(_) => {
        let tys = fields.iter().map(|f| f.ty.clone()).collect();
        FieldCodegen::Product(tys)
      }
    }
  }

  pub fn calc_size(&self) -> TokenStream {
    let types = match self {
      FieldCodegen::Fieldless => return quote! { 1 },
      FieldCodegen::Product(types) if types.is_empty() => return quote! { 0 },
      FieldCodegen::ProductNamed(types) if types.is_empty() => return quote! { 0 },
      FieldCodegen::Product(types) => types.clone(),
      FieldCodegen::ProductNamed(types) => types.iter().map(|t| t.1.clone()).collect(),
    };

    let head = types[0].clone();
    let mut tokens = quote! { (#head::REPRS) };

    for field in types[1..].iter() {
      let seperated = seperate_type(field);
      tokens = quote! { #tokens * (#seperated::REPRS) };
    }

    quote! { (#tokens) }
  }

  pub fn field_destructor(&self) -> TokenStream {
    let with_iterator = |iter: &mut dyn Iterator<Item = TokenStream>| {
      let mut tokens = quote! {};

      if let Some(name) = iter.next() {
        tokens = quote! { #tokens #name };
      }

      for name in iter {
        tokens = quote! { #tokens, #name };
      }

      tokens
    };

    match self {
      FieldCodegen::Fieldless => {
        quote! {}
      }
      FieldCodegen::Product(fields) => {
        let mut iter = fields.iter().enumerate().map(|(i, _)| anon_ident(i));
        let tokens = with_iterator(&mut iter);
        quote! { (#tokens) }
      }
      FieldCodegen::ProductNamed(fields) => {
        let mut iter = fields.iter().map(|(ident, _)| quote! { #ident });
        let tokens = with_iterator(&mut iter);
        quote! { { #tokens } }
      }
    }
  }

  pub fn branch_size(&self, generic: &Ident) -> (TokenStream, TokenStream) {
    let with_iterator = |iter: &mut dyn Iterator<Item = (TokenStream, &Type)>| {
      let (mut tokens, mut previous_type) = if let Some((name, own_type)) = iter.next() {
        let tokens = quote! { #name.into_finite::<#generic>()? };
        (tokens, own_type)
      } else {
        return (quote! {}, quote! { #generic::ONE });
      };

      let mut size_stmt = quote! {};
      let mut size_expr = quote! { #generic::ONE };

      for (name, own_type) in iter {
        let ident_name = format!("size_type_{}", name);
        let size_ident = Ident::new(&ident_name, Span::call_site());
        size_stmt = quote! {
          #size_stmt
          let #size_ident = #generic::from_usize(#previous_type::REPRS)?;
        };

        size_expr = quote! { #size_expr * size_#previous_type };
        tokens = quote! {
          #tokens + (#size_ident * #name.into_finite::<#generic>()?)
        };
        previous_type = own_type;
      }

      (size_stmt, quote! { ( #tokens ) })
    };

    match self {
      FieldCodegen::Fieldless => (quote! {}, quote! { #generic::from_usize(0)? }),
      FieldCodegen::Product(fields) => {
        let mut iter = fields.iter().enumerate().map(|(i, t)| {
          let name = anon_ident(i);
          (quote! { #name }, t)
        });

        with_iterator(&mut iter)
      }
      FieldCodegen::ProductNamed(fields) => {
        let mut iter = fields.iter().map(|(ident, t)| (quote! { #ident }, t));

        with_iterator(&mut iter)
      }
    }
  }

  pub fn branch_construction(
    &self,
    value: &TokenStream,
    generic_type: &Ident,
  ) -> (TokenStream, TokenStream, Option<TokenStream>) {
    let with_iterator = |iter: &mut dyn Iterator<Item = (TokenStream, &Type)>| {
      let mut field_sizes = quote! {};
      let mut assignments = quote! {};
      let mut iter = iter.peekable();

      let create_field_size_ident =
        |n: &TokenStream| Ident::new(&format!("size_{}", n), Span::call_site());

      let mut add_field_size = |name: &Ident, ty: &Type| {
        field_sizes = quote! {
          #field_sizes
          let #name = #generic_type::from_usize(#ty::REPRS)?;
        };
      };

      let mut add_field_assignment = |name: &TokenStream, ty: &Type, arg| {
        assignments = quote! {
          #assignments
          let #name = #ty::from_finite(#arg)?;
        };
      };

      let mut args = quote! {};
      let mut previous_size = quote! {};

      match iter.next() {
        Some((n, t)) => {
          let ident = create_field_size_ident(&n);
          add_field_size(&ident, &t);
          add_field_assignment(&n, &t, quote! { #value % #ident });
          args = quote! { #args #n };
          previous_size = quote! { #previous_size #ident };
        }
        None => {
          let cond = quote! { #value > #generic_type::MIN };
          return (quote! {}, quote! {}, Some(cond));
        }
      };

      while let Some((n, t)) = iter.next() {
        let ident = create_field_size_ident(&n);
        add_field_size(&ident, &t);

        let value_passed = if iter.peek().is_none() {
          quote! { #value / #ident }
        } else {
          quote! { (#value / (#previous_size)) % #ident }
        };

        add_field_assignment(&n, &t, value_passed);

        args = quote! { #args, #n };
        previous_size = quote! { #previous_size * #ident };
      }

      let stmts = quote! {
        #field_sizes
        #assignments
      };

      (stmts, quote! { #args }, None)
    };

    match self {
      FieldCodegen::Fieldless => (
        quote! {},
        quote! {},
        Some(quote! { #value == #generic_type::MIN }),
      ),
      FieldCodegen::Product(fields) => {
        let mut iter = fields.iter().enumerate().map(|(i, t)| {
          let name = anon_ident(i);
          (quote! { #name }, t)
        });

        let (s, args, cond) = with_iterator(&mut iter);
        (s, quote! { ( #args ) }, cond)
      }
      FieldCodegen::ProductNamed(fields) => {
        let mut iter = fields.iter().map(|(n, t)| (quote! { #n }, t));

        let (s, args, cond) = with_iterator(&mut iter);
        (s, quote! { { #args } }, cond)
      }
    }
  }
}

fn anon_ident(index: usize) -> TokenStream {
  let name = format!("unknown_{}", index);
  let ident = Ident::new(&name, Span::call_site());
  quote! { #ident }
}

fn seperate_type(t: &Type) -> TokenStream {
  match t {
    Type::Path(typepath) => {
      let mut typepath = typepath.clone();

      // ensure all the generic types have :: place betwen the type & the arguments
      for segment in typepath.path.segments.iter_mut() {
        if let syn::PathArguments::AngleBracketed(a) = &mut segment.arguments {
          a.colon2_token = Some(syn::token::Colon2::default());
        }
      }

      quote! { #typepath }
    }
    other => quote! { #other },
  }
}
