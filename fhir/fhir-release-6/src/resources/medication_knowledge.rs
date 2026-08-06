//! MedicationKnowledge
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
//!
//! Version: 6.0.0-ballot3
//!
//! Definition of Medication Knowledge
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Information about a medication that is used to support knowledge.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledge;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier for this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Code that identifies this medication
    pub code: Option<types::CodeableConcept>,

    /// draft | active | retired | unknown
    pub status: Option<crate::coded::Coded<crate::r6::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Creator or owner of the knowledge or information about the medication
    pub author: Option<types::ContactDetail>,

    /// Codes that identify the different jurisdictions for which the
    /// information of this resource was created
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// A name associated with the medication being described
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Associated or related medication information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,

    /// The set of medication resources that are associated with this
    /// medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associated_medication: Vec<types::Reference>,

    /// Category of the medication or product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_type: Vec<types::CodeableConcept>,

    /// Associated documentation about the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monograph: Vec<MedicationKnowledgeMonograph>,

    /// The instructions for preparing the medication
    pub preparation_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`preparation_instruction`](Self::preparation_instruction) (FHIR `_preparationInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparationInstruction")]
    pub preparation_instruction_ext: Option<types::Element>,

    /// The pricing of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost: Vec<MedicationKnowledgeCost>,

    /// Program under which a medication is reviewed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,

    /// Guidelines or protocols for administration of the medication for an
    /// indication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication_guideline: Vec<MedicationKnowledgeIndicationGuideline>,

    /// Categorization of the medication within a formulary or classification
    /// system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,

    /// Details about packaged medications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<MedicationKnowledgePackaging>,

    /// Potential clinical issue with or between medication(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clinical_use_issue: Vec<types::Reference>,

    /// How the medication should be stored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage_guideline: Vec<MedicationKnowledgeStorageGuideline>,

    /// Regulatory information about a medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory: Vec<MedicationKnowledgeRegulatory>,

    /// Minimal definition information about the medication
    pub definitional: Option<MedicationKnowledgeDefinitional>,
}

/// The price of the medication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeCost;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct MedicationKnowledgeCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The date range for which the cost is effective
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effective_date: Vec<types::Period>,

    /// The category of the cost information
    pub r#type: types::CodeableConcept,

    /// The source or owner for the price information
    pub source: Option<types::String>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// The price or category of the cost of the medication
    /// The `MedicationKnowledge.cost.cost[x]` choice element (1..1); see [`MedicationKnowledgeCostCost`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub cost: Option<MedicationKnowledgeCostCost>,
}

/// Along with the link to a Medicinal Product Definition resource, this
/// information provides common definitional elements that are needed to
/// understand the specific medication that is being described.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeDefinitional;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeDefinitional {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeDefinitional = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeDefinitional {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definitional resources that provide more information about this
    /// medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// powder | tablets | capsule +
    pub dose_form: Option<types::CodeableConcept>,

    /// The intended or approved route of administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intended_route: Vec<types::CodeableConcept>,

    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationKnowledgeDefinitionalIngredient>,

    /// Specifies descriptive properties of the medicine
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drug_characteristic: Vec<MedicationKnowledgeDefinitionalDrugCharacteristic>,
}

/// Specifies descriptive properties of the medicine, such as color, shape,
/// imprints, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeDefinitionalDrugCharacteristic;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeDefinitionalDrugCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeDefinitionalDrugCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeDefinitionalDrugCharacteristic {
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
    /// The `MedicationKnowledge.definitional.drugCharacteristic.value[x]` choice element (0..1); see [`MedicationKnowledgeDefinitionalDrugCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeDefinitionalDrugCharacteristicValue>,
}

/// Identifies a particular constituent of interest in the product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeDefinitionalIngredient;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeDefinitionalIngredient {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeDefinitionalIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeDefinitionalIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Substances contained in the medication
    pub item: types::CodeableReference,

    /// A code that defines the type of ingredient, active, base, etc
    pub r#type: Option<types::CodeableConcept>,

    /// Quantity of ingredient present
    /// The `MedicationKnowledge.definitional.ingredient.strength[x]` choice element (0..1); see [`MedicationKnowledgeDefinitionalIngredientStrength`].
    #[serde(flatten)]
    pub strength: Option<MedicationKnowledgeDefinitionalIngredientStrength>,
}

/// Guidelines or protocols that are applicable for the administration of the
/// medication based on indication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeIndicationGuideline;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeIndicationGuideline {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeIndicationGuideline = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeIndicationGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Indication for use that applies to the specific administration
    /// guideline
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableReference>,

    /// Guidelines for dosage of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosing_guideline: Vec<MedicationKnowledgeIndicationGuidelineDosingGuideline>,
}

