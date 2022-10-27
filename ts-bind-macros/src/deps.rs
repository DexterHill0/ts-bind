use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, Type};

use crate::types::r#trait;

#[derive(Default, Debug)]
pub struct Dependencies(Vec<TokenStream>);

// TODO: do we need to use `Type`? (replace with `Path`?)
impl Dependencies {
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.0
            .push(quote!(dependencies.append(&mut <#ty as ts_bind::TS>::dependencies());));
    }

    /// Adds a trait as a dependency, by first checking if the trait was exported.
    pub fn append_from_trait(&mut self, trt: Path) {
        let struct_name = r#trait::get_trait_struct_name(&trt);
        self.0
            .push(quote!(dependencies.append(&mut <#struct_name as ts_bind::TS>::dependencies());));
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dependencies = &self.0;
        tokens.extend(quote! {
            {
                let mut dependencies = vec![];
                #( #dependencies )*
                dependencies
            }
        })
    }
}
