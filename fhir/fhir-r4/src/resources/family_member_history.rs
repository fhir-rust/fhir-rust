//! FamilyMemberHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
//!
//! Version: 4.0.1
//!
//! Information about patient's relatives, relevant for patient
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Significant health conditions for a person related to the patient relevant
/// in the context of care for the patient.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::family_member_history::FamilyMemberHistory;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct FamilyMemberHistory {
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
    pub contained: Vec<crate::r4::resources::Resource>,

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
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// partial | completed | entered-in-error | health-unknown
    pub status: crate::coded::Coded<crate::r4::codes::HistoryStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// subject-unknown | withheld | unable-to-obtain | deferred
    pub data_absent_reason: Option<types::CodeableConcept>,

    /// Patient history is about
    pub patient: types::Reference<crate::r4::resources::Patient>,

    /// When history was recorded or last updated
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
    pub sex: Option<types::CodeableConcept>,

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

/// The significant Conditions (or condition) that the family member had. This
/// is a repeating section to allow a system to represent more than one
/// condition per resource, though there is nothing stopping multiple resources
/// \- one per condition.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::family_member_history::FamilyMemberHistoryCondition;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// deceased | permanent disability | etc.
    pub outcome: Option<types::CodeableConcept>,

    /// Whether the condition contributed to the cause of death
    pub contributed_to_death: Option<types::Boolean>,
    /// Primitive extension sibling for [`contributed_to_death`](Self::contributed_to_death) (FHIR `_contributedToDeath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contributedToDeath")]
    pub contributed_to_death_ext: Option<types::Element>,

    /// When condition first manifested
    /// The `FamilyMemberHistory.condition.onset[x]` choice element (0..1); see [`FamilyMemberHistoryConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<FamilyMemberHistoryConditionOnset>,

    /// Extra information about condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The `FamilyMemberHistory.born[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryBorn {
    /// `bornPeriod` variant.
    #[fhir("bornPeriod")]
    Period(Box<types::Period>),
    /// `bornDate` variant.
    #[fhir("bornDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `bornString` variant.
    #[fhir("bornString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.age[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum FamilyMemberHistoryDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r4::choice::Primitive<types::Boolean>),
    /// `deceasedAge` variant.
    #[fhir("deceasedAge")]
    Age(Box<types::Age>),
    /// `deceasedRange` variant.
    #[fhir("deceasedRange")]
    Range(Box<types::Range>),
    /// `deceasedDate` variant.
    #[fhir("deceasedDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `deceasedString` variant.
    #[fhir("deceasedString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `FamilyMemberHistory.condition.onset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
    String(crate::r4::choice::Primitive<types::String>),
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
