//! FormularyItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/FormularyItem
//!
//! Version: 5.0.0
//!
//! FormularyItem Resource: This resource describes a product or service that is available through a program and includes the conditions and constraints of availability.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource describes a product or service that is available through a
/// program and includes the conditions and constraints of availability. All of
/// the information in this resource is specific to the inclusion of the item in
/// the formulary and is not inherent to the item itself. A FormularyItem
/// typically references a medication or other item and captures administrative
/// details such as its business identifier, a code that identifies it, and its
/// status within the formulary.
///
/// In practice, a formulary is a curated list of drugs, devices, or other
/// products that an organization (such as a hospital, pharmacy benefit
/// manager, or health plan) has approved for use or reimbursement. Each
/// `FormularyItem` acts as an administrative wrapper around a catalog entry,
/// distinct from the clinical definition of the underlying product itself
/// (for example a medication resource), and is used by ordering, dispensing,
/// and billing systems to determine whether an item is currently available
/// and under what status. The resource is intentionally lightweight: it does
/// not carry pricing, formulary tier, or coverage rules, which are typically
/// modeled by related resources or extensions in an implementation guide.
///
/// # Related resources
///
/// A `FormularyItem` is commonly referenced from, or referenced alongside,
/// `MedicationKnowledge` and other product-catalog resources, and may be
/// linked to a `Patient`'s benefit plan through coverage-related resources.
/// The identifying `code` field typically uses a
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::formulary_item::FormularyItem;
/// use fhir::r5::types;
///
/// let value = FormularyItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: FormularyItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FormularyItem {
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

    /// Business identifier(s) assigned to this formulary item by the formulary-managing organization or other systems.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Codes that identify this formulary item, such as a drug or product code, used to match the item to its underlying definition.
    pub code: Option<types::CodeableConcept>,

    /// The current status of this formulary item entry: active | entered-in-error | inactive.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::FormularyitemStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = FormularyItem;

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
