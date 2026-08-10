//! MarketingStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MarketingStatus
//!
//! Version: 5.0.0
//!
//! MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The marketing status describes the date when a medicinal product is actually
/// put on the market or the date as of which it is no longer available. It is a
/// complex datatype used in medicinal product resources to capture regulatory
/// marketing information such as the country and jurisdiction of authorization,
/// the current marketing status, and the relevant date range. This type is
/// primarily used in the regulated medicinal product domain of FHIR R5.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::marketing_status::MarketingStatus;
/// use fhir::r5::types;
///
/// let value = MarketingStatus {
///     restore_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `restoreDate` is the name this serializes to on the wire.
/// assert_eq!(json["restoreDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MarketingStatus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MarketingStatus {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The country in which the marketing authorization has been granted shall be
    /// specified. It should be specified using the ISO 3166-1 alpha-2 code elements.
    pub country: Option<types::CodeableConcept>,

    /// Where a Medicines Regulatory Agency has granted a marketing authorization
    /// for which specific provisions within a jurisdiction apply, the jurisdiction
    /// can be specified using an appropriate controlled terminology.
    pub jurisdiction: Option<types::CodeableConcept>,

    /// This attribute provides information on the status of the marketing of the
    /// medicinal product. See ISO/TS 20443 for more information and examples.
    pub status: types::CodeableConcept,

    /// The date range during which this marketing status applies, from when
    /// the product was placed on the market to when it ceased to be marketed, if applicable.
    pub date_range: Option<types::Period>,

    /// The date when a temporarily withdrawn medicinal product's marketing
    /// authorization and status are due to be restored.
    pub restore_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`restore_date`](Self::restore_date) (FHIR `_restoreDate`).
    #[serde(rename = "_restoreDate")]
    pub restore_date_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MarketingStatus;

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
