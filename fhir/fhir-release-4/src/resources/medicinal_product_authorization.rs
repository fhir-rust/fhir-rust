//! MedicinalProductAuthorization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductAuthorization
//!
//! Version: 4.0.1
//!
//! The regulatory authorization of a medicinal product
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The regulatory authorization of a medicinal product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_authorization::MedicinalProductAuthorization;
/// use fhir::r4::types;
///
/// let value = MedicinalProductAuthorization {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicinalProductAuthorization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductAuthorization {
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

    /// Business identifier for the marketing authorization, as assigned by a
    /// regulator
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The medicinal product that is being authorized
    pub subject: Option<types::Reference>,

    /// The country in which the marketing authorization has been granted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,

    /// Jurisdiction within a country
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// The status of the marketing authorization
    pub status: Option<types::CodeableConcept>,

    /// The date at which the given status has become applicable
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// The date when a suspended the marketing or the marketing authorization
    /// of the product is anticipated to be restored
    pub restore_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`restore_date`](Self::restore_date) (FHIR `_restoreDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_restoreDate")]
    pub restore_date_ext: Option<types::Element>,

    /// The beginning of the time period in which the marketing authorization
    /// is in the specific status shall be specified A complete date consisting
    /// of day, month and year shall be specified using the ISO 8601 date
    /// format
    pub validity_period: Option<types::Period>,

    /// A period of time after authorization before generic product
    /// applicatiosn can be submitted
    pub data_exclusivity_period: Option<types::Period>,

    /// The date when the first authorization was granted by a Medicines
    /// Regulatory Agency
    pub date_of_first_authorization: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_of_first_authorization`](Self::date_of_first_authorization) (FHIR `_dateOfFirstAuthorization`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateOfFirstAuthorization")]
    pub date_of_first_authorization_ext: Option<types::Element>,

    /// Date of first marketing authorization for a company's new medicinal
    /// product in any country in the World
    pub international_birth_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`international_birth_date`](Self::international_birth_date) (FHIR `_internationalBirthDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_internationalBirthDate")]
    pub international_birth_date_ext: Option<types::Element>,

    /// The legal framework against which this authorization is granted
    pub legal_basis: Option<types::CodeableConcept>,

    /// Authorization in areas within a country
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdictional_authorization: Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>,

    /// Marketing Authorization Holder
    pub holder: Option<types::Reference>,

    /// Medicines Regulatory Agency
    pub regulator: Option<types::Reference>,

    /// The regulatory procedure for granting or amending a marketing
    /// authorization
    pub procedure: Option<MedicinalProductAuthorizationProcedure>,
}

/// Authorization in areas within a country.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_authorization::MedicinalProductAuthorizationJurisdictionalAuthorization;
/// use fhir::r4::types;
///
/// let value = MedicinalProductAuthorizationJurisdictionalAuthorization {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductAuthorizationJurisdictionalAuthorization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductAuthorizationJurisdictionalAuthorization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The assigned number for the marketing authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Country of authorization
    pub country: Option<types::CodeableConcept>,

    /// Jurisdiction within a country
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// The legal status of supply in a jurisdiction or region
    pub legal_status_of_supply: Option<types::CodeableConcept>,

    /// The start and expected end date of the authorization
    pub validity_period: Option<types::Period>,
}

/// The regulatory procedure for granting or amending a marketing
/// authorization.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_authorization::MedicinalProductAuthorizationProcedure;
/// use fhir::r4::types;
///
/// let value = MedicinalProductAuthorizationProcedure {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductAuthorizationProcedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductAuthorizationProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for this procedure
    pub identifier: Option<types::Identifier>,

    /// Type of procedure
    pub r#type: types::CodeableConcept,

    /// Date of procedure
    /// The `MedicinalProductAuthorization.procedure.date[x]` choice element (0..1); see [`MedicinalProductAuthorizationProcedureDate`].
    #[serde(flatten)]
    pub date: Option<MedicinalProductAuthorizationProcedureDate>,

    /// Applcations submitted to obtain a marketing authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub application: Vec<MedicinalProductAuthorizationProcedure>,
}

/// The `MedicinalProductAuthorization.procedure.date[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicinalProductAuthorizationProcedureDate {
    /// `datePeriod` variant.
    #[fhir("datePeriod")]
    Period(Box<types::Period>),
    /// `dateDateTime` variant.
    #[fhir("dateDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicinalProductAuthorization;

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
