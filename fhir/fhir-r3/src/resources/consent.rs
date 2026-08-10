//! Consent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Consent
//!
//!
//!
//! A healthcare consumer's policy choices to permits or denies recipients or
//! roles to perform actions for specific purposes and periods of time
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Consent Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::Consent;
/// use fhir::r3::types;
///
/// let value = Consent {
///     date_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateTime` is the name this serializes to on the wire.
/// assert_eq!(json["dateTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Consent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ConsentDe")]
#[fhir_version("r3")]
pub struct Consent {
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

    /// Identifier for this record (external references)
    pub identifier: Option<types::Identifier>,

    /// draft | proposed | active | rejected | inactive | entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::ConsentStateCodes>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the consent statement - for indexing/retrieval
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who the consent applies to
    pub patient: types::Reference<crate::r3::resources::Patient>,

    /// Period that this consent applies
    pub period: Option<types::Period>,

    /// When this Consent was created or indexed
    pub date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who is agreeing to the policy and exceptions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consenting_party: Vec<types::Reference>,

    /// Who|what controlled by this consent (or group, by role)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ConsentActor>,

    /// Actions controlled by this consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Custodian of the consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organization: Vec<types::Reference<crate::r3::resources::Organization>>,

    /// Source from which this consent is taken
    /// The `Consent.source[x]` choice element (0..1); see [`ConsentSource`].
    #[serde(flatten)]
    pub source: Option<ConsentSource>,

    /// Policies covered by this consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<ConsentPolicy>,

    /// Policy that this consents to
    pub policy_rule: Option<types::Uri>,
    /// Primitive extension sibling for [`policy_rule`](Self::policy_rule) (FHIR `_policyRule`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policyRule")]
    pub policy_rule_ext: Option<types::Element>,

    /// Security Labels that define affected resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Context of activities for which the agreement is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::Coding>,

    /// Timeframe for data controlled by this consent
    pub data_period: Option<types::Period>,

    /// Data controlled by this consent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ConsentData>,

    /// Additional rule - addition or removal of permissions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<ConsentExcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConsentDe {
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
    identifier: Option<types::Identifier>,
    status: crate::coded::Coded<crate::r3::codes::ConsentStateCodes>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    patient: types::Reference<crate::r3::resources::Patient>,
    period: Option<types::Period>,
    date_time: Option<types::DateTime>,
    #[serde(rename = "_dateTime")]
    date_time_ext: Option<types::Element>,
    #[serde(default)]
    consenting_party: Vec<types::Reference>,
    #[serde(default)]
    actor: Vec<ConsentActor>,
    #[serde(default)]
    action: Vec<types::CodeableConcept>,
    #[serde(default)]
    organization: Vec<types::Reference<crate::r3::resources::Organization>>,
    #[serde(flatten)]
    source: crate::r3::choice::Slot<ConsentSource>,
    #[serde(default)]
    policy: Vec<ConsentPolicy>,
    policy_rule: Option<types::Uri>,
    #[serde(rename = "_policyRule")]
    policy_rule_ext: Option<types::Element>,
    #[serde(default)]
    security_label: Vec<types::Coding>,
    #[serde(default)]
    purpose: Vec<types::Coding>,
    data_period: Option<types::Period>,
    #[serde(default)]
    data: Vec<ConsentData>,
    #[serde(default)]
    except: Vec<ConsentExcept>,
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
            category: v.category,
            patient: v.patient,
            period: v.period,
            date_time: v.date_time,
            date_time_ext: v.date_time_ext,
            consenting_party: v.consenting_party,
            actor: v.actor,
            action: v.action,
            organization: v.organization,
            source: v.source.0,
            policy: v.policy,
            policy_rule: v.policy_rule,
            policy_rule_ext: v.policy_rule_ext,
            security_label: v.security_label,
            purpose: v.purpose,
            data_period: v.data_period,
            data: v.data,
            except: v.except,
        }
    }
}

/// Who or what is controlled by this consent. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentActor;
/// use fhir::r3::types;
///
/// let value = ConsentActor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConsentActor {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How the actor is involved
    pub role: types::CodeableConcept,

    /// Resource for the actor (or group, by role)
    pub reference: types::Reference,
}

/// The resources controlled by this consent, if specific resources are
/// referenced.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentData;
/// use fhir::r3::types;
///
/// let value = ConsentData {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConsentData {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// instance | related | dependents | authoredby
    pub meaning: crate::coded::Coded<crate::r3::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// An exception to the base policy of this consent. An exception can be an
/// addition or removal of access permissions.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentExcept;
/// use fhir::r3::types;
///
/// let value = ConsentExcept {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentExcept = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConsentExcept {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// deny | permit
    pub r#type: crate::coded::Coded<crate::r3::codes::ConsentExceptType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Timeframe for this exception
    pub period: Option<types::Period>,

    /// Who|what controlled by this exception (or group, by role)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ConsentExceptActor>,

    /// Actions controlled by this exception
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Security Labels that define affected resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Context of activities covered by this exception
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::Coding>,

    /// e.g. Resource Type, Profile, or CDA etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::Coding>,

    /// e.g. LOINC or SNOMED CT code, etc in the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// Timeframe for data controlled by this exception
    pub data_period: Option<types::Period>,

    /// Data controlled by this exception
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<ConsentExceptData>,
}

/// Who or what is controlled by this Exception. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentExceptActor;
/// use fhir::r3::types;
///
/// let value = ConsentExceptActor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentExceptActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConsentExceptActor {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How the actor is involved
    pub role: types::CodeableConcept,

    /// Resource for the actor (or group, by role)
    pub reference: types::Reference,
}

/// The resources controlled by this exception, if specific resources are
/// referenced.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentExceptData;
/// use fhir::r3::types;
///
/// let value = ConsentExceptData {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConsentExceptData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConsentExceptData {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// instance | related | dependents | authoredby
    pub meaning: crate::coded::Coded<crate::r3::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// The references to the policies that are included in this consent scope.
/// Policies may be organizational, but are often defined jurisdictionally, or
/// in law.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::consent::ConsentPolicy;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ConsentPolicy {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

/// The `Consent.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ConsentSource {
    /// `sourceAttachment` variant.
    #[fhir("sourceAttachment")]
    Attachment(Box<types::Attachment>),
    /// `sourceIdentifier` variant.
    #[fhir("sourceIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
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
