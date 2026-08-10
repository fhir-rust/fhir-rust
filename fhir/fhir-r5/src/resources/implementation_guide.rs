//! ImplementationGuide
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
//!
//! Version: 5.0.0
//!
//! ImplementationGuide Resource: A set of rules of how a particular interoperability or standards problem is solved.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and
/// to publish a computable definition of all the parts. An ImplementationGuide
/// bundles together conformance resources, narrative pages, and build
/// parameters so that a specification can be rendered, validated, and
/// distributed as an NPM package.
///
/// Authors of national base specifications, domain-specific implementation
/// guides, and local/regional profiles use `ImplementationGuide` to declare the
/// scope of their work: which artifacts (profiles, extensions, value sets,
/// examples, and other resources) belong to the guide, which other guides it
/// depends on, any profiles that apply globally across the guide, and the page
/// structure and build parameters consumed by publication tooling such as the
/// FHIR IG Publisher. The `definition` element captures the authoring-time
/// content of the guide, while the `manifest` element describes a already
/// assembled/published guide, including the rendered location of each
/// contained resource and page.
///
/// # Related resources
///
/// An `ImplementationGuide` typically references and constrains other
/// conformance resources such as `StructureDefinition`, `ValueSet`,
/// `CodeSystem`, `CapabilityStatement`, and `SearchParameter`, and it may
/// include example instances of clinical or administrative resources (for
/// example [`Patient`](crate::r5::resources::patient::Patient)). Coded
/// metadata elements on this resource, such as jurisdiction, are represented
/// using [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::implementation_guide::ImplementationGuide;
///
/// let value = ImplementationGuide::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImplementationGuide = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ImplementationGuideDe")]
pub struct ImplementationGuide {
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

    /// Canonical identifier for this implementation guide, represented as a URI (globally unique); used to reference this guide from other artifacts and registries
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the implementation guide (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the implementation guide
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ImplementationGuide.versionAlgorithm[x]` choice element (0..1); see [`ImplementationGuideVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ImplementationGuideVersionAlgorithm>,

    /// Name for this implementation guide (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this implementation guide (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown; reflects the maturity/publication lifecycle status of this guide
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
    pub date: Option<types::DateTime>,
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

    /// Natural language description of the implementation guide
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for implementation guide (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this implementation guide is defined
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

    /// NPM Package name for IG; identifies the published npm package used by build and installation tooling
    pub package_id: types::Id,
    /// Primitive extension sibling for [`package_id`](Self::package_id) (FHIR `_packageId`).
    #[serde(rename = "_packageId")]
    pub package_id_ext: Option<types::Element>,

    /// SPDX license code for this IG (or not-open-source)
    pub license: Option<crate::r5::coded::Coded<crate::r5::codes::SpdxLicense>>,
    /// Primitive extension sibling for [`license`](Self::license) (FHIR `_license`).
    #[serde(rename = "_license")]
    pub license_ext: Option<types::Element>,

    /// FHIR Version(s) this Implementation Guide targets; conformance artifacts within the guide are validated against these versions
    pub fhir_version: ::vec1::Vec1<crate::r5::coded::Coded<crate::r5::codes::FhirVersion>>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`).
    #[serde(rename = "_fhirVersion")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fhir_version_ext: Vec<Option<types::Element>>,

    /// Another Implementation guide this depends on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ImplementationGuideDependsOn>,

    /// Profiles that apply globally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global: Vec<ImplementationGuideGlobal>,

    /// Information needed to build the IG; the authoring-time content, including groupings, resources, pages, and parameters
    pub definition: Option<ImplementationGuideDefinition>,

    /// Information about an assembled IG; describes the rendered, published output of the guide
    pub manifest: Option<ImplementationGuideManifest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationGuideDe {
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
    url: types::Uri,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<ImplementationGuideVersionAlgorithm>,
    name: types::String,
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
    date: Option<types::DateTime>,
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
    package_id: types::Id,
    #[serde(rename = "_packageId")]
    package_id_ext: Option<types::Element>,
    license: Option<crate::r5::coded::Coded<crate::r5::codes::SpdxLicense>>,
    #[serde(rename = "_license")]
    license_ext: Option<types::Element>,
    fhir_version: ::vec1::Vec1<crate::r5::coded::Coded<crate::r5::codes::FhirVersion>>,
    #[serde(rename = "_fhirVersion")]
    #[serde(default)]
    fhir_version_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    depends_on: Vec<ImplementationGuideDependsOn>,
    #[serde(default)]
    global: Vec<ImplementationGuideGlobal>,
    definition: Option<ImplementationGuideDefinition>,
    manifest: Option<ImplementationGuideManifest>,
}

impl ::core::convert::From<ImplementationGuideDe> for ImplementationGuide {
    fn from(v: ImplementationGuideDe) -> Self {
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
            package_id: v.package_id,
            package_id_ext: v.package_id_ext,
            license: v.license,
            license_ext: v.license_ext,
            fhir_version: v.fhir_version,
            fhir_version_ext: v.fhir_version_ext,
            depends_on: v.depends_on,
            global: v.global,
            definition: v.definition,
            manifest: v.manifest,
        }
    }
}

/// Another Implementation guide that this implementation guide depends on.
/// Dependencies are typically expressed against other published IGs whose
/// profiles, value sets, or examples are reused.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDependsOn;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDependsOn {
///     package_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `packageId` is the name this serializes to on the wire.
/// assert_eq!(json["packageId"], ::serde_json::json!("pat-1"));
///
/// let back: ImplementationGuideDependsOn = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identity of the IG that this depends on
    pub uri: types::Canonical,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// NPM Package name for IG this depends on
    pub package_id: Option<types::Id>,
    /// Primitive extension sibling for [`package_id`](Self::package_id) (FHIR `_packageId`).
    #[serde(rename = "_packageId")]
    pub package_id_ext: Option<types::Element>,

    /// Version of the IG
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Why dependency exists
    pub reason: Option<types::Markdown>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`).
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,
}

/// A profile that applies globally to all resources of a given type within the
/// implementation guide.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideGlobal;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideGlobal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideGlobal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideGlobal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type this profile applies to
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Profile that all resources must conform to
    pub profile: types::Canonical,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,
}

