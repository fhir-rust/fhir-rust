//! Contract
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Contract
//!
//!
//!
//! Legal Agreement
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Contract Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::Contract;
/// use fhir::r3::types;
///
/// let value = Contract {
///     issued: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `issued` is the name this serializes to on the wire.
/// assert_eq!(json["issued"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Contract = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractDe")]
#[fhir_version("r3")]
pub struct Contract {
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

    /// Contract number
    pub identifier: Option<types::Identifier>,

    /// amended | appended | cancelled | disputed | entered-in-error |
    /// executable | executed | negotiable | offered | policy | rejected |
    /// renewed | revoked | resolved | terminated
    pub status: Option<crate::coded::Coded<crate::r3::codes::ContractStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// When this Contract was issued
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Effective time
    pub applies: Option<types::Period>,

    /// Contract Target Entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Context of the Contract
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::Reference>,

    /// Authority under which this Contract has standing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authority: Vec<types::Reference<crate::r3::resources::Organization>>,

    /// Domain in which this Contract applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<types::Reference<crate::r3::resources::Location>>,

    /// Type or form
    pub r#type: Option<types::CodeableConcept>,

    /// Subtype within the context of type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_type: Vec<types::CodeableConcept>,

    /// Action stipulated by this Contract
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Rationale for the stiplulated action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action_reason: Vec<types::CodeableConcept>,

    /// Decision by Grantor
    pub decision_type: Option<types::CodeableConcept>,

    /// Content derived from the basal information
    pub content_derivative: Option<types::CodeableConcept>,

    /// Security Labels that define affected resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Entity being ascribed responsibility
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ContractAgent>,

    /// Contract Signatory
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signer: Vec<ContractSigner>,

    /// Contract Valued Item List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valued_item: Vec<ContractValuedItem>,

    /// Contract Term List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<ContractTerm>,

    /// Binding Contract
    /// The `Contract.binding[x]` choice element (0..1); see [`ContractBinding`].
    #[serde(flatten)]
    pub binding: Option<ContractBinding>,

    /// Contract Friendly Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub friendly: Vec<ContractFriendly>,

    /// Contract Legal Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal: Vec<ContractLegal>,

    /// Computable Contract Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<ContractRule>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractDe {
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
    status: Option<crate::coded::Coded<crate::r3::codes::ContractStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    issued: Option<types::DateTime>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    applies: Option<types::Period>,
    #[serde(default)]
    subject: Vec<types::Reference>,
    #[serde(default)]
    topic: Vec<types::Reference>,
    #[serde(default)]
    authority: Vec<types::Reference<crate::r3::resources::Organization>>,
    #[serde(default)]
    domain: Vec<types::Reference<crate::r3::resources::Location>>,
    r#type: Option<types::CodeableConcept>,
    #[serde(default)]
    sub_type: Vec<types::CodeableConcept>,
    #[serde(default)]
    action: Vec<types::CodeableConcept>,
    #[serde(default)]
    action_reason: Vec<types::CodeableConcept>,
    decision_type: Option<types::CodeableConcept>,
    content_derivative: Option<types::CodeableConcept>,
    #[serde(default)]
    security_label: Vec<types::Coding>,
    #[serde(default)]
    agent: Vec<ContractAgent>,
    #[serde(default)]
    signer: Vec<ContractSigner>,
    #[serde(default)]
    valued_item: Vec<ContractValuedItem>,
    #[serde(default)]
    term: Vec<ContractTerm>,
    #[serde(flatten)]
    binding: crate::r3::choice::Slot<ContractBinding>,
    #[serde(default)]
    friendly: Vec<ContractFriendly>,
    #[serde(default)]
    legal: Vec<ContractLegal>,
    #[serde(default)]
    rule: Vec<ContractRule>,
}

impl ::core::convert::From<ContractDe> for Contract {
    fn from(v: ContractDe) -> Self {
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
            issued: v.issued,
            issued_ext: v.issued_ext,
            applies: v.applies,
            subject: v.subject,
            topic: v.topic,
            authority: v.authority,
            domain: v.domain,
            r#type: v.r#type,
            sub_type: v.sub_type,
            action: v.action,
            action_reason: v.action_reason,
            decision_type: v.decision_type,
            content_derivative: v.content_derivative,
            security_label: v.security_label,
            agent: v.agent,
            signer: v.signer,
            valued_item: v.valued_item,
            term: v.term,
            binding: v.binding.0,
            friendly: v.friendly,
            legal: v.legal,
            rule: v.rule,
        }
    }
}

/// An actor taking a role in an activity for which it can be assigned some
/// degree of responsibility for the activity taking place.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractAgent;
/// use fhir::r3::types;
///
/// let value = ContractAgent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ContractAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Agent Type
    pub actor: types::Reference,

    /// Role type of the agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,
}

