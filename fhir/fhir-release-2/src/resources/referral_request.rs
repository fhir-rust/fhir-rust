//! ReferralRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ReferralRequest
//!
//!
//!
//! A request for referral or transfer of care
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ReferralRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::referral_request::ReferralRequest;
/// use fhir::r2::types;
///
/// let value = ReferralRequest {
///     date_sent: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateSent` is the name this serializes to on the wire.
/// assert_eq!(json["dateSent"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ReferralRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// draft | requested | active | cancelled | accepted | rejected |
    /// completed
    pub status: crate::coded::Coded<crate::r2::codes::Referralstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Date of creation/activation
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Referral/Transition of care request type
    pub r#type: Option<types::CodeableConcept>,

    /// The clinical specialty (discipline) that the referral is requested for
    pub specialty: Option<types::CodeableConcept>,

    /// Urgency of referral / transfer of care request
    pub priority: Option<types::CodeableConcept>,

    /// Patient referred to care or transfer
    pub patient: Option<types::Reference>,

    /// Requester of referral / transfer of care
    pub requester: Option<types::Reference>,

    /// Receiver of referral / transfer of care request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// Originating encounter
    pub encounter: Option<types::Reference>,

    /// Date referral/transfer of care request is sent
    pub date_sent: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_sent`](Self::date_sent) (FHIR `_dateSent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateSent")]
    pub date_sent_ext: Option<types::Element>,

    /// Reason for referral / transfer of care request
    pub reason: Option<types::CodeableConcept>,

    /// A textual description of the referral
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Actions requested as part of the referral
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_requested: Vec<types::CodeableConcept>,

    /// Additonal information to support referral or transfer of care request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Requested service(s) fulfillment time
    pub fulfillment_time: Option<types::Period>,
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
