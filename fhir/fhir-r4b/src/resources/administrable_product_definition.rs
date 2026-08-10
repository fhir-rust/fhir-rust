//! AdministrableProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
//!
//! Version: 4.3.0
//!
//! A medicinal product in the final form, suitable for administration - after
//! any mixing of multiple components
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A medicinal product in the final form which is suitable for administering
/// to a patient (after any mixing of multiple components, dissolution etc. has
/// been performed).
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::administrable_product_definition::AdministrableProductDefinition;
///
/// let value = AdministrableProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AdministrableProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct AdministrableProductDefinition {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for the administrable product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// References a product from which one or more of the constituent parts of
    /// that product can be prepared and used as described by this
    /// administrable product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub form_of: Vec<types::Reference<crate::r4b::resources::MedicinalProductDefinition>>,

    /// The dose form of the final product after necessary reconstitution or
    /// processing
    pub administrable_dose_form: Option<types::CodeableConcept>,

    /// The presentation type in which this item is given to a patient. e.g.
    /// for a spray - 'puff'
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Indicates the specific manufactured items that are part of the 'formOf'
    /// product that are used in the preparation of this specific administrable
    /// form
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produced_from: Vec<types::Reference<crate::r4b::resources::ManufacturedItemDefinition>>,

    /// The ingredients of this administrable medicinal product. This is only
    /// needed if the ingredients are not specified either using
    /// ManufacturedItemDefiniton, or using by incoming references from the
    /// Ingredient resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<types::CodeableConcept>,

    /// A device that is integral to the medicinal product, in effect being
    /// considered as an "ingredient" of the medicinal product
    pub device: Option<types::Reference<crate::r4b::resources::DeviceDefinition>>,

    /// Characteristics e.g. a product's onset of action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<AdministrableProductDefinitionProperty>,

    /// The path by which the product is taken into or makes contact with the
    /// body
    pub route_of_administration: ::vec1::Vec1<AdministrableProductDefinitionRouteOfAdministration>,
}

/// Characteristics e.g. a product's onset of action.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::administrable_product_definition::AdministrableProductDefinitionProperty;
/// use fhir::r4b::types;
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
#[serde(from = "AdministrableProductDefinitionPropertyDe")]
#[fhir_version("r4b")]
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

    /// A value for the characteristic
    /// The `AdministrableProductDefinition.property.value[x]` choice element (0..1); see [`AdministrableProductDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<AdministrableProductDefinitionPropertyValue>,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdministrableProductDefinitionPropertyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r4b::choice::Slot<AdministrableProductDefinitionPropertyValue>,
    status: Option<types::CodeableConcept>,
}

impl ::core::convert::From<AdministrableProductDefinitionPropertyDe>
    for AdministrableProductDefinitionProperty
{
    fn from(v: AdministrableProductDefinitionPropertyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
            status: v.status,
        }
    }
}

/// The path by which the product is taken into or makes contact with the body.
/// In some regions this is referred to as the licenced or approved route.
/// RouteOfAdministration cannot be used when the 'formOf' product already uses
/// MedicinalProductDefinition.route (and vice versa).
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministration;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

    /// The first dose (dose quantity) administered can be specified for the
    /// product
    pub first_dose: Option<types::Quantity>,

    /// The maximum single dose that can be administered
    pub max_single_dose: Option<types::Quantity>,

    /// The maximum dose quantity to be administered in any one 24-h period
    pub max_dose_per_day: Option<types::Quantity>,

    /// The maximum dose per treatment period that can be administered
    pub max_dose_per_treatment_period: Option<types::Ratio>,

    /// The maximum treatment period during which the product can be
    /// administered
    pub max_treatment_period: Option<types::Duration>,

    /// A species for which this route applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_species: Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>,
}

/// A species for which this route applies.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministrationTargetSpecies;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

    /// A species specific time during which consumption of animal product is
    /// not appropriate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub withdrawal_period:
        Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}

/// A species specific time during which consumption of animal product is not
/// appropriate.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of tissue for which the withdrawal period applies, e.g. meat,
    /// milk
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

/// The `AdministrableProductDefinition.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
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
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}
