use syn::{Generics, Path};

pub fn add_trait_bounds(generics_in: &Generics, path: &Path) -> Generics {
  let mut generics = generics_in.clone();

  for param in generics.params.iter_mut() {
    if let syn::GenericParam::Type(param) = param {
      let bound = syn::TypeParamBound::Trait(syn::TraitBound {
        lifetimes: None,
        modifier: syn::TraitBoundModifier::None,
        paren_token: None,
        path: path.clone(),
      });

      param.bounds.push_value(bound);
    }
  }

  generics
}
