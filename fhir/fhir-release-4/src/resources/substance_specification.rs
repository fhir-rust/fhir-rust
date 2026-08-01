//! SubstanceSpecification
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceSpecification
//!
//! Version: 4.0.1
//!
//! The detailed description of a substance, typically at a level beyond what
//! is used for prescribing
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The detailed description of a substance, typically at a level beyond what
/// is used for prescribing.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecification;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecification {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecification {
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
    pub identifier: Option<types::Identifier>,

    /// High level categorization, e.g. polymer or nucleic acid
    pub r#type: Option<types::CodeableConcept>,

    /// Status of substance within the catalogue e.g. approved
    pub status: Option<types::CodeableConcept>,

    /// If the substance applies to only human or veterinary use
    pub domain: Option<types::CodeableConcept>,

    /// Textual description of the substance
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,

    /// Textual comment about this record of a substance
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Moiety, for structural modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub moiety: Vec<SubstanceSpecificationMoiety>,

    /// General specifications for this substance, including how it is related
    /// to other substances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<SubstanceSpecificationProperty>,

    /// General information detailing this substance
    pub reference_information: Option<types::Reference>,

    /// Structural information
    pub structure: Option<SubstanceSpecificationStructure>,

    /// Codes associated with the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<SubstanceSpecificationCode>,

    /// Names applicable to this substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<SubstanceSpecificationName>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic
    /// acids)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,

    /// A link between this substance and another, with details of the
    /// relationship
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<SubstanceSpecificationRelationship>,

    /// Data items specific to nucleic acids
    pub nucleic_acid: Option<types::Reference>,

    /// Data items specific to polymers
    pub polymer: Option<types::Reference>,

    /// Data items specific to proteins
    pub protein: Option<types::Reference>,

    /// Material or taxonomic/anatomical source for the substance
    pub source_material: Option<types::Reference>,
}

/// Codes associated with the substance.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationCode;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationCode {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubstanceSpecificationCode = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationCode {
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

    /// Status of the code assignment
    pub status: Option<types::CodeableConcept>,

    /// The date at which the code status is changed as part of the terminology
    /// maintenance
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Any comment can be provided in this field, if necessary
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Moiety, for structural modifications.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationMoiety;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationMoiety {
///     molecular_formula: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `molecularFormula` is the name this serializes to on the wire.
/// assert_eq!(json["molecularFormula"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationMoiety = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationMoiety {
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

    /// Molecular formula
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Quantitative value for this moiety
    /// The `SubstanceSpecification.moiety.amount[x]` choice element (0..1); see [`SubstanceSpecificationMoietyAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceSpecificationMoietyAmount>,
}

/// Names applicable to this substance.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationName;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationName {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: SubstanceSpecificationName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationName {
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

    /// Name type
    pub r#type: Option<types::CodeableConcept>,

    /// The status of the name
    pub status: Option<types::CodeableConcept>,

    /// If this is the preferred name for this substance
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,

    /// Language of the name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<types::CodeableConcept>,

    /// The use context of this name for example if there is a different name a
    /// drug active ingredient as opposed to a food colour additive
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<types::CodeableConcept>,

    /// The jurisdiction where this name applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// A synonym of this name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonym: Vec<SubstanceSpecificationName>,

    /// A translation for this name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translation: Vec<SubstanceSpecificationName>,

    /// Details of the official nature of this name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub official: Vec<SubstanceSpecificationNameOfficial>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Details of the official nature of this name.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationNameOfficial;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationNameOfficial {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubstanceSpecificationNameOfficial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationNameOfficial {
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

    /// The status of the official name
    pub status: Option<types::CodeableConcept>,

    /// Date of official name change
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
}

/// General specifications for this substance, including how it is related to
/// other substances.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationProperty;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationProperty {
///     parameters: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `parameters` is the name this serializes to on the wire.
/// assert_eq!(json["parameters"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A category for this property, e.g. Physical, Chemical, Enzymatic
    pub category: Option<types::CodeableConcept>,

    /// Property type e.g. viscosity, pH, isoelectric point
    pub code: Option<types::CodeableConcept>,

    /// Parameters that were used in the measurement of a property (e.g. for
    /// viscosity: measured at 20C with a pH of 7.1)
    pub parameters: Option<types::String>,
    /// Primitive extension sibling for [`parameters`](Self::parameters) (FHIR `_parameters`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_parameters")]
    pub parameters_ext: Option<types::Element>,

    /// A substance upon which a defining property depends (e.g. for
    /// solubility: in water, in alcohol)
    /// The `SubstanceSpecification.property.definingSubstance[x]` choice element (0..1); see [`SubstanceSpecificationPropertyDefiningSubstance`].
    #[serde(flatten)]
    pub defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,

    /// Quantitative value for this property
    /// The `SubstanceSpecification.property.amount[x]` choice element (0..1); see [`SubstanceSpecificationPropertyAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceSpecificationPropertyAmount>,
}

