//! FamilyMemberHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
//!
//! Version: 5.0.0
//!
//! FamilyMemberHistory Resource: Significant health conditions for a person related to the patient relevant in the context of care for the patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// FamilyMemberHistory captures significant health conditions for a person
/// related to the patient that are relevant to the patient's care. It records
/// the family relationship, relevant demographic details of the family member,
/// and the conditions, procedures, and other observations pertinent to
/// assessing hereditary or familial risk. It is commonly used to build a
/// family health history for genetic or preventive care planning.
///
/// Clinically, FamilyMemberHistory is used to document what is known about a
/// relative's health rather than to represent the relative as a patient in
/// their own right; the family member is described inline via the
/// `relationship`, `name`, `sex`, and age/birth fields rather than by a
/// separate resource instance. This makes the resource well suited for
/// genetic risk assessment, pedigree/family tree construction, and clinical
/// decision support that depends on hereditary risk factors, such as flagging
/// an elevated likelihood of a condition based on a parent's or sibling's
/// history. Each significant condition or procedure experienced by the family
/// member can be recorded as a repeating backbone element
/// ([`FamilyMemberHistoryCondition`] or [`FamilyMemberHistoryProcedure`]),
/// including onset, outcome, and whether it contributed to the family
/// member's death.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject whose
///   family history this resource describes, referenced via `patient`.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used
///   extensively here for relationship, sex, condition, and outcome coding.
/// - `Condition` and `Procedure` — related resources for documenting the
///   patient's own clinical history, as opposed to a relative's.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::family_member_history::FamilyMemberHistory;
/// use fhir::r5::types;
///
/// let value = FamilyMemberHistory {
///     estimated_age: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `estimatedAge` is the name this serializes to on the wire.
/// assert_eq!(json["estimatedAge"], ::serde_json::json!(true));
///
/// let back: FamilyMemberHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "FamilyMemberHistoryDe")]
pub struct FamilyMemberHistory {
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

    /// External Id(s) for this record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// The workflow/business status of this record: partial | completed |
    /// entered-in-error | health-unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::HistoryStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// subject-unknown | withheld | unable-to-obtain | deferred
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// Reference to the [`Patient`](crate::r5::resources::patient::Patient)
    /// whose family member history is being recorded.
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// When history was recorded or last updated
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who or what participated in the activities related to the family member history and how they were involved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<FamilyMemberHistoryParticipant>,

    /// The family member described
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The coded family relationship of this person to the patient, such as
    /// mother, father, or sibling.
    pub relationship: types::CodeableConcept,

    /// male | female | other | unknown
    pub sex: Option<types::CodeableConcept>,

    /// The `FamilyMemberHistory.born[x]` choice element (0..1); see [`FamilyMemberHistoryBorn`].
    #[serde(flatten)]
    pub born: Option<FamilyMemberHistoryBorn>,

    /// The `FamilyMemberHistory.age[x]` choice element (0..1); see [`FamilyMemberHistoryAge`].
    #[serde(flatten)]
    pub age: Option<FamilyMemberHistoryAge>,

    /// Age is estimated?
    pub estimated_age: Option<types::Boolean>,
    /// Primitive extension sibling for [`estimated_age`](Self::estimated_age) (FHIR `_estimatedAge`).
    #[serde(rename = "_estimatedAge")]
    pub estimated_age_ext: Option<types::Element>,

    /// The `FamilyMemberHistory.deceased[x]` choice element (0..1); see [`FamilyMemberHistoryDeceased`].
    #[serde(flatten)]
    pub deceased: Option<FamilyMemberHistoryDeceased>,

    /// Why was family member history performed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// General note about related person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Condition that the related person had
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<FamilyMemberHistoryCondition>,

