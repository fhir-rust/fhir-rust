//! DeviceUsage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
//!
//! Version: 6.0.0-ballot3
//!
//! Record of use of a device
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a device being used by a patient where the record is the result
/// of a report from the patient or a clinician.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_usage::DeviceUsage;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceUsage {
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
    pub contained: Vec<::serde_json::Value>,

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
    pub based_on: Vec<types::Reference>,

    /// active | completed | not-done | entered-in-error +
    pub status: crate::coded::Coded<crate::r6::codes::DeviceusageStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The category of the statement - classifying how the statement is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Patient using device
    pub patient: types::Reference,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// The encounter or episode of care that establishes the context for this
    /// device use statement
    pub context: Option<types::Reference>,

    /// How often the device was used
    /// The `DeviceUsage.timing[x]` choice element (0..1); see [`DeviceUsageTiming`].
    #[serde(flatten)]
    pub timing: Option<DeviceUsageTiming>,

    /// When the statement was made (and recorded)
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// The status of the device usage, for example always, sometimes, never.
    /// This is not the same as the status of the statement
    pub usage_status: Option<types::CodeableConcept>,

    /// The reason for asserting the usage status - for example forgot, lost,
    /// stolen, broken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage_reason: Vec<types::CodeableConcept>,

    /// How device is being used
    pub adherence: Option<DeviceUsageAdherence>,

    /// Who made the statement
    pub information_source: Option<types::Reference>,

    /// Code or Reference to device used
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

/// This indicates how or if the device is being used.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::device_usage::DeviceUsageAdherence;
///
/// let value = DeviceUsageAdherence::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DeviceUsageAdherence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub reason: ::vec1::Vec1<types::CodeableConcept>,
}

/// The `DeviceUsage.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
