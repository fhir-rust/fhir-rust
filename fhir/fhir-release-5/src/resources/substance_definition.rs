//! SubstanceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
//!
//! Version: 5.0.0
//!
//! SubstanceDefinition Resource: The detailed description of a substance, typically at a level beyond what is used for prescribing.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceDefinition
///
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing. It captures identifying characteristics such as
/// classification, molecular structure, names, codes, and relationships to other
/// substances. This resource is used in medicinal product and regulated substance
/// contexts to describe chemicals, polymers, nucleic acids, proteins, and
/// biological materials in detail.
///
/// `SubstanceDefinition` sits at the reference-data end of the FHIR medicinal
/// product model: rather than describing a dispensed or administered item, it
/// records the scientific and regulatory identity of a raw substance, including
/// its moieties, structure, molecular weight, source material, and synonyms or
/// translated/official names. Regulators, pharmacovigilance systems, and
/// pharmaceutical manufacturers use it to maintain authoritative substance
/// catalogues (for example, aligned with ISO 11238 identification of medicinal
/// product data) that other resources can reference by identifier or code
/// rather than duplicating substance detail inline.
///
/// See also: substances are commonly referenced from medication and ingredient
/// resources such as `Medication` and `MedicinalProductDefinition`, and they
/// make heavy use of shared data types like
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Identifier`](crate::r5::types::Identifier), and
/// [`Reference`](crate::r5::types::Reference) to link to supporting literature,
/// manufacturers, and suppliers.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinition;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceDefinition {
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

    /// One or more business identifiers (e.g. CAS number, UNII) by which this substance is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A business level version identifier of the substance
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Lifecycle status of this substance record within the catalogue, e.g. active, retired
    pub status: Option<types::CodeableConcept>,

    /// A categorization, high level e.g. polymer or nucleic acid, or food, chemical, biological, or lower e.g. polymer linear or branch chain, or type of impurity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,

    /// If the substance applies to human or veterinary use
    pub domain: Option<types::CodeableConcept>,

    /// The quality standard, established benchmark, to which substance complies (e.g. USP/NF, BP)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grade: Vec<types::CodeableConcept>,

    /// Textual description of the substance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_source: Vec<types::Reference<crate::r5::resources::Citation>>,

    /// Textual comment about the substance's catalogue or registry record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The entity that creates, makes, produces or fabricates the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r5::resources::Organization>>,

    /// An entity that is the source for the substance. It may be different from the manufacturer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplier: Vec<types::Reference<crate::r5::resources::Organization>>,

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
    pub reference_information:
        Option<types::Reference<crate::r5::resources::SubstanceReferenceInformation>>,

    /// The average mass of a molecule of a compound
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub molecular_weight: Vec<SubstanceDefinitionMolecularWeight>,

    /// Structural information, such as molecular formula and depictions like SMILES or InChI
    pub structure: Option<SubstanceDefinitionStructure>,

    /// Codes associated with the substance, e.g. regulatory or terminology codes and their status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<SubstanceDefinitionCode>,

    /// Names applicable to this substance, including synonyms, translations, and official names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<SubstanceDefinitionName>,

    /// A link between this substance and another, e.g. salt-to-parent or active-moiety relationships
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<SubstanceDefinitionRelationship>,

    /// Data items specific to nucleic acids
    pub nucleic_acid: Option<types::Reference<crate::r5::resources::SubstanceNucleicAcid>>,

    /// Data items specific to polymers
    pub polymer: Option<types::Reference<crate::r5::resources::SubstancePolymer>>,

    /// Data items specific to proteins
    pub protein: Option<types::Reference<crate::r5::resources::SubstanceProtein>>,

    /// Material or taxonomic/anatomical source
    pub source_material: Option<SubstanceDefinitionSourceMaterial>,
}

/// SubstanceDefinition.moiety - Moiety, for structural modifications
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionMoiety;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Stereochemistry type
    pub stereochemistry: Option<types::CodeableConcept>,

    /// Optical activity type
    pub optical_activity: Option<types::CodeableConcept>,

    /// Molecular formula for this moiety (e.g. with the Hill system)
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`).
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// The `SubstanceDefinition.moiety.amount[x]` choice element (0..1); see [`SubstanceDefinitionMoietyAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceDefinitionMoietyAmount>,

    /// The measurement type of the quantitative value
    pub measurement_type: Option<types::CodeableConcept>,
}

/// SubstanceDefinition.characterization - General specifications for this substance
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionCharacterization;
/// use fhir::r5::types;
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

    /// Describes the nature of the chemical entity and explains, for instance, whether this is a base or a salt form
    pub form: Option<types::CodeableConcept>,

    /// The description or justification in support of the interpretation of the data file
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The data produced by the analytical instrument or a pictorial representation of that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum analysis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file: Vec<types::Attachment>,
}

