//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//! Version: 6.0.0-ballot3
//!
//! A Signature - XML DigSig, JWS, Graphical image of signature, etc.
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Signature Type: A signature along with supporting context. The signature
/// may be a digital signature that is cryptographic in nature, or some other
/// signature acceptable to the domain. This other signature may be as simple
/// as a graphical image representing a hand-written signature, or a signature
/// ceremony Different signature approaches have different utilities.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::signature::Signature;
/// use fhir::r6::types;
///
/// let value = Signature {
///     target_format: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `targetFormat` is the name this serializes to on the wire.
/// assert_eq!(json["targetFormat"], ::serde_json::json!("final"));
///
/// let back: Signature = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Signature {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Indication of the reason the entity signed the object(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::Coding>,

    /// When the signature was created
    pub when: Option<types::Instant>,
    /// Primitive extension sibling for [`when`](Self::when) (FHIR `_when`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_when")]
    pub when_ext: Option<types::Element>,

    /// Who signed
    pub who: Option<types::Reference>,

    /// The party represented
    pub on_behalf_of: Option<types::Reference>,

    /// The technical format of the signed resources
    pub target_format: Option<types::Code>,
    /// Primitive extension sibling for [`target_format`](Self::target_format) (FHIR `_targetFormat`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetFormat")]
    pub target_format_ext: Option<types::Element>,

    /// The technical format of the signature
    pub sig_format: Option<types::Code>,
    /// Primitive extension sibling for [`sig_format`](Self::sig_format) (FHIR `_sigFormat`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sigFormat")]
    pub sig_format_ext: Option<types::Element>,

    /// The actual signature content (XML Signature, JSON Jose, picture, etc.)
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Signature;

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
