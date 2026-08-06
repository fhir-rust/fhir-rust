//! EncounterHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
//!
//! Version: 6.0.0-ballot3
//!
//! A record of significant events/milestones key data throughout the history
//! of an Encounter
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of significant events/milestones key data throughout the history
/// of an Encounter
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter_history::EncounterHistory;
/// use fhir::r6::types;
///
/// let value = EncounterHistory {
///     planned_start_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `plannedStartDate` is the name this serializes to on the wire.
/// assert_eq!(json["plannedStartDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: EncounterHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EncounterHistory {
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

    /// The Encounter associated with this set of historic values
    pub encounter: Option<types::Reference>,

    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// planned | in-progress | on-hold | discharged | completed | cancelled |
    /// discontinued | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of patient encounter
    pub class: types::CodeableConcept,

    /// Specific type of encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specific type of service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The patient or group related to this encounter
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// The actual start and end time associated with this set of values
    /// associated with the encounter
    pub actual_period: Option<types::Period>,

    /// The planned start date/time (or admission date) of the encounter
    pub planned_start_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_start_date`](Self::planned_start_date) (FHIR `_plannedStartDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plannedStartDate")]
    pub planned_start_date_ext: Option<types::Element>,

    /// The planned end date/time (or discharge date) of the encounter
    pub planned_end_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_end_date`](Self::planned_end_date) (FHIR `_plannedEndDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plannedEndDate")]
    pub planned_end_date_ext: Option<types::Element>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// Location of the patient at this point in the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EncounterHistoryLocation>,
}

/// The location of the patient at this point in the encounter, the multiple
/// cardinality permits de-normalizing the levels of the location hierarchy,
/// such as site/ward/room/bed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter_history::EncounterHistoryLocation;
/// use fhir::r6::types;
///
/// let value = EncounterHistoryLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterHistoryLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EncounterHistoryLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location the encounter takes place
    pub location: types::Reference,

    /// The physical type of the location (usually the level in the location
    /// hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EncounterHistory;

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
