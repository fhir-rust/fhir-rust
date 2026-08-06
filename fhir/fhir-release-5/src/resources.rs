//! FHIR R5 Resources
//!
//! This module contains the FHIR R5 resource types (Patient, Observation,
//! Encounter, and so on). Each resource is a Rust struct that serializes to
//! and from the canonical FHIR JSON representation via `serde`.

use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

pub mod account;
pub mod activity_definition;
pub mod actor_definition;
pub mod administrable_product_definition;
pub mod adverse_event;
pub mod allergy_intolerance;
pub mod appointment;
pub mod appointment_response;
pub mod artifact_assessment;
pub mod audit_event;
pub mod basic;
pub mod binary;
pub mod biologically_derived_product;
pub mod biologically_derived_product_dispense;
pub mod body_structure;
pub mod bundle;
pub mod capability_statement;
pub mod care_plan;
pub mod care_team;
pub mod charge_item;
pub mod charge_item_definition;
pub mod citation;
pub mod claim;
pub mod claim_response;
pub mod clinical_impression;
pub mod clinical_use_definition;
pub mod code_system;
pub mod communication;
pub mod communication_request;
pub mod compartment_definition;
pub mod composition;
pub mod concept_map;
pub mod condition;
pub mod condition_definition;
pub mod consent;
pub mod contract;
pub mod coverage;
pub mod coverage_eligibility_request;
pub mod coverage_eligibility_response;
pub mod detected_issue;
pub mod device;
pub mod device_association;
pub mod device_definition;
pub mod device_dispense;
pub mod device_metric;
pub mod device_request;
pub mod device_usage;
pub mod diagnostic_report;
pub mod document_reference;
pub mod encounter;
pub mod encounter_history;
pub mod endpoint;
pub mod enrollment_request;
pub mod enrollment_response;
pub mod episode_of_care;
pub mod event_definition;
pub mod evidence;
pub mod evidence_report;
pub mod evidence_variable;
pub mod example_scenario;
pub mod explanation_of_benefit;
pub mod family_member_history;
pub mod flag;
pub mod formulary_item;
pub mod genomic_study;
pub mod goal;
pub mod graph_definition;
pub mod group;
pub mod guidance_response;
pub mod healthcare_service;
pub mod imaging_selection;
pub mod imaging_study;
pub mod immunization;
pub mod immunization_evaluation;
pub mod immunization_recommendation;
pub mod implementation_guide;
pub mod ingredient;
pub mod insurance_plan;
pub mod inventory_item;
pub mod inventory_report;
pub mod invoice;
pub mod library;
pub mod linkage;
pub mod list;
pub mod location;
pub mod manufactured_item_definition;
pub mod measure;
pub mod measure_report;
pub mod medication;
pub mod medication_administration;
pub mod medication_dispense;
pub mod medication_knowledge;
pub mod medication_request;
pub mod medication_statement;
pub mod medicinal_product_definition;
pub mod message_definition;
pub mod message_header;
pub mod molecular_sequence;
pub mod naming_system;
pub mod nutrition_intake;
pub mod nutrition_order;
pub mod nutrition_product;
pub mod observation;
pub mod observation_definition;
pub mod operation_definition;
pub mod operation_outcome;
pub mod organization;
pub mod organization_affiliation;
pub mod packaged_product_definition;
pub mod parameters;
pub mod patient;
pub mod payment_notice;
pub mod payment_reconciliation;
pub mod permission;
pub mod person;
pub mod plan_definition;
pub mod practitioner;
pub mod practitioner_role;
pub mod procedure;
pub mod provenance;
pub mod questionnaire;
pub mod questionnaire_response;
pub mod regulated_authorization;
pub mod related_person;
pub mod request_orchestration;
pub mod requirements;
pub mod research_study;
pub mod research_subject;
pub mod risk_assessment;
pub mod schedule;
pub mod search_parameter;
pub mod service_request;
pub mod slot;
pub mod specimen;
pub mod specimen_definition;
pub mod structure_definition;
pub mod structure_map;
pub mod subscription;
pub mod subscription_status;
pub mod subscription_topic;
pub mod substance;
pub mod substance_definition;
pub mod substance_nucleic_acid;
pub mod substance_polymer;
pub mod substance_protein;
pub mod substance_reference_information;
pub mod substance_source_material;
pub mod supply_delivery;
pub mod supply_request;
pub mod task;
pub mod terminology_capabilities;
pub mod test_plan;
pub mod test_report;
pub mod test_script;
pub mod transport;
pub mod value_set;
pub mod verification_result;
pub mod vision_prescription;

