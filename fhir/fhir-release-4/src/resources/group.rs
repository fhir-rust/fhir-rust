//! Group
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Group
//!
//! Version: 4.0.1
//!
//! Group of multiple entities
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are
/// not formally or legally recognized; i.e. a collection of entities that
/// isn't an Organization.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::group::Group;
/// use fhir::r4::types;
///
/// let value = Group {
///     active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `active` is the name this serializes to on the wire.
/// assert_eq!(json["active"], ::serde_json::json!(true));
///
/// let back: Group = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this group's record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// person | animal | practitioner | device | medication | substance
    pub r#type: crate::coded::Coded<crate::r4::codes::GroupType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Descriptive or actual
    pub actual: types::Boolean,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// Kind of Group members
    pub code: Option<types::CodeableConcept>,

    /// Label for Group
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Number of members
    pub quantity: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Entity that is the custodian of the Group's definition
    pub managing_entity: Option<types::Reference>,

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
/// use fhir::r4::resources::group::GroupCharacteristic;
/// use fhir::r4::types;
///
/// let value = GroupCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GroupCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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

    /// Period over which characteristic is tested
    pub period: Option<types::Period>,
}

/// Identifies the resource instances that are members of the group.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::group::GroupMember;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// Period member belonged to the group
    pub period: Option<types::Period>,

    /// If member is no longer in group
    pub inactive: Option<types::Boolean>,
    /// Primitive extension sibling for [`inactive`](Self::inactive) (FHIR `_inactive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inactive")]
    pub inactive_ext: Option<types::Element>,
}

/// The `Group.characteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r4::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
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