/// A link between this substance and another, with details of the
/// relationship.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationRelationship;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationRelationship {
///     is_defining: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDefining` is the name this serializes to on the wire.
/// assert_eq!(json["isDefining"], ::serde_json::json!(true));
///
/// let back: SubstanceSpecificationRelationship = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationRelationship {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A pointer to another substance, as a resource or just a
    /// representational code
    /// The `SubstanceSpecification.relationship.substance[x]` choice element (0..1); see [`SubstanceSpecificationRelationshipSubstance`].
    #[serde(flatten)]
    pub substance: Option<SubstanceSpecificationRelationshipSubstance>,

    /// For example "salt to parent", "active moiety", "starting material"
    pub relationship: Option<types::CodeableConcept>,

    /// For example where an enzyme strongly bonds with a particular substance,
    /// this is a defining relationship for that enzyme, out of several
    /// possible substance relationships
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// A numeric factor for the relationship, for instance to express that the
    /// salt of a substance has some percentage of the active substance in
    /// relation to some other
    /// The `SubstanceSpecification.relationship.amount[x]` choice element (0..1); see [`SubstanceSpecificationRelationshipAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceSpecificationRelationshipAmount>,

    /// For use when the numeric
    pub amount_ratio_low_limit: Option<types::Ratio>,

    /// An operator for the amount, for example "average", "approximately",
    /// "less than"
    pub amount_type: Option<types::CodeableConcept>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Structural information.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationStructure;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationStructure {
///     molecular_formula: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `molecularFormula` is the name this serializes to on the wire.
/// assert_eq!(json["molecularFormula"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationStructure {
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

    /// Molecular formula
    pub molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula`](Self::molecular_formula) (FHIR `_molecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormula")]
    pub molecular_formula_ext: Option<types::Element>,

    /// Specified per moiety according to the Hill system, i.e. first C, then
    /// H, then alphabetical, each moiety separated by a dot
    pub molecular_formula_by_moiety: Option<types::String>,
    /// Primitive extension sibling for [`molecular_formula_by_moiety`](Self::molecular_formula_by_moiety) (FHIR `_molecularFormulaByMoiety`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety_ext: Option<types::Element>,

    /// Applicable for single substances that contain a radionuclide or a
    /// non-natural isotopic ratio
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isotope: Vec<SubstanceSpecificationStructureIsotope>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic
    /// acids)
    pub molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,

    /// Supporting literature
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,

    /// Molecular structural representation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation: Vec<SubstanceSpecificationStructureRepresentation>,
}

/// Applicable for single substances that contain a radionuclide or a
/// non-natural isotopic ratio.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationStructureIsotope;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationStructureIsotope {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationStructureIsotope = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationStructureIsotope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Substance identifier for each non-natural or radioisotope
    pub identifier: Option<types::Identifier>,

    /// Substance name for each non-natural or radioisotope
    pub name: Option<types::CodeableConcept>,

    /// The type of isotopic substitution present in a single substance
    pub substitution: Option<types::CodeableConcept>,

    /// Half life - for a non-natural nuclide
    pub half_life: Option<types::Quantity>,

    /// The molecular weight or weight range (for proteins, polymers or nucleic
    /// acids)
    pub molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}

/// The molecular weight or weight range (for proteins, polymers or nucleic
/// acids).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationStructureIsotopeMolecularWeight;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationStructureIsotopeMolecularWeight {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationStructureIsotopeMolecularWeight = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The method by which the molecular weight was determined
    pub method: Option<types::CodeableConcept>,

    /// Type of molecular weight such as exact, average (also known as. number
    /// average), weight average
    pub r#type: Option<types::CodeableConcept>,

    /// Used to capture quantitative values for a variety of elements. If only
    /// limits are given, the arithmetic mean would be the average. If only a
    /// single definite value for a given element is given, it would be
    /// captured in this field
    pub amount: Option<types::Quantity>,
}

/// Molecular structural representation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_specification::SubstanceSpecificationStructureRepresentation;
/// use fhir::r4::types;
///
/// let value = SubstanceSpecificationStructureRepresentation {
///     representation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `representation` is the name this serializes to on the wire.
/// assert_eq!(json["representation"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSpecificationStructureRepresentation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSpecificationStructureRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of structure (e.g. Full, Partial, Representative)
    pub r#type: Option<types::CodeableConcept>,

    /// The structural representation as text string in a format e.g. InChI,
    /// SMILES, MOLFILE, CDX
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// An attached file with the structural representation
    pub attachment: Option<types::Attachment>,
}

/// The `SubstanceSpecification.moiety.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceSpecificationMoietyAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `SubstanceSpecification.property.definingSubstance[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    /// `definingSubstanceReference` variant.
    #[fhir("definingSubstanceReference")]
    Reference(Box<types::Reference>),
    /// `definingSubstanceCodeableConcept` variant.
    #[fhir("definingSubstanceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `SubstanceSpecification.property.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceSpecificationPropertyAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `SubstanceSpecification.relationship.substance[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceSpecificationRelationshipSubstance {
    /// `substanceReference` variant.
    #[fhir("substanceReference")]
    Reference(Box<types::Reference>),
    /// `substanceCodeableConcept` variant.
    #[fhir("substanceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `SubstanceSpecification.relationship.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceSpecificationRelationshipAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountRange` variant.
    #[fhir("amountRange")]
    Range(Box<types::Range>),
    /// `amountRatio` variant.
    #[fhir("amountRatio")]
    Ratio(Box<types::Ratio>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r4::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceSpecification;

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
