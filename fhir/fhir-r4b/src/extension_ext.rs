//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every R4B resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r4b::resources::Patient;
//! use fhir::r4b::types::{Extension, String};
//! use fhir::r4b::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: String("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r4b::types::Extension;

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
    crate::r4b::types::Address,
    crate::r4b::types::Age,
    crate::r4b::types::Annotation,
    crate::r4b::types::Attachment,
    crate::r4b::types::BackboneElement,
    crate::r4b::types::CodeableConcept,
    crate::r4b::types::CodeableReference,
    crate::r4b::types::Coding,
    crate::r4b::types::ContactDetail,
    crate::r4b::types::ContactPoint,
    crate::r4b::types::Contributor,
    crate::r4b::types::Count,
    crate::r4b::types::DataRequirement,
    crate::r4b::types::Distance,
    crate::r4b::types::Dosage,
    crate::r4b::types::Duration,
    crate::r4b::types::Element,
    crate::r4b::types::ElementDefinition,
    crate::r4b::types::Expression,
    crate::r4b::types::Extension,
    crate::r4b::types::HumanName,
    crate::r4b::types::Identifier,
    crate::r4b::types::MarketingStatus,
    crate::r4b::types::Meta,
    crate::r4b::types::Money,
    crate::r4b::types::MoneyQuantity,
    crate::r4b::types::Narrative,
    crate::r4b::types::ParameterDefinition,
    crate::r4b::types::Period,
    crate::r4b::types::Population,
    crate::r4b::types::ProdCharacteristic,
    crate::r4b::types::ProductShelfLife,
    crate::r4b::types::Quantity,
    crate::r4b::types::Range,
    crate::r4b::types::Ratio,
    crate::r4b::types::RatioRange,
    crate::r4b::types::Reference,
    crate::r4b::types::RelatedArtifact,
    crate::r4b::types::SampledData,
    crate::r4b::types::Signature,
    crate::r4b::types::SimpleQuantity,
    crate::r4b::types::Timing,
    crate::r4b::types::TriggerDefinition,
    crate::r4b::types::UsageContext,
    crate::r4b::resources::Account,
    crate::r4b::resources::ActivityDefinition,
    crate::r4b::resources::AdministrableProductDefinition,
    crate::r4b::resources::AdverseEvent,
    crate::r4b::resources::AllergyIntolerance,
    crate::r4b::resources::Appointment,
    crate::r4b::resources::AppointmentResponse,
    crate::r4b::resources::AuditEvent,
    crate::r4b::resources::Basic,
    crate::r4b::resources::BiologicallyDerivedProduct,
    crate::r4b::resources::BodyStructure,
    crate::r4b::resources::CapabilityStatement,
    crate::r4b::resources::CarePlan,
    crate::r4b::resources::CareTeam,
    crate::r4b::resources::CatalogEntry,
    crate::r4b::resources::ChargeItem,
    crate::r4b::resources::ChargeItemDefinition,
    crate::r4b::resources::Citation,
    crate::r4b::resources::Claim,
    crate::r4b::resources::ClaimResponse,
    crate::r4b::resources::ClinicalImpression,
    crate::r4b::resources::ClinicalUseDefinition,
    crate::r4b::resources::CodeSystem,
    crate::r4b::resources::Communication,
    crate::r4b::resources::CommunicationRequest,
    crate::r4b::resources::CompartmentDefinition,
    crate::r4b::resources::Composition,
    crate::r4b::resources::ConceptMap,
    crate::r4b::resources::Condition,
    crate::r4b::resources::Consent,
    crate::r4b::resources::Contract,
    crate::r4b::resources::Coverage,
    crate::r4b::resources::CoverageEligibilityRequest,
    crate::r4b::resources::CoverageEligibilityResponse,
    crate::r4b::resources::DetectedIssue,
    crate::r4b::resources::Device,
    crate::r4b::resources::DeviceDefinition,
    crate::r4b::resources::DeviceMetric,
    crate::r4b::resources::DeviceRequest,
    crate::r4b::resources::DeviceUseStatement,
    crate::r4b::resources::DiagnosticReport,
    crate::r4b::resources::DocumentManifest,
    crate::r4b::resources::DocumentReference,
    crate::r4b::resources::Encounter,
    crate::r4b::resources::Endpoint,
    crate::r4b::resources::EnrollmentRequest,
    crate::r4b::resources::EnrollmentResponse,
    crate::r4b::resources::EpisodeOfCare,
    crate::r4b::resources::EventDefinition,
    crate::r4b::resources::Evidence,
    crate::r4b::resources::EvidenceReport,
    crate::r4b::resources::EvidenceVariable,
    crate::r4b::resources::ExampleScenario,
    crate::r4b::resources::ExplanationOfBenefit,
    crate::r4b::resources::FamilyMemberHistory,
    crate::r4b::resources::Flag,
    crate::r4b::resources::Goal,
    crate::r4b::resources::GraphDefinition,
    crate::r4b::resources::Group,
    crate::r4b::resources::GuidanceResponse,
    crate::r4b::resources::HealthcareService,
    crate::r4b::resources::ImagingStudy,
    crate::r4b::resources::Immunization,
    crate::r4b::resources::ImmunizationEvaluation,
    crate::r4b::resources::ImmunizationRecommendation,
    crate::r4b::resources::ImplementationGuide,
    crate::r4b::resources::Ingredient,
    crate::r4b::resources::InsurancePlan,
    crate::r4b::resources::Invoice,
    crate::r4b::resources::Library,
    crate::r4b::resources::Linkage,
    crate::r4b::resources::List,
    crate::r4b::resources::Location,
    crate::r4b::resources::ManufacturedItemDefinition,
    crate::r4b::resources::Measure,
    crate::r4b::resources::MeasureReport,
    crate::r4b::resources::Media,
    crate::r4b::resources::Medication,
    crate::r4b::resources::MedicationAdministration,
    crate::r4b::resources::MedicationDispense,
    crate::r4b::resources::MedicationKnowledge,
    crate::r4b::resources::MedicationRequest,
    crate::r4b::resources::MedicationStatement,
    crate::r4b::resources::MedicinalProductDefinition,
    crate::r4b::resources::MessageDefinition,
    crate::r4b::resources::MessageHeader,
    crate::r4b::resources::MolecularSequence,
    crate::r4b::resources::NamingSystem,
    crate::r4b::resources::NutritionOrder,
    crate::r4b::resources::NutritionProduct,
    crate::r4b::resources::Observation,
    crate::r4b::resources::ObservationDefinition,
    crate::r4b::resources::OperationDefinition,
    crate::r4b::resources::OperationOutcome,
    crate::r4b::resources::Organization,
    crate::r4b::resources::OrganizationAffiliation,
    crate::r4b::resources::PackagedProductDefinition,
    crate::r4b::resources::Patient,
    crate::r4b::resources::PaymentNotice,
    crate::r4b::resources::PaymentReconciliation,
    crate::r4b::resources::Person,
    crate::r4b::resources::PlanDefinition,
    crate::r4b::resources::Practitioner,
    crate::r4b::resources::PractitionerRole,
    crate::r4b::resources::Procedure,
    crate::r4b::resources::Provenance,
    crate::r4b::resources::Questionnaire,
    crate::r4b::resources::QuestionnaireResponse,
    crate::r4b::resources::RegulatedAuthorization,
    crate::r4b::resources::RelatedPerson,
    crate::r4b::resources::RequestGroup,
    crate::r4b::resources::ResearchDefinition,
    crate::r4b::resources::ResearchElementDefinition,
    crate::r4b::resources::ResearchStudy,
    crate::r4b::resources::ResearchSubject,
    crate::r4b::resources::RiskAssessment,
    crate::r4b::resources::Schedule,
    crate::r4b::resources::SearchParameter,
    crate::r4b::resources::ServiceRequest,
    crate::r4b::resources::Slot,
    crate::r4b::resources::Specimen,
    crate::r4b::resources::SpecimenDefinition,
    crate::r4b::resources::StructureDefinition,
    crate::r4b::resources::StructureMap,
    crate::r4b::resources::Subscription,
    crate::r4b::resources::SubscriptionStatus,
    crate::r4b::resources::SubscriptionTopic,
    crate::r4b::resources::Substance,
    crate::r4b::resources::SubstanceDefinition,
    crate::r4b::resources::SupplyDelivery,
    crate::r4b::resources::SupplyRequest,
    crate::r4b::resources::Task,
    crate::r4b::resources::TerminologyCapabilities,
    crate::r4b::resources::TestReport,
    crate::r4b::resources::TestScript,
    crate::r4b::resources::ValueSet,
    crate::r4b::resources::VerificationResult,
    crate::r4b::resources::VisionPrescription,
);

