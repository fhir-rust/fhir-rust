//! Medication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Medication
//!
//! Version: 5.0.0
//!
//! Medication Resource: This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Medication resource is primarily used for the identification and
/// definition of a medication, including its ingredients, for the purposes of
/// prescribing, dispensing, and administering a medicinal product, as well as
/// for making statements about medication use. In FHIR R5 it represents a
/// specific medication as an actual instance or as an abstract definition: it
/// carries the code that identifies the drug, its dose form (such as tablet,
/// capsule, or powder), its total volume, the active and inactive ingredients
/// together with the strength each contributes, and packaging details such as
/// the batch lot number and expiration date. A Medication may also cite the
/// organization authorized to market it and link to broader knowledge through a
/// definition reference.
///
/// Clinically and administratively, Medication acts as the shared, reusable
/// description of a drug that medication-workflow resources point to rather than
/// duplicate. It is referenced by resources such as `MedicationRequest`,
/// `MedicationDispense`, `MedicationAdministration`, and `MedicationStatement`,
/// which record the ordering, supply, giving, and reported use of the drug for a
/// particular subject.
///
/// # See also
///
/// The medication is described using data types including
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Quantity`](crate::r5::types::Quantity), and
/// [`Reference`](crate::r5::types::Reference). Ingredients and packaging are
/// modeled by the nested [`MedicationIngredient`] and [`MedicationBatch`] types.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication::Medication;
/// use fhir::r5::types;
///
/// let value = Medication {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Medication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Medication {
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

    /// Business identifier for this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Coded value, such as an RxNorm or SNOMED CT code, that identifies this medication
    pub code: Option<types::CodeableConcept>,

    /// Lifecycle status of the record: active, inactive, or entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::MedicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Organization that has authorization to market medication
    pub marketing_authorization_holder:
        Option<types::Reference<crate::r5::resources::Organization>>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// When the specified product code does not infer a package size, this is the specific amount of drug in the product
    pub total_volume: Option<types::Quantity>,

    /// Active or inactive ingredients that make up the medication, each with an optional strength
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationIngredient>,

    /// Details about the packaged medication, such as its batch lot number and expiration date
    pub batch: Option<MedicationBatch>,

    /// Knowledge about this medication
    pub definition: Option<types::Reference<crate::r5::resources::MedicationKnowledge>>,
}

/// Active or inactive ingredient contained in the medication, identifying the
/// substance or medication and, when known, the strength it contributes.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication::MedicationIngredient;
/// use fhir::r5::types;
///
/// let value = MedicationIngredient {
///     is_active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isActive` is the name this serializes to on the wire.
/// assert_eq!(json["isActive"], ::serde_json::json!(true));
///
/// let back: MedicationIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationIngredientDe")]
pub struct MedicationIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The ingredient (substance or medication) that the ingredient.strength relates to
    pub item: types::CodeableReference,

    /// Active ingredient indicator
    pub is_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_active`](Self::is_active) (FHIR `_isActive`).
    #[serde(rename = "_isActive")]
    pub is_active_ext: Option<types::Element>,

    /// The `Medication.ingredient.strength[x]` choice element (0..1); see [`MedicationIngredientStrength`].
    #[serde(flatten)]
    pub strength: Option<MedicationIngredientStrength>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationIngredientDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    item: types::CodeableReference,
    is_active: Option<types::Boolean>,
    #[serde(rename = "_isActive")]
    is_active_ext: Option<types::Element>,
    #[serde(flatten)]
    strength: crate::r5::choice::Slot<MedicationIngredientStrength>,
}

impl ::core::convert::From<MedicationIngredientDe> for MedicationIngredient {
    fn from(v: MedicationIngredientDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            item: v.item,
            is_active: v.is_active,
            is_active_ext: v.is_active_ext,
            strength: v.strength.0,
        }
    }
}

/// Details about the packaged medication, such as the lot number assigned to the
/// batch and the date on which it will expire.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication::MedicationBatch;
/// use fhir::r5::types;
///
/// let value = MedicationBatch {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: MedicationBatch = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationBatch {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier assigned to batch
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`).
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// When batch will expire
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`).
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Medication;

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
/// The `Medication.ingredient.strength[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationIngredientStrength {
    /// `strengthRatio` variant.
    #[fhir("strengthRatio")]
    Ratio(Box<types::Ratio>),
    /// `strengthCodeableConcept` variant.
    #[fhir("strengthCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `strengthQuantity` variant.
    #[fhir("strengthQuantity")]
    Quantity(Box<types::Quantity>),
}
