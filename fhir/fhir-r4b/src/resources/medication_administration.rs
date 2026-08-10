//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//! Version: 4.3.0
//!
//! Administration of medication to a patient
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the event of a patient consuming or otherwise being administered
/// a medication. This may be as simple as swallowing a tablet or it may be a
/// long running infusion. Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_administration::MedicationAdministration;
/// use fhir::r4b::types;
///
/// let value = MedicationAdministration {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationAdministrationDe")]
#[fhir_version("r4b")]
pub struct MedicationAdministration {
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

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// in-progress | not-done | on-hold | completed | entered-in-error |
    /// stopped | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// Type of medication usage
    pub category: Option<types::CodeableConcept>,

    /// What was administered
    /// The `MedicationAdministration.medication[x]` choice element (1..1); see [`MedicationAdministrationMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationAdministrationMedication>,

    /// Who received medication
    pub subject: types::Reference,

    /// Encounter or Episode of Care administered as part of
    pub context: Option<types::Reference>,

    /// Additional information to support administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Start and end time of administration
    /// The `MedicationAdministration.effective[x]` choice element (1..1); see [`MedicationAdministrationEffective`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub effective: Option<MedicationAdministrationEffective>,

    /// Who performed the medication administration and what they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationAdministrationPerformer>,

    /// Reason administration performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or observation that supports why the medication was
    /// administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Request administration performed against
    pub request: Option<types::Reference<crate::r4b::resources::MedicationRequest>>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference<crate::r4b::resources::Device>>,

    /// Information about the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationAdministrationDe {
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
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    instantiates: Vec<types::Uri>,
    #[serde(rename = "_instantiates")]
    #[serde(default)]
    instantiates_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r4b::codes::MedicationAdminStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    status_reason: Vec<types::CodeableConcept>,
    category: Option<types::CodeableConcept>,
    #[serde(flatten)]
    medication: crate::r4b::choice::Slot<MedicationAdministrationMedication>,
    subject: types::Reference,
    context: Option<types::Reference>,
    #[serde(default)]
    supporting_information: Vec<types::Reference>,
    #[serde(flatten)]
    effective: crate::r4b::choice::Slot<MedicationAdministrationEffective>,
    #[serde(default)]
    performer: Vec<MedicationAdministrationPerformer>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    request: Option<types::Reference<crate::r4b::resources::MedicationRequest>>,
    #[serde(default)]
    device: Vec<types::Reference<crate::r4b::resources::Device>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    dosage: Option<MedicationAdministrationDosage>,
    #[serde(default)]
    event_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,
}

impl ::core::convert::From<MedicationAdministrationDe> for MedicationAdministration {
    fn from(v: MedicationAdministrationDe) -> Self {
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
            instantiates: v.instantiates,
            instantiates_ext: v.instantiates_ext,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            category: v.category,
            medication: v.medication.0,
            subject: v.subject,
            context: v.context,
            supporting_information: v.supporting_information,
            effective: v.effective.0,
            performer: v.performer,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            request: v.request,
            device: v.device,
            note: v.note,
            dosage: v.dosage,
            event_history: v.event_history,
        }
    }
}

/// Describes the medication dosage information details e.g. dose, rate, site,
/// route, etc.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r4b::types;
///
/// let value = MedicationAdministrationDosage {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationAdministrationDosageDe")]
#[fhir_version("r4b")]
pub struct MedicationAdministrationDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Body site administered to
    pub site: Option<types::CodeableConcept>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<types::Quantity>,

    /// Dose quantity per unit of time
    /// The `MedicationAdministration.dosage.rate[x]` choice element (0..1); see [`MedicationAdministrationDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationAdministrationDosageRate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationAdministrationDosageDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    text: Option<types::String>,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
    site: Option<types::CodeableConcept>,
    route: Option<types::CodeableConcept>,
    method: Option<types::CodeableConcept>,
    dose: Option<types::Quantity>,
    #[serde(flatten)]
    rate: crate::r4b::choice::Slot<MedicationAdministrationDosageRate>,
}

impl ::core::convert::From<MedicationAdministrationDosageDe> for MedicationAdministrationDosage {
    fn from(v: MedicationAdministrationDosageDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            text: v.text,
            text_ext: v.text_ext,
            site: v.site,
            route: v.route,
            method: v.method,
            dose: v.dose,
            rate: v.rate.0,
        }
    }
}

/// Indicates who or what performed the medication administration and how they
/// were involved.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_administration::MedicationAdministrationPerformer;
/// use fhir::r4b::types;
///
/// let value = MedicationAdministrationPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct MedicationAdministrationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the medication administration
    pub actor: types::Reference,
}

/// The `MedicationAdministration.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationAdministration.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `MedicationAdministration.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationAdministration;

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
