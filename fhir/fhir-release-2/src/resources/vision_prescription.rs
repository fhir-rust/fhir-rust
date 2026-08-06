//! VisionPrescription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
//!
//!
//!
//! Prescription for vision correction products for a patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for VisionPrescription Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::vision_prescription::VisionPrescription;
/// use fhir::r2::types;
///
/// let value = VisionPrescription {
///     date_written: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateWritten` is the name this serializes to on the wire.
/// assert_eq!(json["dateWritten"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: VisionPrescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct VisionPrescription {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// When prescription was authorized
    pub date_written: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_written`](Self::date_written) (FHIR `_dateWritten`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateWritten")]
    pub date_written_ext: Option<types::Element>,

    /// Who prescription is for
    pub patient: Option<types::Reference>,

    /// Who authorizes the vision product
    pub prescriber: Option<types::Reference>,

    /// Created during encounter / admission / stay
    pub encounter: Option<types::Reference>,

    /// Reason or indication for writing the prescription
    /// The `VisionPrescription.reason[x]` choice element (0..1); see [`VisionPrescriptionReason`].
    #[serde(flatten)]
    pub reason: Option<VisionPrescriptionReason>,

    /// Vision supply authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dispense: Vec<VisionPrescriptionDispense>,
}

/// Deals with details of the dispense part of the supply specification.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::vision_prescription::VisionPrescriptionDispense;
/// use fhir::r2::types;
///
/// let value = VisionPrescriptionDispense {
///     axis: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `axis` is the name this serializes to on the wire.
/// assert_eq!(json["axis"], ::serde_json::json!(42));
///
/// let back: VisionPrescriptionDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct VisionPrescriptionDispense {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Product to be supplied
    pub product: types::Coding,

    /// right | left
    pub eye: Option<crate::coded::Coded<crate::r2::codes::VisionEyeCodes>>,
    /// Primitive extension sibling for [`eye`](Self::eye) (FHIR `_eye`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eye")]
    pub eye_ext: Option<types::Element>,

    /// Lens sphere
    pub sphere: Option<types::Decimal>,
    /// Primitive extension sibling for [`sphere`](Self::sphere) (FHIR `_sphere`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sphere")]
    pub sphere_ext: Option<types::Element>,

    /// Lens cylinder
    pub cylinder: Option<types::Decimal>,
    /// Primitive extension sibling for [`cylinder`](Self::cylinder) (FHIR `_cylinder`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cylinder")]
    pub cylinder_ext: Option<types::Element>,

    /// Lens axis
    pub axis: Option<types::Integer>,
    /// Primitive extension sibling for [`axis`](Self::axis) (FHIR `_axis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_axis")]
    pub axis_ext: Option<types::Element>,

    /// Lens prism
    pub prism: Option<types::Decimal>,
    /// Primitive extension sibling for [`prism`](Self::prism) (FHIR `_prism`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prism")]
    pub prism_ext: Option<types::Element>,

    /// up | down | in | out
    pub base: Option<crate::coded::Coded<crate::r2::codes::VisionBaseCodes>>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Lens add
    pub add: Option<types::Decimal>,
    /// Primitive extension sibling for [`add`](Self::add) (FHIR `_add`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_add")]
    pub add_ext: Option<types::Element>,

    /// Contact lens power
    pub power: Option<types::Decimal>,
    /// Primitive extension sibling for [`power`](Self::power) (FHIR `_power`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_power")]
    pub power_ext: Option<types::Element>,

    /// Contact lens back curvature
    pub back_curve: Option<types::Decimal>,
    /// Primitive extension sibling for [`back_curve`](Self::back_curve) (FHIR `_backCurve`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_backCurve")]
    pub back_curve_ext: Option<types::Element>,

    /// Contact lens diameter
    pub diameter: Option<types::Decimal>,
    /// Primitive extension sibling for [`diameter`](Self::diameter) (FHIR `_diameter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_diameter")]
    pub diameter_ext: Option<types::Element>,

    /// Lens wear duration
    pub duration: Option<types::Quantity>,

    /// Lens add
    pub color: Option<types::String>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_color")]
    pub color_ext: Option<types::Element>,

    /// Lens add
    pub brand: Option<types::String>,
    /// Primitive extension sibling for [`brand`](Self::brand) (FHIR `_brand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_brand")]
    pub brand_ext: Option<types::Element>,

    /// Notes for coatings
    pub notes: Option<types::String>,
    /// Primitive extension sibling for [`notes`](Self::notes) (FHIR `_notes`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notes")]
    pub notes_ext: Option<types::Element>,
}

/// The `VisionPrescription.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum VisionPrescriptionReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VisionPrescription;

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
