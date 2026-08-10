//! Contract
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Contract
//!
//! Version: 4.3.0
//!
//! Legal Agreement
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Legally enforceable, formally recorded unilateral or bilateral directive
/// i.e., a policy or agreement.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::Contract;
/// use fhir::r4b::types;
///
/// let value = Contract {
///     instantiates_uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instantiatesUri` is the name this serializes to on the wire.
/// assert_eq!(json["instantiatesUri"], ::serde_json::json!("http://example.org"));
///
/// let back: Contract = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractDe")]
#[fhir_version("r4b")]
pub struct Contract {
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

    /// Contract number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Basal definition
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business edition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// amended | appended | cancelled | disputed | entered-in-error |
    /// executable | executed | negotiable | offered | policy | rejected |
    /// renewed | revoked | resolved | terminated
    pub status: Option<crate::coded::Coded<crate::r4b::codes::ContractStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Negotiation status
    pub legal_state: Option<types::CodeableConcept>,

    /// Source Contract Definition
    pub instantiates_canonical: Option<types::Reference<crate::r4b::resources::Contract>>,

    /// External Contract Definition
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Content derived from the basal information
    pub content_derivative: Option<types::CodeableConcept>,

    /// When this Contract was issued
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Effective time
    pub applies: Option<types::Period>,

    /// Contract cessation cause
    pub expiration_type: Option<types::CodeableConcept>,

    /// Contract Target Entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Authority under which this Contract has standing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authority: Vec<types::Reference<crate::r4b::resources::Organization>>,

    /// A sphere of control governed by an authoritative jurisdiction,
    /// organization, or person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<types::Reference<crate::r4b::resources::Location>>,

    /// Specific Location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<types::Reference<crate::r4b::resources::Location>>,

    /// Computer friendly designation
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human Friendly name
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate Friendly name
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// Acronym or short name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// Source of Contract
    pub author: Option<types::Reference>,

    /// Range of Legal Concerns
    pub scope: Option<types::CodeableConcept>,

    /// Focus of contract interest
    /// The `Contract.topic[x]` choice element (0..1); see [`ContractTopic`].
    #[serde(flatten)]
    pub topic: Option<ContractTopic>,

    /// Legal instrument category
    pub r#type: Option<types::CodeableConcept>,

    /// Subtype within the context of type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_type: Vec<types::CodeableConcept>,

    /// Contract precursor content
    pub content_definition: Option<ContractContentDefinition>,

    /// Contract Term List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<ContractTerm>,

    /// Extra Information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Key event in Contract History
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,

    /// Contract Signatory
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signer: Vec<ContractSigner>,

    /// Contract Friendly Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub friendly: Vec<ContractFriendly>,

    /// Contract Legal Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal: Vec<ContractLegal>,

    /// Computable Contract Language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<ContractRule>,

