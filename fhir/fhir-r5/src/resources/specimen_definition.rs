//! SpecimenDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
//!
//! Version: 5.0.0
//!
//! SpecimenDefinition Resource: A kind of specimen with associated set of requirements.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A kind of specimen with associated set of requirements.
///
/// The SpecimenDefinition resource describes the characteristics and
/// requirements of a kind of specimen, including how it is collected,
/// the containers and additives it is placed in, and how it must be
/// handled prior to testing. It supports laboratory catalogs and order
/// entry by defining reusable specimen requirements.
///
/// SpecimenDefinition is a canonical, definitional resource: instances are
/// typically authored and maintained by a laboratory or diagnostic service
/// as part of its test catalog, and are referenced by orders and by the
/// resulting `Specimen` instances collected for a given
/// [`Patient`](crate::r5::resources::patient::Patient) or other subject.
/// A single SpecimenDefinition may describe several
/// acceptable specimen/container combinations (via `type_tested`), each
/// with its own preference, handling, and rejection criteria, allowing an
/// ordering system to present the range of valid collection options for a
/// given kind of test. It is closely related to `ServiceRequest` (which
/// orders a test that requires a specimen of a defined kind) and to
/// `ObservationDefinition` (which defines the expected observation produced
/// once the specimen is tested).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinition;
/// use fhir::r5::types;
///
/// let value = SpecimenDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenDefinitionDe")]
pub struct SpecimenDefinition {
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

    /// Logical canonical URL to reference this SpecimenDefinition (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier used by catalogs and order systems to identify this kind of specimen
    pub identifier: Option<types::Identifier>,

    /// Business version of the SpecimenDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `SpecimenDefinition.versionAlgorithm[x]` choice element (0..1); see [`SpecimenDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<SpecimenDefinitionVersionAlgorithm>,

    /// Name for this {{title}} (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this SpecimenDefinition (Human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Based on FHIR definition of another SpecimenDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from_canonical`](Self::derived_from_canonical) (FHIR `_derivedFromCanonical`).
    #[serde(rename = "_derivedFromCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical_ext: Vec<Option<types::Element>>,

    /// Based on external definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`derived_from_uri`](Self::derived_from_uri) (FHIR `_derivedFromUri`).
    #[serde(rename = "_derivedFromUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri_ext: Vec<Option<types::Element>>,

    /// Publication status of this definition: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If this SpecimenDefinition is not for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The `SpecimenDefinition.subject[x]` choice element (0..1); see [`SpecimenDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<SpecimenDefinitionSubject>,

    /// Date status first applied
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The name of the individual or organization that published the SpecimenDefinition
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the SpecimenDefinition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this SpecimenDefinition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When SpecimenDefinition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// The date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the SpecimenDefinition
    pub effective_period: Option<types::Period>,

    /// Kind of material to collect, coded via a [`CodeableConcept`](crate::r5::types::CodeableConcept) such as blood or urine
    pub type_collected: Option<types::CodeableConcept>,

    /// Patient preparation for collection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_preparation: Vec<types::CodeableConcept>,

    /// Time aspect for collection
    pub time_aspect: Option<types::String>,
    /// Primitive extension sibling for [`time_aspect`](Self::time_aspect) (FHIR `_timeAspect`).
    #[serde(rename = "_timeAspect")]
    pub time_aspect_ext: Option<types::Element>,

    /// Specimen collection procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collection: Vec<types::CodeableConcept>,

    /// One or more acceptable specimen/container combinations for testing by the lab, each with its own preference and handling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_tested: Vec<SpecimenDefinitionTypeTested>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinitionDe {
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
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    identifier: Option<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<SpecimenDefinitionVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    #[serde(default)]
    derived_from_canonical: Vec<types::Canonical>,
    #[serde(rename = "_derivedFromCanonical")]
    #[serde(default)]
    derived_from_canonical_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    derived_from_uri: Vec<types::Uri>,
    #[serde(rename = "_derivedFromUri")]
    #[serde(default)]
    derived_from_uri_ext: Vec<Option<types::Element>>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    #[serde(flatten)]
    subject: crate::r5::choice::Slot<SpecimenDefinitionSubject>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    effective_period: Option<types::Period>,
    type_collected: Option<types::CodeableConcept>,
    #[serde(default)]
    patient_preparation: Vec<types::CodeableConcept>,
    time_aspect: Option<types::String>,
    #[serde(rename = "_timeAspect")]
    time_aspect_ext: Option<types::Element>,
    #[serde(default)]
    collection: Vec<types::CodeableConcept>,
    #[serde(default)]
    type_tested: Vec<SpecimenDefinitionTypeTested>,
}

