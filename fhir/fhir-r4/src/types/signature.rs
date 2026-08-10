//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//! Version: 4.0.1
//!
//! A Signature - XML DigSig, JWS, Graphical image of signature, etc.
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Signature Type: A signature along with
/// supporting context. The signature may be a digital signature that is
/// cryptographic in nature, or some other signature acceptable to the domain.
/// This other signature may be as simple as a graphical image representing a
/// hand-written signature, or a signature ceremony Different signature
/// approaches have different utilities.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::types::signature::Signature;
///
/// let value = Signature::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Signature = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Signature {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Indication of the reason the entity signed the object(s)
    pub r#type: ::vec1::Vec1<types::Coding>,

    /// When the signature was created
    pub when: types::Instant,
    /// Primitive extension sibling for [`when`](Self::when) (FHIR `_when`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_when")]
    pub when_ext: Option<types::Element>,

    /// Who signed
    pub who: types::Reference,

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

    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}
