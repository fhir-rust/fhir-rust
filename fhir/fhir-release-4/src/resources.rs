//! FHIR R4 resources.
//!
//! This module contains the FHIR R4 resource types (Patient, Observation,
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
pub mod biologically_derived_product;
pub mod body_structure;
pub mod bundle;
pub mod capability_statement;
pub mod care_plan;
pub mod care_team;
pub mod catalog_entry;
pub mod charge_item;
pub mod charge_item_definition;
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
pub mod coverage_eligibility_request;
pub mod coverage_eligibility_response;
pub mod detected_issue;
pub mod device;
pub mod device_definition;
pub mod device_metric;
pub mod device_request;
pub mod device_use_statement;
pub mod diagnostic_report;
pub mod document_manifest;
pub mod document_reference;
pub mod effect_evidence_synthesis;
pub mod encounter;
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
pub mod goal;
pub mod graph_definition;
pub mod group;
pub mod guidance_response;
pub mod healthcare_service;
pub mod imaging_study;
pub mod immunization;
pub mod immunization_evaluation;
pub mod immunization_recommendation;
pub mod implementation_guide;
pub mod insurance_plan;
pub mod invoice;
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
pub mod medication_knowledge;
pub mod medication_request;
pub mod medication_statement;
pub mod medicinal_product;
pub mod medicinal_product_authorization;
pub mod medicinal_product_contraindication;
pub mod medicinal_product_indication;
pub mod medicinal_product_ingredient;
pub mod medicinal_product_interaction;
pub mod medicinal_product_manufactured;
pub mod medicinal_product_packaged;
pub mod medicinal_product_pharmaceutical;
pub mod medicinal_product_undesirable_effect;
pub mod message_definition;
pub mod message_header;
pub mod molecular_sequence;
pub mod naming_system;
pub mod nutrition_order;
pub mod observation;
pub mod observation_definition;
pub mod operation_definition;
pub mod operation_outcome;
pub mod organization;
pub mod organization_affiliation;
pub mod parameters;
pub mod patient;
pub mod payment_notice;
pub mod payment_reconciliation;
pub mod person;
pub mod plan_definition;
pub mod practitioner;
pub mod practitioner_role;
pub mod procedure;
pub mod provenance;
pub mod questionnaire;
pub mod questionnaire_response;
pub mod related_person;
pub mod request_group;
pub mod research_definition;
pub mod research_element_definition;
pub mod research_study;
pub mod research_subject;
pub mod risk_assessment;
pub mod risk_evidence_synthesis;
pub mod schedule;
pub mod search_parameter;
pub mod service_request;
pub mod slot;
pub mod specimen;
pub mod specimen_definition;
pub mod structure_definition;
pub mod structure_map;
pub mod subscription;
pub mod substance;
pub mod substance_nucleic_acid;
pub mod substance_polymer;
pub mod substance_protein;
pub mod substance_reference_information;
pub mod substance_source_material;
pub mod substance_specification;
pub mod supply_delivery;
pub mod supply_request;
pub mod task;
pub mod terminology_capabilities;
pub mod test_report;
pub mod test_script;
pub mod value_set;
pub mod verification_result;
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
pub use biologically_derived_product::BiologicallyDerivedProduct;
pub use body_structure::BodyStructure;
pub use bundle::Bundle;
pub use capability_statement::CapabilityStatement;
pub use care_plan::CarePlan;
pub use care_team::CareTeam;
pub use catalog_entry::CatalogEntry;
pub use charge_item::ChargeItem;
pub use charge_item_definition::ChargeItemDefinition;
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
pub use coverage_eligibility_request::CoverageEligibilityRequest;
pub use coverage_eligibility_response::CoverageEligibilityResponse;
pub use detected_issue::DetectedIssue;
pub use device::Device;
pub use device_definition::DeviceDefinition;
pub use device_metric::DeviceMetric;
pub use device_request::DeviceRequest;
pub use device_use_statement::DeviceUseStatement;
pub use diagnostic_report::DiagnosticReport;
pub use document_manifest::DocumentManifest;
pub use document_reference::DocumentReference;
pub use effect_evidence_synthesis::EffectEvidenceSynthesis;
pub use encounter::Encounter;
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
pub use goal::Goal;
pub use graph_definition::GraphDefinition;
pub use group::Group;
pub use guidance_response::GuidanceResponse;
pub use healthcare_service::HealthcareService;
pub use imaging_study::ImagingStudy;
pub use immunization::Immunization;
pub use immunization_evaluation::ImmunizationEvaluation;
pub use immunization_recommendation::ImmunizationRecommendation;
pub use implementation_guide::ImplementationGuide;
pub use insurance_plan::InsurancePlan;
pub use invoice::Invoice;
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
pub use medication_knowledge::MedicationKnowledge;
pub use medication_request::MedicationRequest;
pub use medication_statement::MedicationStatement;
pub use medicinal_product::MedicinalProduct;
pub use medicinal_product_authorization::MedicinalProductAuthorization;
pub use medicinal_product_contraindication::MedicinalProductContraindication;
pub use medicinal_product_indication::MedicinalProductIndication;
pub use medicinal_product_ingredient::MedicinalProductIngredient;
pub use medicinal_product_interaction::MedicinalProductInteraction;
pub use medicinal_product_manufactured::MedicinalProductManufactured;
pub use medicinal_product_packaged::MedicinalProductPackaged;
pub use medicinal_product_pharmaceutical::MedicinalProductPharmaceutical;
pub use medicinal_product_undesirable_effect::MedicinalProductUndesirableEffect;
pub use message_definition::MessageDefinition;
pub use message_header::MessageHeader;
pub use molecular_sequence::MolecularSequence;
pub use naming_system::NamingSystem;
pub use nutrition_order::NutritionOrder;
pub use observation::Observation;
pub use observation_definition::ObservationDefinition;
pub use operation_definition::OperationDefinition;
pub use operation_outcome::OperationOutcome;
pub use organization::Organization;
pub use organization_affiliation::OrganizationAffiliation;
pub use parameters::Parameters;
pub use patient::Patient;
pub use payment_notice::PaymentNotice;
pub use payment_reconciliation::PaymentReconciliation;
pub use person::Person;
pub use plan_definition::PlanDefinition;
pub use practitioner::Practitioner;
pub use practitioner_role::PractitionerRole;
pub use procedure::Procedure;
pub use provenance::Provenance;
pub use questionnaire::Questionnaire;
pub use questionnaire_response::QuestionnaireResponse;
pub use related_person::RelatedPerson;
pub use request_group::RequestGroup;
pub use research_definition::ResearchDefinition;
pub use research_element_definition::ResearchElementDefinition;
pub use research_study::ResearchStudy;
pub use research_subject::ResearchSubject;
pub use risk_assessment::RiskAssessment;
pub use risk_evidence_synthesis::RiskEvidenceSynthesis;
pub use schedule::Schedule;
pub use search_parameter::SearchParameter;
pub use service_request::ServiceRequest;
pub use slot::Slot;
pub use specimen::Specimen;
pub use specimen_definition::SpecimenDefinition;
pub use structure_definition::StructureDefinition;
pub use structure_map::StructureMap;
pub use subscription::Subscription;
pub use substance::Substance;
pub use substance_nucleic_acid::SubstanceNucleicAcid;
pub use substance_polymer::SubstancePolymer;
pub use substance_protein::SubstanceProtein;
pub use substance_reference_information::SubstanceReferenceInformation;
pub use substance_source_material::SubstanceSourceMaterial;
pub use substance_specification::SubstanceSpecification;
pub use supply_delivery::SupplyDelivery;
pub use supply_request::SupplyRequest;
pub use task::Task;
pub use terminology_capabilities::TerminologyCapabilities;
pub use test_report::TestReport;
pub use test_script::TestScript;
pub use value_set::ValueSet;
pub use verification_result::VerificationResult;
pub use vision_prescription::VisionPrescription;

