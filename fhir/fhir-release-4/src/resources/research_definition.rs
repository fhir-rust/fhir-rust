//! ResearchDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchDefinition
//!
//! Version: 4.0.1
//!
//! A research context or question
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The ResearchDefinition resource describes the conditional state (population
/// and any exposures being compared within the population) and outcome (if
/// specified) that the knowledge (evidence, assertion, recommendation) is
/// about.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::research_definition::ResearchDefinition;
/// use fhir::r4::types;
///
/// let value = ResearchDefinition {
///     short_title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `shortTitle` is the name this serializes to on the wire.
/// assert_eq!(json["shortTitle"], ::serde_json::json!("abc"));
///
/// let back: ResearchDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ResearchDefinition {
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

    /// Canonical identifier for this research definition, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the research definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the research definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this research definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this research definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Title for use in informal contexts
    pub short_title: Option<types::String>,
    /// Primitive extension sibling for [`short_title`](Self::short_title) (FHIR `_shortTitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_shortTitle")]
    pub short_title_ext: Option<types::Element>,

    /// Subordinate title of the ResearchDefinition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location,
    /// Device
    /// The `ResearchDefinition.subject[x]` choice element (0..1); see [`ResearchDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<ResearchDefinitionSubject>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the research definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment_ext: Vec<Option<types::Element>>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for research definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this research definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the ResearchDefinition
    pub usage: Option<types::String>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the research definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the research definition was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the research definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the ResearchDefinition, such as Education, Treatment,
    /// Assessment, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the ResearchDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// What population?
    pub population: types::Reference,

    /// What exposure?
    pub exposure: Option<types::Reference>,

    /// What alternative exposure state?
    pub exposure_alternative: Option<types::Reference>,

    /// What outcome?
    pub outcome: Option<types::Reference>,
}

/// The `ResearchDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchDefinition;

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