impl ::core::convert::From<SpecimenDefinitionDe> for SpecimenDefinition {
    fn from(v: SpecimenDefinitionDe) -> Self {
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
            derived_from_canonical: v.derived_from_canonical,
            derived_from_canonical_ext: v.derived_from_canonical_ext,
            derived_from_uri: v.derived_from_uri,
            derived_from_uri_ext: v.derived_from_uri_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            subject: v.subject.0,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            effective_period: v.effective_period,
            type_collected: v.type_collected,
            patient_preparation: v.patient_preparation,
            time_aspect: v.time_aspect,
            time_aspect_ext: v.time_aspect_ext,
            collection: v.collection,
            type_tested: v.type_tested,
        }
    }
}

/// Specimen in container intended for testing by lab.
///
/// Describes a kind of specimen conditioned for testing, expected from
/// the collected specimen, including the container and additives, the
/// handling requirements, and the acceptable rejection criteria.
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinitionTypeTested;
/// use fhir::r5::types;
///
/// let value = SpecimenDefinitionTypeTested {
///     is_derived: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDerived` is the name this serializes to on the wire.
/// assert_eq!(json["isDerived"], ::serde_json::json!(true));
///
/// let back: SpecimenDefinitionTypeTested = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTested {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Primary or secondary specimen
    pub is_derived: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_derived`](Self::is_derived) (FHIR `_isDerived`).
    #[serde(rename = "_isDerived")]
    pub is_derived_ext: Option<types::Element>,

    /// Type of intended specimen
    pub r#type: Option<types::CodeableConcept>,

    /// preferred | alternate
    pub preference: crate::r5::coded::Coded<crate::r5::codes::SpecimenContainedPreference>,
    /// Primitive extension sibling for [`preference`](Self::preference) (FHIR `_preference`).
    #[serde(rename = "_preference")]
    pub preference_ext: Option<types::Element>,

    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypeTestedContainer>,

    /// Requirements for specimen delivery and special handling
    pub requirement: Option<types::Markdown>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`).
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// The usual time for retaining this kind of specimen
    pub retention_time: Option<types::Duration>,

    /// Specimen for single use only
    pub single_use: Option<types::Boolean>,
    /// Primitive extension sibling for [`single_use`](Self::single_use) (FHIR `_singleUse`).
    #[serde(rename = "_singleUse")]
    pub single_use_ext: Option<types::Element>,

    /// Criterion specified for specimen rejection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejection_criterion: Vec<types::CodeableConcept>,

    /// Specimen handling before testing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handling: Vec<SpecimenDefinitionTypeTestedHandling>,

    /// Where the specimen will be tested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub testing_destination: Vec<types::CodeableConcept>,
}

/// The specimen's container.
///
/// The specimen's container, including its material, type, cap color,
/// capacity, minimum volume, associated additives, and any special
/// preparation applied to the container for this specimen type.
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinitionTypeTestedContainer;
/// use fhir::r5::types;
///
/// let value = SpecimenDefinitionTypeTestedContainer {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: SpecimenDefinitionTypeTestedContainer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenDefinitionTypeTestedContainerDe")]
pub struct SpecimenDefinitionTypeTestedContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The material type used for the container
    pub material: Option<types::CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Color of container cap
    pub cap: Option<types::CodeableConcept>,

    /// The description of the kind of container
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The capacity of this kind of container
    pub capacity: Option<types::Quantity>,

    /// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (0..1); see [`SpecimenDefinitionTypeTestedContainerMinimumVolume`].
    #[serde(flatten)]
    pub minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,

    /// Additive associated with container
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,

    /// Special processing applied to the container for this specimen type
    pub preparation: Option<types::Markdown>,
    /// Primitive extension sibling for [`preparation`](Self::preparation) (FHIR `_preparation`).
    #[serde(rename = "_preparation")]
    pub preparation_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinitionTypeTestedContainerDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    material: Option<types::CodeableConcept>,
    r#type: Option<types::CodeableConcept>,
    cap: Option<types::CodeableConcept>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    capacity: Option<types::Quantity>,
    #[serde(flatten)]
    minimum_volume: crate::r5::choice::Slot<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    #[serde(default)]
    additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    preparation: Option<types::Markdown>,
    #[serde(rename = "_preparation")]
    preparation_ext: Option<types::Element>,
}

