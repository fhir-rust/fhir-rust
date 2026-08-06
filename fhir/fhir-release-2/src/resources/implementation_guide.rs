//! ImplementationGuide
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
//!
//!
//!
//! A set of rules about how FHIR is used
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImplementationGuide Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::implementation_guide::ImplementationGuide;
///
/// let value = ImplementationGuide::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImplementationGuide = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuide {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Absolute URL used to reference this Implementation Guide
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Logical id for this version of the Implementation Guide
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this Implementation Guide
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired
    pub status: crate::coded::Coded<crate::r2::codes::ConformanceResourceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Name of the publisher (Organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ImplementationGuideContact>,

    /// Date for this version of the Implementation Guide
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Natural language description of the Implementation Guide
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The implementation guide is intended to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// FHIR Version this Implementation Guide targets
    pub fhir_version: Option<types::Id>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// Another Implementation guide this depends on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency: Vec<ImplementationGuideDependency>,

    /// Group of resources as used in .page.package
    pub package: ::vec1::Vec1<ImplementationGuidePackage>,

    /// Profiles that apply globally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global: Vec<ImplementationGuideGlobal>,

    /// Image, css, script, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binary: Vec<types::Uri>,
    /// Primitive extension sibling for [`binary`](Self::binary) (FHIR `_binary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_binary")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binary_ext: Vec<Option<types::Element>>,

    /// Page/Section in the Guide
    pub page: ImplementationGuidePage,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::implementation_guide::ImplementationGuideContact;
/// use fhir::r2::types;
///
/// let value = ImplementationGuideContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuideContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuideContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a individual to contact
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details for individual or publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,
}

/// Another implementation guide that this implementation depends on.
/// Typically, an implementation guide uses value sets, profiles etc.defined in
/// other implementation guides.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::implementation_guide::ImplementationGuideDependency;
/// use fhir::r2::types;
///
/// let value = ImplementationGuideDependency {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ImplementationGuideDependency = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuideDependency {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// reference | inclusion
    pub r#type: crate::coded::Coded<crate::r2::codes::GuideDependencyType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Where to find dependency
    pub uri: types::Uri,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,
}

/// A set of profiles that all resources covered by this implementation guide
/// must conform to.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::implementation_guide::ImplementationGuideGlobal;
/// use fhir::r2::types;
///
/// let value = ImplementationGuideGlobal {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ImplementationGuideGlobal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuideGlobal {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type this profiles applies to
    pub r#type: crate::coded::Coded<crate::r2::codes::ResourceTypes>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Profile that all resources must conform to
    pub profile: types::Reference,
}

/// A logical group of resources. Logical groups can be used when building
/// pages.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::implementation_guide::ImplementationGuidePackage;
///
/// let value = ImplementationGuidePackage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImplementationGuidePackage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuidePackage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name used .page.package
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable text describing the package
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Resource in the implementation guide
    pub resource: ::vec1::Vec1<ImplementationGuidePackageResource>,
}

/// A resource that is part of the implementation guide. Conformance resources
/// (value set, structure definition, conformance statements etc.) are obvious
/// candidates for inclusion, but any kind of resource can be included as an
/// example resource.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::implementation_guide::ImplementationGuidePackageResource;
/// use fhir::r2::types;
///
/// let value = ImplementationGuidePackageResource {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ImplementationGuidePackageResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuidePackageResource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// example | terminology | profile | extension | dictionary | logical
    pub purpose: crate::coded::Coded<crate::r2::codes::GuideResourcePurpose>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Human Name for the resource
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Reason why included in guide
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Short code to identify the resource
    pub acronym: Option<types::String>,
    /// Primitive extension sibling for [`acronym`](Self::acronym) (FHIR `_acronym`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_acronym")]
    pub acronym_ext: Option<types::Element>,

    /// Location of the resource
    /// The `ImplementationGuide.package.resource.source[x]` choice element (1..1); see [`ImplementationGuidePackageResourceSource`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub source: Option<ImplementationGuidePackageResourceSource>,

    /// Resource this is an example of (if applicable)
    pub example_for: Option<types::Reference>,
}

/// A page / section in the implementation guide. The root page is the
/// implementation guide home page.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::implementation_guide::ImplementationGuidePage;
/// use fhir::r2::types;
///
/// let value = ImplementationGuidePage {
///     format: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `format` is the name this serializes to on the wire.
/// assert_eq!(json["format"], ::serde_json::json!("final"));
///
/// let back: ImplementationGuidePage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImplementationGuidePage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Where to find that page
    pub source: types::Uri,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Short name shown for navigational assistance
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// page | example | list | include | directory | dictionary | toc |
    /// resource
    pub kind: crate::coded::Coded<crate::r2::codes::GuidePageKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Kind of resource to include in the list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<crate::coded::Coded<crate::r2::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_ext: Vec<Option<types::Element>>,

    /// Name of package to include
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package: Vec<types::String>,
    /// Primitive extension sibling for [`package`](Self::package) (FHIR `_package`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_package")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_ext: Vec<Option<types::Element>>,

    /// Format of the page (e.g. html, markdown, etc.)
    pub format: Option<types::Code>,
    /// Primitive extension sibling for [`format`](Self::format) (FHIR `_format`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_format")]
    pub format_ext: Option<types::Element>,

    /// Nested Pages / Sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub page: Vec<ImplementationGuidePage>,
}

/// The `ImplementationGuide.package.resource.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ImplementationGuidePackageResourceSource {
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}
