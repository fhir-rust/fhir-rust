//! FHIR R2 resources.
//!
//! This module contains the FHIR R2 resource types (Patient, Observation,
//! Encounter, and so on). Each resource is a Rust struct that serializes to
//! and from the canonical FHIR JSON representation via `serde`.

use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

pub mod account;
pub mod allergy_intolerance;
pub mod appointment;
pub mod appointment_response;
pub mod audit_event;
pub mod basic;
pub mod binary;
pub mod body_site;
pub mod bundle;
pub mod care_plan;
pub mod claim;
pub mod claim_response;
pub mod clinical_impression;
pub mod communication;
pub mod communication_request;
pub mod composition;
pub mod concept_map;
pub mod condition;
pub mod conformance;
pub mod contract;
pub mod coverage;
pub mod data_element;
pub mod detected_issue;
pub mod device;
pub mod device_component;
pub mod device_metric;
pub mod device_use_request;
pub mod device_use_statement;
pub mod diagnostic_order;
pub mod diagnostic_report;
pub mod document_manifest;
pub mod document_reference;
pub mod eligibility_request;
pub mod eligibility_response;
pub mod encounter;
pub mod enrollment_request;
pub mod enrollment_response;
pub mod episode_of_care;
pub mod explanation_of_benefit;
pub mod family_member_history;
pub mod flag;
pub mod goal;
pub mod group;
pub mod healthcare_service;
pub mod imaging_object_selection;
pub mod imaging_study;
pub mod immunization;
pub mod immunization_recommendation;
pub mod implementation_guide;
pub mod list;
pub mod location;
pub mod media;
pub mod medication;
pub mod medication_administration;
pub mod medication_dispense;
pub mod medication_order;
pub mod medication_statement;
pub mod message_header;
pub mod naming_system;
pub mod nutrition_order;
pub mod observation;
pub mod operation_definition;
pub mod operation_outcome;
pub mod order;
pub mod order_response;
pub mod organization;
pub mod parameters;
pub mod patient;
pub mod payment_notice;
pub mod payment_reconciliation;
pub mod person;
pub mod practitioner;
pub mod procedure;
pub mod procedure_request;
pub mod process_request;
pub mod process_response;
pub mod provenance;
pub mod questionnaire;
pub mod questionnaire_response;
pub mod referral_request;
pub mod related_person;
pub mod risk_assessment;
pub mod schedule;
pub mod search_parameter;
pub mod slot;
pub mod specimen;
pub mod structure_definition;
pub mod subscription;
pub mod substance;
pub mod supply_delivery;
pub mod supply_request;
pub mod test_script;
pub mod value_set;
pub mod vision_prescription;

