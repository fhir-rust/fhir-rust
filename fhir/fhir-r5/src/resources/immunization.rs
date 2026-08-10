//! Immunization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Immunization
//!
//! Version: 5.0.0
//!
//! Immunization Resource: Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Immunization
///
/// Describes the event of a patient being administered a vaccine or a record of
/// an immunization as reported by a patient, a clinician or another party. It
/// captures what vaccine was given, to whom, when, where, and by whom, along
/// with related clinical details such as reactions, protocols applied, and
/// program eligibility. This resource supports both first-hand administration
/// records and reported immunization histories.
///
/// Clinically, an Immunization is used to build and maintain a patient's
/// vaccination history, drive clinical decision support such as forecasting
/// which vaccines are due or overdue, support public health reporting to
/// immunization registries, and document adverse reactions observed after
/// administration. A single event is normally represented by one instance of
/// this resource; entries reported without direct observation (for example,
/// self-reported by the patient) are distinguished from first-hand records via
/// the `primary_source` and `information_source` elements. Multi-dose series
/// tracking and clinical protocol context are captured through the nested
/// `protocol_applied` and `program_eligibility` structures.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) is the
/// typical subject of the `patient` reference, and vaccine, site, and route
/// details are represented using [`CodeableConcept`](crate::r5::types::CodeableConcept).
/// Related workflow resources include `ImmunizationEvaluation` and
/// `ImmunizationRecommendation`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization::Immunization;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Immunization {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Authority that the immunization event is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The overall status of the event: completed | entered-in-error | not-done
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// The vaccine product that was administered, expressed as a coded concept
    pub vaccine_code: types::CodeableConcept,

    /// Product that was administered
    pub administered_product: Option<types::CodeableReference>,

    /// Vaccine manufacturer
    pub manufacturer: Option<types::CodeableReference>,

    /// Vaccine lot number
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`).
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// Vaccine expiration date
    pub expiration_date: Option<types::Date>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`).
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Reference to the patient who was immunized
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// Encounter immunization was part of
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// Additional information in support of the immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// The `Immunization.occurrence[x]` choice element (0..1); see [`ImmunizationOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ImmunizationOccurrence>,

    /// Indicates context the data was captured in
    pub primary_source: Option<types::Boolean>,
    /// Primitive extension sibling for [`primary_source`](Self::primary_source) (FHIR `_primarySource`).
    #[serde(rename = "_primarySource")]
    pub primary_source_ext: Option<types::Element>,

    /// Indicates the source of a reported record
    pub information_source: Option<types::CodeableReference>,

    /// Where immunization occurred
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

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
    /// Primitive extension sibling for [`is_subpotent`](Self::is_subpotent) (FHIR `_isSubpotent`).
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

/// Immunization.performer: Who performed event
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization::ImmunizationPerformer;
/// use fhir::r5::types;
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
pub struct ImmunizationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What type of performance was done
    pub function: Option<types::CodeableConcept>,

    /// Individual or organization who was performing
    pub actor: types::Reference,
}

/// Immunization.programEligibility: Patient eligibility for a specific vaccination program
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization::ImmunizationProgramEligibility;
/// use fhir::r5::types;
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

/// Immunization.reaction: Details of a reaction that follows immunization
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization::ImmunizationReaction;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Additional information on reaction
    pub manifestation: Option<types::CodeableReference>,

    /// Indicates self-reported reaction
    pub reported: Option<types::Boolean>,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`).
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,
}

/// Immunization.protocolApplied: Protocol followed by the provider
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization::ImmunizationProtocolApplied;
/// use fhir::r5::types;
///
/// let value = ImmunizationProtocolApplied {
///     series_doses: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `seriesDoses` is the name this serializes to on the wire.
/// assert_eq!(json["seriesDoses"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationProtocolApplied = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`).
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Vaccine preventatable disease being targeted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<types::CodeableConcept>,

    /// Dose number within series
    pub dose_number: types::String,
    /// Primitive extension sibling for [`dose_number`](Self::dose_number) (FHIR `_doseNumber`).
    #[serde(rename = "_doseNumber")]
    pub dose_number_ext: Option<types::Element>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::String>,
    /// Primitive extension sibling for [`series_doses`](Self::series_doses) (FHIR `_seriesDoses`).
    #[serde(rename = "_seriesDoses")]
    pub series_doses_ext: Option<types::Element>,
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
/// The `Immunization.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrenceString` variant.
    #[fhir("occurrenceString")]
    String(crate::r5::choice::Primitive<types::String>),
}