/// The "patient friendly language" versionof the Contract in whole or in
/// parts. "Patient friendly language" means the representation of the Contract
/// and Contract Provisions in a manner that is readily accessible and
/// understandable by a layperson in accordance with best practices for
/// communication styles that ensure that those agreeing to or signing the
/// Contract understand the roles, actions, obligations, responsibilities, and
/// implication of the agreement.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractFriendly;
/// use fhir::r3::types;
///
/// let value = ContractFriendly {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractFriendly = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractFriendlyDe")]
#[fhir_version("r3")]
pub struct ContractFriendly {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Easily comprehended representation of this Contract
    /// The `Contract.friendly.content[x]` choice element (1..1); see [`ContractFriendlyContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<ContractFriendlyContent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractFriendlyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    content: crate::r3::choice::Slot<ContractFriendlyContent>,
}

impl ::core::convert::From<ContractFriendlyDe> for ContractFriendly {
    fn from(v: ContractFriendlyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            content: v.content.0,
        }
    }
}

/// List of Legal expressions or representations of this Contract.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractLegal;
/// use fhir::r3::types;
///
/// let value = ContractLegal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractLegal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractLegalDe")]
#[fhir_version("r3")]
pub struct ContractLegal {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Legal Text
    /// The `Contract.legal.content[x]` choice element (1..1); see [`ContractLegalContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<ContractLegalContent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractLegalDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    content: crate::r3::choice::Slot<ContractLegalContent>,
}

impl ::core::convert::From<ContractLegalDe> for ContractLegal {
    fn from(v: ContractLegalDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            content: v.content.0,
        }
    }
}

/// List of Computable Policy Rule Language Representations of this Contract.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractRule;
/// use fhir::r3::types;
///
/// let value = ContractRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractRuleDe")]
#[fhir_version("r3")]
pub struct ContractRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Computable Contract Rules
    /// The `Contract.rule.content[x]` choice element (1..1); see [`ContractRuleContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<ContractRuleContent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractRuleDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    content: crate::r3::choice::Slot<ContractRuleContent>,
}

impl ::core::convert::From<ContractRuleDe> for ContractRule {
    fn from(v: ContractRuleDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            content: v.content.0,
        }
    }
}

/// Parties with legal standing in the Contract, including the principal
/// parties, the grantor(s) and grantee(s), which are any person or
/// organization bound by the contract, and any ancillary parties, which
/// facilitate the execution of the contract such as a notary or witness.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::contract::ContractSigner;
///
/// let value = ContractSigner::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ContractSigner = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ContractSigner {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Signatory Role
    pub r#type: types::Coding,

    /// Contract Signatory Party
    pub party: types::Reference,

    /// Contract Documentation Signature
    pub signature: ::vec1::Vec1<types::Signature>,
}

/// One or more Contract Provisions, which may be related and conveyed as a
/// group, and may contain nested groups.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractTerm;
/// use fhir::r3::types;
///
/// let value = ContractTerm {
///     issued: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `issued` is the name this serializes to on the wire.
/// assert_eq!(json["issued"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ContractTerm = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ContractTerm {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Term Number
    pub identifier: Option<types::Identifier>,

    /// Contract Term Issue Date Time
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Contract Term Effective Time
    pub applies: Option<types::Period>,

    /// Contract Term Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Contract Term Type specific classification
    pub sub_type: Option<types::CodeableConcept>,

    /// Context of the Contract term
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::Reference>,

    /// Contract Term Activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// Purpose for the Contract Term Action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action_reason: Vec<types::CodeableConcept>,

    /// Security Labels that define affected terms
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Contract Term Agent List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ContractTermAgent>,

    /// Human readable Contract term text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Contract Term Valued Item List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valued_item: Vec<ContractTermValuedItem>,

    /// Nested Contract Term Group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<ContractTerm>,
}

/// An actor taking a role in an activity for which it can be assigned some
/// degree of responsibility for the activity taking place.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractTermAgent;
/// use fhir::r3::types;
///
/// let value = ContractTermAgent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractTermAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ContractTermAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Term Agent Subject
    pub actor: types::Reference,

    /// Type of the Contract Term Agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,
}