/// The information needed to build the implementation guide, including the
/// groupings, resources, pages, parameters, and templates that make up the IG.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinition;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Grouping used to present related resources in the IG
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ImplementationGuideDefinitionGrouping>,

    /// Resource in the implementation guide
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<ImplementationGuideDefinitionResource>,

    /// Page/Section in the Guide
    pub page: Option<ImplementationGuideDefinitionPage>,

    /// Defines how IG is built by tools
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<ImplementationGuideDefinitionParameter>,

    /// A template for building resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub template: Vec<ImplementationGuideDefinitionTemplate>,
}

/// A logical group used to present related resources together in the rendered
/// implementation guide.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinitionGrouping;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinitionGrouping {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ImplementationGuideDefinitionGrouping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionGrouping {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Descriptive name for the package
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable text describing the package
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// A resource that is part of the implementation guide, with metadata about how
/// it is included and rendered.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinitionResource;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinitionResource {
///     is_example: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isExample` is the name this serializes to on the wire.
/// assert_eq!(json["isExample"], ::serde_json::json!(true));
///
/// let back: ImplementationGuideDefinitionResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location of the resource
    pub reference: types::Reference,

    /// Versions this applies to (if different to IG)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fhir_version: Vec<crate::r5::coded::Coded<crate::r5::codes::FhirVersion>>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`).
    #[serde(rename = "_fhirVersion")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fhir_version_ext: Vec<Option<types::Element>>,

    /// Human readable name for the resource
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Reason why included in guide
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Is this an example
    pub is_example: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_example`](Self::is_example) (FHIR `_isExample`).
    #[serde(rename = "_isExample")]
    pub is_example_ext: Option<types::Element>,

    /// Profile(s) this is an example of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Grouping this is part of
    pub grouping_id: Option<types::Id>,
    /// Primitive extension sibling for [`grouping_id`](Self::grouping_id) (FHIR `_groupingId`).
    #[serde(rename = "_groupingId")]
    pub grouping_id_ext: Option<types::Element>,
}

/// A page or section within the implementation guide. Pages nest recursively to
/// form the navigation tree of the rendered guide.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinitionPage;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinitionPage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideDefinitionPage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ImplementationGuideDefinitionPageDe")]
pub struct ImplementationGuideDefinitionPage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `ImplementationGuide.definition.page.source[x]` choice element (0..1); see [`ImplementationGuideDefinitionPageSource`].
    #[serde(flatten)]
    pub source: Option<ImplementationGuideDefinitionPageSource>,

    /// Name of the page when published
    pub name: types::Url,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Short title shown for navigational assistance
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// html | markdown | xml | generated
    pub generation: crate::r5::coded::Coded<crate::r5::codes::GuidePageGeneration>,
    /// Primitive extension sibling for [`generation`](Self::generation) (FHIR `_generation`).
    #[serde(rename = "_generation")]
    pub generation_ext: Option<types::Element>,

    /// Nested Pages / Sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub page: Vec<ImplementationGuideDefinitionPage>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationGuideDefinitionPageDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    source: crate::r5::choice::Slot<ImplementationGuideDefinitionPageSource>,
    name: types::Url,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: types::String,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    generation: crate::r5::coded::Coded<crate::r5::codes::GuidePageGeneration>,
    #[serde(rename = "_generation")]
    generation_ext: Option<types::Element>,
    #[serde(default)]
    page: Vec<ImplementationGuideDefinitionPage>,
}

impl ::core::convert::From<ImplementationGuideDefinitionPageDe>
    for ImplementationGuideDefinitionPage
{
    fn from(v: ImplementationGuideDefinitionPageDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            source: v.source.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            generation: v.generation,
            generation_ext: v.generation_ext,
            page: v.page,
        }
    }
}

/// A parameter that defines how the implementation guide is built by tooling.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinitionParameter;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinitionParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideDefinitionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies parameter
    pub code: types::Coding,

    /// Value for named type
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A template used for building resources within the implementation guide.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideDefinitionTemplate;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideDefinitionTemplate {
///     scope: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `scope` is the name this serializes to on the wire.
/// assert_eq!(json["scope"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideDefinitionTemplate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideDefinitionTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of template specified
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The source location for the template
    pub source: types::String,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`).
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// The scope in which the template applies
    pub scope: Option<types::String>,
    /// Primitive extension sibling for [`scope`](Self::scope) (FHIR `_scope`).
    #[serde(rename = "_scope")]
    pub scope_ext: Option<types::Element>,
}

