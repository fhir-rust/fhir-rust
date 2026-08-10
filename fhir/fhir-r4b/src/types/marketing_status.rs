//! MarketingStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MarketingStatus
//!
//! Version: 4.3.0
//!
//! The marketing status describes the date when a medicinal product is
//! actually put on the market or the date as of which it is no longer
//! available
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MarketingStatus Type: The marketing status
/// describes the date when a medicinal product is actually put on the market
/// or the date as of which it is no longer available.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::marketing_status::MarketingStatus;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct MarketingStatus {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The country in which the marketing authorisation has been granted shall
    /// be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code
    /// elements
    pub country: Option<types::CodeableConcept>,

    /// Where a Medicines Regulatory Agency has granted a marketing
    /// authorisation for which specific provisions within a jurisdiction
    /// apply, the jurisdiction can be specified using an appropriate
    /// controlled terminology The controlled term and the controlled term
    /// identifier shall be specified
    pub jurisdiction: Option<types::CodeableConcept>,

    /// This attribute provides information on the status of the marketing of
    /// the medicinal product See ISO/TS 20443 for more information and
    /// examples
    pub status: types::CodeableConcept,

    /// The date when the Medicinal Product is placed on the market by the
    /// Marketing Authorisation Holder (or where applicable, the
    /// manufacturer/distributor) in a country and/or jurisdiction shall be
    /// provided A complete date consisting of day, month and year shall be
    /// specified using the ISO 8601 date format NOTE “Placed on the market”
    /// refers to the release of the Medicinal Product into the distribution
    /// chain
    pub date_range: Option<types::Period>,

    /// The date when the Medicinal Product is placed on the market by the
    /// Marketing Authorisation Holder (or where applicable, the
    /// manufacturer/distributor) in a country and/or jurisdiction shall be
    /// provided A complete date consisting of day, month and year shall be
    /// specified using the ISO 8601 date format NOTE “Placed on the market”
    /// refers to the release of the Medicinal Product into the distribution
    /// chain
    pub restore_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`restore_date`](Self::restore_date) (FHIR `_restoreDate`):
    /// carries `id` and/or `extension` for the primitive value.
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
