//! Medication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Medication
//!
//! Version: 6.0.0-ballot3
//!
//! Definition of a Medication
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource is primarily used for the identification and definition of a
/// medication, including ingredients, for the purposes of prescribing,
/// dispensing, and administering a medication as well as for making statements
/// about medication use.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication::Medication;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Medication {
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

    /// Business identifier for this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Codes that identify this medication
    pub code: Option<types::CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r6::codes::MedicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Organization that has authorization to market medication
    pub marketing_authorization_holder:
        Option<types::Reference<crate::r6::resources::Organization>>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// When the specified product code does not infer a package size, this is
    /// the specific amount of drug in the product
    pub total_volume: Option<types::Quantity>,

    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationIngredient>,

    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,

    /// Knowledge about this medication
    pub definition: Option<types::Reference<crate::r6::resources::MedicationKnowledge>>,
}

/// Information that only applies to packages (not products).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication::MedicationBatch;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// When batch will expire
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,
}

/// Identifies a particular constituent of interest in the product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication::MedicationIngredient;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct MedicationIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The ingredient (substance or medication) that the ingredient.strength
    /// relates to
    pub item: types::CodeableReference,

    /// Active ingredient indicator
    pub is_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_active`](Self::is_active) (FHIR `_isActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isActive")]
    pub is_active_ext: Option<types::Element>,

    /// Quantity of ingredient present
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
    strength: crate::r6::choice::Slot<MedicationIngredientStrength>,
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

/// The `Medication.ingredient.strength[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
