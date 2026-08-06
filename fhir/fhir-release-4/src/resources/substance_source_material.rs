//! SubstanceSourceMaterial
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
//!
//! Version: 4.0.1
//!
//! Source material shall capture information on the taxonomic and anatomical
//! origins as well as the fraction of a material that can result in or can be
//! modified to form a substance. This set of data elements shall be used to
//! define polymer substances isolated from biological matrices. Taxonomic and
//! anatomical origins shall be described using a controlled vocabulary as
//! required. This information is captured for naturally derived polymers ( .
//! starch) and structurally diverse substances. For Organisms belonging to the
//! Kingdom Plantae the Substance level defines the fresh material of a single
//! species or infraspecies, the Herbal Drug and the Herbal preparation. For
//! Herbal preparations, the fraction information will be captured at the
//! Substance information level and additional information for herbal extracts
//! will be captured at the Specified Substance Group 1 information level. See
//! for further explanation the Substance Class: Structurally Diverse and the
//! herbal annex
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to
/// define polymer substances isolated from biological matrices. Taxonomic and
/// anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the
/// Substance information level and additional information for herbal extracts
/// will be captured at the Specified Substance Group 1 information level. See
/// for further explanation the Substance Class: Structurally Diverse and the
/// herbal annex.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterial;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterial {
///     organism_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `organismName` is the name this serializes to on the wire.
/// assert_eq!(json["organismName"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterial {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// General high level classification of the source material specific to
    /// the origin of the material
    pub source_material_class: Option<types::CodeableConcept>,

    /// The type of the source material shall be specified based on a
    /// controlled vocabulary. For vaccines, this subclause refers to the class
    /// of infectious agent
    pub source_material_type: Option<types::CodeableConcept>,

    /// The state of the source material when extracted
    pub source_material_state: Option<types::CodeableConcept>,

    /// The unique identifier associated with the source material parent
    /// organism shall be specified
    pub organism_id: Option<types::Identifier>,

    /// The organism accepted Scientific name shall be provided based on the
    /// organism taxonomy
    pub organism_name: Option<types::String>,
    /// Primitive extension sibling for [`organism_name`](Self::organism_name) (FHIR `_organismName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_organismName")]
    pub organism_name_ext: Option<types::Element>,

    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID
    /// of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole
    /// plant)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_substance_id: Vec<types::Identifier>,

    /// The parent substance of the Herbal Drug, or Herbal preparation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_substance_name: Vec<types::String>,
    /// Primitive extension sibling for [`parent_substance_name`](Self::parent_substance_name) (FHIR `_parentSubstanceName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_parentSubstanceName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_substance_name_ext: Vec<Option<types::Element>>,

    /// The country where the plant material is harvested or the countries
    /// where the plasma is sourced from as laid down in accordance with the
    /// Plasma Master File. For “Plasma-derived substances” the attribute
    /// country of origin provides information about the countries used for the
    /// manufacturing of the Cryopoor plama or Crioprecipitate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<types::CodeableConcept>,

    /// The place/region where the plant is harvested or the places/regions
    /// where the animal source material has its habitat
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geographical_location: Vec<types::String>,
    /// Primitive extension sibling for [`geographical_location`](Self::geographical_location) (FHIR `_geographicalLocation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_geographicalLocation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geographical_location_ext: Vec<Option<types::Element>>,

    /// Stage of life for animals, plants, insects and microorganisms. This
    /// information shall be provided only when the substance is significantly
    /// different in these stages (e.g. foetal bovine serum)
    pub development_stage: Option<types::CodeableConcept>,

    /// Many complex materials are fractions of parts of plants, animals, or
    /// minerals. Fraction elements are often necessary to define both
    /// Substances and Specified Group 1 Substances. For substances derived
    /// from Plants, fraction information will be captured at the Substance
    /// information level ( . Oils, Juices and Exudates). Additional
    /// information for Extracts, such as extraction solvent composition, will
    /// be captured at the Specified Substance Group 1 information level. For
    /// plasma-derived products fraction information will be captured at the
    /// Substance and the Specified Substance Group 1 levels
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,

    /// This subclause describes the organism which the substance is derived
    /// from. For vaccines, the parent organism shall be specified based on
    /// these subclause elements. As an example, full taxonomy will be
    /// described for the Substance Name: ., Leaf
    pub organism: Option<SubstanceSourceMaterialOrganism>,

    /// To do
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_description: Vec<SubstanceSourceMaterialPartDescription>,
}

/// Many complex materials are fractions of parts of plants, animals, or
/// minerals. Fraction elements are often necessary to define both Substances
/// and Specified Group 1 Substances. For substances derived from Plants,
/// fraction information will be captured at the Substance information level (
/// . Oils, Juices and Exudates). Additional information for Extracts, such as
/// extraction solvent composition, will be captured at the Specified Substance
/// Group 1 information level. For plasma-derived products fraction information
/// will be captured at the Substance and the Specified Substance Group 1
/// levels.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialFractionDescription;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialFractionDescription {
///     fraction: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `fraction` is the name this serializes to on the wire.
/// assert_eq!(json["fraction"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialFractionDescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialFractionDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// This element is capturing information about the fraction of a plant
    /// part, or human plasma for fractionation
    pub fraction: Option<types::String>,
    /// Primitive extension sibling for [`fraction`](Self::fraction) (FHIR `_fraction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fraction")]
    pub fraction_ext: Option<types::Element>,

    /// The specific type of the material constituting the component. For
    /// Herbal preparations the particulars of the extracts (liquid/dry) is
    /// described in Specified Substance Group 1
    pub material_type: Option<types::CodeableConcept>,
}

