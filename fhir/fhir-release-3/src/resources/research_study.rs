//! ResearchStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
//!
//!
//!
//! Investigation to increase healthcare-related patient-independent knowledge
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ResearchStudy Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::research_study::ResearchStudy;
/// use fhir::r3::types;
///
/// let value = ResearchStudy {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ResearchStudy {
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Name for this study
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Reference>,

    /// Part of larger study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// draft | in-progress | suspended | stopped | completed |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::ResearchStudyStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classifications for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Drugs, devices, conditions, etc. under study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::CodeableConcept>,

    /// Contact details for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// References and dependencies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Used to search for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<types::CodeableConcept>,

    /// Geographic region(s) for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// What this is study doing
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Inclusion & exclusion criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment: Vec<types::Reference>,

    /// When the study began and ended
    pub period: Option<types::Period>,

    /// Organization responsible for the study
    pub sponsor: Option<types::Reference>,

    /// The individual responsible for the study
    pub principal_investigator: Option<types::Reference>,

    /// Location involved in study execution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<types::Reference>,

    /// Reason for terminating study early
    pub reason_stopped: Option<types::CodeableConcept>,

    /// Comments made about the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Defined path through the study for a subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arm: Vec<ResearchStudyArm>,
}

/// Describes an expected sequence of events for one of the participants of a
/// study. E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out,
/// follow-up.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::research_study::ResearchStudyArm;
/// use fhir::r3::types;
///
/// let value = ResearchStudyArm {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyArm = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ResearchStudyArm {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for study arm
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Categorization of study arm
    pub code: Option<types::CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchStudy;

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
