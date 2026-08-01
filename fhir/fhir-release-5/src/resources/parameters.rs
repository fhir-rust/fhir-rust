//! Parameters
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Parameters
//!
//! Version: 5.0.0
//!
//! Parameters Resource: This resource is used to pass information into and back from an operation.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Parameters resource is a general-purpose container used to pass
/// information into and back from an operation invoked on a FHIR server,
/// whether that operation is called directly through the RESTful API (for
/// example via the `$` operation syntax such as `$expand`, `$validate-code`,
/// or `$everything`) or exchanged within a messaging environment. Unlike most
/// other resources, Parameters is a transient, non-clinical envelope: it is not
/// persisted in a repository and cannot be referenced by other resources except
/// as described in the definition of the Parameters resource itself. This makes
/// it the standard mechanism for representing operation inputs and outputs that
/// do not correspond to a single concrete resource.
///
/// Each entry in the resource is a named parameter that carries exactly one of
/// three kinds of payload: a primitive or complex data-type value (the many
/// `value[x]` choices below), a complete nested resource, or a set of nested
/// `part` parameters that allow parameters to be grouped hierarchically. This
/// flexible structure lets a single Parameters instance model arbitrarily
/// complex operation arguments and results.
///
/// See also: the individual parameter entries are represented by
/// [`ParametersParameter`], and typed values commonly use core data types such
/// as [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Coding`](crate::r5::types::Coding),
/// [`Quantity`](crate::r5::types::Quantity), and
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::parameters::Parameters;
/// use fhir::r5::types;
///
/// let value = Parameters {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Parameters = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// The ordered set of named operation parameters carried by this resource, each supplying an input to or output from the operation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<ParametersParameter>,
}

/// A parameter passed to or received from the operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::parameters::ParametersParameter;
/// use fhir::r5::types;
///
/// let value = ParametersParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ParametersParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ParametersParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter name as defined by the operation definition; it identifies which formal parameter this entry supplies.
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The `Parameters.parameter.value[x]` choice element (0..1); see [`ParametersParameterValue`].
    #[serde(flatten)]
    pub value: Option<ParametersParameterValue>,

    /// Used when the parameter value is an entire nested FHIR resource rather than a single data-type value.
    pub resource: Option<::serde_json::Value>,

    /// Nested child parameters that let this parameter group several named values together as a multi-part structure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<ParametersParameter>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Parameters;

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
/// The `Parameters.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ParametersParameterValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r5::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r5::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r5::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueInteger64` variant.
    #[fhir("valueInteger64")]
    Integer64(crate::r5::choice::Primitive<types::Integer64>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r5::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r5::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `valueUuid` variant.
    #[fhir("valueUuid")]
    Uuid(crate::r5::choice::Primitive<types::Uuid>),
    /// `valueAddress` variant.
    #[fhir("valueAddress")]
    Address(Box<types::Address>),
    /// `valueAge` variant.
    #[fhir("valueAge")]
    Age(Box<types::Age>),
    /// `valueAnnotation` variant.
    #[fhir("valueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueCodeableReference` variant.
    #[fhir("valueCodeableReference")]
    CodeableReference(Box<types::CodeableReference>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueContactPoint` variant.
    #[fhir("valueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `valueCount` variant.
    #[fhir("valueCount")]
    Count(Box<types::Count>),
    /// `valueDistance` variant.
    #[fhir("valueDistance")]
    Distance(Box<types::Distance>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
    /// `valueHumanName` variant.
    #[fhir("valueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `valueIdentifier` variant.
    #[fhir("valueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `valueMoney` variant.
    #[fhir("valueMoney")]
    Money(Box<types::Money>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueRatioRange` variant.
    #[fhir("valueRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueSignature` variant.
    #[fhir("valueSignature")]
    Signature(Box<types::Signature>),
    /// `valueTiming` variant.
    #[fhir("valueTiming")]
    Timing(Box<types::Timing>),
    /// `valueContactDetail` variant.
    #[fhir("valueContactDetail")]
    ContactDetail(Box<types::ContactDetail>),
    /// `valueDataRequirement` variant.
    #[fhir("valueDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `valueExpression` variant.
    #[fhir("valueExpression")]
    Expression(Box<types::Expression>),
    /// `valueParameterDefinition` variant.
    #[fhir("valueParameterDefinition")]
    ParameterDefinition(Box<types::ParameterDefinition>),
    /// `valueRelatedArtifact` variant.
    #[fhir("valueRelatedArtifact")]
    RelatedArtifact(Box<types::RelatedArtifact>),
    /// `valueTriggerDefinition` variant.
    #[fhir("valueTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
    /// `valueUsageContext` variant.
    #[fhir("valueUsageContext")]
    UsageContext(Box<types::UsageContext>),
    /// `valueAvailability` variant.
    #[fhir("valueAvailability")]
    Availability(Box<types::Availability>),
    /// `valueExtendedContactDetail` variant.
    #[fhir("valueExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
    /// `valueDosage` variant.
    #[fhir("valueDosage")]
    Dosage(Box<types::Dosage>),
    /// `valueMeta` variant.
    #[fhir("valueMeta")]
    Meta(Box<types::Meta>),
}
