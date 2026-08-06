//! MedicationKnowledge
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
//!
//! Version: 4.0.1
//!
//! Definition of Medication Knowledge
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Information about a medication that is used to support knowledge.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledge;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledge {
///     preparation_instruction: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preparationInstruction` is the name this serializes to on the wire.
/// assert_eq!(json["preparationInstruction"], ::serde_json::json!("# Heading"));
///
/// let back: MedicationKnowledge = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledge {
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

    /// Code that identifies this medication
    pub code: Option<types::CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r4::codes::MedicationknowledgeStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Manufacturer of the item
    pub manufacturer: Option<types::Reference>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// Amount of drug in package
    pub amount: Option<types::Quantity>,

    /// Additional names for a medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonym: Vec<types::String>,
    /// Primitive extension sibling for [`synonym`](Self::synonym) (FHIR `_synonym`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_synonym")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonym_ext: Vec<Option<types::Element>>,

    /// Associated or related medication information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,

    /// A medication resource that is associated with this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associated_medication: Vec<types::Reference>,

    /// Category of the medication or product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_type: Vec<types::CodeableConcept>,

    /// Associated documentation about the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monograph: Vec<MedicationKnowledgeMonograph>,

    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationKnowledgeIngredient>,

    /// The instructions for preparing the medication
    pub preparation_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`preparation_instruction`](Self::preparation_instruction) (FHIR `_preparationInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparationInstruction")]
    pub preparation_instruction_ext: Option<types::Element>,

    /// The intended or approved route of administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intended_route: Vec<types::CodeableConcept>,

    /// The pricing of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost: Vec<MedicationKnowledgeCost>,

    /// Program under which a medication is reviewed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,

    /// Guidelines for administration of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,

    /// Categorization of the medication within a formulary or classification
    /// system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,

    /// Details about packaged medications
    pub packaging: Option<MedicationKnowledgePackaging>,

    /// Specifies descriptive properties of the medicine
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,

    /// Potential clinical issue with or between medication(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contraindication: Vec<types::Reference>,

    /// Regulatory information about a medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory: Vec<MedicationKnowledgeRegulatory>,

    /// The time course of drug absorption, distribution, metabolism and
    /// excretion of a medication from the body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kinetics: Vec<MedicationKnowledgeKinetics>,
}

/// Guidelines for the administration of the medication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeAdministrationGuidelines;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeAdministrationGuidelines {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeAdministrationGuidelines = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeAdministrationGuidelines {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Dosage for the medication for the specific guidelines
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,

    /// Indication for use that apply to the specific administration guidelines
    /// The `MedicationKnowledge.administrationGuidelines.indication[x]` choice element (0..1); see [`MedicationKnowledgeAdministrationGuidelinesIndication`].
    #[serde(flatten)]
    pub indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,

    /// Characteristics of the patient that are relevant to the administration
    /// guidelines
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
}

/// Dosage for the medication for the specific guidelines.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeAdministrationGuidelinesDosage;
///
/// let value = MedicationKnowledgeAdministrationGuidelinesDosage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationKnowledgeAdministrationGuidelinesDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of dosage
    pub r#type: types::CodeableConcept,

    /// Dosage for the medication for the specific guidelines
    pub dosage: ::vec1::Vec1<types::Dosage>,
}

/// Characteristics of the patient that are relevant to the administration
/// guidelines (for example, height, weight, gender, etc.).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific characteristic that is relevant to the administration
    /// guideline
    /// The `MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristic[x]` choice element (1..1); see [`MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub characteristic:
        Option<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic>,

    /// The specific characteristic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_ext: Vec<Option<types::Element>>,
}

/// The price of the medication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeCost;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeCost {
///     source: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `source` is the name this serializes to on the wire.
/// assert_eq!(json["source"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The category of the cost information
    pub r#type: types::CodeableConcept,

    /// The source or owner for the price information
    pub source: Option<types::String>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// The price of the medication
    pub cost: types::Money,
}

/// Specifies descriptive properties of the medicine, such as color, shape,
/// imprints, etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeDrugCharacteristic;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeDrugCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeDrugCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeDrugCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code specifying the type of characteristic of medication
    pub r#type: Option<types::CodeableConcept>,

    /// Description of the characteristic
    /// The `MedicationKnowledge.drugCharacteristic.value[x]` choice element (0..1); see [`MedicationKnowledgeDrugCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeDrugCharacteristicValue>,
}

