//! DeviceComponent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceComponent
//!
//!
//!
//! An instance of a medical-related component of a medical device
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DeviceComponent Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_component::DeviceComponent;
/// use fhir::r3::types;
///
/// let value = DeviceComponent {
///     last_system_change: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lastSystemChange` is the name this serializes to on the wire.
/// assert_eq!(json["lastSystemChange"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceComponent {
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

    /// Instance id assigned by the software stack
    pub identifier: types::Identifier,

    /// What kind of component it is
    pub r#type: types::CodeableConcept,

    /// Recent system change timestamp
    pub last_system_change: Option<types::Instant>,
    /// Primitive extension sibling for [`last_system_change`](Self::last_system_change) (FHIR `_lastSystemChange`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastSystemChange")]
    pub last_system_change_ext: Option<types::Element>,

    /// Top-level device resource link
    pub source: Option<types::Reference>,

    /// Parent resource link
    pub parent: Option<types::Reference>,

    /// Current operational status of the component, for example On, Off or
    /// Standby
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operational_status: Vec<types::CodeableConcept>,

    /// Current supported parameter group
    pub parameter_group: Option<types::CodeableConcept>,

    /// other | chemical | electrical | impedance | nuclear | optical | thermal
    /// | biological | mechanical | acoustical | manual+
    pub measurement_principle: Option<crate::coded::Coded<crate::r3::codes::MeasurementPrinciple>>,
    /// Primitive extension sibling for [`measurement_principle`](Self::measurement_principle) (FHIR `_measurementPrinciple`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measurementPrinciple")]
    pub measurement_principle_ext: Option<types::Element>,

    /// Specification details such as Component Revisions, or Serial Numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub production_specification: Vec<DeviceComponentProductionSpecification>,

    /// Language code for the human-readable text strings produced by the
    /// device
    pub language_code: Option<types::CodeableConcept>,
}

/// The production specification such as component revision, serial number,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_component::DeviceComponentProductionSpecification;
/// use fhir::r3::types;
///
/// let value = DeviceComponentProductionSpecification {
///     production_spec: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productionSpec` is the name this serializes to on the wire.
/// assert_eq!(json["productionSpec"], ::serde_json::json!("abc"));
///
/// let back: DeviceComponentProductionSpecification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceComponentProductionSpecification {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type or kind of production specification, for example serial number or
    /// software revision
    pub spec_type: Option<types::CodeableConcept>,

    /// Internal component unique identification
    pub component_id: Option<types::Identifier>,

    /// A printable string defining the component
    pub production_spec: Option<types::String>,
    /// Primitive extension sibling for [`production_spec`](Self::production_spec) (FHIR `_productionSpec`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productionSpec")]
    pub production_spec_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceComponent;

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
