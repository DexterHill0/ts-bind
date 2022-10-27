use darling::FromAttributes;
use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated,
    token::{Add, Colon2},
    GenericArgument, GenericParam, Generics, Path, PathArguments, PathSegment, Result, Type,
    TypeParamBound, WherePredicate,
};

use crate::utils::warning;

/// Attributes that can be applied to the trait bound of a generics clause.
#[derive(FromAttributes)]
#[darling(attributes(ts))]
pub(crate) struct GenericBoundAttrs {
    /// Skips the entire generic
    pub skip: Option<bool>,

    /// Ignores the `: Trait` bound
    pub ignore_bounds: Option<bool>,
    /// Ignores the `= Struct` default.
    pub ignore_default: Option<bool>,
    // The last two are required as attributes can only be applied to the generic ident,
    // and not anywhere after (`<### T ***, ...>` attribute allowed in ### but not in ***)
}

pub(crate) struct ParsedGenericBound {
    pub trait_name: Path,
    pub inner_types: Vec<InnerType>,
}

pub(crate) struct ParsedGeneric {
    pub name: Ident,
    pub bounds: Vec<ParsedGenericBound>,
    pub default: Option<Path>,
}

pub(crate) type ParsedGenerics = Vec<ParsedGeneric>;

pub(crate) fn parse_generics(generics: &Generics) -> Result<ParsedGenerics> {
    // <T>
    // <T: Trait>
    // <T: Trait = Struct>
    // #[ts(skip)]
    // #[ts(ignore_bound)]
    // #[ts(ignore_default)]

    // keeps track of all known defined generics
    let mut known_generics = vec![];
    // stores the inner generic to save matching against `GenericParam` again
    let mut parsed_generics = vec![];

    for param in &generics.params {
        match param {
            // `T` ...
            GenericParam::Type(t) => {
                known_generics.push(&t.ident);
                parsed_generics.push(t);
            },

            _ => warning::print_warning(
                "unknown generic parameter",
                "",
                "ts-bind does not support all generics parameters (no lifetimes or const generics).",
            )
            .unwrap(),
        }
    }

    let mut final_generics = vec![];

    for g in parsed_generics {
        let param_attrs = GenericBoundAttrs::from_attributes(&g.attrs)?;

        let mut parsed = ParsedGeneric {
            name: g.ident.clone(),
            bounds: vec![],
            default: None,
        };

        if param_attrs.skip.is_some() {
            continue;
        }

        if param_attrs.ignore_bounds.is_none() {
            // `T: A`, `T: B` ...
            parse_generic_bounds(&mut parsed, &known_generics, &g.bounds);
        }

        if param_attrs.ignore_default.is_none() {
            if let Some(def) = &g.default {
                match def {
                    Type::Path(p) => parsed.default = Some(p.path.clone()),

                    _ => warning::print_warning(
                        "unknown trait bound",
                        "lifetime",
                        "ts-bind does not support lifetime trait bounds.",
                    )
                    .unwrap(),
                }
            }
        }

        // where clauses cannot have attributes so we dont need to check that
        if let Some(whre) = &generics.where_clause {
            for predicate in &whre.predicates {
                match predicate {
                    WherePredicate::Type(t) => {
                        parse_generic_bounds(&mut parsed, &known_generics, &t.bounds);
                    }

                    _ => warning::print_warning(
                        "unknown where predicate",
                        "",
                        "ts-bind only supports type predicates.",
                    )
                    .unwrap(),
                }
            }
        }

        final_generics.push(parsed);
    }

    Ok(final_generics)
}

/// Parses the generics bounds of `T: A + B + C`
/// (`A + B + C`)
fn parse_generic_bounds(
    parsed: &mut ParsedGeneric,
    known_generics: &[&Ident],
    bounds: &Punctuated<TypeParamBound, Add>,
) {
    for bound in bounds {
        match bound {
            // `A`, `B` ...
            TypeParamBound::Trait(tb) => {
                let (trait_name, inner_types) = parse_path(&tb.path, known_generics);

                parsed.bounds.push(ParsedGenericBound {
                    trait_name,
                    inner_types,
                })
            }

            _ => warning::print_warning(
                "unknown trait bound",
                "lifetime",
                "ts-bind does not support lifetime trait bounds.",
            )
            .unwrap(),
        }
    }
}

pub(crate) enum InnerType {
    Generic(Ident),
    Object(Path),
}

// parses the path returning the trait and the generics
// `x::y::Trait<T, Vec<T>>` returns
// (x::y::Trait, vec![InnerType::Generic(T), InnerType::Object(Vec<T>)])
fn parse_path(path: &Path, known_generics: &[&Ident]) -> (Path, Vec<InnerType>) {
    let mut trait_path = path.clone();

    let mut inner_types = vec![];

    for (i, segment) in path.segments.iter().enumerate() {
        // clear the arguments of every segment so it is a plain path
        trait_path.segments[i].arguments = PathArguments::default();

        // as soon as we encounter generics on the trait
        // we want to parse those generics
        if let PathArguments::AngleBracketed(ab) = &segment.arguments {
            // `<A, B, ...>`
            for arg in &ab.args {
                match arg {
                    // `A` ...
                    GenericArgument::Type(t) => match t {
                        Type::Path(path) => {
                            // `qself` is the fully qualified syntax. not sure how that would turn into typescript bindings so...
                            // it wont :)
                            if path.qself.is_some() {
                                warning::print_warning(
                                    "unsupported generic path",
                                    "qualified path",
                                    "ts-bind does not support fully qualified paths (`<A as B>::`)",
                                )
                                .unwrap()
                            }

                            // generic `T` or trait name `Trait` or path `x::y::Trait`
                            if let Some(name) = path.path.get_ident() {
                                // generic `T` or trait name `Trait`
                                if known_generics.contains(&name) {
                                    // single generic
                                    inner_types.push(InnerType::Generic(name.clone()));

                                    continue;
                                }
                            }

                            // trait name `Trait` or path `x::y::Trait`
                            inner_types.push(InnerType::Object(path.path.clone()));
                        }

                        _ => warning::print_warning(
                            "unsupported generic argument type",
                            "",
                            "ts-bind only supports paths in generic arguments",
                        )
                        .unwrap(),
                    },

                    _ => warning::print_warning(
                        "unsupported generic argument type",
                        "",
                        "ts-bind only supports types in generic arguments",
                    )
                    .unwrap(),
                }
            }

            // once we have parsed the generics we dont want to continue looping
            // as there is nothing more to be added to either the trait path or the inner types
            break;
        }
        // else if path.segments.len() > 1 {
        //     // if we dont have generics yet, we are probably still parsing the path of the trait so continue to add `::`
        //     trait_path.push_str("::");
        // }
    }

    (trait_path, inner_types)
}