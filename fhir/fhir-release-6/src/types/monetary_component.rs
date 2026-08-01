//! MonetaryComponent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MonetaryComponent
//!
//! Version: 6.0.0-ballot3
//!
//! Types and value of financial information that apply to line item(s)
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// MonetaryComponent Type: Financial line items use this datatype to commonly
/// categorize the value, and other factors that may effect how the value
/// should be interpreted.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::monetary_component::MonetaryComponent;
/// use fhir::r6::types;
///
/// let value = MonetaryComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MonetaryComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MonetaryComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// base | surcharge | discount | tax | informational
    pub r#type: crate::coded::Coded<crate::r6::codes::PriceComponentType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Codes may be used to differentiate between kinds of taxes, surcharges,
    /// discounts etc.
    pub code: Option<types::CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Explicit value amount to be used
    pub amount: Option<types::Money>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MonetaryComponent;

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
