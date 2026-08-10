//! RegulatedAuthorization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
//!
//! Version: 5.0.0
//!
//! RegulatedAuthorization Resource: Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Regulatory approval, clearance or licencing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation, rule
/// or legislative act. A common example is a Marketing Authorization relating to a
/// Medicinal Product, but the resource is deliberately general and can also describe
/// approvals, clearances, designations and licences for devices, foods, treatments,
/// facilities and other regulated activities.
///
/// In FHIR R5 this administrative resource records the regulatory context around
/// something that has been authorized: the subject of the authorization, the type
/// and legal basis of the approval, the territory or region in which it applies, the
/// authorizing regulator and the organization that holds the authorization, together
/// with its status, status date and validity period. It also carries an optional
/// case or procedure structure that can nest applications and steps, allowing the
/// progress of an approval process to be tracked over time. RegulatedAuthorization is
/// typically used by regulators, marketing authorization holders and product data
/// systems to exchange the authorization metadata that governs how a regulated item
/// may lawfully be supplied or used.
///
/// See also related types such as [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Reference`](crate::r5::types::Reference), [`Identifier`](crate::r5::types::Identifier)
/// and [`Period`](crate::r5::types::Period), and the nested `RegulatedAuthorizationCase`
/// structure that captures the underlying regulatory procedure.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::regulated_authorization::RegulatedAuthorization;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegulatedAuthorization {
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

    /// Business identifier for the authorization, typically assigned by the authorizing body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// References to the medicinal product, device, treatment, facility or activity that is the subject being authorized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Overall type of this authorization, for example drug marketing approval, orphan drug designation
    pub r#type: Option<types::CodeableConcept>,

    /// General textual supporting information
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The territory in which the authorization has been granted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub region: Vec<types::CodeableConcept>,

    /// The authorized status such as approved; intermediate states can instead be tracked through cases and applications
    pub status: Option<types::CodeableConcept>,

    /// The date at which the current status was assigned
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// The time period in which the regulatory approval etc. is in effect, e.g. a Marketing Authorization includes the date of authorization and/or expiration date
    pub validity_period: Option<types::Period>,

    /// Condition for which the use of the regulated product applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableReference>,

    /// The intended use of the product, e.g. prevention, treatment
    pub intended_use: Option<types::CodeableConcept>,

    /// The legal/regulatory framework or reasons under which this authorization is granted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basis: Vec<types::CodeableConcept>,

    /// The organization that has been granted this authorization, by the regulator
    pub holder: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Reference to the regulatory authority or authorizing body that granted the authorization
    pub regulator: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Additional information or supporting documentation about the authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<types::Reference<crate::r5::resources::DocumentReference>>,

    /// The case or regulatory procedure for granting or amending a regulated authorization
    pub case: Option<RegulatedAuthorizationCase>,
}

/// The case or regulatory procedure for granting or amending a regulated
/// authorization. Note: This area is subject to ongoing review and the workgroup
/// is seeking implementer feedback on its use.
/// # Examples
///
/// ```
/// use fhir::r5::resources::regulated_authorization::RegulatedAuthorizationCase;
/// use fhir::r5::types;
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

    /// The `RegulatedAuthorization.case.date[x]` choice element (0..1); see [`RegulatedAuthorizationCaseDate`].
    #[serde(flatten)]
    pub date: Option<RegulatedAuthorizationCaseDate>,

    /// Applications submitted to obtain a regulated authorization. Steps within the longer running case or procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub application: Vec<RegulatedAuthorizationCase>,
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
/// The `RegulatedAuthorization.case.date[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum RegulatedAuthorizationCaseDate {
    /// `datePeriod` variant.
    #[fhir("datePeriod")]
    Period(Box<types::Period>),
    /// `dateDateTime` variant.
    #[fhir("dateDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
