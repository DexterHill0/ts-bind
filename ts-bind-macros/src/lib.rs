#![macro_use]

#[macro_use]
pub(crate) mod utils;
mod attrs;
mod deps;
mod generics;
mod types;

use deps::Dependencies;
use generics::ParsedGenerics;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ConstParam, GenericParam, Generics, ItemFn, LifetimeDef,
    TypeParam,
};
use ts_bind_shared::Bindings;

use crate::types::r#trait::try_parse_trait;

#[derive(Debug)]
struct DerivedTS {
    name: String,
    decl: TokenStream,
    extra: TokenStream,
    dependencies: Dependencies,

    export: Option<bool>,
    export_to: Option<String>,
}

impl DerivedTS {
    // fn generate_export_test(&self, rust_ty: &Ident, generics: &Generics) -> Option<TokenStream> {
    //     let test_fn = format_ident!("export_bindings_{}", &self.name.to_lowercase());
    //     let generic_params = generics
    //         .params
    //         .iter()
    //         .filter(|param| matches!(param, GenericParam::Type(_)))
    //         .map(|_| quote! { () });
    //     let ty = quote!(<#rust_ty<#(#generic_params),*> as ts_rs::TS>);

    //     Some(quote! {
    //         #[cfg(test)]
    //         #[test]
    //         fn #test_fn() {
    //             #ty::export().expect("could not export type");
    //         }
    //     })
    // }

    fn into_impl(self, rust_ty: Ident, generics: &Generics) -> TokenStream {
        let export_to = match &self.export_to {
            Some(dirname) if dirname.ends_with('/') => {
                format!("{}{}.ts", dirname, self.name)
            }
            Some(filename) => filename.clone(),
            None => {
                format!("bindings/{}.ts", self.name)
            }
        };

        // let export = match self.export {
        //     true => Some(self.generate_export_test(&rust_ty, &generics)),
        //     false => None,
        // };

        let DerivedTS {
            name,
            decl,
            dependencies,
            extra,
            ..
        } = self;

        let impl_start = generate_impl(&rust_ty, generics);

        quote! {
            #extra

            #impl_start {
                const EXPORT_TO: Option<&'static str> = Some(#export_to);

                fn decl(bindings: &mut ts_bind::Bindings) -> ts_bind::TSType {
                    #decl
                }

                fn name() -> String {
                    #name.into()
                }

                fn dependencies() -> Vec<ts_bind::Dependency> {
                    #dependencies
                }
            }

            //#export
        }
    }
}

// generate start of the `impl TS for #ty` block, up to (excluding) the open brace
fn generate_impl(ty: &Ident, generics: &Generics) -> TokenStream {
    let mut type_args = vec![];

    let bounds = generics.params.iter().map(|param| match param {
        GenericParam::Type(TypeParam {
            ident,
            colon_token,
            bounds,
            ..
        }) => {
            type_args.push(quote! { #ident });
            quote!(#ident #colon_token #bounds)
        }
        GenericParam::Lifetime(LifetimeDef {
            lifetime,
            colon_token,
            bounds,
            ..
        }) => {
            type_args.push(quote! { #lifetime });
            quote!(#lifetime #colon_token #bounds)
        }
        GenericParam::Const(ConstParam {
            const_token,
            ident,
            colon_token,
            ty,
            ..
        }) => {
            type_args.push(quote! { #ident });
            quote!(#const_token #ident #colon_token #ty)
        }
    });

    //let where_bound = add_ts_to_where_clause(generics);
    quote!(impl <#(#bounds),*> ts_bind::TS for #ty <#(#type_args),*>)
}

#[proc_macro_derive(TS, attributes(ts))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // match entry(input) {
    //     Err(err) => err.to_compile_error(),
    //     Ok(result) => result,
    // }
    // .into()

    todo!()
}

// used on traits to define abstract classes
#[proc_macro_attribute]
pub fn ts(data: proc_macro::TokenStream, trt: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut bindings = Bindings::default();

    match try_parse_trait(trt, data, &mut bindings) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}
