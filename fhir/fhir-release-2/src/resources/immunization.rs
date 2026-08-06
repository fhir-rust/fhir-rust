//! Immunization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Immunization
//!
//!
//!
//! Immunization event information
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Immunization Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::immunization::Immunization;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct Immunization {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// in-progress | on-hold | completed | entered-in-error | stopped
    pub status: crate::coded::Coded<crate::r2::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Vaccination administration date
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Vaccine product administered
    pub vaccine_code: types::CodeableConcept,

    /// Who was immunized
    pub patient: types::Reference,

    /// Flag for whether immunization was given
    pub was_not_given: types::Boolean,
    /// Primitive extension sibling for [`was_not_given`](Self::was_not_given) (FHIR `_wasNotGiven`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_wasNotGiven")]
    pub was_not_given_ext: Option<types::Element>,

    /// Indicates a self-reported record
    pub reported: types::Boolean,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,

    /// Who administered vaccine
    pub performer: Option<types::Reference>,

    /// Who ordered vaccination
    pub requester: Option<types::Reference>,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference>,

    /// Vaccine manufacturer
    pub manufacturer: Option<types::Reference>,

    /// Where vaccination occurred
    pub location: Option<types::Reference>,

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

    /// Body site vaccine was administered
    pub site: Option<types::CodeableConcept>,

    /// How vaccine entered body
    pub route: Option<types::CodeableConcept>,

    /// Amount of vaccine administered
    pub dose_quantity: Option<types::Quantity>,

    /// Vaccination notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Administration/non-administration reasons
    pub explanation: Option<ImmunizationExplanation>,

    /// Details of a reaction that follows immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ImmunizationReaction>,

    /// What protocol was followed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vaccination_protocol: Vec<ImmunizationVaccinationProtocol>,
}

/// Reasons why a vaccine was or was not administered.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::immunization::ImmunizationExplanation;
/// use fhir::r2::types;
///
/// let value = ImmunizationExplanation {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ImmunizationExplanation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImmunizationExplanation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Why immunization occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Why immunization did not occur
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_given: Vec<types::CodeableConcept>,
}

/// Categorical data indicating that an adverse event is associated in time to
/// an immunization.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::immunization::ImmunizationReaction;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct ImmunizationReaction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When reaction started
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Additional information on reaction
    pub detail: Option<types::Reference>,

    /// Indicates self-reported reaction
    pub reported: Option<types::Boolean>,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,
}

/// Contains information about the protocol(s) under which the vaccine was
/// administered.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::immunization::ImmunizationVaccinationProtocol;
///
/// let value = ImmunizationVaccinationProtocol::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationVaccinationProtocol = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImmunizationVaccinationProtocol {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Dose number within series
    pub dose_sequence: types::PositiveInt,
    /// Primitive extension sibling for [`dose_sequence`](Self::dose_sequence) (FHIR `_doseSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doseSequence")]
    pub dose_sequence_ext: Option<types::Element>,

    /// Details of vaccine protocol
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who is responsible for protocol
    pub authority: Option<types::Reference>,

    /// Name of vaccine series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`series_doses`](Self::series_doses) (FHIR `_seriesDoses`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_seriesDoses")]
    pub series_doses_ext: Option<types::Element>,

    /// Disease immunized against
    pub target_disease: ::vec1::Vec1<types::CodeableConcept>,

    /// Indicates if dose counts towards immunity
    pub dose_status: types::CodeableConcept,

    /// Why dose does (not) count
    pub dose_status_reason: Option<types::CodeableConcept>,
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