/// Information about an assembled implementation guide, describing where the
/// rendered output and its constituent resources and pages can be found.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::implementation_guide::ImplementationGuideManifest;
///
/// let value = ImplementationGuideManifest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImplementationGuideManifest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location of rendered implementation guide
    pub rendering: Option<types::Url>,
    /// Primitive extension sibling for [`rendering`](Self::rendering) (FHIR `_rendering`).
    #[serde(rename = "_rendering")]
    pub rendering_ext: Option<types::Element>,

    /// Resource in the implementation guide
    pub resource: vec1::Vec1<ImplementationGuideManifestResource>,

    /// HTML page within the parent IG
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub page: Vec<ImplementationGuideManifestPage>,

    /// Image within the IG
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<types::String>,
    /// Primitive extension sibling for [`image`](Self::image) (FHIR `_image`).
    #[serde(rename = "_image")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_ext: Vec<Option<types::Element>>,

    /// Additional linkable file in IG
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other: Vec<types::String>,
    /// Primitive extension sibling for [`other`](Self::other) (FHIR `_other`).
    #[serde(rename = "_other")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_ext: Vec<Option<types::Element>>,
}

/// A resource in the assembled implementation guide, with its rendered location.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideManifestResource;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideManifestResource {
///     is_example: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isExample` is the name this serializes to on the wire.
/// assert_eq!(json["isExample"], ::serde_json::json!(true));
///
/// let back: ImplementationGuideManifestResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location of the resource
    pub reference: types::Reference,

    /// Is this an example
    pub is_example: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_example`](Self::is_example) (FHIR `_isExample`).
    #[serde(rename = "_isExample")]
    pub is_example_ext: Option<types::Element>,

    /// Profile(s) this is an example of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Relative path for page in IG
    pub relative_path: Option<types::Url>,
    /// Primitive extension sibling for [`relative_path`](Self::relative_path) (FHIR `_relativePath`).
    #[serde(rename = "_relativePath")]
    pub relative_path_ext: Option<types::Element>,
}

/// An HTML page within the assembled implementation guide.
/// # Examples
///
/// ```
/// use fhir::r5::resources::implementation_guide::ImplementationGuideManifestPage;
/// use fhir::r5::types;
///
/// let value = ImplementationGuideManifestPage {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideManifestPage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuideManifestPage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// HTML page name
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Title of the page, for references
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Anchor available on the page
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchor: Vec<types::String>,
    /// Primitive extension sibling for [`anchor`](Self::anchor) (FHIR `_anchor`).
    #[serde(rename = "_anchor")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchor_ext: Vec<Option<types::Element>>,
}

/// The `ImplementationGuide.definition.page.source[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ImplementationGuideDefinitionPageSource {
    /// `sourceUrl` variant.
    #[fhir("sourceUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `sourceMarkdown` variant.
    #[fhir("sourceMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
}

/// The `ImplementationGuide.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ImplementationGuideVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