/// Convenience
pub use account::Account;
pub use activity_definition::ActivityDefinition;
pub use actor_definition::ActorDefinition;
pub use administrable_product_definition::AdministrableProductDefinition;
pub use adverse_event::AdverseEvent;
pub use allergy_intolerance::AllergyIntolerance;
pub use appointment::Appointment;
pub use appointment_response::AppointmentResponse;
pub use artifact_assessment::ArtifactAssessment;
pub use audit_event::AuditEvent;
pub use basic::Basic;
pub use binary::Binary;
pub use biologically_derived_product::BiologicallyDerivedProduct;
pub use biologically_derived_product_dispense::BiologicallyDerivedProductDispense;
pub use body_structure::BodyStructure;
pub use bundle::Bundle;
pub use capability_statement::CapabilityStatement;
pub use care_plan::CarePlan;
pub use care_team::CareTeam;
pub use charge_item::ChargeItem;
pub use charge_item_definition::ChargeItemDefinition;
pub use citation::Citation;
pub use claim::Claim;
pub use claim_response::ClaimResponse;
pub use clinical_impression::ClinicalImpression;
pub use clinical_use_definition::ClinicalUseDefinition;
pub use code_system::CodeSystem;
pub use communication::Communication;
pub use communication_request::CommunicationRequest;
pub use compartment_definition::CompartmentDefinition;
pub use composition::Composition;
pub use concept_map::ConceptMap;
pub use condition::Condition;
pub use condition_definition::ConditionDefinition;
pub use consent::Consent;
pub use contract::Contract;
pub use coverage::Coverage;
pub use coverage_eligibility_request::CoverageEligibilityRequest;
pub use coverage_eligibility_response::CoverageEligibilityResponse;
pub use detected_issue::DetectedIssue;
pub use device::Device;
pub use device_association::DeviceAssociation;
pub use device_definition::DeviceDefinition;
pub use device_dispense::DeviceDispense;
pub use device_metric::DeviceMetric;
pub use device_request::DeviceRequest;
pub use device_usage::DeviceUsage;
pub use diagnostic_report::DiagnosticReport;
pub use document_reference::DocumentReference;
pub use encounter::Encounter;
pub use encounter_history::EncounterHistory;
pub use endpoint::Endpoint;
pub use enrollment_request::EnrollmentRequest;
pub use enrollment_response::EnrollmentResponse;
pub use episode_of_care::EpisodeOfCare;
pub use event_definition::EventDefinition;
pub use evidence::Evidence;
pub use evidence_report::EvidenceReport;
pub use evidence_variable::EvidenceVariable;
pub use example_scenario::ExampleScenario;
pub use explanation_of_benefit::ExplanationOfBenefit;
pub use family_member_history::FamilyMemberHistory;
pub use flag::Flag;
pub use formulary_item::FormularyItem;
pub use genomic_study::GenomicStudy;
pub use goal::Goal;
pub use graph_definition::GraphDefinition;
pub use group::Group;
pub use guidance_response::GuidanceResponse;
pub use healthcare_service::HealthcareService;
pub use imaging_selection::ImagingSelection;
pub use imaging_study::ImagingStudy;
pub use immunization::Immunization;
pub use immunization_evaluation::ImmunizationEvaluation;
pub use immunization_recommendation::ImmunizationRecommendation;
pub use implementation_guide::ImplementationGuide;
pub use ingredient::Ingredient;
pub use insurance_plan::InsurancePlan;
pub use inventory_item::InventoryItem;
pub use inventory_report::InventoryReport;
pub use invoice::Invoice;
pub use library::Library;
pub use linkage::Linkage;
pub use list::List;
pub use location::Location;
pub use manufactured_item_definition::ManufacturedItemDefinition;
pub use measure::Measure;
pub use measure_report::MeasureReport;
pub use medication::Medication;
pub use medication_administration::MedicationAdministration;
pub use medication_dispense::MedicationDispense;
pub use medication_knowledge::MedicationKnowledge;
pub use medication_request::MedicationRequest;
pub use medication_statement::MedicationStatement;
pub use medicinal_product_definition::MedicinalProductDefinition;
pub use message_definition::MessageDefinition;
pub use message_header::MessageHeader;
pub use molecular_sequence::MolecularSequence;
pub use naming_system::NamingSystem;
pub use nutrition_intake::NutritionIntake;
pub use nutrition_order::NutritionOrder;
pub use nutrition_product::NutritionProduct;
pub use observation::Observation;
pub use observation_definition::ObservationDefinition;
pub use operation_definition::OperationDefinition;
pub use operation_outcome::OperationOutcome;
pub use organization::Organization;
pub use organization_affiliation::OrganizationAffiliation;
pub use packaged_product_definition::PackagedProductDefinition;
pub use parameters::Parameters;
pub use patient::Patient;
pub use payment_notice::PaymentNotice;
pub use payment_reconciliation::PaymentReconciliation;
pub use permission::Permission;
pub use person::Person;
pub use plan_definition::PlanDefinition;
pub use practitioner::Practitioner;
pub use practitioner_role::PractitionerRole;
pub use procedure::Procedure;
pub use provenance::Provenance;
pub use questionnaire::Questionnaire;
pub use questionnaire_response::QuestionnaireResponse;
pub use regulated_authorization::RegulatedAuthorization;
pub use related_person::RelatedPerson;
pub use request_orchestration::RequestOrchestration;
pub use requirements::Requirements;
pub use research_study::ResearchStudy;
pub use research_subject::ResearchSubject;
pub use risk_assessment::RiskAssessment;
pub use schedule::Schedule;
pub use search_parameter::SearchParameter;
pub use service_request::ServiceRequest;
pub use slot::Slot;
pub use specimen::Specimen;
pub use specimen_definition::SpecimenDefinition;
pub use structure_definition::StructureDefinition;
pub use structure_map::StructureMap;
pub use subscription::Subscription;
pub use subscription_status::SubscriptionStatus;
pub use subscription_topic::SubscriptionTopic;
pub use substance::Substance;
pub use substance_definition::SubstanceDefinition;
pub use substance_nucleic_acid::SubstanceNucleicAcid;
pub use substance_polymer::SubstancePolymer;
pub use substance_protein::SubstanceProtein;
pub use substance_reference_information::SubstanceReferenceInformation;
pub use substance_source_material::SubstanceSourceMaterial;
pub use supply_delivery::SupplyDelivery;
pub use supply_request::SupplyRequest;
pub use task::Task;
pub use terminology_capabilities::TerminologyCapabilities;
pub use test_plan::TestPlan;
pub use test_report::TestReport;
pub use test_script::TestScript;
pub use transport::Transport;
pub use value_set::ValueSet;
pub use verification_result::VerificationResult;
pub use vision_prescription::VisionPrescription;

