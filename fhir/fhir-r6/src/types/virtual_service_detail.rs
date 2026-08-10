//! VirtualServiceDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail
//!
//! Version: 6.0.0-ballot3
//!
//! Virtual Service Contact Details
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// VirtualServiceDetail Type: The set of values required to describe a virtual
/// service's connection details, including some limitations of the service.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::virtual_service_detail::VirtualServiceDetail;
/// use fhir::r6::types;
///
/// let value = VirtualServiceDetail {
///     max_participants: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `maxParticipants` is the name this serializes to on the wire.
/// assert_eq!(json["maxParticipants"], ::serde_json::json!(1));
///
/// let back: VirtualServiceDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "VirtualServiceDetailDe")]
#[fhir_version("r6")]
pub struct VirtualServiceDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Channel Type
    pub channel_type: Option<types::Coding>,

    /// Contact address/number
    /// The `VirtualServiceDetail.address[x]` choice element (0..1); see [`VirtualServiceDetailAddress`].
    #[serde(flatten)]
    pub address: Option<VirtualServiceDetailAddress>,

    /// Web address to see alternative connection details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<types::Url>,
    /// Primitive extension sibling for [`additional_info`](Self::additional_info) (FHIR `_additionalInfo`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_additionalInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info_ext: Vec<Option<types::Element>>,

    /// Maximum number of participants supported by the virtual service
    pub max_participants: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`max_participants`](Self::max_participants) (FHIR `_maxParticipants`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maxParticipants")]
    pub max_participants_ext: Option<types::Element>,

    /// Session Key required by the virtual service
    pub session_key: Option<types::String>,
    /// Primitive extension sibling for [`session_key`](Self::session_key) (FHIR `_sessionKey`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sessionKey")]
    pub session_key_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VirtualServiceDetailDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    channel_type: Option<types::Coding>,
    #[serde(flatten)]
    address: crate::r6::choice::Slot<VirtualServiceDetailAddress>,
    #[serde(default)]
    additional_info: Vec<types::Url>,
    #[serde(rename = "_additionalInfo")]
    #[serde(default)]
    additional_info_ext: Vec<Option<types::Element>>,
    max_participants: Option<types::PositiveInt>,
    #[serde(rename = "_maxParticipants")]
    max_participants_ext: Option<types::Element>,
    session_key: Option<types::String>,
    #[serde(rename = "_sessionKey")]
    session_key_ext: Option<types::Element>,
}

impl ::core::convert::From<VirtualServiceDetailDe> for VirtualServiceDetail {
    fn from(v: VirtualServiceDetailDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            channel_type: v.channel_type,
            address: v.address.0,
            additional_info: v.additional_info,
            additional_info_ext: v.additional_info_ext,
            max_participants: v.max_participants,
            max_participants_ext: v.max_participants_ext,
            session_key: v.session_key,
            session_key_ext: v.session_key_ext,
        }
    }
}

/// The `VirtualServiceDetail.address[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum VirtualServiceDetailAddress {
    /// `addressUrl` variant.
    #[fhir("addressUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `addressString` variant.
    #[fhir("addressString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `addressContactPoint` variant.
    #[fhir("addressContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `addressExtendedContactDetail` variant.
    #[fhir("addressExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VirtualServiceDetail;

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
