//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every R3 resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r3::resources::Patient;
//! use fhir::r3::types::{Extension, Uri};
//! use fhir::r3::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: Uri("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r3::types::Extension;

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
    crate::r3::types::Address,
    crate::r3::types::Age,
    crate::r3::types::Annotation,
    crate::r3::types::Attachment,
    crate::r3::types::BackboneElement,
    crate::r3::types::CodeableConcept,
    crate::r3::types::Coding,
    crate::r3::types::ContactDetail,
    crate::r3::types::ContactPoint,
    crate::r3::types::Contributor,
    crate::r3::types::Count,
    crate::r3::types::DataRequirement,
    crate::r3::types::Distance,
    crate::r3::types::Dosage,
    crate::r3::types::Duration,
    crate::r3::types::Element,
    crate::r3::types::ElementDefinition,
    crate::r3::types::Extension,
    crate::r3::types::HumanName,
    crate::r3::types::Identifier,
    crate::r3::types::Meta,
    crate::r3::types::Money,
    crate::r3::types::Narrative,
    crate::r3::types::ParameterDefinition,
    crate::r3::types::Period,
    crate::r3::types::Quantity,
    crate::r3::types::Range,
    crate::r3::types::Ratio,
    crate::r3::types::Reference,
    crate::r3::types::RelatedArtifact,
    crate::r3::types::SampledData,
    crate::r3::types::Signature,
    crate::r3::types::SimpleQuantity,
    crate::r3::types::Timing,
    crate::r3::types::TriggerDefinition,
    crate::r3::types::UsageContext,
    crate::r3::resources::Account,
    crate::r3::resources::ActivityDefinition,
    crate::r3::resources::AdverseEvent,
    crate::r3::resources::AllergyIntolerance,
    crate::r3::resources::Appointment,
    crate::r3::resources::AppointmentResponse,
    crate::r3::resources::AuditEvent,
    crate::r3::resources::Basic,
    crate::r3::resources::BodySite,
    crate::r3::resources::CapabilityStatement,
    crate::r3::resources::CarePlan,
    crate::r3::resources::CareTeam,
    crate::r3::resources::ChargeItem,
    crate::r3::resources::Claim,
    crate::r3::resources::ClaimResponse,
    crate::r3::resources::ClinicalImpression,
    crate::r3::resources::CodeSystem,
    crate::r3::resources::Communication,
    crate::r3::resources::CommunicationRequest,
    crate::r3::resources::CompartmentDefinition,
    crate::r3::resources::Composition,
    crate::r3::resources::ConceptMap,
    crate::r3::resources::Condition,
    crate::r3::resources::Consent,
    crate::r3::resources::Contract,
    crate::r3::resources::Coverage,
    crate::r3::resources::DataElement,
    crate::r3::resources::DetectedIssue,
    crate::r3::resources::Device,
    crate::r3::resources::DeviceComponent,
    crate::r3::resources::DeviceMetric,
    crate::r3::resources::DeviceRequest,
    crate::r3::resources::DeviceUseStatement,
    crate::r3::resources::DiagnosticReport,
    crate::r3::resources::DocumentManifest,
    crate::r3::resources::DocumentReference,
    crate::r3::resources::EligibilityRequest,
    crate::r3::resources::EligibilityResponse,
    crate::r3::resources::Encounter,
    crate::r3::resources::Endpoint,
    crate::r3::resources::EnrollmentRequest,
    crate::r3::resources::EnrollmentResponse,
    crate::r3::resources::EpisodeOfCare,
    crate::r3::resources::ExpansionProfile,
    crate::r3::resources::ExplanationOfBenefit,
    crate::r3::resources::FamilyMemberHistory,
    crate::r3::resources::Flag,
    crate::r3::resources::Goal,
    crate::r3::resources::GraphDefinition,
    crate::r3::resources::Group,
    crate::r3::resources::GuidanceResponse,
    crate::r3::resources::HealthcareService,
    crate::r3::resources::ImagingManifest,
    crate::r3::resources::ImagingStudy,
    crate::r3::resources::Immunization,
    crate::r3::resources::ImmunizationRecommendation,
    crate::r3::resources::ImplementationGuide,
    crate::r3::resources::Library,
    crate::r3::resources::Linkage,
    crate::r3::resources::List,
    crate::r3::resources::Location,
    crate::r3::resources::Measure,
    crate::r3::resources::MeasureReport,
    crate::r3::resources::Media,
    crate::r3::resources::Medication,
    crate::r3::resources::MedicationAdministration,
    crate::r3::resources::MedicationDispense,
    crate::r3::resources::MedicationRequest,
    crate::r3::resources::MedicationStatement,
    crate::r3::resources::MessageDefinition,
    crate::r3::resources::MessageHeader,
    crate::r3::resources::NamingSystem,
    crate::r3::resources::NutritionOrder,
    crate::r3::resources::Observation,
    crate::r3::resources::OperationDefinition,
    crate::r3::resources::OperationOutcome,
    crate::r3::resources::Organization,
    crate::r3::resources::Patient,
    crate::r3::resources::PaymentNotice,
    crate::r3::resources::PaymentReconciliation,
    crate::r3::resources::Person,
    crate::r3::resources::PlanDefinition,
    crate::r3::resources::Practitioner,
    crate::r3::resources::PractitionerRole,
    crate::r3::resources::Procedure,
    crate::r3::resources::ProcedureRequest,
    crate::r3::resources::ProcessRequest,
    crate::r3::resources::ProcessResponse,
    crate::r3::resources::Provenance,
    crate::r3::resources::Questionnaire,
    crate::r3::resources::QuestionnaireResponse,
    crate::r3::resources::ReferralRequest,
    crate::r3::resources::RelatedPerson,
    crate::r3::resources::RequestGroup,
    crate::r3::resources::ResearchStudy,
    crate::r3::resources::ResearchSubject,
    crate::r3::resources::RiskAssessment,
    crate::r3::resources::Schedule,
    crate::r3::resources::SearchParameter,
    crate::r3::resources::Sequence,
    crate::r3::resources::ServiceDefinition,
    crate::r3::resources::Slot,
    crate::r3::resources::Specimen,
    crate::r3::resources::StructureDefinition,
    crate::r3::resources::StructureMap,
    crate::r3::resources::Subscription,
    crate::r3::resources::Substance,
    crate::r3::resources::SupplyDelivery,
    crate::r3::resources::SupplyRequest,
    crate::r3::resources::Task,
    crate::r3::resources::TestReport,
    crate::r3::resources::TestScript,
    crate::r3::resources::ValueSet,
    crate::r3::resources::VisionPrescription,
);