/// Identifies a particular constituent of interest in the product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeIngredient;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeIngredient {
///     is_active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isActive` is the name this serializes to on the wire.
/// assert_eq!(json["isActive"], ::serde_json::json!(true));
///
/// let back: MedicationKnowledgeIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Medication(s) or substance(s) contained in the medication
    /// The `MedicationKnowledge.ingredient.item[x]` choice element (1..1); see [`MedicationKnowledgeIngredientItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub item: Option<MedicationKnowledgeIngredientItem>,

    /// Active ingredient indicator
    pub is_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_active`](Self::is_active) (FHIR `_isActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isActive")]
    pub is_active_ext: Option<types::Element>,

    /// Quantity of ingredient present
    pub strength: Option<types::Ratio>,
}

/// The time course of drug absorption, distribution, metabolism and excretion
/// of a medication from the body.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeKinetics;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeKinetics {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeKinetics = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeKinetics {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The drug concentration measured at certain discrete points in time
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub area_under_curve: Vec<types::Quantity>,

    /// The median lethal dose of a drug
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lethal_dose_50: Vec<types::Quantity>,

    /// Time required for concentration in the body to decrease by half
    pub half_life_period: Option<types::Duration>,
}

/// Categorization of the medication within a formulary or classification
/// system.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeMedicineClassification;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeMedicineClassification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeMedicineClassification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeMedicineClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of category for the medication (for example, therapeutic
    /// classification, therapeutic sub-classification)
    pub r#type: types::CodeableConcept,

    /// Specific category assigned to the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,
}

/// The program under which the medication is reviewed.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeMonitoringProgram;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeMonitoringProgram {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeMonitoringProgram = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeMonitoringProgram {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of program under which the medication is monitored
    pub r#type: Option<types::CodeableConcept>,

    /// Name of the reviewing program
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// Associated documentation about the medication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeMonograph;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeMonograph {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeMonograph = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeMonograph {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The category of medication document
    pub r#type: Option<types::CodeableConcept>,

    /// Associated documentation about the medication
    pub source: Option<types::Reference>,
}

/// Information that only applies to packages (not products).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgePackaging;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgePackaging {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgePackaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgePackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code that defines the specific type of packaging that the medication
    /// can be found in
    pub r#type: Option<types::CodeableConcept>,

    /// The number of product units the package would contain if fully loaded
    pub quantity: Option<types::Quantity>,
}

/// Regulatory information about a medication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeRegulatory;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeRegulatory {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeRegulatory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeRegulatory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specifies the authority of the regulation
    pub regulatory_authority: types::Reference,

    /// Specifies if changes are allowed when dispensing a medication from a
    /// regulatory perspective
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,

    /// Specifies the schedule of a medication in jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<MedicationKnowledgeRegulatorySchedule>,

    /// The maximum number of units of the medication that can be dispensed in
    /// a period
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}

/// The maximum number of units of the medication that can be dispensed in a
/// period.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeRegulatoryMaxDispense;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeRegulatoryMaxDispense {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeRegulatoryMaxDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The maximum number of units of the medication that can be dispensed
    pub quantity: types::Quantity,

    /// The period that applies to the maximum number of units
    pub period: Option<types::Duration>,
}

/// Specifies the schedule of a medication in jurisdiction.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeRegulatorySchedule;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeRegulatorySchedule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeRegulatorySchedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeRegulatorySchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specifies the specific drug schedule
    pub schedule: types::CodeableConcept,
}

/// Specifies if changes are allowed when dispensing a medication from a
/// regulatory perspective.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeRegulatorySubstitution;
/// use fhir::r4::types;
///
/// let value = MedicationKnowledgeRegulatorySubstitution {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeRegulatorySubstitution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeRegulatorySubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specifies the type of substitution allowed
    pub r#type: types::CodeableConcept,

    /// Specifies if regulation allows for changes in the medication when
    /// dispensing
    pub allowed: types::Boolean,
    /// Primitive extension sibling for [`allowed`](Self::allowed) (FHIR `_allowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allowed")]
    pub allowed_ext: Option<types::Element>,
}

/// Associated or related knowledge about a medication.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::medication_knowledge::MedicationKnowledgeRelatedMedicationKnowledge;
///
/// let value = MedicationKnowledgeRelatedMedicationKnowledge::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationKnowledgeRelatedMedicationKnowledge = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category of medicationKnowledge
    pub r#type: types::CodeableConcept,

    /// Associated documentation about the associated medication knowledge
    pub reference: ::vec1::Vec1<types::Reference>,
}

/// The `MedicationKnowledge.administrationGuidelines.indication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    /// `indicationCodeableConcept` variant.
    #[fhir("indicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `indicationReference` variant.
    #[fhir("indicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationKnowledge.administrationGuidelines.patientCharacteristics.characteristic[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    /// `characteristicCodeableConcept` variant.
    #[fhir("characteristicCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `characteristicQuantity` variant.
    #[fhir("characteristicQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationKnowledge.drugCharacteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r4::choice::Primitive<types::Base64Binary>),
}

/// The `MedicationKnowledge.ingredient.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeIngredientItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationKnowledge;

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
