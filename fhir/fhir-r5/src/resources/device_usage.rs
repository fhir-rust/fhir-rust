//! DeviceUsage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
//!
//! Version: 5.0.0
//!
//! DeviceUsage Resource: A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a device being used by a patient where the record is the result
/// of a report from the patient or a clinician.
///
/// DeviceUsage records that a patient is or was using a particular device. In
/// FHIR R5 it captures who is using the device, which device is involved, the
/// timing and context of the usage, and any adherence, status, or reason
/// information supplied by the patient or a clinician. It is commonly used to
/// track patient-reported or clinician-reported device use over time.
///
/// Unlike [`DeviceRequest`](crate::r5::resources::device_request::DeviceRequest),
/// which represents an order or proposal for device use, DeviceUsage is a
/// statement of fact describing actual (or historical) use as reported by the
/// patient, a caregiver, or a clinician; it is not itself an authorization. It
/// is administratively useful for medication-reconciliation-style workflows for
/// durable medical equipment and other devices, for tracking compliance with a
/// prescribed device regimen, and for capturing why a patient stopped, changed,
/// or continued using a device.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject who is using the device.
/// - `Device` — the physical or virtual device being used, referenced via the `device` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for coded values such as `usage_status` and `usage_reason`.
/// - `Encounter` — the encounter or episode of care providing context, referenced via `context`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_usage::DeviceUsage;
/// use fhir::r5::types;
///
/// let value = DeviceUsage {
///     date_asserted: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateAsserted` is the name this serializes to on the wire.
/// assert_eq!(json["dateAsserted"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceUsage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DeviceUsageDe")]
pub struct DeviceUsage {
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

    /// External identifier for this record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,

    /// The current state of this device usage record: active | completed | not-done | entered-in-error +
    pub status: crate::r5::coded::Coded<crate::r5::codes::DeviceusageStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The category of the statement - classifying how the statement is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// A reference to the [`Patient`](crate::r5::resources::patient::Patient) (or group) reported to be using the device
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// The encounter or episode of care that establishes the context for this device use statement
    pub context: Option<types::Reference>,

    /// The `DeviceUsage.timing[x]` choice element (0..1); see [`DeviceUsageTiming`].
    #[serde(flatten)]
    pub timing: Option<DeviceUsageTiming>,

    /// When the statement was made (and recorded)
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`).
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement
    pub usage_status: Option<types::CodeableConcept>,

    /// The reason for asserting the usage status - for example forgot, lost, stolen, broken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage_reason: Vec<types::CodeableConcept>,

    /// How device is being used
    pub adherence: Option<DeviceUsageAdherence>,

    /// Who made the statement
    pub information_source: Option<types::Reference>,

    /// The device that was used, given either as a coded value or as a reference to a `Device` resource
    pub device: types::CodeableReference,

    /// Why device was used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Target body site
    pub body_site: Option<types::CodeableReference>,

    /// Addition details (comments, instructions)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceUsageDe {
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
    #[serde(default)]
    based_on: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,
    status: crate::r5::coded::Coded<crate::r5::codes::DeviceusageStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    patient: types::Reference<crate::r5::resources::Patient>,
    #[serde(default)]
    derived_from: Vec<types::Reference>,
    context: Option<types::Reference>,
    #[serde(flatten)]
    timing: crate::r5::choice::Slot<DeviceUsageTiming>,
    date_asserted: Option<types::DateTime>,
    #[serde(rename = "_dateAsserted")]
    date_asserted_ext: Option<types::Element>,
    usage_status: Option<types::CodeableConcept>,
    #[serde(default)]
    usage_reason: Vec<types::CodeableConcept>,
    adherence: Option<DeviceUsageAdherence>,
    information_source: Option<types::Reference>,
    device: types::CodeableReference,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    body_site: Option<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<DeviceUsageDe> for DeviceUsage {
    fn from(v: DeviceUsageDe) -> Self {
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
            based_on: v.based_on,
            status: v.status,
            status_ext: v.status_ext,
            category: v.category,
            patient: v.patient,
            derived_from: v.derived_from,
            context: v.context,
            timing: v.timing.0,
            date_asserted: v.date_asserted,
            date_asserted_ext: v.date_asserted_ext,
            usage_status: v.usage_status,
            usage_reason: v.usage_reason,
            adherence: v.adherence,
            information_source: v.information_source,
            device: v.device,
            reason: v.reason,
            body_site: v.body_site,
            note: v.note,
        }
    }
}

/// How device is being used.
///
/// Backbone element describing the patient's adherence to the device usage,
/// including a code such as always, never, or sometimes, and the reasons that
/// explain that adherence status.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::device_usage::DeviceUsageAdherence;
///
/// let value = DeviceUsageAdherence::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceUsageAdherence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUsageAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// always | never | sometimes
    pub code: types::CodeableConcept,

    /// lost | stolen | prescribed | broken | burned | forgot
    pub reason: vec1::Vec1<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceUsage;

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
/// The `DeviceUsage.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DeviceUsageTiming {
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
