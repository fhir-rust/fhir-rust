//! ChargeItemDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition
//!
//! Version: 5.0.0
//!
//! ChargeItemDefinition Resource: The properties that apply to the (billing) codes necessary to calculate costs and prices.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The ChargeItemDefinition resource provides the properties that apply to the
/// (billing) codes necessary to calculate costs and prices. The properties may
/// differ largely depending on type and realm, therefore this resource gives
/// only a rough structure and requires profiling for each type of billing code
/// system. It is used to define the applicability rules and the price
/// components (base price, surcharges, discounts, taxes) associated with a
/// billing code or product type.
///
/// ChargeItemDefinition acts as a catalog or master-data entry: it is defined
/// once by a payer, provider, or billing authority and then referenced by
/// individual `ChargeItem` instances at the point of care or invoicing to
/// determine which price components and applicability conditions apply. A
/// definition can be scoped to a specific billing code (via `code`), to
/// particular instances (via `instance`), and can be conditioned on runtime
/// facts through the `applicability` expressions, allowing definitions to
/// vary by context such as payer contract, patient status, or effective
/// period. Definitions may also derive from, or supersede, other definitions
/// via `derived_from_uri`, `part_of`, and `replaces`, supporting versioned
/// and modular billing rule sets.
///
/// # Related resources
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used for the
///   `code` and `jurisdiction` elements.
/// - [`Reference`](crate::r5::types::Reference) values in `instance` typically
///   point to `ChargeItem`, `ActivityDefinition`, `PlanDefinition`, or `DeviceDefinition` resources.
/// - The related `ChargeItem` resource records an actual billable event and
///   references a `ChargeItemDefinition` to apply its pricing rules.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item_definition::ChargeItemDefinition;
/// use fhir::r5::types;
///
/// let value = ChargeItemDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ChargeItemDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ChargeItemDefinitionDe")]
pub struct ChargeItemDefinition {
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

    /// Canonical identifier for this charge item definition, represented as a URI (globally unique); used to reference this definition from a `ChargeItem`
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the charge item definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the charge item definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ChargeItemDefinition.versionAlgorithm[x]` choice element (0..1); see [`ChargeItemDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ChargeItemDefinitionVersionAlgorithm>,

    /// Name for this charge item definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this charge item definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Underlying externally-defined charge item definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`derived_from_uri`](Self::derived_from_uri) (FHIR `_derivedFromUri`).
    #[serde(rename = "_derivedFromUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri_ext: Vec<Option<types::Element>>,

    /// A larger definition of which this particular definition is a component or step
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Canonical>,
    /// Primitive extension sibling for [`part_of`](Self::part_of) (FHIR `_partOf`).
    #[serde(rename = "_partOf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of_ext: Vec<Option<types::Element>>,

    /// Completed or terminated request(s) whose function is taken by this new request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Canonical>,
    /// Primitive extension sibling for [`replaces`](Self::replaces) (FHIR `_replaces`).
    #[serde(rename = "_replaces")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces_ext: Vec<Option<types::Element>>,

    /// draft | active | retired | unknown
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

    /// Natural language description of the charge item definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for charge item definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this charge item definition is defined
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

    /// When the charge item definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the charge item definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// Billing code or product type this definition applies to
    pub code: Option<types::CodeableConcept>,

    /// Instances this definition applies to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<types::Reference>,

    /// Whether or not the billing code is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applicability: Vec<ChargeItemDefinitionApplicability>,

    /// Group of properties which are applicable under the same conditions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_group: Vec<ChargeItemDefinitionPropertyGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChargeItemDefinitionDe {
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
    version_algorithm: crate::r5::choice::Slot<ChargeItemDefinitionVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    #[serde(default)]
    derived_from_uri: Vec<types::Uri>,
    #[serde(rename = "_derivedFromUri")]
    #[serde(default)]
    derived_from_uri_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    part_of: Vec<types::Canonical>,
    #[serde(rename = "_partOf")]
    #[serde(default)]
    part_of_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    replaces: Vec<types::Canonical>,
    #[serde(rename = "_replaces")]
    #[serde(default)]
    replaces_ext: Vec<Option<types::Element>>,
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
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    instance: Vec<types::Reference>,
    #[serde(default)]
    applicability: Vec<ChargeItemDefinitionApplicability>,
    #[serde(default)]
    property_group: Vec<ChargeItemDefinitionPropertyGroup>,
}

impl ::core::convert::From<ChargeItemDefinitionDe> for ChargeItemDefinition {
    fn from(v: ChargeItemDefinitionDe) -> Self {
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
            derived_from_uri: v.derived_from_uri,
            derived_from_uri_ext: v.derived_from_uri_ext,
            part_of: v.part_of,
            part_of_ext: v.part_of_ext,
            replaces: v.replaces,
            replaces_ext: v.replaces_ext,
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
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            code: v.code,
            instance: v.instance,
            applicability: v.applicability,
            property_group: v.property_group,
        }
    }
}

/// Expressions that describe applicability criteria for the billing code.
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item_definition::ChargeItemDefinitionApplicability;
/// use fhir::r5::types;
///
/// let value = ChargeItemDefinitionApplicability {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ChargeItemDefinitionApplicability = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinitionApplicability {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Boolean-valued expression
    pub condition: Option<types::Expression>,

    /// When the charge item definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// Reference to / quotation of the external source of the group of properties
    pub related_artifact: Option<types::RelatedArtifact>,
}

/// Group of properties which are applicable under the same conditions. If no
/// applicability rules are established for the group, then all properties
/// always apply.
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item_definition::ChargeItemDefinitionPropertyGroup;
/// use fhir::r5::types;
///
/// let value = ChargeItemDefinitionPropertyGroup {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ChargeItemDefinitionPropertyGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinitionPropertyGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Conditions under which the priceComponent is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applicability: Vec<ChargeItemDefinitionApplicability>,

    /// Components of total line item price
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_component: Vec<types::MonetaryComponent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ChargeItemDefinition;

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
/// The `ChargeItemDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ChargeItemDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