/// Contract Provision Valued Item List.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractTermValuedItem;
/// use fhir::r3::types;
///
/// let value = ContractTermValuedItem {
///     effective_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `effectiveTime` is the name this serializes to on the wire.
/// assert_eq!(json["effectiveTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ContractTermValuedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractTermValuedItemDe")]
#[fhir_version("r3")]
pub struct ContractTermValuedItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Term Valued Item Type
    /// The `Contract.term.valuedItem.entity[x]` choice element (0..1); see [`ContractTermValuedItemEntity`].
    #[serde(flatten)]
    pub entity: Option<ContractTermValuedItemEntity>,

    /// Contract Term Valued Item Number
    pub identifier: Option<types::Identifier>,

    /// Contract Term Valued Item Effective Tiem
    pub effective_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`effective_time`](Self::effective_time) (FHIR `_effectiveTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_effectiveTime")]
    pub effective_time_ext: Option<types::Element>,

    /// Contract Term Valued Item Count
    pub quantity: Option<types::Quantity>,

    /// Contract Term Valued Item fee, charge, or cost
    pub unit_price: Option<types::Money>,

    /// Contract Term Valued Item Price Scaling Factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Contract Term Valued Item Difficulty Scaling Factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Total Contract Term Valued Item Value
    pub net: Option<types::Money>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTermValuedItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    entity: crate::r3::choice::Slot<ContractTermValuedItemEntity>,
    identifier: Option<types::Identifier>,
    effective_time: Option<types::DateTime>,
    #[serde(rename = "_effectiveTime")]
    effective_time_ext: Option<types::Element>,
    quantity: Option<types::Quantity>,
    unit_price: Option<types::Money>,
    factor: Option<types::Decimal>,
    #[serde(rename = "_factor")]
    factor_ext: Option<types::Element>,
    points: Option<types::Decimal>,
    #[serde(rename = "_points")]
    points_ext: Option<types::Element>,
    net: Option<types::Money>,
}

impl ::core::convert::From<ContractTermValuedItemDe> for ContractTermValuedItem {
    fn from(v: ContractTermValuedItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            entity: v.entity.0,
            identifier: v.identifier,
            effective_time: v.effective_time,
            effective_time_ext: v.effective_time_ext,
            quantity: v.quantity,
            unit_price: v.unit_price,
            factor: v.factor,
            factor_ext: v.factor_ext,
            points: v.points,
            points_ext: v.points_ext,
            net: v.net,
        }
    }
}

/// Contract Valued Item List.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::contract::ContractValuedItem;
/// use fhir::r3::types;
///
/// let value = ContractValuedItem {
///     effective_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `effectiveTime` is the name this serializes to on the wire.
/// assert_eq!(json["effectiveTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ContractValuedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractValuedItemDe")]
#[fhir_version("r3")]
pub struct ContractValuedItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Valued Item Type
    /// The `Contract.valuedItem.entity[x]` choice element (0..1); see [`ContractValuedItemEntity`].
    #[serde(flatten)]
    pub entity: Option<ContractValuedItemEntity>,

    /// Contract Valued Item Number
    pub identifier: Option<types::Identifier>,

    /// Contract Valued Item Effective Tiem
    pub effective_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`effective_time`](Self::effective_time) (FHIR `_effectiveTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_effectiveTime")]
    pub effective_time_ext: Option<types::Element>,

    /// Count of Contract Valued Items
    pub quantity: Option<types::Quantity>,

    /// Contract Valued Item fee, charge, or cost
    pub unit_price: Option<types::Money>,

    /// Contract Valued Item Price Scaling Factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Contract Valued Item Difficulty Scaling Factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Total Contract Valued Item Value
    pub net: Option<types::Money>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractValuedItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    entity: crate::r3::choice::Slot<ContractValuedItemEntity>,
    identifier: Option<types::Identifier>,
    effective_time: Option<types::DateTime>,
    #[serde(rename = "_effectiveTime")]
    effective_time_ext: Option<types::Element>,
    quantity: Option<types::Quantity>,
    unit_price: Option<types::Money>,
    factor: Option<types::Decimal>,
    #[serde(rename = "_factor")]
    factor_ext: Option<types::Element>,
    points: Option<types::Decimal>,
    #[serde(rename = "_points")]
    points_ext: Option<types::Element>,
    net: Option<types::Money>,
}

impl ::core::convert::From<ContractValuedItemDe> for ContractValuedItem {
    fn from(v: ContractValuedItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            entity: v.entity.0,
            identifier: v.identifier,
            effective_time: v.effective_time,
            effective_time_ext: v.effective_time_ext,
            quantity: v.quantity,
            unit_price: v.unit_price,
            factor: v.factor,
            factor_ext: v.factor_ext,
            points: v.points,
            points_ext: v.points_ext,
            net: v.net,
        }
    }
}

/// The `Contract.binding[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractBinding {
    /// `bindingAttachment` variant.
    #[fhir("bindingAttachment")]
    Attachment(Box<types::Attachment>),
    /// `bindingReference` variant.
    #[fhir("bindingReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.friendly.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractFriendlyContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.legal.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractLegalContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.rule.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractRuleContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.valuedItem.entity[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermValuedItemEntity {
    /// `entityCodeableConcept` variant.
    #[fhir("entityCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `entityReference` variant.
    #[fhir("entityReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.valuedItem.entity[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ContractValuedItemEntity {
    /// `entityCodeableConcept` variant.
    #[fhir("entityCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `entityReference` variant.
    #[fhir("entityReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contract;

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
