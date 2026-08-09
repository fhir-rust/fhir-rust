//! Observation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Observation
//!
//! Version: 6.0.0-ballot3
//!
//! Measurements and simple assertions
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Measurements and simple assertions made about a patient, device or other
/// subject.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation::Observation;
/// use fhir::r6::types;
///
/// let value = Observation {
///     organizer: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `organizer` is the name this serializes to on the wire.
/// assert_eq!(json["organizer"], ::serde_json::json!(true));
///
/// let back: Observation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Observation {
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

    /// Business Identifier for observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR ObservationDefinition
    /// The `Observation.instantiates[x]` choice element (0..1); see [`ObservationInstantiates`].
    #[serde(flatten)]
    pub instantiates: Option<ObservationInstantiates>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Triggering observation(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub triggered_by: Vec<ObservationTriggeredBy>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// registered | specimen-in-process | preliminary | final | amended |
    /// corrected | appended | cancelled | entered-in-error | unknown |
    /// cannot-be-obtained
    pub status: crate::coded::Coded<crate::r6::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of type of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of observation (code / type)
    pub code: types::CodeableConcept,

    /// Who and/or what the observation is about
    pub subject: Option<types::Reference>,

    /// What the observation is about, when it is not about the subject of
    /// record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// This observation organizes/groups a set of sub-observations
    pub organizer: Option<types::Boolean>,
    /// Primitive extension sibling for [`organizer`](Self::organizer) (FHIR `_organizer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_organizer")]
    pub organizer_ext: Option<types::Element>,

    /// Healthcare event during which this observation is made
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Clinically relevant time/time-period for observation
    /// The `Observation.effective[x]` choice element (0..1); see [`ObservationEffective`].
    #[serde(flatten)]
    pub effective: Option<ObservationEffective>,

    /// Date/Time this version was made available
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

    /// Why the result value is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpretation: Vec<types::CodeableConcept>,

    /// Comments about the observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Observed body part
    pub body_site: Option<types::CodeableConcept>,

    /// Observed body structure
    pub body_structure: Option<types::Reference<crate::r6::resources::BodyStructure>>,

    /// How it was done
    pub method: Option<types::CodeableConcept>,

    /// Specimen used for this observation
    pub specimen: Option<types::Reference>,

    /// A reference to the device that generates the measurements or the device
    /// settings for the device
    pub device: Option<types::Reference>,

    /// Provides guide for interpretation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ObservationReferenceRange>,

    /// Related resource that belongs to the Observation group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_member: Vec<types::Reference>,

    /// Related resource from which the observation is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

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
/// use fhir::r6::resources::observation::ObservationComponent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ObservationComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of component observation (code / type)
    pub code: types::CodeableConcept,

    /// Actual component result
    /// The `Observation.component.value[x]` choice element (0..1); see [`ObservationComponentValue`].
    #[serde(flatten)]
    pub value: Option<ObservationComponentValue>,

    /// Why the component result value is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpretation: Vec<types::CodeableConcept>,

    /// Provides guide for interpretation of component result value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ObservationReferenceRange>,
}

/// Guidance on how to interpret the value by comparison to a normal or
/// recommended range. Multiple reference ranges are interpreted as an "OR". In
/// other words, to represent two distinct target populations, two
/// `referenceRange` elements would be used.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation::ObservationReferenceRange;
/// use fhir::r6::types;
///
/// let value = ObservationReferenceRange {
///     text: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("# Heading"));
///
/// let back: ObservationReferenceRange = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ObservationReferenceRange {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Low Range, if relevant
    pub low: Option<types::Quantity>,

    /// High Range, if relevant
    pub high: Option<types::Quantity>,

    /// Normal value, if relevant
    pub normal_value: Option<types::CodeableConcept>,

    /// Reference range qualifier
    pub r#type: Option<types::CodeableConcept>,

    /// Reference range population
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applies_to: Vec<types::CodeableConcept>,

    /// Applicable age range, if relevant
    pub age: Option<types::Range>,

    /// Text based reference range in an observation
    pub text: Option<types::Markdown>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Identifies the observation(s) that triggered the performance of this
/// observation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::observation::ObservationTriggeredBy;
/// use fhir::r6::types;
///
/// let value = ObservationTriggeredBy {
///     reason: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reason` is the name this serializes to on the wire.
/// assert_eq!(json["reason"], ::serde_json::json!("abc"));
///
/// let back: ObservationTriggeredBy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ObservationTriggeredBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Triggering observation
    pub observation: types::Reference<crate::r6::resources::Observation>,

    /// reflex | repeat | re-run
    pub r#type: crate::coded::Coded<crate::r6::codes::ObservationTriggeredbytype>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Reason that the observation was triggered
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,
}

/// The `Observation.instantiates[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationInstantiates {
    /// `instantiatesCanonical` variant.
    #[fhir("instantiatesCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `instantiatesReference` variant.
    #[fhir("instantiatesReference")]
    Reference(Box<types::Reference>),
}

/// The `Observation.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ObservationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
    /// `effectiveTiming` variant.
    #[fhir("effectiveTiming")]
    Timing(Box<types::Timing>),
    /// `effectiveInstant` variant.
    #[fhir("effectiveInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
}

/// The `Observation.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

/// The `Observation.component.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
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
