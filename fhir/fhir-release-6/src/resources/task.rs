//! Task
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Task
//!
//! Version: 6.0.0-ballot3
//!
//! A task to be performed
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A task to be performed as a part of a workflow and the related information
/// like inputs, outputs and execution progress.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::task::Task;
/// use fhir::r6::types;
///
/// let value = Task {
///     instantiates_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instantiatesCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["instantiatesCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: Task = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Task {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Task Instance Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Formal definition of task
    pub instantiates_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<types::Element>,

    /// Formal definition of task
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Request fulfilled by this task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Composite task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// draft | requested | received | accepted | +
    pub status: crate::coded::Coded<crate::r6::codes::TaskStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableReference>,

    /// E.g. "Specimen collected", "IV prepped"
    pub business_status: Option<types::CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r6::codes::TaskIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if Task is prohibiting action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// Task Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of task
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What task is acting on
    pub focus: Option<types::Reference>,

    /// Beneficiary of the Task
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this task originated
    pub encounter: Option<types::Reference>,

    /// When the task should be performed
    pub requested_period: Option<types::Period>,

    /// Start and end time of execution
    pub execution_period: Option<types::Period>,

    /// Task Creation Date
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Task Last Modified Date
    pub last_modified: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_modified`](Self::last_modified) (FHIR `_lastModified`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastModified")]
    pub last_modified_ext: Option<types::Element>,

    /// Who is asking for task to be done
    pub requester: Option<types::Reference>,

    /// Who should perform the Task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_performer: Vec<types::CodeableReference>,

    /// Responsible individual
    pub owner: Option<types::Reference>,

    /// Who or what performed the task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<TaskPerformer>,

    /// Where task occurs
    pub location: Option<types::Reference>,

    /// Why task is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Comments made about the task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Key events in history of the Task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference>,

    /// Constraints on fulfillment tasks
    pub restriction: Option<TaskRestriction>,

    /// Information used to perform task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<TaskInput>,

    /// Information produced as part of task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<TaskOutput>,
}

/// Additional information that may be needed in the execution of the task.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::task::TaskInput;
/// use fhir::r6::types;
///
/// let value = TaskInput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TaskInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TaskInput {
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

    /// Content to use in performing the task
    /// The `Task.input.value[x]` choice element (1..1); see [`TaskInputValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<TaskInputValue>,
}

/// Outputs produced by the Task.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::task::TaskOutput;
/// use fhir::r6::types;
///
/// let value = TaskOutput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TaskOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TaskOutput {
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
    /// The `Task.output.value[x]` choice element (1..1); see [`TaskOutputValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<TaskOutputValue>,
}

/// The entity who performed the requested task.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::task::TaskPerformer;
/// use fhir::r6::types;
///
/// let value = TaskPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TaskPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TaskPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the task
    pub actor: types::Reference,
}

/// If the Task.focus is a request resource and the task is seeking fulfillment
/// (i.e. is asking for the request to be actioned), this element identifies
/// any limitations on what parts of the referenced request should be actioned.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::task::TaskRestriction;
/// use fhir::r6::types;
///
/// let value = TaskRestriction {
///     repetitions: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `repetitions` is the name this serializes to on the wire.
/// assert_eq!(json["repetitions"], ::serde_json::json!(1));
///
/// let back: TaskRestriction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TaskRestriction {
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

    /// When fulfillment is sought
    pub period: Option<types::Period>,

    /// For whom is fulfillment sought?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,
}

/// The `Task.input.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TaskInputValue {
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

/// The `Task.output.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TaskOutputValue {
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
    type T = Task;

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
