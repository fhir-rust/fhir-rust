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
