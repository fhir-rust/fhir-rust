//! FHIR R6 resources.
//!
//! This module contains the FHIR R6 resource types (Patient, Observation,
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
pub mod clinical_assessment;
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
pub mod device_alert;
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
pub mod insurance_product;
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
pub mod molecular_definition;
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
pub mod personal_relationship;
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
pub use clinical_assessment::ClinicalAssessment;
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
pub use device_alert::DeviceAlert;
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
pub use insurance_product::InsuranceProduct;
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
pub use molecular_definition::MolecularDefinition;
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
pub use personal_relationship::PersonalRelationship;
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

/// Any FHIR R6 resource, tagged by its `resourceType`.
///
/// Used wherever a resource of any type may appear — for example a
/// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes
/// the `resourceType` discriminator automatically.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::Resource;
///
/// let json = ::serde_json::json!({"resourceType": "Patient"});
/// let resource: Resource = ::serde_json::from_value(json).unwrap();
/// assert!(matches!(resource, Resource::Patient(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(tag = "resourceType")]
#[fhir_version("r6")]
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
    ClinicalAssessment(Box<clinical_assessment::ClinicalAssessment>),
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
    DeviceAlert(Box<device_alert::DeviceAlert>),
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
    InsuranceProduct(Box<insurance_product::InsuranceProduct>),
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
    MolecularDefinition(Box<molecular_definition::MolecularDefinition>),
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
    PersonalRelationship(Box<personal_relationship::PersonalRelationship>),
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
            Self::ClinicalAssessment(r) => !r.contained.is_empty(),
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
            Self::DeviceAlert(r) => !r.contained.is_empty(),
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
            Self::InsuranceProduct(r) => !r.contained.is_empty(),
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
            Self::MolecularDefinition(r) => !r.contained.is_empty(),
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
            Self::PersonalRelationship(r) => !r.contained.is_empty(),
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
    pub fn meta(&self) -> Option<&crate::r6::types::Meta> {
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
            Self::ClinicalAssessment(r) => r.meta.as_ref(),
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
            Self::DeviceAlert(r) => r.meta.as_ref(),
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
            Self::InsuranceProduct(r) => r.meta.as_ref(),
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
            Self::MolecularDefinition(r) => r.meta.as_ref(),
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
            Self::PersonalRelationship(r) => r.meta.as_ref(),
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

// The typed-reference target markers (T11): `types::Reference<Patient>`
// points at this module's `Patient`. See `types::reference::ResourceType`.
impl crate::r6::types::reference::ResourceType for account::Account {
    fn resource_type_name() -> Option<&'static str> {
        Some("Account")
    }
}
impl crate::r6::types::reference::ResourceType for activity_definition::ActivityDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ActivityDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for actor_definition::ActorDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ActorDefinition")
    }
}
impl crate::r6::types::reference::ResourceType
    for administrable_product_definition::AdministrableProductDefinition
{
    fn resource_type_name() -> Option<&'static str> {
        Some("AdministrableProductDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for adverse_event::AdverseEvent {
    fn resource_type_name() -> Option<&'static str> {
        Some("AdverseEvent")
    }
}
impl crate::r6::types::reference::ResourceType for allergy_intolerance::AllergyIntolerance {
    fn resource_type_name() -> Option<&'static str> {
        Some("AllergyIntolerance")
    }
}
impl crate::r6::types::reference::ResourceType for appointment::Appointment {
    fn resource_type_name() -> Option<&'static str> {
        Some("Appointment")
    }
}
impl crate::r6::types::reference::ResourceType for appointment_response::AppointmentResponse {
    fn resource_type_name() -> Option<&'static str> {
        Some("AppointmentResponse")
    }
}
impl crate::r6::types::reference::ResourceType for artifact_assessment::ArtifactAssessment {
    fn resource_type_name() -> Option<&'static str> {
        Some("ArtifactAssessment")
    }
}
impl crate::r6::types::reference::ResourceType for audit_event::AuditEvent {
    fn resource_type_name() -> Option<&'static str> {
        Some("AuditEvent")
    }
}
impl crate::r6::types::reference::ResourceType for basic::Basic {
    fn resource_type_name() -> Option<&'static str> {
        Some("Basic")
    }
}
impl crate::r6::types::reference::ResourceType for binary::Binary {
    fn resource_type_name() -> Option<&'static str> {
        Some("Binary")
    }
}
impl crate::r6::types::reference::ResourceType
    for biologically_derived_product::BiologicallyDerivedProduct
{
    fn resource_type_name() -> Option<&'static str> {
        Some("BiologicallyDerivedProduct")
    }
}
impl crate::r6::types::reference::ResourceType
    for biologically_derived_product_dispense::BiologicallyDerivedProductDispense
{
    fn resource_type_name() -> Option<&'static str> {
        Some("BiologicallyDerivedProductDispense")
    }
}
impl crate::r6::types::reference::ResourceType for body_structure::BodyStructure {
    fn resource_type_name() -> Option<&'static str> {
        Some("BodyStructure")
    }
}
impl crate::r6::types::reference::ResourceType for bundle::Bundle {
    fn resource_type_name() -> Option<&'static str> {
        Some("Bundle")
    }
}
impl crate::r6::types::reference::ResourceType for capability_statement::CapabilityStatement {
    fn resource_type_name() -> Option<&'static str> {
        Some("CapabilityStatement")
    }
}
impl crate::r6::types::reference::ResourceType for care_plan::CarePlan {
    fn resource_type_name() -> Option<&'static str> {
        Some("CarePlan")
    }
}
impl crate::r6::types::reference::ResourceType for care_team::CareTeam {
    fn resource_type_name() -> Option<&'static str> {
        Some("CareTeam")
    }
}
impl crate::r6::types::reference::ResourceType for charge_item::ChargeItem {
    fn resource_type_name() -> Option<&'static str> {
        Some("ChargeItem")
    }
}
impl crate::r6::types::reference::ResourceType for charge_item_definition::ChargeItemDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ChargeItemDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for citation::Citation {
    fn resource_type_name() -> Option<&'static str> {
        Some("Citation")
    }
}
impl crate::r6::types::reference::ResourceType for claim::Claim {
    fn resource_type_name() -> Option<&'static str> {
        Some("Claim")
    }
}
impl crate::r6::types::reference::ResourceType for claim_response::ClaimResponse {
    fn resource_type_name() -> Option<&'static str> {
        Some("ClaimResponse")
    }
}
impl crate::r6::types::reference::ResourceType for clinical_assessment::ClinicalAssessment {
    fn resource_type_name() -> Option<&'static str> {
        Some("ClinicalAssessment")
    }
}
impl crate::r6::types::reference::ResourceType for clinical_use_definition::ClinicalUseDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ClinicalUseDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for code_system::CodeSystem {
    fn resource_type_name() -> Option<&'static str> {
        Some("CodeSystem")
    }
}
impl crate::r6::types::reference::ResourceType for communication::Communication {
    fn resource_type_name() -> Option<&'static str> {
        Some("Communication")
    }
}
impl crate::r6::types::reference::ResourceType for communication_request::CommunicationRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("CommunicationRequest")
    }
}
impl crate::r6::types::reference::ResourceType for compartment_definition::CompartmentDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("CompartmentDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for composition::Composition {
    fn resource_type_name() -> Option<&'static str> {
        Some("Composition")
    }
}
impl crate::r6::types::reference::ResourceType for concept_map::ConceptMap {
    fn resource_type_name() -> Option<&'static str> {
        Some("ConceptMap")
    }
}
impl crate::r6::types::reference::ResourceType for condition::Condition {
    fn resource_type_name() -> Option<&'static str> {
        Some("Condition")
    }
}
impl crate::r6::types::reference::ResourceType for condition_definition::ConditionDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ConditionDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for consent::Consent {
    fn resource_type_name() -> Option<&'static str> {
        Some("Consent")
    }
}
impl crate::r6::types::reference::ResourceType for contract::Contract {
    fn resource_type_name() -> Option<&'static str> {
        Some("Contract")
    }
}
impl crate::r6::types::reference::ResourceType for coverage::Coverage {
    fn resource_type_name() -> Option<&'static str> {
        Some("Coverage")
    }
}
impl crate::r6::types::reference::ResourceType
    for coverage_eligibility_request::CoverageEligibilityRequest
{
    fn resource_type_name() -> Option<&'static str> {
        Some("CoverageEligibilityRequest")
    }
}
impl crate::r6::types::reference::ResourceType
    for coverage_eligibility_response::CoverageEligibilityResponse
{
    fn resource_type_name() -> Option<&'static str> {
        Some("CoverageEligibilityResponse")
    }
}
impl crate::r6::types::reference::ResourceType for detected_issue::DetectedIssue {
    fn resource_type_name() -> Option<&'static str> {
        Some("DetectedIssue")
    }
}
impl crate::r6::types::reference::ResourceType for device::Device {
    fn resource_type_name() -> Option<&'static str> {
        Some("Device")
    }
}
impl crate::r6::types::reference::ResourceType for device_alert::DeviceAlert {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceAlert")
    }
}
impl crate::r6::types::reference::ResourceType for device_association::DeviceAssociation {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceAssociation")
    }
}
impl crate::r6::types::reference::ResourceType for device_definition::DeviceDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for device_dispense::DeviceDispense {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceDispense")
    }
}
impl crate::r6::types::reference::ResourceType for device_metric::DeviceMetric {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceMetric")
    }
}
impl crate::r6::types::reference::ResourceType for device_request::DeviceRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceRequest")
    }
}
impl crate::r6::types::reference::ResourceType for device_usage::DeviceUsage {
    fn resource_type_name() -> Option<&'static str> {
        Some("DeviceUsage")
    }
}
impl crate::r6::types::reference::ResourceType for diagnostic_report::DiagnosticReport {
    fn resource_type_name() -> Option<&'static str> {
        Some("DiagnosticReport")
    }
}
impl crate::r6::types::reference::ResourceType for document_reference::DocumentReference {
    fn resource_type_name() -> Option<&'static str> {
        Some("DocumentReference")
    }
}
impl crate::r6::types::reference::ResourceType for encounter::Encounter {
    fn resource_type_name() -> Option<&'static str> {
        Some("Encounter")
    }
}
impl crate::r6::types::reference::ResourceType for encounter_history::EncounterHistory {
    fn resource_type_name() -> Option<&'static str> {
        Some("EncounterHistory")
    }
}
impl crate::r6::types::reference::ResourceType for endpoint::Endpoint {
    fn resource_type_name() -> Option<&'static str> {
        Some("Endpoint")
    }
}
impl crate::r6::types::reference::ResourceType for enrollment_request::EnrollmentRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("EnrollmentRequest")
    }
}
impl crate::r6::types::reference::ResourceType for enrollment_response::EnrollmentResponse {
    fn resource_type_name() -> Option<&'static str> {
        Some("EnrollmentResponse")
    }
}
impl crate::r6::types::reference::ResourceType for episode_of_care::EpisodeOfCare {
    fn resource_type_name() -> Option<&'static str> {
        Some("EpisodeOfCare")
    }
}
impl crate::r6::types::reference::ResourceType for event_definition::EventDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("EventDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for evidence::Evidence {
    fn resource_type_name() -> Option<&'static str> {
        Some("Evidence")
    }
}
impl crate::r6::types::reference::ResourceType for evidence_variable::EvidenceVariable {
    fn resource_type_name() -> Option<&'static str> {
        Some("EvidenceVariable")
    }
}
impl crate::r6::types::reference::ResourceType for example_scenario::ExampleScenario {
    fn resource_type_name() -> Option<&'static str> {
        Some("ExampleScenario")
    }
}
impl crate::r6::types::reference::ResourceType for explanation_of_benefit::ExplanationOfBenefit {
    fn resource_type_name() -> Option<&'static str> {
        Some("ExplanationOfBenefit")
    }
}
impl crate::r6::types::reference::ResourceType for family_member_history::FamilyMemberHistory {
    fn resource_type_name() -> Option<&'static str> {
        Some("FamilyMemberHistory")
    }
}
impl crate::r6::types::reference::ResourceType for flag::Flag {
    fn resource_type_name() -> Option<&'static str> {
        Some("Flag")
    }
}
impl crate::r6::types::reference::ResourceType for formulary_item::FormularyItem {
    fn resource_type_name() -> Option<&'static str> {
        Some("FormularyItem")
    }
}
impl crate::r6::types::reference::ResourceType for genomic_study::GenomicStudy {
    fn resource_type_name() -> Option<&'static str> {
        Some("GenomicStudy")
    }
}
impl crate::r6::types::reference::ResourceType for goal::Goal {
    fn resource_type_name() -> Option<&'static str> {
        Some("Goal")
    }
}
impl crate::r6::types::reference::ResourceType for graph_definition::GraphDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("GraphDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for group::Group {
    fn resource_type_name() -> Option<&'static str> {
        Some("Group")
    }
}
impl crate::r6::types::reference::ResourceType for guidance_response::GuidanceResponse {
    fn resource_type_name() -> Option<&'static str> {
        Some("GuidanceResponse")
    }
}
impl crate::r6::types::reference::ResourceType for healthcare_service::HealthcareService {
    fn resource_type_name() -> Option<&'static str> {
        Some("HealthcareService")
    }
}
impl crate::r6::types::reference::ResourceType for imaging_selection::ImagingSelection {
    fn resource_type_name() -> Option<&'static str> {
        Some("ImagingSelection")
    }
}
impl crate::r6::types::reference::ResourceType for imaging_study::ImagingStudy {
    fn resource_type_name() -> Option<&'static str> {
        Some("ImagingStudy")
    }
}
impl crate::r6::types::reference::ResourceType for immunization::Immunization {
    fn resource_type_name() -> Option<&'static str> {
        Some("Immunization")
    }
}
impl crate::r6::types::reference::ResourceType for immunization_evaluation::ImmunizationEvaluation {
    fn resource_type_name() -> Option<&'static str> {
        Some("ImmunizationEvaluation")
    }
}
impl crate::r6::types::reference::ResourceType
    for immunization_recommendation::ImmunizationRecommendation
{
    fn resource_type_name() -> Option<&'static str> {
        Some("ImmunizationRecommendation")
    }
}
impl crate::r6::types::reference::ResourceType for implementation_guide::ImplementationGuide {
    fn resource_type_name() -> Option<&'static str> {
        Some("ImplementationGuide")
    }
}
impl crate::r6::types::reference::ResourceType for ingredient::Ingredient {
    fn resource_type_name() -> Option<&'static str> {
        Some("Ingredient")
    }
}
impl crate::r6::types::reference::ResourceType for insurance_plan::InsurancePlan {
    fn resource_type_name() -> Option<&'static str> {
        Some("InsurancePlan")
    }
}
impl crate::r6::types::reference::ResourceType for insurance_product::InsuranceProduct {
    fn resource_type_name() -> Option<&'static str> {
        Some("InsuranceProduct")
    }
}
impl crate::r6::types::reference::ResourceType for inventory_item::InventoryItem {
    fn resource_type_name() -> Option<&'static str> {
        Some("InventoryItem")
    }
}
impl crate::r6::types::reference::ResourceType for inventory_report::InventoryReport {
    fn resource_type_name() -> Option<&'static str> {
        Some("InventoryReport")
    }
}
impl crate::r6::types::reference::ResourceType for invoice::Invoice {
    fn resource_type_name() -> Option<&'static str> {
        Some("Invoice")
    }
}
impl crate::r6::types::reference::ResourceType for library::Library {
    fn resource_type_name() -> Option<&'static str> {
        Some("Library")
    }
}
impl crate::r6::types::reference::ResourceType for linkage::Linkage {
    fn resource_type_name() -> Option<&'static str> {
        Some("Linkage")
    }
}
impl crate::r6::types::reference::ResourceType for list::List {
    fn resource_type_name() -> Option<&'static str> {
        Some("List")
    }
}
impl crate::r6::types::reference::ResourceType for location::Location {
    fn resource_type_name() -> Option<&'static str> {
        Some("Location")
    }
}
impl crate::r6::types::reference::ResourceType
    for manufactured_item_definition::ManufacturedItemDefinition
{
    fn resource_type_name() -> Option<&'static str> {
        Some("ManufacturedItemDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for measure::Measure {
    fn resource_type_name() -> Option<&'static str> {
        Some("Measure")
    }
}
impl crate::r6::types::reference::ResourceType for measure_report::MeasureReport {
    fn resource_type_name() -> Option<&'static str> {
        Some("MeasureReport")
    }
}
impl crate::r6::types::reference::ResourceType for medication::Medication {
    fn resource_type_name() -> Option<&'static str> {
        Some("Medication")
    }
}
impl crate::r6::types::reference::ResourceType
    for medication_administration::MedicationAdministration
{
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicationAdministration")
    }
}
impl crate::r6::types::reference::ResourceType for medication_dispense::MedicationDispense {
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicationDispense")
    }
}
impl crate::r6::types::reference::ResourceType for medication_knowledge::MedicationKnowledge {
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicationKnowledge")
    }
}
impl crate::r6::types::reference::ResourceType for medication_request::MedicationRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicationRequest")
    }
}
impl crate::r6::types::reference::ResourceType for medication_statement::MedicationStatement {
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicationStatement")
    }
}
impl crate::r6::types::reference::ResourceType
    for medicinal_product_definition::MedicinalProductDefinition
{
    fn resource_type_name() -> Option<&'static str> {
        Some("MedicinalProductDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for message_definition::MessageDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("MessageDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for message_header::MessageHeader {
    fn resource_type_name() -> Option<&'static str> {
        Some("MessageHeader")
    }
}
impl crate::r6::types::reference::ResourceType for molecular_definition::MolecularDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("MolecularDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for molecular_sequence::MolecularSequence {
    fn resource_type_name() -> Option<&'static str> {
        Some("MolecularSequence")
    }
}
impl crate::r6::types::reference::ResourceType for naming_system::NamingSystem {
    fn resource_type_name() -> Option<&'static str> {
        Some("NamingSystem")
    }
}
impl crate::r6::types::reference::ResourceType for nutrition_intake::NutritionIntake {
    fn resource_type_name() -> Option<&'static str> {
        Some("NutritionIntake")
    }
}
impl crate::r6::types::reference::ResourceType for nutrition_order::NutritionOrder {
    fn resource_type_name() -> Option<&'static str> {
        Some("NutritionOrder")
    }
}
impl crate::r6::types::reference::ResourceType for nutrition_product::NutritionProduct {
    fn resource_type_name() -> Option<&'static str> {
        Some("NutritionProduct")
    }
}
impl crate::r6::types::reference::ResourceType for observation::Observation {
    fn resource_type_name() -> Option<&'static str> {
        Some("Observation")
    }
}
impl crate::r6::types::reference::ResourceType for observation_definition::ObservationDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("ObservationDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for operation_definition::OperationDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("OperationDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for operation_outcome::OperationOutcome {
    fn resource_type_name() -> Option<&'static str> {
        Some("OperationOutcome")
    }
}
impl crate::r6::types::reference::ResourceType for organization::Organization {
    fn resource_type_name() -> Option<&'static str> {
        Some("Organization")
    }
}
impl crate::r6::types::reference::ResourceType
    for organization_affiliation::OrganizationAffiliation
{
    fn resource_type_name() -> Option<&'static str> {
        Some("OrganizationAffiliation")
    }
}
impl crate::r6::types::reference::ResourceType
    for packaged_product_definition::PackagedProductDefinition
{
    fn resource_type_name() -> Option<&'static str> {
        Some("PackagedProductDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for parameters::Parameters {
    fn resource_type_name() -> Option<&'static str> {
        Some("Parameters")
    }
}
impl crate::r6::types::reference::ResourceType for patient::Patient {
    fn resource_type_name() -> Option<&'static str> {
        Some("Patient")
    }
}
impl crate::r6::types::reference::ResourceType for payment_notice::PaymentNotice {
    fn resource_type_name() -> Option<&'static str> {
        Some("PaymentNotice")
    }
}
impl crate::r6::types::reference::ResourceType for payment_reconciliation::PaymentReconciliation {
    fn resource_type_name() -> Option<&'static str> {
        Some("PaymentReconciliation")
    }
}
impl crate::r6::types::reference::ResourceType for permission::Permission {
    fn resource_type_name() -> Option<&'static str> {
        Some("Permission")
    }
}
impl crate::r6::types::reference::ResourceType for person::Person {
    fn resource_type_name() -> Option<&'static str> {
        Some("Person")
    }
}
impl crate::r6::types::reference::ResourceType for personal_relationship::PersonalRelationship {
    fn resource_type_name() -> Option<&'static str> {
        Some("PersonalRelationship")
    }
}
impl crate::r6::types::reference::ResourceType for plan_definition::PlanDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("PlanDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for practitioner::Practitioner {
    fn resource_type_name() -> Option<&'static str> {
        Some("Practitioner")
    }
}
impl crate::r6::types::reference::ResourceType for practitioner_role::PractitionerRole {
    fn resource_type_name() -> Option<&'static str> {
        Some("PractitionerRole")
    }
}
impl crate::r6::types::reference::ResourceType for procedure::Procedure {
    fn resource_type_name() -> Option<&'static str> {
        Some("Procedure")
    }
}
impl crate::r6::types::reference::ResourceType for provenance::Provenance {
    fn resource_type_name() -> Option<&'static str> {
        Some("Provenance")
    }
}
impl crate::r6::types::reference::ResourceType for questionnaire::Questionnaire {
    fn resource_type_name() -> Option<&'static str> {
        Some("Questionnaire")
    }
}
impl crate::r6::types::reference::ResourceType for questionnaire_response::QuestionnaireResponse {
    fn resource_type_name() -> Option<&'static str> {
        Some("QuestionnaireResponse")
    }
}
impl crate::r6::types::reference::ResourceType for regulated_authorization::RegulatedAuthorization {
    fn resource_type_name() -> Option<&'static str> {
        Some("RegulatedAuthorization")
    }
}
impl crate::r6::types::reference::ResourceType for related_person::RelatedPerson {
    fn resource_type_name() -> Option<&'static str> {
        Some("RelatedPerson")
    }
}
impl crate::r6::types::reference::ResourceType for request_orchestration::RequestOrchestration {
    fn resource_type_name() -> Option<&'static str> {
        Some("RequestOrchestration")
    }
}
impl crate::r6::types::reference::ResourceType for requirements::Requirements {
    fn resource_type_name() -> Option<&'static str> {
        Some("Requirements")
    }
}
impl crate::r6::types::reference::ResourceType for research_study::ResearchStudy {
    fn resource_type_name() -> Option<&'static str> {
        Some("ResearchStudy")
    }
}
impl crate::r6::types::reference::ResourceType for research_subject::ResearchSubject {
    fn resource_type_name() -> Option<&'static str> {
        Some("ResearchSubject")
    }
}
impl crate::r6::types::reference::ResourceType for risk_assessment::RiskAssessment {
    fn resource_type_name() -> Option<&'static str> {
        Some("RiskAssessment")
    }
}
impl crate::r6::types::reference::ResourceType for schedule::Schedule {
    fn resource_type_name() -> Option<&'static str> {
        Some("Schedule")
    }
}
impl crate::r6::types::reference::ResourceType for search_parameter::SearchParameter {
    fn resource_type_name() -> Option<&'static str> {
        Some("SearchParameter")
    }
}
impl crate::r6::types::reference::ResourceType for service_request::ServiceRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("ServiceRequest")
    }
}
impl crate::r6::types::reference::ResourceType for slot::Slot {
    fn resource_type_name() -> Option<&'static str> {
        Some("Slot")
    }
}
impl crate::r6::types::reference::ResourceType for specimen::Specimen {
    fn resource_type_name() -> Option<&'static str> {
        Some("Specimen")
    }
}
impl crate::r6::types::reference::ResourceType for specimen_definition::SpecimenDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("SpecimenDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for structure_definition::StructureDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("StructureDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for structure_map::StructureMap {
    fn resource_type_name() -> Option<&'static str> {
        Some("StructureMap")
    }
}
impl crate::r6::types::reference::ResourceType for subscription::Subscription {
    fn resource_type_name() -> Option<&'static str> {
        Some("Subscription")
    }
}
impl crate::r6::types::reference::ResourceType for subscription_status::SubscriptionStatus {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubscriptionStatus")
    }
}
impl crate::r6::types::reference::ResourceType for subscription_topic::SubscriptionTopic {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubscriptionTopic")
    }
}
impl crate::r6::types::reference::ResourceType for substance::Substance {
    fn resource_type_name() -> Option<&'static str> {
        Some("Substance")
    }
}
impl crate::r6::types::reference::ResourceType for substance_definition::SubstanceDefinition {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstanceDefinition")
    }
}
impl crate::r6::types::reference::ResourceType for substance_nucleic_acid::SubstanceNucleicAcid {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstanceNucleicAcid")
    }
}
impl crate::r6::types::reference::ResourceType for substance_polymer::SubstancePolymer {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstancePolymer")
    }
}
impl crate::r6::types::reference::ResourceType for substance_protein::SubstanceProtein {
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstanceProtein")
    }
}
impl crate::r6::types::reference::ResourceType
    for substance_reference_information::SubstanceReferenceInformation
{
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstanceReferenceInformation")
    }
}
impl crate::r6::types::reference::ResourceType
    for substance_source_material::SubstanceSourceMaterial
{
    fn resource_type_name() -> Option<&'static str> {
        Some("SubstanceSourceMaterial")
    }
}
impl crate::r6::types::reference::ResourceType for supply_delivery::SupplyDelivery {
    fn resource_type_name() -> Option<&'static str> {
        Some("SupplyDelivery")
    }
}
impl crate::r6::types::reference::ResourceType for supply_request::SupplyRequest {
    fn resource_type_name() -> Option<&'static str> {
        Some("SupplyRequest")
    }
}
impl crate::r6::types::reference::ResourceType for task::Task {
    fn resource_type_name() -> Option<&'static str> {
        Some("Task")
    }
}
impl crate::r6::types::reference::ResourceType
    for terminology_capabilities::TerminologyCapabilities
{
    fn resource_type_name() -> Option<&'static str> {
        Some("TerminologyCapabilities")
    }
}
impl crate::r6::types::reference::ResourceType for test_plan::TestPlan {
    fn resource_type_name() -> Option<&'static str> {
        Some("TestPlan")
    }
}
impl crate::r6::types::reference::ResourceType for test_report::TestReport {
    fn resource_type_name() -> Option<&'static str> {
        Some("TestReport")
    }
}
impl crate::r6::types::reference::ResourceType for test_script::TestScript {
    fn resource_type_name() -> Option<&'static str> {
        Some("TestScript")
    }
}
impl crate::r6::types::reference::ResourceType for transport::Transport {
    fn resource_type_name() -> Option<&'static str> {
        Some("Transport")
    }
}
impl crate::r6::types::reference::ResourceType for value_set::ValueSet {
    fn resource_type_name() -> Option<&'static str> {
        Some("ValueSet")
    }
}
impl crate::r6::types::reference::ResourceType for verification_result::VerificationResult {
    fn resource_type_name() -> Option<&'static str> {
        Some("VerificationResult")
    }
}
impl crate::r6::types::reference::ResourceType for vision_prescription::VisionPrescription {
    fn resource_type_name() -> Option<&'static str> {
        Some("VisionPrescription")
    }
}
