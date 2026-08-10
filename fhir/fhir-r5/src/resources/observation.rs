//! Observation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Observation
//!
//! Version: 5.0.0
//!
//! Observation Resource: Measurements and simple assertions made about a patient, device or other subject.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Measurements and simple assertions made about a patient, device or other
/// subject.
///
/// Observation is one of the most widely used resources in FHIR R5 and serves
/// as the general-purpose container for recording facts about a subject at a
/// point in time. It supports clinical use cases such as vital signs, laboratory
/// results, imaging findings, device measurements, social history, and clinical
/// assessments, as well as administrative and survey data. Each Observation
/// carries a code identifying what was measured or asserted, an effective time,
/// a status in the observation lifecycle, and a value expressed through one of
/// the polymorphic value[x] elements (for example a Quantity, CodeableConcept,
/// string, or Ratio) or, when no value is available, a data-absent reason.
///
/// Most observations are simple name/value pair assertions with some metadata,
/// but observations can also group other observations together logically via
/// members, or be composed of several components that share the same metadata,
/// such as the systolic and diastolic readings of a blood pressure. Reference
/// ranges, interpretations, body sites, and methods provide additional context
/// for interpreting results. Observations are commonly derived from or based on
/// orders and definitions, and they support diagnosis, monitoring of progress,
/// establishing baselines and patterns, and capturing demographic
/// characteristics.
///
/// # See also
///
/// The observation typically references a subject such as a
/// [`Patient`](crate::r5::resources::patient::Patient), an
/// [`Encounter`](crate::r5::resources::encounter::Encounter) during which it was
/// made, and optionally a [`Device`](crate::r5::resources::device::Device) or
/// [`Specimen`](crate::r5::resources::specimen::Specimen). Coded elements use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), measured values use
/// [`Quantity`](crate::r5::types::Quantity), and links to other resources use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation::Observation;
/// use fhir::r5::types;
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
#[serde(from = "ObservationDe")]
pub struct Observation {
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

    /// Business Identifier for observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

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

    /// Lifecycle status of the observation, such as registered, preliminary, final, or amended; required.
    pub status: crate::r5::coded::Coded<crate::r5::codes::ObservationStatus>,

    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of  type of observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Coded concept identifying what was observed or measured, such as a LOINC code; required.
    pub code: types::CodeableConcept,

    /// Reference to who or what the observation is about, most often a patient but possibly a group, device, or location.
    pub subject: Option<types::Reference>,

    /// What the observation is about, when it is not about the subject of record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// Healthcare event during which this observation is made
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `Observation.effective[x]` choice element (0..1); see [`ObservationEffective`].
    #[serde(flatten)]
    pub effective: Option<ObservationEffective>,

    /// Date/Time this version was made available
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Who is responsible for the observation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// The `Observation.value[x]` choice element (0..1); see [`ObservationValue`].
    #[serde(flatten)]
    pub value: Option<ObservationValue>,

    /// Why the result is missing
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
    pub body_structure: Option<types::Reference<crate::r5::resources::BodyStructure>>,

    /// How it was done
    pub method: Option<types::CodeableConcept>,

    /// Specimen used for this observation
    pub specimen: Option<types::Reference>,

    /// A reference to the device that generates the measurements or the device settings for the device
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObservationDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(flatten)]
    instantiates: crate::r5::choice::Slot<ObservationInstantiates>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    triggered_by: Vec<ObservationTriggeredBy>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::r5::coded::Coded<crate::r5::codes::ObservationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    code: types::CodeableConcept,
    subject: Option<types::Reference>,
    #[serde(default)]
    focus: Vec<types::Reference>,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r5::choice::Slot<ObservationEffective>,
    issued: Option<types::Instant>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<ObservationValue>,
    data_absent_reason: Option<types::CodeableConcept>,
    #[serde(default)]
    interpretation: Vec<types::CodeableConcept>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    body_site: Option<types::CodeableConcept>,
    body_structure: Option<types::Reference<crate::r5::resources::BodyStructure>>,
    method: Option<types::CodeableConcept>,
    specimen: Option<types::Reference>,
    device: Option<types::Reference>,
    #[serde(default)]
    reference_range: Vec<ObservationReferenceRange>,
    #[serde(default)]
    has_member: Vec<types::Reference>,
    #[serde(default)]
    derived_from: Vec<types::Reference>,
    #[serde(default)]
    component: Vec<ObservationComponent>,
}

