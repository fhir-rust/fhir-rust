//! Evidence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Evidence
//!
//! Version: 6.0.0-ballot3
//!
//! Single evidence bit
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population,
/// exposures/interventions, comparators, outcomes, measured variables,
/// confounding variables), the statistics, and the certainty of this evidence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::Evidence;
/// use fhir::r6::types;
///
/// let value = Evidence {
///     cite_as: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `citeAs` is the name this serializes to on the wire.
/// assert_eq!(json["citeAs"], ::serde_json::json!("# Heading"));
///
/// let back: Evidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "EvidenceDe")]
#[fhir_version("r6")]
pub struct Evidence {
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

    /// Canonical identifier for this evidence, represented as a globally
    /// unique URI
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the summary
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of this summary
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `Evidence.versionAlgorithm[x]` choice element (0..1); see [`EvidenceVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<EvidenceVersionAlgorithm>,

    /// Name for this summary (machine friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this summary (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Display of how to cite this Evidence
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

    /// When the summary was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the summary was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

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

    /// Who entered the data for the evidence
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

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Why this Evidence is defined
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

    /// Relationships to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<EvidenceRelatesTo>,

    /// Description of the particular summary
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Declarative description of the Evidence
    pub assertion: Option<types::Markdown>,
    /// Primitive extension sibling for [`assertion`](Self::assertion) (FHIR `_assertion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assertion")]
    pub assertion_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Description, classification, and definition of a single variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable_definition: Vec<EvidenceVariableDefinition>,

    /// The design of the synthesis (combination of studies) that produced this
    /// evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synthesis_type: Vec<types::CodeableConcept>,

    /// The design of the study that produced this evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study_design: Vec<types::CodeableConcept>,

    /// Values and parameters for a single statistic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statistic: Vec<EvidenceStatistic>,

    /// Certainty or quality of the evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<EvidenceCertainty>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r6::choice::Slot<EvidenceVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    cite_as: Option<types::Markdown>,
    #[serde(rename = "_citeAs")]
    cite_as_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    #[serde(default)]
    author: Vec<types::ContactDetail>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    #[serde(default)]
    recorder: Vec<types::ContactDetail>,
    #[serde(default)]
    editor: Vec<types::ContactDetail>,
    #[serde(default)]
    reviewer: Vec<types::ContactDetail>,
    #[serde(default)]
    endorser: Vec<types::ContactDetail>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    #[serde(default)]
    relates_to: Vec<EvidenceRelatesTo>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    assertion: Option<types::Markdown>,
    #[serde(rename = "_assertion")]
    assertion_ext: Option<types::Element>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    variable_definition: Vec<EvidenceVariableDefinition>,
    #[serde(default)]
    synthesis_type: Vec<types::CodeableConcept>,
    #[serde(default)]
    study_design: Vec<types::CodeableConcept>,
    #[serde(default)]
    statistic: Vec<EvidenceStatistic>,
    #[serde(default)]
    certainty: Vec<EvidenceCertainty>,
}

impl ::core::convert::From<EvidenceDe> for Evidence {
    fn from(v: EvidenceDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            cite_as: v.cite_as,
            cite_as_ext: v.cite_as_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            author: v.author,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            recorder: v.recorder,
            editor: v.editor,
            reviewer: v.reviewer,
            endorser: v.endorser,
            use_context: v.use_context,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            relates_to: v.relates_to,
            description: v.description,
            description_ext: v.description_ext,
            assertion: v.assertion,
            assertion_ext: v.assertion_ext,
            note: v.note,
            variable_definition: v.variable_definition,
            synthesis_type: v.synthesis_type,
            study_design: v.study_design,
            statistic: v.statistic,
            certainty: v.certainty,
        }
    }
}

/// Assessment of certainty, confidence in the estimates, or quality of the
/// evidence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceCertainty;
/// use fhir::r6::types;
///
/// let value = EvidenceCertainty {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: EvidenceCertainty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of certainty
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Aspect of certainty being rated
    pub r#type: Option<types::CodeableConcept>,

    /// Assessment or judgement of the aspect
    pub rating: Option<types::CodeableConcept>,

    /// Individual or group who did the rating
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub rater: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`rater`](Self::rater) (FHIR `_rater`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rater")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rater_ext: Vec<Option<types::Element>>,

    /// A domain or subdomain of certainty
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subcomponent: Vec<EvidenceCertainty>,
}

/// Relationships that this Evidence has with other FHIR or non-FHIR resources
/// that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceRelatesTo;
/// use fhir::r6::types;
///
/// let value = EvidenceRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "EvidenceRelatesToDe")]
#[fhir_version("r6")]
pub struct EvidenceRelatesTo {
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

