//! Observation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Observation
//!
//!
//!
//! Measurements and simple assertions
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Observation Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::observation::Observation;
/// use fhir::r3::types;
///
/// let value = Observation {
///     issued: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `issued` is the name this serializes to on the wire.
/// assert_eq!(json["issued"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Observation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Observation {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// registered | preliminary | final | amended +
    pub status: crate::coded::Coded<crate::r3::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of type of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of observation (code / type)
    pub code: types::CodeableConcept,

    /// Who and/or what this is about
    pub subject: Option<types::Reference>,

    /// Healthcare event during which this observation is made
    pub context: Option<types::Reference>,

    /// Clinically relevant time/time-period for observation
    /// The `Observation.effective[x]` choice element (0..1); see [`ObservationEffective`].
    #[serde(flatten)]
    pub effective: Option<ObservationEffective>,

    /// Date/Time this was made available
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Who is responsible for the observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Actual result
    /// The `Observation.value[x]` choice element (0..1); see [`ObservationValue`].
    #[serde(flatten)]
    pub value: Option<ObservationValue>,

    /// Why the result is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc.
    pub interpretation: Option<types::CodeableConcept>,

    /// Comments about result
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Observed body part
    pub body_site: Option<types::CodeableConcept>,

    /// How it was done
    pub method: Option<types::CodeableConcept>,

    /// Specimen used for this observation
    pub specimen: Option<types::Reference>,

    /// (Measurement) Device
    pub device: Option<types::Reference>,

    /// Provides guide for interpretation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ObservationReferenceRange>,

    /// Resource related to this observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ObservationRelated>,

    /// Component results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ObservationComponent>,
}

/// Some observations have multiple component observations. These component
/// observations are expressed as separate code value pairs that share the same
/// attributes. Examples include systolic and diastolic component observations
/// for blood pressure measurement and multiple component observations for
/// genetics observations.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::observation::ObservationComponent;
/// use fhir::r3::types;
///
/// let value = ObservationComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ObservationComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ObservationComponent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of component observation (code / type)
    pub code: types::CodeableConcept,

    /// Actual component result
    /// The `Observation.component.value[x]` choice element (0..1); see [`ObservationComponentValue`].
    #[serde(flatten)]
    pub value: Option<ObservationComponentValue>,

    /// Why the component result is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc.
    pub interpretation: Option<types::CodeableConcept>,

    /// Provides guide for interpretation of component result
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ObservationReferenceRange>,
}

/// Guidance on how to interpret the value by comparison to a normal or
/// recommended range.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::observation::ObservationReferenceRange;
/// use fhir::r3::types;
///
/// let value = ObservationReferenceRange {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: ObservationReferenceRange = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ObservationReferenceRange {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Low Range, if relevant
    pub low: Option<types::Quantity>,

    /// High Range, if relevant
    pub high: Option<types::Quantity>,

    /// Reference range qualifier
    pub r#type: Option<types::CodeableConcept>,

    /// Reference range population
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applies_to: Vec<types::CodeableConcept>,

    /// Applicable age range, if relevant
    pub age: Option<types::Range>,

    /// Text based reference range in an observation
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// A reference to another resource (usually another Observation) whose
/// relationship is defined by the relationship type code.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::observation::ObservationRelated;
/// use fhir::r3::types;
///
/// let value = ObservationRelated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ObservationRelated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ObservationRelated {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// has-member | derived-from | sequel-to | replaces | qualified-by |
    /// interfered-by
    pub r#type: Option<crate::coded::Coded<crate::r3::codes::ObservationRelationshiptypes>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Resource that is related to this one
    pub target: types::Reference,
}

/// The `Observation.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `Observation.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
}

/// The `Observation.component.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationComponentValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Observation;

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
