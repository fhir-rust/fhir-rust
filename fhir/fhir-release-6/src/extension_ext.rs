//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every R6 resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r6::resources::Patient;
//! use fhir::r6::types::{Extension, String};
//! use fhir::r6::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: String("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r6::types::Extension;

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
    crate::r6::types::Address,
    crate::r6::types::Age,
    crate::r6::types::Annotation,
    crate::r6::types::Attachment,
    crate::r6::types::Availability,
    crate::r6::types::BackboneElement,
    crate::r6::types::BackboneType,
    crate::r6::types::CodeableConcept,
    crate::r6::types::CodeableReference,
    crate::r6::types::Coding,
    crate::r6::types::ContactDetail,
    crate::r6::types::ContactPoint,
    crate::r6::types::Contributor,
    crate::r6::types::Count,
    crate::r6::types::DataRequirement,
    crate::r6::types::DataType,
    crate::r6::types::Distance,
    crate::r6::types::Dosage,
    crate::r6::types::Duration,
    crate::r6::types::Element,
    crate::r6::types::ElementDefinition,
    crate::r6::types::Expression,
    crate::r6::types::ExtendedContactDetail,
    crate::r6::types::Extension,
    crate::r6::types::HumanName,
    crate::r6::types::Identifier,
    crate::r6::types::MarketingStatus,
    crate::r6::types::Meta,
    crate::r6::types::MonetaryComponent,
    crate::r6::types::Money,
    crate::r6::types::MoneyQuantity,
    crate::r6::types::Narrative,
    crate::r6::types::ParameterDefinition,
    crate::r6::types::Period,
    crate::r6::types::PrimitiveType,
    crate::r6::types::ProductShelfLife,
    crate::r6::types::Quantity,
    crate::r6::types::Range,
    crate::r6::types::Ratio,
    crate::r6::types::RatioRange,
    crate::r6::types::Reference,
    crate::r6::types::RelatedArtifact,
    crate::r6::types::RelativeTime,
    crate::r6::types::SampledData,
    crate::r6::types::Signature,
    crate::r6::types::SimpleQuantity,
    crate::r6::types::Timing,
    crate::r6::types::TriggerDefinition,
    crate::r6::types::UsageContext,
    crate::r6::types::VirtualServiceDetail,
    crate::r6::resources::Account,
    crate::r6::resources::ActivityDefinition,
    crate::r6::resources::ActorDefinition,
    crate::r6::resources::AdministrableProductDefinition,
    crate::r6::resources::AdverseEvent,
    crate::r6::resources::AllergyIntolerance,
    crate::r6::resources::Appointment,
    crate::r6::resources::AppointmentResponse,
    crate::r6::resources::ArtifactAssessment,
    crate::r6::resources::AuditEvent,
    crate::r6::resources::Basic,
    crate::r6::resources::BiologicallyDerivedProduct,
    crate::r6::resources::BiologicallyDerivedProductDispense,
    crate::r6::resources::BodyStructure,
    crate::r6::resources::CapabilityStatement,
    crate::r6::resources::CarePlan,
    crate::r6::resources::CareTeam,
    crate::r6::resources::ChargeItem,
    crate::r6::resources::ChargeItemDefinition,
    crate::r6::resources::Citation,
    crate::r6::resources::Claim,
    crate::r6::resources::ClaimResponse,
    crate::r6::resources::ClinicalAssessment,
    crate::r6::resources::ClinicalUseDefinition,
    crate::r6::resources::CodeSystem,
    crate::r6::resources::Communication,
    crate::r6::resources::CommunicationRequest,
    crate::r6::resources::CompartmentDefinition,
    crate::r6::resources::Composition,
    crate::r6::resources::ConceptMap,
    crate::r6::resources::Condition,
    crate::r6::resources::ConditionDefinition,
    crate::r6::resources::Consent,
    crate::r6::resources::Contract,
    crate::r6::resources::Coverage,
    crate::r6::resources::CoverageEligibilityRequest,
    crate::r6::resources::CoverageEligibilityResponse,
    crate::r6::resources::DetectedIssue,
    crate::r6::resources::Device,
    crate::r6::resources::DeviceAlert,
    crate::r6::resources::DeviceAssociation,
    crate::r6::resources::DeviceDefinition,
    crate::r6::resources::DeviceDispense,
    crate::r6::resources::DeviceMetric,
    crate::r6::resources::DeviceRequest,
    crate::r6::resources::DeviceUsage,
    crate::r6::resources::DiagnosticReport,
    crate::r6::resources::DocumentReference,
    crate::r6::resources::Encounter,
    crate::r6::resources::EncounterHistory,
    crate::r6::resources::Endpoint,
    crate::r6::resources::EnrollmentRequest,
    crate::r6::resources::EnrollmentResponse,
    crate::r6::resources::EpisodeOfCare,
    crate::r6::resources::EventDefinition,
    crate::r6::resources::Evidence,
    crate::r6::resources::EvidenceVariable,
    crate::r6::resources::ExampleScenario,
    crate::r6::resources::ExplanationOfBenefit,
    crate::r6::resources::FamilyMemberHistory,
    crate::r6::resources::Flag,
    crate::r6::resources::FormularyItem,
    crate::r6::resources::GenomicStudy,
    crate::r6::resources::Goal,
    crate::r6::resources::GraphDefinition,
    crate::r6::resources::Group,
    crate::r6::resources::GuidanceResponse,
    crate::r6::resources::HealthcareService,
    crate::r6::resources::ImagingSelection,
    crate::r6::resources::ImagingStudy,
    crate::r6::resources::Immunization,
    crate::r6::resources::ImmunizationEvaluation,
    crate::r6::resources::ImmunizationRecommendation,
    crate::r6::resources::ImplementationGuide,
    crate::r6::resources::Ingredient,
    crate::r6::resources::InsurancePlan,
    crate::r6::resources::InsuranceProduct,
    crate::r6::resources::InventoryItem,
    crate::r6::resources::InventoryReport,
    crate::r6::resources::Invoice,
    crate::r6::resources::Library,
    crate::r6::resources::Linkage,
    crate::r6::resources::List,
    crate::r6::resources::Location,
    crate::r6::resources::ManufacturedItemDefinition,
    crate::r6::resources::Measure,
    crate::r6::resources::MeasureReport,
    crate::r6::resources::Medication,
    crate::r6::resources::MedicationAdministration,
    crate::r6::resources::MedicationDispense,
    crate::r6::resources::MedicationKnowledge,
    crate::r6::resources::MedicationRequest,
    crate::r6::resources::MedicationStatement,
    crate::r6::resources::MedicinalProductDefinition,
    crate::r6::resources::MessageDefinition,
    crate::r6::resources::MessageHeader,
    crate::r6::resources::MolecularDefinition,
    crate::r6::resources::MolecularSequence,
    crate::r6::resources::NamingSystem,
    crate::r6::resources::NutritionIntake,
    crate::r6::resources::NutritionOrder,
    crate::r6::resources::NutritionProduct,
    crate::r6::resources::Observation,
    crate::r6::resources::ObservationDefinition,
    crate::r6::resources::OperationDefinition,
    crate::r6::resources::OperationOutcome,
    crate::r6::resources::Organization,
    crate::r6::resources::OrganizationAffiliation,
    crate::r6::resources::PackagedProductDefinition,
    crate::r6::resources::Patient,
    crate::r6::resources::PaymentNotice,
    crate::r6::resources::PaymentReconciliation,
    crate::r6::resources::Permission,
    crate::r6::resources::Person,
    crate::r6::resources::PersonalRelationship,
    crate::r6::resources::PlanDefinition,
    crate::r6::resources::Practitioner,
    crate::r6::resources::PractitionerRole,
    crate::r6::resources::Procedure,
    crate::r6::resources::Provenance,
    crate::r6::resources::Questionnaire,
    crate::r6::resources::QuestionnaireResponse,
    crate::r6::resources::RegulatedAuthorization,
    crate::r6::resources::RelatedPerson,
    crate::r6::resources::RequestOrchestration,
    crate::r6::resources::Requirements,
    crate::r6::resources::ResearchStudy,
    crate::r6::resources::ResearchSubject,
    crate::r6::resources::RiskAssessment,
    crate::r6::resources::Schedule,
    crate::r6::resources::SearchParameter,
    crate::r6::resources::ServiceRequest,
    crate::r6::resources::Slot,
    crate::r6::resources::Specimen,
    crate::r6::resources::SpecimenDefinition,
    crate::r6::resources::StructureDefinition,
    crate::r6::resources::StructureMap,
    crate::r6::resources::Subscription,
    crate::r6::resources::SubscriptionStatus,
    crate::r6::resources::SubscriptionTopic,
    crate::r6::resources::Substance,
    crate::r6::resources::SubstanceDefinition,
    crate::r6::resources::SubstanceNucleicAcid,
    crate::r6::resources::SubstancePolymer,
    crate::r6::resources::SubstanceProtein,
    crate::r6::resources::SubstanceReferenceInformation,
    crate::r6::resources::SubstanceSourceMaterial,
    crate::r6::resources::SupplyDelivery,
    crate::r6::resources::SupplyRequest,
    crate::r6::resources::Task,
    crate::r6::resources::TerminologyCapabilities,
    crate::r6::resources::TestPlan,
    crate::r6::resources::TestReport,
    crate::r6::resources::TestScript,
    crate::r6::resources::Transport,
    crate::r6::resources::ValueSet,
    crate::r6::resources::VerificationResult,
    crate::r6::resources::VisionPrescription,
);