impl ::core::convert::From<SpecimenDefinitionTypeTestedContainerDe>
    for SpecimenDefinitionTypeTestedContainer
{
    fn from(v: SpecimenDefinitionTypeTestedContainerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            material: v.material,
            r#type: v.r#type,
            cap: v.cap,
            description: v.description,
            description_ext: v.description_ext,
            capacity: v.capacity,
            minimum_volume: v.minimum_volume.0,
            additive: v.additive,
            preparation: v.preparation,
            preparation_ext: v.preparation_ext,
        }
    }
}

/// Additive associated with container.
///
/// Substance introduced in the kind of container to preserve, maintain
/// or enhance the specimen, referenced either as a coded concept or as
/// a reference to a Substance resource.
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinitionTypeTestedContainerAdditive;
/// use fhir::r5::types;
///
/// let value = SpecimenDefinitionTypeTestedContainerAdditive {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinitionTypeTestedContainerAdditive = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenDefinitionTypeTestedContainerAdditiveDe")]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `SpecimenDefinition.typeTested.container.additive.additive[x]` choice element (0..1); see [`SpecimenDefinitionTypeTestedContainerAdditiveAdditive`].
    #[serde(flatten)]
    pub additive: Option<SpecimenDefinitionTypeTestedContainerAdditiveAdditive>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinitionTypeTestedContainerAdditiveDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    additive: crate::r5::choice::Slot<SpecimenDefinitionTypeTestedContainerAdditiveAdditive>,
}

impl ::core::convert::From<SpecimenDefinitionTypeTestedContainerAdditiveDe>
    for SpecimenDefinitionTypeTestedContainerAdditive
{
    fn from(v: SpecimenDefinitionTypeTestedContainerAdditiveDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            additive: v.additive.0,
        }
    }
}

/// Specimen handling before testing.
///
/// Set of instructions for preservation and handling of a specimen at a
/// given temperature interval, including the maximum preservation time
/// for the specimen under those conditions.
/// # Examples
///
/// ```
/// use fhir::r5::resources::specimen_definition::SpecimenDefinitionTypeTestedHandling;
/// use fhir::r5::types;
///
/// let value = SpecimenDefinitionTypeTestedHandling {
///     instruction: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instruction` is the name this serializes to on the wire.
/// assert_eq!(json["instruction"], ::serde_json::json!("# Heading"));
///
/// let back: SpecimenDefinitionTypeTestedHandling = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinitionTypeTestedHandling {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Qualifies the interval of temperature
    pub temperature_qualifier: Option<types::CodeableConcept>,

    /// Temperature range for these handling instructions
    pub temperature_range: Option<types::Range>,

    /// Maximum preservation time
    pub max_duration: Option<types::Duration>,

    /// Preservation instruction
    pub instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`).
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SpecimenDefinition;

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
/// The `SpecimenDefinition.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `SpecimenDefinition.typeTested.container.additive.additive[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    /// `additiveCodeableConcept` variant.
    #[fhir("additiveCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `additiveReference` variant.
    #[fhir("additiveReference")]
    Reference(Box<types::Reference>),
}

/// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    /// `minimumVolumeQuantity` variant.
    #[fhir("minimumVolumeQuantity")]
    Quantity(Box<types::Quantity>),
    /// `minimumVolumeString` variant.
    #[fhir("minimumVolumeString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `SpecimenDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
