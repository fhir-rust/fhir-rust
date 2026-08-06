//! ResearchSubject
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
//!
//! Version: 4.0.1
//!
//! Physical entity which is the primary unit of interest in the study
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A physical entity which is the primary unit of operational and/or
/// administrative interest in a study.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::research_subject::ResearchSubject;
/// use fhir::r4::types;
///
/// let value = ResearchSubject {
///     assigned_arm: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `assignedArm` is the name this serializes to on the wire.
/// assert_eq!(json["assignedArm"], ::serde_json::json!("abc"));
///
/// let back: ResearchSubject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ResearchSubject {
    /// Logical id of this artifact
    pub id: Option<types::String>,

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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for research subject in a study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// candidate | eligible | follow-up | ineligible | not-registered |
    /// off-study | on-study | on-study-intervention | on-study-observation |
    /// pending-on-study | potential-candidate | screening | withdrawn
    pub status: crate::coded::Coded<crate::r4::codes::ResearchSubjectStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Start and end of participation
    pub period: Option<types::Period>,

    /// Study subject is part of
    pub study: types::Reference,

    /// Who is part of study
    pub individual: types::Reference,

    /// What path should be followed
    pub assigned_arm: Option<types::String>,
    /// Primitive extension sibling for [`assigned_arm`](Self::assigned_arm) (FHIR `_assignedArm`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assignedArm")]
    pub assigned_arm_ext: Option<types::Element>,

    /// What path was followed
    pub actual_arm: Option<types::String>,
    /// Primitive extension sibling for [`actual_arm`](Self::actual_arm) (FHIR `_actualArm`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actualArm")]
    pub actual_arm_ext: Option<types::Element>,

    /// Agreement to participate in study
    pub consent: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchSubject;

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
