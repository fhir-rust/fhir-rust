//! Ergonomic extension accessors: the [`ExtensionExt`] and
//! [`ModifierExtensionExt`] traits.
//!
//! FHIR extensions are a `Vec<Extension>` keyed by `url`. These traits add the
//! everyday operations — find an extension by url, iterate all with a url, and
//! set/add — to every R4 resource and datatype that carries extensions.
//!
//! ```
//! use fhir::r4::resources::Patient;
//! use fhir::r4::types::{Extension, String};
//! use fhir::r4::extension_ext::ExtensionExt;
//!
//! let mut patient = Patient::default();
//! patient.set_extension(Extension {
//!     url: String("http://example.org/eye-color".to_string()),
//!     ..Default::default()
//! });
//! assert!(patient.extension("http://example.org/eye-color").is_some());
//! assert!(patient.extension("http://other").is_none());
//! ```

use crate::r4::types::Extension;

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
    crate::r4::types::Address,
    crate::r4::types::Age,
    crate::r4::types::Annotation,
    crate::r4::types::Attachment,
    crate::r4::types::BackboneElement,
    crate::r4::types::CodeableConcept,
    crate::r4::types::Coding,
    crate::r4::types::ContactDetail,
    crate::r4::types::ContactPoint,
    crate::r4::types::Contributor,
    crate::r4::types::Count,
    crate::r4::types::DataRequirement,
    crate::r4::types::Distance,
    crate::r4::types::Dosage,
    crate::r4::types::Duration,
    crate::r4::types::Element,
    crate::r4::types::ElementDefinition,
    crate::r4::types::Expression,
    crate::r4::types::Extension,
    crate::r4::types::HumanName,
    crate::r4::types::Identifier,
    crate::r4::types::MarketingStatus,
    crate::r4::types::Meta,
    crate::r4::types::Money,
    crate::r4::types::MoneyQuantity,
    crate::r4::types::Narrative,
    crate::r4::types::ParameterDefinition,
    crate::r4::types::Period,
    crate::r4::types::Population,
    crate::r4::types::ProdCharacteristic,
    crate::r4::types::ProductShelfLife,
    crate::r4::types::Quantity,
    crate::r4::types::Range,
    crate::r4::types::Ratio,
    crate::r4::types::Reference,
    crate::r4::types::RelatedArtifact,
    crate::r4::types::SampledData,
    crate::r4::types::Signature,
    crate::r4::types::SimpleQuantity,
    crate::r4::types::SubstanceAmount,
    crate::r4::types::Timing,
    crate::r4::types::TriggerDefinition,
    crate::r4::types::UsageContext,
    crate::r4::resources::Account,
    crate::r4::resources::ActivityDefinition,
    crate::r4::resources::AdverseEvent,
    crate::r4::resources::AllergyIntolerance,
    crate::r4::resources::Appointment,
    crate::r4::resources::AppointmentResponse,
    crate::r4::resources::AuditEvent,
    crate::r4::resources::Basic,
    crate::r4::resources::BiologicallyDerivedProduct,
    crate::r4::resources::BodyStructure,
    crate::r4::resources::CapabilityStatement,
    crate::r4::resources::CarePlan,
    crate::r4::resources::CareTeam,
    crate::r4::resources::CatalogEntry,
    crate::r4::resources::ChargeItem,
    crate::r4::resources::ChargeItemDefinition,
    crate::r4::resources::Claim,
    crate::r4::resources::ClaimResponse,
    crate::r4::resources::ClinicalImpression,
    crate::r4::resources::CodeSystem,
    crate::r4::resources::Communication,
    crate::r4::resources::CommunicationRequest,
    crate::r4::resources::CompartmentDefinition,
    crate::r4::resources::Composition,
    crate::r4::resources::ConceptMap,
    crate::r4::resources::Condition,
    crate::r4::resources::Consent,
    crate::r4::resources::Contract,
    crate::r4::resources::Coverage,
    crate::r4::resources::CoverageEligibilityRequest,
    crate::r4::resources::CoverageEligibilityResponse,
    crate::r4::resources::DetectedIssue,
    crate::r4::resources::Device,
    crate::r4::resources::DeviceDefinition,
    crate::r4::resources::DeviceMetric,
    crate::r4::resources::DeviceRequest,
    crate::r4::resources::DeviceUseStatement,
    crate::r4::resources::DiagnosticReport,
    crate::r4::resources::DocumentManifest,
    crate::r4::resources::DocumentReference,
    crate::r4::resources::EffectEvidenceSynthesis,
    crate::r4::resources::Encounter,
    crate::r4::resources::Endpoint,
    crate::r4::resources::EnrollmentRequest,
    crate::r4::resources::EnrollmentResponse,
    crate::r4::resources::EpisodeOfCare,
    crate::r4::resources::EventDefinition,
    crate::r4::resources::Evidence,
    crate::r4::resources::EvidenceVariable,
    crate::r4::resources::ExampleScenario,
    crate::r4::resources::ExplanationOfBenefit,
    crate::r4::resources::FamilyMemberHistory,
    crate::r4::resources::Flag,
    crate::r4::resources::Goal,
    crate::r4::resources::GraphDefinition,
    crate::r4::resources::Group,
    crate::r4::resources::GuidanceResponse,
    crate::r4::resources::HealthcareService,
    crate::r4::resources::ImagingStudy,
    crate::r4::resources::Immunization,
    crate::r4::resources::ImmunizationEvaluation,
    crate::r4::resources::ImmunizationRecommendation,
    crate::r4::resources::ImplementationGuide,
    crate::r4::resources::InsurancePlan,
    crate::r4::resources::Invoice,
    crate::r4::resources::Library,
    crate::r4::resources::Linkage,
    crate::r4::resources::List,
    crate::r4::resources::Location,
    crate::r4::resources::Measure,
    crate::r4::resources::MeasureReport,
    crate::r4::resources::Media,
    crate::r4::resources::Medication,
    crate::r4::resources::MedicationAdministration,
    crate::r4::resources::MedicationDispense,
    crate::r4::resources::MedicationKnowledge,
    crate::r4::resources::MedicationRequest,
    crate::r4::resources::MedicationStatement,
    crate::r4::resources::MedicinalProduct,
    crate::r4::resources::MedicinalProductAuthorization,
    crate::r4::resources::MedicinalProductContraindication,
    crate::r4::resources::MedicinalProductIndication,
    crate::r4::resources::MedicinalProductIngredient,
    crate::r4::resources::MedicinalProductInteraction,
    crate::r4::resources::MedicinalProductManufactured,
    crate::r4::resources::MedicinalProductPackaged,
    crate::r4::resources::MedicinalProductPharmaceutical,
    crate::r4::resources::MedicinalProductUndesirableEffect,
    crate::r4::resources::MessageDefinition,
    crate::r4::resources::MessageHeader,
    crate::r4::resources::MolecularSequence,
    crate::r4::resources::NamingSystem,
    crate::r4::resources::NutritionOrder,
    crate::r4::resources::Observation,
    crate::r4::resources::ObservationDefinition,
    crate::r4::resources::OperationDefinition,
    crate::r4::resources::OperationOutcome,
    crate::r4::resources::Organization,
    crate::r4::resources::OrganizationAffiliation,
    crate::r4::resources::Patient,
    crate::r4::resources::PaymentNotice,
    crate::r4::resources::PaymentReconciliation,
    crate::r4::resources::Person,
    crate::r4::resources::PlanDefinition,
    crate::r4::resources::Practitioner,
    crate::r4::resources::PractitionerRole,
    crate::r4::resources::Procedure,
    crate::r4::resources::Provenance,
    crate::r4::resources::Questionnaire,
    crate::r4::resources::QuestionnaireResponse,
    crate::r4::resources::RelatedPerson,
    crate::r4::resources::RequestGroup,
    crate::r4::resources::ResearchDefinition,
    crate::r4::resources::ResearchElementDefinition,
    crate::r4::resources::ResearchStudy,
    crate::r4::resources::ResearchSubject,
    crate::r4::resources::RiskAssessment,
    crate::r4::resources::RiskEvidenceSynthesis,
    crate::r4::resources::Schedule,
    crate::r4::resources::SearchParameter,
    crate::r4::resources::ServiceRequest,
    crate::r4::resources::Slot,
    crate::r4::resources::Specimen,
    crate::r4::resources::SpecimenDefinition,
    crate::r4::resources::StructureDefinition,
    crate::r4::resources::StructureMap,
    crate::r4::resources::Subscription,
    crate::r4::resources::Substance,
    crate::r4::resources::SubstanceNucleicAcid,
    crate::r4::resources::SubstancePolymer,
    crate::r4::resources::SubstanceProtein,
    crate::r4::resources::SubstanceReferenceInformation,
    crate::r4::resources::SubstanceSourceMaterial,
    crate::r4::resources::SubstanceSpecification,
    crate::r4::resources::SupplyDelivery,
    crate::r4::resources::SupplyRequest,
    crate::r4::resources::Task,
    crate::r4::resources::TerminologyCapabilities,
    crate::r4::resources::TestReport,
    crate::r4::resources::TestScript,
    crate::r4::resources::ValueSet,
    crate::r4::resources::VerificationResult,
    crate::r4::resources::VisionPrescription,
);

