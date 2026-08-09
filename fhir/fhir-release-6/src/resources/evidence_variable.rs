//! EvidenceVariable
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
//!
//! Version: 6.0.0-ballot3
//!
//! A definition of an exposure, outcome, or other variable
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The EvidenceVariable resource describes an element that knowledge
/// (Evidence) is about.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariable;
/// use fhir::r6::types;
///
/// let value = EvidenceVariable {
///     short_title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `shortTitle` is the name this serializes to on the wire.
/// assert_eq!(json["shortTitle"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariable {
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

    /// Canonical identifier for this evidence variable, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the evidence variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the evidence variable
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `EvidenceVariable.versionAlgorithm[x]` choice element (0..1); see [`EvidenceVariableVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<EvidenceVariableVersionAlgorithm>,

    /// Name for this evidence variable (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this evidence variable (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Title for use in informal contexts
    pub short_title: Option<types::String>,
    /// Primitive extension sibling for [`short_title`](Self::short_title) (FHIR `_shortTitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_shortTitle")]
    pub short_title_ext: Option<types::Element>,

    /// Display of how to cite this EvidenceVariable
    pub cite_as: Option<types::Markdown>,
    /// Primitive extension sibling for [`cite_as`](Self::cite_as) (FHIR `_citeAs`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citeAs")]
    pub cite_as_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Who entered the data for the evidence variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recorder: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Natural language description of the evidence variable
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Why this EvidenceVariable is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Intellectual property ownership, may include restrictions on use
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the resource was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the resource was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the resource is expected to be used
    pub effective_period: Option<types::Period>,

    /// Relationships to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<EvidenceVariableRelatesTo>,

    /// Actual or conceptual
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// The meaning of the evidence variable
    pub definition: Option<types::CodeableReference>,

    /// Further specification of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_modifier: Vec<EvidenceVariableDefinitionModifier>,

    /// boolean | continuous | dichotomous | ordinal | polychotomous |
    /// extension
    pub handling: Option<crate::coded::Coded<crate::r6::codes::VariableHandling>>,
    /// Primitive extension sibling for [`handling`](Self::handling) (FHIR `_handling`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_handling")]
    pub handling_ext: Option<types::Element>,

    /// A grouping for dichotomous, ordinal, or polychotomouos variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<EvidenceVariableCategory>,

    /// Condition determining whether the data will be collected
    pub conditional: Option<types::Expression>,

    /// Classification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// How the data element (value of the variable) is found
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_storage: Vec<EvidenceVariableDataStorage>,

    /// When the variable is observed
    pub timing: Option<types::RelativeTime>,

    /// Calendar-based timing when the variable is observed
    pub period: Option<types::Period>,

    /// Limit on acceptability of data value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint: Vec<EvidenceVariableConstraint>,

    /// How missing data can be interpreted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_data_meaning: Vec<types::CodeableConcept>,

    /// How erroneous data is processed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unacceptable_data_handling: Vec<types::CodeableConcept>,
}

/// A grouping for dichotomous, ordinal, or polychotomouos variables.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariableCategory;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableCategory {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableCategory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableCategory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of the grouping
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Definition of the grouping
    /// The `EvidenceVariable.category.value[x]` choice element (0..1); see [`EvidenceVariableCategoryValue`].
    #[serde(flatten)]
    pub value: Option<EvidenceVariableCategoryValue>,
}

