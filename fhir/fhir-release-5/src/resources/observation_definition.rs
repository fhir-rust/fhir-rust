//! ObservationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
//!
//! Version: 5.0.0
//!
//! ObservationDefinition Resource: Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of definitional characteristics for a kind of observation or
/// measurement produced or consumed by an orderable health care service. An
/// ObservationDefinition describes the permitted data types, units, reference
/// ranges, qualified values, and component structure that conforming
/// observations are expected to follow. It is used to formally specify what a
/// laboratory or diagnostic service can produce or requires, supporting both
/// order catalogs and result validation in FHIR R5.
///
/// In practice, an ObservationDefinition acts as a reusable, canonical template
/// that governs a family of related observation instances rather than recording
/// any single patient result. Diagnostic laboratories, imaging services, and
/// device manufacturers publish these definitions to declare the code that
/// identifies the observation, the data types and units in which a result may
/// be expressed, and context-specific reference and critical ranges keyed to
/// population characteristics such as gender, age, and gestational age. Clinical
/// and terminology systems then consult the definition to build order catalogs,
/// pre-populate result forms, drive decision support, and validate that captured
/// results conform to the expected structure and permissible values. Being a
/// canonical, versioned resource, it can be referenced by URL, derived from
/// other definitions, and constrained for local use.
///
/// # Related resources
///
/// Actual results produced under this template are represented by the
/// [`Observation`](crate::r5::resources::observation::Observation) resource,
/// which is typically requested via a
/// [`ServiceRequest`](crate::r5::resources::service_request::ServiceRequest).
/// The specimen characteristics that such observations rely on can be described
/// by a [`SpecimenDefinition`](crate::r5::resources::specimen_definition::SpecimenDefinition).
/// Many descriptive fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) for coded content.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation_definition::ObservationDefinition;
/// use fhir::r5::types;
///
/// let value = ObservationDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ObservationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition {
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
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical canonical URL to reference this ObservationDefinition (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier of the ObservationDefinition
    pub identifier: Option<types::Identifier>,

    /// Business version of the ObservationDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ObservationDefinition.versionAlgorithm[x]` choice element (0..1); see [`ObservationDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ObservationDefinitionVersionAlgorithm>,

    /// Name for this ObservationDefinition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this ObservationDefinition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication lifecycle state of this definition: draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The name of the individual or organization that published the ObservationDefinition
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the ObservationDefinition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for this ObservationDefinition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this ObservationDefinition is defined
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

    /// When ObservationDefinition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// Date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the ObservationDefinition
    pub effective_period: Option<types::Period>,

    /// Based on FHIR definition of another observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from_canonical`](Self::derived_from_canonical) (FHIR `_derivedFromCanonical`).
    #[serde(rename = "_derivedFromCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical_ext: Vec<Option<types::Element>>,

    /// Based on external definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`derived_from_uri`](Self::derived_from_uri) (FHIR `_derivedFromUri`).
    #[serde(rename = "_derivedFromUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri_ext: Vec<Option<types::Element>>,

    /// Type of subject for the defined observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::CodeableConcept>,

    /// Desired kind of performer for such kind of observation
    pub performer_type: Option<types::CodeableConcept>,

    /// General type of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Coded concept identifying the kind of observation this definition describes.
    pub code: types::CodeableConcept,

    /// Permitted result value types, such as Quantity, CodeableConcept, string, boolean, integer, Range, Ratio, SampledData, time, dateTime, or Period.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type: Vec<crate::r5::coded::Coded<crate::r5::codes::PermittedDataType>>,
    /// Primitive extension sibling for [`permitted_data_type`](Self::permitted_data_type) (FHIR `_permittedDataType`).
    #[serde(rename = "_permittedDataType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type_ext: Vec<Option<types::Element>>,

    /// Whether more than one result is allowed for observations conforming to this definition.
    pub multiple_results_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_results_allowed`](Self::multiple_results_allowed) (FHIR `_multipleResultsAllowed`).
    #[serde(rename = "_multipleResultsAllowed")]
    pub multiple_results_allowed_ext: Option<types::Element>,

    /// Body part to be observed
    pub body_site: Option<types::CodeableConcept>,

    /// Method used to produce the observation
    pub method: Option<types::CodeableConcept>,

    /// Kind of specimen used by this type of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// Measurement device or model of device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference>,

    /// The preferred name to be used when reporting the observation results
    pub preferred_report_name: Option<types::String>,
    /// Primitive extension sibling for [`preferred_report_name`](Self::preferred_report_name) (FHIR `_preferredReportName`).
    #[serde(rename = "_preferredReportName")]
    pub preferred_report_name_ext: Option<types::Element>,

    /// Unit for quantitative results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_unit: Vec<types::Coding>,

    /// Context-specific reference, critical, and absolute ranges plus valid coded value sets for conforming results.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_value: Vec<ObservationDefinitionQualifiedValue>,

    /// Definitions of related resources belonging to this kind of observation group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_member: Vec<types::Reference>,

    /// Component observation definitions when this observation is composed of multiple parts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ObservationDefinitionComponent>,
}

/// Set of qualified values for observation results.
///
/// A qualified value bundles together a context (population, gender, age,
/// gestational age, condition) with the reference, critical, or absolute
/// ranges and the value sets of valid, normal, abnormal, and critical coded
/// values that apply to conforming observations under that context.
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation_definition::ObservationDefinitionQualifiedValue;
/// use fhir::r5::types;
///
/// let value = ObservationDefinitionQualifiedValue {
///     valid_coded_value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `validCodedValueSet` is the name this serializes to on the wire.
/// assert_eq!(json["validCodedValueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ObservationDefinitionQualifiedValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionQualifiedValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Context qualifier for the set of qualified values
    pub context: Option<types::CodeableConcept>,

    /// Targetted population for the set of qualified values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applies_to: Vec<types::CodeableConcept>,

    /// male | female | other | unknown
    pub gender: Option<crate::r5::coded::Coded<crate::r5::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Applicable age range for the set of qualified values
    pub age: Option<types::Range>,

    /// Applicable gestational age range for the set of qualified values
    pub gestational_age: Option<types::Range>,

    /// Condition associated with the set of qualified values
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`).
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// reference | critical | absolute
    pub range_category: Option<crate::r5::coded::Coded<crate::r5::codes::ObservationRangeCategory>>,
    /// Primitive extension sibling for [`range_category`](Self::range_category) (FHIR `_rangeCategory`).
    #[serde(rename = "_rangeCategory")]
    pub range_category_ext: Option<types::Element>,

    /// The range for continuous or ordinal observations
    pub range: Option<types::Range>,

    /// Value set of valid coded values as part of this set of qualified values
    pub valid_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`valid_coded_value_set`](Self::valid_coded_value_set) (FHIR `_validCodedValueSet`).
    #[serde(rename = "_validCodedValueSet")]
    pub valid_coded_value_set_ext: Option<types::Element>,

    /// Value set of normal coded values as part of this set of qualified values
    pub normal_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`normal_coded_value_set`](Self::normal_coded_value_set) (FHIR `_normalCodedValueSet`).
    #[serde(rename = "_normalCodedValueSet")]
    pub normal_coded_value_set_ext: Option<types::Element>,

    /// Value set of abnormal coded values as part of this set of qualified values
    pub abnormal_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`abnormal_coded_value_set`](Self::abnormal_coded_value_set) (FHIR `_abnormalCodedValueSet`).
    #[serde(rename = "_abnormalCodedValueSet")]
    pub abnormal_coded_value_set_ext: Option<types::Element>,

    /// Value set of critical coded values as part of this set of qualified values
    pub critical_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`critical_coded_value_set`](Self::critical_coded_value_set) (FHIR `_criticalCodedValueSet`).
    #[serde(rename = "_criticalCodedValueSet")]
    pub critical_coded_value_set_ext: Option<types::Element>,
}

/// Component results.
///
/// Describes a component observation that is part of a composite observation
/// definition, specifying its code, permitted data types and units, and the
/// set of qualified values that apply to the component's results.
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation_definition::ObservationDefinitionComponent;
/// use fhir::r5::types;
///
/// let value = ObservationDefinitionComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ObservationDefinitionComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of observation
    pub code: types::CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type: Vec<crate::r5::coded::Coded<crate::r5::codes::PermittedDataType>>,
    /// Primitive extension sibling for [`permitted_data_type`](Self::permitted_data_type) (FHIR `_permittedDataType`).
    #[serde(rename = "_permittedDataType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type_ext: Vec<Option<types::Element>>,

    /// Unit for quantitative results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_unit: Vec<types::Coding>,

    /// Set of qualified values for observation results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_value: Vec<ObservationDefinitionQualifiedValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ObservationDefinition;

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
/// The `ObservationDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ObservationDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
