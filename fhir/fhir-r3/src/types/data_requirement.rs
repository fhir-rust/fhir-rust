//! DataRequirement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
//!
//!
//!
//! Describes a required data item
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DataRequirement Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::data_requirement::DataRequirement;
/// use fhir::r3::types;
///
/// let value = DataRequirement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DataRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DataRequirement {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The type of the required data
    pub r#type: types::Code,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The profile of the required data
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub profile: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Indicates that specific structure elements are referenced by the
    /// knowledge module
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub must_support: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`must_support`](Self::must_support) (FHIR `_mustSupport`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mustSupport")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_support_ext: Vec<Option<types::Element>>,

    /// What codes are expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_filter: Vec<DataRequirementCodeFilter>,

    /// What dates/date ranges are expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_filter: Vec<DataRequirementDateFilter>,
}

/// Code filters specify additional constraints on the data, specifying the
/// value set of interest for a particular element of the data.
///
/// # Examples
///
/// ```
/// use fhir::r3::types::data_requirement::DataRequirementCodeFilter;
/// use fhir::r3::types;
///
/// let value = DataRequirementCodeFilter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementCodeFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DataRequirementCodeFilterDe")]
#[fhir_version("r3")]
pub struct DataRequirementCodeFilter {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The code-valued attribute of the filter
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Valueset for the filter
    /// The `DataRequirement.codeFilter.valueSet[x]` choice element (0..1); see [`DataRequirementCodeFilterValueSet`].
    #[serde(flatten)]
    pub value_set: Option<DataRequirementCodeFilterValueSet>,

    /// What code is expected
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub value_code: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`value_code`](Self::value_code) (FHIR `_valueCode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueCode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_code_ext: Vec<Option<types::Element>>,

    /// What Coding is expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_coding: Vec<types::Coding>,

    /// What CodeableConcept is expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_codeable_concept: Vec<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirementCodeFilterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    path: types::String,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    #[serde(flatten)]
    value_set: crate::r3::choice::Slot<DataRequirementCodeFilterValueSet>,
    #[serde(default)]
    value_code: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_valueCode")]
    #[serde(default)]
    value_code_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    value_coding: Vec<types::Coding>,
    #[serde(default)]
    value_codeable_concept: Vec<types::CodeableConcept>,
}

impl ::core::convert::From<DataRequirementCodeFilterDe> for DataRequirementCodeFilter {
    fn from(v: DataRequirementCodeFilterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            path: v.path,
            path_ext: v.path_ext,
            value_set: v.value_set.0,
            value_code: v.value_code,
            value_code_ext: v.value_code_ext,
            value_coding: v.value_coding,
            value_codeable_concept: v.value_codeable_concept,
        }
    }
}

/// Date filters specify additional constraints on the data in terms of the
/// applicable date range for specific elements.
///
/// # Examples
///
/// ```
/// use fhir::r3::types::data_requirement::DataRequirementDateFilter;
/// use fhir::r3::types;
///
/// let value = DataRequirementDateFilter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementDateFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DataRequirementDateFilterDe")]
#[fhir_version("r3")]
pub struct DataRequirementDateFilter {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The date-valued attribute of the filter
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    /// The `DataRequirement.dateFilter.value[x]` choice element (0..1); see [`DataRequirementDateFilterValue`].
    #[serde(flatten)]
    pub value: Option<DataRequirementDateFilterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirementDateFilterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    path: types::String,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r3::choice::Slot<DataRequirementDateFilterValue>,
}

impl ::core::convert::From<DataRequirementDateFilterDe> for DataRequirementDateFilter {
    fn from(v: DataRequirementDateFilterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            path: v.path,
            path_ext: v.path_ext,
            value: v.value.0,
        }
    }
}

/// The `DataRequirement.codeFilter.valueSet[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementCodeFilterValueSet {
    /// `valueSetString` variant.
    #[fhir("valueSetString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueSetReference` variant.
    #[fhir("valueSetReference")]
    Reference(Box<types::Reference>),
}

/// The `DataRequirement.dateFilter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementDateFilterValue {
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DataRequirement;

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