pub use account::Account;
pub use allergy_intolerance::AllergyIntolerance;
pub use appointment::Appointment;
pub use appointment_response::AppointmentResponse;
pub use audit_event::AuditEvent;
pub use basic::Basic;
pub use binary::Binary;
pub use body_site::BodySite;
pub use bundle::Bundle;
pub use care_plan::CarePlan;
pub use claim::Claim;
pub use claim_response::ClaimResponse;
pub use clinical_impression::ClinicalImpression;
pub use communication::Communication;
pub use communication_request::CommunicationRequest;
pub use composition::Composition;
pub use concept_map::ConceptMap;
pub use condition::Condition;
pub use conformance::Conformance;
pub use contract::Contract;
pub use coverage::Coverage;
pub use data_element::DataElement;
pub use detected_issue::DetectedIssue;
pub use device::Device;
pub use device_component::DeviceComponent;
pub use device_metric::DeviceMetric;
pub use device_use_request::DeviceUseRequest;
pub use device_use_statement::DeviceUseStatement;
pub use diagnostic_order::DiagnosticOrder;
pub use diagnostic_report::DiagnosticReport;
pub use document_manifest::DocumentManifest;
pub use document_reference::DocumentReference;
pub use eligibility_request::EligibilityRequest;
pub use eligibility_response::EligibilityResponse;
pub use encounter::Encounter;
pub use enrollment_request::EnrollmentRequest;
pub use enrollment_response::EnrollmentResponse;
pub use episode_of_care::EpisodeOfCare;
pub use explanation_of_benefit::ExplanationOfBenefit;
pub use family_member_history::FamilyMemberHistory;
pub use flag::Flag;
pub use goal::Goal;
pub use group::Group;
pub use healthcare_service::HealthcareService;
pub use imaging_object_selection::ImagingObjectSelection;
pub use imaging_study::ImagingStudy;
pub use immunization::Immunization;
pub use immunization_recommendation::ImmunizationRecommendation;
pub use implementation_guide::ImplementationGuide;
pub use list::List;
pub use location::Location;
pub use media::Media;
pub use medication::Medication;
pub use medication_administration::MedicationAdministration;
pub use medication_dispense::MedicationDispense;
pub use medication_order::MedicationOrder;
pub use medication_statement::MedicationStatement;
pub use message_header::MessageHeader;
pub use naming_system::NamingSystem;
pub use nutrition_order::NutritionOrder;
pub use observation::Observation;
pub use operation_definition::OperationDefinition;
pub use operation_outcome::OperationOutcome;
pub use order::Order;
pub use order_response::OrderResponse;
pub use organization::Organization;
pub use parameters::Parameters;
pub use patient::Patient;
pub use payment_notice::PaymentNotice;
pub use payment_reconciliation::PaymentReconciliation;
pub use person::Person;
pub use practitioner::Practitioner;
pub use procedure::Procedure;
pub use procedure_request::ProcedureRequest;
pub use process_request::ProcessRequest;
pub use process_response::ProcessResponse;
pub use provenance::Provenance;
pub use questionnaire::Questionnaire;
pub use questionnaire_response::QuestionnaireResponse;
pub use referral_request::ReferralRequest;
pub use related_person::RelatedPerson;
pub use risk_assessment::RiskAssessment;
pub use schedule::Schedule;
pub use search_parameter::SearchParameter;
pub use slot::Slot;
pub use specimen::Specimen;
pub use structure_definition::StructureDefinition;
pub use subscription::Subscription;
pub use substance::Substance;
pub use supply_delivery::SupplyDelivery;
pub use supply_request::SupplyRequest;
pub use test_script::TestScript;
pub use value_set::ValueSet;
pub use vision_prescription::VisionPrescription;

