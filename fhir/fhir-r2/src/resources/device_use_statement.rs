//! DeviceUseStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceUseStatement
//!
//!
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DeviceUseStatement Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::device_use_statement::DeviceUseStatement;
/// use fhir::r2::types;
///
/// let value = DeviceUseStatement {
///     recorded_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recordedOn` is the name this serializes to on the wire.
/// assert_eq!(json["recordedOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceUseStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DeviceUseStatementDe")]
#[fhir_version("r2")]
pub struct DeviceUseStatement {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Target body site
    /// The `DeviceUseStatement.bodySite[x]` choice element (0..1); see [`DeviceUseStatementBodySite`].
    #[serde(flatten)]
    pub body_site: Option<DeviceUseStatementBodySite>,

    pub when_used: Option<types::Period>,

    pub device: types::Reference<crate::r2::resources::Device>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableConcept>,

    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub notes: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`notes`](Self::notes) (FHIR `_notes`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notes")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes_ext: Vec<Option<types::Element>>,

    pub recorded_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_on`](Self::recorded_on) (FHIR `_recordedOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedOn")]
    pub recorded_on_ext: Option<types::Element>,

    pub subject: types::Reference<crate::r2::resources::Patient>,

    /// The `DeviceUseStatement.timing[x]` choice element (0..1); see [`DeviceUseStatementTiming`].
    #[serde(flatten)]
    pub timing: Option<DeviceUseStatementTiming>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceUseStatementDe {
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
    contained: Vec<crate::r2::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    body_site: crate::r2::choice::Slot<DeviceUseStatementBodySite>,
    when_used: Option<types::Period>,
    device: types::Reference<crate::r2::resources::Device>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    indication: Vec<types::CodeableConcept>,
    #[serde(default)]
    notes: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_notes")]
    #[serde(default)]
    notes_ext: Vec<Option<types::Element>>,
    recorded_on: Option<types::DateTime>,
    #[serde(rename = "_recordedOn")]
    recorded_on_ext: Option<types::Element>,
    subject: types::Reference<crate::r2::resources::Patient>,
    #[serde(flatten)]
    timing: crate::r2::choice::Slot<DeviceUseStatementTiming>,
}

impl ::core::convert::From<DeviceUseStatementDe> for DeviceUseStatement {
    fn from(v: DeviceUseStatementDe) -> Self {
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
            body_site: v.body_site.0,
            when_used: v.when_used,
            device: v.device,
            identifier: v.identifier,
            indication: v.indication,
            notes: v.notes,
            notes_ext: v.notes_ext,
            recorded_on: v.recorded_on,
            recorded_on_ext: v.recorded_on_ext,
            subject: v.subject,
            timing: v.timing.0,
        }
    }
}

/// The `DeviceUseStatement.bodySite[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceUseStatementBodySite {
    /// `bodySiteCodeableConcept` variant.
    #[fhir("bodySiteCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `bodySiteReference` variant.
    #[fhir("bodySiteReference")]
    Reference(Box<types::Reference>),
}

/// The `DeviceUseStatement.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceUseStatementTiming {
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceUseStatement;

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