    /// Procedures that the related person had
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<FamilyMemberHistoryProcedure>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FamilyMemberHistoryDe {
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
    instantiates_canonical: Vec<types::Canonical>,
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default)]
    instantiates_canonical_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    instantiates_uri: Vec<types::Uri>,
    #[serde(rename = "_instantiatesUri")]
    #[serde(default)]
    instantiates_uri_ext: Vec<Option<types::Element>>,
    status: crate::r5::coded::Coded<crate::r5::codes::HistoryStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    data_absent_reason: Option<types::CodeableConcept>,
    patient: types::Reference<crate::r5::resources::Patient>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    #[serde(default)]
    participant: Vec<FamilyMemberHistoryParticipant>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    relationship: types::CodeableConcept,
    sex: Option<types::CodeableConcept>,
    #[serde(flatten)]
    born: crate::r5::choice::Slot<FamilyMemberHistoryBorn>,
    #[serde(flatten)]
    age: crate::r5::choice::Slot<FamilyMemberHistoryAge>,
    estimated_age: Option<types::Boolean>,
    #[serde(rename = "_estimatedAge")]
    estimated_age_ext: Option<types::Element>,
    #[serde(flatten)]
    deceased: crate::r5::choice::Slot<FamilyMemberHistoryDeceased>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    condition: Vec<FamilyMemberHistoryCondition>,
    #[serde(default)]
    procedure: Vec<FamilyMemberHistoryProcedure>,
}

impl ::core::convert::From<FamilyMemberHistoryDe> for FamilyMemberHistory {
    fn from(v: FamilyMemberHistoryDe) -> Self {
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
            instantiates_canonical: v.instantiates_canonical,
            instantiates_canonical_ext: v.instantiates_canonical_ext,
            instantiates_uri: v.instantiates_uri,
            instantiates_uri_ext: v.instantiates_uri_ext,
            status: v.status,
            status_ext: v.status_ext,
            data_absent_reason: v.data_absent_reason,
            patient: v.patient,
            date: v.date,
            date_ext: v.date_ext,
            participant: v.participant,
            name: v.name,
            name_ext: v.name_ext,
            relationship: v.relationship,
            sex: v.sex,
            born: v.born.0,
            age: v.age.0,
            estimated_age: v.estimated_age,
            estimated_age_ext: v.estimated_age_ext,
            deceased: v.deceased.0,
            reason: v.reason,
            note: v.note,
            condition: v.condition,
            procedure: v.procedure,
        }
    }
}

/// Who or what participated in the activities related to the family member
/// history and how they were involved.
/// # Examples
///
/// ```
/// use fhir::r5::resources::family_member_history::FamilyMemberHistoryParticipant;
/// use fhir::r5::types;
///
/// let value = FamilyMemberHistoryParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: FamilyMemberHistoryParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistoryParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the family member history
    pub actor: types::Reference,
}

/// Condition that the related person had. This is a backbone element describing
/// the significant conditions suffered by the family member.
/// # Examples
///
/// ```
/// use fhir::r5::resources::family_member_history::FamilyMemberHistoryCondition;
/// use fhir::r5::types;
///
/// let value = FamilyMemberHistoryCondition {
///     contributed_to_death: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `contributedToDeath` is the name this serializes to on the wire.
/// assert_eq!(json["contributedToDeath"], ::serde_json::json!(true));
///
/// let back: FamilyMemberHistoryCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "FamilyMemberHistoryConditionDe")]
pub struct FamilyMemberHistoryCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Condition suffered by relation
    pub code: types::CodeableConcept,

    /// deceased | permanent disability | etc
    pub outcome: Option<types::CodeableConcept>,

    /// Whether the condition contributed to the cause of death
    pub contributed_to_death: Option<types::Boolean>,
    /// Primitive extension sibling for [`contributed_to_death`](Self::contributed_to_death) (FHIR `_contributedToDeath`).
    #[serde(rename = "_contributedToDeath")]
    pub contributed_to_death_ext: Option<types::Element>,

    /// The `FamilyMemberHistory.condition.onset[x]` choice element (0..1); see [`FamilyMemberHistoryConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<FamilyMemberHistoryConditionOnset>,

    /// Extra information about condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FamilyMemberHistoryConditionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    outcome: Option<types::CodeableConcept>,
    contributed_to_death: Option<types::Boolean>,
    #[serde(rename = "_contributedToDeath")]
    contributed_to_death_ext: Option<types::Element>,
    #[serde(flatten)]
    onset: crate::r5::choice::Slot<FamilyMemberHistoryConditionOnset>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<FamilyMemberHistoryConditionDe> for FamilyMemberHistoryCondition {
    fn from(v: FamilyMemberHistoryConditionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            outcome: v.outcome,
            contributed_to_death: v.contributed_to_death,
            contributed_to_death_ext: v.contributed_to_death_ext,
            onset: v.onset.0,
            note: v.note,
        }
    }
}

