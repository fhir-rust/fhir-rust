//! Account
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Account
//!
//! Version: 5.0.0
//!
//! Account Resource: A financial tool for tracking value accrued for a particular purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A financial tool for tracking value accrued for a particular purpose. In the
/// healthcare field, an Account is used to track charges for a patient, cost
/// centers, etc. It aggregates the financial context (coverage, guarantors,
/// diagnoses, procedures, and balances) needed to bill and reconcile the value
/// accrued against a subject such as a patient, encounter, or organization.
///
/// An Account represents a grouping of financial transactions and is used
/// administratively to track charges, payments, and adjustments over a
/// service period. It is the FHIR analog of a patient or departmental ledger
/// and is commonly created when a patient is admitted or registered, then
/// referenced by billable events (such as procedures, encounters, or
/// invoices) as they occur. An Account may track one or more subjects, list
/// the coverages and guarantors responsible for payment, and roll up
/// calculated balances, but it does not itself represent the individual
/// line-item transactions, which are typically recorded by other resources
/// such as ChargeItem or Invoice and linked back to the Account.
///
/// # Related resources
///
/// See also [`Reference`](crate::r5::types::Reference), used to link an
/// Account to its subject, owner, guarantor, and related accounts, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), used for the
/// account type, billing status, and currency. Related resources referenced
/// by an Account commonly include `Patient`, `Encounter`, `Organization`,
/// `Coverage`, `RelatedPerson`, `ChargeItem`, and `Invoice`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::Account;
/// use fhir::r5::types;
///
/// let value = Account {
///     calculated_at: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `calculatedAt` is the name this serializes to on the wire.
/// assert_eq!(json["calculatedAt"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Account = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Account {
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

    /// Account number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Indicates whether the account is currently in use: active | inactive |
    /// entered-in-error | on-hold | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::AccountStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Tracks the lifecycle of the account through the billing process
    pub billing_status: Option<types::CodeableConcept>,

    /// E.g. patient, expense, depreciation
    pub r#type: Option<types::CodeableConcept>,

    /// Human-readable label
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The entity that caused the expenses, such as a `Patient` or `Device`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Transaction window
    pub service_period: Option<types::Period>,

    /// The party(s) that are responsible for covering the payment of this
    /// account, and what order should they be applied to the account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<AccountCoverage>,

    /// Entity managing the Account
    pub owner: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Explanation of purpose/use
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The parties, typically the patient or a family member, ultimately
    /// responsible for balancing the Account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guarantor: Vec<AccountGuarantor>,

    /// The list of diagnoses relevant to this account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<AccountDiagnosis>,

    /// The list of procedures relevant to this account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<AccountProcedure>,

    /// Other associated accounts related to this account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_account: Vec<AccountRelatedAccount>,

    /// The base or default currency
    pub currency: Option<types::CodeableConcept>,

    /// Calculated account balance(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub balance: Vec<AccountBalance>,

    /// Time the balance amount was calculated
    pub calculated_at: Option<types::Instant>,
    /// Primitive extension sibling for [`calculated_at`](Self::calculated_at) (FHIR `_calculatedAt`).
    #[serde(rename = "_calculatedAt")]
    pub calculated_at_ext: Option<types::Element>,
}

/// The party(s) that are responsible for covering the payment of this account,
/// and what order should they be applied to the account.
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountCoverage;
/// use fhir::r5::types;
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
    pub coverage: types::Reference<crate::r5::resources::Coverage>,

    /// The priority of the coverage in the context of this account
    pub priority: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,
}

/// The parties ultimately responsible for balancing the Account.
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountGuarantor;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`on_hold`](Self::on_hold) (FHIR `_onHold`).
    #[serde(rename = "_onHold")]
    pub on_hold_ext: Option<types::Element>,

    /// Guarantee account during
    pub period: Option<types::Period>,
}

/// The list of diagnoses relevant to this account.
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountDiagnosis;
/// use fhir::r5::types;
///
/// let value = AccountDiagnosis {
///     date_of_diagnosis: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateOfDiagnosis` is the name this serializes to on the wire.
/// assert_eq!(json["dateOfDiagnosis"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AccountDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Ranking of the diagnosis (for each type)
    pub sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The diagnosis relevant to the account
    pub condition: types::CodeableReference,

    /// Date of the diagnosis (when coded diagnosis)
    pub date_of_diagnosis: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_of_diagnosis`](Self::date_of_diagnosis) (FHIR `_dateOfDiagnosis`).
    #[serde(rename = "_dateOfDiagnosis")]
    pub date_of_diagnosis_ext: Option<types::Element>,

    /// Type that this diagnosis has relevant to the account (e.g. admission,
    /// billing, discharge …)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Diagnosis present on Admission
    pub on_admission: Option<types::Boolean>,
    /// Primitive extension sibling for [`on_admission`](Self::on_admission) (FHIR `_onAdmission`).
    #[serde(rename = "_onAdmission")]
    pub on_admission_ext: Option<types::Element>,

    /// Package Code specific for billing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_code: Vec<types::CodeableConcept>,
}

/// The list of procedures relevant to this account.
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountProcedure;
/// use fhir::r5::types;
///
/// let value = AccountProcedure {
///     date_of_service: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateOfService` is the name this serializes to on the wire.
/// assert_eq!(json["dateOfService"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AccountProcedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Ranking of the procedure (for each type)
    pub sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The procedure relevant to the account
    pub code: types::CodeableReference,

    /// Date of the procedure (when coded procedure)
    pub date_of_service: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_of_service`](Self::date_of_service) (FHIR `_dateOfService`).
    #[serde(rename = "_dateOfService")]
    pub date_of_service_ext: Option<types::Element>,

    /// How this procedure value should be used in charging the account
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Package Code specific for billing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_code: Vec<types::CodeableConcept>,

    /// Any devices that were associated with the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference<crate::r5::resources::Device>>,
}

/// Other associated accounts related to this account.
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountRelatedAccount;
/// use fhir::r5::types;
///
/// let value = AccountRelatedAccount {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AccountRelatedAccount = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelatedAccount {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Relationship of the associated Account
    pub relationship: Option<types::CodeableConcept>,

    /// Reference to an associated Account
    pub account: types::Reference<crate::r5::resources::Account>,
}

/// Calculated account balance(s).
/// # Examples
///
/// ```
/// use fhir::r5::resources::account::AccountBalance;
/// use fhir::r5::types;
///
/// let value = AccountBalance {
///     estimate: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `estimate` is the name this serializes to on the wire.
/// assert_eq!(json["estimate"], ::serde_json::json!(true));
///
/// let back: AccountBalance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who is expected to pay this part of the balance
    pub aggregate: Option<types::CodeableConcept>,

    /// current | 30 | 60 | 90 | 120
    pub term: Option<types::CodeableConcept>,

    /// Estimated balance
    pub estimate: Option<types::Boolean>,
    /// Primitive extension sibling for [`estimate`](Self::estimate) (FHIR `_estimate`).
    #[serde(rename = "_estimate")]
    pub estimate_ext: Option<types::Element>,

    /// Calculated amount
    pub amount: types::Money,
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