/// A polymorphic FHIR R5 resource, tagged by its `resourceType`.
///
/// Used wherever a resource of any type may appear — for example a
/// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes
/// the `resourceType` discriminator automatically.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::Resource;
///
/// let json = ::serde_json::json!({"resourceType": "Patient"});
/// let resource: Resource = ::serde_json::from_value(json).unwrap();
/// assert!(matches!(resource, Resource::Patient(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(tag = "resourceType")]
pub enum Resource {
    Account(Box<account::Account>),
    ActivityDefinition(Box<activity_definition::ActivityDefinition>),
    ActorDefinition(Box<actor_definition::ActorDefinition>),
    AdministrableProductDefinition(
        Box<administrable_product_definition::AdministrableProductDefinition>,
    ),
    AdverseEvent(Box<adverse_event::AdverseEvent>),
    AllergyIntolerance(Box<allergy_intolerance::AllergyIntolerance>),
    Appointment(Box<appointment::Appointment>),
    AppointmentResponse(Box<appointment_response::AppointmentResponse>),
    ArtifactAssessment(Box<artifact_assessment::ArtifactAssessment>),
    AuditEvent(Box<audit_event::AuditEvent>),
    Basic(Box<basic::Basic>),
    Binary(Box<binary::Binary>),
    BiologicallyDerivedProduct(Box<biologically_derived_product::BiologicallyDerivedProduct>),
    BiologicallyDerivedProductDispense(
        Box<biologically_derived_product_dispense::BiologicallyDerivedProductDispense>,
    ),
    BodyStructure(Box<body_structure::BodyStructure>),
    Bundle(Box<bundle::Bundle>),
    CapabilityStatement(Box<capability_statement::CapabilityStatement>),
    CarePlan(Box<care_plan::CarePlan>),
    CareTeam(Box<care_team::CareTeam>),
    ChargeItem(Box<charge_item::ChargeItem>),
    ChargeItemDefinition(Box<charge_item_definition::ChargeItemDefinition>),
    Citation(Box<citation::Citation>),
    Claim(Box<claim::Claim>),
    ClaimResponse(Box<claim_response::ClaimResponse>),
    ClinicalImpression(Box<clinical_impression::ClinicalImpression>),
    ClinicalUseDefinition(Box<clinical_use_definition::ClinicalUseDefinition>),
    CodeSystem(Box<code_system::CodeSystem>),
    Communication(Box<communication::Communication>),
    CommunicationRequest(Box<communication_request::CommunicationRequest>),
    CompartmentDefinition(Box<compartment_definition::CompartmentDefinition>),
    Composition(Box<composition::Composition>),
    ConceptMap(Box<concept_map::ConceptMap>),
    Condition(Box<condition::Condition>),
    ConditionDefinition(Box<condition_definition::ConditionDefinition>),
    Consent(Box<consent::Consent>),
    Contract(Box<contract::Contract>),
    Coverage(Box<coverage::Coverage>),
    CoverageEligibilityRequest(Box<coverage_eligibility_request::CoverageEligibilityRequest>),
    CoverageEligibilityResponse(Box<coverage_eligibility_response::CoverageEligibilityResponse>),
    DetectedIssue(Box<detected_issue::DetectedIssue>),
    Device(Box<device::Device>),
    DeviceAssociation(Box<device_association::DeviceAssociation>),
    DeviceDefinition(Box<device_definition::DeviceDefinition>),
    DeviceDispense(Box<device_dispense::DeviceDispense>),
    DeviceMetric(Box<device_metric::DeviceMetric>),
    DeviceRequest(Box<device_request::DeviceRequest>),
    DeviceUsage(Box<device_usage::DeviceUsage>),
    DiagnosticReport(Box<diagnostic_report::DiagnosticReport>),
    DocumentReference(Box<document_reference::DocumentReference>),
    Encounter(Box<encounter::Encounter>),
    EncounterHistory(Box<encounter_history::EncounterHistory>),
    Endpoint(Box<endpoint::Endpoint>),
    EnrollmentRequest(Box<enrollment_request::EnrollmentRequest>),
    EnrollmentResponse(Box<enrollment_response::EnrollmentResponse>),
    EpisodeOfCare(Box<episode_of_care::EpisodeOfCare>),
    EventDefinition(Box<event_definition::EventDefinition>),
    Evidence(Box<evidence::Evidence>),
    EvidenceReport(Box<evidence_report::EvidenceReport>),
    EvidenceVariable(Box<evidence_variable::EvidenceVariable>),
    ExampleScenario(Box<example_scenario::ExampleScenario>),
    ExplanationOfBenefit(Box<explanation_of_benefit::ExplanationOfBenefit>),
    FamilyMemberHistory(Box<family_member_history::FamilyMemberHistory>),
    Flag(Box<flag::Flag>),
    FormularyItem(Box<formulary_item::FormularyItem>),
    GenomicStudy(Box<genomic_study::GenomicStudy>),
    Goal(Box<goal::Goal>),
    GraphDefinition(Box<graph_definition::GraphDefinition>),
    Group(Box<group::Group>),
    GuidanceResponse(Box<guidance_response::GuidanceResponse>),
    HealthcareService(Box<healthcare_service::HealthcareService>),
    ImagingSelection(Box<imaging_selection::ImagingSelection>),
    ImagingStudy(Box<imaging_study::ImagingStudy>),
    Immunization(Box<immunization::Immunization>),
    ImmunizationEvaluation(Box<immunization_evaluation::ImmunizationEvaluation>),
    ImmunizationRecommendation(Box<immunization_recommendation::ImmunizationRecommendation>),
    ImplementationGuide(Box<implementation_guide::ImplementationGuide>),
    Ingredient(Box<ingredient::Ingredient>),
    InsurancePlan(Box<insurance_plan::InsurancePlan>),
    InventoryItem(Box<inventory_item::InventoryItem>),
    InventoryReport(Box<inventory_report::InventoryReport>),
    Invoice(Box<invoice::Invoice>),
    Library(Box<library::Library>),
    Linkage(Box<linkage::Linkage>),
    List(Box<list::List>),
    Location(Box<location::Location>),
    ManufacturedItemDefinition(Box<manufactured_item_definition::ManufacturedItemDefinition>),
    Measure(Box<measure::Measure>),
    MeasureReport(Box<measure_report::MeasureReport>),
    Medication(Box<medication::Medication>),
    MedicationAdministration(Box<medication_administration::MedicationAdministration>),
    MedicationDispense(Box<medication_dispense::MedicationDispense>),
    MedicationKnowledge(Box<medication_knowledge::MedicationKnowledge>),
    MedicationRequest(Box<medication_request::MedicationRequest>),
    MedicationStatement(Box<medication_statement::MedicationStatement>),
    MedicinalProductDefinition(Box<medicinal_product_definition::MedicinalProductDefinition>),
    MessageDefinition(Box<message_definition::MessageDefinition>),
    MessageHeader(Box<message_header::MessageHeader>),
    MolecularSequence(Box<molecular_sequence::MolecularSequence>),
    NamingSystem(Box<naming_system::NamingSystem>),
    NutritionIntake(Box<nutrition_intake::NutritionIntake>),
    NutritionOrder(Box<nutrition_order::NutritionOrder>),
    NutritionProduct(Box<nutrition_product::NutritionProduct>),
    Observation(Box<observation::Observation>),
    ObservationDefinition(Box<observation_definition::ObservationDefinition>),
    OperationDefinition(Box<operation_definition::OperationDefinition>),
    OperationOutcome(Box<operation_outcome::OperationOutcome>),
    Organization(Box<organization::Organization>),
    OrganizationAffiliation(Box<organization_affiliation::OrganizationAffiliation>),
    PackagedProductDefinition(Box<packaged_product_definition::PackagedProductDefinition>),
    Parameters(Box<parameters::Parameters>),
    Patient(Box<patient::Patient>),
    PaymentNotice(Box<payment_notice::PaymentNotice>),
    PaymentReconciliation(Box<payment_reconciliation::PaymentReconciliation>),
    Permission(Box<permission::Permission>),
    Person(Box<person::Person>),
    PlanDefinition(Box<plan_definition::PlanDefinition>),
    Practitioner(Box<practitioner::Practitioner>),
    PractitionerRole(Box<practitioner_role::PractitionerRole>),
    Procedure(Box<procedure::Procedure>),
    Provenance(Box<provenance::Provenance>),
    Questionnaire(Box<questionnaire::Questionnaire>),
    QuestionnaireResponse(Box<questionnaire_response::QuestionnaireResponse>),
    RegulatedAuthorization(Box<regulated_authorization::RegulatedAuthorization>),
    RelatedPerson(Box<related_person::RelatedPerson>),
    RequestOrchestration(Box<request_orchestration::RequestOrchestration>),
    Requirements(Box<requirements::Requirements>),
    ResearchStudy(Box<research_study::ResearchStudy>),
    ResearchSubject(Box<research_subject::ResearchSubject>),
    RiskAssessment(Box<risk_assessment::RiskAssessment>),
    Schedule(Box<schedule::Schedule>),
    SearchParameter(Box<search_parameter::SearchParameter>),
    ServiceRequest(Box<service_request::ServiceRequest>),
    Slot(Box<slot::Slot>),
    Specimen(Box<specimen::Specimen>),
    SpecimenDefinition(Box<specimen_definition::SpecimenDefinition>),
    StructureDefinition(Box<structure_definition::StructureDefinition>),
    StructureMap(Box<structure_map::StructureMap>),
    Subscription(Box<subscription::Subscription>),
    SubscriptionStatus(Box<subscription_status::SubscriptionStatus>),
    SubscriptionTopic(Box<subscription_topic::SubscriptionTopic>),
    Substance(Box<substance::Substance>),
    SubstanceDefinition(Box<substance_definition::SubstanceDefinition>),
    SubstanceNucleicAcid(Box<substance_nucleic_acid::SubstanceNucleicAcid>),
    SubstancePolymer(Box<substance_polymer::SubstancePolymer>),
    SubstanceProtein(Box<substance_protein::SubstanceProtein>),
    SubstanceReferenceInformation(
        Box<substance_reference_information::SubstanceReferenceInformation>,
    ),
    SubstanceSourceMaterial(Box<substance_source_material::SubstanceSourceMaterial>),
    SupplyDelivery(Box<supply_delivery::SupplyDelivery>),
    SupplyRequest(Box<supply_request::SupplyRequest>),
    Task(Box<task::Task>),
    TerminologyCapabilities(Box<terminology_capabilities::TerminologyCapabilities>),
    TestPlan(Box<test_plan::TestPlan>),
    TestReport(Box<test_report::TestReport>),
    TestScript(Box<test_script::TestScript>),
    Transport(Box<transport::Transport>),
    ValueSet(Box<value_set::ValueSet>),
    VerificationResult(Box<verification_result::VerificationResult>),
    VisionPrescription(Box<vision_prescription::VisionPrescription>),
}

