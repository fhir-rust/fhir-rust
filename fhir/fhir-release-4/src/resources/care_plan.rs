//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//! Version: 4.0.1
//!
//! Healthcare plan for patient or group
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the intention of how one or more practitioners intend to deliver
/// care for a particular patient, group or community for a period of time,
/// possibly limited to care for a specific condition or set of conditions.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::care_plan::CarePlan;
/// use fhir::r4::types;
///
/// let value = CarePlan {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: CarePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct CarePlan {
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

    /// External Ids for this plan
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

    /// Fulfills CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// CarePlan replaced by this CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Part of referenced CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// draft | active | on-hold | revoked | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r4::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order | option
    pub intent: types::Code,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Human-friendly name for the care plan
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Summary of nature of plan
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who the care plan is for
    pub subject: types::Reference,

    /// Encounter created as part of
    pub encounter: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Date record was first recorded
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Who is the designated responsible party
    pub author: Option<types::Reference>,

    /// Who provided the content of the care plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<types::Reference>,

    /// Who's involved in plan?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference>,

    /// Health issues this plan addresses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference>,

    /// Information considered as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Desired outcome of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference>,

    /// Action to occur as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<CarePlanActivity>,

    /// Comments about the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Identifies a planned action to occur as part of the plan. For example, a
/// medication to be used, lab tests to perform, self-monitoring, education,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::care_plan::CarePlanActivity;
/// use fhir::r4::types;
///
/// let value = CarePlanActivity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CarePlanActivity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct CarePlanActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Results of the activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_codeable_concept: Vec<types::CodeableConcept>,

    /// Appointment, Encounter, Procedure, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_reference: Vec<types::Reference>,

    /// Comments about the activity status/progress
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress: Vec<types::Annotation>,

    /// Activity details defined in specific resource
    pub reference: Option<types::Reference>,

    /// In-line definition of activity
    pub detail: Option<CarePlanActivityDetail>,
}

/// A simple summary of a planned activity suitable for a general care plan
/// system (e.g. form driven) that doesn't know about specific resources such
/// as procedure etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::care_plan::CarePlanActivityDetail;
/// use fhir::r4::types;
///
/// let value = CarePlanActivityDetail {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: CarePlanActivityDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct CarePlanActivityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Appointment | CommunicationRequest | DeviceRequest | MedicationRequest
    /// | NutritionOrder | Task | ServiceRequest | VisionPrescription
    pub kind: Option<types::Code>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

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

    /// Detail type of activity
    pub code: Option<types::CodeableConcept>,

    /// Why activity should be done or why activity was prohibited
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why activity is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Goals this activity relates to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference>,

    /// not-started | scheduled | in-progress | on-hold | completed | cancelled
    /// | stopped | unknown | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::CarePlanActivityStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// If true, activity is prohibiting action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// When activity is to occur
    /// The `CarePlan.activity.detail.scheduled[x]` choice element (0..1); see [`CarePlanActivityDetailScheduled`].
    #[serde(flatten)]
    pub scheduled: Option<CarePlanActivityDetailScheduled>,

    /// Where it should happen
    pub location: Option<types::Reference>,

    /// Who will be responsible?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// What is to be administered/supplied
    /// The `CarePlan.activity.detail.product[x]` choice element (0..1); see [`CarePlanActivityDetailProduct`].
    #[serde(flatten)]
    pub product: Option<CarePlanActivityDetailProduct>,

    /// How to consume/day?
    pub daily_amount: Option<types::Quantity>,

    /// How much to administer/supply/consume
    pub quantity: Option<types::Quantity>,

    /// Extra info describing activity to perform
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// The `CarePlan.activity.detail.scheduled[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum CarePlanActivityDetailScheduled {
    /// `scheduledTiming` variant.
    #[fhir("scheduledTiming")]
    Timing(Box<types::Timing>),
    /// `scheduledPeriod` variant.
    #[fhir("scheduledPeriod")]
    Period(Box<types::Period>),
    /// `scheduledString` variant.
    #[fhir("scheduledString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `CarePlan.activity.detail.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum CarePlanActivityDetailProduct {
    /// `productCodeableConcept` variant.
    #[fhir("productCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `productReference` variant.
    #[fhir("productReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CarePlan;

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
