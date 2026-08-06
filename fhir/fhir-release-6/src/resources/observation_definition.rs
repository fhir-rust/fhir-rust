//! ObservationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Definition of an observation
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Set of definitional characteristics for a kind of observation or
/// measurement produced or consumed by an orderable health care service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation_definition::ObservationDefinition;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ObservationDefinition {
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

    /// Logical canonical URL to reference this ObservationDefinition (globally
    /// unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier of the ObservationDefinition
    pub identifier: Option<types::Identifier>,

    /// Business version of the ObservationDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `ObservationDefinition.versionAlgorithm[x]` choice element (0..1); see [`ObservationDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ObservationDefinitionVersionAlgorithm>,

    /// Name for this ObservationDefinition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this ObservationDefinition (human friendly)
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

    /// If For testing only - never for real usage
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

    /// The name of the individual or organization that published the
    /// ObservationDefinition
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the ObservationDefinition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// When ObservationDefinition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// Date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the ObservationDefinition
    pub effective_period: Option<types::Period>,

    /// Based on FHIR definition of another observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from_canonical`](Self::derived_from_canonical) (FHIR `_derivedFromCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFromCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical_ext: Vec<Option<types::Element>>,

    /// Based on external definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`derived_from_uri`](Self::derived_from_uri) (FHIR `_derivedFromUri`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Type of observation
    pub code: types::CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio
    /// | SampledData | time | dateTime | Period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type: Vec<crate::coded::Coded<crate::r6::codes::PermittedDataType>>,
    /// Primitive extension sibling for [`permitted_data_type`](Self::permitted_data_type) (FHIR `_permittedDataType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_permittedDataType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type_ext: Vec<Option<types::Element>>,

    /// Multiple results allowed for conforming observations
    pub multiple_results_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_results_allowed`](Self::multiple_results_allowed) (FHIR `_multipleResultsAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
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
    /// Primitive extension sibling for [`preferred_report_name`](Self::preferred_report_name) (FHIR `_preferredReportName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferredReportName")]
    pub preferred_report_name_ext: Option<types::Element>,

    /// Unit for quantitative results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_unit: Vec<types::Coding>,

    /// Set of qualified values for observation results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_value: Vec<ObservationDefinitionQualifiedValue>,

    /// Definitions of related resources belonging to this kind of observation
    /// group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_member: Vec<types::Reference>,

    /// Component results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ObservationDefinitionComponent>,
}

/// Some observations have multiple component observations, expressed as
/// separate code value pairs.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation_definition::ObservationDefinitionComponent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio
    /// | SampledData | time | dateTime | Period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type: Vec<crate::coded::Coded<crate::r6::codes::PermittedDataType>>,
    /// Primitive extension sibling for [`permitted_data_type`](Self::permitted_data_type) (FHIR `_permittedDataType`):
    /// carries `id` and/or `extension` for the primitive value.
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

/// A set of qualified values associated with a context and a set of conditions
/// \- provides a range for quantitative and ordinal observations and a
/// collection of value sets for qualitative observations.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation_definition::ObservationDefinitionQualifiedValue;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Applicable age range for the set of qualified values
    pub age: Option<types::Range>,

    /// Applicable gestational age range for the set of qualified values
    pub gestational_age: Option<types::Range>,

    /// Condition associated with the set of qualified values
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// reference | critical | absolute
    pub range_category: Option<crate::coded::Coded<crate::r6::codes::ObservationRangeCategory>>,
    /// Primitive extension sibling for [`range_category`](Self::range_category) (FHIR `_rangeCategory`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rangeCategory")]
    pub range_category_ext: Option<types::Element>,

    /// The range for continuous or ordinal observations
    pub range: Option<types::Range>,

    /// Value set of valid coded values as part of this set of qualified values
    pub valid_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`valid_coded_value_set`](Self::valid_coded_value_set) (FHIR `_validCodedValueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_validCodedValueSet")]
    pub valid_coded_value_set_ext: Option<types::Element>,

    /// Value set of normal coded values as part of this set of qualified
    /// values
    pub normal_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`normal_coded_value_set`](Self::normal_coded_value_set) (FHIR `_normalCodedValueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_normalCodedValueSet")]
    pub normal_coded_value_set_ext: Option<types::Element>,

    /// Value set of abnormal coded values as part of this set of qualified
    /// values
    pub abnormal_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`abnormal_coded_value_set`](Self::abnormal_coded_value_set) (FHIR `_abnormalCodedValueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_abnormalCodedValueSet")]
    pub abnormal_coded_value_set_ext: Option<types::Element>,

    /// Value set of critical coded values as part of this set of qualified
    /// values
    pub critical_coded_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`critical_coded_value_set`](Self::critical_coded_value_set) (FHIR `_criticalCodedValueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criticalCodedValueSet")]
    pub critical_coded_value_set_ext: Option<types::Element>,

    /// Expected coded interpretation values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpretation: Vec<types::CodeableConcept>,
}

/// The `ObservationDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
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