/// Limit on acceptability of data used to express values of the variable.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariableConstraint;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableConstraint {
///     earliest_date_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `earliestDateTime` is the name this serializes to on the wire.
/// assert_eq!(json["earliestDateTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: EvidenceVariableConstraint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableConstraint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Condition determining whether this constraint applies
    pub conditional: Option<types::CodeableConcept>,

    /// The lowest permissible value of the variable
    pub minimum_quantity: Option<types::Quantity>,

    /// The highest permissible value of the variable
    pub maximum_quantity: Option<types::Quantity>,

    /// The earliest permissible value of the variable
    pub earliest_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`earliest_date_time`](Self::earliest_date_time) (FHIR `_earliestDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_earliestDateTime")]
    pub earliest_date_time_ext: Option<types::Element>,

    /// The latest permissible value of the variable
    pub latest_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`latest_date_time`](Self::latest_date_time) (FHIR `_latestDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_latestDateTime")]
    pub latest_date_time_ext: Option<types::Element>,

    /// The lowest number of characters allowed for a value of the variable
    pub minimum_string_length: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`minimum_string_length`](Self::minimum_string_length) (FHIR `_minimumStringLength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_minimumStringLength")]
    pub minimum_string_length_ext: Option<types::Element>,

    /// The highest number of characters allowed for a value of the variable
    pub maximum_string_length: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`maximum_string_length`](Self::maximum_string_length) (FHIR `_maximumStringLength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maximumStringLength")]
    pub maximum_string_length_ext: Option<types::Element>,

    /// Rule for acceptable data values
    pub code: Option<types::CodeableConcept>,

    /// Rule for acceptable data values, as an Expression
    pub expression: Option<types::Expression>,

    /// List of anticipated values used to express value of the variable
    pub expected_value_set: Option<types::Reference<crate::r6::resources::ValueSet>>,

    /// List of anticipated values used to express units for the value of the
    /// variable
    pub expected_units_value_set: Option<types::Reference<crate::r6::resources::ValueSet>>,

    /// Permissibility of unanticipated value used to express value of the
    /// variable
    pub any_value_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`any_value_allowed`](Self::any_value_allowed) (FHIR `_anyValueAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_anyValueAllowed")]
    pub any_value_allowed_ext: Option<types::Element>,
}

/// How the data element is organized and where the data element (expressing
/// the value of the variable) is found in the dataset.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariableDataStorage;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableDataStorage {
///     path: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `path` is the name this serializes to on the wire.
/// assert_eq!(json["path"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableDataStorage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableDataStorage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of data used to express value of the variable
    pub datatype: Option<types::CodeableConcept>,

    /// Where to find the data element in the dataset
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Character(s) separating values in a string-based list
    pub delimiter: Option<types::String>,
    /// Primitive extension sibling for [`delimiter`](Self::delimiter) (FHIR `_delimiter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_delimiter")]
    pub delimiter_ext: Option<types::Element>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<EvidenceVariableDataStorage>,
}

/// Further specification of the definition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariableDefinitionModifier;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableDefinitionModifier {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableDefinitionModifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableDefinitionModifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Attribute of the definition
    pub code: types::CodeableConcept,

    /// Specification of the definition attribute
    /// The `EvidenceVariable.definitionModifier.value[x]` choice element (1..1); see [`EvidenceVariableDefinitionModifierValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<EvidenceVariableDefinitionModifierValue>,
}

/// Relationships that this EvidenceVariable has with other FHIR or non-FHIR
/// resources that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence_variable::EvidenceVariableRelatesTo;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// documentation | justification | predecessor | successor | derived-from
    /// | depends-on | composed-of | part-of | amends | amended-with | appends
    /// | appended-with | cites | cited-by | comments-on | comment-in |
    /// contains | contained-in | corrects | correction-in | replaces |
    /// replaced-with | retracts | retracted-by | signs | similar-to | supports
    /// | supported-with | transforms | transformed-into | transformed-with |
    /// specification-of | created-with | cite-as | summarizes
    pub r#type: crate::coded::Coded<crate::r6::codes::ArtifactRelationshipType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The artifact that is related to this EvidenceVariable
    /// The `EvidenceVariable.relatesTo.target[x]` choice element (1..1); see [`EvidenceVariableRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<EvidenceVariableRelatesToTarget>,
}

/// The `EvidenceVariable.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `EvidenceVariable.category.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCategoryValue {
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

/// The `EvidenceVariable.definitionModifier.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableDefinitionModifierValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueRelativeTime` variant.
    #[fhir("valueRelativeTime")]
    RelativeTime(Box<types::RelativeTime>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueExpression` variant.
    #[fhir("valueExpression")]
    Expression(Box<types::Expression>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `EvidenceVariable.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableRelatesToTarget {
    /// `targetUri` variant.
    #[fhir("targetUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `targetAttachment` variant.
    #[fhir("targetAttachment")]
    Attachment(Box<types::Attachment>),
    /// `targetCanonical` variant.
    #[fhir("targetCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
    /// `targetMarkdown` variant.
    #[fhir("targetMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EvidenceVariable;

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