impl_has_modifier_extension!(
    crate::r4::types::BackboneElement,
    crate::r4::types::Dosage,
    crate::r4::types::ElementDefinition,
    crate::r4::types::MarketingStatus,
    crate::r4::types::Population,
    crate::r4::types::ProdCharacteristic,
    crate::r4::types::ProductShelfLife,
    crate::r4::types::SubstanceAmount,
    crate::r4::types::Timing,
    crate::r4::resources::Account,
    crate::r4::resources::ActivityDefinition,
    crate::r4::resources::AdverseEvent,
    crate::r4::resources::AllergyIntolerance,
    crate::r4::resources::Appointment,
    crate::r4::resources::AppointmentResponse,
    crate::r4::resources::AuditEvent,
    crate::r4::resources::Basic,
    crate::r4::resources::BiologicallyDerivedProduct,
    crate::r4::resources::BodyStructure,
    crate::r4::resources::CapabilityStatement,
    crate::r4::resources::CarePlan,
    crate::r4::resources::CareTeam,
    crate::r4::resources::CatalogEntry,
    crate::r4::resources::ChargeItem,
    crate::r4::resources::ChargeItemDefinition,
    crate::r4::resources::Claim,
    crate::r4::resources::ClaimResponse,
    crate::r4::resources::ClinicalImpression,
    crate::r4::resources::CodeSystem,
    crate::r4::resources::Communication,
    crate::r4::resources::CommunicationRequest,
    crate::r4::resources::CompartmentDefinition,
    crate::r4::resources::Composition,
    crate::r4::resources::ConceptMap,
    crate::r4::resources::Condition,
    crate::r4::resources::Consent,
    crate::r4::resources::Contract,
    crate::r4::resources::Coverage,
    crate::r4::resources::CoverageEligibilityRequest,
    crate::r4::resources::CoverageEligibilityResponse,
    crate::r4::resources::DetectedIssue,
    crate::r4::resources::Device,
    crate::r4::resources::DeviceDefinition,
    crate::r4::resources::DeviceMetric,
    crate::r4::resources::DeviceRequest,
    crate::r4::resources::DeviceUseStatement,
    crate::r4::resources::DiagnosticReport,
    crate::r4::resources::DocumentManifest,
    crate::r4::resources::DocumentReference,
    crate::r4::resources::EffectEvidenceSynthesis,
    crate::r4::resources::Encounter,
    crate::r4::resources::Endpoint,
    crate::r4::resources::EnrollmentRequest,
    crate::r4::resources::EnrollmentResponse,
    crate::r4::resources::EpisodeOfCare,
    crate::r4::resources::EventDefinition,
    crate::r4::resources::Evidence,
    crate::r4::resources::EvidenceVariable,
    crate::r4::resources::ExampleScenario,
    crate::r4::resources::ExplanationOfBenefit,
    crate::r4::resources::FamilyMemberHistory,
    crate::r4::resources::Flag,
    crate::r4::resources::Goal,
    crate::r4::resources::GraphDefinition,
    crate::r4::resources::Group,
    crate::r4::resources::GuidanceResponse,
    crate::r4::resources::HealthcareService,
    crate::r4::resources::ImagingStudy,
    crate::r4::resources::Immunization,
    crate::r4::resources::ImmunizationEvaluation,
    crate::r4::resources::ImmunizationRecommendation,
    crate::r4::resources::ImplementationGuide,
    crate::r4::resources::InsurancePlan,
    crate::r4::resources::Invoice,
    crate::r4::resources::Library,
    crate::r4::resources::Linkage,
    crate::r4::resources::List,
    crate::r4::resources::Location,
    crate::r4::resources::Measure,
    crate::r4::resources::MeasureReport,
    crate::r4::resources::Media,
    crate::r4::resources::Medication,
    crate::r4::resources::MedicationAdministration,
    crate::r4::resources::MedicationDispense,
    crate::r4::resources::MedicationKnowledge,
    crate::r4::resources::MedicationRequest,
    crate::r4::resources::MedicationStatement,
    crate::r4::resources::MedicinalProduct,
    crate::r4::resources::MedicinalProductAuthorization,
    crate::r4::resources::MedicinalProductContraindication,
    crate::r4::resources::MedicinalProductIndication,
    crate::r4::resources::MedicinalProductIngredient,
    crate::r4::resources::MedicinalProductInteraction,
    crate::r4::resources::MedicinalProductManufactured,
    crate::r4::resources::MedicinalProductPackaged,
    crate::r4::resources::MedicinalProductPharmaceutical,
    crate::r4::resources::MedicinalProductUndesirableEffect,
    crate::r4::resources::MessageDefinition,
    crate::r4::resources::MessageHeader,
    crate::r4::resources::MolecularSequence,
    crate::r4::resources::NamingSystem,
    crate::r4::resources::NutritionOrder,
    crate::r4::resources::Observation,
    crate::r4::resources::ObservationDefinition,
    crate::r4::resources::OperationDefinition,
    crate::r4::resources::OperationOutcome,
    crate::r4::resources::Organization,
    crate::r4::resources::OrganizationAffiliation,
    crate::r4::resources::Patient,
    crate::r4::resources::PaymentNotice,
    crate::r4::resources::PaymentReconciliation,
    crate::r4::resources::Person,
    crate::r4::resources::PlanDefinition,
    crate::r4::resources::Practitioner,
    crate::r4::resources::PractitionerRole,
    crate::r4::resources::Procedure,
    crate::r4::resources::Provenance,
    crate::r4::resources::Questionnaire,
    crate::r4::resources::QuestionnaireResponse,
    crate::r4::resources::RelatedPerson,
    crate::r4::resources::RequestGroup,
    crate::r4::resources::ResearchDefinition,
    crate::r4::resources::ResearchElementDefinition,
    crate::r4::resources::ResearchStudy,
    crate::r4::resources::ResearchSubject,
    crate::r4::resources::RiskAssessment,
    crate::r4::resources::RiskEvidenceSynthesis,
    crate::r4::resources::Schedule,
    crate::r4::resources::SearchParameter,
    crate::r4::resources::ServiceRequest,
    crate::r4::resources::Slot,
    crate::r4::resources::Specimen,
    crate::r4::resources::SpecimenDefinition,
    crate::r4::resources::StructureDefinition,
    crate::r4::resources::StructureMap,
    crate::r4::resources::Subscription,
    crate::r4::resources::Substance,
    crate::r4::resources::SubstanceNucleicAcid,
    crate::r4::resources::SubstancePolymer,
    crate::r4::resources::SubstanceProtein,
    crate::r4::resources::SubstanceReferenceInformation,
    crate::r4::resources::SubstanceSourceMaterial,
    crate::r4::resources::SubstanceSpecification,
    crate::r4::resources::SupplyDelivery,
    crate::r4::resources::SupplyRequest,
    crate::r4::resources::Task,
    crate::r4::resources::TerminologyCapabilities,
    crate::r4::resources::TestReport,
    crate::r4::resources::TestScript,
    crate::r4::resources::ValueSet,
    crate::r4::resources::VerificationResult,
    crate::r4::resources::VisionPrescription,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4::resources::Patient;

    fn extension(url: &str) -> Extension {
        Extension {
            url: crate::r4::types::String(url.to_string()),
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