impl_has_modifier_extension!(
    crate::r6::types::BackboneElement,
    crate::r6::types::BackboneType,
    crate::r6::types::Dosage,
    crate::r6::types::ElementDefinition,
    crate::r6::types::MarketingStatus,
    crate::r6::types::ProductShelfLife,
    crate::r6::types::RelativeTime,
    crate::r6::types::Timing,
    crate::r6::resources::Account,
    crate::r6::resources::ActivityDefinition,
    crate::r6::resources::ActorDefinition,
    crate::r6::resources::AdministrableProductDefinition,
    crate::r6::resources::AdverseEvent,
    crate::r6::resources::AllergyIntolerance,
    crate::r6::resources::Appointment,
    crate::r6::resources::AppointmentResponse,
    crate::r6::resources::ArtifactAssessment,
    crate::r6::resources::AuditEvent,
    crate::r6::resources::Basic,
    crate::r6::resources::BiologicallyDerivedProduct,
    crate::r6::resources::BiologicallyDerivedProductDispense,
    crate::r6::resources::BodyStructure,
    crate::r6::resources::CapabilityStatement,
    crate::r6::resources::CarePlan,
    crate::r6::resources::CareTeam,
    crate::r6::resources::ChargeItem,
    crate::r6::resources::ChargeItemDefinition,
    crate::r6::resources::Citation,
    crate::r6::resources::Claim,
    crate::r6::resources::ClaimResponse,
    crate::r6::resources::ClinicalAssessment,
    crate::r6::resources::ClinicalUseDefinition,
    crate::r6::resources::CodeSystem,
    crate::r6::resources::Communication,
    crate::r6::resources::CommunicationRequest,
    crate::r6::resources::CompartmentDefinition,
    crate::r6::resources::Composition,
    crate::r6::resources::ConceptMap,
    crate::r6::resources::Condition,
    crate::r6::resources::ConditionDefinition,
    crate::r6::resources::Consent,
    crate::r6::resources::Contract,
    crate::r6::resources::Coverage,
    crate::r6::resources::CoverageEligibilityRequest,
    crate::r6::resources::CoverageEligibilityResponse,
    crate::r6::resources::DetectedIssue,
    crate::r6::resources::Device,
    crate::r6::resources::DeviceAlert,
    crate::r6::resources::DeviceAssociation,
    crate::r6::resources::DeviceDefinition,
    crate::r6::resources::DeviceDispense,
    crate::r6::resources::DeviceMetric,
    crate::r6::resources::DeviceRequest,
    crate::r6::resources::DeviceUsage,
    crate::r6::resources::DiagnosticReport,
    crate::r6::resources::DocumentReference,
    crate::r6::resources::Encounter,
    crate::r6::resources::EncounterHistory,
    crate::r6::resources::Endpoint,
    crate::r6::resources::EnrollmentRequest,
    crate::r6::resources::EnrollmentResponse,
    crate::r6::resources::EpisodeOfCare,
    crate::r6::resources::EventDefinition,
    crate::r6::resources::Evidence,
    crate::r6::resources::EvidenceVariable,
    crate::r6::resources::ExampleScenario,
    crate::r6::resources::ExplanationOfBenefit,
    crate::r6::resources::FamilyMemberHistory,
    crate::r6::resources::Flag,
    crate::r6::resources::FormularyItem,
    crate::r6::resources::GenomicStudy,
    crate::r6::resources::Goal,
    crate::r6::resources::GraphDefinition,
    crate::r6::resources::Group,
    crate::r6::resources::GuidanceResponse,
    crate::r6::resources::HealthcareService,
    crate::r6::resources::ImagingSelection,
    crate::r6::resources::ImagingStudy,
    crate::r6::resources::Immunization,
    crate::r6::resources::ImmunizationEvaluation,
    crate::r6::resources::ImmunizationRecommendation,
    crate::r6::resources::ImplementationGuide,
    crate::r6::resources::Ingredient,
    crate::r6::resources::InsurancePlan,
    crate::r6::resources::InsuranceProduct,
    crate::r6::resources::InventoryItem,
    crate::r6::resources::InventoryReport,
    crate::r6::resources::Invoice,
    crate::r6::resources::Library,
    crate::r6::resources::Linkage,
    crate::r6::resources::List,
    crate::r6::resources::Location,
    crate::r6::resources::ManufacturedItemDefinition,
    crate::r6::resources::Measure,
    crate::r6::resources::MeasureReport,
    crate::r6::resources::Medication,
    crate::r6::resources::MedicationAdministration,
    crate::r6::resources::MedicationDispense,
    crate::r6::resources::MedicationKnowledge,
    crate::r6::resources::MedicationRequest,
    crate::r6::resources::MedicationStatement,
    crate::r6::resources::MedicinalProductDefinition,
    crate::r6::resources::MessageDefinition,
    crate::r6::resources::MessageHeader,
    crate::r6::resources::MolecularDefinition,
    crate::r6::resources::MolecularSequence,
    crate::r6::resources::NamingSystem,
    crate::r6::resources::NutritionIntake,
    crate::r6::resources::NutritionOrder,
    crate::r6::resources::NutritionProduct,
    crate::r6::resources::Observation,
    crate::r6::resources::ObservationDefinition,
    crate::r6::resources::OperationDefinition,
    crate::r6::resources::OperationOutcome,
    crate::r6::resources::Organization,
    crate::r6::resources::OrganizationAffiliation,
    crate::r6::resources::PackagedProductDefinition,
    crate::r6::resources::Patient,
    crate::r6::resources::PaymentNotice,
    crate::r6::resources::PaymentReconciliation,
    crate::r6::resources::Permission,
    crate::r6::resources::Person,
    crate::r6::resources::PersonalRelationship,
    crate::r6::resources::PlanDefinition,
    crate::r6::resources::Practitioner,
    crate::r6::resources::PractitionerRole,
    crate::r6::resources::Procedure,
    crate::r6::resources::Provenance,
    crate::r6::resources::Questionnaire,
    crate::r6::resources::QuestionnaireResponse,
    crate::r6::resources::RegulatedAuthorization,
    crate::r6::resources::RelatedPerson,
    crate::r6::resources::RequestOrchestration,
    crate::r6::resources::Requirements,
    crate::r6::resources::ResearchStudy,
    crate::r6::resources::ResearchSubject,
    crate::r6::resources::RiskAssessment,
    crate::r6::resources::Schedule,
    crate::r6::resources::SearchParameter,
    crate::r6::resources::ServiceRequest,
    crate::r6::resources::Slot,
    crate::r6::resources::Specimen,
    crate::r6::resources::SpecimenDefinition,
    crate::r6::resources::StructureDefinition,
    crate::r6::resources::StructureMap,
    crate::r6::resources::Subscription,
    crate::r6::resources::SubscriptionStatus,
    crate::r6::resources::SubscriptionTopic,
    crate::r6::resources::Substance,
    crate::r6::resources::SubstanceDefinition,
    crate::r6::resources::SubstanceNucleicAcid,
    crate::r6::resources::SubstancePolymer,
    crate::r6::resources::SubstanceProtein,
    crate::r6::resources::SubstanceReferenceInformation,
    crate::r6::resources::SubstanceSourceMaterial,
    crate::r6::resources::SupplyDelivery,
    crate::r6::resources::SupplyRequest,
    crate::r6::resources::Task,
    crate::r6::resources::TerminologyCapabilities,
    crate::r6::resources::TestPlan,
    crate::r6::resources::TestReport,
    crate::r6::resources::TestScript,
    crate::r6::resources::Transport,
    crate::r6::resources::ValueSet,
    crate::r6::resources::VerificationResult,
    crate::r6::resources::VisionPrescription,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r6::resources::Patient;

    fn extension(url: &str) -> Extension {
        Extension {
            url: crate::r6::types::String(url.to_string()),
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