/// Any FHIR R4 resource, tagged by its `resourceType`.
///
/// Used wherever a resource of any type may appear — for example a
/// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes
/// the `resourceType` discriminator automatically.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::Resource;
///
/// let json = ::serde_json::json!({"resourceType": "Patient"});
/// let resource: Resource = ::serde_json::from_value(json).unwrap();
/// assert!(matches!(resource, Resource::Patient(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(tag = "resourceType")]
#[fhir_version("r4")]
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
    BiologicallyDerivedProduct(Box<biologically_derived_product::BiologicallyDerivedProduct>),
    BodyStructure(Box<body_structure::BodyStructure>),
    Bundle(Box<bundle::Bundle>),
    CapabilityStatement(Box<capability_statement::CapabilityStatement>),
    CarePlan(Box<care_plan::CarePlan>),
    CareTeam(Box<care_team::CareTeam>),
    CatalogEntry(Box<catalog_entry::CatalogEntry>),
    ChargeItem(Box<charge_item::ChargeItem>),
    ChargeItemDefinition(Box<charge_item_definition::ChargeItemDefinition>),
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
    CoverageEligibilityRequest(Box<coverage_eligibility_request::CoverageEligibilityRequest>),
    CoverageEligibilityResponse(Box<coverage_eligibility_response::CoverageEligibilityResponse>),
    DetectedIssue(Box<detected_issue::DetectedIssue>),
    Device(Box<device::Device>),
    DeviceDefinition(Box<device_definition::DeviceDefinition>),
    DeviceMetric(Box<device_metric::DeviceMetric>),
    DeviceRequest(Box<device_request::DeviceRequest>),
    DeviceUseStatement(Box<device_use_statement::DeviceUseStatement>),
    DiagnosticReport(Box<diagnostic_report::DiagnosticReport>),
    DocumentManifest(Box<document_manifest::DocumentManifest>),
    DocumentReference(Box<document_reference::DocumentReference>),
    EffectEvidenceSynthesis(Box<effect_evidence_synthesis::EffectEvidenceSynthesis>),
    Encounter(Box<encounter::Encounter>),
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
    Goal(Box<goal::Goal>),
    GraphDefinition(Box<graph_definition::GraphDefinition>),
    Group(Box<group::Group>),
    GuidanceResponse(Box<guidance_response::GuidanceResponse>),
    HealthcareService(Box<healthcare_service::HealthcareService>),
    ImagingStudy(Box<imaging_study::ImagingStudy>),
    Immunization(Box<immunization::Immunization>),
    ImmunizationEvaluation(Box<immunization_evaluation::ImmunizationEvaluation>),
    ImmunizationRecommendation(Box<immunization_recommendation::ImmunizationRecommendation>),
    ImplementationGuide(Box<implementation_guide::ImplementationGuide>),
    InsurancePlan(Box<insurance_plan::InsurancePlan>),
    Invoice(Box<invoice::Invoice>),
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
    MedicationKnowledge(Box<medication_knowledge::MedicationKnowledge>),
    MedicationRequest(Box<medication_request::MedicationRequest>),
    MedicationStatement(Box<medication_statement::MedicationStatement>),
    MedicinalProduct(Box<medicinal_product::MedicinalProduct>),
    MedicinalProductAuthorization(
        Box<medicinal_product_authorization::MedicinalProductAuthorization>,
    ),
    MedicinalProductContraindication(
        Box<medicinal_product_contraindication::MedicinalProductContraindication>,
    ),
    MedicinalProductIndication(Box<medicinal_product_indication::MedicinalProductIndication>),
    MedicinalProductIngredient(Box<medicinal_product_ingredient::MedicinalProductIngredient>),
    MedicinalProductInteraction(Box<medicinal_product_interaction::MedicinalProductInteraction>),
    MedicinalProductManufactured(Box<medicinal_product_manufactured::MedicinalProductManufactured>),
    MedicinalProductPackaged(Box<medicinal_product_packaged::MedicinalProductPackaged>),
    MedicinalProductPharmaceutical(
        Box<medicinal_product_pharmaceutical::MedicinalProductPharmaceutical>,
    ),
    MedicinalProductUndesirableEffect(
        Box<medicinal_product_undesirable_effect::MedicinalProductUndesirableEffect>,
    ),
    MessageDefinition(Box<message_definition::MessageDefinition>),
    MessageHeader(Box<message_header::MessageHeader>),
    MolecularSequence(Box<molecular_sequence::MolecularSequence>),
    NamingSystem(Box<naming_system::NamingSystem>),
    NutritionOrder(Box<nutrition_order::NutritionOrder>),
    Observation(Box<observation::Observation>),
    ObservationDefinition(Box<observation_definition::ObservationDefinition>),
    OperationDefinition(Box<operation_definition::OperationDefinition>),
    OperationOutcome(Box<operation_outcome::OperationOutcome>),
    Organization(Box<organization::Organization>),
    OrganizationAffiliation(Box<organization_affiliation::OrganizationAffiliation>),
    Parameters(Box<parameters::Parameters>),
    Patient(Box<patient::Patient>),
    PaymentNotice(Box<payment_notice::PaymentNotice>),
    PaymentReconciliation(Box<payment_reconciliation::PaymentReconciliation>),
    Person(Box<person::Person>),
    PlanDefinition(Box<plan_definition::PlanDefinition>),
    Practitioner(Box<practitioner::Practitioner>),
    PractitionerRole(Box<practitioner_role::PractitionerRole>),
    Procedure(Box<procedure::Procedure>),
    Provenance(Box<provenance::Provenance>),
    Questionnaire(Box<questionnaire::Questionnaire>),
    QuestionnaireResponse(Box<questionnaire_response::QuestionnaireResponse>),
    RelatedPerson(Box<related_person::RelatedPerson>),
    RequestGroup(Box<request_group::RequestGroup>),
    ResearchDefinition(Box<research_definition::ResearchDefinition>),
    ResearchElementDefinition(Box<research_element_definition::ResearchElementDefinition>),
    ResearchStudy(Box<research_study::ResearchStudy>),
    ResearchSubject(Box<research_subject::ResearchSubject>),
    RiskAssessment(Box<risk_assessment::RiskAssessment>),
    RiskEvidenceSynthesis(Box<risk_evidence_synthesis::RiskEvidenceSynthesis>),
    Schedule(Box<schedule::Schedule>),
    SearchParameter(Box<search_parameter::SearchParameter>),
    ServiceRequest(Box<service_request::ServiceRequest>),
    Slot(Box<slot::Slot>),
    Specimen(Box<specimen::Specimen>),
    SpecimenDefinition(Box<specimen_definition::SpecimenDefinition>),
    StructureDefinition(Box<structure_definition::StructureDefinition>),
    StructureMap(Box<structure_map::StructureMap>),
    Subscription(Box<subscription::Subscription>),
    Substance(Box<substance::Substance>),
    SubstanceNucleicAcid(Box<substance_nucleic_acid::SubstanceNucleicAcid>),
    SubstancePolymer(Box<substance_polymer::SubstancePolymer>),
    SubstanceProtein(Box<substance_protein::SubstanceProtein>),
    SubstanceReferenceInformation(
        Box<substance_reference_information::SubstanceReferenceInformation>,
    ),
    SubstanceSourceMaterial(Box<substance_source_material::SubstanceSourceMaterial>),
    SubstanceSpecification(Box<substance_specification::SubstanceSpecification>),
    SupplyDelivery(Box<supply_delivery::SupplyDelivery>),
    SupplyRequest(Box<supply_request::SupplyRequest>),
    Task(Box<task::Task>),
    TerminologyCapabilities(Box<terminology_capabilities::TerminologyCapabilities>),
    TestReport(Box<test_report::TestReport>),
    TestScript(Box<test_script::TestScript>),
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
            Self::AdverseEvent(r) => !r.contained.is_empty(),
            Self::AllergyIntolerance(r) => !r.contained.is_empty(),
            Self::Appointment(r) => !r.contained.is_empty(),
            Self::AppointmentResponse(r) => !r.contained.is_empty(),
            Self::AuditEvent(r) => !r.contained.is_empty(),
            Self::Basic(r) => !r.contained.is_empty(),
            Self::Binary(_) => false,
            Self::BiologicallyDerivedProduct(r) => !r.contained.is_empty(),
            Self::BodyStructure(r) => !r.contained.is_empty(),
            Self::Bundle(_) => false,
            Self::CapabilityStatement(r) => !r.contained.is_empty(),
            Self::CarePlan(r) => !r.contained.is_empty(),
            Self::CareTeam(r) => !r.contained.is_empty(),
            Self::CatalogEntry(r) => !r.contained.is_empty(),
            Self::ChargeItem(r) => !r.contained.is_empty(),
            Self::ChargeItemDefinition(r) => !r.contained.is_empty(),
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
            Self::CoverageEligibilityRequest(r) => !r.contained.is_empty(),
            Self::CoverageEligibilityResponse(r) => !r.contained.is_empty(),
            Self::DetectedIssue(r) => !r.contained.is_empty(),
            Self::Device(r) => !r.contained.is_empty(),
            Self::DeviceDefinition(r) => !r.contained.is_empty(),
            Self::DeviceMetric(r) => !r.contained.is_empty(),
            Self::DeviceRequest(r) => !r.contained.is_empty(),
            Self::DeviceUseStatement(r) => !r.contained.is_empty(),
            Self::DiagnosticReport(r) => !r.contained.is_empty(),
            Self::DocumentManifest(r) => !r.contained.is_empty(),
            Self::DocumentReference(r) => !r.contained.is_empty(),
            Self::EffectEvidenceSynthesis(r) => !r.contained.is_empty(),
            Self::Encounter(r) => !r.contained.is_empty(),
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
            Self::Goal(r) => !r.contained.is_empty(),
            Self::GraphDefinition(r) => !r.contained.is_empty(),
            Self::Group(r) => !r.contained.is_empty(),
            Self::GuidanceResponse(r) => !r.contained.is_empty(),
            Self::HealthcareService(r) => !r.contained.is_empty(),
            Self::ImagingStudy(r) => !r.contained.is_empty(),
            Self::Immunization(r) => !r.contained.is_empty(),
            Self::ImmunizationEvaluation(r) => !r.contained.is_empty(),
            Self::ImmunizationRecommendation(r) => !r.contained.is_empty(),
            Self::ImplementationGuide(r) => !r.contained.is_empty(),
            Self::InsurancePlan(r) => !r.contained.is_empty(),
            Self::Invoice(r) => !r.contained.is_empty(),
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
            Self::MedicationKnowledge(r) => !r.contained.is_empty(),
            Self::MedicationRequest(r) => !r.contained.is_empty(),
            Self::MedicationStatement(r) => !r.contained.is_empty(),
            Self::MedicinalProduct(r) => !r.contained.is_empty(),
            Self::MedicinalProductAuthorization(r) => !r.contained.is_empty(),
            Self::MedicinalProductContraindication(r) => !r.contained.is_empty(),
            Self::MedicinalProductIndication(r) => !r.contained.is_empty(),
            Self::MedicinalProductIngredient(r) => !r.contained.is_empty(),
            Self::MedicinalProductInteraction(r) => !r.contained.is_empty(),
            Self::MedicinalProductManufactured(r) => !r.contained.is_empty(),
            Self::MedicinalProductPackaged(r) => !r.contained.is_empty(),
            Self::MedicinalProductPharmaceutical(r) => !r.contained.is_empty(),
            Self::MedicinalProductUndesirableEffect(r) => !r.contained.is_empty(),
            Self::MessageDefinition(r) => !r.contained.is_empty(),
            Self::MessageHeader(r) => !r.contained.is_empty(),
            Self::MolecularSequence(r) => !r.contained.is_empty(),
            Self::NamingSystem(r) => !r.contained.is_empty(),
            Self::NutritionOrder(r) => !r.contained.is_empty(),
            Self::Observation(r) => !r.contained.is_empty(),
            Self::ObservationDefinition(r) => !r.contained.is_empty(),
            Self::OperationDefinition(r) => !r.contained.is_empty(),
            Self::OperationOutcome(r) => !r.contained.is_empty(),
            Self::Organization(r) => !r.contained.is_empty(),
            Self::OrganizationAffiliation(r) => !r.contained.is_empty(),
            Self::Parameters(_) => false,
            Self::Patient(r) => !r.contained.is_empty(),
            Self::PaymentNotice(r) => !r.contained.is_empty(),
            Self::PaymentReconciliation(r) => !r.contained.is_empty(),
            Self::Person(r) => !r.contained.is_empty(),
            Self::PlanDefinition(r) => !r.contained.is_empty(),
            Self::Practitioner(r) => !r.contained.is_empty(),
            Self::PractitionerRole(r) => !r.contained.is_empty(),
            Self::Procedure(r) => !r.contained.is_empty(),
            Self::Provenance(r) => !r.contained.is_empty(),
            Self::Questionnaire(r) => !r.contained.is_empty(),
            Self::QuestionnaireResponse(r) => !r.contained.is_empty(),
            Self::RelatedPerson(r) => !r.contained.is_empty(),
            Self::RequestGroup(r) => !r.contained.is_empty(),
            Self::ResearchDefinition(r) => !r.contained.is_empty(),
            Self::ResearchElementDefinition(r) => !r.contained.is_empty(),
            Self::ResearchStudy(r) => !r.contained.is_empty(),
            Self::ResearchSubject(r) => !r.contained.is_empty(),
            Self::RiskAssessment(r) => !r.contained.is_empty(),
            Self::RiskEvidenceSynthesis(r) => !r.contained.is_empty(),
            Self::Schedule(r) => !r.contained.is_empty(),
            Self::SearchParameter(r) => !r.contained.is_empty(),
            Self::ServiceRequest(r) => !r.contained.is_empty(),
            Self::Slot(r) => !r.contained.is_empty(),
            Self::Specimen(r) => !r.contained.is_empty(),
            Self::SpecimenDefinition(r) => !r.contained.is_empty(),
            Self::StructureDefinition(r) => !r.contained.is_empty(),
            Self::StructureMap(r) => !r.contained.is_empty(),
            Self::Subscription(r) => !r.contained.is_empty(),
            Self::Substance(r) => !r.contained.is_empty(),
            Self::SubstanceNucleicAcid(r) => !r.contained.is_empty(),
            Self::SubstancePolymer(r) => !r.contained.is_empty(),
            Self::SubstanceProtein(r) => !r.contained.is_empty(),
            Self::SubstanceReferenceInformation(r) => !r.contained.is_empty(),
            Self::SubstanceSourceMaterial(r) => !r.contained.is_empty(),
            Self::SubstanceSpecification(r) => !r.contained.is_empty(),
            Self::SupplyDelivery(r) => !r.contained.is_empty(),
            Self::SupplyRequest(r) => !r.contained.is_empty(),
            Self::Task(r) => !r.contained.is_empty(),
            Self::TerminologyCapabilities(r) => !r.contained.is_empty(),
            Self::TestReport(r) => !r.contained.is_empty(),
            Self::TestScript(r) => !r.contained.is_empty(),
            Self::ValueSet(r) => !r.contained.is_empty(),
            Self::VerificationResult(r) => !r.contained.is_empty(),
            Self::VisionPrescription(r) => !r.contained.is_empty(),
        }
    }

    /// The resource's `meta`, when present (`dom-4`).
    #[must_use]
    pub fn meta(&self) -> Option<&crate::r4::types::Meta> {
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
            Self::BiologicallyDerivedProduct(r) => r.meta.as_ref(),
            Self::BodyStructure(r) => r.meta.as_ref(),
            Self::Bundle(r) => r.meta.as_ref(),
            Self::CapabilityStatement(r) => r.meta.as_ref(),
            Self::CarePlan(r) => r.meta.as_ref(),
            Self::CareTeam(r) => r.meta.as_ref(),
            Self::CatalogEntry(r) => r.meta.as_ref(),
            Self::ChargeItem(r) => r.meta.as_ref(),
            Self::ChargeItemDefinition(r) => r.meta.as_ref(),
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
            Self::CoverageEligibilityRequest(r) => r.meta.as_ref(),
            Self::CoverageEligibilityResponse(r) => r.meta.as_ref(),
            Self::DetectedIssue(r) => r.meta.as_ref(),
            Self::Device(r) => r.meta.as_ref(),
            Self::DeviceDefinition(r) => r.meta.as_ref(),
            Self::DeviceMetric(r) => r.meta.as_ref(),
            Self::DeviceRequest(r) => r.meta.as_ref(),
            Self::DeviceUseStatement(r) => r.meta.as_ref(),
            Self::DiagnosticReport(r) => r.meta.as_ref(),
            Self::DocumentManifest(r) => r.meta.as_ref(),
            Self::DocumentReference(r) => r.meta.as_ref(),
            Self::EffectEvidenceSynthesis(r) => r.meta.as_ref(),
            Self::Encounter(r) => r.meta.as_ref(),
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
            Self::Goal(r) => r.meta.as_ref(),
            Self::GraphDefinition(r) => r.meta.as_ref(),
            Self::Group(r) => r.meta.as_ref(),
            Self::GuidanceResponse(r) => r.meta.as_ref(),
            Self::HealthcareService(r) => r.meta.as_ref(),
            Self::ImagingStudy(r) => r.meta.as_ref(),
            Self::Immunization(r) => r.meta.as_ref(),
            Self::ImmunizationEvaluation(r) => r.meta.as_ref(),
            Self::ImmunizationRecommendation(r) => r.meta.as_ref(),
            Self::ImplementationGuide(r) => r.meta.as_ref(),
            Self::InsurancePlan(r) => r.meta.as_ref(),
            Self::Invoice(r) => r.meta.as_ref(),
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
            Self::MedicationKnowledge(r) => r.meta.as_ref(),
            Self::MedicationRequest(r) => r.meta.as_ref(),
            Self::MedicationStatement(r) => r.meta.as_ref(),
            Self::MedicinalProduct(r) => r.meta.as_ref(),
            Self::MedicinalProductAuthorization(r) => r.meta.as_ref(),
            Self::MedicinalProductContraindication(r) => r.meta.as_ref(),
            Self::MedicinalProductIndication(r) => r.meta.as_ref(),
            Self::MedicinalProductIngredient(r) => r.meta.as_ref(),
            Self::MedicinalProductInteraction(r) => r.meta.as_ref(),
            Self::MedicinalProductManufactured(r) => r.meta.as_ref(),
            Self::MedicinalProductPackaged(r) => r.meta.as_ref(),
            Self::MedicinalProductPharmaceutical(r) => r.meta.as_ref(),
            Self::MedicinalProductUndesirableEffect(r) => r.meta.as_ref(),
            Self::MessageDefinition(r) => r.meta.as_ref(),
            Self::MessageHeader(r) => r.meta.as_ref(),
            Self::MolecularSequence(r) => r.meta.as_ref(),
            Self::NamingSystem(r) => r.meta.as_ref(),
            Self::NutritionOrder(r) => r.meta.as_ref(),
            Self::Observation(r) => r.meta.as_ref(),
            Self::ObservationDefinition(r) => r.meta.as_ref(),
            Self::OperationDefinition(r) => r.meta.as_ref(),
            Self::OperationOutcome(r) => r.meta.as_ref(),
            Self::Organization(r) => r.meta.as_ref(),
            Self::OrganizationAffiliation(r) => r.meta.as_ref(),
            Self::Parameters(r) => r.meta.as_ref(),
            Self::Patient(r) => r.meta.as_ref(),
            Self::PaymentNotice(r) => r.meta.as_ref(),
            Self::PaymentReconciliation(r) => r.meta.as_ref(),
            Self::Person(r) => r.meta.as_ref(),
            Self::PlanDefinition(r) => r.meta.as_ref(),
            Self::Practitioner(r) => r.meta.as_ref(),
            Self::PractitionerRole(r) => r.meta.as_ref(),
            Self::Procedure(r) => r.meta.as_ref(),
            Self::Provenance(r) => r.meta.as_ref(),
            Self::Questionnaire(r) => r.meta.as_ref(),
            Self::QuestionnaireResponse(r) => r.meta.as_ref(),
            Self::RelatedPerson(r) => r.meta.as_ref(),
            Self::RequestGroup(r) => r.meta.as_ref(),
            Self::ResearchDefinition(r) => r.meta.as_ref(),
            Self::ResearchElementDefinition(r) => r.meta.as_ref(),
            Self::ResearchStudy(r) => r.meta.as_ref(),
            Self::ResearchSubject(r) => r.meta.as_ref(),
            Self::RiskAssessment(r) => r.meta.as_ref(),
            Self::RiskEvidenceSynthesis(r) => r.meta.as_ref(),
            Self::Schedule(r) => r.meta.as_ref(),
            Self::SearchParameter(r) => r.meta.as_ref(),
            Self::ServiceRequest(r) => r.meta.as_ref(),
            Self::Slot(r) => r.meta.as_ref(),
            Self::Specimen(r) => r.meta.as_ref(),
            Self::SpecimenDefinition(r) => r.meta.as_ref(),
            Self::StructureDefinition(r) => r.meta.as_ref(),
            Self::StructureMap(r) => r.meta.as_ref(),
            Self::Subscription(r) => r.meta.as_ref(),
            Self::Substance(r) => r.meta.as_ref(),
            Self::SubstanceNucleicAcid(r) => r.meta.as_ref(),
            Self::SubstancePolymer(r) => r.meta.as_ref(),
            Self::SubstanceProtein(r) => r.meta.as_ref(),
            Self::SubstanceReferenceInformation(r) => r.meta.as_ref(),
            Self::SubstanceSourceMaterial(r) => r.meta.as_ref(),
            Self::SubstanceSpecification(r) => r.meta.as_ref(),
            Self::SupplyDelivery(r) => r.meta.as_ref(),
            Self::SupplyRequest(r) => r.meta.as_ref(),
            Self::Task(r) => r.meta.as_ref(),
            Self::TerminologyCapabilities(r) => r.meta.as_ref(),
            Self::TestReport(r) => r.meta.as_ref(),
            Self::TestScript(r) => r.meta.as_ref(),
            Self::ValueSet(r) => r.meta.as_ref(),
            Self::VerificationResult(r) => r.meta.as_ref(),
            Self::VisionPrescription(r) => r.meta.as_ref(),
        }
    }
}