/// Procedures that the related person had. This is a backbone element describing
/// the significant procedures performed on the family member.
/// # Examples
///
/// ```
/// use fhir::r5::resources::family_member_history::FamilyMemberHistoryProcedure;
/// use fhir::r5::types;
///
/// let value = FamilyMemberHistoryProcedure {
///     contributed_to_death: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `contributedToDeath` is the name this serializes to on the wire.
/// assert_eq!(json["contributedToDeath"], ::serde_json::json!(true));
///
/// let back: FamilyMemberHistoryProcedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "FamilyMemberHistoryProcedureDe")]
pub struct FamilyMemberHistoryProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Procedures performed on the related person
    pub code: types::CodeableConcept,

    /// What happened following the procedure
    pub outcome: Option<types::CodeableConcept>,

    /// Whether the procedure contributed to the cause of death
    pub contributed_to_death: Option<types::Boolean>,
    /// Primitive extension sibling for [`contributed_to_death`](Self::contributed_to_death) (FHIR `_contributedToDeath`).
    #[serde(rename = "_contributedToDeath")]
    pub contributed_to_death_ext: Option<types::Element>,

    /// The `FamilyMemberHistory.procedure.performed[x]` choice element (0..1); see [`FamilyMemberHistoryProcedurePerformed`].
    #[serde(flatten)]
    pub performed: Option<FamilyMemberHistoryProcedurePerformed>,

    /// Extra information about the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FamilyMemberHistoryProcedureDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    outcome: Option<types::CodeableConcept>,
    contributed_to_death: Option<types::Boolean>,
    #[serde(rename = "_contributedToDeath")]
    contributed_to_death_ext: Option<types::Element>,
    #[serde(flatten)]
    performed: crate::r5::choice::Slot<FamilyMemberHistoryProcedurePerformed>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<FamilyMemberHistoryProcedureDe> for FamilyMemberHistoryProcedure {
    fn from(v: FamilyMemberHistoryProcedureDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            outcome: v.outcome,
            contributed_to_death: v.contributed_to_death,
            contributed_to_death_ext: v.contributed_to_death_ext,
            performed: v.performed.0,
            note: v.note,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = FamilyMemberHistory;

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
/// The `FamilyMemberHistory.age[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryAge {
    /// `ageAge` variant.
    #[fhir("ageAge")]
    Age(Box<types::Age>),
    /// `ageRange` variant.
    #[fhir("ageRange")]
    Range(Box<types::Range>),
    /// `ageString` variant.
    #[fhir("ageString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.born[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryBorn {
    /// `bornPeriod` variant.
    #[fhir("bornPeriod")]
    Period(Box<types::Period>),
    /// `bornDate` variant.
    #[fhir("bornDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `bornString` variant.
    #[fhir("bornString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.condition.onset[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryConditionOnset {
    /// `onsetAge` variant.
    #[fhir("onsetAge")]
    Age(Box<types::Age>),
    /// `onsetRange` variant.
    #[fhir("onsetRange")]
    Range(Box<types::Range>),
    /// `onsetPeriod` variant.
    #[fhir("onsetPeriod")]
    Period(Box<types::Period>),
    /// `onsetString` variant.
    #[fhir("onsetString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.deceased[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `deceasedAge` variant.
    #[fhir("deceasedAge")]
    Age(Box<types::Age>),
    /// `deceasedRange` variant.
    #[fhir("deceasedRange")]
    Range(Box<types::Range>),
    /// `deceasedDate` variant.
    #[fhir("deceasedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `deceasedString` variant.
    #[fhir("deceasedString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.procedure.performed[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryProcedurePerformed {
    /// `performedAge` variant.
    #[fhir("performedAge")]
    Age(Box<types::Age>),
    /// `performedRange` variant.
    #[fhir("performedRange")]
    Range(Box<types::Range>),
    /// `performedPeriod` variant.
    #[fhir("performedPeriod")]
    Period(Box<types::Period>),
    /// `performedString` variant.
    #[fhir("performedString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `performedDateTime` variant.
    #[fhir("performedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
