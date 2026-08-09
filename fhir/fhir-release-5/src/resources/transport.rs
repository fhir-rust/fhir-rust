//! Transport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Transport
//!
//! Version: 5.0.0
//!
//! Transport Resource: Record of transport.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Transport
///
/// Record of transport of item. A Transport resource tracks the movement of an
/// item (such as a specimen, a device, or a patient) from one location to
/// another, capturing who requested it, its current status, the desired and
/// current locations, and any inputs and outputs associated with performing the
/// transport. It is used in FHIR R5 to coordinate and audit logistics workflows,
/// such as moving a specimen from a collection site to a laboratory, relocating
/// equipment between departments, or transporting a patient between care areas.
/// Transport is a "worker" resource similar in spirit to `Task`: it is created,
/// assigned to an owner, tracked through its lifecycle (from `in-progress` to
/// `completed`, `abandoned`, or `cancelled`), and may reference the request it
/// fulfills via `basedOn`, as well as capture the requester, the responsible
/// performer, and the reason the transport is needed.
///
/// # Related resources
///
/// The entity being moved, or the beneficiary of the transport, is commonly
/// referenced through `focus` or `for`, which may point at a
/// [`Patient`](crate::r5::resources::patient::Patient), a `Specimen`, or a
/// `Device`. Classification and status details are typically expressed using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values such as
/// `code`, `statusReason`, and `performerType`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::transport::Transport;
/// use fhir::r5::types;
///
/// let value = Transport {
///     instantiates_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instantiatesCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["instantiatesCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: Transport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
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

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Formal definition of transport
    pub instantiates_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<types::Element>,

    /// Formal definition of transport
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Request fulfilled by this transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r5::resources::Transport>>,

    /// Current lifecycle state: in-progress | completed | abandoned | cancelled | planned | entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::TransportStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: crate::r5::coded::Coded<crate::r5::codes::TransportIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Transport Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of transport
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The item, resource, or entity that the transport is moving or acting on
    pub focus: Option<types::Reference>,

    /// The individual for whose benefit the transport is being performed, such as a patient
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this transport originated
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// Completion time of the event (the occurrence)
    pub completion_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`completion_time`](Self::completion_time) (FHIR `_completionTime`).
    #[serde(rename = "_completionTime")]
    pub completion_time_ext: Option<types::Element>,

    /// Transport Creation Date
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Transport Last Modified Date
    pub last_modified: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_modified`](Self::last_modified) (FHIR `_lastModified`).
    #[serde(rename = "_lastModified")]
    pub last_modified_ext: Option<types::Element>,

    /// Who is asking for transport to be done
    pub requester: Option<types::Reference>,

    /// The kind of participant expected to carry out the transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_type: Vec<types::CodeableConcept>,

    /// The individual or organization responsible for carrying out the transport
    pub owner: Option<types::Reference>,

    /// The principal physical location where the transport is performed
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Comments made about the transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Key events in history of the Transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r5::resources::Provenance>>,

    /// Constraints on fulfillment transports
    pub restriction: Option<TransportRestriction>,

    /// Information used to perform transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<TransportInput>,

    /// Information produced as part of transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<TransportOutput>,

    /// The location to which the focus of the transport should be moved
    pub requested_location: types::Reference<crate::r5::resources::Location>,

    /// The current physical location of the focus of the transport
    pub current_location: types::Reference<crate::r5::resources::Location>,

    /// The clinical or administrative reason the transport is needed
    pub reason: Option<types::CodeableReference>,

    /// Parent (or preceding) transport
    pub history: Option<types::Reference<crate::r5::resources::Transport>>,
}

/// Transport.restriction
///
/// Constraints on fulfillment transports. If the Transport.focus is a request
/// resource and the transport is seeking fulfillment, this element identifies any
/// limitations on what parts of the referenced request should be actioned.
/// # Examples
///
/// ```
/// use fhir::r5::resources::transport::TransportRestriction;
/// use fhir::r5::types;
///
/// let value = TransportRestriction {
///     repetitions: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `repetitions` is the name this serializes to on the wire.
/// assert_eq!(json["repetitions"], ::serde_json::json!(1));
///
/// let back: TransportRestriction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportRestriction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How many times to repeat
    pub repetitions: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`repetitions`](Self::repetitions) (FHIR `_repetitions`).
    #[serde(rename = "_repetitions")]
    pub repetitions_ext: Option<types::Element>,

    /// When fulfillment sought
    pub period: Option<types::Period>,

    /// For whom is fulfillment sought?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,
}

/// Transport.input
///
/// Information used to perform transport. Additional information that may be
/// needed in the execution of the transport, expressed as a labelled, typed value.
/// # Examples
///
/// ```
/// use fhir::r5::resources::transport::TransportInput;
/// use fhir::r5::types;
///
/// let value = TransportInput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TransportInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for the input
    pub r#type: types::CodeableConcept,

    /// The `Transport.input.value[x]` choice element (0..1); see [`TransportInputValue`].
    #[serde(flatten)]
    pub value: Option<TransportInputValue>,
}

/// Transport.output
///
/// Information produced as part of transport. Outputs produced by the Transport,
/// expressed as labelled, typed values.
/// # Examples
///
/// ```
/// use fhir::r5::resources::transport::TransportOutput;
/// use fhir::r5::types;
///
/// let value = TransportOutput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TransportOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransportOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for output
    pub r#type: types::CodeableConcept,

    /// The `Transport.output.value[x]` choice element (0..1); see [`TransportOutputValue`].
    #[serde(flatten)]
    pub value: Option<TransportOutputValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Transport;

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
/// The `Transport.input.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TransportInputValue {
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

/// The `Transport.output.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TransportOutputValue {
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
