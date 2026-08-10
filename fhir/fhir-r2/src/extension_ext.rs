//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every R2 resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r2::resources::Patient;
//! use fhir::r2::types::{Extension, Uri};
//! use fhir::r2::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: Uri("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r2::types::Extension;

/// Types that carry a FHIR `extension` list.
pub trait HasExtension {
    /// The extensions as a slice (empty if none).
    fn extension_slice(&self) -> &[Extension];
    /// Mutable access to the underlying `Vec<Extension>`.
    fn extension_mut(&mut self) -> &mut Vec<Extension>;
}

/// Ergonomic accessors over [`HasExtension`]. Blanket-implemented.
pub trait ExtensionExt: HasExtension {
    /// The first extension with the given `url`, if any.
    fn extension(&self, url: &str) -> Option<&Extension> {
        self.extension_slice().iter().find(|e| e.url.0 == url)
    }
    /// Every extension with the given `url`.
    fn extensions(&self, url: &str) -> Vec<&Extension> {
        self.extension_slice()
            .iter()
            .filter(|e| e.url.0 == url)
            .collect()
    }
    /// Set an extension, replacing any existing ones with the same `url`.
    fn set_extension(&mut self, ext: Extension) {
        let url = ext.url.0.clone();
        let list = self.extension_mut();
        list.retain(|e| e.url.0 != url);
        list.push(ext);
    }
    /// Append an extension without removing existing ones of the same `url`.
    fn add_extension(&mut self, ext: Extension) {
        self.extension_mut().push(ext);
    }
}

impl<T: HasExtension + ?Sized> ExtensionExt for T {}

/// Types that carry a FHIR `modifierExtension` list (resources and backbones).
pub trait HasModifierExtension {
    /// The modifier extensions as a slice (empty if none).
    fn modifier_extension_slice(&self) -> &[Extension];
    /// Mutable access to the underlying `Vec<Extension>`.
    fn modifier_extension_mut(&mut self) -> &mut Vec<Extension>;
}

/// Ergonomic accessors over [`HasModifierExtension`]. Blanket-implemented.
pub trait ModifierExtensionExt: HasModifierExtension {
    /// The first modifier extension with the given `url`, if any.
    fn modifier_extension(&self, url: &str) -> Option<&Extension> {
        self.modifier_extension_slice()
            .iter()
            .find(|e| e.url.0 == url)
    }
    /// Set a modifier extension, replacing any with the same `url`.
    fn set_modifier_extension(&mut self, ext: Extension) {
        let url = ext.url.0.clone();
        let list = self.modifier_extension_mut();
        list.retain(|e| e.url.0 != url);
        list.push(ext);
    }
}

impl<T: HasModifierExtension + ?Sized> ModifierExtensionExt for T {}

macro_rules! impl_has_extension {
    ($($t:ty),* $(,)?) => { $(
        impl HasExtension for $t {
            fn extension_slice(&self) -> &[Extension] {
                &self.extension
            }
            fn extension_mut(&mut self) -> &mut Vec<Extension> {
                &mut self.extension
            }
        }
    )* };
}

macro_rules! impl_has_modifier_extension {
    ($($t:ty),* $(,)?) => { $(
        impl HasModifierExtension for $t {
            fn modifier_extension_slice(&self) -> &[Extension] {
                &self.modifier_extension
            }
            fn modifier_extension_mut(&mut self) -> &mut Vec<Extension> {
                &mut self.modifier_extension
            }
        }
    )* };
}

