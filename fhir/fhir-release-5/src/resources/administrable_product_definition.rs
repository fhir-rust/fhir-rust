//! AdministrableProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
//!
//! Version: 5.0.0
//!
//! AdministrableProductDefinition Resource: A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// AdministrableProductDefinition
///
/// A medicinal product in the final form which is suitable for administering to a
/// patient (after any mixing of multiple components, dissolution etc. has been
/// performed). It describes the administrable form of a product, including its
/// dose form, unit of presentation, ingredients, and the routes by which it may
/// be given. This resource links back to the overall medicinal product and to the
/// manufactured items from which the administrable form is prepared.
///
/// This resource is typically used in medicinal product and regulatory workflows,
/// where it bridges the gap between the packaged, manufactured item and the form
/// that is actually administered to a subject of care, once any reconstitution,
/// dilution, or mixing has occurred. It captures clinically relevant properties
/// such as onset of action, allowed routes of administration, dosing limits, and
/// species-specific withdrawal periods for veterinary products. Consumers such as
/// prescribing, dispensing, and pharmacovigilance systems reference this resource
/// to determine how a product may safely and appropriately be given.
///
/// # Related resources
///
/// The `formOf` and `producedFrom` elements reference the source medicinal
/// product definition and manufactured item resources, while several fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Reference`](crate::r5::types::Reference) to describe dose forms, routes,
/// properties, and integral devices. This resource is closely related to
/// `MedicinalProductDefinition`, `ManufacturedItemDefinition`, and `Ingredient`.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::administrable_product_definition::AdministrableProductDefinition;
///
/// let value = AdministrableProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AdministrableProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinition {
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

    /// An identifier for the administrable product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The publication lifecycle status of this definition: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// References the overall medicinal product definition from which one or more constituent parts of this administrable product are prepared and used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub form_of: Vec<types::Reference>,

    /// The dose form of the final product after necessary reconstitution or processing, e.g. solution for injection
    pub administrable_dose_form: Option<types::CodeableConcept>,

    /// The presentation type in which this item is given to a patient. e.g. for a spray - 'puff'
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produced_from: Vec<types::Reference>,

    /// The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton, or using by incoming references from the Ingredient resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<types::CodeableConcept>,

    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product
    pub device: Option<types::Reference>,

    /// A general description of the product, when in its final form, suitable for administration e.g. effervescent blue liquid, to be swallowed
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Characteristics e.g. a product's onset of action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<AdministrableProductDefinitionProperty>,

    /// The path(s) by which the product is taken into or makes contact with the body, along with any associated dosing limits
    pub route_of_administration: vec1::Vec1<AdministrableProductDefinitionRouteOfAdministration>,
}

/// AdministrableProductDefinitionProperty
///
/// Characteristics e.g. a product's onset of action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::administrable_product_definition::AdministrableProductDefinitionProperty;
/// use fhir::r5::types;
///
/// let value = AdministrableProductDefinitionProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdministrableProductDefinitionProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// The `AdministrableProductDefinition.property.value[x]` choice element (0..1); see [`AdministrableProductDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<AdministrableProductDefinitionPropertyValue>,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<types::CodeableConcept>,
}

/// AdministrableProductDefinitionRouteOfAdministration
///
/// The path by which the product is taken into or makes contact with the body.
/// # Examples
///
/// ```
/// use fhir::r5::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministration;
/// use fhir::r5::types;
///
/// let value = AdministrableProductDefinitionRouteOfAdministration {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdministrableProductDefinitionRouteOfAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministration {
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

    /// The first dose (dose quantity) administered can be specified for the product
    pub first_dose: Option<types::Quantity>,

    /// The maximum single dose that can be administered
    pub max_single_dose: Option<types::Quantity>,

    /// The maximum dose quantity to be administered in any one 24-h period
    pub max_dose_per_day: Option<types::Quantity>,

    /// The maximum dose per treatment period that can be administered
    pub max_dose_per_treatment_period: Option<types::Ratio>,

    /// The maximum treatment period during which the product can be administered
    pub max_treatment_period: Option<types::Duration>,

    /// A species for which this route applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_species: Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>,
}

/// AdministrableProductDefinitionRouteOfAdministrationTargetSpecies
///
/// A species for which this route applies.
/// # Examples
///
/// ```
/// use fhir::r5::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministrationTargetSpecies;
/// use fhir::r5::types;
///
/// let value = AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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

    /// A species specific time during which consumption of animal product is not appropriate
    pub withdrawal_period:
        Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}

/// AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod
///
/// A species specific time during which consumption of animal product is not
/// appropriate.
/// # Examples
///
/// ```
/// use fhir::r5::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod;
/// use fhir::r5::types;
///
/// let value = AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
///     supporting_information: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `supportingInformation` is the name this serializes to on the wire.
/// assert_eq!(json["supportingInformation"], ::serde_json::json!("abc"));
///
/// let back: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of tissue for which the withdrawal period applies, e.g. meat, milk
    pub tissue: types::CodeableConcept,

    /// A value for the time
    pub value: types::Quantity,

    /// Extra information about the withdrawal period
    pub supporting_information: Option<types::String>,
    /// Primitive extension sibling for [`supporting_information`](Self::supporting_information) (FHIR `_supportingInformation`).
    #[serde(rename = "_supportingInformation")]
    pub supporting_information_ext: Option<types::Element>,
}

/// The `AdministrableProductDefinition.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdministrableProductDefinitionPropertyValue {
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
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}
