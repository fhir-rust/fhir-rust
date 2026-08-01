//! Task
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Task
//!
//!
//!
//! A task to be performed
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Task Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::task::Task;
/// use fhir::r3::types;
///
/// let value = Task {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Task = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Task {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Task Instance Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Formal definition of task
    /// The `Task.definition[x]` choice element (0..1); see [`TaskDefinition`].
    #[serde(flatten)]
    pub definition: Option<TaskDefinition>,

    /// Request fulfilled by this task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Requisition or grouper id
    pub group_identifier: Option<types::Identifier>,

    /// Composite task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// draft | requested | received | accepted | +
    pub status: crate::coded::Coded<crate::r3::codes::TaskStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// E.g. "Specimen collected", "IV prepped"
    pub business_status: Option<types::CodeableConcept>,

    /// proposal | plan | order +
    pub intent: crate::coded::Coded<crate::r3::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// normal | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r3::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Task Type
    pub code: Option<types::CodeableConcept>,

    /// Human-readable explanation of task
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What task is acting on
    pub focus: Option<types::Reference>,

    /// Beneficiary of the Task
    pub r#for: Option<types::Reference>,

    /// Healthcare event during which this task originated
    pub context: Option<types::Reference>,

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
    pub requester: Option<TaskRequester>,

    /// requester | dispatcher | scheduler | performer | monitor | manager |
    /// acquirer | reviewer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer_type: Vec<types::CodeableConcept>,

    /// Responsible individual
    pub owner: Option<types::Reference>,

    /// Why task is needed
    pub reason: Option<types::CodeableConcept>,

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
/// use fhir::r3::resources::task::TaskInput;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct TaskInput {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
/// use fhir::r3::resources::task::TaskOutput;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct TaskOutput {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for output
    pub r#type: types::CodeableConcept,

    /// Result of output
    /// The `Task.output.value[x]` choice element (1..1); see [`TaskOutputValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<TaskOutputValue>,
}

/// The creator of the task.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::task::TaskRequester;
/// use fhir::r3::types;
///
/// let value = TaskRequester {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TaskRequester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TaskRequester {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Individual asking for task
    pub agent: types::Reference,

    /// Organization individual is acting for
    pub on_behalf_of: Option<types::Reference>,
}

/// If the Task.focus is a request resource and the task is seeking fulfillment
/// (i.e is asking for the request to be actioned), this element identifies any
/// limitations on what parts of the referenced request should be actioned.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::task::TaskRestriction;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct TaskRestriction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

/// The `Task.definition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum TaskDefinition {
    /// `definitionUri` variant.
    #[fhir("definitionUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `definitionReference` variant.
    #[fhir("definitionReference")]
    Reference(Box<types::Reference>),
}

/// The `Task.input.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum TaskInputValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r3::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r3::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r3::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r3::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r3::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r3::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r3::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
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
    /// `valueMeta` variant.
    #[fhir("valueMeta")]
    Meta(Box<types::Meta>),
}

/// The `Task.output.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum TaskOutputValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r3::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r3::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r3::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r3::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r3::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r3::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r3::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
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