impl_has_extension!(
    crate::r2::types::Address,
    crate::r2::types::Annotation,
    crate::r2::types::Attachment,
    crate::r2::types::BackboneElement,
    crate::r2::types::CodeableConcept,
    crate::r2::types::Coding,
    crate::r2::types::ContactPoint,
    crate::r2::types::Element,
    crate::r2::types::ElementDefinition,
    crate::r2::types::Extension,
    crate::r2::types::HumanName,
    crate::r2::types::Identifier,
    crate::r2::types::Meta,
    crate::r2::types::Narrative,
    crate::r2::types::Period,
    crate::r2::types::Quantity,
    crate::r2::types::Range,
    crate::r2::types::Ratio,
    crate::r2::types::Reference,
    crate::r2::types::SampledData,
    crate::r2::types::Signature,
    crate::r2::types::Timing,
    crate::r2::resources::Account,
    crate::r2::resources::AllergyIntolerance,
    crate::r2::resources::Appointment,
    crate::r2::resources::AppointmentResponse,
    crate::r2::resources::AuditEvent,
    crate::r2::resources::Basic,
    crate::r2::resources::BodySite,
    crate::r2::resources::CarePlan,
    crate::r2::resources::Claim,
    crate::r2::resources::ClaimResponse,
    crate::r2::resources::ClinicalImpression,
    crate::r2::resources::Communication,
    crate::r2::resources::CommunicationRequest,
    crate::r2::resources::Composition,
    crate::r2::resources::ConceptMap,
    crate::r2::resources::Condition,
    crate::r2::resources::Conformance,
    crate::r2::resources::Contract,
    crate::r2::resources::Coverage,
    crate::r2::resources::DataElement,
    crate::r2::resources::DetectedIssue,
    crate::r2::resources::Device,
    crate::r2::resources::DeviceComponent,
    crate::r2::resources::DeviceMetric,
    crate::r2::resources::DeviceUseRequest,
    crate::r2::resources::DeviceUseStatement,
    crate::r2::resources::DiagnosticOrder,
    crate::r2::resources::DiagnosticReport,
    crate::r2::resources::DocumentManifest,
    crate::r2::resources::DocumentReference,
    crate::r2::resources::EligibilityRequest,
    crate::r2::resources::EligibilityResponse,
    crate::r2::resources::Encounter,
    crate::r2::resources::EnrollmentRequest,
    crate::r2::resources::EnrollmentResponse,
    crate::r2::resources::EpisodeOfCare,
    crate::r2::resources::ExplanationOfBenefit,
    crate::r2::resources::FamilyMemberHistory,
    crate::r2::resources::Flag,
    crate::r2::resources::Goal,
    crate::r2::resources::Group,
    crate::r2::resources::HealthcareService,
    crate::r2::resources::ImagingObjectSelection,
    crate::r2::resources::ImagingStudy,
    crate::r2::resources::Immunization,
    crate::r2::resources::ImmunizationRecommendation,
    crate::r2::resources::ImplementationGuide,
    crate::r2::resources::List,
    crate::r2::resources::Location,
    crate::r2::resources::Media,
    crate::r2::resources::Medication,
    crate::r2::resources::MedicationAdministration,
    crate::r2::resources::MedicationDispense,
    crate::r2::resources::MedicationOrder,
    crate::r2::resources::MedicationStatement,
    crate::r2::resources::MessageHeader,
    crate::r2::resources::NamingSystem,
    crate::r2::resources::NutritionOrder,
    crate::r2::resources::Observation,
    crate::r2::resources::OperationDefinition,
    crate::r2::resources::OperationOutcome,
    crate::r2::resources::Order,
    crate::r2::resources::OrderResponse,
    crate::r2::resources::Organization,
    crate::r2::resources::Patient,
    crate::r2::resources::PaymentNotice,
    crate::r2::resources::PaymentReconciliation,
    crate::r2::resources::Person,
    crate::r2::resources::Practitioner,
    crate::r2::resources::Procedure,
    crate::r2::resources::ProcedureRequest,
    crate::r2::resources::ProcessRequest,
    crate::r2::resources::ProcessResponse,
    crate::r2::resources::Provenance,
    crate::r2::resources::Questionnaire,
    crate::r2::resources::QuestionnaireResponse,
    crate::r2::resources::ReferralRequest,
    crate::r2::resources::RelatedPerson,
    crate::r2::resources::RiskAssessment,
    crate::r2::resources::Schedule,
    crate::r2::resources::SearchParameter,
    crate::r2::resources::Slot,
    crate::r2::resources::Specimen,
    crate::r2::resources::StructureDefinition,
    crate::r2::resources::Subscription,
    crate::r2::resources::Substance,
    crate::r2::resources::SupplyDelivery,
    crate::r2::resources::SupplyRequest,
    crate::r2::resources::TestScript,
    crate::r2::resources::ValueSet,
    crate::r2::resources::VisionPrescription,
);

