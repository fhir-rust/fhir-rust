//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//! Version: 5.0.0
//!
//! Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This datatype captures a digital or other signature together with the metadata needed to interpret and verify it.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A signature along with supporting context, recorded to indicate that a resource or
/// a set of content has been endorsed or attested by a particular party. The signature
/// may be a cryptographic digital signature, or any other signature acceptable to the
/// domain, such as a graphical image of a hand-written signature or a signature
/// ceremony record. This type is typically used within `Provenance` resources or
/// attached to specific content to establish authorship and integrity.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::signature::Signature;
/// use fhir::r5::types;
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
pub struct Signature {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Indication of the reason the entity signed the object(s), e.g. author, coauthor.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::Coding>, // « SignatureTypeCodes? »

    /// When the signature was created.
    pub when: Option<types::Instant>,
    /// Primitive extension sibling for [`when`](Self::when) (FHIR `_when`).
    #[serde(rename = "_when")]
    pub when_ext: Option<types::Element>,

    /// Who signed the content, referencing the individual or system involved.
    pub who: Option<types::Reference>, // « Practitioner | PractitionerRole | RelatedPerson | Patient | Device | Organization »

    /// The party represented by the signer, when the signer acts on behalf of another.
    pub on_behalf_of: Option<types::Reference>, // « Practitioner | PractitionerRole | RelatedPerson | Patient | Device | Organization »

    /// The technical MIME type format of the signed content that was signed.
    pub target_format: Option<types::Code>, // « MimeTypes! »
    /// Primitive extension sibling for [`target_format`](Self::target_format) (FHIR `_targetFormat`).
    #[serde(rename = "_targetFormat")]
    pub target_format_ext: Option<types::Element>,

    /// The technical MIME type format of the signature data itself, e.g. an image or XML-DSig format.
    pub sig_format: Option<types::Code>, // « MimeTypes! »
    /// Primitive extension sibling for [`sig_format`](Self::sig_format) (FHIR `_sigFormat`).
    #[serde(rename = "_sigFormat")]
    pub sig_format_ext: Option<types::Element>,

    /// The base64-encoded actual signature content, e.g. XML DigSig or a JWT.
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`).
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Signature;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            r#type: vec![],
            when: None,
            who: None,
            on_behalf_of: None,
            target_format: None,
            sig_format: None,
            data: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "type": []
                }
            );
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
