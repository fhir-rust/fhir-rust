//! FHIR R3 resources.
//!
//! This module contains the FHIR R3 resource types (Patient, Observation,
//! Encounter, and so on). Each resource is a Rust struct that serializes to
//! and from the canonical FHIR JSON representation via `serde`.

use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

pub mod account;
pub mod activity_definition;
pub mod adverse_event;
pub mod allergy_intolerance;
pub mod appointment;
pub mod appointment_response;
pub mod audit_event;
pub mod basic;
pub mod binary;
pub mod body_site;
pub mod bundle;
pub mod capability_statement;
pub mod care_plan;
pub mod care_team;
pub mod charge_item;
pub mod claim;
pub mod claim_response;
pub mod clinical_impression;
pub mod code_system;
pub mod communication;
pub mod communication_request;
pub mod compartment_definition;
pub mod composition;
pub mod concept_map;
pub mod condition;
pub mod consent;
pub mod contract;
pub mod coverage;
pub mod data_element;
pub mod detected_issue;
pub mod device;
pub mod device_component;
pub mod device_metric;
pub mod device_request;
pub mod device_use_statement;
pub mod diagnostic_report;
pub mod document_manifest;
pub mod document_reference;
pub mod eligibility_request;
pub mod eligibility_response;
pub mod encounter;
pub mod endpoint;
pub mod enrollment_request;
pub mod enrollment_response;
pub mod episode_of_care;
pub mod expansion_profile;
pub mod explanation_of_benefit;
pub mod family_member_history;
pub mod flag;
pub mod goal;
pub mod graph_definition;
pub mod group;
pub mod guidance_response;
pub mod healthcare_service;
pub mod imaging_manifest;
pub mod imaging_study;
pub mod immunization;
pub mod immunization_recommendation;
pub mod implementation_guide;
pub mod library;
pub mod linkage;
pub mod list;
pub mod location;
pub mod measure;
pub mod measure_report;
pub mod media;
pub mod medication;
pub mod medication_administration;
pub mod medication_dispense;
pub mod medication_request;
pub mod medication_statement;
pub mod message_definition;
pub mod message_header;
pub mod naming_system;
pub mod nutrition_order;
pub mod observation;
pub mod operation_definition;
pub mod operation_outcome;
pub mod organization;
pub mod parameters;
pub mod patient;
pub mod payment_notice;
pub mod payment_reconciliation;
pub mod person;
pub mod plan_definition;
pub mod practitioner;
pub mod practitioner_role;
pub mod procedure;
pub mod procedure_request;
pub mod process_request;
pub mod process_response;
pub mod provenance;
pub mod questionnaire;
pub mod questionnaire_response;
pub mod referral_request;
pub mod related_person;
pub mod request_group;
pub mod research_study;
pub mod research_subject;
pub mod risk_assessment;
pub mod schedule;
pub mod search_parameter;
pub mod sequence;
pub mod service_definition;
pub mod slot;
pub mod specimen;
pub mod structure_definition;
pub mod structure_map;
pub mod subscription;
pub mod substance;
pub mod supply_delivery;
pub mod supply_request;
pub mod task;
pub mod test_report;
pub mod test_script;
pub mod value_set;
pub mod vision_prescription;

