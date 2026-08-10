//! ClinicalImpression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
//!
//!
//!
//! A clinical assessment performed when planning treatments and management
//! strategies for a patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ClinicalImpression Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::clinical_impression::ClinicalImpression;
/// use fhir::r3::types;
///
/// let value = ClinicalImpression {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpression = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ClinicalImpressionDe")]
#[fhir_version("r3")]
pub struct ClinicalImpression {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | completed | entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::ClinicalImpressionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of assessment performed
    pub code: Option<types::CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Patient or group assessed
    pub subject: types::Reference,

    /// Encounter or Episode created from
    pub context: Option<types::Reference>,

    /// Time of assessment
    /// The `ClinicalImpression.effective[x]` choice element (0..1); see [`ClinicalImpressionEffective`].
    #[serde(flatten)]
    pub effective: Option<ClinicalImpressionEffective>,

    /// When the assessment was documented
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The clinician performing the assessment
    pub assessor: Option<types::Reference<crate::r3::resources::Practitioner>>,

    /// Reference to last assessment
    pub previous: Option<types::Reference<crate::r3::resources::ClinicalImpression>>,

    /// Relevant impressions of patient state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<types::Reference>,

    /// One or more sets of investigations (signs, symptions, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub investigation: Vec<ClinicalImpressionInvestigation>,

    /// Clinical Protocol followed
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub protocol: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`protocol`](Self::protocol) (FHIR `_protocol`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_protocol")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_ext: Vec<Option<types::Element>>,

    /// Summary of the assessment
    pub summary: Option<types::String>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_summary")]
    pub summary_ext: Option<types::Element>,

    /// Possible or likely findings and diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finding: Vec<ClinicalImpressionFinding>,

    /// Estimate of likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_codeable_concept: Vec<types::CodeableConcept>,

    /// RiskAssessment expressing likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_reference: Vec<types::Reference<crate::r3::resources::RiskAssessment>>,

    /// Action taken as part of assessment procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::Reference>,

    /// Comments made about the ClinicalImpression
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClinicalImpressionDe {
    id: Option<types::Id>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r3::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r3::codes::ClinicalImpressionStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    subject: types::Reference,
    context: Option<types::Reference>,
    #[serde(flatten)]
    effective: crate::r3::choice::Slot<ClinicalImpressionEffective>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    assessor: Option<types::Reference<crate::r3::resources::Practitioner>>,
    previous: Option<types::Reference<crate::r3::resources::ClinicalImpression>>,
    #[serde(default)]
    problem: Vec<types::Reference>,
    #[serde(default)]
    investigation: Vec<ClinicalImpressionInvestigation>,
    #[serde(default)]
    protocol: ::fhir_core::PrimVec<types::Uri>,
    #[serde(rename = "_protocol")]
    #[serde(default)]
    protocol_ext: Vec<Option<types::Element>>,
    summary: Option<types::String>,
    #[serde(rename = "_summary")]
    summary_ext: Option<types::Element>,
    #[serde(default)]
    finding: Vec<ClinicalImpressionFinding>,
    #[serde(default)]
    prognosis_codeable_concept: Vec<types::CodeableConcept>,
    #[serde(default)]
    prognosis_reference: Vec<types::Reference<crate::r3::resources::RiskAssessment>>,
    #[serde(default)]
    action: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<ClinicalImpressionDe> for ClinicalImpression {
    fn from(v: ClinicalImpressionDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            code: v.code,
            description: v.description,
            description_ext: v.description_ext,
            subject: v.subject,
            context: v.context,
            effective: v.effective.0,
            date: v.date,
            date_ext: v.date_ext,
            assessor: v.assessor,
            previous: v.previous,
            problem: v.problem,
            investigation: v.investigation,
            protocol: v.protocol,
            protocol_ext: v.protocol_ext,
            summary: v.summary,
            summary_ext: v.summary_ext,
            finding: v.finding,
            prognosis_codeable_concept: v.prognosis_codeable_concept,
            prognosis_reference: v.prognosis_reference,
            action: v.action,
            note: v.note,
        }
    }
}

/// Specific findings or diagnoses that was considered likely or relevant to
/// ongoing treatment.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::clinical_impression::ClinicalImpressionFinding;
/// use fhir::r3::types;
///
/// let value = ClinicalImpressionFinding {
///     basis: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `basis` is the name this serializes to on the wire.
/// assert_eq!(json["basis"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionFinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ClinicalImpressionFindingDe")]
#[fhir_version("r3")]
pub struct ClinicalImpressionFinding {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What was found
    /// The `ClinicalImpression.finding.item[x]` choice element (1..1); see [`ClinicalImpressionFindingItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub item: Option<ClinicalImpressionFindingItem>,

    /// Which investigations support finding
    pub basis: Option<types::String>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClinicalImpressionFindingDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    item: crate::r3::choice::Slot<ClinicalImpressionFindingItem>,
    basis: Option<types::String>,
    #[serde(rename = "_basis")]
    basis_ext: Option<types::Element>,
}

impl ::core::convert::From<ClinicalImpressionFindingDe> for ClinicalImpressionFinding {
    fn from(v: ClinicalImpressionFindingDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            item: v.item.0,
            basis: v.basis,
            basis_ext: v.basis_ext,
        }
    }
}

/// One or more sets of investigations (signs, symptions, etc.). The actual
/// grouping of investigations vary greatly depending on the type and context
/// of the assessment. These investigations may include data generated during
/// the assessment process, or data previously generated and recorded that is
/// pertinent to the outcomes.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::clinical_impression::ClinicalImpressionInvestigation;
/// use fhir::r3::types;
///
/// let value = ClinicalImpressionInvestigation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionInvestigation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClinicalImpressionInvestigation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name/code for the set
    pub code: types::CodeableConcept,

    /// Record of a specific investigation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<types::Reference>,
}

/// The `ClinicalImpression.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalImpressionEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `ClinicalImpression.finding.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalImpressionFindingItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClinicalImpression;

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