    /// Binding Contract
    /// The `Contract.legallyBinding[x]` choice element (0..1); see [`ContractLegallyBinding`].
    #[serde(flatten)]
    pub legally_binding: Option<ContractLegallyBinding>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractDe {
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
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    status: Option<crate::coded::Coded<crate::r4b::codes::ContractStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    legal_state: Option<types::CodeableConcept>,
    instantiates_canonical: Option<types::Reference<crate::r4b::resources::Contract>>,
    instantiates_uri: Option<types::Uri>,
    #[serde(rename = "_instantiatesUri")]
    instantiates_uri_ext: Option<types::Element>,
    content_derivative: Option<types::CodeableConcept>,
    issued: Option<types::DateTime>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    applies: Option<types::Period>,
    expiration_type: Option<types::CodeableConcept>,
    #[serde(default)]
    subject: Vec<types::Reference>,
    #[serde(default)]
    authority: Vec<types::Reference<crate::r4b::resources::Organization>>,
    #[serde(default)]
    domain: Vec<types::Reference<crate::r4b::resources::Location>>,
    #[serde(default)]
    site: Vec<types::Reference<crate::r4b::resources::Location>>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    subtitle: Option<types::String>,
    #[serde(rename = "_subtitle")]
    subtitle_ext: Option<types::Element>,
    #[serde(default)]
    alias: Vec<types::String>,
    #[serde(rename = "_alias")]
    #[serde(default)]
    alias_ext: Vec<Option<types::Element>>,
    author: Option<types::Reference>,
    scope: Option<types::CodeableConcept>,
    #[serde(flatten)]
    topic: crate::r4b::choice::Slot<ContractTopic>,
    r#type: Option<types::CodeableConcept>,
    #[serde(default)]
    sub_type: Vec<types::CodeableConcept>,
    content_definition: Option<ContractContentDefinition>,
    #[serde(default)]
    term: Vec<ContractTerm>,
    #[serde(default)]
    supporting_info: Vec<types::Reference>,
    #[serde(default)]
    relevant_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,
    #[serde(default)]
    signer: Vec<ContractSigner>,
    #[serde(default)]
    friendly: Vec<ContractFriendly>,
    #[serde(default)]
    legal: Vec<ContractLegal>,
    #[serde(default)]
    rule: Vec<ContractRule>,
    #[serde(flatten)]
    legally_binding: crate::r4b::choice::Slot<ContractLegallyBinding>,
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
            url: v.url,
            url_ext: v.url_ext,
            version: v.version,
            version_ext: v.version_ext,
            status: v.status,
            status_ext: v.status_ext,
            legal_state: v.legal_state,
            instantiates_canonical: v.instantiates_canonical,
            instantiates_uri: v.instantiates_uri,
            instantiates_uri_ext: v.instantiates_uri_ext,
            content_derivative: v.content_derivative,
            issued: v.issued,
            issued_ext: v.issued_ext,
            applies: v.applies,
            expiration_type: v.expiration_type,
            subject: v.subject,
            authority: v.authority,
            domain: v.domain,
            site: v.site,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            subtitle: v.subtitle,
            subtitle_ext: v.subtitle_ext,
            alias: v.alias,
            alias_ext: v.alias_ext,
            author: v.author,
            scope: v.scope,
            topic: v.topic.0,
            r#type: v.r#type,
            sub_type: v.sub_type,
            content_definition: v.content_definition,
            term: v.term,
            supporting_info: v.supporting_info,
            relevant_history: v.relevant_history,
            signer: v.signer,
            friendly: v.friendly,
            legal: v.legal,
            rule: v.rule,
            legally_binding: v.legally_binding.0,
        }
    }
}

