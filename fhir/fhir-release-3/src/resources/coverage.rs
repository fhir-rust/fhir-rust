//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//!
//!
//! Insurance or medical plan or a payment agreement
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Coverage Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::coverage::Coverage;
/// use fhir::r3::types;
///
/// let value = Coverage {
///     subscriber_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `subscriberId` is the name this serializes to on the wire.
/// assert_eq!(json["subscriberId"], ::serde_json::json!("abc"));
///
/// let back: Coverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Coverage {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The primary coverage ID
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of coverage such as medical or accident
    pub r#type: Option<types::CodeableConcept>,

    /// Owner of the policy
    pub policy_holder: Option<types::Reference>,

    /// Subscriber to the policy
    pub subscriber: Option<types::Reference>,

    /// ID assigned to the Subscriber
    pub subscriber_id: Option<types::String>,
    /// Primitive extension sibling for [`subscriber_id`](Self::subscriber_id) (FHIR `_subscriberId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subscriberId")]
    pub subscriber_id_ext: Option<types::Element>,

    /// Plan Beneficiary
    pub beneficiary: Option<types::Reference>,

    /// Beneficiary relationship to the Subscriber
    pub relationship: Option<types::CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Identifier for the plan or agreement issuer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payor: Vec<types::Reference>,

    /// Additional coverage classifications
    pub grouping: Option<CoverageGrouping>,

    /// Dependent number
    pub dependent: Option<types::String>,
    /// Primitive extension sibling for [`dependent`](Self::dependent) (FHIR `_dependent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dependent")]
    pub dependent_ext: Option<types::Element>,

    /// The plan instance or sequence counter
    pub sequence: Option<types::String>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Relative order of the coverage
    pub order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`order`](Self::order) (FHIR `_order`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_order")]
    pub order_ext: Option<types::Element>,

    /// Insurer network
    pub network: Option<types::String>,
    /// Primitive extension sibling for [`network`](Self::network) (FHIR `_network`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_network")]
    pub network_ext: Option<types::Element>,

    /// Contract details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<types::Reference>,
}

/// A suite of underwrite specific classifiers, for example may be used to
/// identify a class of coverage or employer group, Policy, Plan.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::coverage::CoverageGrouping;
/// use fhir::r3::types;
///
/// let value = CoverageGrouping {
///     group_display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `groupDisplay` is the name this serializes to on the wire.
/// assert_eq!(json["groupDisplay"], ::serde_json::json!("abc"));
///
/// let back: CoverageGrouping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CoverageGrouping {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for the group
    pub group: Option<types::String>,
    /// Primitive extension sibling for [`group`](Self::group) (FHIR `_group`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_group")]
    pub group_ext: Option<types::Element>,

    /// Display text for an identifier for the group
    pub group_display: Option<types::String>,
    /// Primitive extension sibling for [`group_display`](Self::group_display) (FHIR `_groupDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupDisplay")]
    pub group_display_ext: Option<types::Element>,

    /// An identifier for the subsection of the group
    pub sub_group: Option<types::String>,
    /// Primitive extension sibling for [`sub_group`](Self::sub_group) (FHIR `_subGroup`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subGroup")]
    pub sub_group_ext: Option<types::Element>,

    /// Display text for the subsection of the group
    pub sub_group_display: Option<types::String>,
    /// Primitive extension sibling for [`sub_group_display`](Self::sub_group_display) (FHIR `_subGroupDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subGroupDisplay")]
    pub sub_group_display_ext: Option<types::Element>,

    /// An identifier for the plan
    pub plan: Option<types::String>,
    /// Primitive extension sibling for [`plan`](Self::plan) (FHIR `_plan`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plan")]
    pub plan_ext: Option<types::Element>,

    /// Display text for the plan
    pub plan_display: Option<types::String>,
    /// Primitive extension sibling for [`plan_display`](Self::plan_display) (FHIR `_planDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_planDisplay")]
    pub plan_display_ext: Option<types::Element>,

    /// An identifier for the subsection of the plan
    pub sub_plan: Option<types::String>,
    /// Primitive extension sibling for [`sub_plan`](Self::sub_plan) (FHIR `_subPlan`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subPlan")]
    pub sub_plan_ext: Option<types::Element>,

    /// Display text for the subsection of the plan
    pub sub_plan_display: Option<types::String>,
    /// Primitive extension sibling for [`sub_plan_display`](Self::sub_plan_display) (FHIR `_subPlanDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subPlanDisplay")]
    pub sub_plan_display_ext: Option<types::Element>,

    /// An identifier for the class
    pub class: Option<types::String>,
    /// Primitive extension sibling for [`class`](Self::class) (FHIR `_class`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_class")]
    pub class_ext: Option<types::Element>,

    /// Display text for the class
    pub class_display: Option<types::String>,
    /// Primitive extension sibling for [`class_display`](Self::class_display) (FHIR `_classDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_classDisplay")]
    pub class_display_ext: Option<types::Element>,

    /// An identifier for the subsection of the class
    pub sub_class: Option<types::String>,
    /// Primitive extension sibling for [`sub_class`](Self::sub_class) (FHIR `_subClass`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subClass")]
    pub sub_class_ext: Option<types::Element>,

    /// Display text for the subsection of the subclass
    pub sub_class_display: Option<types::String>,
    /// Primitive extension sibling for [`sub_class_display`](Self::sub_class_display) (FHIR `_subClassDisplay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subClassDisplay")]
    pub sub_class_display_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coverage;

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
