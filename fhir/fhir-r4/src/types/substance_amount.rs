//! SubstanceAmount
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceAmount
//!
//! Version: 4.0.1
//!
//! Chemical substances are a single substance type whose primary defining
//! element is the molecular structure. Chemical substances shall be defined on
//! the basis of their complete covalent molecular structure; the presence of a
//! salt (counter-ion) and/or solvates (water, alcohols) is also captured.
//! Purity, grade, physical form or particle size are not taken into account in
//! the definition of a chemical substance or in the assignment of a Substance
//! ID
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for SubstanceAmount Type: Chemical substances are
/// a single substance type whose primary defining element is the molecular
/// structure. Chemical substances shall be defined on the basis of their
/// complete covalent molecular structure; the presence of a salt (counter-ion)
/// and/or solvates (water, alcohols) is also captured. Purity, grade, physical
/// form or particle size are not taken into account in the definition of a
/// chemical substance or in the assignment of a Substance ID.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::substance_amount::SubstanceAmount;
/// use fhir::r4::types;
///
/// let value = SubstanceAmount {
///     amount_text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `amountText` is the name this serializes to on the wire.
/// assert_eq!(json["amountText"], ::serde_json::json!("abc"));
///
/// let back: SubstanceAmount = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SubstanceAmountDe")]
#[fhir_version("r4")]
pub struct SubstanceAmount {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Used to capture quantitative values for a variety of elements. If only
    /// limits are given, the arithmetic mean would be the average. If only a
    /// single definite value for a given element is given, it would be
    /// captured in this field
    /// The `SubstanceAmount.amount[x]` choice element (0..1); see [`SubstanceAmountAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceAmountAmount>,

    /// Most elements that require a quantitative value will also have a field
    /// called amount type. Amount type should always be specified because the
    /// actual value of the amount is often dependent on it. EXAMPLE: In
    /// capturing the actual relative amounts of substances or molecular
    /// fragments it is essential to indicate whether the amount refers to a
    /// mole ratio or weight ratio. For any given element an effort should be
    /// made to use same the amount type for all related definitional elements
    pub amount_type: Option<types::CodeableConcept>,

    /// A textual comment on a numeric value
    pub amount_text: Option<types::String>,
    /// Primitive extension sibling for [`amount_text`](Self::amount_text) (FHIR `_amountText`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_amountText")]
    pub amount_text_ext: Option<types::Element>,

    /// Reference range of possible or expected values
    pub reference_range: Option<SubstanceAmountReferenceRange>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceAmountDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    amount: crate::r4::choice::Slot<SubstanceAmountAmount>,
    amount_type: Option<types::CodeableConcept>,
    amount_text: Option<types::String>,
    #[serde(rename = "_amountText")]
    amount_text_ext: Option<types::Element>,
    reference_range: Option<SubstanceAmountReferenceRange>,
}

impl ::core::convert::From<SubstanceAmountDe> for SubstanceAmount {
    fn from(v: SubstanceAmountDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            amount: v.amount.0,
            amount_type: v.amount_type,
            amount_text: v.amount_text,
            amount_text_ext: v.amount_text_ext,
            reference_range: v.reference_range,
        }
    }
}

/// Reference range of possible or expected values.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::substance_amount::SubstanceAmountReferenceRange;
/// use fhir::r4::types;
///
/// let value = SubstanceAmountReferenceRange {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceAmountReferenceRange = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceAmountReferenceRange {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Lower limit possible or expected
    pub low_limit: Option<types::Quantity>,

    /// Upper limit possible or expected
    pub high_limit: Option<types::Quantity>,
}

/// The `SubstanceAmount.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceAmountAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountRange` variant.
    #[fhir("amountRange")]
    Range(Box<types::Range>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r4::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceAmount;

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
