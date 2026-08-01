//! Citation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Citation
//!
//! Version: 6.0.0-ballot3
//!
//! A description of identification, location, or contributorship of a
//! publication (article or artifact)
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The Citation Resource enables reference to any knowledge artifact for
/// purposes of identification and attribution. The Citation Resource supports
/// existing reference structures and developing publication practices such as
/// versioning, expressing complex contributorship roles, and referencing
/// computable resources.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::Citation;
/// use fhir::r6::types;
///
/// let value = Citation {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: Citation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Citation {
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

    /// Canonical identifier for this citation record, represented as a
    /// globally unique URI
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Identifier for the citation record itself
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the citation record
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `Citation.versionAlgorithm[x]` choice element (0..1); see [`CitationVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CitationVersionAlgorithm>,

    /// Name for this citation record (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this citation record (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

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

    /// Who authored the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// The publisher of the citation record, not the publisher of the article
    /// or artifact being cited
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher of the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the citation
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the citation record content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for citation record (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this citation is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions for the citation record, not for the
    /// cited artifact
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s) for the ciation record, not for the cited
    /// artifact
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the citation record was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the citation record was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the citation record is expected to be used
    pub effective_period: Option<types::Period>,

    /// Who entered the data for the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recorder: Vec<types::ContactDetail>,

    /// Who edited the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// A human-readable display of key concepts to represent the citation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<CitationSummary>,

    /// The assignment to an organizing scheme
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<CitationClassification>,

    /// Used for general notes and annotations not coded elsewhere
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The status of the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_state: Vec<types::CodeableConcept>,

    /// An effective date or period for a status of the citation record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_date: Vec<CitationStatusDate>,

    /// Relationships to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<CitationRelatesTo>,

    /// The article or artifact being described
    pub cited_artifact: Option<CitationCitedArtifact>,
}

/// The article or artifact being described.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifact;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifact {
///     date_accessed: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateAccessed` is the name this serializes to on the wire.
/// assert_eq!(json["dateAccessed"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CitationCitedArtifact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier. May include DOI, PMID, PMCID, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Identifier not unique to the cited artifact. May include trial registry
    /// identifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_identifier: Vec<types::Identifier>,

    /// When the cited artifact was accessed
    pub date_accessed: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_accessed`](Self::date_accessed) (FHIR `_dateAccessed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateAccessed")]
    pub date_accessed_ext: Option<types::Element>,

    /// The defined version of the cited artifact
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The status of the cited artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_state: Vec<types::CodeableConcept>,

    /// An effective date or period for a status of the cited artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_date: Vec<CitationCitedArtifactStatusDate>,

    /// The title details of the article or artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title: Vec<CitationCitedArtifactTitle>,

    /// Summary of the article or artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#abstract: Vec<CitationCitedArtifactAbstract>,

    /// The component of the article or artifact
    pub part: Option<CitationCitedArtifactPart>,

    /// Citation for the primary version and complete form of the cited
    /// artifact
    pub base_citation: Option<types::Reference>,

    /// The artifact related to the cited artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<CitationCitedArtifactRelatesTo>,

    /// If multiple, used to represent alternative forms of the article that
    /// are not separate citations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publication_form: Vec<CitationCitedArtifactPublicationForm>,

    /// Used for any URL for the article or artifact cited
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web_location: Vec<CitationCitedArtifactWebLocation>,

    /// The assignment to an organizing scheme
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<CitationCitedArtifactClassification>,

    /// Attribution of authors and other contributors
    pub contributorship: Option<CitationCitedArtifactContributorship>,

    /// Any additional information or content for the article or artifact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The abstract may be used to convey article-contained abstracts,
/// externally-created abstracts, or other descriptive summaries.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactAbstract;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactAbstract {
///     copyright: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyright` is the name this serializes to on the wire.
/// assert_eq!(json["copyright"], ::serde_json::json!("# Heading"));
///
/// let back: CitationCitedArtifactAbstract = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactAbstract {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of abstract
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Used to express the specific language
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Abstract content
    pub text: types::Markdown,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Copyright notice for the abstract
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
}

