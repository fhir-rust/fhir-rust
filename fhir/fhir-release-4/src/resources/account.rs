//! Account
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Account
//!
//! Version: 4.0.1
//!
//! Tracks balance, charges, for patient or cost center
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A financial tool for tracking value accrued for a particular purpose. In
/// the healthcare field, used to track charges for a patient, cost centers,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::account::Account;
/// use fhir::r4::types;
///
/// let value = Account {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: Account = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Account {
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

    /// Account number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | entered-in-error | on-hold | unknown
    pub status: crate::coded::Coded<crate::r4::codes::AccountStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// E.g. patient, expense, depreciation
    pub r#type: Option<types::CodeableConcept>,

    /// Human-readable label
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The entity that caused the expenses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Transaction window
    pub service_period: Option<types::Period>,

    /// The party(s) that are responsible for covering the payment of this
    /// account, and what order should they be applied to the account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<AccountCoverage>,

    /// Entity managing the Account
    pub owner: Option<types::Reference>,

    /// Explanation of purpose/use
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The parties ultimately responsible for balancing the Account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guarantor: Vec<AccountGuarantor>,

    /// Reference to a parent Account
    pub part_of: Option<types::Reference>,
}

/// The party(s) that are responsible for covering the payment of this account,
/// and what order should they be applied to the account.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::account::AccountCoverage;
/// use fhir::r4::types;
///
/// let value = AccountCoverage {
///     priority: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `priority` is the name this serializes to on the wire.
/// assert_eq!(json["priority"], ::serde_json::json!(1));
///
/// let back: AccountCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AccountCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The party(s), such as insurances, that may contribute to the payment of
    /// this account
    pub coverage: types::Reference,

    /// The priority of the coverage in the context of this account
    pub priority: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,
}

/// The parties responsible for balancing the account if other payment options
/// fall short.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::account::AccountGuarantor;
/// use fhir::r4::types;
///
/// let value = AccountGuarantor {
///     on_hold: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `onHold` is the name this serializes to on the wire.
/// assert_eq!(json["onHold"], ::serde_json::json!(true));
///
/// let back: AccountGuarantor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AccountGuarantor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Responsible entity
    pub party: types::Reference,

    /// Credit or other hold applied
    pub on_hold: Option<types::Boolean>,
    /// Primitive extension sibling for [`on_hold`](Self::on_hold) (FHIR `_onHold`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_onHold")]
    pub on_hold_ext: Option<types::Element>,

    /// Guarantee account during
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Account;

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
