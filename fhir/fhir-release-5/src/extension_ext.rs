//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::types::Extension;
//! use fhir::r5::types::String as FhirString;
//! use fhir::r5::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: FhirString("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r5::types::Extension;

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

// `Reference<T>` is generic, so implement it directly.
impl<T> HasExtension for crate::r5::types::reference::Reference<T> {
    fn extension_slice(&self) -> &[Extension] {
        &self.extension
    }
    fn extension_mut(&mut self) -> &mut Vec<Extension> {
        &mut self.extension
    }
}

impl_has_extension!(
    crate::r5::resources::Account,
    crate::r5::resources::ActivityDefinition,
    crate::r5::resources::ActorDefinition,
    crate::r5::resources::AdministrableProductDefinition,
    crate::r5::resources::AdverseEvent,
    crate::r5::resources::AllergyIntolerance,
    crate::r5::resources::Appointment,
    crate::r5::resources::AppointmentResponse,
    crate::r5::resources::ArtifactAssessment,
    crate::r5::resources::AuditEvent,
    crate::r5::resources::Basic,
    crate::r5::resources::BiologicallyDerivedProduct,
    crate::r5::resources::BiologicallyDerivedProductDispense,
    crate::r5::resources::BodyStructure,
    crate::r5::resources::CapabilityStatement,
    crate::r5::resources::CarePlan,
    crate::r5::resources::CareTeam,
    crate::r5::resources::ChargeItem,
    crate::r5::resources::ChargeItemDefinition,
    crate::r5::resources::Citation,
    crate::r5::resources::Claim,
    crate::r5::resources::ClaimResponse,
    crate::r5::resources::ClinicalImpression,
    crate::r5::resources::ClinicalUseDefinition,
    crate::r5::resources::CodeSystem,
    crate::r5::resources::Communication,
    crate::r5::resources::CommunicationRequest,
    crate::r5::resources::CompartmentDefinition,
    crate::r5::resources::Composition,
    crate::r5::resources::ConceptMap,
    crate::r5::resources::Condition,
    crate::r5::resources::ConditionDefinition,
    crate::r5::resources::Consent,
    crate::r5::resources::Contract,
    crate::r5::resources::Coverage,
    crate::r5::resources::CoverageEligibilityRequest,
    crate::r5::resources::CoverageEligibilityResponse,
    crate::r5::resources::DetectedIssue,
    crate::r5::resources::Device,
    crate::r5::resources::DeviceAssociation,
    crate::r5::resources::DeviceDefinition,
    crate::r5::resources::DeviceDispense,
    crate::r5::resources::DeviceMetric,
    crate::r5::resources::DeviceRequest,
    crate::r5::resources::DeviceUsage,
    crate::r5::resources::DiagnosticReport,
    crate::r5::resources::DocumentReference,
    crate::r5::resources::Encounter,
    crate::r5::resources::EncounterHistory,
    crate::r5::resources::Endpoint,
    crate::r5::resources::EnrollmentRequest,
    crate::r5::resources::EnrollmentResponse,
    crate::r5::resources::EpisodeOfCare,
    crate::r5::resources::EventDefinition,
    crate::r5::resources::Evidence,
    crate::r5::resources::EvidenceReport,
    crate::r5::resources::EvidenceVariable,
    crate::r5::resources::ExampleScenario,
    crate::r5::resources::ExplanationOfBenefit,
    crate::r5::resources::FamilyMemberHistory,
    crate::r5::resources::Flag,
    crate::r5::resources::FormularyItem,
    crate::r5::resources::GenomicStudy,
    crate::r5::resources::Goal,
    crate::r5::resources::GraphDefinition,
    crate::r5::resources::Group,
    crate::r5::resources::GuidanceResponse,
    crate::r5::resources::HealthcareService,
    crate::r5::resources::ImagingSelection,
    crate::r5::resources::ImagingStudy,
    crate::r5::resources::Immunization,
    crate::r5::resources::ImmunizationEvaluation,
    crate::r5::resources::ImmunizationRecommendation,
    crate::r5::resources::ImplementationGuide,
    crate::r5::resources::Ingredient,
    crate::r5::resources::InsurancePlan,
    crate::r5::resources::InventoryItem,
    crate::r5::resources::InventoryReport,
    crate::r5::resources::Invoice,
    crate::r5::resources::Library,
    crate::r5::resources::Linkage,
    crate::r5::resources::List,
    crate::r5::resources::Location,
    crate::r5::resources::ManufacturedItemDefinition,
    crate::r5::resources::Measure,
    crate::r5::resources::MeasureReport,
    crate::r5::resources::Medication,
    crate::r5::resources::MedicationAdministration,
    crate::r5::resources::MedicationDispense,
    crate::r5::resources::MedicationKnowledge,
    crate::r5::resources::MedicationRequest,
    crate::r5::resources::MedicationStatement,
    crate::r5::resources::MedicinalProductDefinition,
    crate::r5::resources::MessageDefinition,
    crate::r5::resources::MessageHeader,
    crate::r5::resources::MolecularSequence,
    crate::r5::resources::NamingSystem,
    crate::r5::resources::NutritionIntake,
    crate::r5::resources::NutritionOrder,
    crate::r5::resources::NutritionProduct,
    crate::r5::resources::Observation,
    crate::r5::resources::ObservationDefinition,
    crate::r5::resources::OperationDefinition,
    crate::r5::resources::OperationOutcome,
    crate::r5::resources::Organization,
    crate::r5::resources::OrganizationAffiliation,
    crate::r5::resources::PackagedProductDefinition,
    crate::r5::resources::Patient,
    crate::r5::resources::PaymentNotice,
    crate::r5::resources::PaymentReconciliation,
    crate::r5::resources::Permission,
    crate::r5::resources::Person,
    crate::r5::resources::PlanDefinition,
    crate::r5::resources::Practitioner,
    crate::r5::resources::PractitionerRole,
    crate::r5::resources::Procedure,
    crate::r5::resources::Provenance,
    crate::r5::resources::Questionnaire,
    crate::r5::resources::QuestionnaireResponse,
    crate::r5::resources::RegulatedAuthorization,
    crate::r5::resources::RelatedPerson,
    crate::r5::resources::RequestOrchestration,
    crate::r5::resources::Requirements,
    crate::r5::resources::ResearchStudy,
    crate::r5::resources::ResearchSubject,
    crate::r5::resources::RiskAssessment,
    crate::r5::resources::Schedule,
    crate::r5::resources::SearchParameter,
    crate::r5::resources::ServiceRequest,
    crate::r5::resources::Slot,
    crate::r5::resources::Specimen,
    crate::r5::resources::SpecimenDefinition,
    crate::r5::resources::StructureDefinition,
    crate::r5::resources::StructureMap,
    crate::r5::resources::Subscription,
    crate::r5::resources::SubscriptionStatus,
    crate::r5::resources::SubscriptionTopic,
    crate::r5::resources::Substance,
    crate::r5::resources::SubstanceDefinition,
    crate::r5::resources::SubstanceNucleicAcid,
    crate::r5::resources::SubstancePolymer,
    crate::r5::resources::SubstanceProtein,
    crate::r5::resources::SubstanceReferenceInformation,
    crate::r5::resources::SubstanceSourceMaterial,
    crate::r5::resources::SupplyDelivery,
    crate::r5::resources::SupplyRequest,
    crate::r5::resources::Task,
    crate::r5::resources::TerminologyCapabilities,
    crate::r5::resources::TestPlan,
    crate::r5::resources::TestReport,
    crate::r5::resources::TestScript,
    crate::r5::resources::Transport,
    crate::r5::resources::ValueSet,
    crate::r5::resources::VerificationResult,
    crate::r5::resources::VisionPrescription,
    crate::r5::types::Address,
    crate::r5::types::Age,
    crate::r5::types::Annotation,
    crate::r5::types::Attachment,
    crate::r5::types::Availability,
    crate::r5::types::CodeableConcept,
    crate::r5::types::CodeableReference,
    crate::r5::types::Coding,
    crate::r5::types::ContactDetail,
    crate::r5::types::ContactPoint,
    crate::r5::types::Contributor,
    crate::r5::types::Count,
    crate::r5::types::DataRequirement,
    crate::r5::types::Distance,
    crate::r5::types::Dosage,
    crate::r5::types::Duration,
    crate::r5::types::Element,
    crate::r5::types::ElementDefinition,
    crate::r5::types::Expression,
    crate::r5::types::ExtendedContactDetail,
    crate::r5::types::Extension,
    crate::r5::types::HumanName,
    crate::r5::types::Identifier,
    crate::r5::types::MarketingStatus,
    crate::r5::types::Meta,
    crate::r5::types::MonetaryComponent,
    crate::r5::types::Money,
    crate::r5::types::MoneyQuantity,
    crate::r5::types::Narrative,
    crate::r5::types::ParameterDefinition,
    crate::r5::types::Period,
    crate::r5::types::ProductShelfLife,
    crate::r5::types::Quantity,
    crate::r5::types::Range,
    crate::r5::types::Ratio,
    crate::r5::types::RatioRange,
    crate::r5::types::RelatedArtifact,
    crate::r5::types::SampledData,
    crate::r5::types::Signature,
    crate::r5::types::SimpleQuantity,
    crate::r5::types::Timing,
    crate::r5::types::TriggerDefinition,
    crate::r5::types::UsageContext,
    crate::r5::types::VirtualServiceDetail,
);

