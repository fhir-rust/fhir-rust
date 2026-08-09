//! VerificationResult
//!
//! URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
//!
//! Version: 4.0.1
//!
//! Describes validation requirements, source(s), status and dates for one or
//! more elements
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes validation requirements, source(s), status and dates for one or
/// more elements.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::verification_result::VerificationResult;
/// use fhir::r4::types;
///
/// let value = VerificationResult {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: VerificationResult = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VerificationResult {
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

    /// A resource that was validated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<types::Reference>,

    /// The fhirpath location(s) within the resource that was validated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_location: Vec<types::String>,
    /// Primitive extension sibling for [`target_location`](Self::target_location) (FHIR `_targetLocation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetLocation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_location_ext: Vec<Option<types::Element>>,

    /// none | initial | periodic
    pub need: Option<types::CodeableConcept>,

    /// attested | validated | in-process | req-revalid | val-fail | reval-fail
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// When the validation status was updated
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// nothing | primary | multiple
    pub validation_type: Option<types::CodeableConcept>,

    /// The primary process by which the target is validated (edit check; value
    /// set; primary source; multiple sources; standalone; in context)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_process: Vec<types::CodeableConcept>,

    /// Frequency of revalidation
    pub frequency: Option<types::Timing>,

    /// The date/time validation was last completed (including failed
    /// validations)
    pub last_performed: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_performed`](Self::last_performed) (FHIR `_lastPerformed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastPerformed")]
    pub last_performed_ext: Option<types::Element>,

    /// The date when target is next validated, if appropriate
    pub next_scheduled: Option<types::Date>,
    /// Primitive extension sibling for [`next_scheduled`](Self::next_scheduled) (FHIR `_nextScheduled`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_nextScheduled")]
    pub next_scheduled_ext: Option<types::Element>,

    /// fatal | warn | rec-only | none
    pub failure_action: Option<types::CodeableConcept>,

    /// Information about the primary source(s) involved in validation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub primary_source: Vec<VerificationResultPrimarySource>,

    /// Information about the entity attesting to information
    pub attestation: Option<VerificationResultAttestation>,

    /// Information about the entity validating information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validator: Vec<VerificationResultValidator>,
}

/// Information about the entity attesting to information.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::verification_result::VerificationResultAttestation;
/// use fhir::r4::types;
///
/// let value = VerificationResultAttestation {
///     source_identity_certificate: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sourceIdentityCertificate` is the name this serializes to on the wire.
/// assert_eq!(json["sourceIdentityCertificate"], ::serde_json::json!("abc"));
///
/// let back: VerificationResultAttestation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VerificationResultAttestation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The individual or organization attesting to information
    pub who: Option<types::Reference>,

    /// When the who is asserting on behalf of another (organization or
    /// individual)
    pub on_behalf_of: Option<types::Reference>,

    /// The method by which attested information was submitted/retrieved
    pub communication_method: Option<types::CodeableConcept>,

    /// The date the information was attested to
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// A digital identity certificate associated with the attestation source
    pub source_identity_certificate: Option<types::String>,
    /// Primitive extension sibling for [`source_identity_certificate`](Self::source_identity_certificate) (FHIR `_sourceIdentityCertificate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceIdentityCertificate")]
    pub source_identity_certificate_ext: Option<types::Element>,

    /// A digital identity certificate associated with the proxy entity
    /// submitting attested information on behalf of the attestation source
    pub proxy_identity_certificate: Option<types::String>,
    /// Primitive extension sibling for [`proxy_identity_certificate`](Self::proxy_identity_certificate) (FHIR `_proxyIdentityCertificate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_proxyIdentityCertificate")]
    pub proxy_identity_certificate_ext: Option<types::Element>,

    /// Proxy signature
    pub proxy_signature: Option<types::Signature>,

    /// Attester signature
    pub source_signature: Option<types::Signature>,
}

/// Information about the primary source(s) involved in validation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::verification_result::VerificationResultPrimarySource;
/// use fhir::r4::types;
///
/// let value = VerificationResultPrimarySource {
///     validation_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `validationDate` is the name this serializes to on the wire.
/// assert_eq!(json["validationDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: VerificationResultPrimarySource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VerificationResultPrimarySource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the primary source
    pub who: Option<types::Reference>,

    /// Type of primary source (License Board; Primary Education; Continuing
    /// Education; Postal Service; Relationship owner; Registration Authority;
    /// legal source; issuing source; authoritative source)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Method for exchanging information with the primary source
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication_method: Vec<types::CodeableConcept>,

    /// successful | failed | unknown
    pub validation_status: Option<types::CodeableConcept>,

    /// When the target was validated against the primary source
    pub validation_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`validation_date`](Self::validation_date) (FHIR `_validationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_validationDate")]
    pub validation_date_ext: Option<types::Element>,

    /// yes | no | undetermined
    pub can_push_updates: Option<types::CodeableConcept>,

    /// specific | any | source
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub push_type_available: Vec<types::CodeableConcept>,
}

/// Information about the entity validating information.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::verification_result::VerificationResultValidator;
/// use fhir::r4::types;
///
/// let value = VerificationResultValidator {
///     identity_certificate: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `identityCertificate` is the name this serializes to on the wire.
/// assert_eq!(json["identityCertificate"], ::serde_json::json!("abc"));
///
/// let back: VerificationResultValidator = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct VerificationResultValidator {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the organization validating information
    pub organization: types::Reference<crate::r4::resources::Organization>,

    /// A digital identity certificate associated with the validator
    pub identity_certificate: Option<types::String>,
    /// Primitive extension sibling for [`identity_certificate`](Self::identity_certificate) (FHIR `_identityCertificate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identityCertificate")]
    pub identity_certificate_ext: Option<types::Element>,

    /// Validator signature
    pub attestation_signature: Option<types::Signature>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VerificationResult;

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
