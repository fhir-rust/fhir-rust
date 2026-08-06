//! VisionPrescription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
//!
//! Version: 4.0.1
//!
//! Prescription for vision correction products for a patient
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::vision_prescription::VisionPrescription;
///
/// let value = VisionPrescription::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: VisionPrescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VisionPrescription {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for vision prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Who prescription is for
    pub patient: types::Reference,

    /// Created during encounter / admission / stay
    pub encounter: Option<types::Reference>,

    /// When prescription was authorized
    pub date_written: types::DateTime,
    /// Primitive extension sibling for [`date_written`](Self::date_written) (FHIR `_dateWritten`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateWritten")]
    pub date_written_ext: Option<types::Element>,

    /// Who authorized the vision prescription
    pub prescriber: types::Reference,

    /// Vision lens authorization
    pub lens_specification: ::vec1::Vec1<VisionPrescriptionLensSpecification>,
}

/// Contain the details of the individual lens specifications and serves as the
/// authorization for the fullfillment by certified professionals.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::vision_prescription::VisionPrescriptionLensSpecification;
/// use fhir::r4::types;
///
/// let value = VisionPrescriptionLensSpecification {
///     axis: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `axis` is the name this serializes to on the wire.
/// assert_eq!(json["axis"], ::serde_json::json!(42));
///
/// let back: VisionPrescriptionLensSpecification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VisionPrescriptionLensSpecification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Product to be supplied
    pub product: types::CodeableConcept,

    /// right | left
    pub eye: crate::coded::Coded<crate::r4::codes::VisionEyeCodes>,
    /// Primitive extension sibling for [`eye`](Self::eye) (FHIR `_eye`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eye")]
    pub eye_ext: Option<types::Element>,

    /// Power of the lens
    pub sphere: Option<types::Decimal>,
    /// Primitive extension sibling for [`sphere`](Self::sphere) (FHIR `_sphere`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sphere")]
    pub sphere_ext: Option<types::Element>,

    /// Lens power for astigmatism
    pub cylinder: Option<types::Decimal>,
    /// Primitive extension sibling for [`cylinder`](Self::cylinder) (FHIR `_cylinder`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cylinder")]
    pub cylinder_ext: Option<types::Element>,

    /// Lens meridian which contain no power for astigmatism
    pub axis: Option<types::Integer>,
    /// Primitive extension sibling for [`axis`](Self::axis) (FHIR `_axis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_axis")]
    pub axis_ext: Option<types::Element>,

    /// Eye alignment compensation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prism: Vec<VisionPrescriptionLensSpecificationPrism>,

    /// Added power for multifocal levels
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

    /// Color required
    pub color: Option<types::String>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_color")]
    pub color_ext: Option<types::Element>,

    /// Brand required
    pub brand: Option<types::String>,
    /// Primitive extension sibling for [`brand`](Self::brand) (FHIR `_brand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_brand")]
    pub brand_ext: Option<types::Element>,

    /// Notes for coatings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Allows for adjustment on two axis.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::vision_prescription::VisionPrescriptionLensSpecificationPrism;
/// use fhir::r4::types;
///
/// let value = VisionPrescriptionLensSpecificationPrism {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: VisionPrescriptionLensSpecificationPrism = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VisionPrescriptionLensSpecificationPrism {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Amount of adjustment
    pub amount: types::Decimal,
    /// Primitive extension sibling for [`amount`](Self::amount) (FHIR `_amount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_amount")]
    pub amount_ext: Option<types::Element>,

    /// up | down | in | out
    pub base: crate::coded::Coded<crate::r4::codes::VisionBaseCodes>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,
}