/// Precusory content developed with a focus and intent of supporting the
/// formation a Contract instance, which may be associated with and
/// transformable into a Contract.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractContentDefinition;
/// use fhir::r4b::types;
///
/// let value = ContractContentDefinition {
///     publication_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `publicationDate` is the name this serializes to on the wire.
/// assert_eq!(json["publicationDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ContractContentDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractContentDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Content structure and use
    pub r#type: types::CodeableConcept,

    /// Detailed Content Type Definition
    pub sub_type: Option<types::CodeableConcept>,

    /// Publisher Entity
    pub publisher: Option<types::Reference>,

    /// When published
    pub publication_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`publication_date`](Self::publication_date) (FHIR `_publicationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationDate")]
    pub publication_date_ext: Option<types::Element>,

    /// amended | appended | cancelled | disputed | entered-in-error |
    /// executable | executed | negotiable | offered | policy | rejected |
    /// renewed | revoked | resolved | terminated
    pub publication_status: crate::coded::Coded<crate::r4b::codes::ContractPublicationstatus>,
    /// Primitive extension sibling for [`publication_status`](Self::publication_status) (FHIR `_publicationStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationStatus")]
    pub publication_status_ext: Option<types::Element>,

    /// Publication Ownership
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
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
/// use fhir::r4b::resources::contract::ContractFriendly;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct ContractFriendly {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    content: crate::r4b::choice::Slot<ContractFriendlyContent>,
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
/// use fhir::r4b::resources::contract::ContractLegal;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct ContractLegal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    content: crate::r4b::choice::Slot<ContractLegalContent>,
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
/// use fhir::r4b::resources::contract::ContractRule;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct ContractRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    content: crate::r4b::choice::Slot<ContractRuleContent>,
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
/// use fhir::r4b::resources::contract::ContractSigner;
///
/// let value = ContractSigner::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ContractSigner = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractSigner {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
/// use fhir::r4b::resources::contract::ContractTerm;
/// use fhir::r4b::types;
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
#[serde(from = "ContractTermDe")]
#[fhir_version("r4b")]
pub struct ContractTerm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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

    /// Term Concern
    /// The `Contract.term.topic[x]` choice element (0..1); see [`ContractTermTopic`].
    #[serde(flatten)]
    pub topic: Option<ContractTermTopic>,

    /// Contract Term Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Contract Term Type specific classification
    pub sub_type: Option<types::CodeableConcept>,

    /// Term Statement
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Protection for the Term
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<ContractTermSecurityLabel>,

    /// Context of the Contract term
    pub offer: ContractTermOffer,

    /// Contract Term Asset List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset: Vec<ContractTermAsset>,

    /// Entity being ascribed responsibility
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<ContractTermAction>,

    /// Nested Contract Term Group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<ContractTerm>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTermDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    identifier: Option<types::Identifier>,
    issued: Option<types::DateTime>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    applies: Option<types::Period>,
    #[serde(flatten)]
    topic: crate::r4b::choice::Slot<ContractTermTopic>,
    r#type: Option<types::CodeableConcept>,
    sub_type: Option<types::CodeableConcept>,
    text: Option<types::String>,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
    #[serde(default)]
    security_label: Vec<ContractTermSecurityLabel>,
    offer: ContractTermOffer,
    #[serde(default)]
    asset: Vec<ContractTermAsset>,
    #[serde(default)]
    action: Vec<ContractTermAction>,
    #[serde(default)]
    group: Vec<ContractTerm>,
}

impl ::core::convert::From<ContractTermDe> for ContractTerm {
    fn from(v: ContractTermDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            issued: v.issued,
            issued_ext: v.issued_ext,
            applies: v.applies,
            topic: v.topic.0,
            r#type: v.r#type,
            sub_type: v.sub_type,
            text: v.text,
            text_ext: v.text_ext,
            security_label: v.security_label,
            offer: v.offer,
            asset: v.asset,
            action: v.action,
            group: v.group,
        }
    }
}

