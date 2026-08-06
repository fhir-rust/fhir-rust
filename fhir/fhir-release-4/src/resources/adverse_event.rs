//! AdverseEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
//!
//! Version: 4.0.1
//!
//! Medical care, research study or other healthcare event causing physical
//! injury
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Actual or potential/avoided event causing unintended physical injury
/// resulting from or contributed to by medical care, a research study or other
/// healthcare setting factors that requires additional monitoring, treatment,
/// or hospitalization, or that results in death.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::adverse_event::AdverseEvent;
/// use fhir::r4::types;
///
/// let value = AdverseEvent {
///     recorded_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recordedDate` is the name this serializes to on the wire.
/// assert_eq!(json["recordedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AdverseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    pub identifier: Option<types::Identifier>,

    /// actual | potential
    pub actuality: crate::coded::Coded<crate::r4::codes::AdverseEventActuality>,
    /// Primitive extension sibling for [`actuality`](Self::actuality) (FHIR `_actuality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actuality")]
    pub actuality_ext: Option<types::Element>,

    /// product-problem | product-quality | product-use-error | wrong-dose |
    /// incorrect-prescribing-information | wrong-technique |
    /// wrong-route-of-administration | wrong-rate | wrong-duration |
    /// wrong-time | expired-drug | medical-device-use-error |
    /// problem-different-manufacturer | unsafe-physical-environment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of the event itself in relation to the subject
    pub event: Option<types::CodeableConcept>,

    /// Subject impacted by event
    pub subject: types::Reference,

    /// Encounter created as part of
    pub encounter: Option<types::Reference>,

    /// When the event occurred
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// When the event was detected
    pub detected: Option<types::DateTime>,
    /// Primitive extension sibling for [`detected`](Self::detected) (FHIR `_detected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detected")]
    pub detected_ext: Option<types::Element>,

    /// When the event was recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Effect on the subject due to this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resulting_condition: Vec<types::Reference>,

    /// Location where adverse event occurred
    pub location: Option<types::Reference>,

    /// Seriousness of the event
    pub seriousness: Option<types::CodeableConcept>,

    /// mild | moderate | severe
    pub severity: Option<types::CodeableConcept>,

    /// resolved | recovering | ongoing | resolvedWithSequelae | fatal |
    /// unknown
    pub outcome: Option<types::CodeableConcept>,

    /// Who recorded the adverse event
    pub recorder: Option<types::Reference>,

    /// Who was involved in the adverse event or the potential adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<types::Reference>,

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
/// use fhir::r4::resources::adverse_event::AdverseEventSuspectEntity;
/// use fhir::r4::types;
///
/// let value = AdverseEventSuspectEntity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSuspectEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Refers to the specific entity that caused the adverse event
    pub instance: types::Reference,

    /// Information on the possible cause of the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causality: Vec<AdverseEventSuspectEntityCausality>,
}

/// Information on the possible cause of the event.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::adverse_event::AdverseEventSuspectEntityCausality;
/// use fhir::r4::types;
///
/// let value = AdverseEventSuspectEntityCausality {
///     product_relatedness: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productRelatedness` is the name this serializes to on the wire.
/// assert_eq!(json["productRelatedness"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSuspectEntityCausality = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Assessment of if the entity caused the event
    pub assessment: Option<types::CodeableConcept>,

    /// AdverseEvent.suspectEntity.causalityProductRelatedness
    pub product_relatedness: Option<types::String>,
    /// Primitive extension sibling for [`product_relatedness`](Self::product_relatedness) (FHIR `_productRelatedness`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productRelatedness")]
    pub product_relatedness_ext: Option<types::Element>,

    /// AdverseEvent.suspectEntity.causalityAuthor
    pub author: Option<types::Reference>,

    /// ProbabilityScale | Bayesian | Checklist
    pub method: Option<types::CodeableConcept>,
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
