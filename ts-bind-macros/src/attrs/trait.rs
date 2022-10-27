use darling::{FromAttributes, FromMeta};

use crate::attrs::Inflection;

/// Attributes that van be applied to the trait definition.
#[derive(Default, Clone, Debug, FromMeta)]
pub(crate) struct TraitAttrs {
    pub rename: Option<String>,
    pub export_to: Option<String>,
    pub export: Option<bool>,
}

/// Any attributes that can be applied to items within the trait.
#[derive(Default, Clone, Debug, FromAttributes)]
#[darling(attributes(ts))]
pub(crate) struct TraitItemAttrs {
    pub rename: Option<String>,
    pub skip: Option<bool>,

    #[darling(rename = "type")]
    pub type_override: Option<String>,

    pub no_abstract: Option<bool>,
}