impl_has_modifier_extension!(
    crate::r2::types::BackboneElement,
    crate::r2::resources::Account,
    crate::r2::resources::AllergyIntolerance,
    crate::r2::resources::Appointment,
    crate::r2::resources::AppointmentResponse,
    crate::r2::resources::AuditEvent,
    crate::r2::resources::Basic,
    crate::r2::resources::BodySite,
    crate::r2::resources::CarePlan,
    crate::r2::resources::Claim,
    crate::r2::resources::ClaimResponse,
    crate::r2::resources::ClinicalImpression,
    crate::r2::resources::Communication,
    crate::r2::resources::CommunicationRequest,
    crate::r2::resources::Composition,
    crate::r2::resources::ConceptMap,
    crate::r2::resources::Condition,
    crate::r2::resources::Conformance,
    crate::r2::resources::Contract,
    crate::r2::resources::Coverage,
    crate::r2::resources::DataElement,
    crate::r2::resources::DetectedIssue,
    crate::r2::resources::Device,
    crate::r2::resources::DeviceComponent,
    crate::r2::resources::DeviceMetric,
    crate::r2::resources::DeviceUseRequest,
    crate::r2::resources::DeviceUseStatement,
    crate::r2::resources::DiagnosticOrder,
    crate::r2::resources::DiagnosticReport,
    crate::r2::resources::DocumentManifest,
    crate::r2::resources::DocumentReference,
    crate::r2::resources::EligibilityRequest,
    crate::r2::resources::EligibilityResponse,
    crate::r2::resources::Encounter,
    crate::r2::resources::EnrollmentRequest,
    crate::r2::resources::EnrollmentResponse,
    crate::r2::resources::EpisodeOfCare,
    crate::r2::resources::ExplanationOfBenefit,
    crate::r2::resources::FamilyMemberHistory,
    crate::r2::resources::Flag,
    crate::r2::resources::Goal,
    crate::r2::resources::Group,
    crate::r2::resources::HealthcareService,
    crate::r2::resources::ImagingObjectSelection,
    crate::r2::resources::ImagingStudy,
    crate::r2::resources::Immunization,
    crate::r2::resources::ImmunizationRecommendation,
    crate::r2::resources::ImplementationGuide,
    crate::r2::resources::List,
    crate::r2::resources::Location,
    crate::r2::resources::Media,
    crate::r2::resources::Medication,
    crate::r2::resources::MedicationAdministration,
    crate::r2::resources::MedicationDispense,
    crate::r2::resources::MedicationOrder,
    crate::r2::resources::MedicationStatement,
    crate::r2::resources::MessageHeader,
    crate::r2::resources::NamingSystem,
    crate::r2::resources::NutritionOrder,
    crate::r2::resources::Observation,
    crate::r2::resources::OperationDefinition,
    crate::r2::resources::OperationOutcome,
    crate::r2::resources::Order,
    crate::r2::resources::OrderResponse,
    crate::r2::resources::Organization,
    crate::r2::resources::Patient,
    crate::r2::resources::PaymentNotice,
    crate::r2::resources::PaymentReconciliation,
    crate::r2::resources::Person,
    crate::r2::resources::Practitioner,
    crate::r2::resources::Procedure,
    crate::r2::resources::ProcedureRequest,
    crate::r2::resources::ProcessRequest,
    crate::r2::resources::ProcessResponse,
    crate::r2::resources::Provenance,
    crate::r2::resources::Questionnaire,
    crate::r2::resources::QuestionnaireResponse,
    crate::r2::resources::ReferralRequest,
    crate::r2::resources::RelatedPerson,
    crate::r2::resources::RiskAssessment,
    crate::r2::resources::Schedule,
    crate::r2::resources::SearchParameter,
    crate::r2::resources::Slot,
    crate::r2::resources::Specimen,
    crate::r2::resources::StructureDefinition,
    crate::r2::resources::Subscription,
    crate::r2::resources::Substance,
    crate::r2::resources::SupplyDelivery,
    crate::r2::resources::SupplyRequest,
    crate::r2::resources::TestScript,
    crate::r2::resources::ValueSet,
    crate::r2::resources::VisionPrescription,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r2::resources::Patient;

    fn extension(url: &str) -> Extension {
        Extension {
            url: crate::r2::types::Uri(url.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn set_replaces_and_add_appends() {
        let mut patient = Patient::default();
        patient.set_extension(extension("http://example.org/a"));
        patient.set_extension(extension("http://example.org/a"));
        assert_eq!(patient.extensions("http://example.org/a").len(), 1);

        patient.add_extension(extension("http://example.org/a"));
        assert_eq!(patient.extensions("http://example.org/a").len(), 2);
        assert!(patient.extension("http://example.org/missing").is_none());
    }

    #[test]
    fn modifier_extensions_are_separate() {
        let mut patient = Patient::default();
        patient.set_modifier_extension(extension("http://example.org/m"));
        assert!(patient.modifier_extension("http://example.org/m").is_some());
        assert!(patient.extension("http://example.org/m").is_none());
    }
}