/// SubstanceDefinition.property - General specifications for this substance
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionProperty;
/// use fhir::r5::types;
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

    /// The `SubstanceDefinition.property.value[x]` choice element (0..1); see [`SubstanceDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<SubstanceDefinitionPropertyValue>,
}

/// SubstanceDefinition.molecularWeight - The average mass of a molecule of a compound
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionMolecularWeight;
/// use fhir::r5::types;
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

/// SubstanceDefinition.structure - Structural information
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionStructure;
/// use fhir::r5::types;
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

    /// An expression which states the number and type of atoms present in a molecule of a substance
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`).
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Specified per moiety according to the Hill system
    pub molecular_formula_by_moiety: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula_by_moiety`](Self::molecular_formula_by_moiety) (FHIR `_molecularFormulaByMoiety`).
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety_ext: Option<types::Element>,

    /// The molecular weight or weight range
    pub molecular_weight: Option<SubstanceDefinitionMolecularWeight>,

    /// The method used to find the structure e.g. X-ray, NMR
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technique: Vec<types::CodeableConcept>,

    /// Source of information for the structure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_document: Vec<types::Reference<crate::r5::resources::DocumentReference>>,

    /// A depiction of the structure of the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation: Vec<SubstanceDefinitionStructureRepresentation>,
}

/// SubstanceDefinition.structure.representation - A depiction of the structure of the substance
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionStructureRepresentation;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`).
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not the physical file format)
    pub format: Option<types::CodeableConcept>,

    /// An attachment with the structural representation e.g. a structure graphic or AnIML file
    pub document: Option<types::Reference<crate::r5::resources::DocumentReference>>,
}

/// SubstanceDefinition.code - Codes associated with the substance
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionCode;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Any comment can be provided in this field
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference<crate::r5::resources::DocumentReference>>,
}

/// SubstanceDefinition.name - Names applicable to this substance
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionName;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name type e.g. 'systematic',  'scientific, 'brand'
    pub r#type: Option<types::CodeableConcept>,

    /// The status of the name e.g. 'current', 'proposed'
    pub status: Option<types::CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,

    /// Human language that the name is written in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<types::CodeableConcept>,

    /// The use context of this name e.g. as an active ingredient or as a food colour additive
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
    pub source: Vec<types::Reference<crate::r5::resources::DocumentReference>>,
}

/// SubstanceDefinition.name.official - Details of the official nature of this name
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionNameOfficial;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
}

/// SubstanceDefinition.relationship - A link between this substance and another
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionRelationship;
/// use fhir::r5::types;
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
pub struct SubstanceDefinitionRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `SubstanceDefinition.relationship.substanceDefinition[x]` choice element (0..1); see [`SubstanceDefinitionRelationshipSubstanceDefinition`].
    #[serde(flatten)]
    pub substance_definition: Option<SubstanceDefinitionRelationshipSubstanceDefinition>,

    /// For example "salt to parent", "active moiety"
    pub r#type: types::CodeableConcept,

    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible relationships
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`).
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// The `SubstanceDefinition.relationship.amount[x]` choice element (0..1); see [`SubstanceDefinitionRelationshipAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceDefinitionRelationshipAmount>,

    /// For use when the numeric has an uncertain range
    pub ratio_high_limit_amount: Option<types::Ratio>,

    /// An operator for the amount, for example "average", "approximately", "less than"
    pub comparator: Option<types::CodeableConcept>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference<crate::r5::resources::DocumentReference>>,
}

/// SubstanceDefinition.sourceMaterial - Material or taxonomic/anatomical source
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_definition::SubstanceDefinitionSourceMaterial;
/// use fhir::r5::types;
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
pub struct SubstanceDefinitionSourceMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of the origin of the raw material. e.g. cat hair is an Animal source type
    pub r#type: Option<types::CodeableConcept>,

    /// The genus of an organism e.g. the Latin epithet of the plant/animal scientific name
    pub genus: Option<types::CodeableConcept>,

    /// The species of an organism e.g. the Latin epithet of the species of the plant/animal
    pub species: Option<types::CodeableConcept>,

    /// An anatomical origin of the source material within an organism
    pub part: Option<types::CodeableConcept>,

    /// The country or countries where the material is harvested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<types::CodeableConcept>,
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
/// The `SubstanceDefinition.moiety.amount[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionMoietyAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `SubstanceDefinition.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}

/// The `SubstanceDefinition.relationship.amount[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `SubstanceDefinition.relationship.substanceDefinition[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceDefinitionRelationshipSubstanceDefinition {
    /// `substanceDefinitionReference` variant.
    #[fhir("substanceDefinitionReference")]
    Reference(Box<types::Reference>),
    /// `substanceDefinitionCodeableConcept` variant.
    #[fhir("substanceDefinitionCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
