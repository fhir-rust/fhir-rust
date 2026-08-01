//! BiologicallyDerivedProductDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
//!
//! Version: 6.0.0-ballot3
//!
//! A record of dispensation of a biologically derived product
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of dispensation of a biologically derived product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispense;
/// use fhir::r6::types;
///
/// let value = BiologicallyDerivedProductDispense {
///     prepared_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preparedDate` is the name this serializes to on the wire.
/// assert_eq!(json["preparedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: BiologicallyDerivedProductDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BiologicallyDerivedProductDispense {
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

    /// Business identifier for this dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The order or request that this dispense is fulfilling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Short description
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | allocated | issued | unfulfilled | returned
    /// | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::BiologicallyderivedproductdispenseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Relationship between the donor and intended recipient
    pub origin_relationship_type: Option<types::CodeableConcept>,

    /// The BiologicallyDerivedProduct that is dispensed
    pub product: types::Reference,

    /// The intended recipient of the dispensed product
    pub patient: types::Reference,

    /// Indicates the type of matching associated with the dispense
    pub match_status: Option<types::CodeableConcept>,

    /// Indicates who or what performed an action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<BiologicallyDerivedProductDispensePerformer>,

    /// Where the dispense occurred
    pub location: Option<types::Reference>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// When product was selected/matched
    pub prepared_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`prepared_date`](Self::prepared_date) (FHIR `_preparedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparedDate")]
    pub prepared_date_ext: Option<types::Element>,

    /// When the product was dispatched
    pub when_handed_over: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_handed_over`](Self::when_handed_over) (FHIR `_whenHandedOver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_whenHandedOver")]
    pub when_handed_over_ext: Option<types::Element>,

    /// Where the product was dispatched to
    pub destination: Option<types::Reference>,

    /// Additional notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Specific instructions for use
    pub usage_instruction: Option<types::String>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,
}

/// Indicates who or what performed an action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispensePerformer;
/// use fhir::r6::types;
///
/// let value = BiologicallyDerivedProductDispensePerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductDispensePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BiologicallyDerivedProductDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the function of the performer during the dispense
    pub function: Option<types::CodeableConcept>,

    /// Who performed the action
    pub actor: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BiologicallyDerivedProductDispense;

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