impl ::core::convert::From<ObservationDe> for Observation {
    fn from(v: ObservationDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            instantiates: v.instantiates.0,
            based_on: v.based_on,
            triggered_by: v.triggered_by,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            category: v.category,
            code: v.code,
            subject: v.subject,
            focus: v.focus,
            encounter: v.encounter,
            effective: v.effective.0,
            issued: v.issued,
            issued_ext: v.issued_ext,
            performer: v.performer,
            value: v.value.0,
            data_absent_reason: v.data_absent_reason,
            interpretation: v.interpretation,
            note: v.note,
            body_site: v.body_site,
            body_structure: v.body_structure,
            method: v.method,
            specimen: v.specimen,
            device: v.device,
            reference_range: v.reference_range,
            has_member: v.has_member,
            derived_from: v.derived_from,
            component: v.component,
        }
    }
}

/// Triggering observation(s).
///
/// Identifies one or more observations that triggered the performance of this
/// observation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation::ObservationTriggeredBy;
/// use fhir::r5::types;
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
    pub observation: types::Reference<crate::r5::resources::Observation>,

    /// reflex | repeat | re-run
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ObservationTriggeredbytype>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Reason that the observation was triggered
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`).
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,
}

/// Provides guide for interpretation.
///
/// Guidance on how to interpret the value by comparison to a normal or
/// recommended range. Multiple reference ranges are interpreted as an "OR". In
/// other words, to represent two distinct target populations, two
/// referenceRange elements would be used.
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation::ObservationReferenceRange;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Component results.
///
/// Some observations have multiple component observations. These component
/// observations are expressed as separate code value pairs that share the same
/// attributes. Examples include systolic and diastolic component observations
/// for blood pressure measurement and multiple component observations for
/// genetics observations.
/// # Examples
///
/// ```
/// use fhir::r5::resources::observation::ObservationComponent;
/// use fhir::r5::types;
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
#[serde(from = "ObservationComponentDe")]
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

    /// The `Observation.component.value[x]` choice element (0..1); see [`ObservationComponentValue`].
    #[serde(flatten)]
    pub value: Option<ObservationComponentValue>,

    /// Why the component result is missing
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// High, low, normal, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpretation: Vec<types::CodeableConcept>,

    /// Provides guide for interpretation of component result
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_range: Vec<ObservationReferenceRange>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObservationComponentDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<ObservationComponentValue>,
    data_absent_reason: Option<types::CodeableConcept>,
    #[serde(default)]
    interpretation: Vec<types::CodeableConcept>,
    #[serde(default)]
    reference_range: Vec<ObservationReferenceRange>,
}

impl ::core::convert::From<ObservationComponentDe> for ObservationComponent {
    fn from(v: ObservationComponentDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
            data_absent_reason: v.data_absent_reason,
            interpretation: v.interpretation,
            reference_range: v.reference_range,
        }
    }
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
/// The `Observation.component.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
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
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
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

/// The `Observation.effective[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ObservationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
    /// `effectiveTiming` variant.
    #[fhir("effectiveTiming")]
    Timing(Box<types::Timing>),
    /// `effectiveInstant` variant.
    #[fhir("effectiveInstant")]
    Instant(crate::r5::choice::Primitive<types::Instant>),
}

/// The `Observation.instantiates[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ObservationInstantiates {
    /// `instantiatesCanonical` variant.
    #[fhir("instantiatesCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `instantiatesReference` variant.
    #[fhir("instantiatesReference")]
    Reference(Box<types::Reference>),
}

/// The `Observation.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
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
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
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
