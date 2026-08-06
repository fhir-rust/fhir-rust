//! AdverseEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
//!
//!
//!
//! Medical care, research study or other healthcare event causing physical
//! injury
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for AdverseEvent Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::adverse_event::AdverseEvent;
/// use fhir::r3::types;
///
/// let value = AdverseEvent {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AdverseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    pub identifier: Option<types::Identifier>,

    /// AE | PAE An adverse event is an event that caused harm to a patient, an
    /// adverse reaction is a something that is a subject-specific event that
    /// is a result of an exposure to a medication, food, device or
    /// environmental substance, a potential adverse event is something that
    /// occurred and that could have caused harm to a patient but did not
    pub category: Option<crate::coded::Coded<crate::r3::codes::AdverseEventCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// actual | potential
    pub r#type: Option<types::CodeableConcept>,

    /// Subject or group impacted by event
    pub subject: Option<types::Reference>,

    /// When the event occurred
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Adverse Reaction Events linked to exposure to substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<types::Reference>,

    /// Location where adverse event occurred
    pub location: Option<types::Reference>,

    /// Mild | Moderate | Severe
    pub seriousness: Option<types::CodeableConcept>,

    /// resolved | recovering | ongoing | resolvedWithSequelae | fatal |
    /// unknown
    pub outcome: Option<types::CodeableConcept>,

    /// Who recorded the adverse event
    pub recorder: Option<types::Reference>,

    /// Who was involved in the adverse event or the potential adverse event
    pub event_participant: Option<types::Reference>,

    /// Description of the adverse event
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The suspected agent causing the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suspect_entity: Vec<AdverseEventSuspectEntity>,

    /// AdverseEvent.subjectMedicalHistory
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_medical_history: Vec<types::Reference>,

    /// AdverseEvent.referenceDocument
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_document: Vec<types::Reference>,

    /// AdverseEvent.study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<types::Reference>,
}

/// Describes the entity that is suspected to have caused the adverse event.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::adverse_event::AdverseEventSuspectEntity;
/// use fhir::r3::types;
///
/// let value = AdverseEventSuspectEntity {
///     causality_product_relatedness: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `causalityProductRelatedness` is the name this serializes to on the wire.
/// assert_eq!(json["causalityProductRelatedness"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSuspectEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AdverseEventSuspectEntity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Refers to the specific entity that caused the adverse event
    pub instance: types::Reference,

    /// causality1 | causality2
    pub causality: Option<crate::coded::Coded<crate::r3::codes::AdverseEventCausality>>,
    /// Primitive extension sibling for [`causality`](Self::causality) (FHIR `_causality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_causality")]
    pub causality_ext: Option<types::Element>,

    /// assess1 | assess2
    pub causality_assessment: Option<types::CodeableConcept>,

    /// AdverseEvent.suspectEntity.causalityProductRelatedness
    pub causality_product_relatedness: Option<types::String>,
    /// Primitive extension sibling for [`causality_product_relatedness`](Self::causality_product_relatedness) (FHIR `_causalityProductRelatedness`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_causalityProductRelatedness")]
    pub causality_product_relatedness_ext: Option<types::Element>,

    /// method1 | method2
    pub causality_method: Option<types::CodeableConcept>,

    /// AdverseEvent.suspectEntity.causalityAuthor
    pub causality_author: Option<types::Reference>,

    /// result1 | result2
    pub causality_result: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdverseEvent;

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