pub use account::Account;
pub use activity_definition::ActivityDefinition;
pub use adverse_event::AdverseEvent;
pub use allergy_intolerance::AllergyIntolerance;
pub use appointment::Appointment;
pub use appointment_response::AppointmentResponse;
pub use audit_event::AuditEvent;
pub use basic::Basic;
pub use binary::Binary;
pub use body_site::BodySite;
pub use bundle::Bundle;
pub use capability_statement::CapabilityStatement;
pub use care_plan::CarePlan;
pub use care_team::CareTeam;
pub use charge_item::ChargeItem;
pub use claim::Claim;
pub use claim_response::ClaimResponse;
pub use clinical_impression::ClinicalImpression;
pub use code_system::CodeSystem;
pub use communication::Communication;
pub use communication_request::CommunicationRequest;
pub use compartment_definition::CompartmentDefinition;
pub use composition::Composition;
pub use concept_map::ConceptMap;
pub use condition::Condition;
pub use consent::Consent;
pub use contract::Contract;
pub use coverage::Coverage;
pub use data_element::DataElement;
pub use detected_issue::DetectedIssue;
pub use device::Device;
pub use device_component::DeviceComponent;
pub use device_metric::DeviceMetric;
pub use device_request::DeviceRequest;
pub use device_use_statement::DeviceUseStatement;
pub use diagnostic_report::DiagnosticReport;
pub use document_manifest::DocumentManifest;
pub use document_reference::DocumentReference;
pub use eligibility_request::EligibilityRequest;
pub use eligibility_response::EligibilityResponse;
pub use encounter::Encounter;
pub use endpoint::Endpoint;
pub use enrollment_request::EnrollmentRequest;
pub use enrollment_response::EnrollmentResponse;
pub use episode_of_care::EpisodeOfCare;
pub use expansion_profile::ExpansionProfile;
pub use explanation_of_benefit::ExplanationOfBenefit;
pub use family_member_history::FamilyMemberHistory;
pub use flag::Flag;
pub use goal::Goal;
pub use graph_definition::GraphDefinition;
pub use group::Group;
pub use guidance_response::GuidanceResponse;
pub use healthcare_service::HealthcareService;
pub use imaging_manifest::ImagingManifest;
pub use imaging_study::ImagingStudy;
pub use immunization::Immunization;
pub use immunization_recommendation::ImmunizationRecommendation;
pub use implementation_guide::ImplementationGuide;
pub use library::Library;
pub use linkage::Linkage;
pub use list::List;
pub use location::Location;
pub use measure::Measure;
pub use measure_report::MeasureReport;
pub use media::Media;
pub use medication::Medication;
pub use medication_administration::MedicationAdministration;
pub use medication_dispense::MedicationDispense;
pub use medication_request::MedicationRequest;
pub use medication_statement::MedicationStatement;
pub use message_definition::MessageDefinition;
pub use message_header::MessageHeader;
pub use naming_system::NamingSystem;
pub use nutrition_order::NutritionOrder;
pub use observation::Observation;
pub use operation_definition::OperationDefinition;
pub use operation_outcome::OperationOutcome;
pub use organization::Organization;
pub use parameters::Parameters;
pub use patient::Patient;
pub use payment_notice::PaymentNotice;
pub use payment_reconciliation::PaymentReconciliation;
pub use person::Person;
pub use plan_definition::PlanDefinition;
pub use practitioner::Practitioner;
pub use practitioner_role::PractitionerRole;
pub use procedure::Procedure;
pub use procedure_request::ProcedureRequest;
pub use process_request::ProcessRequest;
pub use process_response::ProcessResponse;
pub use provenance::Provenance;
pub use questionnaire::Questionnaire;
pub use questionnaire_response::QuestionnaireResponse;
pub use referral_request::ReferralRequest;
pub use related_person::RelatedPerson;
pub use request_group::RequestGroup;
pub use research_study::ResearchStudy;
pub use research_subject::ResearchSubject;
pub use risk_assessment::RiskAssessment;
pub use schedule::Schedule;
pub use search_parameter::SearchParameter;
pub use sequence::Sequence;
pub use service_definition::ServiceDefinition;
pub use slot::Slot;
pub use specimen::Specimen;
pub use structure_definition::StructureDefinition;
pub use structure_map::StructureMap;
pub use subscription::Subscription;
pub use substance::Substance;
pub use supply_delivery::SupplyDelivery;
pub use supply_request::SupplyRequest;
pub use task::Task;
pub use test_report::TestReport;
pub use test_script::TestScript;
pub use value_set::ValueSet;
pub use vision_prescription::VisionPrescription;

