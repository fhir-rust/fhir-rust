//! Immunization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Immunization
//!
//! Version: 4.3.0
//!
//! Immunization event information
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the event of a patient being administered a vaccine or a record
/// of an immunization as reported by a patient, a clinician or another party.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::immunization::Immunization;
/// use fhir::r4b::types;
///
/// let value = Immunization {
///     primary_source: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `primarySource` is the name this serializes to on the wire.
/// assert_eq!(json["primarySource"], ::serde_json::json!(true));
///
/// let back: Immunization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ImmunizationDe")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// completed | entered-in-error | not-done
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason not done
    pub status_reason: Option<types::CodeableConcept>,

    /// Vaccine product administered
    pub vaccine_code: types::CodeableConcept,

    /// Who was immunized
    pub patient: types::Reference<crate::r4b::resources::Patient>,

    /// Encounter immunization was part of
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// Vaccine administration date
    /// The `Immunization.occurrence[x]` choice element (1..1); see [`ImmunizationOccurrence`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub occurrence: Option<ImmunizationOccurrence>,

    /// When the immunization was first captured in the subject's record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Indicates context the data was recorded in
    pub primary_source: Option<types::Boolean>,
    /// Primitive extension sibling for [`primary_source`](Self::primary_source) (FHIR `_primarySource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_primarySource")]
    pub primary_source_ext: Option<types::Element>,

    /// Indicates the source of a secondarily reported record
    pub report_origin: Option<types::CodeableConcept>,

    /// Where immunization occurred
    pub location: Option<types::Reference<crate::r4b::resources::Location>>,

    /// Vaccine manufacturer
    pub manufacturer: Option<types::Reference<crate::r4b::resources::Organization>>,

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

    /// Who performed event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImmunizationPerformer>,

    /// Additional immunization notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Why immunization occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why immunization occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Dose potency
    pub is_subpotent: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_subpotent`](Self::is_subpotent) (FHIR `_isSubpotent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isSubpotent")]
    pub is_subpotent_ext: Option<types::Element>,

    /// Reason for being subpotent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subpotent_reason: Vec<types::CodeableConcept>,

    /// Educational material presented to patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<ImmunizationEducation>,

    /// Patient eligibility for a vaccination program
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_eligibility: Vec<types::CodeableConcept>,

    /// Funding source for the vaccine
    pub funding_source: Option<types::CodeableConcept>,

    /// Details of a reaction that follows immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ImmunizationReaction>,

    /// Protocol followed by the provider
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_applied: Vec<ImmunizationProtocolApplied>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmunizationDe {
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
    status: types::Code,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_reason: Option<types::CodeableConcept>,
    vaccine_code: types::CodeableConcept,
    patient: types::Reference<crate::r4b::resources::Patient>,
    encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r4b::choice::Slot<ImmunizationOccurrence>,
    recorded: Option<types::DateTime>,
    #[serde(rename = "_recorded")]
    recorded_ext: Option<types::Element>,
    primary_source: Option<types::Boolean>,
    #[serde(rename = "_primarySource")]
    primary_source_ext: Option<types::Element>,
    report_origin: Option<types::CodeableConcept>,
    location: Option<types::Reference<crate::r4b::resources::Location>>,
    manufacturer: Option<types::Reference<crate::r4b::resources::Organization>>,
    lot_number: Option<types::String>,
    #[serde(rename = "_lotNumber")]
    lot_number_ext: Option<types::Element>,
    expiration_date: Option<types::Date>,
    #[serde(rename = "_expirationDate")]
    expiration_date_ext: Option<types::Element>,
    site: Option<types::CodeableConcept>,
    route: Option<types::CodeableConcept>,
    dose_quantity: Option<types::Quantity>,
    #[serde(default)]
    performer: Vec<ImmunizationPerformer>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    is_subpotent: Option<types::Boolean>,
    #[serde(rename = "_isSubpotent")]
    is_subpotent_ext: Option<types::Element>,
    #[serde(default)]
    subpotent_reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    education: Vec<ImmunizationEducation>,
    #[serde(default)]
    program_eligibility: Vec<types::CodeableConcept>,
    funding_source: Option<types::CodeableConcept>,
    #[serde(default)]
    reaction: Vec<ImmunizationReaction>,
    #[serde(default)]
    protocol_applied: Vec<ImmunizationProtocolApplied>,
}

impl ::core::convert::From<ImmunizationDe> for Immunization {
    fn from(v: ImmunizationDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            vaccine_code: v.vaccine_code,
            patient: v.patient,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            recorded: v.recorded,
            recorded_ext: v.recorded_ext,
            primary_source: v.primary_source,
            primary_source_ext: v.primary_source_ext,
            report_origin: v.report_origin,
            location: v.location,
            manufacturer: v.manufacturer,
            lot_number: v.lot_number,
            lot_number_ext: v.lot_number_ext,
            expiration_date: v.expiration_date,
            expiration_date_ext: v.expiration_date_ext,
            site: v.site,
            route: v.route,
            dose_quantity: v.dose_quantity,
            performer: v.performer,
            note: v.note,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            is_subpotent: v.is_subpotent,
            is_subpotent_ext: v.is_subpotent_ext,
            subpotent_reason: v.subpotent_reason,
            education: v.education,
            program_eligibility: v.program_eligibility,
            funding_source: v.funding_source,
            reaction: v.reaction,
            protocol_applied: v.protocol_applied,
        }
    }
}