/// The assignment to an organizing scheme.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactClassification;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactClassification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactClassification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of classifier (e.g. publication type, keyword)
    pub r#type: Option<types::CodeableConcept>,

    /// The specific classification value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Complex or externally created classification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_assessment: Vec<types::Reference>,
}

/// This element is used to list authors and other contributors, their contact
/// information, specific contributions, and summary statements.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactContributorship;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactContributorship {
///     complete: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `complete` is the name this serializes to on the wire.
/// assert_eq!(json["complete"], ::serde_json::json!(true));
///
/// let back: CitationCitedArtifactContributorship = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactContributorship {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Indicates if all authors are included
    pub complete: Option<types::Boolean>,
    /// Primitive extension sibling for [`complete`](Self::complete) (FHIR `_complete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_complete")]
    pub complete_ext: Option<types::Element>,

    /// An individual entity named as a contributor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<CitationCitedArtifactContributorshipEntry>,

    /// Used to record a display of the author/contributor list without
    /// separate data element for each list member
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<CitationCitedArtifactContributorshipSummary>,
}

/// An individual entity named as a contributor, for example in the author list
/// or contributor list.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactContributorshipEntry;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactContributorshipEntry {
///     forename_initials: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `forenameInitials` is the name this serializes to on the wire.
/// assert_eq!(json["forenameInitials"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactContributorshipEntry = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactContributorshipEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identity of the individual contributor
    pub contributor: types::Reference,

    /// For citation styles that use initials
    pub forename_initials: Option<types::String>,
    /// Primitive extension sibling for [`forename_initials`](Self::forename_initials) (FHIR `_forenameInitials`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_forenameInitials")]
    pub forename_initials_ext: Option<types::Element>,

    /// Organizational affiliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affiliation: Vec<types::Reference>,

    /// The specific contribution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contribution_type: Vec<types::CodeableConcept>,

    /// The role of the contributor (e.g. author, editor, reviewer, funder)
    pub role: Option<types::CodeableConcept>,

    /// Contributions with accounting for time or number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contribution_instance: Vec<CitationCitedArtifactContributorshipEntryContributionInstance>,

    /// Whether the contributor is the corresponding contributor for the role
    pub corresponding_contact: Option<types::Boolean>,
    /// Primitive extension sibling for [`corresponding_contact`](Self::corresponding_contact) (FHIR `_correspondingContact`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_correspondingContact")]
    pub corresponding_contact_ext: Option<types::Element>,

    /// Ranked order of contribution
    pub ranking_order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`ranking_order`](Self::ranking_order) (FHIR `_rankingOrder`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rankingOrder")]
    pub ranking_order_ext: Option<types::Element>,
}

/// Contributions with accounting for time or number.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactContributorshipEntryContributionInstance;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactContributorshipEntryContributionInstance {
///     time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `time` is the name this serializes to on the wire.
/// assert_eq!(json["time"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CitationCitedArtifactContributorshipEntryContributionInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactContributorshipEntryContributionInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific contribution
    pub r#type: types::CodeableConcept,

    /// The time that the contribution was made
    pub time: Option<types::DateTime>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,
}

