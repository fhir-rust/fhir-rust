//! VisionPrescription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
//!
//! Version: 5.0.0
//!
//! VisionPrescription Resource: An authorization for the provision of glasses and/or contact lenses to a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
///
/// A VisionPrescription records the details of an eye examination that results
/// in the authorization of corrective lenses. It captures the prescriber, the
/// patient, the date the prescription was written, and one or more lens
/// specifications describing the optical parameters for each eye. In FHIR R5 it
/// supports both spectacle and contact lens prescriptions and is commonly used
/// in claims, dispensing, and ordering workflows.
///
/// Clinically, a VisionPrescription is generated after an eye examination and
/// is the authoritative order used downstream by opticians, optical labs, and
/// dispensing systems to manufacture or fit glasses or contact lenses. It is
/// analogous in role to a `MedicationRequest`, but scoped specifically to
/// ophthalmic devices. The resource distinguishes the clinical encounter in
/// which the prescription was written from the date the order itself was
/// authorized, and it associates a status (active, cancelled, draft, or
/// entered-in-error) to track the order's lifecycle.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject for whom
///   the prescription is written.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used to describe
///   the product supplied in each lens specification.
/// - `Encounter` and `Practitioner` — referenced, respectively, as the
///   originating encounter and the prescriber.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::vision_prescription::VisionPrescription;
///
/// let value = VisionPrescription::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: VisionPrescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription {
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

    /// Business Identifier for vision prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The current lifecycle status of the prescription: active | cancelled |
    /// draft | entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Reference to the [`Patient`](crate::r5::resources::patient::Patient)
    /// for whom the vision prescription was written.
    pub patient: types::Reference,

    /// Created during encounter / admission / stay
    pub encounter: Option<types::Reference>,

    /// The date on which the eye examination was performed and the
    /// prescription was authorized by the prescriber.
    pub date_written: types::DateTime,
    /// Primitive extension sibling for [`date_written`](Self::date_written) (FHIR `_dateWritten`).
    #[serde(rename = "_dateWritten")]
    pub date_written_ext: Option<types::Element>,

    /// Reference to the practitioner (for example an optometrist or
    /// ophthalmologist) who authorized the vision prescription.
    pub prescriber: types::Reference,

    /// One or more lens authorizations, each describing the optical
    /// parameters prescribed for a single eye.
    pub lens_specification: vec1::Vec1<VisionPrescriptionLensSpecification>,
}

/// Vision lens authorization.
///
/// Each lens specification contains the optical parameters authorized for a
/// single eye, including sphere, cylinder, axis, prism, and contact lens
/// attributes.
/// # Examples
///
/// ```
/// use fhir::r5::resources::vision_prescription::VisionPrescriptionLensSpecification;
/// use fhir::r5::types;
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
    pub eye: crate::r5::coded::Coded<crate::r5::codes::VisionEyeCodes>,
    /// Primitive extension sibling for [`eye`](Self::eye) (FHIR `_eye`).
    #[serde(rename = "_eye")]
    pub eye_ext: Option<types::Element>,

    /// Power of the lens
    pub sphere: Option<types::Decimal>,
    /// Primitive extension sibling for [`sphere`](Self::sphere) (FHIR `_sphere`).
    #[serde(rename = "_sphere")]
    pub sphere_ext: Option<types::Element>,

    /// Lens power for astigmatism
    pub cylinder: Option<types::Decimal>,
    /// Primitive extension sibling for [`cylinder`](Self::cylinder) (FHIR `_cylinder`).
    #[serde(rename = "_cylinder")]
    pub cylinder_ext: Option<types::Element>,

    /// Lens meridian which contain no power for astigmatism
    pub axis: Option<types::Integer>,
    /// Primitive extension sibling for [`axis`](Self::axis) (FHIR `_axis`).
    #[serde(rename = "_axis")]
    pub axis_ext: Option<types::Element>,

    /// Eye alignment compensation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prism: Vec<VisionPrescriptionLensSpecificationPrism>,

    /// Added power for multifocal levels
    pub add: Option<types::Decimal>,
    /// Primitive extension sibling for [`add`](Self::add) (FHIR `_add`).
    #[serde(rename = "_add")]
    pub add_ext: Option<types::Element>,

    /// Contact lens power
    pub power: Option<types::Decimal>,
    /// Primitive extension sibling for [`power`](Self::power) (FHIR `_power`).
    #[serde(rename = "_power")]
    pub power_ext: Option<types::Element>,

    /// Contact lens back curvature
    pub back_curve: Option<types::Decimal>,
    /// Primitive extension sibling for [`back_curve`](Self::back_curve) (FHIR `_backCurve`).
    #[serde(rename = "_backCurve")]
    pub back_curve_ext: Option<types::Element>,

    /// Contact lens diameter
    pub diameter: Option<types::Decimal>,
    /// Primitive extension sibling for [`diameter`](Self::diameter) (FHIR `_diameter`).
    #[serde(rename = "_diameter")]
    pub diameter_ext: Option<types::Element>,

    /// Lens wear duration
    pub duration: Option<types::Quantity>,

    /// Color required
    pub color: Option<types::String>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`).
    #[serde(rename = "_color")]
    pub color_ext: Option<types::Element>,

    /// Brand required
    pub brand: Option<types::String>,
    /// Primitive extension sibling for [`brand`](Self::brand) (FHIR `_brand`).
    #[serde(rename = "_brand")]
    pub brand_ext: Option<types::Element>,

    /// Notes for coatings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Eye alignment compensation.
///
/// Describes the amount and base direction of prism correction applied to a
/// lens to compensate for eye alignment.
/// # Examples
///
/// ```
/// use fhir::r5::resources::vision_prescription::VisionPrescriptionLensSpecificationPrism;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`amount`](Self::amount) (FHIR `_amount`).
    #[serde(rename = "_amount")]
    pub amount_ext: Option<types::Element>,

    /// up | down | in | out
    pub base: crate::r5::coded::Coded<crate::r5::codes::VisionBaseCodes>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`).
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,
}
