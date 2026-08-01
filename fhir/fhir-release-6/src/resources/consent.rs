//! Consent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Consent
//!
//! Version: 6.0.0-ballot3
//!
//! A healthcare consumer's or third party's choices to permit or deny
//! recipients or roles to perform actions for specific purposes and periods of
//! time
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a healthcare consumer’s choices or choices made on their behalf
/// by a third party, which permits or denies identified recipient(s) or
/// recipient role(s) to perform one or more actions within a given policy
/// context, for specific purposes and periods of time.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::Consent;
/// use fhir::r6::types;
///
/// let value = Consent {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: Consent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Consent {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for this record (external references)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | inactive | not-done | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::ConsentStateCodes>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the consent statement - for indexing/retrieval
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who the consent applies to
    pub subject: Option<types::Reference>,

    /// Fully executed date of the consent
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Effective period for this Consent
    pub period: Option<types::Period>,

    /// Who is granting rights according to the policy and rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grantor: Vec<types::Reference>,

    /// Who is agreeing to the policy and rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grantee: Vec<types::Reference>,

    /// Consent workflow management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manager: Vec<types::Reference>,

    /// Consent Enforcer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controller: Vec<types::Reference>,

    /// Source from which this consent is taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_attachment: Vec<types::Attachment>,

    /// Source from which this consent is taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_reference: Vec<types::Reference>,

    /// Regulations establishing base Consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_basis: Vec<types::CodeableConcept>,

    /// Computable version of the backing policy
    pub policy_basis: Option<ConsentPolicyBasis>,

    /// Human Readable Policy
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_text: Vec<types::Reference>,

    /// Consent Verified by patient or family
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification: Vec<ConsentVerification>,

    /// deny | permit
    pub decision: Option<crate::coded::Coded<crate::r6::codes::ConsentProvisionType>>,
    /// Primitive extension sibling for [`decision`](Self::decision) (FHIR `_decision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_decision")]
    pub decision_ext: Option<types::Element>,

    /// Permission Resource for provisions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision_reference: Vec<types::Reference>,

    /// Constraints to the base Consent.policyRule/Consent.policy
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision: Vec<ConsentProvision>,
}

/// A Reference or URL used to uniquely identify the policy the organization
/// will enforce for this Consent. This Reference or URL should be specific to
/// the version of the policy and should be dereferencable to a computable
/// policy of some form.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::ConsentPolicyBasis;
/// use fhir::r6::types;
///
/// let value = ConsentPolicyBasis {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: ConsentPolicyBasis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConsentPolicyBasis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference backing policy resource
    pub reference: Option<types::Reference>,

    /// URI to a computable backing policy
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,
}

/// An exception to the base policy of this consent. An exception can be an
/// addition or removal of access permissions.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::ConsentProvision;
/// use fhir::r6::types;
///
/// let value = ConsentProvision {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentProvision = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConsentProvision {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Timeframe for this provision
    pub period: Option<types::Period>,

    /// Who|what controlled by this provision (or group, by role)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ConsentProvisionActor>,

    /// Actions controlled by this provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Security Labels that define affected resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Context of activities covered by this provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::Coding>,

    /// e.g. Resource Type, Profile, CDA, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_type: Vec<types::Coding>,

    /// e.g. Resource Type, Profile, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type: Vec<types::Coding>,

    /// e.g. LOINC or SNOMED CT code, etc. in the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Timeframe for data controlled by this provision
    pub data_period: Option<types::Period>,

    /// Data controlled by this provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ConsentProvisionData>,

    /// A computable expression of the consent
    pub expression: Option<types::Expression>,

    /// Nested Exception Provisions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision: Vec<ConsentProvision>,
}

/// Who or what is controlled by this provision. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::ConsentProvisionActor;
/// use fhir::r6::types;
///
/// let value = ConsentProvisionActor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentProvisionActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConsentProvisionActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How the actor is involved
    pub role: Option<types::CodeableConcept>,

    /// Resource for the actor (or group, by role)
    pub reference: Option<types::Reference>,
}

/// The resources controlled by this provision if specific resources are
/// referenced.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::ConsentProvisionData;
/// use fhir::r6::types;
///
/// let value = ConsentProvisionData {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentProvisionData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConsentProvisionData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// instance | related | dependents | authoredby
    pub meaning: crate::coded::Coded<crate::r6::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// Whether a treatment instruction (e.g. artificial respiration: yes or no)
/// was verified with the patient, his/her family or another authorized person.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::consent::ConsentVerification;
/// use fhir::r6::types;
///
/// let value = ConsentVerification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentVerification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConsentVerification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Has been verified
    pub verified: types::Boolean,
    /// Primitive extension sibling for [`verified`](Self::verified) (FHIR `_verified`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verified")]
    pub verified_ext: Option<types::Element>,

    /// Business case of verification
    pub verification_type: Option<types::CodeableConcept>,

    /// Person conducting verification
    pub verified_by: Option<types::Reference>,

    /// Person who verified
    pub verified_with: Option<types::Reference>,

    /// When consent verified
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_date: Vec<types::DateTime>,
    /// Primitive extension sibling for [`verification_date`](Self::verification_date) (FHIR `_verificationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verificationDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_date_ext: Vec<Option<types::Element>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Consent;

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