/// Any FHIR R2 resource, tagged by its `resourceType`.
///
/// Used wherever a resource of any type may appear — for example a
/// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes
/// the `resourceType` discriminator automatically.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::Resource;
///
/// let json = ::serde_json::json!({"resourceType": "Patient"});
/// let resource: Resource = ::serde_json::from_value(json).unwrap();
/// assert!(matches!(resource, Resource::Patient(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(tag = "resourceType")]
#[fhir_version("r2")]
pub enum Resource {
    Account(Box<account::Account>),
    AllergyIntolerance(Box<allergy_intolerance::AllergyIntolerance>),
    Appointment(Box<appointment::Appointment>),
    AppointmentResponse(Box<appointment_response::AppointmentResponse>),
    AuditEvent(Box<audit_event::AuditEvent>),
    Basic(Box<basic::Basic>),
    Binary(Box<binary::Binary>),
    BodySite(Box<body_site::BodySite>),
    Bundle(Box<bundle::Bundle>),
    CarePlan(Box<care_plan::CarePlan>),
    Claim(Box<claim::Claim>),
    ClaimResponse(Box<claim_response::ClaimResponse>),
    ClinicalImpression(Box<clinical_impression::ClinicalImpression>),
    Communication(Box<communication::Communication>),
    CommunicationRequest(Box<communication_request::CommunicationRequest>),
    Composition(Box<composition::Composition>),
    ConceptMap(Box<concept_map::ConceptMap>),
    Condition(Box<condition::Condition>),
    Conformance(Box<conformance::Conformance>),
    Contract(Box<contract::Contract>),
    Coverage(Box<coverage::Coverage>),
    DataElement(Box<data_element::DataElement>),
    DetectedIssue(Box<detected_issue::DetectedIssue>),
    Device(Box<device::Device>),
    DeviceComponent(Box<device_component::DeviceComponent>),
    DeviceMetric(Box<device_metric::DeviceMetric>),
    DeviceUseRequest(Box<device_use_request::DeviceUseRequest>),
    DeviceUseStatement(Box<device_use_statement::DeviceUseStatement>),
    DiagnosticOrder(Box<diagnostic_order::DiagnosticOrder>),
    DiagnosticReport(Box<diagnostic_report::DiagnosticReport>),
    DocumentManifest(Box<document_manifest::DocumentManifest>),
    DocumentReference(Box<document_reference::DocumentReference>),
    EligibilityRequest(Box<eligibility_request::EligibilityRequest>),
    EligibilityResponse(Box<eligibility_response::EligibilityResponse>),
    Encounter(Box<encounter::Encounter>),
    EnrollmentRequest(Box<enrollment_request::EnrollmentRequest>),
    EnrollmentResponse(Box<enrollment_response::EnrollmentResponse>),
    EpisodeOfCare(Box<episode_of_care::EpisodeOfCare>),
    ExplanationOfBenefit(Box<explanation_of_benefit::ExplanationOfBenefit>),
    FamilyMemberHistory(Box<family_member_history::FamilyMemberHistory>),
    Flag(Box<flag::Flag>),
    Goal(Box<goal::Goal>),
    Group(Box<group::Group>),
    HealthcareService(Box<healthcare_service::HealthcareService>),
    ImagingObjectSelection(Box<imaging_object_selection::ImagingObjectSelection>),
    ImagingStudy(Box<imaging_study::ImagingStudy>),
    Immunization(Box<immunization::Immunization>),
    ImmunizationRecommendation(Box<immunization_recommendation::ImmunizationRecommendation>),
    ImplementationGuide(Box<implementation_guide::ImplementationGuide>),
    List(Box<list::List>),
    Location(Box<location::Location>),
    Media(Box<media::Media>),
    Medication(Box<medication::Medication>),
    MedicationAdministration(Box<medication_administration::MedicationAdministration>),
    MedicationDispense(Box<medication_dispense::MedicationDispense>),
    MedicationOrder(Box<medication_order::MedicationOrder>),
    MedicationStatement(Box<medication_statement::MedicationStatement>),
    MessageHeader(Box<message_header::MessageHeader>),
    NamingSystem(Box<naming_system::NamingSystem>),
    NutritionOrder(Box<nutrition_order::NutritionOrder>),
    Observation(Box<observation::Observation>),
    OperationDefinition(Box<operation_definition::OperationDefinition>),
    OperationOutcome(Box<operation_outcome::OperationOutcome>),
    Order(Box<order::Order>),
    OrderResponse(Box<order_response::OrderResponse>),
    Organization(Box<organization::Organization>),
    Parameters(Box<parameters::Parameters>),
    Patient(Box<patient::Patient>),
    PaymentNotice(Box<payment_notice::PaymentNotice>),
    PaymentReconciliation(Box<payment_reconciliation::PaymentReconciliation>),
    Person(Box<person::Person>),
    Practitioner(Box<practitioner::Practitioner>),
    Procedure(Box<procedure::Procedure>),
    ProcedureRequest(Box<procedure_request::ProcedureRequest>),
    ProcessRequest(Box<process_request::ProcessRequest>),
    ProcessResponse(Box<process_response::ProcessResponse>),
    Provenance(Box<provenance::Provenance>),
    Questionnaire(Box<questionnaire::Questionnaire>),
    QuestionnaireResponse(Box<questionnaire_response::QuestionnaireResponse>),
    ReferralRequest(Box<referral_request::ReferralRequest>),
    RelatedPerson(Box<related_person::RelatedPerson>),
    RiskAssessment(Box<risk_assessment::RiskAssessment>),
    Schedule(Box<schedule::Schedule>),
    SearchParameter(Box<search_parameter::SearchParameter>),
    Slot(Box<slot::Slot>),
    Specimen(Box<specimen::Specimen>),
    StructureDefinition(Box<structure_definition::StructureDefinition>),
    Subscription(Box<subscription::Subscription>),
    Substance(Box<substance::Substance>),
    SupplyDelivery(Box<supply_delivery::SupplyDelivery>),
    SupplyRequest(Box<supply_request::SupplyRequest>),
    TestScript(Box<test_script::TestScript>),
    ValueSet(Box<value_set::ValueSet>),
    VisionPrescription(Box<vision_prescription::VisionPrescription>),
}