/// The guidelines for the dosage of the medication for the indication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeIndicationGuidelineDosingGuideline;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeIndicationGuidelineDosingGuideline {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeIndicationGuidelineDosingGuideline = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Intention of the treatment
    pub treatment_intent: Option<types::CodeableConcept>,

    /// Dosage for the medication for the specific guidelines
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage>,

    /// Type of treatment the guideline applies to
    pub administration_treatment: Option<types::CodeableConcept>,

    /// Characteristics of the patient that are relevant to the administration
    /// guidelines
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_characteristic:
        Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic>,
}

/// Dosage for the medication for the specific guidelines.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage;
///
/// let value = MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category of dosage for a medication
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
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Categorization of specific characteristic that is relevant to the
    /// administration guideline
    pub r#type: types::CodeableConcept,

    /// The specific characteristic
    /// The `MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value[x]` choice element (0..1); see [`MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue`].
    #[serde(flatten)]
    pub value:
        Option<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue>,
}

/// Categorization of the medication within a formulary or classification
/// system.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeMedicineClassification;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The source of the classification
    /// The `MedicationKnowledge.medicineClassification.source[x]` choice element (0..1); see [`MedicationKnowledgeMedicineClassificationSource`].
    #[serde(flatten)]
    pub source: Option<MedicationKnowledgeMedicineClassificationSource>,

    /// Specific category assigned to the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,
}

/// The program under which the medication is reviewed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeMonitoringProgram;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeMonograph;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgePackaging;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct MedicationKnowledgePackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Cost of the packaged medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost: Vec<MedicationKnowledgeCost>,

    /// The packaged medication that is being priced
    pub packaged_product: Option<types::Reference>,
}

/// Regulatory information about a medication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeRegulatory;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub schedule: Vec<types::CodeableConcept>,

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
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeRegulatoryMaxDispense;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Specifies if changes are allowed when dispensing a medication from a
/// regulatory perspective.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeRegulatorySubstitution;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Associated or related medications. For example, if the medication is a
/// branded product (e.g. Crestor), this is the Therapeutic Moeity (e.g.
/// Rosuvastatin) or if this is a generic medication (e.g. Rosuvastatin), this
/// would link to a branded product (e.g. Crestor.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeRelatedMedicationKnowledge;
///
/// let value = MedicationKnowledgeRelatedMedicationKnowledge::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicationKnowledgeRelatedMedicationKnowledge = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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

/// Information on how the medication should be stored, for example,
/// refrigeration temperatures and length of stability at a given temperature.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeStorageGuideline;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeStorageGuideline {
///     reference: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reference` is the name this serializes to on the wire.
/// assert_eq!(json["reference"], ::serde_json::json!("http://example.org"));
///
/// let back: MedicationKnowledgeStorageGuideline = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeStorageGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to additional information
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Additional storage notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Duration remains stable
    pub stability_duration: Option<types::Duration>,

    /// Setting or value of environment for adequate storage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environmental_setting: Vec<MedicationKnowledgeStorageGuidelineEnvironmentalSetting>,
}

/// Describes a setting/value on the environment for the adequate storage of
/// the medication and other substances. Environment settings may involve
/// temperature, humidity, or exposure to light.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_knowledge::MedicationKnowledgeStorageGuidelineEnvironmentalSetting;
/// use fhir::r6::types;
///
/// let value = MedicationKnowledgeStorageGuidelineEnvironmentalSetting {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationKnowledgeStorageGuidelineEnvironmentalSetting = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationKnowledgeStorageGuidelineEnvironmentalSetting {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Categorization of the setting
    pub r#type: types::CodeableConcept,

    /// Value of the setting
    /// The `MedicationKnowledge.storageGuideline.environmentalSetting.value[x]` choice element (1..1); see [`MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue>,
}

/// The `MedicationKnowledge.cost.cost[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeCostCost {
    /// `costMoney` variant.
    #[fhir("costMoney")]
    Money(Box<types::Money>),
    /// `costCodeableConcept` variant.
    #[fhir("costCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `MedicationKnowledge.definitional.drugCharacteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeDefinitionalDrugCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}

/// The `MedicationKnowledge.definitional.ingredient.strength[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeDefinitionalIngredientStrength {
    /// `strengthRatio` variant.
    #[fhir("strengthRatio")]
    Ratio(Box<types::Ratio>),
    /// `strengthCodeableConcept` variant.
    #[fhir("strengthCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `strengthQuantity` variant.
    #[fhir("strengthQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
}

/// The `MedicationKnowledge.medicineClassification.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeMedicineClassificationSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `MedicationKnowledge.storageGuideline.environmentalSetting.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue {
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
