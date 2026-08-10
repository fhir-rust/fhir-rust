//! CapabilityStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
//!
//! Version: 5.0.0
//!
//! CapabilityStatement Resource: A set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or
/// desired server implementation. It provides for a degree of automatic
/// negotiation of features and interoperability between FHIR systems.
///
/// Servers typically publish a `CapabilityStatement` at the `/metadata` endpoint so
/// that clients can discover which resource types, interactions, search parameters,
/// and operations are supported before attempting to exchange data. Implementation
/// guides also use `CapabilityStatement` to express conformance requirements that
/// implementations must satisfy, distinguishing between the `kind` values of
/// `instance` (an actual running system), `capability` (a reusable base
/// definition), and `requirements` (an abstract set of expectations). Because a
/// `CapabilityStatement` is itself a canonical resource, it carries the usual
/// metadata fields (`url`, `version`, `status`, `date`, `publisher`) shared with
/// other conformance resources such as `StructureDefinition` and `OperationDefinition`.
///
/// # Related resources
///
/// A `CapabilityStatement` describes a server or client's support for other
/// resource types, such as [`Patient`](crate::r5::resources::patient::Patient),
/// and it references coded terminology using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) and `Coding` for
/// values like security services and messaging protocols. See also
/// `StructureDefinition`, `OperationDefinition`, and `SearchParameter`, which
/// a `CapabilityStatement` may point to via canonical URLs.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::capability_statement::CapabilityStatement;
///
/// let value = CapabilityStatement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CapabilityStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CapabilityStatementDe")]
pub struct CapabilityStatement {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the CapabilityStatement (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the capability statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `CapabilityStatement.versionAlgorithm[x]` choice element (0..1); see [`CapabilityStatementVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CapabilityStatementVersionAlgorithm>,

    /// Name for this capability statement (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this capability statement (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication lifecycle status of this capability statement, one of draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the capability statement
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for capability statement (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this capability statement is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Whether this statement describes an actual running instance, a reusable base capability, or a set of requirements, one of instance | capability | requirements
    pub kind: crate::r5::coded::Coded<crate::r5::codes::CapabilityStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Canonical URL of another capability statement this implements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`).
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// Canonical URL of another capability statement this adds to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<types::Canonical>,
    /// Primitive extension sibling for [`imports`](Self::imports) (FHIR `_imports`).
    #[serde(rename = "_imports")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports_ext: Vec<Option<types::Element>>,

    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,

    /// The FHIR specification version that this capability statement describes support for
    pub fhir_version: crate::r5::coded::Coded<crate::r5::codes::FhirVersion>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`).
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// formats supported (xml | json | ttl | mime type)
    pub format: vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`format`](Self::format) (FHIR `_format`).
    #[serde(rename = "_format")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format_ext: Vec<Option<types::Element>>,

    /// Patch formats supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_format: Vec<types::Code>,
    /// Primitive extension sibling for [`patch_format`](Self::patch_format) (FHIR `_patchFormat`).
    #[serde(rename = "_patchFormat")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_format_ext: Vec<Option<types::Element>>,

    /// Languages supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accept_language: Vec<types::Code>,
    /// Primitive extension sibling for [`accept_language`](Self::accept_language) (FHIR `_acceptLanguage`).
    #[serde(rename = "_acceptLanguage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accept_language_ext: Vec<Option<types::Element>>,

    /// Implementation guides supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implementation_guide: Vec<types::Canonical>,
    /// Primitive extension sibling for [`implementation_guide`](Self::implementation_guide) (FHIR `_implementationGuide`).
    #[serde(rename = "_implementationGuide")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implementation_guide_ext: Vec<Option<types::Element>>,

    /// One or more RESTful endpoint descriptions, each covering the resources, interactions, and search parameters supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rest: Vec<CapabilityStatementRest>,

    /// Descriptions of messaging-based interfaces this system supports, if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messaging: Vec<CapabilityStatementMessaging>,

    /// Document definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document: Vec<CapabilityStatementDocument>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityStatementDe {
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
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<CapabilityStatementVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: types::DateTime,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    kind: crate::r5::coded::Coded<crate::r5::codes::CapabilityStatementKind>,
    #[serde(rename = "_kind")]
    kind_ext: Option<types::Element>,
    #[serde(default)]
    instantiates: Vec<types::Canonical>,
    #[serde(rename = "_instantiates")]
    #[serde(default)]
    instantiates_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    imports: Vec<types::Canonical>,
    #[serde(rename = "_imports")]
    #[serde(default)]
    imports_ext: Vec<Option<types::Element>>,
    software: Option<CapabilityStatementSoftware>,
    implementation: Option<CapabilityStatementImplementation>,
    fhir_version: crate::r5::coded::Coded<crate::r5::codes::FhirVersion>,
    #[serde(rename = "_fhirVersion")]
    fhir_version_ext: Option<types::Element>,
    format: vec1::Vec1<types::Code>,
    #[serde(rename = "_format")]
    #[serde(default)]
    format_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    patch_format: Vec<types::Code>,
    #[serde(rename = "_patchFormat")]
    #[serde(default)]
    patch_format_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    accept_language: Vec<types::Code>,
    #[serde(rename = "_acceptLanguage")]
    #[serde(default)]
    accept_language_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    implementation_guide: Vec<types::Canonical>,
    #[serde(rename = "_implementationGuide")]
    #[serde(default)]
    implementation_guide_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    rest: Vec<CapabilityStatementRest>,
    #[serde(default)]
    messaging: Vec<CapabilityStatementMessaging>,
    #[serde(default)]
    document: Vec<CapabilityStatementDocument>,
}

impl ::core::convert::From<CapabilityStatementDe> for CapabilityStatement {
    fn from(v: CapabilityStatementDe) -> Self {
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
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            kind: v.kind,
            kind_ext: v.kind_ext,
            instantiates: v.instantiates,
            instantiates_ext: v.instantiates_ext,
            imports: v.imports,
            imports_ext: v.imports_ext,
            software: v.software,
            implementation: v.implementation,
            fhir_version: v.fhir_version,
            fhir_version_ext: v.fhir_version_ext,
            format: v.format,
            format_ext: v.format_ext,
            patch_format: v.patch_format,
            patch_format_ext: v.patch_format_ext,
            accept_language: v.accept_language,
            accept_language_ext: v.accept_language_ext,
            implementation_guide: v.implementation_guide,
            implementation_guide_ext: v.implementation_guide_ext,
            rest: v.rest,
            messaging: v.messaging,
            document: v.document,
        }
    }
}

/// Software that is covered by this capability statement.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementSoftware;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementSoftware {
///     release_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `releaseDate` is the name this serializes to on the wire.
/// assert_eq!(json["releaseDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CapabilityStatementSoftware = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementSoftware {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name the software is known by
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Version covered by this statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Date this version was released
    pub release_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`release_date`](Self::release_date) (FHIR `_releaseDate`).
    #[serde(rename = "_releaseDate")]
    pub release_date_ext: Option<types::Element>,
}

/// If this describes a specific instance.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementImplementation;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementImplementation {
///     url: Some(types::Url("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: CapabilityStatementImplementation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes this specific instance
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Base URL for the installation
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Organization that manages the data
    pub custodian: Option<types::Reference<crate::r5::resources::Organization>>,
}

/// If the endpoint is a RESTful one.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRest;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRest {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: CapabilityStatementRest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// client | server
    pub mode: crate::r5::coded::Coded<crate::r5::codes::RestfulCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// General description of implementation
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,

    /// Resource served on the REST interface
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<CapabilityStatementRestResource>,

    /// What operations are supported?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction: Vec<CapabilityStatementRestInteraction>,

    /// Search parameters for searching all resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<CapabilityStatementRestResourceSearchParam>,

    /// Definition of a system level operation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<CapabilityStatementRestResourceOperation>,

    /// Compartments served/used by system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compartment: Vec<types::Canonical>,
    /// Primitive extension sibling for [`compartment`](Self::compartment) (FHIR `_compartment`).
    #[serde(rename = "_compartment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compartment_ext: Vec<Option<types::Element>>,
}

/// Information about security of implementation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestSecurity;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestSecurity {
///     cors: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cors` is the name this serializes to on the wire.
/// assert_eq!(json["cors"], ::serde_json::json!(true));
///
/// let back: CapabilityStatementRestSecurity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestSecurity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<types::Boolean>,
    /// Primitive extension sibling for [`cors`](Self::cors) (FHIR `_cors`).
    #[serde(rename = "_cors")]
    pub cors_ext: Option<types::Element>,

    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<types::CodeableConcept>,

    /// General description of how security works
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Resource served on the REST interface.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestResource;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestResource {
///     read_history: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `readHistory` is the name this serializes to on the wire.
/// assert_eq!(json["readHistory"], ::serde_json::json!(true));
///
/// let back: CapabilityStatementRestResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A resource type that is supported
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// System-wide profile
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Use-case specific profiles
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_profile: Vec<types::Canonical>,
    /// Primitive extension sibling for [`supported_profile`](Self::supported_profile) (FHIR `_supportedProfile`).
    #[serde(rename = "_supportedProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_profile_ext: Vec<Option<types::Element>>,

    /// Additional information about the use of the resource type
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What operations are supported?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction: Vec<CapabilityStatementRestResourceInteraction>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<crate::r5::coded::Coded<crate::r5::codes::VersioningPolicy>>,
    /// Primitive extension sibling for [`versioning`](Self::versioning) (FHIR `_versioning`).
    #[serde(rename = "_versioning")]
    pub versioning_ext: Option<types::Element>,

    /// Whether vRead can return past versions
    pub read_history: Option<types::Boolean>,
    /// Primitive extension sibling for [`read_history`](Self::read_history) (FHIR `_readHistory`).
    #[serde(rename = "_readHistory")]
    pub read_history_ext: Option<types::Element>,

    /// If update can commit to a new identity
    pub update_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`update_create`](Self::update_create) (FHIR `_updateCreate`).
    #[serde(rename = "_updateCreate")]
    pub update_create_ext: Option<types::Element>,

    /// If allows/uses conditional create
    pub conditional_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_create`](Self::conditional_create) (FHIR `_conditionalCreate`).
    #[serde(rename = "_conditionalCreate")]
    pub conditional_create_ext: Option<types::Element>,

    /// not-supported | modified-since | not-match | full-support
    pub conditional_read: Option<crate::r5::coded::Coded<crate::r5::codes::ConditionalReadStatus>>,
    /// Primitive extension sibling for [`conditional_read`](Self::conditional_read) (FHIR `_conditionalRead`).
    #[serde(rename = "_conditionalRead")]
    pub conditional_read_ext: Option<types::Element>,

    /// If allows/uses conditional update
    pub conditional_update: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_update`](Self::conditional_update) (FHIR `_conditionalUpdate`).
    #[serde(rename = "_conditionalUpdate")]
    pub conditional_update_ext: Option<types::Element>,