/// An actor taking a role in an activity for which it can be assigned some
/// degree of responsibility for the activity taking place.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermAction;
/// use fhir::r4b::types;
///
/// let value = ContractTermAction {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: ContractTermAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractTermActionDe")]
#[fhir_version("r4b")]
pub struct ContractTermAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// True if the term prohibits the action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// Type or form of the action
    pub r#type: types::CodeableConcept,

    /// Entity of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<ContractTermActionSubject>,

    /// Purpose for the Contract Term Action
    pub intent: types::CodeableConcept,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id: Vec<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id_ext: Vec<Option<types::Element>>,

    /// State of the action
    pub status: types::CodeableConcept,

    /// Episode associated with action
    pub context: Option<types::Reference>,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_link_id: Vec<types::String>,
    /// Primitive extension sibling for [`context_link_id`](Self::context_link_id) (FHIR `_contextLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_link_id_ext: Vec<Option<types::Element>>,

    /// When action happens
    /// The `Contract.term.action.occurrence[x]` choice element (0..1); see [`ContractTermActionOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ContractTermActionOccurrence>,

    /// Who asked for action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requester: Vec<types::Reference>,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requester_link_id: Vec<types::String>,
    /// Primitive extension sibling for [`requester_link_id`](Self::requester_link_id) (FHIR `_requesterLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requesterLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requester_link_id_ext: Vec<Option<types::Element>>,

    /// Kind of service performer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_type: Vec<types::CodeableConcept>,

    /// Competency of the performer
    pub performer_role: Option<types::CodeableConcept>,

    /// Actor that wil execute (or not) the action
    pub performer: Option<types::Reference>,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_link_id: Vec<types::String>,
    /// Primitive extension sibling for [`performer_link_id`](Self::performer_link_id) (FHIR `_performerLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_performerLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_link_id_ext: Vec<Option<types::Element>>,

    /// Why is action (not) needed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why is action (not) needed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Why action is to be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reason")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_ext: Vec<Option<types::Element>>,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_link_id: Vec<types::String>,
    /// Primitive extension sibling for [`reason_link_id`](Self::reason_link_id) (FHIR `_reasonLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reasonLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_link_id_ext: Vec<Option<types::Element>>,

    /// Comments about the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Action restriction numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number: Vec<types::UnsignedInt>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number_ext: Vec<Option<types::Element>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTermActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    r#type: types::CodeableConcept,
    #[serde(default)]
    subject: Vec<ContractTermActionSubject>,
    intent: types::CodeableConcept,
    #[serde(default)]
    link_id: Vec<types::String>,
    #[serde(rename = "_linkId")]
    #[serde(default)]
    link_id_ext: Vec<Option<types::Element>>,
    status: types::CodeableConcept,
    context: Option<types::Reference>,
    #[serde(default)]
    context_link_id: Vec<types::String>,
    #[serde(rename = "_contextLinkId")]
    #[serde(default)]
    context_link_id_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    occurrence: crate::r4b::choice::Slot<ContractTermActionOccurrence>,
    #[serde(default)]
    requester: Vec<types::Reference>,
    #[serde(default)]
    requester_link_id: Vec<types::String>,
    #[serde(rename = "_requesterLinkId")]
    #[serde(default)]
    requester_link_id_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    performer_type: Vec<types::CodeableConcept>,
    performer_role: Option<types::CodeableConcept>,
    performer: Option<types::Reference>,
    #[serde(default)]
    performer_link_id: Vec<types::String>,
    #[serde(rename = "_performerLinkId")]
    #[serde(default)]
    performer_link_id_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    #[serde(default)]
    reason: Vec<types::String>,
    #[serde(rename = "_reason")]
    #[serde(default)]
    reason_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    reason_link_id: Vec<types::String>,
    #[serde(rename = "_reasonLinkId")]
    #[serde(default)]
    reason_link_id_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    security_label_number: Vec<types::UnsignedInt>,
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default)]
    security_label_number_ext: Vec<Option<types::Element>>,
}

impl ::core::convert::From<ContractTermActionDe> for ContractTermAction {
    fn from(v: ContractTermActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            r#type: v.r#type,
            subject: v.subject,
            intent: v.intent,
            link_id: v.link_id,
            link_id_ext: v.link_id_ext,
            status: v.status,
            context: v.context,
            context_link_id: v.context_link_id,
            context_link_id_ext: v.context_link_id_ext,
            occurrence: v.occurrence.0,
            requester: v.requester,
            requester_link_id: v.requester_link_id,
            requester_link_id_ext: v.requester_link_id_ext,
            performer_type: v.performer_type,
            performer_role: v.performer_role,
            performer: v.performer,
            performer_link_id: v.performer_link_id,
            performer_link_id_ext: v.performer_link_id_ext,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            reason: v.reason,
            reason_ext: v.reason_ext,
            reason_link_id: v.reason_link_id,
            reason_link_id_ext: v.reason_link_id_ext,
            note: v.note,
            security_label_number: v.security_label_number,
            security_label_number_ext: v.security_label_number_ext,
        }
    }
}

/// Entity of the action.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::contract::ContractTermActionSubject;
///
/// let value = ContractTermActionSubject::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ContractTermActionSubject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermActionSubject {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Entity of the action
    pub reference: ::vec1::Vec1<types::Reference>,

    /// Role type of the agent
    pub role: Option<types::CodeableConcept>,
}