    /// The artifact that is related to this Evidence
    /// The `Evidence.relatesTo.target[x]` choice element (1..1); see [`EvidenceRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<EvidenceRelatesToTarget>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceRelatesToDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r6::codes::ArtifactRelationshipType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(flatten)]
    target: crate::r6::choice::Slot<EvidenceRelatesToTarget>,
}

impl ::core::convert::From<EvidenceRelatesToDe> for EvidenceRelatesTo {
    fn from(v: EvidenceRelatesToDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            target: v.target.0,
        }
    }
}

/// Values and parameters for a single statistic.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceStatistic;
/// use fhir::r6::types;
///
/// let value = EvidenceStatistic {
///     number_of_events: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfEvents` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfEvents"], ::serde_json::json!(0));
///
/// let back: EvidenceStatistic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceStatistic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A natural language summary of the statistic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Type of statistic, e.g., relative risk
    pub statistic_type: Option<types::CodeableConcept>,

    /// Associated category for categorical variable
    pub category: Option<types::CodeableConcept>,

    /// Statistic value
    pub quantity: Option<types::Quantity>,

    /// The number of events associated with the statistic
    pub number_of_events: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_events`](Self::number_of_events) (FHIR `_numberOfEvents`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfEvents")]
    pub number_of_events_ext: Option<types::Element>,

    /// The number of participants affected
    pub number_affected: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_affected`](Self::number_affected) (FHIR `_numberAffected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberAffected")]
    pub number_affected_ext: Option<types::Element>,

    /// Count of participants in the study sample
    pub sample_size: Option<EvidenceStatisticSampleSize>,

    /// An attribute of the Statistic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,

    /// An aspect of the statistical model
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_characteristic: Vec<EvidenceStatisticModelCharacteristic>,
}

/// A statistical attribute of the statistic such as a measure of
/// heterogeneity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceStatisticAttributeEstimate;
/// use fhir::r6::types;
///
/// let value = EvidenceStatisticAttributeEstimate {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: EvidenceStatisticAttributeEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceStatisticAttributeEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of the attribute estimate
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnote or explanatory note about the estimate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The type of attribute estimate, e.g., confidence interval or p value
    pub r#type: Option<types::CodeableConcept>,

    /// The singular quantity of the attribute estimate, for attribute
    /// estimates represented as single values, which may include a unit of
    /// measure
    pub quantity: Option<types::Quantity>,

    /// Level of confidence interval, e.g., 0.95 for 95% confidence interval
    pub level: Option<types::Decimal>,
    /// Primitive extension sibling for [`level`](Self::level) (FHIR `_level`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_level")]
    pub level_ext: Option<types::Element>,

    /// Lower and upper bound values of the attribute estimate
    pub range: Option<types::Range>,

    /// A nested attribute estimate; which is the attribute estimate of an
    /// attribute estimate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}

/// A component of the method to generate the statistic.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceStatisticModelCharacteristic;
/// use fhir::r6::types;
///
/// let value = EvidenceStatisticModelCharacteristic {
///     intended: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `intended` is the name this serializes to on the wire.
/// assert_eq!(json["intended"], ::serde_json::json!(true));
///
/// let back: EvidenceStatisticModelCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "EvidenceStatisticModelCharacteristicDe")]
#[fhir_version("r6")]
pub struct EvidenceStatisticModelCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Model specification
    pub code: types::CodeableConcept,

    /// The specific value (when paired with code)
    /// The `Evidence.statistic.modelCharacteristic.value[x]` choice element (0..1); see [`EvidenceStatisticModelCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<EvidenceStatisticModelCharacteristicValue>,

    /// The plan for analysis
    pub intended: Option<types::Boolean>,
    /// Primitive extension sibling for [`intended`](Self::intended) (FHIR `_intended`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intended")]
    pub intended_ext: Option<types::Element>,

    /// This model characteristic is part of the analysis that was applied,
    /// whether or not the analysis followed the plan
    pub applied: Option<types::Boolean>,
    /// Primitive extension sibling for [`applied`](Self::applied) (FHIR `_applied`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_applied")]
    pub applied_ext: Option<types::Element>,

    /// A variable adjusted for in the adjusted analysis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<EvidenceStatisticModelCharacteristicVariable>,

    /// An attribute of the model characteristic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute: Vec<EvidenceStatisticAttributeEstimate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceStatisticModelCharacteristicDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<EvidenceStatisticModelCharacteristicValue>,
    intended: Option<types::Boolean>,
    #[serde(rename = "_intended")]
    intended_ext: Option<types::Element>,
    applied: Option<types::Boolean>,
    #[serde(rename = "_applied")]
    applied_ext: Option<types::Element>,
    #[serde(default)]
    variable: Vec<EvidenceStatisticModelCharacteristicVariable>,
    #[serde(default)]
    attribute: Vec<EvidenceStatisticAttributeEstimate>,
}

