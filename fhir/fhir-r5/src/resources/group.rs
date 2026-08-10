//! Group
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Group
//!
//! Version: 5.0.0
//!
//! Group Resource: Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively.
///
/// A Group resource describes a set of entities of the same kind — such as
/// people, animals, practitioners, devices, or specimens — that are grouped
/// together for a common purpose. Membership may be defined intensionally by a
/// set of characteristics (a definitional group) or extensionally by explicitly
/// listing members (an enumerated group). In FHIR R5 it is commonly used for
/// cohorts, research study populations, herds of animals, or collections of
/// devices, and is distinct from an Organization, which represents a formally
/// or legally recognized entity.
///
/// Clinically and administratively, Group is used wherever an action, order,
/// communication, or observation needs to apply to many entities at once
/// rather than to a single record — for example targeting a public health
/// intervention at a cohort of patients, enrolling a herd of animals in a
/// veterinary study, or scoping a `CarePlan` or `Communication` to a set of
/// recipients. The `type` and `membership` fields establish what kind of
/// entities the group contains and whether membership is rule-based
/// (`characteristic`) or explicitly enumerated (`member`), while `quantity`
/// and `managingEntity` support cases where the exact membership list is not
/// tracked by the system.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) and
/// `Practitioner`, `Device`, `Specimen`, and `Organization`, which are the
/// typical kinds of entities referenced as group members, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), used to describe
/// group characteristics.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::group::Group;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Group {
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

    /// Business Identifier for this Group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this group's record is in active use, as opposed to being retired or entered in error
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The kind of entities held by this group: person | animal | practitioner | device | careteam | healthcareservice | location | organization | relatedperson | specimen
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::GroupType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Basis for membership: definitional (rule-based, via `characteristic`) or enumerated (explicitly listed, via `member`)
    pub membership: crate::r5::coded::Coded<crate::r5::codes::GroupMembershipBasis>,
    /// Primitive extension sibling for [`membership`](Self::membership) (FHIR `_membership`).
    #[serde(rename = "_membership")]
    pub membership_ext: Option<types::Element>,

    /// Kind of Group members
    pub code: Option<types::CodeableConcept>,

    /// Label for Group
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Natural language description of the group
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Number of members
    pub quantity: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`).
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Entity that is the custodian of the Group's definition
    pub managing_entity: Option<types::Reference>,

    /// Rules for including or excluding members of a definitional Group by trait
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<GroupCharacteristic>,

    /// The explicit list of entities that are members of an enumerated Group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GroupMember>,
}

/// Include / Exclude group members by Trait.
///
/// Identifies traits whose presence or absence is used to describe members of a
/// definitional Group.
/// # Examples
///
/// ```
/// use fhir::r5::resources::group::GroupCharacteristic;
/// use fhir::r5::types;
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
#[serde(from = "GroupCharacteristicDe")]
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

    /// The `Group.characteristic.value[x]` choice element (0..1); see [`GroupCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<GroupCharacteristicValue>,

    /// Group includes or excludes
    pub exclude: types::Boolean,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`).
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Period over which characteristic is tested
    pub period: Option<types::Period>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupCharacteristicDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<GroupCharacteristicValue>,
    exclude: types::Boolean,
    #[serde(rename = "_exclude")]
    exclude_ext: Option<types::Element>,
    period: Option<types::Period>,
}

impl ::core::convert::From<GroupCharacteristicDe> for GroupCharacteristic {
    fn from(v: GroupCharacteristicDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
            exclude: v.exclude,
            exclude_ext: v.exclude_ext,
            period: v.period,
        }
    }
}

/// Who or what is in group.
///
/// Identifies the resource instances that are members of an enumerated Group.
/// # Examples
///
/// ```
/// use fhir::r5::resources::group::GroupMember;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`inactive`](Self::inactive) (FHIR `_inactive`).
    #[serde(rename = "_inactive")]
    pub inactive_ext: Option<types::Element>,
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
/// The `Group.characteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GroupCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
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