impl_has_modifier_extension!(
    crate::r3::types::BackboneElement,
    crate::r3::resources::Account,
    crate::r3::resources::ActivityDefinition,
    crate::r3::resources::AdverseEvent,
    crate::r3::resources::AllergyIntolerance,
    crate::r3::resources::Appointment,
    crate::r3::resources::AppointmentResponse,
    crate::r3::resources::AuditEvent,
    crate::r3::resources::Basic,
    crate::r3::resources::BodySite,
    crate::r3::resources::CapabilityStatement,
    crate::r3::resources::CarePlan,
    crate::r3::resources::CareTeam,
    crate::r3::resources::ChargeItem,
    crate::r3::resources::Claim,
    crate::r3::resources::ClaimResponse,
    crate::r3::resources::ClinicalImpression,
    crate::r3::resources::CodeSystem,
    crate::r3::resources::Communication,
    crate::r3::resources::CommunicationRequest,
    crate::r3::resources::CompartmentDefinition,
    crate::r3::resources::Composition,
    crate::r3::resources::ConceptMap,
    crate::r3::resources::Condition,
    crate::r3::resources::Consent,
    crate::r3::resources::Contract,
    crate::r3::resources::Coverage,
    crate::r3::resources::DataElement,
    crate::r3::resources::DetectedIssue,
    crate::r3::resources::Device,
    crate::r3::resources::DeviceComponent,
    crate::r3::resources::DeviceMetric,
    crate::r3::resources::DeviceRequest,
    crate::r3::resources::DeviceUseStatement,
    crate::r3::resources::DiagnosticReport,
    crate::r3::resources::DocumentManifest,
    crate::r3::resources::DocumentReference,
    crate::r3::resources::EligibilityRequest,
    crate::r3::resources::EligibilityResponse,
    crate::r3::resources::Encounter,
    crate::r3::resources::Endpoint,
    crate::r3::resources::EnrollmentRequest,
    crate::r3::resources::EnrollmentResponse,
    crate::r3::resources::EpisodeOfCare,
    crate::r3::resources::ExpansionProfile,
    crate::r3::resources::ExplanationOfBenefit,
    crate::r3::resources::FamilyMemberHistory,
    crate::r3::resources::Flag,
    crate::r3::resources::Goal,
    crate::r3::resources::GraphDefinition,
    crate::r3::resources::Group,
    crate::r3::resources::GuidanceResponse,
    crate::r3::resources::HealthcareService,
    crate::r3::resources::ImagingManifest,
    crate::r3::resources::ImagingStudy,
    crate::r3::resources::Immunization,
    crate::r3::resources::ImmunizationRecommendation,
    crate::r3::resources::ImplementationGuide,
    crate::r3::resources::Library,
    crate::r3::resources::Linkage,
    crate::r3::resources::List,
    crate::r3::resources::Location,
    crate::r3::resources::Measure,
    crate::r3::resources::MeasureReport,
    crate::r3::resources::Media,
    crate::r3::resources::Medication,
    crate::r3::resources::MedicationAdministration,
    crate::r3::resources::MedicationDispense,
    crate::r3::resources::MedicationRequest,
    crate::r3::resources::MedicationStatement,
    crate::r3::resources::MessageDefinition,
    crate::r3::resources::MessageHeader,
    crate::r3::resources::NamingSystem,
    crate::r3::resources::NutritionOrder,
    crate::r3::resources::Observation,
    crate::r3::resources::OperationDefinition,
    crate::r3::resources::OperationOutcome,
    crate::r3::resources::Organization,
    crate::r3::resources::Patient,
    crate::r3::resources::PaymentNotice,
    crate::r3::resources::PaymentReconciliation,
    crate::r3::resources::Person,
    crate::r3::resources::PlanDefinition,
    crate::r3::resources::Practitioner,
    crate::r3::resources::PractitionerRole,
    crate::r3::resources::Procedure,
    crate::r3::resources::ProcedureRequest,
    crate::r3::resources::ProcessRequest,
    crate::r3::resources::ProcessResponse,
    crate::r3::resources::Provenance,
    crate::r3::resources::Questionnaire,
    crate::r3::resources::QuestionnaireResponse,
    crate::r3::resources::ReferralRequest,
    crate::r3::resources::RelatedPerson,
    crate::r3::resources::RequestGroup,
    crate::r3::resources::ResearchStudy,
    crate::r3::resources::ResearchSubject,
    crate::r3::resources::RiskAssessment,
    crate::r3::resources::Schedule,
    crate::r3::resources::SearchParameter,
    crate::r3::resources::Sequence,
    crate::r3::resources::ServiceDefinition,
    crate::r3::resources::Slot,
    crate::r3::resources::Specimen,
    crate::r3::resources::StructureDefinition,
    crate::r3::resources::StructureMap,
    crate::r3::resources::Subscription,
    crate::r3::resources::Substance,
    crate::r3::resources::SupplyDelivery,
    crate::r3::resources::SupplyRequest,
    crate::r3::resources::Task,
    crate::r3::resources::TestReport,
    crate::r3::resources::TestScript,
    crate::r3::resources::ValueSet,
    crate::r3::resources::VisionPrescription,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r3::resources::Patient;

    fn extension(url: &str) -> Extension {
        Extension {
            url: crate::r3::types::Uri(url.to_string()),
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
