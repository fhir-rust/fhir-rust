//! ObservationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
//!
//! Version: 4.3.0
//!
//! Definition of an observation
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Set of definitional characteristics for a kind of observation or
/// measurement produced or consumed by an orderable health care service.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::observation_definition::ObservationDefinition;
/// use fhir::r4b::types;
///
/// let value = ObservationDefinition {
///     multiple_results_allowed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `multipleResultsAllowed` is the name this serializes to on the wire.
/// assert_eq!(json["multipleResultsAllowed"], ::serde_json::json!(true));
///
/// let back: ObservationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of observation (code / type)
    pub code: types::CodeableConcept,

    /// Business identifier for this ObservationDefinition instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio
    /// | SampledData | time | dateTime | Period
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub permitted_data_type:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r4b::codes::PermittedDataType>>,
    /// Primitive extension sibling for [`permitted_data_type`](Self::permitted_data_type) (FHIR `_permittedDataType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_permittedDataType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted_data_type_ext: Vec<Option<types::Element>>,

    /// Multiple results allowed
    pub multiple_results_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_results_allowed`](Self::multiple_results_allowed) (FHIR `_multipleResultsAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_multipleResultsAllowed")]
    pub multiple_results_allowed_ext: Option<types::Element>,

    /// Method used to produce the observation
    pub method: Option<types::CodeableConcept>,

    /// Preferred report name
    pub preferred_report_name: Option<types::String>,
    /// Primitive extension sibling for [`preferred_report_name`](Self::preferred_report_name) (FHIR `_preferredReportName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferredReportName")]
    pub preferred_report_name_ext: Option<types::Element>,

    /// Characteristics of quantitative results
    pub quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,

    /// Qualified range for continuous and ordinal observation results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_interval: Vec<ObservationDefinitionQualifiedInterval>,

    /// Value set of valid coded values for the observations conforming to this
    /// ObservationDefinition
    pub valid_coded_value_set: Option<types::Reference<crate::r4b::resources::ValueSet>>,

    /// Value set of normal coded values for the observations conforming to
    /// this ObservationDefinition
    pub normal_coded_value_set: Option<types::Reference<crate::r4b::resources::ValueSet>>,

    /// Value set of abnormal coded values for the observations conforming to
    /// this ObservationDefinition
    pub abnormal_coded_value_set: Option<types::Reference<crate::r4b::resources::ValueSet>>,

    /// Value set of critical coded values for the observations conforming to
    /// this ObservationDefinition
    pub critical_coded_value_set: Option<types::Reference<crate::r4b::resources::ValueSet>>,
}

/// Multiple ranges of results qualified by different contexts for ordinal or
/// continuous observations conforming to this ObservationDefinition.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::observation_definition::ObservationDefinitionQualifiedInterval;
/// use fhir::r4b::types;
///
/// let value = ObservationDefinitionQualifiedInterval {
///     condition: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `condition` is the name this serializes to on the wire.
/// assert_eq!(json["condition"], ::serde_json::json!("abc"));
///
/// let back: ObservationDefinitionQualifiedInterval = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ObservationDefinitionQualifiedInterval {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// reference | critical | absolute
    pub category: Option<crate::coded::Coded<crate::r4b::codes::ObservationRangeCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// The interval itself, for continuous or ordinal observations
    pub range: Option<types::Range>,

    /// Range context qualifier
    pub context: Option<types::CodeableConcept>,

    /// Targetted population of the range
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applies_to: Vec<types::CodeableConcept>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r4b::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Applicable age range, if relevant
    pub age: Option<types::Range>,

    /// Applicable gestational age range, if relevant
    pub gestational_age: Option<types::Range>,

    /// Condition associated with the reference range
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,
}

/// Characteristics for quantitative results of this observation.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::observation_definition::ObservationDefinitionQuantitativeDetails;
/// use fhir::r4b::types;
///
/// let value = ObservationDefinitionQuantitativeDetails {
///     decimal_precision: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `decimalPrecision` is the name this serializes to on the wire.
/// assert_eq!(json["decimalPrecision"], ::serde_json::json!(42));
///
/// let back: ObservationDefinitionQuantitativeDetails = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ObservationDefinitionQuantitativeDetails {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Customary unit for quantitative results
    pub customary_unit: Option<types::CodeableConcept>,

    /// SI unit for quantitative results
    pub unit: Option<types::CodeableConcept>,

    /// SI to Customary unit conversion factor
    pub conversion_factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`conversion_factor`](Self::conversion_factor) (FHIR `_conversionFactor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conversionFactor")]
    pub conversion_factor_ext: Option<types::Element>,

    /// Decimal precision of observation quantitative results
    pub decimal_precision: Option<types::Integer>,
    /// Primitive extension sibling for [`decimal_precision`](Self::decimal_precision) (FHIR `_decimalPrecision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_decimalPrecision")]
    pub decimal_precision_ext: Option<types::Element>,
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
