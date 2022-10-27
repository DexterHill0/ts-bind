use std::path::Path;

pub use ts_bind_macros::{ts, TS};
pub use ts_bind_shared::*;

// pub trait TS {
//     const EXPORT_TO: Option<&'static str> = None;

//     fn name() -> String;

//     fn
//     // fn dependencies() -> Vec<Dependency>;
//     // fn transparent() -> bool;

//     // fn decl() -> String { ... }
//     // fn name_with_type_args(args: Vec<String>) -> String { ... }
//     // fn inline() -> String { ... }
//     // fn inline_flattened() -> String { ... }
//     // fn export() -> Result<(), ExportError> { ... }
//     // fn export_to(path: impl AsRef<Path>) -> Result<(), ExportError> { ... }
//     // fn export_to_string() -> Result<String, ExportError> { ... }
// }
