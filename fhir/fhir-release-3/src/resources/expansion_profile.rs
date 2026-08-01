//! ExpansionProfile
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExpansionProfile
//!
//!
//!
//! Defines behaviour and contraints on the ValueSet Expansion operation
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ExpansionProfile Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfile;
/// use fhir::r3::types;
///
/// let value = ExpansionProfile {
///     include_designations: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `includeDesignations` is the name this serializes to on the wire.
/// assert_eq!(json["includeDesignations"], ::serde_json::json!(true));
///
/// let back: ExpansionProfile = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfile {
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

    /// Logical URI to reference this expansion profile (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the expansion profile
    pub identifier: Option<types::Identifier>,

    /// Business version of the expansion profile
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this expansion profile (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date this was last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the expansion profile
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for expansion profile (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Fix use of a code system to a particular version
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixed_version: Vec<ExpansionProfileFixedVersion>,

    /// Systems/Versions to be exclude
    pub excluded_system: Option<ExpansionProfileExcludedSystem>,

    /// Whether the expansion should include concept designations
    pub include_designations: Option<types::Boolean>,
    /// Primitive extension sibling for [`include_designations`](Self::include_designations) (FHIR `_includeDesignations`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_includeDesignations")]
    pub include_designations_ext: Option<types::Element>,

    /// When the expansion profile imposes designation contraints
    pub designation: Option<ExpansionProfileDesignation>,

    /// Include or exclude the value set definition in the expansion
    pub include_definition: Option<types::Boolean>,
    /// Primitive extension sibling for [`include_definition`](Self::include_definition) (FHIR `_includeDefinition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_includeDefinition")]
    pub include_definition_ext: Option<types::Element>,

    /// Include or exclude inactive concepts in the expansion
    pub active_only: Option<types::Boolean>,
    /// Primitive extension sibling for [`active_only`](Self::active_only) (FHIR `_activeOnly`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_activeOnly")]
    pub active_only_ext: Option<types::Element>,

    /// Nested codes in the expansion or not
    pub exclude_nested: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude_nested`](Self::exclude_nested) (FHIR `_excludeNested`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excludeNested")]
    pub exclude_nested_ext: Option<types::Element>,

    /// Include or exclude codes which cannot be rendered in user interfaces in
    /// the value set expansion
    #[serde(rename = "excludeNotForUI")]
    pub exclude_not_for_ui: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude_not_for_ui`](Self::exclude_not_for_ui) (FHIR `_excludeNotForUI`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excludeNotForUI")]
    pub exclude_not_for_ui_ext: Option<types::Element>,

    /// Include or exclude codes which are post coordinated expressions in the
    /// value set expansion
    pub exclude_post_coordinated: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude_post_coordinated`](Self::exclude_post_coordinated) (FHIR `_excludePostCoordinated`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excludePostCoordinated")]
    pub exclude_post_coordinated_ext: Option<types::Element>,

    /// Specify the language for the display element of codes in the value set
    /// expansion
    pub display_language: Option<types::Code>,
    /// Primitive extension sibling for [`display_language`](Self::display_language) (FHIR `_displayLanguage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_displayLanguage")]
    pub display_language_ext: Option<types::Element>,

    /// Controls behaviour of the value set expand operation when value sets
    /// are too large to be completely expanded
    pub limited_expansion: Option<types::Boolean>,
    /// Primitive extension sibling for [`limited_expansion`](Self::limited_expansion) (FHIR `_limitedExpansion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_limitedExpansion")]
    pub limited_expansion_ext: Option<types::Element>,
}

/// A set of criteria that provide the constraints imposed on the value set
/// expansion by including or excluding designations.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileDesignation;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileDesignation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileDesignation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Designations to be included
    pub include: Option<ExpansionProfileDesignationInclude>,

    /// Designations to be excluded
    pub exclude: Option<ExpansionProfileDesignationExclude>,
}

/// Designations to be excluded.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileDesignationExclude;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileDesignationExclude {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileDesignationExclude = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileDesignationExclude {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The designation to be excluded
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ExpansionProfileDesignationExcludeDesignation>,
}

/// A data group for each designation to be excluded.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileDesignationExcludeDesignation;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileDesignationExcludeDesignation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileDesignationExcludeDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileDesignationExcludeDesignation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human language of the designation to be excluded
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// What kind of Designation to exclude
    pub r#use: Option<types::Coding>,
}

/// Designations to be included.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileDesignationInclude;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileDesignationInclude {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileDesignationInclude = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileDesignationInclude {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The designation to be included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ExpansionProfileDesignationIncludeDesignation>,
}

/// A data group for each designation to be included.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileDesignationIncludeDesignation;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileDesignationIncludeDesignation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileDesignationIncludeDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileDesignationIncludeDesignation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human language of the designation to be included
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// What kind of Designation to include
    pub r#use: Option<types::Coding>,
}

/// Code system, or a particular version of a code system to be excluded from
/// value set expansions.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileExcludedSystem;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileExcludedSystem {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileExcludedSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileExcludedSystem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific code system to be excluded
    pub system: types::Uri,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Specific version of the code system referred to
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// Fix use of a particular code system to a particular version.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::expansion_profile::ExpansionProfileFixedVersion;
/// use fhir::r3::types;
///
/// let value = ExpansionProfileFixedVersion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExpansionProfileFixedVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExpansionProfileFixedVersion {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// System to have its version fixed
    pub system: types::Uri,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Specific version of the code system referred to
    pub version: types::String,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// default | check | override
    pub mode: crate::coded::Coded<crate::r3::codes::SystemVersionProcessingMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExpansionProfile;

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
