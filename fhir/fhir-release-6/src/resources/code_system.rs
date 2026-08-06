//! CodeSystem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeSystem
//!
//! Version: 6.0.0-ballot3
//!
//! Declares the existence of and describes a code system or code system
//! supplement
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The CodeSystem resource is used to declare the existence of and describe a
/// code system or code system supplement and its key properties, and
/// optionally define a part or all of its content.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::code_system::CodeSystem;
/// use fhir::r6::types;
///
/// let value = CodeSystem {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: CodeSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystem {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this code system, represented as a URI
    /// (globally unique) (Coding.system)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the code system (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the code system (Coding.version)
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `CodeSystem.versionAlgorithm[x]` choice element (0..1); see [`CodeSystemVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CodeSystemVersionAlgorithm>,

    /// Name for this code system (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this code system (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the code system
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for code system (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this code system is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the CodeSystem was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the CodeSystem was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the CodeSystem is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the CodeSystem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the CodeSystem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the CodeSystem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the CodeSystem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// If code comparison is case sensitive
    pub case_sensitive: Option<types::Boolean>,
    /// Primitive extension sibling for [`case_sensitive`](Self::case_sensitive) (FHIR `_caseSensitive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_caseSensitive")]
    pub case_sensitive_ext: Option<types::Element>,

    /// Canonical reference to the value set with entire code system
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// grouped-by | is-a | part-of | classified-with
    pub hierarchy_meaning:
        Option<crate::coded::Coded<crate::r6::codes::CodesystemHierarchyMeaning>>,
    /// Primitive extension sibling for [`hierarchy_meaning`](Self::hierarchy_meaning) (FHIR `_hierarchyMeaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hierarchyMeaning")]
    pub hierarchy_meaning_ext: Option<types::Element>,

    /// If code system defines a compositional grammar
    pub compositional: Option<types::Boolean>,
    /// Primitive extension sibling for [`compositional`](Self::compositional) (FHIR `_compositional`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compositional")]
    pub compositional_ext: Option<types::Element>,

    /// If definitions are not stable
    pub version_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`version_needed`](Self::version_needed) (FHIR `_versionNeeded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versionNeeded")]
    pub version_needed_ext: Option<types::Element>,

    /// not-present | example | fragment | complete | supplement
    pub content: crate::coded::Coded<crate::r6::codes::CodesystemContentMode>,
    /// Primitive extension sibling for [`content`](Self::content) (FHIR `_content`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_content")]
    pub content_ext: Option<types::Element>,

    /// Canonical URL of Code System this adds designations and properties to
    pub supplements: Option<types::Canonical>,
    /// Primitive extension sibling for [`supplements`](Self::supplements) (FHIR `_supplements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_supplements")]
    pub supplements_ext: Option<types::Element>,

    /// Total concepts in the code system
    pub count: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// Filter that can be used in a value set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<CodeSystemFilter>,

    /// Additional information supplied about each concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<CodeSystemProperty>,

    /// Concepts in the code system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<CodeSystemConcept>,
}

/// Concepts that are in the code system. The concept definitions are
/// inherently hierarchical, but the definitions must be consulted to determine
/// what the meanings of the hierarchical relationships are.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::code_system::CodeSystemConcept;
/// use fhir::r6::types;
///
/// let value = CodeSystemConcept {
///     display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!("abc"));
///
/// let back: CodeSystemConcept = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystemConcept {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies concept
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Text to display to the user
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Formal definition
    pub definition: Option<types::String>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Additional representations for the concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<CodeSystemConceptDesignation>,

    /// Property value for the concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<CodeSystemConceptProperty>,

    /// Child Concepts (is-a/contains/categorizes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<CodeSystemConcept>,
}

/// Additional representations for the concept - other languages, aliases,
/// specialized purposes, used for particular purposes, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::code_system::CodeSystemConceptDesignation;
/// use fhir::r6::types;
///
/// let value = CodeSystemConceptDesignation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CodeSystemConceptDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystemConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human language of the designation
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Details how this designation would be used
    pub r#use: Option<types::Coding>,

    /// Additional ways how this designation would be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_use: Vec<types::Coding>,

    /// The text value for this designation
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A property value for this concept.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::code_system::CodeSystemConceptProperty;
/// use fhir::r6::types;
///
/// let value = CodeSystemConceptProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CodeSystemConceptProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystemConceptProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to CodeSystem.property.code
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Value of the property for this concept
    /// The `CodeSystem.concept.property.value[x]` choice element (1..1); see [`CodeSystemConceptPropertyValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<CodeSystemConceptPropertyValue>,
}

/// A filter that can be used in a value set compose statement when selecting
/// concepts using a filter.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::code_system::CodeSystemFilter;
///
/// let value = CodeSystemFilter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CodeSystemFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystemFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies the filter
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// How or why the filter is used
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// \= | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes
    /// | child-of | descendent-leaf | exists
    pub operator: ::vec1::Vec1<crate::coded::Coded<crate::r6::codes::FilterOperator>>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_operator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operator_ext: Vec<Option<types::Element>>,

    /// What to use for the value
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A property defines an additional slot through which additional information
/// can be provided about a concept.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::code_system::CodeSystemProperty;
/// use fhir::r6::types;
///
/// let value = CodeSystemProperty {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: CodeSystemProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CodeSystemProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the property on the concepts, and when referred to in
    /// operations
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// code | Coding | string | integer | boolean | dateTime | decimal
    pub r#type: crate::coded::Coded<crate::r6::codes::ConceptPropertyType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// The `CodeSystem.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CodeSystemVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `CodeSystem.concept.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CodeSystemConceptPropertyValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CodeSystem;

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
