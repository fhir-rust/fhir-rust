//! ChargeItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
//!
//! Version: 6.0.0-ballot3
//!
//! Item containing charge code(s) associated with the provision of healthcare
//! provider products
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The resource ChargeItem describes the provision of healthcare provider
/// products for a certain patient, therefore referring not only to the
/// product, but containing in addition details of the provision, like date,
/// time, amounts and participating organizations and persons. Main Usage of
/// the ChargeItem is to enable the billing process and internal cost
/// allocation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::charge_item::ChargeItem;
/// use fhir::r6::types;
///
/// let value = ChargeItem {
///     entered_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `enteredDate` is the name this serializes to on the wire.
/// assert_eq!(json["enteredDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ChargeItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ChargeItem {
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

    /// Business Identifier for item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Defining information about the code of this charge item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`definition_uri`](Self::definition_uri) (FHIR `_definitionUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definitionUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_uri_ext: Vec<Option<types::Element>>,

    /// Resource defining the code of this ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`definition_canonical`](Self::definition_canonical) (FHIR `_definitionCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definitionCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_canonical_ext: Vec<Option<types::Element>>,

    /// planned | billable | not-billable | aborted | billed | entered-in-error
    /// | unknown
    pub status: crate::coded::Coded<crate::r6::codes::ChargeitemStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Part of referenced ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// A code that identifies the charge, like a billing code
    pub code: types::CodeableConcept,

    /// Individual service was done for/to
    pub subject: types::Reference,

    /// Encounter associated with this ChargeItem
    pub encounter: Option<types::Reference>,

    /// When the charged service was applied
    /// The `ChargeItem.occurrence[x]` choice element (0..1); see [`ChargeItemOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ChargeItemOccurrence>,

    /// Who performed charged service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ChargeItemPerformer>,

    /// Organization providing the charged service
    pub performing_organization: Option<types::Reference>,

    /// Organization requesting the charged service
    pub requesting_organization: Option<types::Reference>,

    /// Organization that has ownership of the (potential, future) revenue
    pub cost_center: Option<types::Reference>,

    /// Quantity of which the charge item has been serviced
    pub quantity: Option<types::Quantity>,

    /// Anatomical location, if relevant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bodysite: Vec<types::CodeableConcept>,

    /// Unit price overriding the associated rules
    pub unit_price_component: Option<types::MonetaryComponent>,

    /// Total price overriding the associated rules
    pub total_price_component: Option<types::MonetaryComponent>,

    /// Reason for overriding the list price/factor
    pub override_reason: Option<types::CodeableConcept>,

    /// Individual who was entering
    pub enterer: Option<types::Reference>,

    /// Date the charge item was entered
    pub entered_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`entered_date`](Self::entered_date) (FHIR `_enteredDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_enteredDate")]
    pub entered_date_ext: Option<types::Element>,

    /// Why was the charged service rendered?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Which rendered service is being charged?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<types::CodeableReference>,

    /// Product charged
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<types::CodeableReference>,

    /// Account to place this charge
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference>,

    /// Comments made about the ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Further information supporting this charge
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,
}

/// Indicates who or what performed or participated in the charged service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::charge_item::ChargeItemPerformer;
/// use fhir::r6::types;
///
/// let value = ChargeItemPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ChargeItemPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ChargeItemPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What type of performance was done
    pub function: Option<types::CodeableConcept>,

    /// Individual who was performing
    pub actor: types::Reference,
}

/// The `ChargeItem.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ChargeItemOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ChargeItem;

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
