//! SubstanceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! The detailed description of a substance, typically at a level beyond what
//! is used for prescribing
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The detailed description of a substance, typically at a level beyond what
/// is used for prescribing.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinition;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinition {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinition {
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

    /// Identifier by which this substance is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A business level version identifier of the substance
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Status of substance within the catalogue e.g. active, retired
    pub status: Option<types::CodeableConcept>,

    /// A categorization, high level e.g. polymer or nucleic acid, or food,
    /// chemical, biological, or lower e.g. polymer linear or branch chain, or
    /// type of impurity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,

    /// If the substance applies to human or veterinary use
    pub domain: Option<types::CodeableConcept>,

    /// The quality standard, established benchmark, to which substance
    /// complies (e.g. USP/NF, BP)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grade: Vec<types::CodeableConcept>,

    /// Textual description of the substance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_source: Vec<types::Reference>,

    /// Textual comment about the substance's catalogue or registry record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The entity that creates, makes, produces or fabricates the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,

    /// An entity that is the source for the substance. It may be different
    /// from the manufacturer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplier: Vec<types::Reference>,

    /// Moiety, for structural modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub moiety: Vec<SubstanceDefinitionMoiety>,

    /// General specifications for this substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characterization: Vec<SubstanceDefinitionCharacterization>,

    /// General specifications for this substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<SubstanceDefinitionProperty>,

    /// General information detailing this substance
    pub reference_information: Option<types::Reference>,

    /// The average mass of a molecule of a compound
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub molecular_weight: Vec<SubstanceDefinitionMolecularWeight>,

    /// Structural information
    pub structure: Option<SubstanceDefinitionStructure>,

    /// Codes associated with the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<SubstanceDefinitionCode>,

    /// Names applicable to this substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<SubstanceDefinitionName>,

    /// A link between this substance and another
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<SubstanceDefinitionRelationship>,

    /// Data items specific to nucleic acids
    pub nucleic_acid: Option<types::Reference>,

    /// Data items specific to polymers
    pub polymer: Option<types::Reference>,

    /// Data items specific to proteins
    pub protein: Option<types::Reference>,

    /// Material or taxonomic/anatomical source
    pub source_material: Option<SubstanceDefinitionSourceMaterial>,
}

/// General specifications for this substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionCharacterization;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionCharacterization {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: SubstanceDefinitionCharacterization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionCharacterization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The method used to find the characterization e.g. HPLC
    pub technique: Option<types::CodeableConcept>,

    /// Describes the nature of the chemical entity and explains, for instance,
    /// whether this is a base or a salt form
    pub form: Option<types::CodeableConcept>,

    /// The description or justification in support of the interpretation of
    /// the data file
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The data produced by the analytical instrument or a pictorial
    /// representation of that data. Examples: a JCAMP, JDX, or ADX file, or a
    /// chromatogram or spectrum analysis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file: Vec<types::Attachment>,
}

/// Codes associated with the substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionCode;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionCode {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubstanceDefinitionCode = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionCode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific code
    pub code: Option<types::CodeableConcept>,

    /// Status of the code assignment, for example 'provisional', 'approved'
    pub status: Option<types::CodeableConcept>,

    /// The date at which the code status was changed
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Any comment can be provided in this field
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Moiety, for structural modifications.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionMoiety;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionMoiety {
///     molecular_formula: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `molecularFormula` is the name this serializes to on the wire.
/// assert_eq!(json["molecularFormula"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionMoiety = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionMoiety {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role that the moiety is playing
    pub role: Option<types::CodeableConcept>,

    /// Identifier by which this moiety substance is known
    pub identifier: Option<types::Identifier>,

    /// Textual name for this moiety substance
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Stereochemistry type
    pub stereochemistry: Option<types::CodeableConcept>,

    /// Optical activity type
    pub optical_activity: Option<types::CodeableConcept>,

    /// Molecular formula for this moiety (e.g. with the Hill system)
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Quantitative value for this moiety
    /// The `SubstanceDefinition.moiety.amount[x]` choice element (0..1); see [`SubstanceDefinitionMoietyAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceDefinitionMoietyAmount>,

    /// The measurement type of the quantitative value
    pub measurement_type: Option<types::CodeableConcept>,
}

/// The average mass of a molecule of a compound compared to 1/12 the mass of
/// carbon 12 and calculated as the sum of the atomic weights of the
/// constituent atoms.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionMolecularWeight;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionMolecularWeight {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionMolecularWeight = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionMolecularWeight {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The method by which the weight was determined
    pub method: Option<types::CodeableConcept>,

    /// Type of molecular weight e.g. exact, average, weight average
    pub r#type: Option<types::CodeableConcept>,

    /// Used to capture quantitative values for a variety of elements
    pub amount: types::Quantity,
}

/// Names applicable to this substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionName;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionName {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: SubstanceDefinitionName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The actual name
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name type e.g. 'systematic', 'scientific, 'brand'
    pub r#type: Option<types::CodeableConcept>,

    /// The status of the name e.g. 'current', 'proposed'
    pub status: Option<types::CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,

    /// Human language that the name is written in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<types::CodeableConcept>,

    /// The use context of this name e.g. as an active ingredient or as a food
    /// colour additive
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<types::CodeableConcept>,

    /// The jurisdiction where this name applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// A synonym of this particular name, by which the substance is also known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonym: Vec<SubstanceDefinitionName>,

    /// A translation for this name into another human language
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translation: Vec<SubstanceDefinitionName>,

    /// Details of the official nature of this name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub official: Vec<SubstanceDefinitionNameOfficial>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Details of the official nature of this name.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionNameOfficial;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionNameOfficial {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubstanceDefinitionNameOfficial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionNameOfficial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Which authority uses this official name
    pub authority: Option<types::CodeableConcept>,

    /// The status of the official name, for example 'draft', 'active'
    pub status: Option<types::CodeableConcept>,

    /// Date of official name change
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
}