impl Resource {
    /// Whether this resource itself carries contained resources (`dom-2`).
    #[must_use]
    pub fn has_contained(&self) -> bool {
        match self {
            Self::Account(r) => !r.contained.is_empty(),
            Self::ActivityDefinition(r) => !r.contained.is_empty(),
            Self::ActorDefinition(r) => !r.contained.is_empty(),
            Self::AdministrableProductDefinition(r) => !r.contained.is_empty(),
            Self::AdverseEvent(r) => !r.contained.is_empty(),
            Self::AllergyIntolerance(r) => !r.contained.is_empty(),
            Self::Appointment(r) => !r.contained.is_empty(),
            Self::AppointmentResponse(r) => !r.contained.is_empty(),
            Self::ArtifactAssessment(r) => !r.contained.is_empty(),
            Self::AuditEvent(r) => !r.contained.is_empty(),
            Self::Basic(r) => !r.contained.is_empty(),
            Self::Binary(_) => false,
            Self::BiologicallyDerivedProduct(r) => !r.contained.is_empty(),
            Self::BiologicallyDerivedProductDispense(r) => !r.contained.is_empty(),
            Self::BodyStructure(r) => !r.contained.is_empty(),
            Self::Bundle(_) => false,
            Self::CapabilityStatement(r) => !r.contained.is_empty(),
            Self::CarePlan(r) => !r.contained.is_empty(),
            Self::CareTeam(r) => !r.contained.is_empty(),
            Self::ChargeItem(r) => !r.contained.is_empty(),
            Self::ChargeItemDefinition(r) => !r.contained.is_empty(),
            Self::Citation(r) => !r.contained.is_empty(),
            Self::Claim(r) => !r.contained.is_empty(),
            Self::ClaimResponse(r) => !r.contained.is_empty(),
            Self::ClinicalImpression(r) => !r.contained.is_empty(),
            Self::ClinicalUseDefinition(r) => !r.contained.is_empty(),
            Self::CodeSystem(r) => !r.contained.is_empty(),
            Self::Communication(r) => !r.contained.is_empty(),
            Self::CommunicationRequest(r) => !r.contained.is_empty(),
            Self::CompartmentDefinition(r) => !r.contained.is_empty(),
            Self::Composition(r) => !r.contained.is_empty(),
            Self::ConceptMap(r) => !r.contained.is_empty(),
            Self::Condition(r) => !r.contained.is_empty(),
            Self::ConditionDefinition(r) => !r.contained.is_empty(),
            Self::Consent(r) => !r.contained.is_empty(),
            Self::Contract(r) => !r.contained.is_empty(),
            Self::Coverage(r) => !r.contained.is_empty(),
            Self::CoverageEligibilityRequest(r) => !r.contained.is_empty(),
            Self::CoverageEligibilityResponse(r) => !r.contained.is_empty(),
            Self::DetectedIssue(r) => !r.contained.is_empty(),
            Self::Device(r) => !r.contained.is_empty(),
            Self::DeviceAssociation(r) => !r.contained.is_empty(),
            Self::DeviceDefinition(r) => !r.contained.is_empty(),
            Self::DeviceDispense(r) => !r.contained.is_empty(),
            Self::DeviceMetric(r) => !r.contained.is_empty(),
            Self::DeviceRequest(r) => !r.contained.is_empty(),
            Self::DeviceUsage(r) => !r.contained.is_empty(),
            Self::DiagnosticReport(r) => !r.contained.is_empty(),
            Self::DocumentReference(r) => !r.contained.is_empty(),
            Self::Encounter(r) => !r.contained.is_empty(),
            Self::EncounterHistory(r) => !r.contained.is_empty(),
            Self::Endpoint(r) => !r.contained.is_empty(),
            Self::EnrollmentRequest(r) => !r.contained.is_empty(),
            Self::EnrollmentResponse(r) => !r.contained.is_empty(),
            Self::EpisodeOfCare(r) => !r.contained.is_empty(),
            Self::EventDefinition(r) => !r.contained.is_empty(),
            Self::Evidence(r) => !r.contained.is_empty(),
            Self::EvidenceReport(r) => !r.contained.is_empty(),
            Self::EvidenceVariable(r) => !r.contained.is_empty(),
            Self::ExampleScenario(r) => !r.contained.is_empty(),
            Self::ExplanationOfBenefit(r) => !r.contained.is_empty(),
            Self::FamilyMemberHistory(r) => !r.contained.is_empty(),
            Self::Flag(r) => !r.contained.is_empty(),
            Self::FormularyItem(r) => !r.contained.is_empty(),
            Self::GenomicStudy(r) => !r.contained.is_empty(),
            Self::Goal(r) => !r.contained.is_empty(),
            Self::GraphDefinition(r) => !r.contained.is_empty(),
            Self::Group(r) => !r.contained.is_empty(),
            Self::GuidanceResponse(r) => !r.contained.is_empty(),
            Self::HealthcareService(r) => !r.contained.is_empty(),
            Self::ImagingSelection(r) => !r.contained.is_empty(),
            Self::ImagingStudy(r) => !r.contained.is_empty(),
            Self::Immunization(r) => !r.contained.is_empty(),
            Self::ImmunizationEvaluation(r) => !r.contained.is_empty(),
            Self::ImmunizationRecommendation(r) => !r.contained.is_empty(),
            Self::ImplementationGuide(r) => !r.contained.is_empty(),
            Self::Ingredient(r) => !r.contained.is_empty(),
            Self::InsurancePlan(r) => !r.contained.is_empty(),
            Self::InventoryItem(r) => !r.contained.is_empty(),
            Self::InventoryReport(r) => !r.contained.is_empty(),
            Self::Invoice(r) => !r.contained.is_empty(),
            Self::Library(r) => !r.contained.is_empty(),
            Self::Linkage(r) => !r.contained.is_empty(),
            Self::List(r) => !r.contained.is_empty(),
            Self::Location(r) => !r.contained.is_empty(),
            Self::ManufacturedItemDefinition(r) => !r.contained.is_empty(),
            Self::Measure(r) => !r.contained.is_empty(),
            Self::MeasureReport(r) => !r.contained.is_empty(),
            Self::Medication(r) => !r.contained.is_empty(),
            Self::MedicationAdministration(r) => !r.contained.is_empty(),
            Self::MedicationDispense(r) => !r.contained.is_empty(),
            Self::MedicationKnowledge(r) => !r.contained.is_empty(),
            Self::MedicationRequest(r) => !r.contained.is_empty(),
            Self::MedicationStatement(r) => !r.contained.is_empty(),
            Self::MedicinalProductDefinition(r) => !r.contained.is_empty(),
            Self::MessageDefinition(r) => !r.contained.is_empty(),
            Self::MessageHeader(r) => !r.contained.is_empty(),
            Self::MolecularSequence(r) => !r.contained.is_empty(),
            Self::NamingSystem(r) => !r.contained.is_empty(),
            Self::NutritionIntake(r) => !r.contained.is_empty(),
            Self::NutritionOrder(r) => !r.contained.is_empty(),
            Self::NutritionProduct(r) => !r.contained.is_empty(),
            Self::Observation(r) => !r.contained.is_empty(),
            Self::ObservationDefinition(r) => !r.contained.is_empty(),
            Self::OperationDefinition(r) => !r.contained.is_empty(),
            Self::OperationOutcome(r) => !r.contained.is_empty(),
            Self::Organization(r) => !r.contained.is_empty(),
            Self::OrganizationAffiliation(r) => !r.contained.is_empty(),
            Self::PackagedProductDefinition(r) => !r.contained.is_empty(),
            Self::Parameters(_) => false,
            Self::Patient(r) => !r.contained.is_empty(),
            Self::PaymentNotice(r) => !r.contained.is_empty(),
            Self::PaymentReconciliation(r) => !r.contained.is_empty(),
            Self::Permission(r) => !r.contained.is_empty(),
            Self::Person(r) => !r.contained.is_empty(),
            Self::PlanDefinition(r) => !r.contained.is_empty(),
            Self::Practitioner(r) => !r.contained.is_empty(),
            Self::PractitionerRole(r) => !r.contained.is_empty(),
            Self::Procedure(r) => !r.contained.is_empty(),
            Self::Provenance(r) => !r.contained.is_empty(),
            Self::Questionnaire(r) => !r.contained.is_empty(),
            Self::QuestionnaireResponse(r) => !r.contained.is_empty(),
            Self::RegulatedAuthorization(r) => !r.contained.is_empty(),
            Self::RelatedPerson(r) => !r.contained.is_empty(),
            Self::RequestOrchestration(r) => !r.contained.is_empty(),
            Self::Requirements(r) => !r.contained.is_empty(),
            Self::ResearchStudy(r) => !r.contained.is_empty(),
            Self::ResearchSubject(r) => !r.contained.is_empty(),
            Self::RiskAssessment(r) => !r.contained.is_empty(),
            Self::Schedule(r) => !r.contained.is_empty(),
            Self::SearchParameter(r) => !r.contained.is_empty(),
            Self::ServiceRequest(r) => !r.contained.is_empty(),
            Self::Slot(r) => !r.contained.is_empty(),
            Self::Specimen(r) => !r.contained.is_empty(),
            Self::SpecimenDefinition(r) => !r.contained.is_empty(),
            Self::StructureDefinition(r) => !r.contained.is_empty(),
            Self::StructureMap(r) => !r.contained.is_empty(),
            Self::Subscription(r) => !r.contained.is_empty(),
            Self::SubscriptionStatus(r) => !r.contained.is_empty(),
            Self::SubscriptionTopic(r) => !r.contained.is_empty(),
            Self::Substance(r) => !r.contained.is_empty(),
            Self::SubstanceDefinition(r) => !r.contained.is_empty(),
            Self::SubstanceNucleicAcid(r) => !r.contained.is_empty(),
            Self::SubstancePolymer(r) => !r.contained.is_empty(),
            Self::SubstanceProtein(r) => !r.contained.is_empty(),
            Self::SubstanceReferenceInformation(r) => !r.contained.is_empty(),
            Self::SubstanceSourceMaterial(r) => !r.contained.is_empty(),
            Self::SupplyDelivery(r) => !r.contained.is_empty(),
            Self::SupplyRequest(r) => !r.contained.is_empty(),
            Self::Task(r) => !r.contained.is_empty(),
            Self::TerminologyCapabilities(r) => !r.contained.is_empty(),
            Self::TestPlan(r) => !r.contained.is_empty(),
            Self::TestReport(r) => !r.contained.is_empty(),
            Self::TestScript(r) => !r.contained.is_empty(),
            Self::Transport(r) => !r.contained.is_empty(),
            Self::ValueSet(r) => !r.contained.is_empty(),
            Self::VerificationResult(r) => !r.contained.is_empty(),
            Self::VisionPrescription(r) => !r.contained.is_empty(),
        }
    }

