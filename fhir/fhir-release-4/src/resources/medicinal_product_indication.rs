//! MedicinalProductIndication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIndication
//!
//! Version: 4.0.1
//!
//! MedicinalProductIndication
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Indication for the Medicinal Product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_indication::MedicinalProductIndication;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIndication {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIndication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIndication {
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

    /// The medication for which this is an indication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// The disease, symptom or procedure that is the indication for treatment
    pub disease_symptom_procedure: Option<types::CodeableConcept>,

    /// The status of the disease or symptom for which the indication applies
    pub disease_status: Option<types::CodeableConcept>,

    /// Comorbidity (concurrent condition) or co-infection as part of the
    /// indication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comorbidity: Vec<types::CodeableConcept>,

    /// The intended effect, aim or strategy to be achieved by the indication
    pub intended_effect: Option<types::CodeableConcept>,

    /// Timing or duration information as part of the indication
    pub duration: Option<types::Quantity>,

    /// Information about the use of the medicinal product in relation to other
    /// therapies described as part of the indication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_therapy: Vec<MedicinalProductIndicationOtherTherapy>,

    /// Describe the undesirable effects of the medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub undesirable_effect: Vec<types::Reference>,

    /// The population group to which this applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<types::Population>,
}

/// Information about the use of the medicinal product in relation to other
/// therapies described as part of the indication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_indication::MedicinalProductIndicationOtherTherapy;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIndicationOtherTherapy {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIndicationOtherTherapy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIndicationOtherTherapy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of relationship between the medicinal product indication or
    /// contraindication and another therapy
    pub therapy_relationship_type: types::CodeableConcept,

    /// Reference to a specific medication (active substance, medicinal product
    /// or class of products) as part of an indication or contraindication
    /// The `MedicinalProductIndication.otherTherapy.medication[x]` choice element (1..1); see [`MedicinalProductIndicationOtherTherapyMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicinalProductIndicationOtherTherapyMedication>,
}

/// The `MedicinalProductIndication.otherTherapy.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicinalProductIndicationOtherTherapyMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicinalProductIndication;

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
