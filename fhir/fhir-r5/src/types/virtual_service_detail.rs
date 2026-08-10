//! VirtualServiceDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail
//!
//! Version: 5.0.0
//!
//! VirtualServiceDetail Type: Virtual Service Contact Details.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The VirtualServiceDetail datatype captures the contact details required to
/// connect to a virtual service, such as a video conference or telephone
/// conference. It describes the channel type (for example, a particular
/// teleconferencing product), the address or number used to reach the service,
/// and additional operational details like the maximum number of participants
/// and any session key needed to join. It is commonly used in scheduling,
/// appointment, and encounter resources to convey how a virtual meeting can be
/// accessed.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::virtual_service_detail::VirtualServiceDetail;
/// use fhir::r5::types;
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
pub struct VirtualServiceDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The type of virtual service to connect to (for example a specific vendor's platform)
    pub channel_type: Option<types::Coding>,

    /// The `VirtualServiceDetail.address[x]` choice element (0..1); see [`VirtualServiceDetailAddress`].
    #[serde(flatten)]
    pub address: Option<VirtualServiceDetailAddress>,

    /// Address(es) with additional information on alternate connection details
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub additional_info: ::fhir_core::PrimVec<types::Url>,
    /// Primitive extension sibling for [`additional_info`](Self::additional_info) (FHIR `_additionalInfo`).
    #[serde(rename = "_additionalInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info_ext: Vec<Option<types::Element>>,

    /// Maximum number of participants supported by the virtual service
    pub max_participants: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`max_participants`](Self::max_participants) (FHIR `_maxParticipants`).
    #[serde(rename = "_maxParticipants")]
    pub max_participants_ext: Option<types::Element>,

    /// Session Key required by the virtual service
    pub session_key: Option<types::String>,
    /// Primitive extension sibling for [`session_key`](Self::session_key) (FHIR `_sessionKey`).
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
    address: crate::r5::choice::Slot<VirtualServiceDetailAddress>,
    #[serde(default)]
    additional_info: ::fhir_core::PrimVec<types::Url>,
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
/// The `VirtualServiceDetail.address[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum VirtualServiceDetailAddress {
    /// `addressUrl` variant.
    #[fhir("addressUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `addressString` variant.
    #[fhir("addressString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `addressContactPoint` variant.
    #[fhir("addressContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `addressExtendedContactDetail` variant.
    #[fhir("addressExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
}