/// Any FHIR R3 resource, tagged by its `resourceType`.
///
/// Used wherever a resource of any type may appear — for example a
/// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes
/// the `resourceType` discriminator automatically.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::Resource;
///
/// let json = ::serde_json::json!({"resourceType": "Patient"});
/// let resource: Resource = ::serde_json::from_value(json).unwrap();
/// assert!(matches!(resource, Resource::Patient(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(tag = "resourceType")]
#[fhir_version("r3")]
pub enum Resource {
    Account(Box<account::Account>),
    ActivityDefinition(Box<activity_definition::ActivityDefinition>),
    AdverseEvent(Box<adverse_event::AdverseEvent>),
    AllergyIntolerance(Box<allergy_intolerance::AllergyIntolerance>),
    Appointment(Box<appointment::Appointment>),
    AppointmentResponse(Box<appointment_response::AppointmentResponse>),
    AuditEvent(Box<audit_event::AuditEvent>),
    Basic(Box<basic::Basic>),
    Binary(Box<binary::Binary>),
    BodySite(Box<body_site::BodySite>),
    Bundle(Box<bundle::Bundle>),
    CapabilityStatement(Box<capability_statement::CapabilityStatement>),
    CarePlan(Box<care_plan::CarePlan>),
    CareTeam(Box<care_team::CareTeam>),
    ChargeItem(Box<charge_item::ChargeItem>),
    Claim(Box<claim::Claim>),
    ClaimResponse(Box<claim_response::ClaimResponse>),
    ClinicalImpression(Box<clinical_impression::ClinicalImpression>),
    CodeSystem(Box<code_system::CodeSystem>),
    Communication(Box<communication::Communication>),
    CommunicationRequest(Box<communication_request::CommunicationRequest>),
    CompartmentDefinition(Box<compartment_definition::CompartmentDefinition>),
    Composition(Box<composition::Composition>),
    ConceptMap(Box<concept_map::ConceptMap>),
    Condition(Box<condition::Condition>),
    Consent(Box<consent::Consent>),
    Contract(Box<contract::Contract>),
    Coverage(Box<coverage::Coverage>),
    DataElement(Box<data_element::DataElement>),
    DetectedIssue(Box<detected_issue::DetectedIssue>),
    Device(Box<device::Device>),
    DeviceComponent(Box<device_component::DeviceComponent>),
    DeviceMetric(Box<device_metric::DeviceMetric>),
    DeviceRequest(Box<device_request::DeviceRequest>),
    DeviceUseStatement(Box<device_use_statement::DeviceUseStatement>),
    DiagnosticReport(Box<diagnostic_report::DiagnosticReport>),
    DocumentManifest(Box<document_manifest::DocumentManifest>),
    DocumentReference(Box<document_reference::DocumentReference>),
    EligibilityRequest(Box<eligibility_request::EligibilityRequest>),
    EligibilityResponse(Box<eligibility_response::EligibilityResponse>),
    Encounter(Box<encounter::Encounter>),
    Endpoint(Box<endpoint::Endpoint>),
    EnrollmentRequest(Box<enrollment_request::EnrollmentRequest>),
    EnrollmentResponse(Box<enrollment_response::EnrollmentResponse>),
    EpisodeOfCare(Box<episode_of_care::EpisodeOfCare>),
    ExpansionProfile(Box<expansion_profile::ExpansionProfile>),
    ExplanationOfBenefit(Box<explanation_of_benefit::ExplanationOfBenefit>),
    FamilyMemberHistory(Box<family_member_history::FamilyMemberHistory>),
    Flag(Box<flag::Flag>),
    Goal(Box<goal::Goal>),
    GraphDefinition(Box<graph_definition::GraphDefinition>),
    Group(Box<group::Group>),
    GuidanceResponse(Box<guidance_response::GuidanceResponse>),
    HealthcareService(Box<healthcare_service::HealthcareService>),
    ImagingManifest(Box<imaging_manifest::ImagingManifest>),
    ImagingStudy(Box<imaging_study::ImagingStudy>),
    Immunization(Box<immunization::Immunization>),
    ImmunizationRecommendation(Box<immunization_recommendation::ImmunizationRecommendation>),
    ImplementationGuide(Box<implementation_guide::ImplementationGuide>),
    Library(Box<library::Library>),
    Linkage(Box<linkage::Linkage>),
    List(Box<list::List>),
    Location(Box<location::Location>),
    Measure(Box<measure::Measure>),
    MeasureReport(Box<measure_report::MeasureReport>),
    Media(Box<media::Media>),
    Medication(Box<medication::Medication>),
    MedicationAdministration(Box<medication_administration::MedicationAdministration>),
    MedicationDispense(Box<medication_dispense::MedicationDispense>),
    MedicationRequest(Box<medication_request::MedicationRequest>),
    MedicationStatement(Box<medication_statement::MedicationStatement>),
    MessageDefinition(Box<message_definition::MessageDefinition>),
    MessageHeader(Box<message_header::MessageHeader>),
    NamingSystem(Box<naming_system::NamingSystem>),
    NutritionOrder(Box<nutrition_order::NutritionOrder>),
    Observation(Box<observation::Observation>),
    OperationDefinition(Box<operation_definition::OperationDefinition>),
    OperationOutcome(Box<operation_outcome::OperationOutcome>),
    Organization(Box<organization::Organization>),
    Parameters(Box<parameters::Parameters>),
    Patient(Box<patient::Patient>),
    PaymentNotice(Box<payment_notice::PaymentNotice>),
    PaymentReconciliation(Box<payment_reconciliation::PaymentReconciliation>),
    Person(Box<person::Person>),
    PlanDefinition(Box<plan_definition::PlanDefinition>),
    Practitioner(Box<practitioner::Practitioner>),
    PractitionerRole(Box<practitioner_role::PractitionerRole>),
    Procedure(Box<procedure::Procedure>),
    ProcedureRequest(Box<procedure_request::ProcedureRequest>),
    ProcessRequest(Box<process_request::ProcessRequest>),
    ProcessResponse(Box<process_response::ProcessResponse>),
    Provenance(Box<provenance::Provenance>),
    Questionnaire(Box<questionnaire::Questionnaire>),
    QuestionnaireResponse(Box<questionnaire_response::QuestionnaireResponse>),
    ReferralRequest(Box<referral_request::ReferralRequest>),
    RelatedPerson(Box<related_person::RelatedPerson>),
    RequestGroup(Box<request_group::RequestGroup>),
    ResearchStudy(Box<research_study::ResearchStudy>),
    ResearchSubject(Box<research_subject::ResearchSubject>),
    RiskAssessment(Box<risk_assessment::RiskAssessment>),
    Schedule(Box<schedule::Schedule>),
    SearchParameter(Box<search_parameter::SearchParameter>),
    Sequence(Box<sequence::Sequence>),
    ServiceDefinition(Box<service_definition::ServiceDefinition>),
    Slot(Box<slot::Slot>),
    Specimen(Box<specimen::Specimen>),
    StructureDefinition(Box<structure_definition::StructureDefinition>),
    StructureMap(Box<structure_map::StructureMap>),
    Subscription(Box<subscription::Subscription>),
    Substance(Box<substance::Substance>),
    SupplyDelivery(Box<supply_delivery::SupplyDelivery>),
    SupplyRequest(Box<supply_request::SupplyRequest>),
    Task(Box<task::Task>),
    TestReport(Box<test_report::TestReport>),
    TestScript(Box<test_script::TestScript>),
    ValueSet(Box<value_set::ValueSet>),
    VisionPrescription(Box<vision_prescription::VisionPrescription>),
}

