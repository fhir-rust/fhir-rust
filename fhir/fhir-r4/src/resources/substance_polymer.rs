//! SubstancePolymer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
//!
//! Version: 4.0.1
//!
//! Todo
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymer;
/// use fhir::r4::types;
///
/// let value = SubstancePolymer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymer {
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

    /// Todo
    pub class: Option<types::CodeableConcept>,

    /// Todo
    pub geometry: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub copolymer_connectivity: Vec<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub modification: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`modification`](Self::modification) (FHIR `_modification`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modification_ext: Vec<Option<types::Element>>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monomer_set: Vec<SubstancePolymerMonomerSet>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat: Vec<SubstancePolymerRepeat>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerMonomerSet;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerMonomerSet {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerMonomerSet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerMonomerSet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub ratio_type: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerMonomerSetStartingMaterial;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerMonomerSetStartingMaterial {
///     is_defining: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDefining` is the name this serializes to on the wire.
/// assert_eq!(json["isDefining"], ::serde_json::json!(true));
///
/// let back: SubstancePolymerMonomerSetStartingMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub material: Option<types::CodeableConcept>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// Todo
    pub amount: Option<types::SubstanceAmount>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerRepeat;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerRepeat {
///     number_of_units: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfUnits` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfUnits"], ::serde_json::json!(42));
///
/// let back: SubstancePolymerRepeat = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerRepeat {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub number_of_units: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_units`](Self::number_of_units) (FHIR `_numberOfUnits`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfUnits")]
    pub number_of_units_ext: Option<types::Element>,

    /// Todo
    pub average_molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`average_molecular_formula`](Self::average_molecular_formula) (FHIR `_averageMolecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_averageMolecularFormula")]
    pub average_molecular_formula_ext: Option<types::Element>,

    /// Todo
    pub repeat_unit_amount_type: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerRepeatRepeatUnit;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerRepeatRepeatUnit {
///     repeat_unit: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `repeatUnit` is the name this serializes to on the wire.
/// assert_eq!(json["repeatUnit"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeatRepeatUnit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerRepeatRepeatUnit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub orientation_of_polymerisation: Option<types::CodeableConcept>,

    /// Todo
    pub repeat_unit: Option<types::String>,
    /// Primitive extension sibling for [`repeat_unit`](Self::repeat_unit) (FHIR `_repeatUnit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_repeatUnit")]
    pub repeat_unit_ext: Option<types::Element>,

    /// Todo
    pub amount: Option<types::SubstanceAmount>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub degree: Option<types::CodeableConcept>,

    /// Todo
    pub amount: Option<types::SubstanceAmount>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::substance_polymer::SubstancePolymerRepeatRepeatUnitStructuralRepresentation;
/// use fhir::r4::types;
///
/// let value = SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
///     representation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `representation` is the name this serializes to on the wire.
/// assert_eq!(json["representation"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeatRepeatUnitStructuralRepresentation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// Todo
    pub attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstancePolymer;

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
