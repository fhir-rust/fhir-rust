//! ChargeItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
//!
//! Version: 5.0.0
//!
//! ChargeItem Resource: The provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The resource ChargeItem describes the provision of healthcare provider
/// products for a certain patient, therefore referring not only to the product,
/// but containing in addition details of the provision, like date, time,
/// amounts and participating organizations and persons. The main usage of the
/// ChargeItem is to enable the billing process and internal cost allocation.
///
/// A ChargeItem is typically generated automatically or semi-automatically
/// whenever a billable event occurs during patient care, such as the
/// performance of a procedure, the dispensing of a medication, or the use of
/// a device or facility resource. It captures who or what performed the
/// service, when it occurred, the quantity and pricing information, and the
/// organizations that requested and performed the service, as well as any
/// override reasons applied to standard pricing rules. Downstream systems
/// aggregate ChargeItem instances, typically via an Invoice or similar
/// billing resource, to produce claims or patient statements.
///
/// # Related resources
///
/// - The `subject` of a ChargeItem is usually a
///   [`Patient`](crate::r5::resources::patient::Patient), and the `encounter`
///   field commonly references an `Encounter` during which the service was
///   rendered.
/// - Pricing and reason fields such as `code`, `bodysite`, `reason`, and
///   `override_reason` are represented using
///   [`CodeableConcept`](crate::r5::types::CodeableConcept).
/// - The `performer`, `enterer`, `cost_center`, `requesting_organization`,
///   and `performing_organization` fields link to actors via
///   [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item::ChargeItem;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ChargeItemDe")]
pub struct ChargeItem {
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

    /// Business Identifier for item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Defining information about the code of this charge item
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub definition_uri: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`definition_uri`](Self::definition_uri) (FHIR `_definitionUri`).
    #[serde(rename = "_definitionUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_uri_ext: Vec<Option<types::Element>>,

    /// Resource defining the code of this ChargeItem
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub definition_canonical: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`definition_canonical`](Self::definition_canonical) (FHIR `_definitionCanonical`).
    #[serde(rename = "_definitionCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_canonical_ext: Vec<Option<types::Element>>,

    /// The current lifecycle status of the charge: planned | billable | not-billable | aborted | billed | entered-in-error | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::ChargeitemStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Part of referenced ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r5::resources::ChargeItem>>,

    /// A code that identifies the charge, like a billing code
    pub code: types::CodeableConcept,

    /// The individual, typically a `Patient`, for whom the charged service was performed
    pub subject: types::Reference,

    /// Encounter associated with this ChargeItem
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `ChargeItem.occurrence[x]` choice element (0..1); see [`ChargeItemOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ChargeItemOccurrence>,

    /// Who performed charged service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ChargeItemPerformer>,

    /// Organization providing the charged service
    pub performing_organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Organization requesting the charged service
    pub requesting_organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Organization that has ownership of the (potential, future) revenue
    pub cost_center: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Quantity of the item or service being charged, e.g. the number of units administered
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
    /// Primitive extension sibling for [`entered_date`](Self::entered_date) (FHIR `_enteredDate`).
    #[serde(rename = "_enteredDate")]
    pub entered_date_ext: Option<types::Element>,

    /// Why was the charged  service rendered?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Which rendered service is being charged?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<types::CodeableReference>,

    /// Product charged
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<types::CodeableReference>,

    /// Account to place this charge
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r5::resources::Account>>,

    /// Comments made about the ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Further information supporting this charge
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChargeItemDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    definition_uri: ::fhir_core::PrimVec<types::Uri>,
    #[serde(rename = "_definitionUri")]
    #[serde(default)]
    definition_uri_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    definition_canonical: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_definitionCanonical")]
    #[serde(default)]
    definition_canonical_ext: Vec<Option<types::Element>>,
    status: crate::r5::coded::Coded<crate::r5::codes::ChargeitemStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    part_of: Vec<types::Reference<crate::r5::resources::ChargeItem>>,
    code: types::CodeableConcept,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r5::choice::Slot<ChargeItemOccurrence>,
    #[serde(default)]
    performer: Vec<ChargeItemPerformer>,
    performing_organization: Option<types::Reference<crate::r5::resources::Organization>>,
    requesting_organization: Option<types::Reference<crate::r5::resources::Organization>>,
    cost_center: Option<types::Reference<crate::r5::resources::Organization>>,
    quantity: Option<types::Quantity>,
    #[serde(default)]
    bodysite: Vec<types::CodeableConcept>,
    unit_price_component: Option<types::MonetaryComponent>,
    total_price_component: Option<types::MonetaryComponent>,
    override_reason: Option<types::CodeableConcept>,
    enterer: Option<types::Reference>,
    entered_date: Option<types::DateTime>,
    #[serde(rename = "_enteredDate")]
    entered_date_ext: Option<types::Element>,
    #[serde(default)]
    reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    service: Vec<types::CodeableReference>,
    #[serde(default)]
    product: Vec<types::CodeableReference>,
    #[serde(default)]
    account: Vec<types::Reference<crate::r5::resources::Account>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    supporting_information: Vec<types::Reference>,
}

impl ::core::convert::From<ChargeItemDe> for ChargeItem {
    fn from(v: ChargeItemDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            definition_uri: v.definition_uri,
            definition_uri_ext: v.definition_uri_ext,
            definition_canonical: v.definition_canonical,
            definition_canonical_ext: v.definition_canonical_ext,
            status: v.status,
            status_ext: v.status_ext,
            part_of: v.part_of,
            code: v.code,
            subject: v.subject,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            performer: v.performer,
            performing_organization: v.performing_organization,
            requesting_organization: v.requesting_organization,
            cost_center: v.cost_center,
            quantity: v.quantity,
            bodysite: v.bodysite,
            unit_price_component: v.unit_price_component,
            total_price_component: v.total_price_component,
            override_reason: v.override_reason,
            enterer: v.enterer,
            entered_date: v.entered_date,
            entered_date_ext: v.entered_date_ext,
            reason: v.reason,
            service: v.service,
            product: v.product,
            account: v.account,
            note: v.note,
            supporting_information: v.supporting_information,
        }
    }
}

/// Who performed charged service.
///
/// Indicates who or what performed or participated in the charged service.
/// # Examples
///
/// ```
/// use fhir::r5::resources::charge_item::ChargeItemPerformer;
/// use fhir::r5::types;
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
/// The `ChargeItem.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ChargeItemOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}
