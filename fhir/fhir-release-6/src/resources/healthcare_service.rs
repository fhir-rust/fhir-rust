//! HealthcareService
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
//!
//! Version: 6.0.0-ballot3
//!
//! The details of a healthcare service available at a location
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The details of a healthcare service available at a location or in a
/// catalog. In the case where there is a hierarchy of services (for example,
/// Lab -> Pathology -> Wound Cultures), this can be represented using a set of
/// linked HealthcareServices.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::healthcare_service::HealthcareService;
/// use fhir::r6::types;
///
/// let value = HealthcareService {
///     extra_details: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `extraDetails` is the name this serializes to on the wire.
/// assert_eq!(json["extraDetails"], ::serde_json::json!("# Heading"));
///
/// let back: HealthcareService = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this HealthcareService record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Organization that provides this service
    pub provided_by: Option<types::Reference<crate::r6::resources::Organization>>,

    /// The service within which this service is offered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offered_in: Vec<types::Reference<crate::r6::resources::HealthcareService>>,

    /// Broad category of service being performed or delivered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of service that may be delivered or performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specialties handled by the HealthcareService
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// Location(s) where service may be provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r6::resources::Location>>,

    /// Description of service as presented to a consumer while searching
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Extra details about the service that can't be placed in the other
    /// fields
    pub extra_details: Option<types::Markdown>,
    /// Primitive extension sibling for [`extra_details`](Self::extra_details) (FHIR `_extraDetails`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extraDetails")]
    pub extra_details_ext: Option<types::Element>,

    /// Facilitates quick identification of the service
    pub photo: Option<types::Attachment>,

    /// Official contact details for the HealthcareService
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Location(s) service is intended for/available to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference<crate::r6::resources::Location>>,

    /// Conditions under which service is available/offered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_provision_code: Vec<types::CodeableConcept>,

    /// Specific eligibility requirements required to use the service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eligibility: Vec<HealthcareServiceEligibility>,

    /// Programs that this service is applicable to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program: Vec<types::CodeableConcept>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// The language that this service is offered in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<types::CodeableConcept>,

    /// Ways that the service accepts referrals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_method: Vec<types::CodeableConcept>,

    /// A referral is required for access to this service
    pub referral_required: Option<types::Boolean>,
    /// Primitive extension sibling for [`referral_required`](Self::referral_required) (FHIR `_referralRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_referralRequired")]
    pub referral_required_ext: Option<types::Element>,

    /// An appointment is required for access to this service
    pub appointment_required: Option<types::Boolean>,
    /// Primitive extension sibling for [`appointment_required`](Self::appointment_required) (FHIR `_appointmentRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_appointmentRequired")]
    pub appointment_required_ext: Option<types::Element>,

    /// Times the healthcare service is available (including exceptions)
    pub availability: Option<types::Availability>,

    /// Technical endpoints providing access to electronic services operated
    /// for the healthcare service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r6::resources::Endpoint>>,
}

/// Does this service have specific eligibility requirements that need to be
/// met in order to use the service?
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::healthcare_service::HealthcareServiceEligibility;
/// use fhir::r6::types;
///
/// let value = HealthcareServiceEligibility {
///     comment: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("# Heading"));
///
/// let back: HealthcareServiceEligibility = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct HealthcareServiceEligibility {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded value for the eligibility
    pub code: Option<types::CodeableConcept>,

    /// Value associated with the eligibility code
    /// The `HealthcareService.eligibility.value[x]` choice element (0..1); see [`HealthcareServiceEligibilityValue`].
    #[serde(flatten)]
    pub value: Option<HealthcareServiceEligibilityValue>,

    /// Describes the eligibility conditions for the service
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// The period this eligibility rule applies
    pub period: Option<types::Markdown>,
    /// Primitive extension sibling for [`period`](Self::period) (FHIR `_period`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_period")]
    pub period_ext: Option<types::Element>,
}

/// The `HealthcareService.eligibility.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum HealthcareServiceEligibilityValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HealthcareService;

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
