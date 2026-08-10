//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//!
//!
//! A digital Signature - XML DigSig, JWT, Graphical image of signature, etc.
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Signature Type
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::types::signature::Signature;
///
/// let value = Signature::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Signature = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SignatureDe")]
#[fhir_version("r3")]
pub struct Signature {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
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
    /// The `Signature.who[x]` choice element (1..1); see [`SignatureWho`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub who: Option<SignatureWho>,

    /// The party represented
    /// The `Signature.onBehalfOf[x]` choice element (0..1); see [`SignatureOnBehalfOf`].
    #[serde(flatten)]
    pub on_behalf_of: Option<SignatureOnBehalfOf>,

    /// The technical format of the signature
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// The actual signature content (XML DigSig. JWT, picture, etc.)
    pub blob: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`blob`](Self::blob) (FHIR `_blob`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_blob")]
    pub blob_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignatureDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    r#type: ::vec1::Vec1<types::Coding>,
    when: types::Instant,
    #[serde(rename = "_when")]
    when_ext: Option<types::Element>,
    #[serde(flatten)]
    who: crate::r3::choice::Slot<SignatureWho>,
    #[serde(flatten)]
    on_behalf_of: crate::r3::choice::Slot<SignatureOnBehalfOf>,
    content_type: Option<types::Code>,
    #[serde(rename = "_contentType")]
    content_type_ext: Option<types::Element>,
    blob: Option<types::Base64Binary>,
    #[serde(rename = "_blob")]
    blob_ext: Option<types::Element>,
}

impl ::core::convert::From<SignatureDe> for Signature {
    fn from(v: SignatureDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            r#type: v.r#type,
            when: v.when,
            when_ext: v.when_ext,
            who: v.who.0,
            on_behalf_of: v.on_behalf_of.0,
            content_type: v.content_type,
            content_type_ext: v.content_type_ext,
            blob: v.blob,
            blob_ext: v.blob_ext,
        }
    }
}

/// The `Signature.who[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum SignatureWho {
    /// `whoUri` variant.
    #[fhir("whoUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `whoReference` variant.
    #[fhir("whoReference")]
    Reference(Box<types::Reference>),
}

/// The `Signature.onBehalfOf[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum SignatureOnBehalfOf {
    /// `onBehalfOfUri` variant.
    #[fhir("onBehalfOfUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `onBehalfOfReference` variant.
    #[fhir("onBehalfOfReference")]
    Reference(Box<types::Reference>),
}
