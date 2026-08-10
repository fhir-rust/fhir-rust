//! DataRequirement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
//!
//! Version: 6.0.0-ballot3
//!
//! Describes a required data item
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// DataRequirement Type: Describes a required data item for evaluation in
/// terms of the type of data, and optional code or date-based filters of the
/// data.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::data_requirement::DataRequirement;
/// use fhir::r6::types;
///
/// let value = DataRequirement {
///     limit: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `limit` is the name this serializes to on the wire.
/// assert_eq!(json["limit"], ::serde_json::json!(1));
///
/// let back: DataRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DataRequirementDe")]
#[fhir_version("r6")]
pub struct DataRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The type of the required data
    pub r#type: crate::coded::Coded<crate::r6::codes::FhirTypes>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The profile of the required data
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub profile: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location,
    /// Device
    /// The `DataRequirement.subject[x]` choice element (0..1); see [`DataRequirementSubject`].
    #[serde(flatten)]
    pub subject: Option<DataRequirementSubject>,

    /// Indicates specific structure elements that are referenced by the
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

    /// What values are expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_filter: Vec<DataRequirementValueFilter>,

    /// Number of results
    pub limit: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`limit`](Self::limit) (FHIR `_limit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_limit")]
    pub limit_ext: Option<types::Element>,

    /// Order of the results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<DataRequirementSort>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirementDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r6::codes::FhirTypes>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(default)]
    profile: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_profile")]
    #[serde(default)]
    profile_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    subject: crate::r6::choice::Slot<DataRequirementSubject>,
    #[serde(default)]
    must_support: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_mustSupport")]
    #[serde(default)]
    must_support_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    code_filter: Vec<DataRequirementCodeFilter>,
    #[serde(default)]
    date_filter: Vec<DataRequirementDateFilter>,
    #[serde(default)]
    value_filter: Vec<DataRequirementValueFilter>,
    limit: Option<types::PositiveInt>,
    #[serde(rename = "_limit")]
    limit_ext: Option<types::Element>,
    #[serde(default)]
    sort: Vec<DataRequirementSort>,
}

impl ::core::convert::From<DataRequirementDe> for DataRequirement {
    fn from(v: DataRequirementDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            profile: v.profile,
            profile_ext: v.profile_ext,
            subject: v.subject.0,
            must_support: v.must_support,
            must_support_ext: v.must_support_ext,
            code_filter: v.code_filter,
            date_filter: v.date_filter,
            value_filter: v.value_filter,
            limit: v.limit,
            limit_ext: v.limit_ext,
            sort: v.sort,
        }
    }
}

/// Code filters specify additional constraints on the data, specifying the
/// value set of interest for a particular element of the data. Each code
/// filter defines an additional constraint on the data, i.e. code filters are
/// AND'ed, not OR'ed.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::data_requirement::DataRequirementCodeFilter;
/// use fhir::r6::types;
///
/// let value = DataRequirementCodeFilter {
///     search_param: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `searchParam` is the name this serializes to on the wire.
/// assert_eq!(json["searchParam"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementCodeFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DataRequirementCodeFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// A code-valued attribute to filter on
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A coded (token) parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

    /// ValueSet for the filter
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// What code is expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,
}

/// Date filters specify additional constraints on the data in terms of the
/// applicable date range for specific elements. Each date filter specifies an
/// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::data_requirement::DataRequirementDateFilter;
/// use fhir::r6::types;
///
/// let value = DataRequirementDateFilter {
///     search_param: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `searchParam` is the name this serializes to on the wire.
/// assert_eq!(json["searchParam"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementDateFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DataRequirementDateFilterDe")]
#[fhir_version("r6")]
pub struct DataRequirementDateFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// A date-valued attribute to filter on
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A date valued parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

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
    path: Option<types::String>,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    search_param: Option<types::String>,
    #[serde(rename = "_searchParam")]
    search_param_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<DataRequirementDateFilterValue>,
}

impl ::core::convert::From<DataRequirementDateFilterDe> for DataRequirementDateFilter {
    fn from(v: DataRequirementDateFilterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            path: v.path,
            path_ext: v.path_ext,
            search_param: v.search_param,
            search_param_ext: v.search_param_ext,
            value: v.value.0,
        }
    }
}

/// Specifies the order of the results to be returned.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::data_requirement::DataRequirementSort;
/// use fhir::r6::types;
///
/// let value = DataRequirementSort {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementSort = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DataRequirementSort {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The name of the attribute to perform the sort
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// ascending | descending
    pub direction: crate::coded::Coded<crate::r6::codes::SortDirection>,
    /// Primitive extension sibling for [`direction`](Self::direction) (FHIR `_direction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_direction")]
    pub direction_ext: Option<types::Element>,
}

/// Value filters specify additional constraints on the data for elements other
/// than code-valued or date-valued. Each value filter specifies an additional
/// constraint on the data (i.e. valueFilters are AND'ed, not OR'ed).
///
/// # Examples
///
/// ```
/// use fhir::r6::types::data_requirement::DataRequirementValueFilter;
/// use fhir::r6::types;
///
/// let value = DataRequirementValueFilter {
///     search_param: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `searchParam` is the name this serializes to on the wire.
/// assert_eq!(json["searchParam"], ::serde_json::json!("abc"));
///
/// let back: DataRequirementValueFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DataRequirementValueFilterDe")]
#[fhir_version("r6")]
pub struct DataRequirementValueFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// An attribute to filter on
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// A parameter to search on
    pub search_param: Option<types::String>,
    /// Primitive extension sibling for [`search_param`](Self::search_param) (FHIR `_searchParam`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchParam")]
    pub search_param_ext: Option<types::Element>,

    /// eq | gt | lt | ge | le | sa | eb
    pub comparator: Option<types::Code>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,

    /// The value of the filter, as a Period, DateTime, or Duration value
    /// The `DataRequirement.valueFilter.value[x]` choice element (0..1); see [`DataRequirementValueFilterValue`].
    #[serde(flatten)]
    pub value: Option<DataRequirementValueFilterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirementValueFilterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    path: Option<types::String>,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    search_param: Option<types::String>,
    #[serde(rename = "_searchParam")]
    search_param_ext: Option<types::Element>,
    comparator: Option<types::Code>,
    #[serde(rename = "_comparator")]
    comparator_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<DataRequirementValueFilterValue>,
}

impl ::core::convert::From<DataRequirementValueFilterDe> for DataRequirementValueFilter {
    fn from(v: DataRequirementValueFilterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            path: v.path,
            path_ext: v.path_ext,
            search_param: v.search_param,
            search_param_ext: v.search_param_ext,
            comparator: v.comparator,
            comparator_ext: v.comparator_ext,
            value: v.value.0,
        }
    }
}

/// The `DataRequirement.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `DataRequirement.dateFilter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementDateFilterValue {
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
}

/// The `DataRequirement.valueFilter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DataRequirementValueFilterValue {
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