    /// The resource's `meta`, when present (`dom-4`).
    #[must_use]
    pub fn meta(&self) -> Option<&crate::r5::types::Meta> {
        match self {
            Self::Account(r) => r.meta.as_ref(),
            Self::ActivityDefinition(r) => r.meta.as_ref(),
            Self::ActorDefinition(r) => r.meta.as_ref(),
            Self::AdministrableProductDefinition(r) => r.meta.as_ref(),
            Self::AdverseEvent(r) => r.meta.as_ref(),
            Self::AllergyIntolerance(r) => r.meta.as_ref(),
            Self::Appointment(r) => r.meta.as_ref(),
            Self::AppointmentResponse(r) => r.meta.as_ref(),
            Self::ArtifactAssessment(r) => r.meta.as_ref(),
            Self::AuditEvent(r) => r.meta.as_ref(),
            Self::Basic(r) => r.meta.as_ref(),
            Self::Binary(r) => r.meta.as_ref(),
            Self::BiologicallyDerivedProduct(r) => r.meta.as_ref(),
            Self::BiologicallyDerivedProductDispense(r) => r.meta.as_ref(),
            Self::BodyStructure(r) => r.meta.as_ref(),
            Self::Bundle(r) => r.meta.as_ref(),
            Self::CapabilityStatement(r) => r.meta.as_ref(),
            Self::CarePlan(r) => r.meta.as_ref(),
            Self::CareTeam(r) => r.meta.as_ref(),
            Self::ChargeItem(r) => r.meta.as_ref(),
            Self::ChargeItemDefinition(r) => r.meta.as_ref(),
            Self::Citation(r) => r.meta.as_ref(),
            Self::Claim(r) => r.meta.as_ref(),
            Self::ClaimResponse(r) => r.meta.as_ref(),
            Self::ClinicalImpression(r) => r.meta.as_ref(),
            Self::ClinicalUseDefinition(r) => r.meta.as_ref(),
            Self::CodeSystem(r) => r.meta.as_ref(),
            Self::Communication(r) => r.meta.as_ref(),
            Self::CommunicationRequest(r) => r.meta.as_ref(),
            Self::CompartmentDefinition(r) => r.meta.as_ref(),
            Self::Composition(r) => r.meta.as_ref(),
            Self::ConceptMap(r) => r.meta.as_ref(),
            Self::Condition(r) => r.meta.as_ref(),
            Self::ConditionDefinition(r) => r.meta.as_ref(),
            Self::Consent(r) => r.meta.as_ref(),
            Self::Contract(r) => r.meta.as_ref(),
            Self::Coverage(r) => r.meta.as_ref(),
            Self::CoverageEligibilityRequest(r) => r.meta.as_ref(),
            Self::CoverageEligibilityResponse(r) => r.meta.as_ref(),
            Self::DetectedIssue(r) => r.meta.as_ref(),
            Self::Device(r) => r.meta.as_ref(),
            Self::DeviceAssociation(r) => r.meta.as_ref(),
            Self::DeviceDefinition(r) => r.meta.as_ref(),
            Self::DeviceDispense(r) => r.meta.as_ref(),
            Self::DeviceMetric(r) => r.meta.as_ref(),
            Self::DeviceRequest(r) => r.meta.as_ref(),
            Self::DeviceUsage(r) => r.meta.as_ref(),
            Self::DiagnosticReport(r) => r.meta.as_ref(),
            Self::DocumentReference(r) => r.meta.as_ref(),
            Self::Encounter(r) => r.meta.as_ref(),
            Self::EncounterHistory(r) => r.meta.as_ref(),
            Self::Endpoint(r) => r.meta.as_ref(),
            Self::EnrollmentRequest(r) => r.meta.as_ref(),
            Self::EnrollmentResponse(r) => r.meta.as_ref(),
            Self::EpisodeOfCare(r) => r.meta.as_ref(),
            Self::EventDefinition(r) => r.meta.as_ref(),
            Self::Evidence(r) => r.meta.as_ref(),
            Self::EvidenceReport(r) => r.meta.as_ref(),
            Self::EvidenceVariable(r) => r.meta.as_ref(),
            Self::ExampleScenario(r) => r.meta.as_ref(),
            Self::ExplanationOfBenefit(r) => r.meta.as_ref(),
            Self::FamilyMemberHistory(r) => r.meta.as_ref(),
            Self::Flag(r) => r.meta.as_ref(),
            Self::FormularyItem(r) => r.meta.as_ref(),
            Self::GenomicStudy(r) => r.meta.as_ref(),
            Self::Goal(r) => r.meta.as_ref(),
            Self::GraphDefinition(r) => r.meta.as_ref(),
            Self::Group(r) => r.meta.as_ref(),
            Self::GuidanceResponse(r) => r.meta.as_ref(),
            Self::HealthcareService(r) => r.meta.as_ref(),
            Self::ImagingSelection(r) => r.meta.as_ref(),
            Self::ImagingStudy(r) => r.meta.as_ref(),
            Self::Immunization(r) => r.meta.as_ref(),
            Self::ImmunizationEvaluation(r) => r.meta.as_ref(),
            Self::ImmunizationRecommendation(r) => r.meta.as_ref(),
            Self::ImplementationGuide(r) => r.meta.as_ref(),
            Self::Ingredient(r) => r.meta.as_ref(),
            Self::InsurancePlan(r) => r.meta.as_ref(),
            Self::InventoryItem(r) => r.meta.as_ref(),
            Self::InventoryReport(r) => r.meta.as_ref(),
            Self::Invoice(r) => r.meta.as_ref(),
            Self::Library(r) => r.meta.as_ref(),
            Self::Linkage(r) => r.meta.as_ref(),
            Self::List(r) => r.meta.as_ref(),
            Self::Location(r) => r.meta.as_ref(),
            Self::ManufacturedItemDefinition(r) => r.meta.as_ref(),
            Self::Measure(r) => r.meta.as_ref(),
            Self::MeasureReport(r) => r.meta.as_ref(),
            Self::Medication(r) => r.meta.as_ref(),
            Self::MedicationAdministration(r) => r.meta.as_ref(),
            Self::MedicationDispense(r) => r.meta.as_ref(),
            Self::MedicationKnowledge(r) => r.meta.as_ref(),
            Self::MedicationRequest(r) => r.meta.as_ref(),
            Self::MedicationStatement(r) => r.meta.as_ref(),
            Self::MedicinalProductDefinition(r) => r.meta.as_ref(),
            Self::MessageDefinition(r) => r.meta.as_ref(),
            Self::MessageHeader(r) => r.meta.as_ref(),
            Self::MolecularSequence(r) => r.meta.as_ref(),
            Self::NamingSystem(r) => r.meta.as_ref(),
            Self::NutritionIntake(r) => r.meta.as_ref(),
            Self::NutritionOrder(r) => r.meta.as_ref(),
            Self::NutritionProduct(r) => r.meta.as_ref(),
            Self::Observation(r) => r.meta.as_ref(),
            Self::ObservationDefinition(r) => r.meta.as_ref(),
            Self::OperationDefinition(r) => r.meta.as_ref(),
            Self::OperationOutcome(r) => r.meta.as_ref(),
            Self::Organization(r) => r.meta.as_ref(),
            Self::OrganizationAffiliation(r) => r.meta.as_ref(),
            Self::PackagedProductDefinition(r) => r.meta.as_ref(),
            Self::Parameters(r) => r.meta.as_ref(),
            Self::Patient(r) => r.meta.as_ref(),
            Self::PaymentNotice(r) => r.meta.as_ref(),
            Self::PaymentReconciliation(r) => r.meta.as_ref(),
            Self::Permission(r) => r.meta.as_ref(),
            Self::Person(r) => r.meta.as_ref(),
            Self::PlanDefinition(r) => r.meta.as_ref(),
            Self::Practitioner(r) => r.meta.as_ref(),
            Self::PractitionerRole(r) => r.meta.as_ref(),
            Self::Procedure(r) => r.meta.as_ref(),
            Self::Provenance(r) => r.meta.as_ref(),
            Self::Questionnaire(r) => r.meta.as_ref(),
            Self::QuestionnaireResponse(r) => r.meta.as_ref(),
            Self::RegulatedAuthorization(r) => r.meta.as_ref(),
            Self::RelatedPerson(r) => r.meta.as_ref(),
            Self::RequestOrchestration(r) => r.meta.as_ref(),
            Self::Requirements(r) => r.meta.as_ref(),
            Self::ResearchStudy(r) => r.meta.as_ref(),
            Self::ResearchSubject(r) => r.meta.as_ref(),
            Self::RiskAssessment(r) => r.meta.as_ref(),
            Self::Schedule(r) => r.meta.as_ref(),
            Self::SearchParameter(r) => r.meta.as_ref(),
            Self::ServiceRequest(r) => r.meta.as_ref(),
            Self::Slot(r) => r.meta.as_ref(),
            Self::Specimen(r) => r.meta.as_ref(),
            Self::SpecimenDefinition(r) => r.meta.as_ref(),
            Self::StructureDefinition(r) => r.meta.as_ref(),
            Self::StructureMap(r) => r.meta.as_ref(),
            Self::Subscription(r) => r.meta.as_ref(),
            Self::SubscriptionStatus(r) => r.meta.as_ref(),
            Self::SubscriptionTopic(r) => r.meta.as_ref(),
            Self::Substance(r) => r.meta.as_ref(),
            Self::SubstanceDefinition(r) => r.meta.as_ref(),
            Self::SubstanceNucleicAcid(r) => r.meta.as_ref(),
            Self::SubstancePolymer(r) => r.meta.as_ref(),
            Self::SubstanceProtein(r) => r.meta.as_ref(),
            Self::SubstanceReferenceInformation(r) => r.meta.as_ref(),
            Self::SubstanceSourceMaterial(r) => r.meta.as_ref(),
            Self::SupplyDelivery(r) => r.meta.as_ref(),
            Self::SupplyRequest(r) => r.meta.as_ref(),
            Self::Task(r) => r.meta.as_ref(),
            Self::TerminologyCapabilities(r) => r.meta.as_ref(),
            Self::TestPlan(r) => r.meta.as_ref(),
            Self::TestReport(r) => r.meta.as_ref(),
            Self::TestScript(r) => r.meta.as_ref(),
            Self::Transport(r) => r.meta.as_ref(),
            Self::ValueSet(r) => r.meta.as_ref(),
            Self::VerificationResult(r) => r.meta.as_ref(),
            Self::VisionPrescription(r) => r.meta.as_ref(),
        }
    }
}
