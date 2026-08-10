//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//!
//!
//! Insurance or medical plan
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Coverage Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::coverage::Coverage;
/// use fhir::r2::types;
///
/// let value = Coverage {
///     sub_plan: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `subPlan` is the name this serializes to on the wire.
/// assert_eq!(json["subPlan"], ::serde_json::json!("abc"));
///
/// let back: Coverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Coverage {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for the plan issuer
    pub issuer: Option<types::Reference<crate::r2::resources::Organization>>,

    /// BIN Number
    pub bin: Option<types::Identifier>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Type of coverage
    pub r#type: Option<types::Coding>,

    /// Subscriber ID
    pub subscriber_id: Option<types::Identifier>,

    /// The primary coverage ID
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An identifier for the group
    pub group: Option<types::String>,
    /// Primitive extension sibling for [`group`](Self::group) (FHIR `_group`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_group")]
    pub group_ext: Option<types::Element>,

    /// An identifier for the plan
    pub plan: Option<types::String>,
    /// Primitive extension sibling for [`plan`](Self::plan) (FHIR `_plan`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plan")]
    pub plan_ext: Option<types::Element>,

    /// An identifier for the subsection of the plan
    pub sub_plan: Option<types::String>,
    /// Primitive extension sibling for [`sub_plan`](Self::sub_plan) (FHIR `_subPlan`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subPlan")]
    pub sub_plan_ext: Option<types::Element>,

    /// The dependent number
    pub dependent: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`dependent`](Self::dependent) (FHIR `_dependent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dependent")]
    pub dependent_ext: Option<types::Element>,

    /// The plan instance or sequence counter
    pub sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Plan holder information
    pub subscriber: Option<types::Reference<crate::r2::resources::Patient>>,

    /// Insurer network
    pub network: Option<types::Identifier>,

    /// Contract details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<types::Reference<crate::r2::resources::Contract>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coverage;

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