impl_has_modifier_extension!(
    crate::r4b::types::BackboneElement,
    crate::r4b::types::Dosage,
    crate::r4b::types::ElementDefinition,
    crate::r4b::types::MarketingStatus,
    crate::r4b::types::Population,
    crate::r4b::types::ProdCharacteristic,
    crate::r4b::types::ProductShelfLife,
    crate::r4b::types::Timing,
    crate::r4b::resources::Account,
    crate::r4b::resources::ActivityDefinition,
    crate::r4b::resources::AdministrableProductDefinition,
    crate::r4b::resources::AdverseEvent,
    crate::r4b::resources::AllergyIntolerance,
    crate::r4b::resources::Appointment,
    crate::r4b::resources::AppointmentResponse,
    crate::r4b::resources::AuditEvent,
    crate::r4b::resources::Basic,
    crate::r4b::resources::BiologicallyDerivedProduct,
    crate::r4b::resources::BodyStructure,
    crate::r4b::resources::CapabilityStatement,
    crate::r4b::resources::CarePlan,
    crate::r4b::resources::CareTeam,
    crate::r4b::resources::CatalogEntry,
    crate::r4b::resources::ChargeItem,
    crate::r4b::resources::ChargeItemDefinition,
    crate::r4b::resources::Citation,
    crate::r4b::resources::Claim,
    crate::r4b::resources::ClaimResponse,
    crate::r4b::resources::ClinicalImpression,
    crate::r4b::resources::ClinicalUseDefinition,
    crate::r4b::resources::CodeSystem,
    crate::r4b::resources::Communication,
    crate::r4b::resources::CommunicationRequest,
    crate::r4b::resources::CompartmentDefinition,
    crate::r4b::resources::Composition,
    crate::r4b::resources::ConceptMap,
    crate::r4b::resources::Condition,
    crate::r4b::resources::Consent,
    crate::r4b::resources::Contract,
    crate::r4b::resources::Coverage,
    crate::r4b::resources::CoverageEligibilityRequest,
    crate::r4b::resources::CoverageEligibilityResponse,
    crate::r4b::resources::DetectedIssue,
    crate::r4b::resources::Device,
    crate::r4b::resources::DeviceDefinition,
    crate::r4b::resources::DeviceMetric,
    crate::r4b::resources::DeviceRequest,
    crate::r4b::resources::DeviceUseStatement,
    crate::r4b::resources::DiagnosticReport,
    crate::r4b::resources::DocumentManifest,
    crate::r4b::resources::DocumentReference,
    crate::r4b::resources::Encounter,
    crate::r4b::resources::Endpoint,
    crate::r4b::resources::EnrollmentRequest,
    crate::r4b::resources::EnrollmentResponse,
    crate::r4b::resources::EpisodeOfCare,
    crate::r4b::resources::EventDefinition,
    crate::r4b::resources::Evidence,
    crate::r4b::resources::EvidenceReport,
    crate::r4b::resources::EvidenceVariable,
    crate::r4b::resources::ExampleScenario,
    crate::r4b::resources::ExplanationOfBenefit,
    crate::r4b::resources::FamilyMemberHistory,
    crate::r4b::resources::Flag,
    crate::r4b::resources::Goal,
    crate::r4b::resources::GraphDefinition,
    crate::r4b::resources::Group,
    crate::r4b::resources::GuidanceResponse,
    crate::r4b::resources::HealthcareService,
    crate::r4b::resources::ImagingStudy,
    crate::r4b::resources::Immunization,
    crate::r4b::resources::ImmunizationEvaluation,
    crate::r4b::resources::ImmunizationRecommendation,
    crate::r4b::resources::ImplementationGuide,
    crate::r4b::resources::Ingredient,
    crate::r4b::resources::InsurancePlan,
    crate::r4b::resources::Invoice,
    crate::r4b::resources::Library,
    crate::r4b::resources::Linkage,
    crate::r4b::resources::List,
    crate::r4b::resources::Location,
    crate::r4b::resources::ManufacturedItemDefinition,
    crate::r4b::resources::Measure,
    crate::r4b::resources::MeasureReport,
    crate::r4b::resources::Media,
    crate::r4b::resources::Medication,
    crate::r4b::resources::MedicationAdministration,
    crate::r4b::resources::MedicationDispense,
    crate::r4b::resources::MedicationKnowledge,
    crate::r4b::resources::MedicationRequest,
    crate::r4b::resources::MedicationStatement,
    crate::r4b::resources::MedicinalProductDefinition,
    crate::r4b::resources::MessageDefinition,
    crate::r4b::resources::MessageHeader,
    crate::r4b::resources::MolecularSequence,
    crate::r4b::resources::NamingSystem,
    crate::r4b::resources::NutritionOrder,
    crate::r4b::resources::NutritionProduct,
    crate::r4b::resources::Observation,
    crate::r4b::resources::ObservationDefinition,
    crate::r4b::resources::OperationDefinition,
    crate::r4b::resources::OperationOutcome,
    crate::r4b::resources::Organization,
    crate::r4b::resources::OrganizationAffiliation,
    crate::r4b::resources::PackagedProductDefinition,
    crate::r4b::resources::Patient,
    crate::r4b::resources::PaymentNotice,
    crate::r4b::resources::PaymentReconciliation,
    crate::r4b::resources::Person,
    crate::r4b::resources::PlanDefinition,
    crate::r4b::resources::Practitioner,
    crate::r4b::resources::PractitionerRole,
    crate::r4b::resources::Procedure,
    crate::r4b::resources::Provenance,
    crate::r4b::resources::Questionnaire,
    crate::r4b::resources::QuestionnaireResponse,
    crate::r4b::resources::RegulatedAuthorization,
    crate::r4b::resources::RelatedPerson,
    crate::r4b::resources::RequestGroup,
    crate::r4b::resources::ResearchDefinition,
    crate::r4b::resources::ResearchElementDefinition,
    crate::r4b::resources::ResearchStudy,
    crate::r4b::resources::ResearchSubject,
    crate::r4b::resources::RiskAssessment,
    crate::r4b::resources::Schedule,
    crate::r4b::resources::SearchParameter,
    crate::r4b::resources::ServiceRequest,
    crate::r4b::resources::Slot,
    crate::r4b::resources::Specimen,
    crate::r4b::resources::SpecimenDefinition,
    crate::r4b::resources::StructureDefinition,
    crate::r4b::resources::StructureMap,
    crate::r4b::resources::Subscription,
    crate::r4b::resources::SubscriptionStatus,
    crate::r4b::resources::SubscriptionTopic,
    crate::r4b::resources::Substance,
    crate::r4b::resources::SubstanceDefinition,
    crate::r4b::resources::SupplyDelivery,
    crate::r4b::resources::SupplyRequest,
    crate::r4b::resources::Task,
    crate::r4b::resources::TerminologyCapabilities,
    crate::r4b::resources::TestReport,
    crate::r4b::resources::TestScript,
    crate::r4b::resources::ValueSet,
    crate::r4b::resources::VerificationResult,
    crate::r4b::resources::VisionPrescription,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4b::resources::Patient;

    fn extension(url: &str) -> Extension {
        Extension {
            url: crate::r4b::types::String(url.to_string()),
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
