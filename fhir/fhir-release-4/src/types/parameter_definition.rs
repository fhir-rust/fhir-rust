//! ParameterDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ParameterDefinition
//!
//! Version: 4.0.1
//!
//! Definition of a parameter to a module
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ParameterDefinition Type: The parameters to
/// the module. This collection specifies both the input and output parameters.
/// Input parameters are provided by the caller as part of the $evaluate
/// operation. Output parameters are included in the GuidanceResponse.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::parameter_definition::ParameterDefinition;
/// use fhir::r4::types;
///
/// let value = ParameterDefinition {
///     name: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("final"));
///
/// let back: ParameterDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ParameterDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Name used to access the parameter value
    pub name: Option<types::Code>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// in | out
    pub r#use: crate::coded::Coded<crate::r4::codes::OperationParameterUse>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Minimum cardinality
    pub min: Option<types::Integer>,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum cardinality (a number of *)
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// A brief description of the parameter
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What type of value
    pub r#type: types::Code,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// What profile the value is expected to be
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ParameterDefinition;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
