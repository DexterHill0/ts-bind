use darling::{FromAttributes, FromMeta, ToTokens};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse::Parser, parse_macro_input::ParseMacroInput, parse_quote, parse_str, AttributeArgs,
    Generics, ItemTrait, Path, Result, TraitItem, Type, TypePath,
};
use ts_bind_shared::{AbstractClass, Bindings};

use crate::{
    attrs::r#trait::{TraitAttrs, TraitItemAttrs},
    deps::Dependencies,
    generics::{parse_generics, InnerType},
    utils::{to_ts_ident, warning},
    DerivedTS,
};

const STRUCT_PREFIX: &str = "_ts_";

/// fn foo(&self) {}   # public foo(): void { ... };
/// fn foo(&self);     # public abstrct foo(): void ;
///
/// #[ts(no_abstract)]
/// fn foo(&self);     # public foo(): void { ... };
///
/// fn foo() -> {}     # public static foo(): void { ... };
/// fn foo();          # public static foo(): void { ... };

/// Parses a trait definition and turns it into an abstract class.
pub(crate) fn try_parse_trait(
    trt: proc_macro::TokenStream,
    args: proc_macro::TokenStream,
    bindings: &mut Bindings,
) -> Result<proc_macro2::TokenStream> {
    let input = syn::parse::<ItemTrait>(trt)?;

    let mut dependencies = Dependencies::default();

    let attr_parser = AttributeArgs::parse;
    let attrs = attr_parser.parse(args)?;
    let trait_attrs = TraitAttrs::from_list(&attrs)?;

    let generic_bindings = parse_generics(&input.generics)?;

    //let ab_cls = AbstractClass::default();

    for item in &input.items {
        match item {
            TraitItem::Const(c) => {
                // let item_attrs = TraitItemAttrs::from_attributes(&c.attrs)?;

                // let mut cnst =

                // if item_attrs.skip.is_some() {
                //     continue;
                // }
            }
            TraitItem::Verbatim(..) => todo!(),
            TraitItem::Method(..) => todo!(),

            _ => warning::print_warning(
                "unknown trait item",
                "",
                "ts-bind does not support all trait items (only `const` and `fn` items).",
            )
            .unwrap(),
        };
    }

    // // TODO: replace with continuous alphabet?
    // let mut names = vec![];
    // for i in 0..inner_types.len() {
    //     names.push(format_ident!("{}", "A".repeat(i)));
    // }

    // quote! {
    //     impl<#(#names),*> #trait_path<#(#names),*> for ts_bind::IsGeneric {

    //     }
    // };

    let trait_ident = &input.ident;
    let struct_ident = format_ident!("{}{}", STRUCT_PREFIX, to_ts_ident(trait_ident));

    let class_name = match trait_attrs.rename {
        Some(n) => n,
        None => trait_ident.to_string(),
    };

    let mut decl = quote! {
        let mut cls = ts_bind::AbstractClass::new(#class_name);
    };

    for generic in &generic_bindings {
        let g_name = generic.name.to_string();

        decl.extend(quote! {
            let g = cls.generics.add_generic(#g_name);
        });

        let mut bounds = proc_macro2::TokenStream::default();
        for bound in &generic.bounds {
            let trait_bound_name = &bound.trait_name;

            let trait_bound_struct_name = get_trait_struct_name(trait_bound_name);

            dependencies.append_from_trait(trait_bound_name.clone());

            bounds.extend(quote! {
                let b = ts_bind::Bound::new(#trait_bound_struct_name);
                g.add_bound(b);
            });

            // for inner in &bound.inner_types {
            //     match inner {
            //         InnerType::Generic(t) => {
            //             bounds.extend(quote! {
            //                 b.add_generic(#t);
            //             });
            //         }
            //         InnerType::Object(o) => {
            //             bounds.extend(quote! {
            //                 b.add_generic_type(#o);
            //             });
            //         }
            //     }
            // }
        }
        decl.extend(bounds);

        if let Some(def) = &generic.default {
            dependencies.append_from(&Type::Path(TypePath {
                qself: None,
                path: def.clone(),
            }));

            decl.extend(quote! {
                g.add_default(<#def as ts_bind::TS>::name());
            })
        }
    }

    decl.extend(quote! {
        cls.into()
    });

    let extra = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident;

        #input
    };

    Ok(DerivedTS {
        extra,
        name: class_name,
        decl,
        dependencies,
        export: trait_attrs.export,
        export_to: trait_attrs.export_to.clone(),
    }
    .into_impl(struct_ident, &Generics::default())) // traits generate their own structs which will never have generics
}

/// Given a path `x::y::Z` this returns `x::y::_ts_Z`
/// this wont work with paths that have generics `<...>`, but this assumes the path has been retuned by
/// `parse_generics` which returns the trait path without generics.
pub(crate) fn get_trait_struct_name(ident: &Path) -> Path {
    let len = ident.segments.len() - 1;
    let mut ident = ident.clone();

    let trait_name = ident.segments.last().unwrap();
    let trait_name = format_ident!("{}{}", STRUCT_PREFIX, trait_name.ident.to_string());

    ident.segments[len] = parse_quote!(#trait_name);

    ident
}