/// Educational material presented to the patient (or guardian) at the time of
/// vaccine administration.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::immunization::ImmunizationEducation;
/// use fhir::r4b::types;
///
/// let value = ImmunizationEducation {
///     document_type: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentType` is the name this serializes to on the wire.
/// assert_eq!(json["documentType"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationEducation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ImmunizationEducation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Educational material document identifier
    pub document_type: Option<types::String>,
    /// Primitive extension sibling for [`document_type`](Self::document_type) (FHIR `_documentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentType")]
    pub document_type_ext: Option<types::Element>,

    /// Educational material reference pointer
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Educational material publication date
    pub publication_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`publication_date`](Self::publication_date) (FHIR `_publicationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationDate")]
    pub publication_date_ext: Option<types::Element>,

    /// Educational material presentation date
    pub presentation_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`presentation_date`](Self::presentation_date) (FHIR `_presentationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_presentationDate")]
    pub presentation_date_ext: Option<types::Element>,
}

/// Indicates who performed the immunization event.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::immunization::ImmunizationPerformer;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

/// The protocol (set of recommendations) being followed by the provider who
/// administered the dose.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::immunization::ImmunizationProtocolApplied;
/// use fhir::r4b::types;
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
#[serde(from = "ImmunizationProtocolAppliedDe")]
#[fhir_version("r4b")]
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
    pub authority: Option<types::Reference<crate::r4b::resources::Organization>>,

    /// Vaccine preventatable disease being targetted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<types::CodeableConcept>,

    /// Dose number within series
    /// The `Immunization.protocolApplied.doseNumber[x]` choice element (1..1); see [`ImmunizationProtocolAppliedDoseNumber`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub dose_number: Option<ImmunizationProtocolAppliedDoseNumber>,

    /// Recommended number of doses for immunity
    /// The `Immunization.protocolApplied.seriesDoses[x]` choice element (0..1); see [`ImmunizationProtocolAppliedSeriesDoses`].
    #[serde(flatten)]
    pub series_doses: Option<ImmunizationProtocolAppliedSeriesDoses>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmunizationProtocolAppliedDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    series: Option<types::String>,
    #[serde(rename = "_series")]
    series_ext: Option<types::Element>,
    authority: Option<types::Reference<crate::r4b::resources::Organization>>,
    #[serde(default)]
    target_disease: Vec<types::CodeableConcept>,
    #[serde(flatten)]
    dose_number: crate::r4b::choice::Slot<ImmunizationProtocolAppliedDoseNumber>,
    #[serde(flatten)]
    series_doses: crate::r4b::choice::Slot<ImmunizationProtocolAppliedSeriesDoses>,
}

impl ::core::convert::From<ImmunizationProtocolAppliedDe> for ImmunizationProtocolApplied {
    fn from(v: ImmunizationProtocolAppliedDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            series: v.series,
            series_ext: v.series_ext,
            authority: v.authority,
            target_disease: v.target_disease,
            dose_number: v.dose_number.0,
            series_doses: v.series_doses.0,
        }
    }
}

/// Categorical data indicating that an adverse event is associated in time to
/// an immunization.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::immunization::ImmunizationReaction;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    pub detail: Option<types::Reference<crate::r4b::resources::Observation>>,

    /// Indicates self-reported reaction
    pub reported: Option<types::Boolean>,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,
}

/// The `Immunization.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `occurrenceString` variant.
    #[fhir("occurrenceString")]
    String(crate::r4b::choice::Primitive<types::String>),
}

/// The `Immunization.protocolApplied.doseNumber[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationProtocolAppliedDoseNumber {
    /// `doseNumberPositiveInt` variant.
    #[fhir("doseNumberPositiveInt")]
    PositiveInt(crate::r4b::choice::Primitive<types::PositiveInt>),
    /// `doseNumberString` variant.
    #[fhir("doseNumberString")]
    String(crate::r4b::choice::Primitive<types::String>),
}

/// The `Immunization.protocolApplied.seriesDoses[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationProtocolAppliedSeriesDoses {
    /// `seriesDosesPositiveInt` variant.
    #[fhir("seriesDosesPositiveInt")]
    PositiveInt(crate::r4b::choice::Primitive<types::PositiveInt>),
    /// `seriesDosesString` variant.
    #[fhir("seriesDosesString")]
    String(crate::r4b::choice::Primitive<types::String>),
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