/// Contract Term Asset List.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermAsset;
/// use fhir::r4b::types;
///
/// let value = ContractTermAsset {
///     condition: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `condition` is the name this serializes to on the wire.
/// assert_eq!(json["condition"], ::serde_json::json!("abc"));
///
/// let back: ContractTermAsset = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermAsset {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Range of asset
    pub scope: Option<types::CodeableConcept>,

    /// Asset category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Associated entities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_reference: Vec<types::Reference>,

    /// Asset sub-category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtype: Vec<types::CodeableConcept>,

    /// Kinship of the asset
    pub relationship: Option<types::Coding>,

    /// Circumstance of the asset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<ContractTermAssetContext>,

    /// Quality desctiption of asset
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// Asset availability types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period_type: Vec<types::CodeableConcept>,

    /// Time period of the asset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<types::Period>,

    /// Time period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_period: Vec<types::Period>,

    /// Asset clause or question text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Pointer to asset text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id: Vec<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id_ext: Vec<Option<types::Element>>,

    /// Response to assets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer: Vec<ContractTermOfferAnswer>,

    /// Asset restriction numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number: Vec<types::UnsignedInt>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number_ext: Vec<Option<types::Element>>,

    /// Contract Valued Item List
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valued_item: Vec<ContractTermAssetValuedItem>,
}

/// Circumstance of the asset.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermAssetContext;
/// use fhir::r4b::types;
///
/// let value = ContractTermAssetContext {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: ContractTermAssetContext = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermAssetContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Creator,custodian or owner
    pub reference: Option<types::Reference>,

    /// Codeable asset context
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Context description
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Contract Valued Item List.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermAssetValuedItem;
/// use fhir::r4b::types;
///
/// let value = ContractTermAssetValuedItem {
///     effective_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `effectiveTime` is the name this serializes to on the wire.
/// assert_eq!(json["effectiveTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ContractTermAssetValuedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractTermAssetValuedItemDe")]
#[fhir_version("r4b")]
pub struct ContractTermAssetValuedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contract Valued Item Type
    /// The `Contract.term.asset.valuedItem.entity[x]` choice element (0..1); see [`ContractTermAssetValuedItemEntity`].
    #[serde(flatten)]
    pub entity: Option<ContractTermAssetValuedItemEntity>,

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

    /// Terms of valuation
    pub payment: Option<types::String>,
    /// Primitive extension sibling for [`payment`](Self::payment) (FHIR `_payment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_payment")]
    pub payment_ext: Option<types::Element>,

    /// When payment is due
    pub payment_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Who will make payment
    pub responsible: Option<types::Reference>,

    /// Who will receive payment
    pub recipient: Option<types::Reference>,

    /// Pointer to specific item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id: Vec<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id_ext: Vec<Option<types::Element>>,

    /// Security Labels that define affected terms
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number: Vec<types::UnsignedInt>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number_ext: Vec<Option<types::Element>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTermAssetValuedItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    entity: crate::r4b::choice::Slot<ContractTermAssetValuedItemEntity>,
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
    payment: Option<types::String>,
    #[serde(rename = "_payment")]
    payment_ext: Option<types::Element>,
    payment_date: Option<types::DateTime>,
    #[serde(rename = "_paymentDate")]
    payment_date_ext: Option<types::Element>,
    responsible: Option<types::Reference>,
    recipient: Option<types::Reference>,
    #[serde(default)]
    link_id: Vec<types::String>,
    #[serde(rename = "_linkId")]
    #[serde(default)]
    link_id_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    security_label_number: Vec<types::UnsignedInt>,
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default)]
    security_label_number_ext: Vec<Option<types::Element>>,
}

impl ::core::convert::From<ContractTermAssetValuedItemDe> for ContractTermAssetValuedItem {
    fn from(v: ContractTermAssetValuedItemDe) -> Self {
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
            payment: v.payment,
            payment_ext: v.payment_ext,
            payment_date: v.payment_date,
            payment_date_ext: v.payment_date_ext,
            responsible: v.responsible,
            recipient: v.recipient,
            link_id: v.link_id,
            link_id_ext: v.link_id_ext,
            security_label_number: v.security_label_number,
            security_label_number_ext: v.security_label_number_ext,
        }
    }
}

