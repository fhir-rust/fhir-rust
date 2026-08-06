//! ReferralRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ReferralRequest
//!
//!
//!
//! A request for referral or transfer of care
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ReferralRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::referral_request::ReferralRequest;
/// use fhir::r3::types;
///
/// let value = ReferralRequest {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ReferralRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ReferralRequest {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// Request fulfilled by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | suspended | cancelled | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r3::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order
    pub intent: crate::coded::Coded<crate::r3::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Referral/Transition of care request type
    pub r#type: Option<types::CodeableConcept>,

    /// Urgency of referral / transfer of care request
    pub priority: Option<crate::coded::Coded<crate::r3::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Actions requested as part of the referral
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_requested: Vec<types::CodeableConcept>,

    /// Patient referred to care or transfer
    pub subject: types::Reference,

    /// Originating encounter
    pub context: Option<types::Reference>,

    /// When the service(s) requested in the referral should occur
    /// The `ReferralRequest.occurrence[x]` choice element (0..1); see [`ReferralRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ReferralRequestOccurrence>,

    /// Date of creation/activation
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who/what is requesting service
    pub requester: Option<ReferralRequestRequester>,

    /// The clinical specialty (discipline) that the referral is requested for
    pub specialty: Option<types::CodeableConcept>,

    /// Receiver of referral / transfer of care request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// Reason for referral / transfer of care request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why is service needed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// A textual description of the referral
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Additonal information to support referral or transfer of care request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Comments made about referral request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Key events in history of request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference>,
}

/// The individual who initiated the request and has responsibility for its
/// activation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::referral_request::ReferralRequestRequester;
/// use fhir::r3::types;
///
/// let value = ReferralRequestRequester {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ReferralRequestRequester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ReferralRequestRequester {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Individual making the request
    pub agent: types::Reference,

    /// Organization agent is acting for
    pub on_behalf_of: Option<types::Reference>,
}

/// The `ReferralRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ReferralRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ReferralRequest;

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