/// This subclause describes the organism which the substance is derived from.
/// For vaccines, the parent organism shall be specified based on these
/// subclause elements. As an example, full taxonomy will be described for the
/// Substance Name: ., Leaf.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialOrganism;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialOrganism {
///     intraspecific_description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `intraspecificDescription` is the name this serializes to on the wire.
/// assert_eq!(json["intraspecificDescription"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialOrganism = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialOrganism {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The family of an organism shall be specified
    pub family: Option<types::CodeableConcept>,

    /// The genus of an organism shall be specified; refers to the Latin
    /// epithet of the genus element of the plant/animal scientific name; it is
    /// present in names for genera, species and infraspecies
    pub genus: Option<types::CodeableConcept>,

    /// The species of an organism shall be specified; refers to the Latin
    /// epithet of the species of the plant/animal; it is present in names for
    /// species and infraspecies
    pub species: Option<types::CodeableConcept>,

    /// The Intraspecific type of an organism shall be specified
    pub intraspecific_type: Option<types::CodeableConcept>,

    /// The intraspecific description of an organism shall be specified based
    /// on a controlled vocabulary. For Influenza Vaccine, the intraspecific
    /// description shall contain the syntax of the antigen in line with the
    /// WHO convention
    pub intraspecific_description: Option<types::String>,
    /// Primitive extension sibling for [`intraspecific_description`](Self::intraspecific_description) (FHIR `_intraspecificDescription`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intraspecificDescription")]
    pub intraspecific_description_ext: Option<types::Element>,

    /// 4.9.13.6.1 Author type (Conditional)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<SubstanceSourceMaterialOrganismAuthor>,

    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
    pub hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,

    /// 4.9.13.7.1 Kingdom (Conditional)
    pub organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}

/// 4.9.13.6.1 Author type (Conditional).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialOrganismAuthor;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialOrganismAuthor {
///     author_description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authorDescription` is the name this serializes to on the wire.
/// assert_eq!(json["authorDescription"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialOrganismAuthor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialOrganismAuthor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of author of an organism species shall be specified. The
    /// parenthetical author of an organism species refers to the first author
    /// who published the plant/animal name (of any rank). The primary author
    /// of an organism species refers to the first author(s), who validly
    /// published the plant/animal name
    pub author_type: Option<types::CodeableConcept>,

    /// The author of an organism species shall be specified. The author year
    /// of an organism shall also be specified when applicable; refers to the
    /// year in which the first author(s) published the infraspecific
    /// plant/animal name (of any rank)
    pub author_description: Option<types::String>,
    /// Primitive extension sibling for [`author_description`](Self::author_description) (FHIR `_authorDescription`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authorDescription")]
    pub author_description_ext: Option<types::Element>,
}

/// 4.9.13.8.1 Hybrid species maternal organism ID (Optional).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialOrganismHybrid;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialOrganismHybrid {
///     maternal_organism_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `maternalOrganismId` is the name this serializes to on the wire.
/// assert_eq!(json["maternalOrganismId"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialOrganismHybrid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialOrganismHybrid {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier of the maternal species constituting the hybrid organism
    /// shall be specified based on a controlled vocabulary. For plants, the
    /// parents aren’t always known, and it is unlikely that it will be known
    /// which is maternal and which is paternal
    pub maternal_organism_id: Option<types::String>,
    /// Primitive extension sibling for [`maternal_organism_id`](Self::maternal_organism_id) (FHIR `_maternalOrganismId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maternalOrganismId")]
    pub maternal_organism_id_ext: Option<types::Element>,

    /// The name of the maternal species constituting the hybrid organism shall
    /// be specified. For plants, the parents aren’t always known, and it is
    /// unlikely that it will be known which is maternal and which is paternal
    pub maternal_organism_name: Option<types::String>,
    /// Primitive extension sibling for [`maternal_organism_name`](Self::maternal_organism_name) (FHIR `_maternalOrganismName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maternalOrganismName")]
    pub maternal_organism_name_ext: Option<types::Element>,

    /// The identifier of the paternal species constituting the hybrid organism
    /// shall be specified based on a controlled vocabulary
    pub paternal_organism_id: Option<types::String>,
    /// Primitive extension sibling for [`paternal_organism_id`](Self::paternal_organism_id) (FHIR `_paternalOrganismId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paternalOrganismId")]
    pub paternal_organism_id_ext: Option<types::Element>,

    /// The name of the paternal species constituting the hybrid organism shall
    /// be specified
    pub paternal_organism_name: Option<types::String>,
    /// Primitive extension sibling for [`paternal_organism_name`](Self::paternal_organism_name) (FHIR `_paternalOrganismName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paternalOrganismName")]
    pub paternal_organism_name_ext: Option<types::Element>,

    /// The hybrid type of an organism shall be specified
    pub hybrid_type: Option<types::CodeableConcept>,
}

/// 4.9.13.7.1 Kingdom (Conditional).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialOrganismOrganismGeneral;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialOrganismOrganismGeneral {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialOrganismOrganismGeneral = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kingdom of an organism shall be specified
    pub kingdom: Option<types::CodeableConcept>,

    /// The phylum of an organism shall be specified
    pub phylum: Option<types::CodeableConcept>,

    /// The class of an organism shall be specified
    pub class: Option<types::CodeableConcept>,

    /// The order of an organism shall be specified,
    pub order: Option<types::CodeableConcept>,
}

/// To do.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_source_material::SubstanceSourceMaterialPartDescription;
/// use fhir::r4::types;
///
/// let value = SubstanceSourceMaterialPartDescription {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceSourceMaterialPartDescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstanceSourceMaterialPartDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Entity of anatomical origin of source material within an organism
    pub part: Option<types::CodeableConcept>,

    /// The detailed anatomic location when the part can be extracted from
    /// different anatomical locations of the organism. Multiple alternative
    /// locations may apply
    pub part_location: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceSourceMaterial;

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