/// The matter of concern in the context of this provision of the agrement.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermOffer;
/// use fhir::r4b::types;
///
/// let value = ContractTermOffer {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: ContractTermOffer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermOffer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Offer business ID
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Offer Recipient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party: Vec<ContractTermOfferParty>,

    /// Negotiable offer asset
    pub topic: Option<types::Reference>,

    /// Contract Offer Type or Form
    pub r#type: Option<types::CodeableConcept>,

    /// Accepting party choice
    pub decision: Option<types::CodeableConcept>,

    /// How decision is conveyed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decision_mode: Vec<types::CodeableConcept>,

    /// Response to offer text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer: Vec<ContractTermOfferAnswer>,

    /// Human readable offer text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Pointer to text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id: Vec<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_id_ext: Vec<Option<types::Element>>,

    /// Offer restriction numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number: Vec<types::UnsignedInt>,
    /// Primitive extension sibling for [`security_label_number`](Self::security_label_number) (FHIR `_securityLabelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_securityLabelNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label_number_ext: Vec<Option<types::Element>>,
}

/// Response to offer text.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermOfferAnswer;
/// use fhir::r4b::types;
///
/// let value = ContractTermOfferAnswer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractTermOfferAnswer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ContractTermOfferAnswerDe")]
#[fhir_version("r4b")]
pub struct ContractTermOfferAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The actual answer response
    /// The `Contract.term.offer.answer.value[x]` choice element (1..1); see [`ContractTermOfferAnswerValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<ContractTermOfferAnswerValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractTermOfferAnswerDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r4b::choice::Slot<ContractTermOfferAnswerValue>,
}

impl ::core::convert::From<ContractTermOfferAnswerDe> for ContractTermOfferAnswer {
    fn from(v: ContractTermOfferAnswerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
        }
    }
}

/// Offer Recipient.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::contract::ContractTermOfferParty;
///
/// let value = ContractTermOfferParty::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ContractTermOfferParty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermOfferParty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Referenced entity
    pub reference: ::vec1::Vec1<types::Reference>,

    /// Participant engagement type
    pub role: types::CodeableConcept,
}

/// Security labels that protect the handling of information about the term and
/// its elements, which may be specifically identified..
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::contract::ContractTermSecurityLabel;
/// use fhir::r4b::types;
///
/// let value = ContractTermSecurityLabel {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ContractTermSecurityLabel = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ContractTermSecurityLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Link to Security Labels
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub number: Vec<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub number_ext: Vec<Option<types::Element>>,

    /// Confidentiality Protection
    pub classification: types::Coding,

    /// Applicable Policy
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::Coding>,

    /// Handling Instructions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control: Vec<types::Coding>,
}

/// The `Contract.topic[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTopic {
    /// `topicCodeableConcept` variant.
    #[fhir("topicCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `topicReference` variant.
    #[fhir("topicReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.legallyBinding[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractLegallyBinding {
    /// `legallyBindingAttachment` variant.
    #[fhir("legallyBindingAttachment")]
    Attachment(Box<types::Attachment>),
    /// `legallyBindingReference` variant.
    #[fhir("legallyBindingReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.friendly.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
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
#[fhir_version("r4b")]
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
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractRuleContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.topic[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermTopic {
    /// `topicCodeableConcept` variant.
    #[fhir("topicCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `topicReference` variant.
    #[fhir("topicReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.action.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermActionOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `Contract.term.asset.valuedItem.entity[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermAssetValuedItemEntity {
    /// `entityCodeableConcept` variant.
    #[fhir("entityCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `entityReference` variant.
    #[fhir("entityReference")]
    Reference(Box<types::Reference>),
}

/// The `Contract.term.offer.answer.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ContractTermOfferAnswerValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r4b::choice::Primitive<types::Decimal>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r4b::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r4b::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4b::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r4b::choice::Primitive<types::Uri>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
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
