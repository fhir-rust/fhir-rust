//! Extension
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Extension
//!
//!
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Extension Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::extension::Extension;
/// use fhir::r2::types;
///
/// let value = Extension {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: Extension = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Extension {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// identifies the meaning of the extension
    pub url: types::Uri,

    /// Value of extension
    /// The `Extension.value[x]` choice element (0..1); see [`ExtensionValue`].
    #[serde(flatten)]
    pub value: Option<ExtensionValue>,
}

/// The `Extension.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ExtensionValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `valueAnnotation` variant.
    #[fhir("valueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueIdentifier` variant.
    #[fhir("valueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueSignature` variant.
    #[fhir("valueSignature")]
    Signature(Box<types::Signature>),
    /// `valueHumanName` variant.
    #[fhir("valueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `valueAddress` variant.
    #[fhir("valueAddress")]
    Address(Box<types::Address>),
    /// `valueContactPoint` variant.
    #[fhir("valueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `valueTiming` variant.
    #[fhir("valueTiming")]
    Timing(Box<types::Timing>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueMeta` variant.
    #[fhir("valueMeta")]
    Meta(Box<types::Meta>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Extension;

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
