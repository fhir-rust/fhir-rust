//! UsageContext
//!
//! URL: http://hl7.org/fhir/StructureDefinition/UsageContext
//!
//! Version: 5.0.0
//!
//! UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// UsageContext specifies clinical, business, or other metadata that can be
/// used to retrieve, index, and/or categorize an artifact. The metadata may
/// describe the applicable population (e.g., age category, DRG) or the specific
/// context of care (e.g., venue, care setting, provider of care). Each
/// UsageContext pairs a `code` identifying the type of context with a value
/// that defines the context itself.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::usage_context::UsageContext;
/// use fhir::r5::types;
///
/// let value = UsageContext {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: UsageContext = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "UsageContextDe")]
pub struct UsageContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Type of context being specified
    pub code: types::Coding,

    /// The `UsageContext.value[x]` choice element (0..1); see [`UsageContextValue`].
    #[serde(flatten)]
    pub value: Option<UsageContextValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageContextDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    code: types::Coding,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<UsageContextValue>,
}

impl ::core::convert::From<UsageContextDe> for UsageContext {
    fn from(v: UsageContextDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            code: v.code,
            value: v.value.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = UsageContext;

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
/// The `UsageContext.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum UsageContextValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}
