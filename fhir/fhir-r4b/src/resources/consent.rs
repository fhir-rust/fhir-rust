//! Consent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Consent
//!
//! Version: 4.3.0
//!
//! A healthcare consumer's choices to permit or deny recipients or roles to
//! perform actions for specific purposes and periods of time
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a healthcare consumer’s choices, which permits or denies
/// identified recipient(s) or recipient role(s) to perform one or more actions
/// within a given policy context, for specific purposes and periods of time.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::consent::Consent;
///
/// let value = Consent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Consent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ConsentDe")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for this record (external references)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | proposed | active | rejected | inactive | entered-in-error
    pub status: crate::coded::Coded<crate::r4b::codes::ConsentStateCodes>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Which of the four areas this resource covers (extensible)
    pub scope: types::CodeableConcept,

    /// Classification of the consent statement - for indexing/retrieval
    pub category: ::vec1::Vec1<types::CodeableConcept>,

    /// Who the consent applies to
    pub patient: Option<types::Reference<crate::r4b::resources::Patient>>,

    /// When this Consent was created or indexed
    pub date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who is agreeing to the policy and rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Custodian of the consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organization: Vec<types::Reference<crate::r4b::resources::Organization>>,

    /// Source from which this consent is taken
    /// The `Consent.source[x]` choice element (0..1); see [`ConsentSource`].
    #[serde(flatten)]
    pub source: Option<ConsentSource>,

    /// Policies covered by this consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<ConsentPolicy>,

    /// Regulation that this consents to
    pub policy_rule: Option<types::CodeableConcept>,

    /// Consent Verified by patient or family
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification: Vec<ConsentVerification>,

    /// Constraints to the base Consent.policyRule
    pub provision: Option<ConsentProvision>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConsentDe {
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
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r4b::codes::ConsentStateCodes>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    scope: types::CodeableConcept,
    category: ::vec1::Vec1<types::CodeableConcept>,
    patient: Option<types::Reference<crate::r4b::resources::Patient>>,
    date_time: Option<types::DateTime>,
    #[serde(rename = "_dateTime")]
    date_time_ext: Option<types::Element>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(default)]
    organization: Vec<types::Reference<crate::r4b::resources::Organization>>,
    #[serde(flatten)]
    source: crate::r4b::choice::Slot<ConsentSource>,
    #[serde(default)]
    policy: Vec<ConsentPolicy>,
    policy_rule: Option<types::CodeableConcept>,
    #[serde(default)]
    verification: Vec<ConsentVerification>,
    provision: Option<ConsentProvision>,
}

impl ::core::convert::From<ConsentDe> for Consent {
    fn from(v: ConsentDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            scope: v.scope,
            category: v.category,
            patient: v.patient,
            date_time: v.date_time,
            date_time_ext: v.date_time_ext,
            performer: v.performer,
            organization: v.organization,
            source: v.source.0,
            policy: v.policy,
            policy_rule: v.policy_rule,
            verification: v.verification,
            provision: v.provision,
        }
    }
}

/// The references to the policies that are included in this consent scope.
/// Policies may be organizational, but are often defined jurisdictionally, or
/// in law.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::consent::ConsentPolicy;
/// use fhir::r4b::types;
///
/// let value = ConsentPolicy {
///     authority: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authority` is the name this serializes to on the wire.
/// assert_eq!(json["authority"], ::serde_json::json!("http://example.org"));
///
/// let back: ConsentPolicy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ConsentPolicy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Enforcement source for policy
    pub authority: Option<types::Uri>,
    /// Primitive extension sibling for [`authority`](Self::authority) (FHIR `_authority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authority")]
    pub authority_ext: Option<types::Element>,

    /// Specific policy covered by this consent
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
/// use fhir::r4b::resources::consent::ConsentProvision;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct ConsentProvision {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// deny | permit
    pub r#type: Option<crate::coded::Coded<crate::r4b::codes::ConsentProvisionType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Timeframe for this rule
    pub period: Option<types::Period>,

    /// Who|what controlled by this rule (or group, by role)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ConsentProvisionActor>,

    /// Actions controlled by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Security Labels that define affected resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Context of activities covered by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::Coding>,

    /// e.g. Resource Type, Profile, CDA, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::Coding>,

    /// e.g. LOINC or SNOMED CT code, etc. in the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Timeframe for data controlled by this rule
    pub data_period: Option<types::Period>,

    /// Data controlled by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ConsentProvisionData>,

    /// Nested Exception Rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provision: Vec<ConsentProvision>,
}

/// Who or what is controlled by this rule. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::consent::ConsentProvisionActor;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    pub role: types::CodeableConcept,

    /// Resource for the actor (or group, by role)
    pub reference: types::Reference,
}

/// The resources controlled by this rule if specific resources are referenced.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::consent::ConsentProvisionData;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    pub meaning: crate::coded::Coded<crate::r4b::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// Whether a treatment instruction (e.g. artificial respiration yes or no) was
/// verified with the patient, his/her family or another authorized person.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::consent::ConsentVerification;
/// use fhir::r4b::types;
///
/// let value = ConsentVerification {
///     verification_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `verificationDate` is the name this serializes to on the wire.
/// assert_eq!(json["verificationDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ConsentVerification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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

    /// Person who verified
    pub verified_with: Option<types::Reference>,

    /// When consent verified
    pub verification_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`verification_date`](Self::verification_date) (FHIR `_verificationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verificationDate")]
    pub verification_date_ext: Option<types::Element>,
}

/// The `Consent.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ConsentSource {
    /// `sourceAttachment` variant.
    #[fhir("sourceAttachment")]
    Attachment(Box<types::Attachment>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}
