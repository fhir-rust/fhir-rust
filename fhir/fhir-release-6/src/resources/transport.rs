//! Transport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Transport
//!
//! Version: 6.0.0-ballot3
//!
//! Delivery of item
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Record of transport.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::transport::Transport;
/// use fhir::r6::types;
///
/// let value = Transport {
///     completion_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `completionTime` is the name this serializes to on the wire.
/// assert_eq!(json["completionTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Transport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Transport {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r6::resources::Resource>,

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
    pub instantiates: Option<types::Canonical>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiates")]
    pub instantiates_ext: Option<types::Element>,

    /// Request fulfilled by this transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r6::resources::Transport>>,

    /// preparation | in-progress | not-done | suspended | stopped | on-hold |
    /// completed | entered-in-error | unknown
    pub status: Option<crate::coded::Coded<crate::r6::codes::TransportStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Transport Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of transport
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What transport is acting on
    pub focus: Option<types::Reference>,

    /// Beneficiary of the Transport
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this transport originated
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Completion time of the event (the occurrence)
    pub completion_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`completion_time`](Self::completion_time) (FHIR `_completionTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_completionTime")]
    pub completion_time_ext: Option<types::Element>,

    /// Transport Creation Date
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Transport Last Modified Date
    pub last_modified: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_modified`](Self::last_modified) (FHIR `_lastModified`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastModified")]
    pub last_modified_ext: Option<types::Element>,

    /// Who is asking for transport to be done
    pub requester: Option<types::Reference>,

    /// Requested performer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_type: Vec<types::CodeableConcept>,

    /// Responsible individual
    pub owner: Option<types::Reference>,

    /// Where transport occurs
    pub location: Option<types::Reference<crate::r6::resources::Location>>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Comments made about the transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Key events in history of the Transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r6::resources::Provenance>>,

    /// Constraints on fulfillment transports
    pub restriction: Option<TransportRestriction>,

    /// Information used to perform transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<TransportInput>,

    /// Information produced as part of transport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<TransportOutput>,

    /// The desired location
    pub requested_location: types::Reference<crate::r6::resources::Location>,

    /// The entity current location
    pub current_location: types::Reference<crate::r6::resources::Location>,

    /// Why transport is needed
    pub reason: Option<types::CodeableReference>,

    /// Parent (or preceding) transport
    pub history: Option<types::Reference<crate::r6::resources::Transport>>,
}

/// Additional information that may be needed in the execution of the
/// transport.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::transport::TransportInput;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Content to use in performing the transport
    /// The `Transport.input.value[x]` choice element (1..1); see [`TransportInputValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<TransportInputValue>,
}

/// Outputs produced by the Transport.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::transport::TransportOutput;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Result of output
    /// The `Transport.output.value[x]` choice element (1..1); see [`TransportOutputValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<TransportOutputValue>,
}

/// If the Transport.focus is a request resource and the transport is seeking
/// fulfillment (i.e. is asking for the request to be actioned), this element
/// identifies any limitations on what parts of the referenced request should
/// be actioned.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::transport::TransportRestriction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`repetitions`](Self::repetitions) (FHIR `_repetitions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_repetitions")]
    pub repetitions_ext: Option<types::Element>,

    /// When fulfillment sought
    pub period: Option<types::Period>,

    /// For whom is fulfillment sought?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,
}

/// The `Transport.input.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TransportInputValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueInteger64` variant.
    #[fhir("valueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `valueUuid` variant.
    #[fhir("valueUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
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

/// The `Transport.output.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TransportOutputValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueInteger64` variant.
    #[fhir("valueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `valueUuid` variant.
    #[fhir("valueUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
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
