//! MarketingStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MarketingStatus
//!
//! Version: 6.0.0-ballot3
//!
//! The marketing status describes the date when an item is actually put on the
//! market or the date as of which it is no longer available
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// MarketingStatus Type: The marketing status describes the date when an item
/// is actually put on the market or the date as of which it is no longer
/// available.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::marketing_status::MarketingStatus;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct MarketingStatus {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The country in which the marketing status applies
    pub country: Option<types::CodeableConcept>,

    /// The jurisdiction in which the marketing status applies
    pub jurisdiction: Option<types::CodeableConcept>,

    /// This attribute provides information on the status of the marketing of
    /// the item
    pub status: types::CodeableConcept,

    /// The dates that the item is made available on the market by the owner
    /// (or where applicable, the manufacturer/distributor) in a country and/or
    /// jurisdiction. Note that “on the market” refers to the release of the
    /// item into the distribution chain
    pub date_range: Option<types::Period>,

    /// The date when the item is due to be placed back on the market by the
    /// owner, manufacturer or distributor, after a suspension
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
