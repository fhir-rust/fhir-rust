//! RegulatedAuthorization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
//!
//! Version: 6.0.0-ballot3
//!
//! Regulatory approval, clearance or licensing related to a regulated product,
//! treatment, facility or activity e.g. Market Authorization for a Medicinal
//! Product
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Regulatory approval, clearance or licensing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation,
/// rule or legislative act. An example is Market Authorization relating to a
/// Medicinal Product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::regulated_authorization::RegulatedAuthorization;
/// use fhir::r6::types;
///
/// let value = RegulatedAuthorization {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: RegulatedAuthorization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RegulatedAuthorization {
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

    /// Business identifier for the authorization, typically assigned by the
    /// authorizing body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The product type, treatment, facility or activity that is being
    /// authorized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Overall type of this authorization, for example drug marketing
    /// approval, orphan drug designation
    pub r#type: Option<types::CodeableConcept>,

    /// General textual supporting information
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The territory in which the authorization has been granted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub region: Vec<types::CodeableConcept>,

    /// The status that is authorised e.g. approved. Intermediate states can be
    /// tracked with cases and applications
    pub status: Option<types::CodeableConcept>,

    /// The date at which the current status was assigned
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// The time period in which the regulatory approval etc. is in effect,
    /// e.g. a Marketing Authorization includes the date of authorization
    /// and/or expiration date
    pub validity_period: Option<types::Period>,

    /// Condition for which the use of the regulated product applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableReference>,

    /// The intended use of the product, e.g. prevention, treatment
    pub intended_use: Option<types::CodeableConcept>,

    /// The legal/regulatory framework or reasons under which this
    /// authorization is granted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basis: Vec<types::CodeableConcept>,

    /// The organization that has been granted this authorization, by the
    /// regulator
    pub holder: Option<types::Reference<crate::r6::resources::Organization>>,

    /// The regulatory authority or authorizing body granting the authorization
    pub regulator: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Additional information or supporting documentation about the
    /// authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<types::Reference<crate::r6::resources::DocumentReference>>,

    /// The case or regulatory procedure for granting or amending a regulated
    /// authorization. Note: This area is subject to ongoing review and the
    /// workgroup is seeking implementer feedback on its use (see link at
    /// bottom of page)
    pub case: Option<RegulatedAuthorizationCase>,
}

/// The case or regulatory procedure for granting or amending a regulated
/// authorization. An authorization is granted in response to
/// submissions/applications by those seeking authorization. A case is the
/// administrative process that deals with the application(s) that relate to
/// this and assesses them. Note: This area is subject to ongoing review and
/// the workgroup is seeking implementer feedback on its use (see link at
/// bottom of page).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::regulated_authorization::RegulatedAuthorizationCase;
/// use fhir::r6::types;
///
/// let value = RegulatedAuthorizationCase {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RegulatedAuthorizationCase = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RegulatedAuthorizationCase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier by which this case can be referenced
    pub identifier: Option<types::Identifier>,

    /// The defining type of case
    pub r#type: Option<types::CodeableConcept>,

    /// The status associated with the case
    pub status: Option<types::CodeableConcept>,

    /// Relevant date for this case
    /// The `RegulatedAuthorization.case.date[x]` choice element (0..1); see [`RegulatedAuthorizationCaseDate`].
    #[serde(flatten)]
    pub date: Option<RegulatedAuthorizationCaseDate>,

    /// Applications submitted to obtain a regulated authorization. Steps
    /// within the longer running case or procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub application: Vec<RegulatedAuthorizationCase>,
}

/// The `RegulatedAuthorization.case.date[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RegulatedAuthorizationCaseDate {
    /// `datePeriod` variant.
    #[fhir("datePeriod")]
    Period(Box<types::Period>),
    /// `dateDateTime` variant.
    #[fhir("dateDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RegulatedAuthorization;

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