/// Used to record a display of the author/contributor list without separate
/// data element for each list member.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactContributorshipSummary;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactContributorshipSummary {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactContributorshipSummary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactContributorshipSummary {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Such as author list, contributorship statement, funding statement,
    /// acknowledgements statement, or conflicts of interest statement
    pub r#type: Option<types::CodeableConcept>,

    /// The format for the display string
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<types::CodeableConcept>,

    /// Used to code the producer or rule for creating the display string
    pub source: Option<types::CodeableConcept>,

    /// The display string for the author list, contributor list, or
    /// contributorship statement
    pub value: types::Markdown,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The component of the article or artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactPart;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactPart {
///     value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `value` is the name this serializes to on the wire.
/// assert_eq!(json["value"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactPart = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactPart {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of component
    pub r#type: Option<types::CodeableConcept>,

    /// The specification of the component
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Where, when and how the artifact was published. If multiple, used to
/// represent alternative forms of the article that are not separate citations.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactPublicationForm;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactPublicationForm {
///     article_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `articleDate` is the name this serializes to on the wire.
/// assert_eq!(json["articleDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CitationCitedArtifactPublicationForm = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactPublicationForm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The collection the cited article or artifact is published in
    pub published_in: Option<CitationCitedArtifactPublicationFormPublishedIn>,

    /// Internet or Print
    pub cited_medium: Option<types::CodeableConcept>,

    /// Volume number of journal or other collection in which the article is
    /// published
    pub volume: Option<types::String>,
    /// Primitive extension sibling for [`volume`](Self::volume) (FHIR `_volume`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_volume")]
    pub volume_ext: Option<types::Element>,

    /// Issue, part or supplement of journal or other collection in which the
    /// article is published
    pub issue: Option<types::String>,
    /// Primitive extension sibling for [`issue`](Self::issue) (FHIR `_issue`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issue")]
    pub issue_ext: Option<types::Element>,

    /// The date the article was added to the database, or the date the article
    /// was released
    pub article_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`article_date`](Self::article_date) (FHIR `_articleDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_articleDate")]
    pub article_date_ext: Option<types::Element>,

    /// Text representation of the date on which the issue of the cited
    /// artifact was published
    pub publication_date_text: Option<types::String>,
    /// Primitive extension sibling for [`publication_date_text`](Self::publication_date_text) (FHIR `_publicationDateText`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationDateText")]
    pub publication_date_text_ext: Option<types::Element>,

    /// Season in which the cited artifact was published
    pub publication_date_season: Option<types::String>,
    /// Primitive extension sibling for [`publication_date_season`](Self::publication_date_season) (FHIR `_publicationDateSeason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationDateSeason")]
    pub publication_date_season_ext: Option<types::Element>,

    /// The date the article was last revised or updated in the database
    pub last_revision_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_revision_date`](Self::last_revision_date) (FHIR `_lastRevisionDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastRevisionDate")]
    pub last_revision_date_ext: Option<types::Element>,

    /// Language(s) in which this form of the article is published
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_ext: Vec<Option<types::Element>>,

    /// Entry number or identifier for inclusion in a database
    pub accession_number: Option<types::String>,
    /// Primitive extension sibling for [`accession_number`](Self::accession_number) (FHIR `_accessionNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accessionNumber")]
    pub accession_number_ext: Option<types::Element>,

    /// Used for full display of pagination
    pub page_string: Option<types::String>,
    /// Primitive extension sibling for [`page_string`](Self::page_string) (FHIR `_pageString`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_pageString")]
    pub page_string_ext: Option<types::Element>,

    /// Used for isolated representation of first page
    pub first_page: Option<types::String>,
    /// Primitive extension sibling for [`first_page`](Self::first_page) (FHIR `_firstPage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_firstPage")]
    pub first_page_ext: Option<types::Element>,

    /// Used for isolated representation of last page
    pub last_page: Option<types::String>,
    /// Primitive extension sibling for [`last_page`](Self::last_page) (FHIR `_lastPage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastPage")]
    pub last_page_ext: Option<types::Element>,

    /// Number of pages or screens
    pub page_count: Option<types::String>,
    /// Primitive extension sibling for [`page_count`](Self::page_count) (FHIR `_pageCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_pageCount")]
    pub page_count_ext: Option<types::Element>,

    /// Copyright notice for the full article or artifact
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
}

