//! FamilyMemberHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
//!
//!
//!
//! Information about patient's relatives, relevant for patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for FamilyMemberHistory Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::family_member_history::FamilyMemberHistory;
/// use fhir::r3::types;
///
/// let value = FamilyMemberHistory {
///     not_done: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `notDone` is the name this serializes to on the wire.
/// assert_eq!(json["notDone"], ::serde_json::json!(true));
///
/// let back: FamilyMemberHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "FamilyMemberHistoryDe")]
#[fhir_version("r3")]
pub struct FamilyMemberHistory {
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Id(s) for this record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// partial | completed | entered-in-error | health-unknown
    pub status: crate::coded::Coded<crate::r3::codes::HistoryStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The taking of a family member's history did not occur
    pub not_done: Option<types::Boolean>,
    /// Primitive extension sibling for [`not_done`](Self::not_done) (FHIR `_notDone`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notDone")]
    pub not_done_ext: Option<types::Element>,

    /// subject-unknown | withheld | unable-to-obtain | deferred
    pub not_done_reason: Option<types::CodeableConcept>,

    /// Patient history is about
    pub patient: types::Reference<crate::r3::resources::Patient>,

    /// When history was captured/updated
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The family member described
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Relationship to the subject
    pub relationship: types::CodeableConcept,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r3::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// (approximate) date of birth
    /// The `FamilyMemberHistory.born[x]` choice element (0..1); see [`FamilyMemberHistoryBorn`].
    #[serde(flatten)]
    pub born: Option<FamilyMemberHistoryBorn>,

    /// (approximate) age
    /// The `FamilyMemberHistory.age[x]` choice element (0..1); see [`FamilyMemberHistoryAge`].
    #[serde(flatten)]
    pub age: Option<FamilyMemberHistoryAge>,

    /// Age is estimated?
    pub estimated_age: Option<types::Boolean>,
    /// Primitive extension sibling for [`estimated_age`](Self::estimated_age) (FHIR `_estimatedAge`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_estimatedAge")]
    pub estimated_age_ext: Option<types::Element>,

    /// Dead? How old/when?
    /// The `FamilyMemberHistory.deceased[x]` choice element (0..1); see [`FamilyMemberHistoryDeceased`].
    #[serde(flatten)]
    pub deceased: Option<FamilyMemberHistoryDeceased>,

    /// Why was family member history performed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why was family member history performed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// General note about related person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Condition that the related person had
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<FamilyMemberHistoryCondition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FamilyMemberHistoryDe {
    id: Option<types::Id>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r3::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    definition: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r3::codes::HistoryStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    not_done: Option<types::Boolean>,
    #[serde(rename = "_notDone")]
    not_done_ext: Option<types::Element>,
    not_done_reason: Option<types::CodeableConcept>,
    patient: types::Reference<crate::r3::resources::Patient>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    relationship: types::CodeableConcept,
    gender: Option<crate::coded::Coded<crate::r3::codes::AdministrativeGender>>,
    #[serde(rename = "_gender")]
    gender_ext: Option<types::Element>,
    #[serde(flatten)]
    born: crate::r3::choice::Slot<FamilyMemberHistoryBorn>,
    #[serde(flatten)]
    age: crate::r3::choice::Slot<FamilyMemberHistoryAge>,
    estimated_age: Option<types::Boolean>,
    #[serde(rename = "_estimatedAge")]
    estimated_age_ext: Option<types::Element>,
    #[serde(flatten)]
    deceased: crate::r3::choice::Slot<FamilyMemberHistoryDeceased>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    condition: Vec<FamilyMemberHistoryCondition>,
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
            definition: v.definition,
            status: v.status,
            status_ext: v.status_ext,
            not_done: v.not_done,
            not_done_ext: v.not_done_ext,
            not_done_reason: v.not_done_reason,
            patient: v.patient,
            date: v.date,
            date_ext: v.date_ext,
            name: v.name,
            name_ext: v.name_ext,
            relationship: v.relationship,
            gender: v.gender,
            gender_ext: v.gender_ext,
            born: v.born.0,
            age: v.age.0,
            estimated_age: v.estimated_age,
            estimated_age_ext: v.estimated_age_ext,
            deceased: v.deceased.0,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            note: v.note,
            condition: v.condition,
        }
    }
}

/// The significant Conditions (or condition) that the family member had. This
/// is a repeating section to allow a system to represent more than one
/// condition per resource, though there is nothing stopping multiple resources
/// \- one per condition.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::family_member_history::FamilyMemberHistoryCondition;
/// use fhir::r3::types;
///
/// let value = FamilyMemberHistoryCondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: FamilyMemberHistoryCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "FamilyMemberHistoryConditionDe")]
#[fhir_version("r3")]
pub struct FamilyMemberHistoryCondition {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Condition suffered by relation
    pub code: types::CodeableConcept,

    /// deceased | permanent disability | etc.
    pub outcome: Option<types::CodeableConcept>,

    /// When condition first manifested
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
    #[serde(flatten)]
    onset: crate::r3::choice::Slot<FamilyMemberHistoryConditionOnset>,
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
            onset: v.onset.0,
            note: v.note,
        }
    }
}

/// The `FamilyMemberHistory.born[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryBorn {
    /// `bornPeriod` variant.
    #[fhir("bornPeriod")]
    Period(Box<types::Period>),
    /// `bornDate` variant.
    #[fhir("bornDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `bornString` variant.
    #[fhir("bornString")]
    String(crate::r3::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.age[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
    String(crate::r3::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `deceasedAge` variant.
    #[fhir("deceasedAge")]
    Age(Box<types::Age>),
    /// `deceasedRange` variant.
    #[fhir("deceasedRange")]
    Range(Box<types::Range>),
    /// `deceasedDate` variant.
    #[fhir("deceasedDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `deceasedString` variant.
    #[fhir("deceasedString")]
    String(crate::r3::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.condition.onset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
    String(crate::r3::choice::Primitive<types::String>),
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
