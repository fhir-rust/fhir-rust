//! Group
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Group
//!
//! Version: 6.0.0-ballot3
//!
//! Group of multiple entities
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not typically expected to act collectively.
/// These collections are also not typically formally or legally recognized.
/// NOTE: Group may be used to define families or households, which in some
/// circumstances may act collectively or have a degree of legal or formal
/// recognition. This should be considered an exception. When Group is used for
/// types of entities other than Patient or RelatedPerson, the expectation
/// remains that the Group will not act collectively or have formal recognition
/// \- use Organization if these behaviors are needed. For example, it is
/// possible for a 'family' Group to be a performer of an Observation or owner
/// of a Task. However, this is not permitted for a Group made up of
/// Practitioners, PractitionerRoles or Organizations. Organization or CareTeam
/// would need to be used instead. A Group of Practitioners could, however, be
/// a subject of an Observation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::group::Group;
/// use fhir::r6::types;
///
/// let value = Group {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: Group = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Group {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this Group, represented as an absolute URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for this Group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the Group
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `Group.versionAlgorithm[x]` choice element (0..1); see [`GroupVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<GroupVersionAlgorithm>,

    /// Label for Group
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this Group (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: Option<crate::coded::Coded<crate::r6::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the group
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Why this Group is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// person | animal | practitioner | device | careteam | healthcareservice
    /// | location | organization | relatedperson | specimen | medication |
    /// substance | biologicallyDerivedProduct | nutritionProduct
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::GroupType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// definitional | conceptual | enumerated
    pub membership: crate::coded::Coded<crate::r6::codes::GroupMembershipBasis>,
    /// Primitive extension sibling for [`membership`](Self::membership) (FHIR `_membership`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_membership")]
    pub membership_ext: Option<types::Element>,

    /// Use of the Group (and by implication, kind of members)
    pub code: Option<types::CodeableConcept>,

    /// Number of members
    pub quantity: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Entity that is the custodian of the Group's definition
    pub managing_entity: Option<types::Reference>,

    /// all-of | any-of | at-least | at-most | except-subset
    pub combination_method:
        Option<crate::coded::Coded<crate::r6::codes::GroupCharacteristicCombination>>,
    /// Primitive extension sibling for [`combination_method`](Self::combination_method) (FHIR `_combinationMethod`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_combinationMethod")]
    pub combination_method_ext: Option<types::Element>,

    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    pub combination_threshold: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`combination_threshold`](Self::combination_threshold) (FHIR `_combinationThreshold`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_combinationThreshold")]
    pub combination_threshold_ext: Option<types::Element>,

    /// Include / Exclude group members by Trait
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<GroupCharacteristic>,

    /// Who or what is in group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GroupMember>,
}

/// Identifies traits whose presence r absence is shared by members of the
/// group.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::group::GroupCharacteristic;
/// use fhir::r6::types;
///
/// let value = GroupCharacteristic {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: GroupCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GroupCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Kind of characteristic
    pub code: types::CodeableConcept,

    /// Value held by characteristic
    /// The `Group.characteristic.value[x]` choice element (1..1); see [`GroupCharacteristicValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<GroupCharacteristicValue>,

    /// Group includes or excludes
    pub exclude: types::Boolean,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Natural language description of the characteristic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Method for how the characteristic value was determined
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method: Vec<types::CodeableConcept>,

    /// Defines the characteristic
    /// The `Group.characteristic.determinedBy[x]` choice element (0..1); see [`GroupCharacteristicDeterminedBy`].
    #[serde(flatten)]
    pub determined_by: Option<GroupCharacteristicDeterminedBy>,

    /// Reference point for comparison
    pub offset: Option<types::CodeableConcept>,

    /// Number of occurrences meeting the characteristic
    /// The `Group.characteristic.instances[x]` choice element (0..1); see [`GroupCharacteristicInstances`].
    #[serde(flatten)]
    pub instances: Option<GroupCharacteristicInstances>,

    /// Length of time in which the characteristic is met
    /// The `Group.characteristic.duration[x]` choice element (0..1); see [`GroupCharacteristicDuration`].
    #[serde(flatten)]
    pub duration: Option<GroupCharacteristicDuration>,

    /// Period over which characteristic is tested
    pub period: Option<types::Period>,

    /// Timing in which the characteristic is determined
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::RelativeTime>,
}

/// Identifies the resource instances that are members of the group.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::group::GroupMember;
/// use fhir::r6::types;
///
/// let value = GroupMember {
///     inactive: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `inactive` is the name this serializes to on the wire.
/// assert_eq!(json["inactive"], ::serde_json::json!(true));
///
/// let back: GroupMember = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GroupMember {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the group member
    pub entity: types::Reference,

    /// Code that describes how user is part of the group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub involvement: Vec<types::CodeableConcept>,

    /// Period member belonged to the group
    pub period: Option<types::Period>,

    /// If member is no longer in group
    pub inactive: Option<types::Boolean>,
    /// Primitive extension sibling for [`inactive`](Self::inactive) (FHIR `_inactive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inactive")]
    pub inactive_ext: Option<types::Element>,
}

/// The `Group.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GroupVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `Group.characteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `valueExpression` variant.
    #[fhir("valueExpression")]
    Expression(Box<types::Expression>),
}

/// The `Group.characteristic.determinedBy[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicDeterminedBy {
    /// `determinedByReference` variant.
    #[fhir("determinedByReference")]
    Reference(Box<types::Reference>),
    /// `determinedByExpression` variant.
    #[fhir("determinedByExpression")]
    Expression(Box<types::Expression>),
}

/// The `Group.characteristic.instances[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicInstances {
    /// `instancesUnsignedInt` variant.
    #[fhir("instancesUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `instancesRange` variant.
    #[fhir("instancesRange")]
    Range(Box<types::Range>),
}

/// The `Group.characteristic.duration[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicDuration {
    /// `durationDuration` variant.
    #[fhir("durationDuration")]
    Duration(Box<types::Duration>),
    /// `durationRange` variant.
    #[fhir("durationRange")]
    Range(Box<types::Range>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Group;

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
