//! Immunization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Immunization
//!
//! Version: 6.0.0-ballot3
//!
//! Immunization event information
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the event of a patient being administered a vaccine or a record
/// of an immunization as reported by a patient, a clinician or another party.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization::Immunization;
/// use fhir::r6::types;
///
/// let value = Immunization {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: Immunization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Immunization {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Authority that the immunization event is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// completed | entered-in-error | not-done
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Vaccine administered
    pub vaccine_code: types::CodeableConcept,

    /// Product that was administered
    pub administered_product: Option<types::CodeableReference>,

    /// Vaccine manufacturer
    pub manufacturer: Option<types::CodeableReference>,

    /// Vaccine lot number
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// Vaccine expiration date
    pub expiration_date: Option<types::Date>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Who was immunized
    pub patient: types::Reference,

    /// Encounter immunization was part of
    pub encounter: Option<types::Reference>,

    /// Additional information in support of the immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Vaccine administration date
    /// The `Immunization.occurrence[x]` choice element (1..1); see [`ImmunizationOccurrence`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub occurrence: Option<ImmunizationOccurrence>,

    /// Indicates context the data was captured in
    pub primary_source: Option<types::Boolean>,
    /// Primitive extension sibling for [`primary_source`](Self::primary_source) (FHIR `_primarySource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_primarySource")]
    pub primary_source_ext: Option<types::Element>,

    /// Indicates the source of a reported record
    pub information_source: Option<types::CodeableReference>,

    /// The service delivery location
    pub location: Option<types::Reference>,

    /// Body site vaccine was administered
    pub site: Option<types::CodeableConcept>,

    /// How vaccine entered body
    pub route: Option<types::CodeableConcept>,

    /// Amount of vaccine administered
    pub dose_quantity: Option<types::Quantity>,

    /// Who performed event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImmunizationPerformer>,

    /// Additional immunization notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Why immunization occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Dose potency
    pub is_subpotent: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_subpotent`](Self::is_subpotent) (FHIR `_isSubpotent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isSubpotent")]
    pub is_subpotent_ext: Option<types::Element>,

    /// Reason for being subpotent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subpotent_reason: Vec<types::CodeableConcept>,

    /// Patient eligibility for a specific vaccination program
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_eligibility: Vec<ImmunizationProgramEligibility>,

    /// Funding source for the vaccine
    pub funding_source: Option<types::CodeableConcept>,

    /// Details of a reaction that follows immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ImmunizationReaction>,

    /// Protocol followed by the provider
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_applied: Vec<ImmunizationProtocolApplied>,
}

/// Indicates who performed the immunization event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization::ImmunizationPerformer;
/// use fhir::r6::types;
///
/// let value = ImmunizationPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationPerformer {
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

    /// Individual or organization who was performing
    pub actor: types::Reference,
}

/// Indicates a patient's eligibility for a funding program.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization::ImmunizationProgramEligibility;
/// use fhir::r6::types;
///
/// let value = ImmunizationProgramEligibility {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationProgramEligibility = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationProgramEligibility {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The program that eligibility is declared for
    pub program: types::CodeableConcept,

    /// The patient's eligibility status for the program
    pub program_status: types::CodeableConcept,
}

/// The protocol (set of recommendations) being followed by the provider who
/// administered the dose.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization::ImmunizationProtocolApplied;
/// use fhir::r6::types;
///
/// let value = ImmunizationProtocolApplied {
///     series: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `series` is the name this serializes to on the wire.
/// assert_eq!(json["series"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationProtocolApplied = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationProtocolApplied {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of vaccine series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<types::Reference>,

    /// Vaccine preventatable disease being targeted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<types::CodeableConcept>,

    /// Dose number within series
    pub dose_number: Option<types::CodeableConcept>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::CodeableConcept>,
}

/// Categorical data indicating that an adverse event is associated in time to
/// an immunization.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization::ImmunizationReaction;
/// use fhir::r6::types;
///
/// let value = ImmunizationReaction {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ImmunizationReaction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationReaction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When reaction started
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Additional information on reaction
    pub manifestation: Option<types::CodeableReference>,

    /// Indicates self-reported reaction
    pub reported: Option<types::Boolean>,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,
}

/// The `Immunization.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrenceString` variant.
    #[fhir("occurrenceString")]
    String(crate::r6::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Immunization;

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
