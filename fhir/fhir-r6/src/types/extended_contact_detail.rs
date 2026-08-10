//! ExtendedContactDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail
//!
//! Version: 6.0.0-ballot3
//!
//! Contact information
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// ExtendedContactDetail Type: Specifies contact information for a specific
/// purpose over a period of time, might be handled/monitored by a specific
/// named person or organization.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::extended_contact_detail::ExtendedContactDetail;
/// use fhir::r6::types;
///
/// let value = ExtendedContactDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExtendedContactDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExtendedContactDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The type of contact
    pub purpose: Option<types::CodeableConcept>,

    /// Name of an individual to contact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// Contact details (e.g.phone/fax/url)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Address for the contact
    pub address: Option<types::Address>,

    /// This contact detail is handled/monitored by a specific organization
    pub organization: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Period that this contact was valid for usage
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExtendedContactDetail;

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