    /// If allows/uses conditional patch
    pub conditional_patch: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_patch`](Self::conditional_patch) (FHIR `_conditionalPatch`).
    #[serde(rename = "_conditionalPatch")]
    pub conditional_patch_ext: Option<types::Element>,

    /// not-supported | single | multiple - how conditional delete is supported
    pub conditional_delete:
        Option<crate::r5::coded::Coded<crate::r5::codes::ConditionalDeleteStatus>>,
    /// Primitive extension sibling for [`conditional_delete`](Self::conditional_delete) (FHIR `_conditionalDelete`).
    #[serde(rename = "_conditionalDelete")]
    pub conditional_delete_ext: Option<types::Element>,

    /// literal | logical | resolves | enforced | local
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_policy: Vec<crate::r5::coded::Coded<crate::r5::codes::ReferenceHandlingPolicy>>,
    /// Primitive extension sibling for [`reference_policy`](Self::reference_policy) (FHIR `_referencePolicy`).
    #[serde(rename = "_referencePolicy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_policy_ext: Vec<Option<types::Element>>,

    /// _include values supported by the server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_include: Vec<types::String>,
    /// Primitive extension sibling for [`search_include`](Self::search_include) (FHIR `_searchInclude`).
    #[serde(rename = "_searchInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_include_ext: Vec<Option<types::Element>>,

    /// _revinclude values supported by the server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_rev_include: Vec<types::String>,
    /// Primitive extension sibling for [`search_rev_include`](Self::search_rev_include) (FHIR `_searchRevInclude`).
    #[serde(rename = "_searchRevInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_rev_include_ext: Vec<Option<types::Element>>,

    /// Search parameters supported by implementation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<CapabilityStatementRestResourceSearchParam>,

    /// Definition of a resource operation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<CapabilityStatementRestResourceOperation>,
}

/// What operations are supported on a resource type?
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestResourceInteraction;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestResourceInteraction {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: CapabilityStatementRestResourceInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Search parameters supported by implementation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestResourceSearchParam;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestResourceSearchParam {
///     definition: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: CapabilityStatementRestResourceSearchParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceSearchParam {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name for parameter in search url
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Source of definition for parameter
    pub definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::SearchParamType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Server-specific usage
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Definition of a resource operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestResourceOperation;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestResourceOperation {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: CapabilityStatementRestResourceOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestResourceOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name by which the operation/query is invoked
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The defined operation/query
    pub definition: types::Canonical,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Specific details about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// What operations are supported at the system (all-resources) level?
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementRestInteraction;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementRestInteraction {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: CapabilityStatementRestInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementRestInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// transaction | batch | search-system | history-system
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// If messaging is supported.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementMessaging;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementMessaging {
///     reliable_cache: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reliableCache` is the name this serializes to on the wire.
/// assert_eq!(json["reliableCache"], ::serde_json::json!(0));
///
/// let back: CapabilityStatementMessaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Where messages should be sent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<CapabilityStatementMessagingEndpoint>,