impl_has_modifier_extension!(
    crate::r5::resources::Account,
    crate::r5::resources::ActivityDefinition,
    crate::r5::resources::ActorDefinition,
    crate::r5::resources::AdministrableProductDefinition,
    crate::r5::resources::AdverseEvent,
    crate::r5::resources::AllergyIntolerance,
    crate::r5::resources::Appointment,
    crate::r5::resources::AppointmentResponse,
    crate::r5::resources::ArtifactAssessment,
    crate::r5::resources::AuditEvent,
    crate::r5::resources::Basic,
    crate::r5::resources::BiologicallyDerivedProduct,
    crate::r5::resources::BiologicallyDerivedProductDispense,
    crate::r5::resources::BodyStructure,
    crate::r5::resources::CapabilityStatement,
    crate::r5::resources::CarePlan,
    crate::r5::resources::CareTeam,
    crate::r5::resources::ChargeItem,
    crate::r5::resources::ChargeItemDefinition,
    crate::r5::resources::Citation,
    crate::r5::resources::Claim,
    crate::r5::resources::ClaimResponse,
    crate::r5::resources::ClinicalImpression,
    crate::r5::resources::ClinicalUseDefinition,
    crate::r5::resources::CodeSystem,
    crate::r5::resources::Communication,
    crate::r5::resources::CommunicationRequest,
    crate::r5::resources::CompartmentDefinition,
    crate::r5::resources::Composition,
    crate::r5::resources::ConceptMap,
    crate::r5::resources::Condition,
    crate::r5::resources::ConditionDefinition,
    crate::r5::resources::Consent,
    crate::r5::resources::Contract,
    crate::r5::resources::Coverage,
    crate::r5::resources::CoverageEligibilityRequest,
    crate::r5::resources::CoverageEligibilityResponse,
    crate::r5::resources::DetectedIssue,
    crate::r5::resources::Device,
    crate::r5::resources::DeviceAssociation,
    crate::r5::resources::DeviceDefinition,
    crate::r5::resources::DeviceDispense,
    crate::r5::resources::DeviceMetric,
    crate::r5::resources::DeviceRequest,
    crate::r5::resources::DeviceUsage,
    crate::r5::resources::DiagnosticReport,
    crate::r5::resources::DocumentReference,
    crate::r5::resources::Encounter,
    crate::r5::resources::EncounterHistory,
    crate::r5::resources::Endpoint,
    crate::r5::resources::EnrollmentRequest,
    crate::r5::resources::EnrollmentResponse,
    crate::r5::resources::EpisodeOfCare,
    crate::r5::resources::EventDefinition,
    crate::r5::resources::Evidence,
    crate::r5::resources::EvidenceReport,
    crate::r5::resources::EvidenceVariable,
    crate::r5::resources::ExampleScenario,
    crate::r5::resources::ExplanationOfBenefit,
    crate::r5::resources::FamilyMemberHistory,
    crate::r5::resources::Flag,
    crate::r5::resources::FormularyItem,
    crate::r5::resources::GenomicStudy,
    crate::r5::resources::Goal,
    crate::r5::resources::GraphDefinition,
    crate::r5::resources::Group,
    crate::r5::resources::GuidanceResponse,
    crate::r5::resources::HealthcareService,
    crate::r5::resources::ImagingSelection,
    crate::r5::resources::ImagingStudy,
    crate::r5::resources::Immunization,
    crate::r5::resources::ImmunizationEvaluation,
    crate::r5::resources::ImmunizationRecommendation,
    crate::r5::resources::ImplementationGuide,
    crate::r5::resources::Ingredient,
    crate::r5::resources::InsurancePlan,
    crate::r5::resources::InventoryItem,
    crate::r5::resources::InventoryReport,
    crate::r5::resources::Invoice,
    crate::r5::resources::Library,
    crate::r5::resources::Linkage,
    crate::r5::resources::List,
    crate::r5::resources::Location,
    crate::r5::resources::ManufacturedItemDefinition,
    crate::r5::resources::Measure,
    crate::r5::resources::MeasureReport,
    crate::r5::resources::Medication,
    crate::r5::resources::MedicationAdministration,
    crate::r5::resources::MedicationDispense,
    crate::r5::resources::MedicationKnowledge,
    crate::r5::resources::MedicationRequest,
    crate::r5::resources::MedicationStatement,
    crate::r5::resources::MedicinalProductDefinition,
    crate::r5::resources::MessageDefinition,
    crate::r5::resources::MessageHeader,
    crate::r5::resources::MolecularSequence,
    crate::r5::resources::NamingSystem,
    crate::r5::resources::NutritionIntake,
    crate::r5::resources::NutritionOrder,
    crate::r5::resources::NutritionProduct,
    crate::r5::resources::Observation,
    crate::r5::resources::ObservationDefinition,
    crate::r5::resources::OperationDefinition,
    crate::r5::resources::OperationOutcome,
    crate::r5::resources::Organization,
    crate::r5::resources::OrganizationAffiliation,
    crate::r5::resources::PackagedProductDefinition,
    crate::r5::resources::Patient,
    crate::r5::resources::PaymentNotice,
    crate::r5::resources::PaymentReconciliation,
    crate::r5::resources::Permission,
    crate::r5::resources::Person,
    crate::r5::resources::PlanDefinition,
    crate::r5::resources::Practitioner,
    crate::r5::resources::PractitionerRole,
    crate::r5::resources::Procedure,
    crate::r5::resources::Provenance,
    crate::r5::resources::Questionnaire,
    crate::r5::resources::QuestionnaireResponse,
    crate::r5::resources::RegulatedAuthorization,
    crate::r5::resources::RelatedPerson,
    crate::r5::resources::RequestOrchestration,
    crate::r5::resources::Requirements,
    crate::r5::resources::ResearchStudy,
    crate::r5::resources::ResearchSubject,
    crate::r5::resources::RiskAssessment,
    crate::r5::resources::Schedule,
    crate::r5::resources::SearchParameter,
    crate::r5::resources::ServiceRequest,
    crate::r5::resources::Slot,
    crate::r5::resources::Specimen,
    crate::r5::resources::SpecimenDefinition,
    crate::r5::resources::StructureDefinition,
    crate::r5::resources::StructureMap,
    crate::r5::resources::Subscription,
    crate::r5::resources::SubscriptionStatus,
    crate::r5::resources::SubscriptionTopic,
    crate::r5::resources::Substance,
    crate::r5::resources::SubstanceDefinition,
    crate::r5::resources::SubstanceNucleicAcid,
    crate::r5::resources::SubstancePolymer,
    crate::r5::resources::SubstanceProtein,
    crate::r5::resources::SubstanceReferenceInformation,
    crate::r5::resources::SubstanceSourceMaterial,
    crate::r5::resources::SupplyDelivery,
    crate::r5::resources::SupplyRequest,
    crate::r5::resources::Task,
    crate::r5::resources::TerminologyCapabilities,
    crate::r5::resources::TestPlan,
    crate::r5::resources::TestReport,
    crate::r5::resources::TestScript,
    crate::r5::resources::Transport,
    crate::r5::resources::ValueSet,
    crate::r5::resources::VerificationResult,
    crate::r5::resources::VisionPrescription
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r5::resources::Patient;
    use crate::r5::types::String as FhirString;

    fn ext(url: &str) -> Extension {
        Extension {
            url: FhirString(url.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn set_get_and_replace() {
        let mut p = Patient::default();
        assert!(p.extension("u").is_none());
        p.set_extension(ext("u"));
        p.set_extension(ext("u")); // replaces, not duplicates
        assert_eq!(p.extensions("u").len(), 1);
        p.add_extension(ext("u")); // appends
        assert_eq!(p.extensions("u").len(), 2);
        assert!(p.extension("u").is_some());
    }

    #[test]
    fn modifier_extension_accessor() {
        let mut p = Patient::default();
        p.set_modifier_extension(ext("m"));
        assert!(p.modifier_extension("m").is_some());
    }
}