impl Resource {
    /// Whether this resource itself carries contained resources (`dom-2`).
    #[must_use]
    pub fn has_contained(&self) -> bool {
        match self {
            Self::Account(r) => !r.contained.is_empty(),
            Self::ActivityDefinition(r) => !r.contained.is_empty(),
            Self::AdverseEvent(r) => !r.contained.is_empty(),
            Self::AllergyIntolerance(r) => !r.contained.is_empty(),
            Self::Appointment(r) => !r.contained.is_empty(),
            Self::AppointmentResponse(r) => !r.contained.is_empty(),
            Self::AuditEvent(r) => !r.contained.is_empty(),
            Self::Basic(r) => !r.contained.is_empty(),
            Self::Binary(_) => false,
            Self::BodySite(r) => !r.contained.is_empty(),
            Self::Bundle(_) => false,
            Self::CapabilityStatement(r) => !r.contained.is_empty(),
            Self::CarePlan(r) => !r.contained.is_empty(),
            Self::CareTeam(r) => !r.contained.is_empty(),
            Self::ChargeItem(r) => !r.contained.is_empty(),
            Self::Claim(r) => !r.contained.is_empty(),
            Self::ClaimResponse(r) => !r.contained.is_empty(),
            Self::ClinicalImpression(r) => !r.contained.is_empty(),
            Self::CodeSystem(r) => !r.contained.is_empty(),
            Self::Communication(r) => !r.contained.is_empty(),
            Self::CommunicationRequest(r) => !r.contained.is_empty(),
            Self::CompartmentDefinition(r) => !r.contained.is_empty(),
            Self::Composition(r) => !r.contained.is_empty(),
            Self::ConceptMap(r) => !r.contained.is_empty(),
            Self::Condition(r) => !r.contained.is_empty(),
            Self::Consent(r) => !r.contained.is_empty(),
            Self::Contract(r) => !r.contained.is_empty(),
            Self::Coverage(r) => !r.contained.is_empty(),
            Self::DataElement(r) => !r.contained.is_empty(),
            Self::DetectedIssue(r) => !r.contained.is_empty(),
            Self::Device(r) => !r.contained.is_empty(),
            Self::DeviceComponent(r) => !r.contained.is_empty(),
            Self::DeviceMetric(r) => !r.contained.is_empty(),
            Self::DeviceRequest(r) => !r.contained.is_empty(),
            Self::DeviceUseStatement(r) => !r.contained.is_empty(),
            Self::DiagnosticReport(r) => !r.contained.is_empty(),
            Self::DocumentManifest(r) => !r.contained.is_empty(),
            Self::DocumentReference(r) => !r.contained.is_empty(),
            Self::EligibilityRequest(r) => !r.contained.is_empty(),
            Self::EligibilityResponse(r) => !r.contained.is_empty(),
            Self::Encounter(r) => !r.contained.is_empty(),
            Self::Endpoint(r) => !r.contained.is_empty(),
            Self::EnrollmentRequest(r) => !r.contained.is_empty(),
            Self::EnrollmentResponse(r) => !r.contained.is_empty(),
            Self::EpisodeOfCare(r) => !r.contained.is_empty(),
            Self::ExpansionProfile(r) => !r.contained.is_empty(),
            Self::ExplanationOfBenefit(r) => !r.contained.is_empty(),
            Self::FamilyMemberHistory(r) => !r.contained.is_empty(),
            Self::Flag(r) => !r.contained.is_empty(),
            Self::Goal(r) => !r.contained.is_empty(),
            Self::GraphDefinition(r) => !r.contained.is_empty(),
            Self::Group(r) => !r.contained.is_empty(),
            Self::GuidanceResponse(r) => !r.contained.is_empty(),
            Self::HealthcareService(r) => !r.contained.is_empty(),
            Self::ImagingManifest(r) => !r.contained.is_empty(),
            Self::ImagingStudy(r) => !r.contained.is_empty(),
            Self::Immunization(r) => !r.contained.is_empty(),
            Self::ImmunizationRecommendation(r) => !r.contained.is_empty(),
            Self::ImplementationGuide(r) => !r.contained.is_empty(),
            Self::Library(r) => !r.contained.is_empty(),
            Self::Linkage(r) => !r.contained.is_empty(),
            Self::List(r) => !r.contained.is_empty(),
            Self::Location(r) => !r.contained.is_empty(),
            Self::Measure(r) => !r.contained.is_empty(),
            Self::MeasureReport(r) => !r.contained.is_empty(),
            Self::Media(r) => !r.contained.is_empty(),
            Self::Medication(r) => !r.contained.is_empty(),
            Self::MedicationAdministration(r) => !r.contained.is_empty(),
            Self::MedicationDispense(r) => !r.contained.is_empty(),
            Self::MedicationRequest(r) => !r.contained.is_empty(),
            Self::MedicationStatement(r) => !r.contained.is_empty(),
            Self::MessageDefinition(r) => !r.contained.is_empty(),
            Self::MessageHeader(r) => !r.contained.is_empty(),
            Self::NamingSystem(r) => !r.contained.is_empty(),
            Self::NutritionOrder(r) => !r.contained.is_empty(),
            Self::Observation(r) => !r.contained.is_empty(),
            Self::OperationDefinition(r) => !r.contained.is_empty(),
            Self::OperationOutcome(r) => !r.contained.is_empty(),
            Self::Organization(r) => !r.contained.is_empty(),
            Self::Parameters(_) => false,
            Self::Patient(r) => !r.contained.is_empty(),
            Self::PaymentNotice(r) => !r.contained.is_empty(),
            Self::PaymentReconciliation(r) => !r.contained.is_empty(),
            Self::Person(r) => !r.contained.is_empty(),
            Self::PlanDefinition(r) => !r.contained.is_empty(),
            Self::Practitioner(r) => !r.contained.is_empty(),
            Self::PractitionerRole(r) => !r.contained.is_empty(),
            Self::Procedure(r) => !r.contained.is_empty(),
            Self::ProcedureRequest(r) => !r.contained.is_empty(),
            Self::ProcessRequest(r) => !r.contained.is_empty(),
            Self::ProcessResponse(r) => !r.contained.is_empty(),
            Self::Provenance(r) => !r.contained.is_empty(),
            Self::Questionnaire(r) => !r.contained.is_empty(),
            Self::QuestionnaireResponse(r) => !r.contained.is_empty(),
            Self::ReferralRequest(r) => !r.contained.is_empty(),
            Self::RelatedPerson(r) => !r.contained.is_empty(),
            Self::RequestGroup(r) => !r.contained.is_empty(),
            Self::ResearchStudy(r) => !r.contained.is_empty(),
            Self::ResearchSubject(r) => !r.contained.is_empty(),
            Self::RiskAssessment(r) => !r.contained.is_empty(),
            Self::Schedule(r) => !r.contained.is_empty(),
            Self::SearchParameter(r) => !r.contained.is_empty(),
            Self::Sequence(r) => !r.contained.is_empty(),
            Self::ServiceDefinition(r) => !r.contained.is_empty(),
            Self::Slot(r) => !r.contained.is_empty(),
            Self::Specimen(r) => !r.contained.is_empty(),
            Self::StructureDefinition(r) => !r.contained.is_empty(),
            Self::StructureMap(r) => !r.contained.is_empty(),
            Self::Subscription(r) => !r.contained.is_empty(),
            Self::Substance(r) => !r.contained.is_empty(),
            Self::SupplyDelivery(r) => !r.contained.is_empty(),
            Self::SupplyRequest(r) => !r.contained.is_empty(),
            Self::Task(r) => !r.contained.is_empty(),
            Self::TestReport(r) => !r.contained.is_empty(),
            Self::TestScript(r) => !r.contained.is_empty(),
            Self::ValueSet(r) => !r.contained.is_empty(),
            Self::VisionPrescription(r) => !r.contained.is_empty(),
        }
    }

