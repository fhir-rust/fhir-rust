//! ChargeItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
//!
//! Version: 4.0.1
//!
//! Item containing charge code(s) associated with the provision of healthcare
//! provider products
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
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
/// use fhir::r4::resources::charge_item::ChargeItem;
/// use fhir::r4::types;
///
/// let value = ChargeItem {
///     override_reason: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `overrideReason` is the name this serializes to on the wire.
/// assert_eq!(json["overrideReason"], ::serde_json::json!("abc"));
///
/// let back: ChargeItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ChargeItemDe")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

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
    /// Primitive extension sibling for [`definition_uri`](Self::definition_uri) (FHIR `_definitionUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definitionUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_uri_ext: Vec<Option<types::Element>>,

    /// Resource defining the code of this ChargeItem
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub definition_canonical: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`definition_canonical`](Self::definition_canonical) (FHIR `_definitionCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definitionCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_canonical_ext: Vec<Option<types::Element>>,

    /// planned | billable | not-billable | aborted | billed | entered-in-error
    /// | unknown
    pub status: crate::coded::Coded<crate::r4::codes::ChargeitemStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Part of referenced ChargeItem
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r4::resources::ChargeItem>>,

    /// A code that identifies the charge, like a billing code
    pub code: types::CodeableConcept,

    /// Individual service was done for/to
    pub subject: types::Reference,

    /// Encounter / Episode associated with event
    pub context: Option<types::Reference>,

    /// When the charged service was applied
    /// The `ChargeItem.occurrence[x]` choice element (0..1); see [`ChargeItemOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ChargeItemOccurrence>,

    /// Who performed charged service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ChargeItemPerformer>,

    /// Organization providing the charged service
    pub performing_organization: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Organization requesting the charged service
    pub requesting_organization: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Organization that has ownership of the (potential, future) revenue
    pub cost_center: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Quantity of which the charge item has been serviced
    pub quantity: Option<types::Quantity>,

    /// Anatomical location, if relevant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bodysite: Vec<types::CodeableConcept>,

    /// Factor overriding the associated rules
    pub factor_override: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor_override`](Self::factor_override) (FHIR `_factorOverride`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factorOverride")]
    pub factor_override_ext: Option<types::Element>,

    /// Price overriding the associated rules
    pub price_override: Option<types::Money>,

    /// Reason for overriding the list price/factor
    pub override_reason: Option<types::String>,
    /// Primitive extension sibling for [`override_reason`](Self::override_reason) (FHIR `_overrideReason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_overrideReason")]
    pub override_reason_ext: Option<types::Element>,

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
    pub reason: Vec<types::CodeableConcept>,

    /// Which rendered service is being charged?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<types::Reference>,

    /// Product charged
    /// The `ChargeItem.product[x]` choice element (0..1); see [`ChargeItemProduct`].
    #[serde(flatten)]
    pub product: Option<ChargeItemProduct>,

    /// Account to place this charge
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r4::resources::Account>>,

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
    contained: Vec<crate::r4::resources::Resource>,
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
    status: crate::coded::Coded<crate::r4::codes::ChargeitemStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    part_of: Vec<types::Reference<crate::r4::resources::ChargeItem>>,
    code: types::CodeableConcept,
    subject: types::Reference,
    context: Option<types::Reference>,
    #[serde(flatten)]
    occurrence: crate::r4::choice::Slot<ChargeItemOccurrence>,
    #[serde(default)]
    performer: Vec<ChargeItemPerformer>,
    performing_organization: Option<types::Reference<crate::r4::resources::Organization>>,
    requesting_organization: Option<types::Reference<crate::r4::resources::Organization>>,
    cost_center: Option<types::Reference<crate::r4::resources::Organization>>,
    quantity: Option<types::Quantity>,
    #[serde(default)]
    bodysite: Vec<types::CodeableConcept>,
    factor_override: Option<types::Decimal>,
    #[serde(rename = "_factorOverride")]
    factor_override_ext: Option<types::Element>,
    price_override: Option<types::Money>,
    override_reason: Option<types::String>,
    #[serde(rename = "_overrideReason")]
    override_reason_ext: Option<types::Element>,
    enterer: Option<types::Reference>,
    entered_date: Option<types::DateTime>,
    #[serde(rename = "_enteredDate")]
    entered_date_ext: Option<types::Element>,
    #[serde(default)]
    reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    service: Vec<types::Reference>,
    #[serde(flatten)]
    product: crate::r4::choice::Slot<ChargeItemProduct>,
    #[serde(default)]
    account: Vec<types::Reference<crate::r4::resources::Account>>,
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
            context: v.context,
            occurrence: v.occurrence.0,
            performer: v.performer,
            performing_organization: v.performing_organization,
            requesting_organization: v.requesting_organization,
            cost_center: v.cost_center,
            quantity: v.quantity,
            bodysite: v.bodysite,
            factor_override: v.factor_override,
            factor_override_ext: v.factor_override_ext,
            price_override: v.price_override,
            override_reason: v.override_reason,
            override_reason_ext: v.override_reason_ext,
            enterer: v.enterer,
            entered_date: v.entered_date,
            entered_date_ext: v.entered_date_ext,
            reason: v.reason,
            service: v.service,
            product: v.product.0,
            account: v.account,
            note: v.note,
            supporting_information: v.supporting_information,
        }
    }
}

/// Indicates who or what performed or participated in the charged service.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::charge_item::ChargeItemPerformer;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ChargeItemOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `ChargeItem.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ChargeItemProduct {
    /// `productReference` variant.
    #[fhir("productReference")]
    Reference(Box<types::Reference>),
    /// `productCodeableConcept` variant.
    #[fhir("productCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
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