/// General specifications for this substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionProperty;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code expressing the type of property
    pub r#type: types::CodeableConcept,

    /// A value for the property
    /// The `SubstanceDefinition.property.value[x]` choice element (0..1); see [`SubstanceDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<SubstanceDefinitionPropertyValue>,
}

/// A link between this substance and another, with details of the
/// relationship.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionRelationship;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionRelationship {
///     is_defining: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDefining` is the name this serializes to on the wire.
/// assert_eq!(json["isDefining"], ::serde_json::json!(true));
///
/// let back: SubstanceDefinitionRelationship = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A pointer to another substance, as a resource or a representational
    /// code
    /// The `SubstanceDefinition.relationship.substanceDefinition[x]` choice element (0..1); see [`SubstanceDefinitionRelationshipSubstanceDefinition`].
    #[serde(flatten)]
    pub substance_definition: Option<SubstanceDefinitionRelationshipSubstanceDefinition>,

    /// For example "salt to parent", "active moiety"
    pub r#type: types::CodeableConcept,

    /// For example where an enzyme strongly bonds with a particular substance,
    /// this is a defining relationship for that enzyme, out of several
    /// possible relationships
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// A numeric factor for the relationship, e.g. that a substance salt has
    /// some percentage of active substance in relation to some other
    /// The `SubstanceDefinition.relationship.amount[x]` choice element (0..1); see [`SubstanceDefinitionRelationshipAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceDefinitionRelationshipAmount>,

    /// For use when the numeric has an uncertain range
    pub ratio_high_limit_amount: Option<types::Ratio>,

    /// An operator for the amount, for example "average", "approximately",
    /// "less than"
    pub comparator: Option<types::CodeableConcept>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Material or taxonomic/anatomical source for the substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionSourceMaterial;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionSourceMaterial {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionSourceMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionSourceMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of the origin of the raw material. e.g. cat hair is an
    /// Animal source type
    pub r#type: Option<types::CodeableConcept>,

    /// The genus of an organism e.g. the Latin epithet of the plant/animal
    /// scientific name
    pub genus: Option<types::CodeableConcept>,

    /// The species of an organism e.g. the Latin epithet of the species of the
    /// plant/animal
    pub species: Option<types::CodeableConcept>,

    /// An anatomical origin of the source material within an organism
    pub part: Option<types::CodeableConcept>,

    /// The country or countries where the material is harvested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<types::CodeableConcept>,
}

/// Structural information.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionStructure;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionStructure {
///     molecular_formula: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `molecularFormula` is the name this serializes to on the wire.
/// assert_eq!(json["molecularFormula"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Stereochemistry type
    pub stereochemistry: Option<types::CodeableConcept>,

    /// Optical activity type
    pub optical_activity: Option<types::CodeableConcept>,

    /// An expression which states the number and type of atoms present in a
    /// molecule of a substance
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Specified per moiety according to the Hill system
    pub molecular_formula_by_moiety: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula_by_moiety`](Self::molecular_formula_by_moiety) (FHIR `_molecularFormulaByMoiety`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety_ext: Option<types::Element>,

    /// The molecular weight or weight range
    pub molecular_weight: Option<SubstanceDefinitionMolecularWeight>,

    /// The method used to find the structure e.g. X-ray, NMR
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technique: Vec<types::CodeableConcept>,

    /// Source of information for the structure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_document: Vec<types::Reference>,

    /// A depiction of the structure of the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation: Vec<SubstanceDefinitionStructureRepresentation>,
}

/// A depiction of the structure of the substance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_definition::SubstanceDefinitionStructureRepresentation;
/// use fhir::r6::types;
///
/// let value = SubstanceDefinitionStructureRepresentation {
///     representation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `representation` is the name this serializes to on the wire.
/// assert_eq!(json["representation"], ::serde_json::json!("abc"));
///
/// let back: SubstanceDefinitionStructureRepresentation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceDefinitionStructureRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of structural representation (e.g. full, partial)
    pub r#type: Option<types::CodeableConcept>,

    /// The structural representation as a text string in a standard format
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not
    /// the physical file format)
    pub format: Option<types::CodeableConcept>,

    /// An attachment with the structural representation e.g. a structure
    /// graphic or AnIML file
    pub document: Option<types::Reference>,
}

/// The `SubstanceDefinition.moiety.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionMoietyAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `SubstanceDefinition.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionPropertyValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}

/// The `SubstanceDefinition.relationship.substanceDefinition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionRelationshipSubstanceDefinition {
    /// `substanceDefinitionReference` variant.
    #[fhir("substanceDefinitionReference")]
    Reference(Box<types::Reference>),
    /// `substanceDefinitionCodeableConcept` variant.
    #[fhir("substanceDefinitionCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `SubstanceDefinition.relationship.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionRelationshipAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountRatio` variant.
    #[fhir("amountRatio")]
    Ratio(Box<types::Ratio>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r6::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceDefinition;

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
