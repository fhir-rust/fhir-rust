//! CodeSystem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeSystem
//!
//! Version: 5.0.0
//!
//! CodeSystem Resource: The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The CodeSystem resource is used to declare the existence of and describe a
/// code system or code system supplement and its key properties, and optionally
/// define a part or all of its content. In FHIR R5 it is the canonical way to
/// publish terminologies, whether small local code lists or large external
/// systems, and it underpins ValueSet expansion and terminology operations. It
/// captures metadata such as the canonical URL, versioning, filters, and
/// properties, along with an optional hierarchy of concepts and their
/// designations.
///
/// Administratively, a CodeSystem is typically authored and maintained by a
/// standards body, vendor, or local implementer, and is published with a
/// lifecycle status (draft, active, or retired) and a content mode indicating
/// whether the resource is a complete, partial, example, or supplemental
/// representation of the underlying code system. Consumers reference a
/// CodeSystem by its canonical URL and optional version, and terminology
/// servers use it to answer `$lookup`, `$validate-code`, and `$subsumes`
/// operations. Individual codes defined here are commonly bound into other
/// resources through [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// or `Coding` elements, for example when coding a condition, observation, or
/// medication on a [`Patient`](crate::r5::resources::patient::Patient) record.
///
/// See also: `ValueSet`, which selects and constrains codes drawn from one or
/// more CodeSystem resources for use in a particular context.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystem;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this code system, represented as a URI (globally unique) (Coding.system). This is the value used to populate `Coding.system` when referencing codes from this system.
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the code system (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the code system (Coding.version)
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `CodeSystem.versionAlgorithm[x]` choice element (0..1); see [`CodeSystemVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CodeSystemVersionAlgorithm>,

    /// Name for this code system (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this code system (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown; governs the publication lifecycle of this code system.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the code system
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the CodeSystem was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the CodeSystem was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
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
    /// Primitive extension sibling for [`case_sensitive`](Self::case_sensitive) (FHIR `_caseSensitive`).
    #[serde(rename = "_caseSensitive")]
    pub case_sensitive_ext: Option<types::Element>,

    /// Canonical reference to the value set with entire code system
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// grouped-by | is-a | part-of | classified-with
    pub hierarchy_meaning:
        Option<crate::r5::coded::Coded<crate::r5::codes::CodesystemHierarchyMeaning>>,
    /// Primitive extension sibling for [`hierarchy_meaning`](Self::hierarchy_meaning) (FHIR `_hierarchyMeaning`).
    #[serde(rename = "_hierarchyMeaning")]
    pub hierarchy_meaning_ext: Option<types::Element>,

    /// If code system defines a compositional grammar
    pub compositional: Option<types::Boolean>,
    /// Primitive extension sibling for [`compositional`](Self::compositional) (FHIR `_compositional`).
    #[serde(rename = "_compositional")]
    pub compositional_ext: Option<types::Element>,

    /// If definitions are not stable
    pub version_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`version_needed`](Self::version_needed) (FHIR `_versionNeeded`).
    #[serde(rename = "_versionNeeded")]
    pub version_needed_ext: Option<types::Element>,

    /// not-present | example | fragment | complete | supplement; indicates how much of the underlying code system's content is represented by this resource.
    pub content: crate::r5::coded::Coded<crate::r5::codes::CodesystemContentMode>,
    /// Primitive extension sibling for [`content`](Self::content) (FHIR `_content`).
    #[serde(rename = "_content")]
    pub content_ext: Option<types::Element>,

    /// Canonical URL of Code System this adds designations and properties to
    pub supplements: Option<types::Canonical>,
    /// Primitive extension sibling for [`supplements`](Self::supplements) (FHIR `_supplements`).
    #[serde(rename = "_supplements")]
    pub supplements_ext: Option<types::Element>,

    /// Total concepts in the code system
    pub count: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`).
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// Filter that can be used in a value set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<CodeSystemFilter>,

    /// Additional information supplied about each concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<CodeSystemProperty>,

    /// Concepts in the code system; the (optionally hierarchical) list of codes this resource defines or supplements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<CodeSystemConcept>,
}

/// Filter that can be used in a value set
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::code_system::CodeSystemFilter;
///
/// let value = CodeSystemFilter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CodeSystemFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// How or why the filter is used
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    pub operator: vec1::Vec1<crate::r5::coded::Coded<crate::r5::codes::FilterOperator>>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`).
    #[serde(rename = "_operator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operator_ext: Vec<Option<types::Element>>,

    /// What to use for the value
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Additional information supplied about each concept
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystemProperty;
/// use fhir::r5::types;
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
pub struct CodeSystemProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the property on the concepts, and when referred to in operations
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// code | Coding | string | integer | boolean | dateTime | decimal
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ConceptPropertyType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// Concepts in the code system
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystemConcept;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Text to display to the user
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Formal definition
    pub definition: Option<types::String>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
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

/// Additional representations for the concept
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystemConceptDesignation;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Details how this designation would be used
    pub r#use: Option<types::Coding>,

    /// Additional ways how this designation would be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_use: Vec<types::Coding>,

    /// The text value for this designation
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Property value for the concept
/// # Examples
///
/// ```
/// use fhir::r5::resources::code_system::CodeSystemConceptProperty;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The `CodeSystem.concept.property.value[x]` choice element (0..1); see [`CodeSystemConceptPropertyValue`].
    #[serde(flatten)]
    pub value: Option<CodeSystemConceptPropertyValue>,
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
/// The `CodeSystem.concept.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CodeSystemConceptPropertyValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
}

/// The `CodeSystem.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CodeSystemVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