    /// The resource's `meta`, when present (`dom-4`).
    #[must_use]
    pub fn meta(&self) -> Option<&crate::r3::types::Meta> {
        match self {
            Self::Account(r) => r.meta.as_ref(),
            Self::ActivityDefinition(r) => r.meta.as_ref(),
            Self::AdverseEvent(r) => r.meta.as_ref(),
            Self::AllergyIntolerance(r) => r.meta.as_ref(),
            Self::Appointment(r) => r.meta.as_ref(),
            Self::AppointmentResponse(r) => r.meta.as_ref(),
            Self::AuditEvent(r) => r.meta.as_ref(),
            Self::Basic(r) => r.meta.as_ref(),
            Self::Binary(r) => r.meta.as_ref(),
            Self::BodySite(r) => r.meta.as_ref(),
            Self::Bundle(r) => r.meta.as_ref(),
            Self::CapabilityStatement(r) => r.meta.as_ref(),
            Self::CarePlan(r) => r.meta.as_ref(),
            Self::CareTeam(r) => r.meta.as_ref(),
            Self::ChargeItem(r) => r.meta.as_ref(),
            Self::Claim(r) => r.meta.as_ref(),
            Self::ClaimResponse(r) => r.meta.as_ref(),
            Self::ClinicalImpression(r) => r.meta.as_ref(),
            Self::CodeSystem(r) => r.meta.as_ref(),
            Self::Communication(r) => r.meta.as_ref(),
            Self::CommunicationRequest(r) => r.meta.as_ref(),
            Self::CompartmentDefinition(r) => r.meta.as_ref(),
            Self::Composition(r) => r.meta.as_ref(),
            Self::ConceptMap(r) => r.meta.as_ref(),
            Self::Condition(r) => r.meta.as_ref(),
            Self::Consent(r) => r.meta.as_ref(),
            Self::Contract(r) => r.meta.as_ref(),
            Self::Coverage(r) => r.meta.as_ref(),
            Self::DataElement(r) => r.meta.as_ref(),
            Self::DetectedIssue(r) => r.meta.as_ref(),
            Self::Device(r) => r.meta.as_ref(),
            Self::DeviceComponent(r) => r.meta.as_ref(),
            Self::DeviceMetric(r) => r.meta.as_ref(),
            Self::DeviceRequest(r) => r.meta.as_ref(),
            Self::DeviceUseStatement(r) => r.meta.as_ref(),
            Self::DiagnosticReport(r) => r.meta.as_ref(),
            Self::DocumentManifest(r) => r.meta.as_ref(),
            Self::DocumentReference(r) => r.meta.as_ref(),
            Self::EligibilityRequest(r) => r.meta.as_ref(),
            Self::EligibilityResponse(r) => r.meta.as_ref(),
            Self::Encounter(r) => r.meta.as_ref(),
            Self::Endpoint(r) => r.meta.as_ref(),
            Self::EnrollmentRequest(r) => r.meta.as_ref(),
            Self::EnrollmentResponse(r) => r.meta.as_ref(),
            Self::EpisodeOfCare(r) => r.meta.as_ref(),
            Self::ExpansionProfile(r) => r.meta.as_ref(),
            Self::ExplanationOfBenefit(r) => r.meta.as_ref(),
            Self::FamilyMemberHistory(r) => r.meta.as_ref(),
            Self::Flag(r) => r.meta.as_ref(),
            Self::Goal(r) => r.meta.as_ref(),
            Self::GraphDefinition(r) => r.meta.as_ref(),
            Self::Group(r) => r.meta.as_ref(),
            Self::GuidanceResponse(r) => r.meta.as_ref(),
            Self::HealthcareService(r) => r.meta.as_ref(),
            Self::ImagingManifest(r) => r.meta.as_ref(),
            Self::ImagingStudy(r) => r.meta.as_ref(),
            Self::Immunization(r) => r.meta.as_ref(),
            Self::ImmunizationRecommendation(r) => r.meta.as_ref(),
            Self::ImplementationGuide(r) => r.meta.as_ref(),
            Self::Library(r) => r.meta.as_ref(),
            Self::Linkage(r) => r.meta.as_ref(),
            Self::List(r) => r.meta.as_ref(),
            Self::Location(r) => r.meta.as_ref(),
            Self::Measure(r) => r.meta.as_ref(),
            Self::MeasureReport(r) => r.meta.as_ref(),
            Self::Media(r) => r.meta.as_ref(),
            Self::Medication(r) => r.meta.as_ref(),
            Self::MedicationAdministration(r) => r.meta.as_ref(),
            Self::MedicationDispense(r) => r.meta.as_ref(),
            Self::MedicationRequest(r) => r.meta.as_ref(),
            Self::MedicationStatement(r) => r.meta.as_ref(),
            Self::MessageDefinition(r) => r.meta.as_ref(),
            Self::MessageHeader(r) => r.meta.as_ref(),
            Self::NamingSystem(r) => r.meta.as_ref(),
            Self::NutritionOrder(r) => r.meta.as_ref(),
            Self::Observation(r) => r.meta.as_ref(),
            Self::OperationDefinition(r) => r.meta.as_ref(),
            Self::OperationOutcome(r) => r.meta.as_ref(),
            Self::Organization(r) => r.meta.as_ref(),
            Self::Parameters(r) => r.meta.as_ref(),
            Self::Patient(r) => r.meta.as_ref(),
            Self::PaymentNotice(r) => r.meta.as_ref(),
            Self::PaymentReconciliation(r) => r.meta.as_ref(),
            Self::Person(r) => r.meta.as_ref(),
            Self::PlanDefinition(r) => r.meta.as_ref(),
            Self::Practitioner(r) => r.meta.as_ref(),
            Self::PractitionerRole(r) => r.meta.as_ref(),
            Self::Procedure(r) => r.meta.as_ref(),
            Self::ProcedureRequest(r) => r.meta.as_ref(),
            Self::ProcessRequest(r) => r.meta.as_ref(),
            Self::ProcessResponse(r) => r.meta.as_ref(),
            Self::Provenance(r) => r.meta.as_ref(),
            Self::Questionnaire(r) => r.meta.as_ref(),
            Self::QuestionnaireResponse(r) => r.meta.as_ref(),
            Self::ReferralRequest(r) => r.meta.as_ref(),
            Self::RelatedPerson(r) => r.meta.as_ref(),
            Self::RequestGroup(r) => r.meta.as_ref(),
            Self::ResearchStudy(r) => r.meta.as_ref(),
            Self::ResearchSubject(r) => r.meta.as_ref(),
            Self::RiskAssessment(r) => r.meta.as_ref(),
            Self::Schedule(r) => r.meta.as_ref(),
            Self::SearchParameter(r) => r.meta.as_ref(),
            Self::Sequence(r) => r.meta.as_ref(),
            Self::ServiceDefinition(r) => r.meta.as_ref(),
            Self::Slot(r) => r.meta.as_ref(),
            Self::Specimen(r) => r.meta.as_ref(),
            Self::StructureDefinition(r) => r.meta.as_ref(),
            Self::StructureMap(r) => r.meta.as_ref(),
            Self::Subscription(r) => r.meta.as_ref(),
            Self::Substance(r) => r.meta.as_ref(),
            Self::SupplyDelivery(r) => r.meta.as_ref(),
            Self::SupplyRequest(r) => r.meta.as_ref(),
            Self::Task(r) => r.meta.as_ref(),
            Self::TestReport(r) => r.meta.as_ref(),
            Self::TestScript(r) => r.meta.as_ref(),
            Self::ValueSet(r) => r.meta.as_ref(),
            Self::VisionPrescription(r) => r.meta.as_ref(),
        }
    }
}
