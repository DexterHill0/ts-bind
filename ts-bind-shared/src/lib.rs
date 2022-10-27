pub mod generics;
pub mod types;

use std::any::TypeId;

pub use crate::generics::*;
pub use crate::types::*;

/// A typescript type which is depended upon by other types.
/// This information is required for generating the correct import statements.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    /// Type ID of the rust type
    pub type_id: TypeId,
    /// Name of the type in TypeScript
    pub ts_name: String,
    /// Path to where the type would be exported. By default a filename is derived from the types
    /// name, which can be customized with `#[ts(export_to = "..")]`.
    pub exported_to: &'static str,
}

impl Dependency {
    /// Constructs a [`Dependency`] from the given type `T`.
    /// If `T` is not exportable (meaning `T::EXPORT_TO` is `None`), this function will return
    /// `None`
    pub fn from_ty<T: TS>() -> Option<Self> {
        let exported_to = T::EXPORT_TO?;
        // Some(Dependency {
        //     type_id: TypeId::of::<T>(),
        //     ts_name: T::name(),
        //     exported_to,
        // })
        None
    }
}

/// TODO
pub trait TS {
    const EXPORT_TO: Option<&'static str> = None;

    fn name() -> String;

    fn decl(bindings: &mut Bindings) -> TSType;

    fn dependencies() -> Vec<Dependency>;

    // fn export() -> Result<(), ()>;
    // fn export_to(path: impl AsRef<Path>) -> Result<(), ()>;
    // fn export_to_string() -> Result<String, ()>;
}
