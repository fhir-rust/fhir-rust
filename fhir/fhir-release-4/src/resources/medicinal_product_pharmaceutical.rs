//! MedicinalProductPharmaceutical
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPharmaceutical
//!
//! Version: 4.0.1
//!
//! A pharmaceutical product described in terms of its composition and dose
//! form
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A pharmaceutical product described in terms of its composition and dose
/// form.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceutical;
///
/// let value = MedicinalProductPharmaceutical::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicinalProductPharmaceutical = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPharmaceutical {
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

    /// An identifier for the pharmaceutical medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The administrable dose form, after necessary reconstitution
    pub administrable_dose_form: types::CodeableConcept,

    /// Todo
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<types::Reference>,

    /// Accompanying device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference>,

    /// Characteristics e.g. a products onset of action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,

    /// The path by which the pharmaceutical product is taken into or makes
    /// contact with the body
    pub route_of_administration: ::vec1::Vec1<MedicinalProductPharmaceuticalRouteOfAdministration>,
}

/// Characteristics e.g. a products onset of action.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalCharacteristics;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPharmaceuticalCharacteristics {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPharmaceuticalCharacteristics = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A coded characteristic
    pub code: types::CodeableConcept,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<types::CodeableConcept>,
}

/// The path by which the pharmaceutical product is taken into or makes contact
/// with the body.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalRouteOfAdministration;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPharmaceuticalRouteOfAdministration {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPharmaceuticalRouteOfAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded expression for the route
    pub code: types::CodeableConcept,

    /// The first dose (dose quantity) administered in humans can be specified,
    /// for a product under investigation, using a numerical value and its unit
    /// of measurement
    pub first_dose: Option<types::Quantity>,

    /// The maximum single dose that can be administered as per the protocol of
    /// a clinical trial can be specified using a numerical value and its unit
    /// of measurement
    pub max_single_dose: Option<types::Quantity>,

    /// The maximum dose per day (maximum dose quantity to be administered in
    /// any one 24-h period) that can be administered as per the protocol
    /// referenced in the clinical trial authorisation
    pub max_dose_per_day: Option<types::Quantity>,

    /// The maximum dose per treatment period that can be administered as per
    /// the protocol referenced in the clinical trial authorisation
    pub max_dose_per_treatment_period: Option<types::Ratio>,

    /// The maximum treatment period during which an Investigational Medicinal
    /// Product can be administered as per the protocol referenced in the
    /// clinical trial authorisation
    pub max_treatment_period: Option<types::Duration>,

    /// A species for which this route applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
}

/// A species for which this route applies.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded expression for the species
    pub code: types::CodeableConcept,

    /// A species specific time during which consumption of animal product is
    /// not appropriate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}

/// A species specific time during which consumption of animal product is not
/// appropriate.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
///     supporting_information: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `supportingInformation` is the name this serializes to on the wire.
/// assert_eq!(json["supportingInformation"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded expression for the type of tissue for which the withdrawal period
    /// applues, e.g. meat, milk
    pub tissue: types::CodeableConcept,

    /// A value for the time
    pub value: types::Quantity,

    /// Extra information about the withdrawal period
    pub supporting_information: Option<types::String>,
    /// Primitive extension sibling for [`supporting_information`](Self::supporting_information) (FHIR `_supportingInformation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_supportingInformation")]
    pub supporting_information_ext: Option<types::Element>,
}