    /// Reliable Message Cache Length (min)
    pub reliable_cache: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`reliable_cache`](Self::reliable_cache) (FHIR `_reliableCache`).
    #[serde(rename = "_reliableCache")]
    pub reliable_cache_ext: Option<types::Element>,

    /// Messaging interface behavior details
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Messages supported by this system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,
}

/// Where messages should be sent.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementMessagingEndpoint;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementMessagingEndpoint {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementMessagingEndpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingEndpoint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// http | ftp | mllp +
    pub protocol: types::Coding,

    /// Network address or identifier of the end-point
    pub address: types::Url,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`).
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,
}

/// Messages supported by this system.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementMessagingSupportedMessage;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementMessagingSupportedMessage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementMessagingSupportedMessage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementMessagingSupportedMessage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// sender | receiver
    pub mode: crate::r5::coded::Coded<crate::r5::codes::EventCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Message supported by this system
    pub definition: types::Canonical,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// Document definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::capability_statement::CapabilityStatementDocument;
/// use fhir::r5::types;
///
/// let value = CapabilityStatementDocument {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: CapabilityStatementDocument = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatementDocument {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// producer | consumer
    pub mode: crate::r5::coded::Coded<crate::r5::codes::DocumentMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Description of document support
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Constraint on the resources used in the document
    pub profile: types::Canonical,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,
}

/// The `CapabilityStatement.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CapabilityStatementVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