impl ::core::convert::From<EvidenceStatisticModelCharacteristicDe>
    for EvidenceStatisticModelCharacteristic
{
    fn from(v: EvidenceStatisticModelCharacteristicDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
            intended: v.intended,
            intended_ext: v.intended_ext,
            applied: v.applied,
            applied_ext: v.applied_ext,
            variable: v.variable,
            attribute: v.attribute,
        }
    }
}

/// A variable adjusted for in the adjusted analysis.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceStatisticModelCharacteristicVariable;
/// use fhir::r6::types;
///
/// let value = EvidenceStatisticModelCharacteristicVariable {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceStatisticModelCharacteristicVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceStatisticModelCharacteristicVariable {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description and definition of the variable
    pub variable_definition: types::Reference,

    /// boolean | continuous | dichotomous | ordinal | polychotomous |
    /// extension
    pub handling: Option<crate::coded::Coded<crate::r6::codes::VariableHandling>>,
    /// Primitive extension sibling for [`handling`](Self::handling) (FHIR `_handling`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_handling")]
    pub handling_ext: Option<types::Element>,

    /// Qualitative label used for grouping values of a dichotomous, ordinal,
    /// or polychotomous variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_category: Vec<types::CodeableConcept>,

    /// Quantitative label used for grouping values of a dichotomous, ordinal,
    /// or polychotomous variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_quantity: Vec<types::Quantity>,

    /// Range of quantitative labels used for grouping values of a dichotomous,
    /// ordinal, or polychotomous variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_range: Vec<types::Range>,
}

/// Count of participants in the study sample.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceStatisticSampleSize;
/// use fhir::r6::types;
///
/// let value = EvidenceStatisticSampleSize {
///     number_of_studies: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfStudies` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfStudies"], ::serde_json::json!(0));
///
/// let back: EvidenceStatisticSampleSize = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceStatisticSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of sample size for statistic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnote or explanatory note about the sample size
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Number of contributing studies
    pub number_of_studies: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_studies`](Self::number_of_studies) (FHIR `_numberOfStudies`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfStudies")]
    pub number_of_studies_ext: Option<types::Element>,

    /// Total number of participants
    pub number_of_participants: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_participants`](Self::number_of_participants) (FHIR `_numberOfParticipants`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfParticipants")]
    pub number_of_participants_ext: Option<types::Element>,

    /// Number of participants with known results for measured variables
    pub known_data_count: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`known_data_count`](Self::known_data_count) (FHIR `_knownDataCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_knownDataCount")]
    pub known_data_count_ext: Option<types::Element>,
}

/// Description, classification, and definition of a single variable. The
/// collection of variables defines what the evidence is about.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::evidence::EvidenceVariableDefinition;
/// use fhir::r6::types;
///
/// let value = EvidenceVariableDefinition {
///     comparator_category: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comparatorCategory` is the name this serializes to on the wire.
/// assert_eq!(json["comparatorCategory"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EvidenceVariableDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A text description or summary of the variable
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// population | exposure | outcome | covariate
    pub variable_role: crate::coded::Coded<crate::r6::codes::VariableRole>,
    /// Primitive extension sibling for [`variable_role`](Self::variable_role) (FHIR `_variableRole`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variableRole")]
    pub variable_role_ext: Option<types::Element>,

    /// Sub-classification of the role of the variable
    pub role_subtype: Option<types::CodeableConcept>,

    /// The reference value used for comparison
    pub comparator_category: Option<types::String>,
    /// Primitive extension sibling for [`comparator_category`](Self::comparator_category) (FHIR `_comparatorCategory`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comparatorCategory")]
    pub comparator_category_ext: Option<types::Element>,

    /// Definition of the actual variable related to the statistic(s)
    pub observed: Option<types::Reference>,

    /// Definition of the intended variable related to the Evidence
    pub intended: Option<types::Reference>,

    /// low | moderate | high | exact
    pub directness_match: Option<types::CodeableConcept>,
}

/// The `Evidence.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `Evidence.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceRelatesToTarget {
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

/// The `Evidence.statistic.modelCharacteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceStatisticModelCharacteristicValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Evidence;

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