/// The collection the cited article or artifact is published in.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactPublicationFormPublishedIn;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactPublicationFormPublishedIn {
///     publisher_location: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `publisherLocation` is the name this serializes to on the wire.
/// assert_eq!(json["publisherLocation"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactPublicationFormPublishedIn = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactPublicationFormPublishedIn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Kind of container (e.g. Periodical, database, or book)
    pub r#type: Option<types::CodeableConcept>,

    /// Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID;
    /// Book identifiers include ISBN
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Name of the database or title of the book or journal
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Name of or resource describing the publisher
    pub publisher: Option<types::Reference>,

    /// Geographic location of the publisher
    pub publisher_location: Option<types::String>,
    /// Primitive extension sibling for [`publisher_location`](Self::publisher_location) (FHIR `_publisherLocation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisherLocation")]
    pub publisher_location_ext: Option<types::Element>,
}

/// The artifact related to the cited artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactRelatesTo;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactRelatesTo {
///     label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `label` is the name this serializes to on the wire.
/// assert_eq!(json["label"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactRelatesTo {
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

    /// Additional classifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Short label
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Brief description of the related artifact
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Bibliographic citation for the artifact
    pub citation: Option<types::Markdown>,
    /// Primitive extension sibling for [`citation`](Self::citation) (FHIR `_citation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citation")]
    pub citation_ext: Option<types::Element>,

    /// The artifact that is related to this Citation's cited artifact
    /// The `Citation.citedArtifact.relatesTo.target[x]` choice element (0..1); see [`CitationCitedArtifactRelatesToTarget`].
    #[serde(flatten)]
    pub target: Option<CitationCitedArtifactRelatesToTarget>,
}

/// An effective date or period, historical or future, actual or expected, for
/// a status of the cited artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactStatusDate;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactStatusDate {
///     actual: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `actual` is the name this serializes to on the wire.
/// assert_eq!(json["actual"], ::serde_json::json!(true));
///
/// let back: CitationCitedArtifactStatusDate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactStatusDate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of the status
    pub activity: types::CodeableConcept,

    /// Either occurred or expected
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// When the status started and/or ended
    pub period: types::Period,
}

/// The title details of the article or artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactTitle;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactTitle {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationCitedArtifactTitle = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactTitle {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of title
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Used to express the specific language
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// The title of the article or artifact
    pub text: types::Markdown,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// Used for any URL for the article or artifact cited.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationCitedArtifactWebLocation;
/// use fhir::r6::types;
///
/// let value = CitationCitedArtifactWebLocation {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: CitationCitedArtifactWebLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationCitedArtifactWebLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code the reason for different URLs, e.g. abstract and full-text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// The specific URL
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// The assignment to an organizing scheme.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationClassification;
/// use fhir::r6::types;
///
/// let value = CitationClassification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationClassification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of classifier (e.g. publication type, keyword)
    pub r#type: Option<types::CodeableConcept>,

    /// The specific classification value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,
}

/// Relationships that this Citation has with other FHIR or non-FHIR resources
/// that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationRelatesTo;
/// use fhir::r6::types;
///
/// let value = CitationRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationRelatesTo {
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

    /// The artifact that is related to this Citation
    /// The `Citation.relatesTo.target[x]` choice element (1..1); see [`CitationRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<CitationRelatesToTarget>,
}

/// The state or status of the citation record paired with an effective date or
/// period for that state.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationStatusDate;
/// use fhir::r6::types;
///
/// let value = CitationStatusDate {
///     actual: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `actual` is the name this serializes to on the wire.
/// assert_eq!(json["actual"], ::serde_json::json!(true));
///
/// let back: CitationStatusDate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationStatusDate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of the status
    pub activity: types::CodeableConcept,

    /// Either occurred or expected
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// When the status started and/or ended
    pub period: types::Period,
}

/// A human-readable display of key concepts to represent the citation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::citation::CitationSummary;
/// use fhir::r6::types;
///
/// let value = CitationSummary {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CitationSummary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CitationSummary {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Format for display of the citation summary
    pub style: Option<types::CodeableConcept>,

    /// The human-readable display of the citation summary
    pub text: types::Markdown,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// The `Citation.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CitationVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `Citation.citedArtifact.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CitationCitedArtifactRelatesToTarget {
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
}

/// The `Citation.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CitationRelatesToTarget {
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
    type T = Citation;

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
