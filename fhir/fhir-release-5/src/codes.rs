//! FHIR R5 code systems as type-safe Rust enums.
//!
//! Each enum corresponds to a complete FHIR R5 `CodeSystem`. Variants
//! serialize to and from their canonical FHIR code strings via serde renames.
//!
//! # Examples
//!
//! ```
//! use fhir::r5::codes::AdministrativeGender;
//!
//! let code = AdministrativeGender::Male;
//! assert_eq!(::serde_json::to_value(&code).unwrap(), ::serde_json::json!("male"));
//! ```

use ::serde::{Deserialize, Serialize};

/// Indicates who is expected to pay a part of the account balance.
///
/// System: <http://hl7.org/fhir/account-aggregate>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AccountAggregate {
    /// Patient
    #[default]
    #[serde(rename = "patient")]
    Patient,
    /// Insurance
    #[serde(rename = "insurance")]
    Insurance,
    /// Total
    #[serde(rename = "total")]
    Total,
}

/// Indicates the account balance's age
///
/// System: <http://hl7.org/fhir/account-balance-term>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AccountBalanceTerm {
    /// Current
    #[default]
    #[serde(rename = "current")]
    Current,
    /// 30 day
    #[serde(rename = "30")]
    N30,
    /// 60 day
    #[serde(rename = "60")]
    N60,
    /// 90 day
    #[serde(rename = "90")]
    N90,
    /// 120 day
    #[serde(rename = "120")]
    N120,
}

/// Indicates whether the account is available to be used for billing purposes.
///
/// System: <http://hl7.org/fhir/account-billing-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AccountBillingStatus {
    /// Open
    #[default]
    #[serde(rename = "open")]
    Open,
    /// CareComplete/Not Billed
    #[serde(rename = "carecomplete-notbilled")]
    CarecompleteNotbilled,
    /// Billing
    #[serde(rename = "billing")]
    Billing,
    /// Closed-Bad Debt
    #[serde(rename = "closed-baddebt")]
    ClosedBaddebt,
    /// Closed-Voided
    #[serde(rename = "closed-voided")]
    ClosedVoided,
    /// Closed-Completed
    #[serde(rename = "closed-completed")]
    ClosedCompleted,
    /// Closed-Combined
    #[serde(rename = "closed-combined")]
    ClosedCombined,
}

/// Relationship between accounts
///
/// System: <http://hl7.org/fhir/account-relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AccountRelationship {
    /// Parent
    #[default]
    #[serde(rename = "parent")]
    Parent,
    /// Guarantor
    #[serde(rename = "guarantor")]
    Guarantor,
}

/// Indicates whether the account is available to be used.
///
/// System: <http://hl7.org/fhir/account-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AccountStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Defines behavior for an action or a group for how many times that item may
/// be repeated.
///
/// System: <http://hl7.org/fhir/action-cardinality-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionCardinalityBehavior {
    /// Single
    #[default]
    #[serde(rename = "single")]
    Single,
    /// Multiple
    #[serde(rename = "multiple")]
    Multiple,
}

/// Provides examples of actions to be performed.
///
/// System: <http://hl7.org/fhir/action-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionCode {
    /// Send a message
    #[default]
    #[serde(rename = "send-message")]
    SendMessage,
    /// Collect information
    #[serde(rename = "collect-information")]
    CollectInformation,
    /// Prescribe a medication
    #[serde(rename = "prescribe-medication")]
    PrescribeMedication,
    /// Recommend an immunization
    #[serde(rename = "recommend-immunization")]
    RecommendImmunization,
    /// Order a service
    #[serde(rename = "order-service")]
    OrderService,
    /// Propose a diagnosis
    #[serde(rename = "propose-diagnosis")]
    ProposeDiagnosis,
    /// Record a detected issue
    #[serde(rename = "record-detected-issue")]
    RecordDetectedIssue,
    /// Record an inference
    #[serde(rename = "record-inference")]
    RecordInference,
    /// Report a flag
    #[serde(rename = "report-flag")]
    ReportFlag,
}

/// Defines the kinds of conditions that can appear on actions.
///
/// System: <http://hl7.org/fhir/action-condition-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionConditionKind {
    /// Applicability
    #[default]
    #[serde(rename = "applicability")]
    Applicability,
    /// Start
    #[serde(rename = "start")]
    Start,
    /// Stop
    #[serde(rename = "stop")]
    Stop,
}

/// Defines organization behavior of a group.
///
/// System: <http://hl7.org/fhir/action-grouping-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionGroupingBehavior {
    /// Visual Group
    #[default]
    #[serde(rename = "visual-group")]
    VisualGroup,
    /// Logical Group
    #[serde(rename = "logical-group")]
    LogicalGroup,
    /// Sentence Group
    #[serde(rename = "sentence-group")]
    SentenceGroup,
}

/// The function performed by the participant for the action.
///
/// System: <http://hl7.org/fhir/action-participant-function>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionParticipantFunction {
    /// Performer
    #[default]
    #[serde(rename = "performer")]
    Performer,
    /// Author
    #[serde(rename = "author")]
    Author,
    /// Reviewer
    #[serde(rename = "reviewer")]
    Reviewer,
    /// Witness
    #[serde(rename = "witness")]
    Witness,
}

/// The type of participant for the action.
///
/// System: <http://hl7.org/fhir/action-participant-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionParticipantType {
    /// CareTeam
    #[default]
    #[serde(rename = "careteam")]
    Careteam,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Group
    #[serde(rename = "group")]
    Group,
    /// HealthcareService
    #[serde(rename = "healthcareservice")]
    Healthcareservice,
    /// Location
    #[serde(rename = "location")]
    Location,
    /// Organization
    #[serde(rename = "organization")]
    Organization,
    /// Patient
    #[serde(rename = "patient")]
    Patient,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// PractitionerRole
    #[serde(rename = "practitionerrole")]
    Practitionerrole,
    /// RelatedPerson
    #[serde(rename = "relatedperson")]
    Relatedperson,
}

/// Defines selection frequency behavior for an action or group.
///
/// System: <http://hl7.org/fhir/action-precheck-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionPrecheckBehavior {
    /// Yes
    #[default]
    #[serde(rename = "yes")]
    Yes,
    /// No
    #[serde(rename = "no")]
    No,
}

/// Provides examples of reasons for actions to be performed.
///
/// System: <http://hl7.org/fhir/action-reason-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionReasonCode {
    /// Off pathway
    #[default]
    #[serde(rename = "off-pathway")]
    OffPathway,
    /// Risk assessment
    #[serde(rename = "risk-assessment")]
    RiskAssessment,
    /// Care gap detected
    #[serde(rename = "care-gap")]
    CareGap,
    /// Drug-drug interaction
    #[serde(rename = "drug-drug-interaction")]
    DrugDrugInteraction,
    /// Quality measure
    #[serde(rename = "quality-measure")]
    QualityMeasure,
}

/// Defines the types of relationships between actions.
///
/// System: <http://hl7.org/fhir/action-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionRelationshipType {
    /// Before
    #[default]
    #[serde(rename = "before")]
    Before,
    /// Before Start
    #[serde(rename = "before-start")]
    BeforeStart,
    /// Before End
    #[serde(rename = "before-end")]
    BeforeEnd,
    /// Concurrent
    #[serde(rename = "concurrent")]
    Concurrent,
    /// Concurrent With Start
    #[serde(rename = "concurrent-with-start")]
    ConcurrentWithStart,
    /// Concurrent With End
    #[serde(rename = "concurrent-with-end")]
    ConcurrentWithEnd,
    /// After
    #[serde(rename = "after")]
    After,
    /// After Start
    #[serde(rename = "after-start")]
    AfterStart,
    /// After End
    #[serde(rename = "after-end")]
    AfterEnd,
}

/// Defines expectations around whether an action or action group is required.
///
/// System: <http://hl7.org/fhir/action-required-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionRequiredBehavior {
    /// Must
    #[default]
    #[serde(rename = "must")]
    Must,
    /// Could
    #[serde(rename = "could")]
    Could,
    /// Must Unless Documented
    #[serde(rename = "must-unless-documented")]
    MustUnlessDocumented,
}

/// Defines selection behavior of a group.
///
/// System: <http://hl7.org/fhir/action-selection-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ActionSelectionBehavior {
    /// Any
    #[default]
    #[serde(rename = "any")]
    Any,
    /// All
    #[serde(rename = "all")]
    All,
    /// All Or None
    #[serde(rename = "all-or-none")]
    AllOrNone,
    /// Exactly One
    #[serde(rename = "exactly-one")]
    ExactlyOne,
    /// At Most One
    #[serde(rename = "at-most-one")]
    AtMostOne,
    /// One Or More
    #[serde(rename = "one-or-more")]
    OneOrMore,
}

/// Additional Binding Purpose
///
/// System: <http://hl7.org/fhir/CodeSystem/additional-binding-purpose>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdditionalBindingPurpose {
    /// Maximum Binding
    #[default]
    #[serde(rename = "maximum")]
    Maximum,
    /// Minimum Binding
    #[serde(rename = "minimum")]
    Minimum,
    /// Required Binding
    #[serde(rename = "required")]
    Required,
    /// Conformance Binding
    #[serde(rename = "extensible")]
    Extensible,
    /// Candidate Binding
    #[serde(rename = "candidate")]
    Candidate,
    /// Current Binding
    #[serde(rename = "current")]
    Current,
    /// Preferred Binding
    #[serde(rename = "preferred")]
    Preferred,
    /// UI Suggested Binding
    #[serde(rename = "ui")]
    Ui,
    /// Starter Binding
    #[serde(rename = "starter")]
    Starter,
    /// Component Binding
    #[serde(rename = "component")]
    Component,
}

/// The type of an address (physical / postal).
///
/// System: <http://hl7.org/fhir/address-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AddressType {
    /// Postal
    #[default]
    #[serde(rename = "postal")]
    Postal,
    /// Physical
    #[serde(rename = "physical")]
    Physical,
    /// Postal & Physical
    #[serde(rename = "both")]
    Both,
}

/// The use of an address.
///
/// System: <http://hl7.org/fhir/address-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AddressUse {
    /// Home
    #[default]
    #[serde(rename = "home")]
    Home,
    /// Work
    #[serde(rename = "work")]
    Work,
    /// Temporary
    #[serde(rename = "temp")]
    Temp,
    /// Old / Incorrect
    #[serde(rename = "old")]
    Old,
    /// Billing
    #[serde(rename = "billing")]
    Billing,
}

/// Dose form for a medication, in the form suitable for administering to the
/// patient, after mixing, where necessary.
///
/// System: <http://hl7.org/fhir/administrable-dose-form>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdministrableDoseForm {
    /// Oral suspension
    #[default]
    #[serde(rename = "100000073362")]
    N100000073362,
    /// Oral gel
    #[serde(rename = "100000073363")]
    N100000073363,
    /// Powder for oral solution
    #[serde(rename = "100000073364")]
    N100000073364,
    /// Granules for oral solution
    #[serde(rename = "100000073365")]
    N100000073365,
    /// Lyophilisate for suspension
    #[serde(rename = "100000073367")]
    N100000073367,
    /// Powder for syrup
    #[serde(rename = "100000073368")]
    N100000073368,
    /// Soluble tablet
    #[serde(rename = "100000073369")]
    N100000073369,
    /// Herbal tea
    #[serde(rename = "100000073370")]
    N100000073370,
    /// Instant herbal tea
    #[serde(rename = "100000073371")]
    N100000073371,
    /// Granules
    #[serde(rename = "100000073372")]
    N100000073372,
    /// Gastro-resistant granules
    #[serde(rename = "100000073373")]
    N100000073373,
    /// Modified-release granules
    #[serde(rename = "100000073374")]
    N100000073374,
    /// Capsule, hard
    #[serde(rename = "100000073375")]
    N100000073375,
    /// Gastro-resistant capsule, hard
    #[serde(rename = "100000073376")]
    N100000073376,
    /// Chewable capsule, soft
    #[serde(rename = "100000073377")]
    N100000073377,
    /// Prolonged-release capsule, soft
    #[serde(rename = "100000073378")]
    N100000073378,
    /// Modified-release capsule, soft
    #[serde(rename = "100000073379")]
    N100000073379,
    /// Coated tablet
    #[serde(rename = "100000073380")]
    N100000073380,
    /// Oral drops, solution
    #[serde(rename = "100000073642")]
    N100000073642,
    /// Oral drops, suspension
    #[serde(rename = "100000073643")]
    N100000073643,
    /// Oral drops, emulsion
    #[serde(rename = "100000073644")]
    N100000073644,
    /// Oral liquid
    #[serde(rename = "100000073645")]
    N100000073645,
    /// Oral solution
    #[serde(rename = "100000073646")]
    N100000073646,
    /// Oral emulsion
    #[serde(rename = "100000073647")]
    N100000073647,
    /// Oral paste
    #[serde(rename = "100000073648")]
    N100000073648,
    /// Powder for oral suspension
    #[serde(rename = "100000073649")]
    N100000073649,
    /// Granules for oral suspension
    #[serde(rename = "100000073650")]
    N100000073650,
    /// Syrup
    #[serde(rename = "100000073652")]
    N100000073652,
    /// Granules for syrup
    #[serde(rename = "100000073653")]
    N100000073653,
    /// Dispersible tablet
    #[serde(rename = "100000073654")]
    N100000073654,
    /// Oral powder
    #[serde(rename = "100000073655")]
    N100000073655,
    /// Effervescent powder
    #[serde(rename = "100000073656")]
    N100000073656,
    /// Effervescent granules
    #[serde(rename = "100000073657")]
    N100000073657,
    /// Prolonged-release granules
    #[serde(rename = "100000073658")]
    N100000073658,
    /// Cachet
    #[serde(rename = "100000073659")]
    N100000073659,
    /// Capsule, soft
    #[serde(rename = "100000073660")]
    N100000073660,
    /// Gastro-resistant capsule, soft
    #[serde(rename = "100000073661")]
    N100000073661,
    /// Prolonged-release capsule, hard
    #[serde(rename = "100000073662")]
    N100000073662,
    /// Modified-release capsule, hard
    #[serde(rename = "100000073663")]
    N100000073663,
    /// Tablet
    #[serde(rename = "100000073664")]
    N100000073664,
    /// Film-coated tablet
    #[serde(rename = "100000073665")]
    N100000073665,
    /// Orodispersible tablet
    #[serde(rename = "100000073666")]
    N100000073666,
    /// Gastro-resistant tablet
    #[serde(rename = "100000073667")]
    N100000073667,
    /// Modified-release tablet
    #[serde(rename = "100000073668")]
    N100000073668,
    /// Medicated chewing-gum
    #[serde(rename = "100000073669")]
    N100000073669,
    /// Pillules
    #[serde(rename = "100000073670")]
    N100000073670,
    /// Pulsatile-release intraruminal device
    #[serde(rename = "100000073671")]
    N100000073671,
    /// Premix for medicated feeding stuff
    #[serde(rename = "100000073672")]
    N100000073672,
    /// Gargle
    #[serde(rename = "100000073673")]
    N100000073673,
    /// Gargle, powder for solution
    #[serde(rename = "100000073674")]
    N100000073674,
    /// Oromucosal suspension
    #[serde(rename = "100000073675")]
    N100000073675,
    /// Oromucosal spray
    #[serde(rename = "100000073676")]
    N100000073676,
    /// Mouthwash
    #[serde(rename = "100000073677")]
    N100000073677,
    /// Gingival solution
    #[serde(rename = "100000073678")]
    N100000073678,
    /// Oromucosal paste
    #[serde(rename = "100000073679")]
    N100000073679,
    /// Gingival gel
    #[serde(rename = "100000073680")]
    N100000073680,
    /// Effervescent tablet
    #[serde(rename = "100000073681")]
    N100000073681,
    /// Oral lyophilisate
    #[serde(rename = "100000073682")]
    N100000073682,
    /// Prolonged-release tablet
    #[serde(rename = "100000073683")]
    N100000073683,
    /// Chewable tablet
    #[serde(rename = "100000073684")]
    N100000073684,
    /// Oral gum
    #[serde(rename = "100000073685")]
    N100000073685,
    /// Continuous-release intraruminal device
    #[serde(rename = "100000073686")]
    N100000073686,
    /// Lick block
    #[serde(rename = "100000073687")]
    N100000073687,
    /// Medicated pellets
    #[serde(rename = "100000073688")]
    N100000073688,
    /// Concentrate for gargle
    #[serde(rename = "100000073689")]
    N100000073689,
    /// Gargle, tablet for solution
    #[serde(rename = "100000073690")]
    N100000073690,
    /// Oromucosal solution
    #[serde(rename = "100000073691")]
    N100000073691,
    /// Oromucosal drops
    #[serde(rename = "100000073692")]
    N100000073692,
    /// Sublingual spray
    #[serde(rename = "100000073693")]
    N100000073693,
    /// Mouthwash, tablet for solution
    #[serde(rename = "100000073694")]
    N100000073694,
    /// Oromucosal gel
    #[serde(rename = "100000073695")]
    N100000073695,
    /// Oromucosal cream
    #[serde(rename = "100000073696")]
    N100000073696,
    /// Gingival paste
    #[serde(rename = "100000073697")]
    N100000073697,
    /// Sublingual tablet
    #[serde(rename = "100000073698")]
    N100000073698,
    /// Buccal tablet
    #[serde(rename = "100000073699")]
    N100000073699,
    /// Compressed lozenge
    #[serde(rename = "100000073700")]
    N100000073700,
    /// Oromucosal capsule
    #[serde(rename = "100000073701")]
    N100000073701,
    /// Muco-adhesive buccal tablet
    #[serde(rename = "100000073702")]
    N100000073702,
    /// Lozenge
    #[serde(rename = "100000073703")]
    N100000073703,
    /// Pastille
    #[serde(rename = "100000073704")]
    N100000073704,
    /// Dental gel
    #[serde(rename = "100000073705")]
    N100000073705,
    /// Dental insert
    #[serde(rename = "100000073706")]
    N100000073706,
    /// Dental powder
    #[serde(rename = "100000073707")]
    N100000073707,
    /// Dental suspension
    #[serde(rename = "100000073708")]
    N100000073708,
    /// Toothpaste
    #[serde(rename = "100000073709")]
    N100000073709,
    /// Periodontal gel
    #[serde(rename = "100000073710")]
    N100000073710,
    /// Bath additive
    #[serde(rename = "100000073711")]
    N100000073711,
    /// Cream
    #[serde(rename = "100000073712")]
    N100000073712,
    /// Ointment
    #[serde(rename = "100000073713")]
    N100000073713,
    /// Medicated plaster
    #[serde(rename = "100000073714")]
    N100000073714,
    /// Shampoo
    #[serde(rename = "100000073715")]
    N100000073715,
    /// Cutaneous spray, suspension
    #[serde(rename = "100000073716")]
    N100000073716,
    /// Cutaneous liquid
    #[serde(rename = "100000073717")]
    N100000073717,
    /// Concentrate for cutaneous solution
    #[serde(rename = "100000073718")]
    N100000073718,
    /// Cutaneous emulsion
    #[serde(rename = "100000073719")]
    N100000073719,
    /// Cutaneous patch
    #[serde(rename = "100000073720")]
    N100000073720,
    /// Periodontal powder
    #[serde(rename = "100000073721")]
    N100000073721,
    /// Dental stick
    #[serde(rename = "100000073722")]
    N100000073722,
    /// Dental solution
    #[serde(rename = "100000073723")]
    N100000073723,
    /// Dental emulsion
    #[serde(rename = "100000073724")]
    N100000073724,
    /// Periodontal insert
    #[serde(rename = "100000073725")]
    N100000073725,
    /// Gel
    #[serde(rename = "100000073726")]
    N100000073726,
    /// Cutaneous paste
    #[serde(rename = "100000073727")]
    N100000073727,
    /// Cutaneous foam
    #[serde(rename = "100000073728")]
    N100000073728,
    /// Cutaneous spray, solution
    #[serde(rename = "100000073729")]
    N100000073729,
    /// Cutaneous spray, powder
    #[serde(rename = "100000073730")]
    N100000073730,
    /// Cutaneous solution
    #[serde(rename = "100000073731")]
    N100000073731,
    /// Cutaneous suspension
    #[serde(rename = "100000073732")]
    N100000073732,
    /// Cutaneous powder
    #[serde(rename = "100000073733")]
    N100000073733,
    /// Solution for iontophoresis
    #[serde(rename = "100000073734")]
    N100000073734,
    /// Collodion
    #[serde(rename = "100000073735")]
    N100000073735,
    /// Poultice
    #[serde(rename = "100000073736")]
    N100000073736,
    /// Cutaneous sponge
    #[serde(rename = "100000073737")]
    N100000073737,
    /// Collar
    #[serde(rename = "100000073738")]
    N100000073738,
    /// Ear tag
    #[serde(rename = "100000073739")]
    N100000073739,
    /// Dip suspension
    #[serde(rename = "100000073740")]
    N100000073740,
    /// Transdermal patch
    #[serde(rename = "100000073741")]
    N100000073741,
    /// Medicated nail lacquer
    #[serde(rename = "100000073742")]
    N100000073742,
    /// Cutaneous stick
    #[serde(rename = "100000073743")]
    N100000073743,
    /// Impregnated dressing
    #[serde(rename = "100000073744")]
    N100000073744,
    /// Medicated pendant
    #[serde(rename = "100000073745")]
    N100000073745,
    /// Dip solution
    #[serde(rename = "100000073746")]
    N100000073746,
    /// Dip emulsion
    #[serde(rename = "100000073747")]
    N100000073747,
    /// Concentrate for dip suspension
    #[serde(rename = "100000073748")]
    N100000073748,
    /// Powder for dip solution
    #[serde(rename = "100000073749")]
    N100000073749,
    /// Powder for suspension for fish treatment
    #[serde(rename = "100000073750")]
    N100000073750,
    /// Pour-on suspension
    #[serde(rename = "100000073751")]
    N100000073751,
    /// Spot-on solution
    #[serde(rename = "100000073752")]
    N100000073752,
    /// Spot-on emulsion
    #[serde(rename = "100000073753")]
    N100000073753,
    /// Teat dip suspension
    #[serde(rename = "100000073754")]
    N100000073754,
    /// Teat spray solution
    #[serde(rename = "100000073755")]
    N100000073755,
    /// Solution for skin-prick test
    #[serde(rename = "100000073756")]
    N100000073756,
    /// Plaster for provocation test
    #[serde(rename = "100000073757")]
    N100000073757,
    /// Eye gel
    #[serde(rename = "100000073758")]
    N100000073758,
    /// Eye drops, solution
    #[serde(rename = "100000073759")]
    N100000073759,
    /// Eye drops, suspension
    #[serde(rename = "100000073760")]
    N100000073760,
    /// Concentrate for dip solution
    #[serde(rename = "100000073761")]
    N100000073761,
    /// Concentrate for dip emulsion
    #[serde(rename = "100000073762")]
    N100000073762,
    /// Concentrate for solution for fish treatment
    #[serde(rename = "100000073763")]
    N100000073763,
    /// Pour-on solution
    #[serde(rename = "100000073764")]
    N100000073764,
    /// Pour-on emulsion
    #[serde(rename = "100000073765")]
    N100000073765,
    /// Spot-on suspension
    #[serde(rename = "100000073766")]
    N100000073766,
    /// Teat dip solution
    #[serde(rename = "100000073767")]
    N100000073767,
    /// Teat dip emulsion
    #[serde(rename = "100000073768")]
    N100000073768,
    /// Transdermal system
    #[serde(rename = "100000073769")]
    N100000073769,
    /// Solution for skin-scratch test
    #[serde(rename = "100000073770")]
    N100000073770,
    /// Eye cream
    #[serde(rename = "100000073771")]
    N100000073771,
    /// Eye ointment
    #[serde(rename = "100000073772")]
    N100000073772,
    /// Eye drops, emulsion
    #[serde(rename = "100000073773")]
    N100000073773,
    /// Eye drops, solvent for reconstitution
    #[serde(rename = "100000073775")]
    N100000073775,
    /// Eye lotion
    #[serde(rename = "100000073776")]
    N100000073776,
    /// Ophthalmic insert
    #[serde(rename = "100000073777")]
    N100000073777,
    /// Ear cream
    #[serde(rename = "100000073778")]
    N100000073778,
    /// Ear ointment
    #[serde(rename = "100000073779")]
    N100000073779,
    /// Ear drops, suspension
    #[serde(rename = "100000073780")]
    N100000073780,
    /// Eye drops, prolonged-release
    #[serde(rename = "100000073782")]
    N100000073782,
    /// Eye lotion, solvent for reconstitution
    #[serde(rename = "100000073783")]
    N100000073783,
    /// Ophthalmic strip
    #[serde(rename = "100000073784")]
    N100000073784,
    /// Ear gel
    #[serde(rename = "100000073785")]
    N100000073785,
    /// Ear drops, solution
    #[serde(rename = "100000073786")]
    N100000073786,
    /// Ear drops, emulsion
    #[serde(rename = "100000073787")]
    N100000073787,
    /// Ear powder
    #[serde(rename = "100000073788")]
    N100000073788,
    /// Ear spray, suspension
    #[serde(rename = "100000073789")]
    N100000073789,
    /// Ear wash, solution
    #[serde(rename = "100000073790")]
    N100000073790,
    /// Ear tampon
    #[serde(rename = "100000073791")]
    N100000073791,
    /// Nasal cream
    #[serde(rename = "100000073792")]
    N100000073792,
    /// Nasal gel
    #[serde(rename = "100000073793")]
    N100000073793,
    /// Nasal drops, solution
    #[serde(rename = "100000073794")]
    N100000073794,
    /// Nasal drops, emulsion
    #[serde(rename = "100000073795")]
    N100000073795,
    /// Nasal spray, solution
    #[serde(rename = "100000073796")]
    N100000073796,
    /// Nasal spray, emulsion
    #[serde(rename = "100000073797")]
    N100000073797,
    /// Nasal stick
    #[serde(rename = "100000073798")]
    N100000073798,
    /// Vaginal gel
    #[serde(rename = "100000073799")]
    N100000073799,
    /// Vaginal foam
    #[serde(rename = "100000073800")]
    N100000073800,
    /// Ear spray, solution
    #[serde(rename = "100000073802")]
    N100000073802,
    /// Ear spray, emulsion
    #[serde(rename = "100000073803")]
    N100000073803,
    /// Ear wash, emulsion
    #[serde(rename = "100000073804")]
    N100000073804,
    /// Ear stick
    #[serde(rename = "100000073805")]
    N100000073805,
    /// Nasal ointment
    #[serde(rename = "100000073806")]
    N100000073806,
    /// Nasal drops, suspension
    #[serde(rename = "100000073807")]
    N100000073807,
    /// Nasal powder
    #[serde(rename = "100000073808")]
    N100000073808,
    /// Nasal spray, suspension
    #[serde(rename = "100000073809")]
    N100000073809,
    /// Nasal wash
    #[serde(rename = "100000073810")]
    N100000073810,
    /// Vaginal cream
    #[serde(rename = "100000073811")]
    N100000073811,
    /// Vaginal ointment
    #[serde(rename = "100000073812")]
    N100000073812,
    /// Vaginal solution
    #[serde(rename = "100000073813")]
    N100000073813,
    /// Vaginal emulsion
    #[serde(rename = "100000073814")]
    N100000073814,
    /// Pessary
    #[serde(rename = "100000073815")]
    N100000073815,
    /// Vaginal capsule, soft
    #[serde(rename = "100000073816")]
    N100000073816,
    /// Effervescent vaginal tablet
    #[serde(rename = "100000073817")]
    N100000073817,
    /// Vaginal delivery system
    #[serde(rename = "100000073818")]
    N100000073818,
    /// Rectal cream
    #[serde(rename = "100000073819")]
    N100000073819,
    /// Rectal foam
    #[serde(rename = "100000073820")]
    N100000073820,
    /// Vaginal suspension
    #[serde(rename = "100000073821")]
    N100000073821,
    /// Tablet for vaginal solution
    #[serde(rename = "100000073822")]
    N100000073822,
    /// Vaginal capsule, hard
    #[serde(rename = "100000073823")]
    N100000073823,
    /// Vaginal tablet
    #[serde(rename = "100000073824")]
    N100000073824,
    /// Medicated vaginal tampon
    #[serde(rename = "100000073825")]
    N100000073825,
    /// Vaginal sponge
    #[serde(rename = "100000073826")]
    N100000073826,
    /// Rectal gel
    #[serde(rename = "100000073827")]
    N100000073827,
    /// Solution for injection
    #[serde(rename = "100000073863")]
    N100000073863,
}

/// This value set is provided as an example. The value set to instantiate this
/// attribute should be drawn from a robust terminology code system that
/// consists of or contains concepts to support the medication administration
/// process.
///
/// System: <http://hl7.org/fhir/CodeSystem/administration-subpotent-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdministrationSubpotentReason {
    /// Partial Dose
    #[default]
    #[serde(rename = "partialdose")]
    Partialdose,
    /// Vomited
    #[serde(rename = "vomited")]
    Vomited,
    /// Cold Chain Break
    #[serde(rename = "coldchainbreak")]
    Coldchainbreak,
    /// Manufacturer Recall
    #[serde(rename = "recall")]
    Recall,
    /// Adverse Storage
    #[serde(rename = "adversestorage")]
    Adversestorage,
    /// Expired Product
    #[serde(rename = "expired")]
    Expired,
}

/// The gender of a person used for administrative purposes.
///
/// System: <http://hl7.org/fhir/administrative-gender>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdministrativeGender {
    /// Male
    #[default]
    #[serde(rename = "male")]
    Male,
    /// Female
    #[serde(rename = "female")]
    Female,
    /// Other
    #[serde(rename = "other")]
    Other,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Overall nature of the adverse event, e.g. real or potential.
///
/// System: <http://hl7.org/fhir/adverse-event-actuality>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdverseEventActuality {
    /// Adverse Event
    #[default]
    #[serde(rename = "actual")]
    Actual,
    /// Potential Adverse Event
    #[serde(rename = "potential")]
    Potential,
}

/// Category of an identified substance associated with allergies or
/// intolerances.
///
/// System: <http://hl7.org/fhir/allergy-intolerance-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AllergyIntoleranceCategory {
    /// Food
    #[default]
    #[serde(rename = "food")]
    Food,
    /// Medication
    #[serde(rename = "medication")]
    Medication,
    /// Environment
    #[serde(rename = "environment")]
    Environment,
    /// Biologic
    #[serde(rename = "biologic")]
    Biologic,
}

/// Estimate of the potential clinical harm, or seriousness, of a reaction to
/// an identified substance.
///
/// System: <http://hl7.org/fhir/allergy-intolerance-criticality>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AllergyIntoleranceCriticality {
    /// Low Risk
    #[default]
    #[serde(rename = "low")]
    Low,
    /// High Risk
    #[serde(rename = "high")]
    High,
    /// Unable to Assess Risk
    #[serde(rename = "unable-to-assess")]
    UnableToAssess,
}

/// Identification of the underlying physiological mechanism for a Reaction
/// Risk.
///
/// System: <http://hl7.org/fhir/allergy-intolerance-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AllergyIntoleranceType {
    /// Allergy
    #[default]
    #[serde(rename = "allergy")]
    Allergy,
    /// Intolerance
    #[serde(rename = "intolerance")]
    Intolerance,
}

/// A tissue type of an animal.
///
/// System: <http://hl7.org/fhir/animal-tissue-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AnimalTissueType {
    /// All relevant tissues
    #[default]
    #[serde(rename = "100000072091")]
    N100000072091,
    /// Fat
    #[serde(rename = "100000072092")]
    N100000072092,
    /// Honey
    #[serde(rename = "100000072093")]
    N100000072093,
    /// Liver
    #[serde(rename = "100000072094")]
    N100000072094,
    /// Fresh Milk
    #[serde(rename = "100000072095")]
    N100000072095,
    /// Muscle and skin in natural proportions
    #[serde(rename = "100000072096")]
    N100000072096,
    /// Eggs
    #[serde(rename = "100000072104")]
    N100000072104,
    /// Skin and fat
    #[serde(rename = "100000072105")]
    N100000072105,
    /// Kidney
    #[serde(rename = "100000072106")]
    N100000072106,
    /// Meat and offal
    #[serde(rename = "100000072107")]
    N100000072107,
    /// Muscle
    #[serde(rename = "100000072108")]
    N100000072108,
    /// Unspecified
    #[serde(rename = "100000072109")]
    N100000072109,
    /// Adipose tissue
    #[serde(rename = "100000111053")]
    N100000111053,
    /// Adrenal
    #[serde(rename = "100000111054")]
    N100000111054,
    /// Blood vessels
    #[serde(rename = "100000111055")]
    N100000111055,
    /// Bone
    #[serde(rename = "100000111056")]
    N100000111056,
    /// Bone marrow
    #[serde(rename = "100000111057")]
    N100000111057,
    /// Brain
    #[serde(rename = "100000111058")]
    N100000111058,
    /// Connective tissue
    #[serde(rename = "100000111059")]
    N100000111059,
    /// Cornea
    #[serde(rename = "100000111060")]
    N100000111060,
    /// Dental pulp
    #[serde(rename = "100000111061")]
    N100000111061,
    /// Duodenum
    #[serde(rename = "100000111062")]
    N100000111062,
    /// Dura mater
    #[serde(rename = "100000111063")]
    N100000111063,
    /// Egg, embryonated
    #[serde(rename = "100000111064")]
    N100000111064,
    /// Egg
    #[serde(rename = "100000111065")]
    N100000111065,
    /// Egg white
    #[serde(rename = "100000111066")]
    N100000111066,
    /// Egg yolk
    #[serde(rename = "100000111067")]
    N100000111067,
    /// Embryos
    #[serde(rename = "100000111068")]
    N100000111068,
    /// Enteric plexuses
    #[serde(rename = "100000111069")]
    N100000111069,
    /// Esophagus
    #[serde(rename = "100000111070")]
    N100000111070,
    /// Feathers
    #[serde(rename = "100000111071")]
    N100000111071,
    /// Foetus
    #[serde(rename = "100000111072")]
    N100000111072,
    /// Fore-stomach (ruminants only)
    #[serde(rename = "100000111073")]
    N100000111073,
    /// Gingival tissue
    #[serde(rename = "100000111074")]
    N100000111074,
    /// Hair
    #[serde(rename = "100000111075")]
    N100000111075,
    /// Heart/pericardium
    #[serde(rename = "100000111076")]
    N100000111076,
    /// Hide
    #[serde(rename = "100000111077")]
    N100000111077,
    /// Hooves
    #[serde(rename = "100000111078")]
    N100000111078,
    /// Ileum
    #[serde(rename = "100000111079")]
    N100000111079,
    /// Jejunum
    #[serde(rename = "100000111080")]
    N100000111080,
    /// Kidney
    #[serde(rename = "100000111081")]
    N100000111081,
    /// Lard/lard oil
    #[serde(rename = "100000111082")]
    N100000111082,
    /// Large intestine
    #[serde(rename = "100000111083")]
    N100000111083,
    /// Liver
    #[serde(rename = "100000111084")]
    N100000111084,
    /// Lung
    #[serde(rename = "100000111085")]
    N100000111085,
    /// Lymph nodes
    #[serde(rename = "100000111086")]
    N100000111086,
    /// Mammary gland
    #[serde(rename = "100000111087")]
    N100000111087,
    /// Udder
    #[serde(rename = "100000111088")]
    N100000111088,
    /// Mammary tumour
    #[serde(rename = "100000111089")]
    N100000111089,
    /// Meat extract
    #[serde(rename = "100000111090")]
    N100000111090,
    /// Nasopharyngeal
    #[serde(rename = "100000111091")]
    N100000111091,
    /// Nictitating membrane
    #[serde(rename = "100000111092")]
    N100000111092,
    /// Nasal mucosa
    #[serde(rename = "100000111093")]
    N100000111093,
    /// Ovary
    #[serde(rename = "100000111094")]
    N100000111094,
    /// Pancreas
    #[serde(rename = "100000111095")]
    N100000111095,
    /// Peripheral nerves
    #[serde(rename = "100000111096")]
    N100000111096,
    /// Pituitary gland
    #[serde(rename = "100000111097")]
    N100000111097,
    /// Placenta
    #[serde(rename = "100000111098")]
    N100000111098,
    /// Prostate
    #[serde(rename = "100000111099")]
    N100000111099,
    /// Epididymis
    #[serde(rename = "100000111100")]
    N100000111100,
    /// Seminal vesicle
    #[serde(rename = "100000111101")]
    N100000111101,
    /// Rennet, calf
    #[serde(rename = "100000111102")]
    N100000111102,
    /// Retina
    #[serde(rename = "100000111103")]
    N100000111103,
    /// Optic nerve
    #[serde(rename = "100000111104")]
    N100000111104,
    /// Salivary gland
    #[serde(rename = "100000111105")]
    N100000111105,
    /// Shank
    #[serde(rename = "100000111106")]
    N100000111106,
    /// Skeletal muscle
    #[serde(rename = "100000111107")]
    N100000111107,
    /// Skin
    #[serde(rename = "100000111108")]
    N100000111108,
    /// Spinal ganglia
    #[serde(rename = "100000111109")]
    N100000111109,
    /// Spinal cord
    #[serde(rename = "100000111110")]
    N100000111110,
    /// Spleen
    #[serde(rename = "100000111111")]
    N100000111111,
    /// Stomach
    #[serde(rename = "100000111112")]
    N100000111112,
    /// Abomasum
    #[serde(rename = "100000111113")]
    N100000111113,
    /// Submaxillary glands
    #[serde(rename = "100000111114")]
    N100000111114,
    /// Tallow
    #[serde(rename = "100000111115")]
    N100000111115,
    /// Tendon
    #[serde(rename = "100000111116")]
    N100000111116,
    /// Testis
    #[serde(rename = "100000111117")]
    N100000111117,
    /// Thymus
    #[serde(rename = "100000111118")]
    N100000111118,
    /// Thyroid gland
    #[serde(rename = "100000111119")]
    N100000111119,
    /// Tongue
    #[serde(rename = "100000111120")]
    N100000111120,
    /// Tonsil
    #[serde(rename = "100000111121")]
    N100000111121,
    /// Trachea
    #[serde(rename = "100000111122")]
    N100000111122,
    /// Trigeminal ganglia
    #[serde(rename = "100000111123")]
    N100000111123,
    /// Tripe
    #[serde(rename = "100000111124")]
    N100000111124,
    /// Uterus (Non-gravid)
    #[serde(rename = "100000111125")]
    N100000111125,
    /// Wool
    #[serde(rename = "100000111126")]
    N100000111126,
    /// Ascites fluid
    #[serde(rename = "100000111127")]
    N100000111127,
    /// Bile
    #[serde(rename = "100000111128")]
    N100000111128,
    /// Blood1
    #[serde(rename = "100000111129")]
    N100000111129,
    /// Blood, foetal
    #[serde(rename = "100000111130")]
    N100000111130,
    /// Colostrum
    #[serde(rename = "100000111131")]
    N100000111131,
    /// Cord blood
    #[serde(rename = "100000111132")]
    N100000111132,
    /// CSF
    #[serde(rename = "100000111133")]
    N100000111133,
    /// Faeces
    #[serde(rename = "100000111134")]
    N100000111134,
    /// Milk
    #[serde(rename = "100000111135")]
    N100000111135,
    /// Nasal mucus
    #[serde(rename = "100000111136")]
    N100000111136,
    /// Placenta fluids
    #[serde(rename = "100000111137")]
    N100000111137,
    /// Plasma
    #[serde(rename = "100000111138")]
    N100000111138,
    /// Saliva
    #[serde(rename = "100000111139")]
    N100000111139,
    /// Secretion from bees
    #[serde(rename = "100000111140")]
    N100000111140,
    /// Semen
    #[serde(rename = "100000111141")]
    N100000111141,
    /// Serum, calf
    #[serde(rename = "100000111142")]
    N100000111142,
    /// Serum, donor adult bovine
    #[serde(rename = "100000111143")]
    N100000111143,
    /// Serum, donor calf
    #[serde(rename = "100000111144")]
    N100000111144,
    /// Serum, foetal bovine
    #[serde(rename = "100000111145")]
    N100000111145,
    /// Serum, newborn calf
    #[serde(rename = "100000111146")]
    N100000111146,
    /// Serum/plasma derivate, adult bovine
    #[serde(rename = "100000111147")]
    N100000111147,
    /// Serum/plasma, adult bovine
    #[serde(rename = "100000111148")]
    N100000111148,
    /// Sweat
    #[serde(rename = "100000111149")]
    N100000111149,
    /// Tears
    #[serde(rename = "100000111150")]
    N100000111150,
    /// Urine
    #[serde(rename = "100000111151")]
    N100000111151,
    /// Venom
    #[serde(rename = "100000111152")]
    N100000111152,
    /// Whey
    #[serde(rename = "100000111153")]
    N100000111153,
    /// Casein
    #[serde(rename = "100000111154")]
    N100000111154,
    /// Fermentation products
    #[serde(rename = "100000111155")]
    N100000111155,
    /// Gelatin
    #[serde(rename = "100000111156")]
    N100000111156,
    /// Lactose
    #[serde(rename = "100000111157")]
    N100000111157,
    /// Protein
    #[serde(rename = "100000111158")]
    N100000111158,
    /// Insulin
    #[serde(rename = "100000111159")]
    N100000111159,
    /// Collagen
    #[serde(rename = "100000111160")]
    N100000111160,
    /// Animal Charcoal
    #[serde(rename = "100000111161")]
    N100000111161,
    /// Peptones
    #[serde(rename = "100000111162")]
    N100000111162,
    /// Fatty acids
    #[serde(rename = "100000111163")]
    N100000111163,
    /// Glycerol
    #[serde(rename = "100000111164")]
    N100000111164,
    /// Not applicable
    #[serde(rename = "100000125717")]
    N100000125717,
    /// Meat and offal, milk
    #[serde(rename = "100000136180")]
    N100000136180,
    /// Agar blood
    #[serde(rename = "100000136181")]
    N100000136181,
    /// Casamino acid
    #[serde(rename = "100000136182")]
    N100000136182,
    /// Casein, hydrolysate
    #[serde(rename = "100000136183")]
    N100000136183,
    /// Casein, pancreatic digest
    #[serde(rename = "100000136184")]
    N100000136184,
    /// Casein, peptides N3
    #[serde(rename = "100000136185")]
    N100000136185,
    /// Cells
    #[serde(rename = "100000136186")]
    N100000136186,
    /// Cells, BHK21
    #[serde(rename = "100000136187")]
    N100000136187,
    /// Cells, CHO
    #[serde(rename = "100000136188")]
    N100000136188,
    /// Cells, CRFK
    #[serde(rename = "100000136189")]
    N100000136189,
    /// Cells, embryo SPF
    #[serde(rename = "100000136190")]
    N100000136190,
    /// Cells, IRC5
    #[serde(rename = "100000136191")]
    N100000136191,
    /// Cells, kidney
    #[serde(rename = "100000136192")]
    N100000136192,
    /// Cells, MDCK
    #[serde(rename = "100000136193")]
    N100000136193,
    /// Cells, red blood
    #[serde(rename = "100000136194")]
    N100000136194,
    /// Collagen, hydrolysate
    #[serde(rename = "100000136195")]
    N100000136195,
    /// Cholesterol
    #[serde(rename = "100000136196")]
    N100000136196,
    /// Egg, SPF embryonated
    #[serde(rename = "100000136197")]
    N100000136197,
    /// Enzyme
    #[serde(rename = "100000136198")]
    N100000136198,
    /// Enzyme, pancreatic enzymes
    #[serde(rename = "100000136199")]
    N100000136199,
    /// Enzyme, pancreatin 6NF
    #[serde(rename = "100000136200")]
    N100000136200,
    /// Enzyme, pepsin
    #[serde(rename = "100000136201")]
    N100000136201,
    /// Enzyme, pronase
    #[serde(rename = "100000136202")]
    N100000136202,
    /// Enzyme, trypsin
    #[serde(rename = "100000136203")]
    N100000136203,
    /// Heart, digest
    #[serde(rename = "100000136204")]
    N100000136204,
    /// Heart, extract
    #[serde(rename = "100000136205")]
    N100000136205,
    /// Intestinal mucosae
    #[serde(rename = "100000136206")]
    N100000136206,
    /// Lactalbumin hydrolysate
    #[serde(rename = "100000136207")]
    N100000136207,
    /// Liver, digest
    #[serde(rename = "100000136208")]
    N100000136208,
    /// Lymphocytes
    #[serde(rename = "100000136209")]
    N100000136209,
    /// Meat
    #[serde(rename = "100000136210")]
    N100000136210,
    /// Meat, enzymic hydrolysate
    #[serde(rename = "100000136211")]
    N100000136211,
    /// Medium, cooked meat
    #[serde(rename = "100000136212")]
    N100000136212,
    /// Medium, F10-199 medium
    #[serde(rename = "100000136213")]
    N100000136213,
    /// Medium, FMD culture medium
    #[serde(rename = "100000136214")]
    N100000136214,
    /// Medium, Glasgow MEM culture
    #[serde(rename = "100000136215")]
    N100000136215,
    /// Medium, LB Agar Lennox
    #[serde(rename = "100000136216")]
    N100000136216,
    /// Medium, LB Broth Lennox
    #[serde(rename = "100000136217")]
    N100000136217,
    /// Medium, modified thioglycolate medium
    #[serde(rename = "100000136218")]
    N100000136218,
    /// Medium, trypticase soy broth
    #[serde(rename = "100000136219")]
    N100000136219,
    /// Medium, tryptose phosphate broth
    #[serde(rename = "100000136220")]
    N100000136220,
    /// Milk, skimmed
    #[serde(rename = "100000136221")]
    N100000136221,
    /// Pancreas, extract
    #[serde(rename = "100000136222")]
    N100000136222,
    /// Peptones, casein hydrochloric peptone
    #[serde(rename = "100000136223")]
    N100000136223,
    /// Peptones, casein tryptic peptone
    #[serde(rename = "100000136224")]
    N100000136224,
    /// Pituitary extract
    #[serde(rename = "100000136225")]
    N100000136225,
    /// Rennet
    #[serde(rename = "100000136226")]
    N100000136226,
    /// Medium, nutrient broth
    #[serde(rename = "100000136227")]
    N100000136227,
    /// Medium, NZ-Amine
    #[serde(rename = "100000136228")]
    N100000136228,
    /// Medium, thioglycolate medium
    #[serde(rename = "100000136229")]
    N100000136229,
    /// Peptones, proteose peptone
    #[serde(rename = "100000136230")]
    N100000136230,
    /// Serum
    #[serde(rename = "100000136231")]
    N100000136231,
    /// Serum, albumin
    #[serde(rename = "100000136232")]
    N100000136232,
    /// Serum, Iron fortified calf
    #[serde(rename = "100000136233")]
    N100000136233,
    /// Skin, connective tissue and bone
    #[serde(rename = "100000136234")]
    N100000136234,
    /// Sperm
    #[serde(rename = "100000136235")]
    N100000136235,
    /// Tryptone
    #[serde(rename = "100000136236")]
    N100000136236,
    /// Meat, extract desiccated
    #[serde(rename = "100000136237")]
    N100000136237,
    /// Stomach mucosa
    #[serde(rename = "100000136247")]
    N100000136247,
    /// Transferin
    #[serde(rename = "100000136248")]
    N100000136248,
    /// Non-neural
    #[serde(rename = "100000136554")]
    N100000136554,
    /// Not specified
    #[serde(rename = "100000136555")]
    N100000136555,
    /// Organ tissue
    #[serde(rename = "100000136556")]
    N100000136556,
    /// Skin and fat in natural proportions
    #[serde(rename = "100000142485")]
    N100000142485,
}

/// The free/busy status of an appointment.
///
/// System: <http://hl7.org/fhir/appointmentstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Appointmentstatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
    /// Booked
    #[serde(rename = "booked")]
    Booked,
    /// Arrived
    #[serde(rename = "arrived")]
    Arrived,
    /// Fulfilled
    #[serde(rename = "fulfilled")]
    Fulfilled,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// No Show
    #[serde(rename = "noshow")]
    Noshow,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Checked In
    #[serde(rename = "checked-in")]
    CheckedIn,
    /// Waitlisted
    #[serde(rename = "waitlist")]
    Waitlist,
}

/// Artifact Contribution Instance Type
///
/// System: <http://hl7.org/fhir/artifact-contribution-instance-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactContributionInstanceType {
    /// Reviewed
    #[default]
    #[serde(rename = "reviewed")]
    Reviewed,
    /// Approved
    #[serde(rename = "approved")]
    Approved,
    /// Edited
    #[serde(rename = "edited")]
    Edited,
}

/// Citation contribution.
///
/// System: <http://hl7.org/fhir/artifact-contribution-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactContributionType {
    /// Conceptualization
    #[default]
    #[serde(rename = "conceptualization")]
    Conceptualization,
    /// Data curation
    #[serde(rename = "data-curation")]
    DataCuration,
    /// Formal analysis
    #[serde(rename = "formal-analysis")]
    FormalAnalysis,
    /// Funding acquisition
    #[serde(rename = "funding-acquisition")]
    FundingAcquisition,
    /// Investigation
    #[serde(rename = "investigation")]
    Investigation,
    /// Methodology
    #[serde(rename = "methodology")]
    Methodology,
    /// Project administration
    #[serde(rename = "project-administration")]
    ProjectAdministration,
    /// Resources
    #[serde(rename = "resources")]
    Resources,
    /// Software
    #[serde(rename = "software")]
    Software,
    /// Supervision
    #[serde(rename = "supervision")]
    Supervision,
    /// Validation
    #[serde(rename = "validation")]
    Validation,
    /// Visualization
    #[serde(rename = "visualization")]
    Visualization,
    /// Writing - original draft
    #[serde(rename = "writing-original-draft")]
    WritingOriginalDraft,
    /// Writing - review & editing
    #[serde(rename = "writing-review-editing")]
    WritingReviewEditing,
}

/// Code the reason for different URLs, eg abstract and full-text.
///
/// System: <http://hl7.org/fhir/artifact-url-classifier>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactUrlClassifier {
    /// Abstract
    #[default]
    #[serde(rename = "abstract")]
    Abstract,
    /// Full-Text
    #[serde(rename = "full-text")]
    FullText,
    /// Supplement
    #[serde(rename = "supplement")]
    Supplement,
    /// Webpage
    #[serde(rename = "webpage")]
    Webpage,
    /// File directory
    #[serde(rename = "file-directory")]
    FileDirectory,
    /// Code repository
    #[serde(rename = "code-repository")]
    CodeRepository,
    /// Restricted
    #[serde(rename = "restricted")]
    Restricted,
    /// Compressed file
    #[serde(rename = "compressed-file")]
    CompressedFile,
    /// DOI Based
    #[serde(rename = "doi-based")]
    DoiBased,
    /// PDF
    #[serde(rename = "pdf")]
    Pdf,
    /// JSON
    #[serde(rename = "json")]
    Json,
    /// XML
    #[serde(rename = "xml")]
    Xml,
    /// Version Specific
    #[serde(rename = "version-specific")]
    VersionSpecific,
    /// Computable resource
    #[serde(rename = "computable-resource")]
    ComputableResource,
    /// Not Specified
    #[serde(rename = "not-specified")]
    NotSpecified,
}

/// Possible values for the disposition of a comment or change request,
/// typically used for comments and change requests, to indicate the
/// disposition of the responsible party towards the changes suggested by the
/// comment or change request.
///
/// System: <http://hl7.org/fhir/artifactassessment-disposition>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactassessmentDisposition {
    /// Unresolved
    #[default]
    #[serde(rename = "unresolved")]
    Unresolved,
    /// Not Persuasive
    #[serde(rename = "not-persuasive")]
    NotPersuasive,
    /// Persuasive
    #[serde(rename = "persuasive")]
    Persuasive,
    /// Persuasive with Modification
    #[serde(rename = "persuasive-with-modification")]
    PersuasiveWithModification,
    /// Not Persuasive with Modification
    #[serde(rename = "not-persuasive-with-modification")]
    NotPersuasiveWithModification,
}

/// The type of information contained in a component of an artifact assessment.
///
/// System: <http://hl7.org/fhir/artifactassessment-information-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactassessmentInformationType {
    /// Comment
    #[default]
    #[serde(rename = "comment")]
    Comment,
    /// Classifier
    #[serde(rename = "classifier")]
    Classifier,
    /// Rating
    #[serde(rename = "rating")]
    Rating,
    /// Container
    #[serde(rename = "container")]
    Container,
    /// Response
    #[serde(rename = "response")]
    Response,
    /// Change Request
    #[serde(rename = "change-request")]
    ChangeRequest,
}

/// Possible values for the workflow status of the comment or assessment,
/// typically used to coordinate workflow around the process of accepting and
/// rejecting changes and comments on the artifact.
///
/// System: <http://hl7.org/fhir/artifactassessment-workflow-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactassessmentWorkflowStatus {
    /// Submitted
    #[default]
    #[serde(rename = "submitted")]
    Submitted,
    /// Triaged
    #[serde(rename = "triaged")]
    Triaged,
    /// Waiting for Input
    #[serde(rename = "waiting-for-input")]
    WaitingForInput,
    /// Resolved - No Change
    #[serde(rename = "resolved-no-change")]
    ResolvedNoChange,
    /// Resolved - Change Required
    #[serde(rename = "resolved-change-required")]
    ResolvedChangeRequired,
    /// Deferred
    #[serde(rename = "deferred")]
    Deferred,
    /// Duplicate
    #[serde(rename = "duplicate")]
    Duplicate,
    /// Applied
    #[serde(rename = "applied")]
    Applied,
    /// Published
    #[serde(rename = "published")]
    Published,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The type of direction to use for assertion.
///
/// System: <http://hl7.org/fhir/assert-direction-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssertDirectionCodes {
    /// response
    #[default]
    #[serde(rename = "response")]
    Response,
    /// request
    #[serde(rename = "request")]
    Request,
}

/// The type of manual completion to use for assertion.
///
/// System: <http://hl7.org/fhir/assert-manual-completion-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssertManualCompletionCodes {
    /// Fail
    #[default]
    #[serde(rename = "fail")]
    Fail,
    /// Pass
    #[serde(rename = "pass")]
    Pass,
    /// Skip
    #[serde(rename = "skip")]
    Skip,
    /// Stop
    #[serde(rename = "stop")]
    Stop,
}

/// The type of operator to use for assertion.
///
/// System: <http://hl7.org/fhir/assert-operator-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssertOperatorCodes {
    /// equals
    #[default]
    #[serde(rename = "equals")]
    Equals,
    /// notEquals
    #[serde(rename = "notEquals")]
    NotEquals,
    /// in
    #[serde(rename = "in")]
    In,
    /// notIn
    #[serde(rename = "notIn")]
    NotIn,
    /// greaterThan
    #[serde(rename = "greaterThan")]
    GreaterThan,
    /// lessThan
    #[serde(rename = "lessThan")]
    LessThan,
    /// empty
    #[serde(rename = "empty")]
    Empty,
    /// notEmpty
    #[serde(rename = "notEmpty")]
    NotEmpty,
    /// contains
    #[serde(rename = "contains")]
    Contains,
    /// notContains
    #[serde(rename = "notContains")]
    NotContains,
    /// evaluate
    #[serde(rename = "eval")]
    Eval,
    /// manualEvaluate
    #[serde(rename = "manualEval")]
    ManualEval,
}

/// The type of response code to use for assertion.
///
/// System: <http://hl7.org/fhir/assert-response-code-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssertResponseCodeTypes {
    /// Continue
    #[default]
    #[serde(rename = "continue")]
    Continue,
    /// Switching Protocols
    #[serde(rename = "switchingProtocols")]
    SwitchingProtocols,
    /// OK
    #[serde(rename = "okay")]
    Okay,
    /// Created
    #[serde(rename = "created")]
    Created,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Non-Authoritative Information
    #[serde(rename = "nonAuthoritativeInformation")]
    NonAuthoritativeInformation,
    /// No Content
    #[serde(rename = "noContent")]
    NoContent,
    /// Reset Content
    #[serde(rename = "resetContent")]
    ResetContent,
    /// Partial Content
    #[serde(rename = "partialContent")]
    PartialContent,
    /// Multiple Choices
    #[serde(rename = "multipleChoices")]
    MultipleChoices,
    /// Moved Permanently
    #[serde(rename = "movedPermanently")]
    MovedPermanently,
    /// Found
    #[serde(rename = "found")]
    Found,
    /// See Other
    #[serde(rename = "seeOther")]
    SeeOther,
    /// Not Modified
    #[serde(rename = "notModified")]
    NotModified,
    /// Use Proxy
    #[serde(rename = "useProxy")]
    UseProxy,
    /// Temporary Redirect
    #[serde(rename = "temporaryRedirect")]
    TemporaryRedirect,
    /// Permanent Redirect
    #[serde(rename = "permanentRedirect")]
    PermanentRedirect,
    /// Bad Request
    #[serde(rename = "badRequest")]
    BadRequest,
    /// Unauthorized
    #[serde(rename = "unauthorized")]
    Unauthorized,
    /// Payment Required
    #[serde(rename = "paymentRequired")]
    PaymentRequired,
    /// Forbidden
    #[serde(rename = "forbidden")]
    Forbidden,
    /// Not Found
    #[serde(rename = "notFound")]
    NotFound,
    /// Method Not Allowed
    #[serde(rename = "methodNotAllowed")]
    MethodNotAllowed,
    /// Not Acceptable
    #[serde(rename = "notAcceptable")]
    NotAcceptable,
    /// Proxy Authentication Required
    #[serde(rename = "proxyAuthenticationRequired")]
    ProxyAuthenticationRequired,
    /// Request Timeout
    #[serde(rename = "requestTimeout")]
    RequestTimeout,
    /// Conflict
    #[serde(rename = "conflict")]
    Conflict,
    /// Gone
    #[serde(rename = "gone")]
    Gone,
    /// Length Required
    #[serde(rename = "lengthRequired")]
    LengthRequired,
    /// Precondition Failed
    #[serde(rename = "preconditionFailed")]
    PreconditionFailed,
    /// Content Too Large
    #[serde(rename = "contentTooLarge")]
    ContentTooLarge,
    /// URI Too Long
    #[serde(rename = "uriTooLong")]
    UriTooLong,
    /// Unsupported Media Type
    #[serde(rename = "unsupportedMediaType")]
    UnsupportedMediaType,
    /// Range Not Satisfiable
    #[serde(rename = "rangeNotSatisfiable")]
    RangeNotSatisfiable,
    /// Expectation Failed
    #[serde(rename = "expectationFailed")]
    ExpectationFailed,
    /// Misdirected Request
    #[serde(rename = "misdirectedRequest")]
    MisdirectedRequest,
    /// Unprocessable Content
    #[serde(rename = "unprocessableContent")]
    UnprocessableContent,
    /// Upgrade Required
    #[serde(rename = "upgradeRequired")]
    UpgradeRequired,
    /// Internal Server Error
    #[serde(rename = "internalServerError")]
    InternalServerError,
    /// Not Implemented
    #[serde(rename = "notImplemented")]
    NotImplemented,
    /// Bad Gateway
    #[serde(rename = "badGateway")]
    BadGateway,
    /// Service Unavailable
    #[serde(rename = "serviceUnavailable")]
    ServiceUnavailable,
    /// Gateway Timeout
    #[serde(rename = "gatewayTimeout")]
    GatewayTimeout,
    /// HTTP Version Not Supported
    #[serde(rename = "httpVersionNotSupported")]
    HttpVersionNotSupported,
}

/// This CodeSystem contains FHIR-defined contract asset availability types.
///
/// System: <http://hl7.org/fhir/asset-availability>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssetAvailability {
    /// Lease
    #[default]
    #[serde(rename = "lease")]
    Lease,
}

/// Indicator for type of action performed during the event that generated the
/// event.
///
/// System: <http://hl7.org/fhir/audit-event-action>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AuditEventAction {
    /// Create
    #[default]
    #[serde(rename = "C")]
    C,
    /// Read
    #[serde(rename = "R")]
    R,
    /// Update
    #[serde(rename = "U")]
    U,
    /// Delete
    #[serde(rename = "D")]
    D,
    /// Execute
    #[serde(rename = "E")]
    E,
}

/// The severity of the audit entry.
///
/// System: <http://hl7.org/fhir/audit-event-severity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AuditEventSeverity {
    /// Emergency
    #[default]
    #[serde(rename = "emergency")]
    Emergency,
    /// Alert
    #[serde(rename = "alert")]
    Alert,
    /// Critical
    #[serde(rename = "critical")]
    Critical,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Notice
    #[serde(rename = "notice")]
    Notice,
    /// Informational
    #[serde(rename = "informational")]
    Informational,
    /// Debug
    #[serde(rename = "debug")]
    Debug,
}

/// Indication of the degree of conformance expectations associated with a
/// binding.
///
/// System: <http://hl7.org/fhir/binding-strength>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BindingStrength {
    /// Required
    #[default]
    #[serde(rename = "required")]
    Required,
    /// Extensible
    #[serde(rename = "extensible")]
    Extensible,
    /// Preferred
    #[serde(rename = "preferred")]
    Preferred,
    /// Example
    #[serde(rename = "example")]
    Example,
}

/// Biologically Derived Product Status.
///
/// System: <http://hl7.org/fhir/biologicallyderived-product-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedProductStatus {
    /// Available
    #[default]
    #[serde(rename = "available")]
    Available,
    /// Unavailable
    #[serde(rename = "unavailable")]
    Unavailable,
}

/// This code system is a subset of ISBT 128 Product Description Codes©
/// published by ICCBBA as a part of the ISBT 128 standard. These codes support
/// characterization and classification of medical products of human origin
/// inclusive of processing conditions such as additives, volumes and handling
/// conditions.
///
/// System: <http://hl7.org/fhir/biologicallyderived-productcodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedProductcodes {
    /// RED BLOOD CELLS|CPD>AS5/450mL/refg|Irr|ResLeu
    #[default]
    #[serde(rename = "e0398")]
    E0398,
    /// HPC, APHERESIS/Citrate/XX/refg/Mobilized
    #[serde(rename = "s1128")]
    S1128,
    /// HPC, APHERESIS|NS/XX/<=-120C|10% DMSO|Cryopreserved|Mobilized
    #[serde(rename = "s1194")]
    S1194,
    /// HPC, APHERESIS|NS/XX/<=-120C|5% DMSO|Cryopreserved|Mobilized
    #[serde(rename = "s1195")]
    S1195,
    /// HPC, APHERESIS|None/XX/refg|3rd Party Comp:Yes|Other
    /// Additives:Yes|Mobilized|CD34 enriched
    #[serde(rename = "s1310")]
    S1310,
    /// HPC, MARROW|NS/XX/rt|Plasma reduced
    #[serde(rename = "s1398")]
    S1398,
    /// HPC, MARROW|NS/XX/<=-150C|10% DMSO|3rd Party Comp:Yes|Cryopreserved|RBC
    /// reduced
    #[serde(rename = "s2598")]
    S2598,
    /// Apheresis RED BLOOD CELLS|ACD-A/XX/refg|Irradiated|1st container
    #[serde(rename = "e4377")]
    E4377,
    /// BONE, FEMUR|Frozen|Right|Radiation sterilization
    #[serde(rename = "t1396")]
    T1396,
}

/// Biologically derived product dispense - match status
///
/// System: <http://hl7.org/fhir/CodeSystem/biologicallyderivedproductdispense-match-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedproductdispenseMatchStatus {
    /// Crossmatched
    #[default]
    #[serde(rename = "crossmatched")]
    Crossmatched,
    /// Selected
    #[serde(rename = "selected")]
    Selected,
    /// Unmatched
    #[serde(rename = "unmatched")]
    Unmatched,
    /// Least incompatible
    #[serde(rename = "least-incompatible")]
    LeastIncompatible,
}

/// Biologically derived product dispense - origin relationship
///
/// System: <http://hl7.org/fhir/CodeSystem/biologicallyderivedproductdispense-origin-relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedproductdispenseOriginRelationship {
    /// Autologous
    #[default]
    #[serde(rename = "autologous")]
    Autologous,
    /// Related
    #[serde(rename = "related")]
    Related,
    /// Directed
    #[serde(rename = "directed")]
    Directed,
    /// Allogeneic
    #[serde(rename = "allogeneic")]
    Allogeneic,
    /// Xenogenic
    #[serde(rename = "xenogenic")]
    Xenogenic,
}

/// Biologically derived product dispense - performer function
///
/// System: <http://hl7.org/fhir/CodeSystem/biologicallyderivedproductdispense-performer-function>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedproductdispensePerformerFunction {
    /// Group and Type
    #[default]
    #[serde(rename = "group-and-type")]
    GroupAndType,
    /// Antibody Screen
    #[serde(rename = "antibody-screen")]
    AntibodyScreen,
    /// Antibody Identification
    #[serde(rename = "antibody-identification")]
    AntibodyIdentification,
    /// Crossmatch
    #[serde(rename = "crossmatch")]
    Crossmatch,
    /// Release
    #[serde(rename = "release")]
    Release,
    /// Transport
    #[serde(rename = "transport")]
    Transport,
    /// Receipt
    #[serde(rename = "receipt")]
    Receipt,
}

/// BiologicallyDerivedProductDispense Status Codes
///
/// System: <http://hl7.org/fhir/biologicallyderivedproductdispense-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BiologicallyderivedproductdispenseStatus {
    /// Preparation
    #[default]
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Allocated
    #[serde(rename = "allocated")]
    Allocated,
    /// Issued
    #[serde(rename = "issued")]
    Issued,
    /// Unfulfilled
    #[serde(rename = "unfulfilled")]
    Unfulfilled,
    /// Returned
    #[serde(rename = "returned")]
    Returned,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Indicates the purpose of a bundle - how it is intended to be used.
///
/// System: <http://hl7.org/fhir/bundle-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BundleType {
    /// Document
    #[default]
    #[serde(rename = "document")]
    Document,
    /// Message
    #[serde(rename = "message")]
    Message,
    /// Transaction
    #[serde(rename = "transaction")]
    Transaction,
    /// Transaction Response
    #[serde(rename = "transaction-response")]
    TransactionResponse,
    /// Batch
    #[serde(rename = "batch")]
    Batch,
    /// Batch Response
    #[serde(rename = "batch-response")]
    BatchResponse,
    /// History List
    #[serde(rename = "history")]
    History,
    /// Search Results
    #[serde(rename = "searchset")]
    Searchset,
    /// Collection
    #[serde(rename = "collection")]
    Collection,
    /// Subscription Notification
    #[serde(rename = "subscription-notification")]
    SubscriptionNotification,
}

/// How a capability statement is intended to be used.
///
/// System: <http://hl7.org/fhir/capability-statement-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CapabilityStatementKind {
    /// Instance
    #[default]
    #[serde(rename = "instance")]
    Instance,
    /// Capability
    #[serde(rename = "capability")]
    Capability,
    /// Requirements
    #[serde(rename = "requirements")]
    Requirements,
}

/// Indicates the status of the care team.
///
/// System: <http://hl7.org/fhir/care-team-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CareTeamStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// CatalogType
///
/// System: <http://hl7.org/fhir/catalogType>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CatalogType {
    /// Medication Catalog
    #[default]
    #[serde(rename = "medication")]
    Medication,
    /// Device Catalog
    #[serde(rename = "device")]
    Device,
    /// Protocol List
    #[serde(rename = "protocol")]
    Protocol,
}

/// The assessment of quality, confidence, or certainty.
///
/// System: <http://hl7.org/fhir/certainty-rating>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CertaintyRating {
    /// High quality
    #[default]
    #[serde(rename = "high")]
    High,
    /// Moderate quality
    #[serde(rename = "moderate")]
    Moderate,
    /// Low quality
    #[serde(rename = "low")]
    Low,
    /// Very low quality
    #[serde(rename = "very-low")]
    VeryLow,
    /// no serious concern
    #[serde(rename = "no-concern")]
    NoConcern,
    /// serious concern
    #[serde(rename = "serious-concern")]
    SeriousConcern,
    /// very serious concern
    #[serde(rename = "very-serious-concern")]
    VerySeriousConcern,
    /// extremely serious concern
    #[serde(rename = "extremely-serious-concern")]
    ExtremelySeriousConcern,
    /// present
    #[serde(rename = "present")]
    Present,
    /// absent
    #[serde(rename = "absent")]
    Absent,
    /// no change to rating
    #[serde(rename = "no-change")]
    NoChange,
    /// reduce rating: -1
    #[serde(rename = "downcode1")]
    Downcode1,
    /// reduce rating: -2
    #[serde(rename = "downcode2")]
    Downcode2,
    /// reduce rating: -3
    #[serde(rename = "downcode3")]
    Downcode3,
    /// increase rating: +1
    #[serde(rename = "upcode1")]
    Upcode1,
    /// increase rating: +2
    #[serde(rename = "upcode2")]
    Upcode2,
}

/// The aspect of quality, confidence, or certainty.
///
/// System: <http://hl7.org/fhir/certainty-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CertaintyType {
    /// Overall certainty
    #[default]
    #[serde(rename = "Overall")]
    Overall,
    /// Risk of bias
    #[serde(rename = "RiskOfBias")]
    RiskOfBias,
    /// Inconsistency
    #[serde(rename = "Inconsistency")]
    Inconsistency,
    /// Indirectness
    #[serde(rename = "Indirectness")]
    Indirectness,
    /// Imprecision
    #[serde(rename = "Imprecision")]
    Imprecision,
    /// Publication bias
    #[serde(rename = "PublicationBias")]
    PublicationBias,
    /// Dose response gradient
    #[serde(rename = "DoseResponseGradient")]
    DoseResponseGradient,
    /// Plausible confounding
    #[serde(rename = "PlausibleConfounding")]
    PlausibleConfounding,
    /// Large effect
    #[serde(rename = "LargeEffect")]
    LargeEffect,
}

/// Logical grouping of characteristics.
///
/// System: <http://hl7.org/fhir/characteristic-combination>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CharacteristicCombination {
    /// All of
    #[default]
    #[serde(rename = "all-of")]
    AllOf,
    /// Any of
    #[serde(rename = "any-of")]
    AnyOf,
    /// At least
    #[serde(rename = "at-least")]
    AtLeast,
    /// At most
    #[serde(rename = "at-most")]
    AtMost,
    /// Statistical
    #[serde(rename = "statistical")]
    Statistical,
    /// Net effect
    #[serde(rename = "net-effect")]
    NetEffect,
    /// Dataset
    #[serde(rename = "dataset")]
    Dataset,
}

/// Reference point for characteristic.valueQuantity.
///
/// System: <http://hl7.org/fhir/characteristic-offset>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CharacteristicOffset {
    /// Upper Normal Limit
    #[default]
    #[serde(rename = "UNL")]
    Unl,
    /// Lower Normal Limit
    #[serde(rename = "LNL")]
    Lnl,
}

/// Codes identifying the lifecycle stage of a ChargeItem.
///
/// System: <http://hl7.org/fhir/chargeitem-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ChargeitemStatus {
    /// Planned
    #[default]
    #[serde(rename = "planned")]
    Planned,
    /// Billable
    #[serde(rename = "billable")]
    Billable,
    /// Not billable
    #[serde(rename = "not-billable")]
    NotBillable,
    /// Aborted
    #[serde(rename = "aborted")]
    Aborted,
    /// Billed
    #[serde(rename = "billed")]
    Billed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Citation artifact classifier
///
/// System: <http://hl7.org/fhir/citation-artifact-classifier>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitationArtifactClassifier {
    /// Audio file
    #[default]
    #[serde(rename = "audio")]
    Audio,
    /// Book
    #[serde(rename = "D001877")]
    D001877,
    /// Clinical Decision Support Artifact
    #[serde(rename = "cds-artifact")]
    CdsArtifact,
    /// Comment
    #[serde(rename = "D016420")]
    D016420,
    /// Common Share
    #[serde(rename = "common-share")]
    CommonShare,
    /// Database
    #[serde(rename = "D019991")]
    D019991,
    /// Dataset
    #[serde(rename = "D064886")]
    D064886,
    /// Dataset Unpublished
    #[serde(rename = "dataset-unpublished")]
    DatasetUnpublished,
    /// Electronic
    #[serde(rename = "Electronic")]
    Electronic,
    /// Electronic-eCollection
    #[serde(rename = "Electronic-eCollection")]
    ElectronicECollection,
    /// Electronic-Print
    #[serde(rename = "Electronic-Print")]
    ElectronicPrint,
    /// Executable app
    #[serde(rename = "executable-app")]
    ExecutableApp,
    /// FHIR Resource
    #[serde(rename = "fhir-resource")]
    FhirResource,
    /// Image file
    #[serde(rename = "image")]
    Image,
    /// Interactive Form
    #[serde(rename = "interactive-form")]
    InteractiveForm,
    /// Journal Article
    #[serde(rename = "D016428")]
    D016428,
    /// Letter
    #[serde(rename = "D016422")]
    D016422,
    /// Machine code
    #[serde(rename = "machine-code")]
    MachineCode,
    /// Medline Base
    #[serde(rename = "medline-base")]
    MedlineBase,
    /// Prediction Model
    #[serde(rename = "prediction-model")]
    PredictionModel,
    /// Preprint
    #[serde(rename = "D000076942")]
    D000076942,
    /// Print
    #[serde(rename = "Print")]
    Print,
    /// Print Electronic
    #[serde(rename = "Print-Electronic")]
    PrintElectronic,
    /// Project Specific
    #[serde(rename = "project-specific")]
    ProjectSpecific,
    /// Protocol
    #[serde(rename = "protocol")]
    Protocol,
    /// PseudoCode
    #[serde(rename = "pseudocode")]
    Pseudocode,
    /// Published Erratum
    #[serde(rename = "D016425")]
    D016425,
    /// Standard Specification
    #[serde(rename = "standard-specification")]
    StandardSpecification,
    /// Terminology
    #[serde(rename = "terminology")]
    Terminology,
    /// Video-Audio Media
    #[serde(rename = "D059040")]
    D059040,
    /// Webpage
    #[serde(rename = "webpage")]
    Webpage,
}

/// Citation classification type
///
/// System: <http://hl7.org/fhir/citation-classification-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitationClassificationType {
    /// Citation Source
    #[default]
    #[serde(rename = "citation-source")]
    CitationSource,
    /// MEDLINE Citation Owner
    #[serde(rename = "medline-owner")]
    MedlineOwner,
    /// FEvIR Platform Use
    #[serde(rename = "fevir-platform-use")]
    FevirPlatformUse,
}

/// Citation status type
///
/// System: <http://hl7.org/fhir/citation-status-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitationStatusType {
    /// PubMed Pubstatus of Received
    #[default]
    #[serde(rename = "pubmed-pubstatus-received")]
    PubmedPubstatusReceived,
    /// PubMed Pubstatus of Accepted
    #[serde(rename = "pubmed-pubstatus-accepted")]
    PubmedPubstatusAccepted,
    /// PubMed Pubstatus of Epublish
    #[serde(rename = "pubmed-pubstatus-epublish")]
    PubmedPubstatusEpublish,
    /// PubMed Pubstatus of Ppublish
    #[serde(rename = "pubmed-pubstatus-ppublish")]
    PubmedPubstatusPpublish,
    /// PubMed Pubstatus of Revised
    #[serde(rename = "pubmed-pubstatus-revised")]
    PubmedPubstatusRevised,
    /// PubMed Pubstatus of aheadofprint
    #[serde(rename = "pubmed-pubstatus-aheadofprint")]
    PubmedPubstatusAheadofprint,
    /// PubMed Pubstatus of Retracted
    #[serde(rename = "pubmed-pubstatus-retracted")]
    PubmedPubstatusRetracted,
    /// PubMed Pubstatus of Ecollection
    #[serde(rename = "pubmed-pubstatus-ecollection")]
    PubmedPubstatusEcollection,
    /// PubMed Pubstatus of PMC
    #[serde(rename = "pubmed-pubstatus-pmc")]
    PubmedPubstatusPmc,
    /// PubMed Pubstatus of PMCr
    #[serde(rename = "pubmed-pubstatus-pmcr")]
    PubmedPubstatusPmcr,
    /// PubMed Pubstatus of PubMed
    #[serde(rename = "pubmed-pubstatus-pubmed")]
    PubmedPubstatusPubmed,
    /// PubMed Pubstatus of PubMedr
    #[serde(rename = "pubmed-pubstatus-pubmedr")]
    PubmedPubstatusPubmedr,
    /// PubMed Pubstatus of Premedline
    #[serde(rename = "pubmed-pubstatus-premedline")]
    PubmedPubstatusPremedline,
    /// PubMed Pubstatus of Medline
    #[serde(rename = "pubmed-pubstatus-medline")]
    PubmedPubstatusMedline,
    /// PubMed Pubstatus of Medliner
    #[serde(rename = "pubmed-pubstatus-medliner")]
    PubmedPubstatusMedliner,
    /// PubMed Pubstatus of Entrez
    #[serde(rename = "pubmed-pubstatus-entrez")]
    PubmedPubstatusEntrez,
    /// PubMed Pubstatus of PMC release
    #[serde(rename = "pubmed-pubstatus-pmc-release")]
    PubmedPubstatusPmcRelease,
    /// Medline Citation Status of Completed
    #[serde(rename = "medline-completed")]
    MedlineCompleted,
    /// Medline Citation Status of In-Process
    #[serde(rename = "medline-in-process")]
    MedlineInProcess,
    /// Medline Citation Status of PubMed-not-MEDLINE
    #[serde(rename = "medline-pubmed-not-medline")]
    MedlinePubmedNotMedline,
    /// Medline Citation Status of In-Data-Review
    #[serde(rename = "medline-in-data-review")]
    MedlineInDataReview,
    /// Medline Citation Status of Publisher
    #[serde(rename = "medline-publisher")]
    MedlinePublisher,
    /// Medline Citation Status of MEDLINE
    #[serde(rename = "medline-medline")]
    MedlineMedline,
    /// Medline Citation Status of OLDMEDLINE
    #[serde(rename = "medline-oldmedline")]
    MedlineOldmedline,
    /// PubMed PublicationStatus of ppublish
    #[serde(rename = "pubmed-publication-status-ppublish")]
    PubmedPublicationStatusPpublish,
    /// PubMed PublicationStatus of epublish
    #[serde(rename = "pubmed-publication-status-epublish")]
    PubmedPublicationStatusEpublish,
    /// PubMed PublicationStatus of aheadofprint
    #[serde(rename = "pubmed-publication-status-aheadofprint")]
    PubmedPublicationStatusAheadofprint,
}

/// The format for display of the citation.
///
/// System: <http://hl7.org/fhir/citation-summary-style>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitationSummaryStyle {
    /// Vancouver style
    #[default]
    #[serde(rename = "vancouver")]
    Vancouver,
    /// American Medical Association 11th edition
    #[serde(rename = "ama11")]
    Ama11,
    /// American Psychological Association 7th edition
    #[serde(rename = "apa7")]
    Apa7,
    /// American Psychological Association 6th edition
    #[serde(rename = "apa6")]
    Apa6,
    /// American Sociological Association 6th edition
    #[serde(rename = "asa6")]
    Asa6,
    /// Modern Language Association 8th edition
    #[serde(rename = "mla8")]
    Mla8,
    /// Cochrane Style
    #[serde(rename = "cochrane")]
    Cochrane,
    /// Elsevier-Harvard Style
    #[serde(rename = "elsevier-harvard")]
    ElsevierHarvard,
    /// Nature Referencing style
    #[serde(rename = "nature")]
    Nature,
    /// American Chemical Society
    #[serde(rename = "acs")]
    Acs,
    /// Chicago Style Version 17 Author Date
    #[serde(rename = "chicago-a-17")]
    ChicagoA17,
    /// Chicago Style Version 17 Full note
    #[serde(rename = "chicago-b-17")]
    ChicagoB17,
    /// Institute of Electrical and Electronics Engineers
    #[serde(rename = "ieee")]
    Ieee,
    /// Computable Publishing
    #[serde(rename = "comppub")]
    Comppub,
}

/// Used to express the reason and specific aspect for the variant abstract,
/// such as language and specific language
///
/// System: <http://hl7.org/fhir/cited-artifact-abstract-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitedArtifactAbstractType {
    /// Primary human use
    #[default]
    #[serde(rename = "primary-human-use")]
    PrimaryHumanUse,
    /// Primary machine use
    #[serde(rename = "primary-machine-use")]
    PrimaryMachineUse,
    /// Truncated
    #[serde(rename = "truncated")]
    Truncated,
    /// Short abstract
    #[serde(rename = "short-abstract")]
    ShortAbstract,
    /// Long abstract
    #[serde(rename = "long-abstract")]
    LongAbstract,
    /// Plain language
    #[serde(rename = "plain-language")]
    PlainLanguage,
    /// Different publisher for abstract
    #[serde(rename = "different-publisher")]
    DifferentPublisher,
    /// Different language
    #[serde(rename = "language")]
    Language,
    /// Different language derived from autotranslation
    #[serde(rename = "autotranslated")]
    Autotranslated,
    /// Different text in additional Medline entry
    #[serde(rename = "duplicate-pmid")]
    DuplicatePmid,
    /// Different text in an earlier version
    #[serde(rename = "earlier-abstract")]
    EarlierAbstract,
}

/// Cited Artifact Classification Type
///
/// System: <http://hl7.org/fhir/cited-artifact-classification-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitedArtifactClassificationType {
    /// Publication type
    #[default]
    #[serde(rename = "publication-type")]
    PublicationType,
    /// MeSH heading
    #[serde(rename = "mesh-heading")]
    MeshHeading,
    /// Supplemental MeSH for Protocol
    #[serde(rename = "supplemental-mesh-protocol")]
    SupplementalMeshProtocol,
    /// Supplemental MeSH for Disease
    #[serde(rename = "supplemental-mesh-disease")]
    SupplementalMeshDisease,
    /// Supplemental MeSH for Organism
    #[serde(rename = "supplemental-mesh-organism")]
    SupplementalMeshOrganism,
    /// Keyword
    #[serde(rename = "keyword")]
    Keyword,
    /// Citation subset
    #[serde(rename = "citation-subset")]
    CitationSubset,
    /// Chemical
    #[serde(rename = "chemical")]
    Chemical,
    /// Publishing Model
    #[serde(rename = "publishing-model")]
    PublishingModel,
    /// Knowledge Artifact Type
    #[serde(rename = "knowledge-artifact-type")]
    KnowledgeArtifactType,
    /// Coverage
    #[serde(rename = "coverage")]
    Coverage,
}

/// To describe the reason for the variant citation, such as version number or
/// subpart specification.
///
/// System: <http://hl7.org/fhir/cited-artifact-part-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitedArtifactPartType {
    /// pages
    #[default]
    #[serde(rename = "pages")]
    Pages,
    /// sections
    #[serde(rename = "sections")]
    Sections,
    /// paragraphs
    #[serde(rename = "paragraphs")]
    Paragraphs,
    /// lines
    #[serde(rename = "lines")]
    Lines,
    /// tables
    #[serde(rename = "tables")]
    Tables,
    /// figures
    #[serde(rename = "figures")]
    Figures,
    /// Supplement or Appendix
    #[serde(rename = "supplement")]
    Supplement,
    /// Supplement or Appendix Subpart
    #[serde(rename = "supplement-subpart")]
    SupplementSubpart,
    /// Part of an article set
    #[serde(rename = "article-set")]
    ArticleSet,
}

/// Cited Artifact Status Type
///
/// System: <http://hl7.org/fhir/cited-artifact-status-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitedArtifactStatusType {
    /// Created
    #[default]
    #[serde(rename = "created")]
    Created,
    /// Submitted
    #[serde(rename = "submitted")]
    Submitted,
    /// Withdrawn
    #[serde(rename = "withdrawn")]
    Withdrawn,
    /// Pre review
    #[serde(rename = "pre-review")]
    PreReview,
    /// Under review
    #[serde(rename = "under-review")]
    UnderReview,
    /// Post review pre published
    #[serde(rename = "post-review-pre-published")]
    PostReviewPrePublished,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Published early form
    #[serde(rename = "published-early-form")]
    PublishedEarlyForm,
    /// Published final form
    #[serde(rename = "published-final-form")]
    PublishedFinalForm,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Archived
    #[serde(rename = "archived")]
    Archived,
    /// Retracted
    #[serde(rename = "retracted")]
    Retracted,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Approved
    #[serde(rename = "approved")]
    Approved,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// NLM codes Internet or Print.
///
/// System: <http://hl7.org/fhir/cited-medium>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CitedMedium {
    /// Internet
    #[default]
    #[serde(rename = "internet")]
    Internet,
    /// Print
    #[serde(rename = "print")]
    Print,
    /// Offline Digital Storage
    #[serde(rename = "offline-digital-storage")]
    OfflineDigitalStorage,
    /// Internet without issue
    #[serde(rename = "internet-without-issue")]
    InternetWithoutIssue,
    /// Print without issue
    #[serde(rename = "print-without-issue")]
    PrintWithoutIssue,
    /// Offline Digital Storage without issue
    #[serde(rename = "offline-digital-storage-without-issue")]
    OfflineDigitalStorageWithoutIssue,
}

/// This value set provides Claim Adjudication Decision codes.
///
/// System: <http://hl7.org/fhir/claim-decision>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimDecision {
    /// Denied
    #[default]
    #[serde(rename = "denied")]
    Denied,
    /// Approved
    #[serde(rename = "approved")]
    Approved,
    /// Partial
    #[serde(rename = "partial")]
    Partial,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
}

/// This value set provides example Claim Adjudication Decision Reason codes.
///
/// System: <http://hl7.org/fhir/claim-decision-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimDecisionReason {
    /// Not medically necessary
    #[default]
    #[serde(rename = "0001")]
    N0001,
    /// Prior authorization not obtained
    #[serde(rename = "0002")]
    N0002,
    /// Provider out-of-network
    #[serde(rename = "0003")]
    N0003,
    /// Service inconsistent with patient age
    #[serde(rename = "0004")]
    N0004,
    /// Benefit limits exceeded
    #[serde(rename = "0005")]
    N0005,
}

/// This value set includes Claim Processing Outcome codes.
///
/// System: <http://hl7.org/fhir/claim-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimOutcome {
    /// Queued
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Processing Complete
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial Processing
    #[serde(rename = "partial")]
    Partial,
}

/// The purpose of the Claim: predetermination, preauthorization, claim.
///
/// System: <http://hl7.org/fhir/claim-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimUse {
    /// Claim
    #[default]
    #[serde(rename = "claim")]
    Claim,
    /// Preauthorization
    #[serde(rename = "preauthorization")]
    Preauthorization,
    /// Predetermination
    #[serde(rename = "predetermination")]
    Predetermination,
}

/// ClinicalUseDefinitionCategory
///
/// System: <http://hl7.org/fhir/clinical-use-definition-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClinicalUseDefinitionCategory {
    /// Pregnancy and Lactation
    #[default]
    #[serde(rename = "Pregnancy")]
    Pregnancy,
    /// Overdose
    #[serde(rename = "Overdose")]
    Overdose,
    /// Effects on Ability to Drive and Use Machines
    #[serde(rename = "DriveAndMachines")]
    DriveAndMachines,
}

/// Overall defining type of this clinical use definition.
///
/// System: <http://hl7.org/fhir/clinical-use-definition-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClinicalUseDefinitionType {
    /// Indication
    #[default]
    #[serde(rename = "indication")]
    Indication,
    /// Contraindication
    #[serde(rename = "contraindication")]
    Contraindication,
    /// Interaction
    #[serde(rename = "interaction")]
    Interaction,
    /// Undesirable Effect
    #[serde(rename = "undesirable-effect")]
    UndesirableEffect,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
}

/// The degree to which the server supports the code search parameter on
/// ValueSet, if it is supported.
///
/// System: <http://hl7.org/fhir/code-search-support>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CodeSearchSupport {
    /// In Compose
    #[default]
    #[serde(rename = "in-compose")]
    InCompose,
    /// In Expansion
    #[serde(rename = "in-expansion")]
    InExpansion,
    /// In Compose Or Expansion
    #[serde(rename = "in-compose-or-expansion")]
    InComposeOrExpansion,
}

/// The extent of the content of the code system (the concepts and codes it
/// defines) are represented in a code system resource.
///
/// System: <http://hl7.org/fhir/codesystem-content-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CodesystemContentMode {
    /// Not Present
    #[default]
    #[serde(rename = "not-present")]
    NotPresent,
    /// Example
    #[serde(rename = "example")]
    Example,
    /// Fragment
    #[serde(rename = "fragment")]
    Fragment,
    /// Complete
    #[serde(rename = "complete")]
    Complete,
    /// Supplement
    #[serde(rename = "supplement")]
    Supplement,
}

/// The meaning of the hierarchy of concepts in a code system.
///
/// System: <http://hl7.org/fhir/codesystem-hierarchy-meaning>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CodesystemHierarchyMeaning {
    /// Grouped By
    #[default]
    #[serde(rename = "grouped-by")]
    GroupedBy,
    /// Is-A
    #[serde(rename = "is-a")]
    IsA,
    /// Part Of
    #[serde(rename = "part-of")]
    PartOf,
    /// Classified With
    #[serde(rename = "classified-with")]
    ClassifiedWith,
}

/// This code system represents that named RGB colors found in the [CSS4
/// specification](https://www.w3.org/TR/css-color-4/). The names are not case
/// sensitive and different cases are encountered in common use (e.g. AliceBlue
/// vs aliceblue.
///
/// System: <http://hl7.org/fhir/color-names>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ColorNames {
    /// aliceblue
    #[default]
    #[serde(rename = "aliceblue")]
    Aliceblue,
    /// antiquewhite
    #[serde(rename = "antiquewhite")]
    Antiquewhite,
    /// aqua
    #[serde(rename = "aqua")]
    Aqua,
    /// aquamarine
    #[serde(rename = "aquamarine")]
    Aquamarine,
    /// azure
    #[serde(rename = "azure")]
    Azure,
    /// beige
    #[serde(rename = "beige")]
    Beige,
    /// bisque
    #[serde(rename = "bisque")]
    Bisque,
    /// black
    #[serde(rename = "black")]
    Black,
    /// blanchedalmond
    #[serde(rename = "blanchedalmond")]
    Blanchedalmond,
    /// blue
    #[serde(rename = "blue")]
    Blue,
    /// blueviolet
    #[serde(rename = "blueviolet")]
    Blueviolet,
    /// brown
    #[serde(rename = "brown")]
    Brown,
    /// burlywood
    #[serde(rename = "burlywood")]
    Burlywood,
    /// cadetblue
    #[serde(rename = "cadetblue")]
    Cadetblue,
    /// chartreuse
    #[serde(rename = "chartreuse")]
    Chartreuse,
    /// chocolate
    #[serde(rename = "chocolate")]
    Chocolate,
    /// coral
    #[serde(rename = "coral")]
    Coral,
    /// cornflowerblue
    #[serde(rename = "cornflowerblue")]
    Cornflowerblue,
    /// cornsilk
    #[serde(rename = "cornsilk")]
    Cornsilk,
    /// crimson
    #[serde(rename = "crimson")]
    Crimson,
    /// cyan
    #[serde(rename = "cyan")]
    Cyan,
    /// darkblue
    #[serde(rename = "darkblue")]
    Darkblue,
    /// darkcyan
    #[serde(rename = "darkcyan")]
    Darkcyan,
    /// darkgoldenrod
    #[serde(rename = "darkgoldenrod")]
    Darkgoldenrod,
    /// darkgray
    #[serde(rename = "darkgray")]
    Darkgray,
    /// darkgreen
    #[serde(rename = "darkgreen")]
    Darkgreen,
    /// darkgrey
    #[serde(rename = "darkgrey")]
    Darkgrey,
    /// darkkhaki
    #[serde(rename = "darkkhaki")]
    Darkkhaki,
    /// darkmagenta
    #[serde(rename = "darkmagenta")]
    Darkmagenta,
    /// darkolivegreen
    #[serde(rename = "darkolivegreen")]
    Darkolivegreen,
    /// darkorange
    #[serde(rename = "darkorange")]
    Darkorange,
    /// darkorchid
    #[serde(rename = "darkorchid")]
    Darkorchid,
    /// darkred
    #[serde(rename = "darkred")]
    Darkred,
    /// darksalmon
    #[serde(rename = "darksalmon")]
    Darksalmon,
    /// darkseagreen
    #[serde(rename = "darkseagreen")]
    Darkseagreen,
    /// darkslateblue
    #[serde(rename = "darkslateblue")]
    Darkslateblue,
    /// darkslategray
    #[serde(rename = "darkslategray")]
    Darkslategray,
    /// darkslategrey
    #[serde(rename = "darkslategrey")]
    Darkslategrey,
    /// darkturquoise
    #[serde(rename = "darkturquoise")]
    Darkturquoise,
    /// darkviolet
    #[serde(rename = "darkviolet")]
    Darkviolet,
    /// deeppink
    #[serde(rename = "deeppink")]
    Deeppink,
    /// deepskyblue
    #[serde(rename = "deepskyblue")]
    Deepskyblue,
    /// dimgray
    #[serde(rename = "dimgray")]
    Dimgray,
    /// dimgrey
    #[serde(rename = "dimgrey")]
    Dimgrey,
    /// dodgerblue
    #[serde(rename = "dodgerblue")]
    Dodgerblue,
    /// firebrick
    #[serde(rename = "firebrick")]
    Firebrick,
    /// floralwhite
    #[serde(rename = "floralwhite")]
    Floralwhite,
    /// forestgreen
    #[serde(rename = "forestgreen")]
    Forestgreen,
    /// fuchsia
    #[serde(rename = "fuchsia")]
    Fuchsia,
    /// gainsboro
    #[serde(rename = "gainsboro")]
    Gainsboro,
    /// ghostwhite
    #[serde(rename = "ghostwhite")]
    Ghostwhite,
    /// gold
    #[serde(rename = "gold")]
    Gold,
    /// goldenrod
    #[serde(rename = "goldenrod")]
    Goldenrod,
    /// gray
    #[serde(rename = "gray")]
    Gray,
    /// green
    #[serde(rename = "green")]
    Green,
    /// greenyellow
    #[serde(rename = "greenyellow")]
    Greenyellow,
    /// grey
    #[serde(rename = "grey")]
    Grey,
    /// honeydew
    #[serde(rename = "honeydew")]
    Honeydew,
    /// hotpink
    #[serde(rename = "hotpink")]
    Hotpink,
    /// indianred
    #[serde(rename = "indianred")]
    Indianred,
    /// indigo
    #[serde(rename = "indigo")]
    Indigo,
    /// ivory
    #[serde(rename = "ivory")]
    Ivory,
    /// khaki
    #[serde(rename = "khaki")]
    Khaki,
    /// lavender
    #[serde(rename = "lavender")]
    Lavender,
    /// lavenderblush
    #[serde(rename = "lavenderblush")]
    Lavenderblush,
    /// lawngreen
    #[serde(rename = "lawngreen")]
    Lawngreen,
    /// lemonchiffon
    #[serde(rename = "lemonchiffon")]
    Lemonchiffon,
    /// lightblue
    #[serde(rename = "lightblue")]
    Lightblue,
    /// lightcoral
    #[serde(rename = "lightcoral")]
    Lightcoral,
    /// lightcyan
    #[serde(rename = "lightcyan")]
    Lightcyan,
    /// lightgoldenrodyellow
    #[serde(rename = "lightgoldenrodyellow")]
    Lightgoldenrodyellow,
    /// lightgray
    #[serde(rename = "lightgray")]
    Lightgray,
    /// lightgreen
    #[serde(rename = "lightgreen")]
    Lightgreen,
    /// lightgrey
    #[serde(rename = "lightgrey")]
    Lightgrey,
    /// lightpink
    #[serde(rename = "lightpink")]
    Lightpink,
    /// lightsalmon
    #[serde(rename = "lightsalmon")]
    Lightsalmon,
    /// lightseagreen
    #[serde(rename = "lightseagreen")]
    Lightseagreen,
    /// lightskyblue
    #[serde(rename = "lightskyblue")]
    Lightskyblue,
    /// lightslategray
    #[serde(rename = "lightslategray")]
    Lightslategray,
    /// lightslategrey
    #[serde(rename = "lightslategrey")]
    Lightslategrey,
    /// lightsteelblue
    #[serde(rename = "lightsteelblue")]
    Lightsteelblue,
    /// lightyellow
    #[serde(rename = "lightyellow")]
    Lightyellow,
    /// lime
    #[serde(rename = "lime")]
    Lime,
    /// limegreen
    #[serde(rename = "limegreen")]
    Limegreen,
    /// linen
    #[serde(rename = "linen")]
    Linen,
    /// magenta
    #[serde(rename = "magenta")]
    Magenta,
    /// maroon
    #[serde(rename = "maroon")]
    Maroon,
    /// mediumaquamarine
    #[serde(rename = "mediumaquamarine")]
    Mediumaquamarine,
    /// mediumblue
    #[serde(rename = "mediumblue")]
    Mediumblue,
    /// mediumorchid
    #[serde(rename = "mediumorchid")]
    Mediumorchid,
    /// mediumpurple
    #[serde(rename = "mediumpurple")]
    Mediumpurple,
    /// mediumseagreen
    #[serde(rename = "mediumseagreen")]
    Mediumseagreen,
    /// mediumslateblue
    #[serde(rename = "mediumslateblue")]
    Mediumslateblue,
    /// mediumspringgreen
    #[serde(rename = "mediumspringgreen")]
    Mediumspringgreen,
    /// mediumturquoise
    #[serde(rename = "mediumturquoise")]
    Mediumturquoise,
    /// mediumvioletred
    #[serde(rename = "mediumvioletred")]
    Mediumvioletred,
    /// midnightblue
    #[serde(rename = "midnightblue")]
    Midnightblue,
    /// mintcream
    #[serde(rename = "mintcream")]
    Mintcream,
    /// mistyrose
    #[serde(rename = "mistyrose")]
    Mistyrose,
    /// moccasin
    #[serde(rename = "moccasin")]
    Moccasin,
    /// navajowhite
    #[serde(rename = "navajowhite")]
    Navajowhite,
    /// navy
    #[serde(rename = "navy")]
    Navy,
    /// oldlace
    #[serde(rename = "oldlace")]
    Oldlace,
    /// olive
    #[serde(rename = "olive")]
    Olive,
    /// olivedrab
    #[serde(rename = "olivedrab")]
    Olivedrab,
    /// orange
    #[serde(rename = "orange")]
    Orange,
    /// orangered
    #[serde(rename = "orangered")]
    Orangered,
    /// orchid
    #[serde(rename = "orchid")]
    Orchid,
    /// palegoldenrod
    #[serde(rename = "palegoldenrod")]
    Palegoldenrod,
    /// palegreen
    #[serde(rename = "palegreen")]
    Palegreen,
    /// paleturquoise
    #[serde(rename = "paleturquoise")]
    Paleturquoise,
    /// palevioletred
    #[serde(rename = "palevioletred")]
    Palevioletred,
    /// papayawhip
    #[serde(rename = "papayawhip")]
    Papayawhip,
    /// peachpuff
    #[serde(rename = "peachpuff")]
    Peachpuff,
    /// peru
    #[serde(rename = "peru")]
    Peru,
    /// pink
    #[serde(rename = "pink")]
    Pink,
    /// plum
    #[serde(rename = "plum")]
    Plum,
    /// powderblue
    #[serde(rename = "powderblue")]
    Powderblue,
    /// purple
    #[serde(rename = "purple")]
    Purple,
    /// rebeccapurple
    #[serde(rename = "rebeccapurple")]
    Rebeccapurple,
    /// red
    #[serde(rename = "red")]
    Red,
    /// rosybrown
    #[serde(rename = "rosybrown")]
    Rosybrown,
    /// royalblue
    #[serde(rename = "royalblue")]
    Royalblue,
    /// saddlebrown
    #[serde(rename = "saddlebrown")]
    Saddlebrown,
    /// salmon
    #[serde(rename = "salmon")]
    Salmon,
    /// sandybrown
    #[serde(rename = "sandybrown")]
    Sandybrown,
    /// seagreen
    #[serde(rename = "seagreen")]
    Seagreen,
    /// seashell
    #[serde(rename = "seashell")]
    Seashell,
    /// sienna
    #[serde(rename = "sienna")]
    Sienna,
    /// silver
    #[serde(rename = "silver")]
    Silver,
    /// skyblue
    #[serde(rename = "skyblue")]
    Skyblue,
    /// slateblue
    #[serde(rename = "slateblue")]
    Slateblue,
    /// slategray
    #[serde(rename = "slategray")]
    Slategray,
    /// slategrey
    #[serde(rename = "slategrey")]
    Slategrey,
    /// snow
    #[serde(rename = "snow")]
    Snow,
    /// springgreen
    #[serde(rename = "springgreen")]
    Springgreen,
    /// steelblue
    #[serde(rename = "steelblue")]
    Steelblue,
    /// tan
    #[serde(rename = "tan")]
    Tan,
    /// teal
    #[serde(rename = "teal")]
    Teal,
    /// thistle
    #[serde(rename = "thistle")]
    Thistle,
    /// tomato
    #[serde(rename = "tomato")]
    Tomato,
    /// turquoise
    #[serde(rename = "turquoise")]
    Turquoise,
    /// violet
    #[serde(rename = "violet")]
    Violet,
    /// wheat
    #[serde(rename = "wheat")]
    Wheat,
    /// white
    #[serde(rename = "white")]
    White,
    /// whitesmoke
    #[serde(rename = "whitesmoke")]
    Whitesmoke,
    /// yellow
    #[serde(rename = "yellow")]
    Yellow,
    /// yellowgreen
    #[serde(rename = "yellowgreen")]
    Yellowgreen,
}

/// Dose forms for a product as a whole, considering all individual parts, but
/// before any mixing
///
/// System: <http://hl7.org/fhir/combined-dose-form>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CombinedDoseForm {
    /// Powder and solvent for oral solution
    #[default]
    #[serde(rename = "100000073366")]
    N100000073366,
    /// Powder and solvent for oral suspension
    #[serde(rename = "100000073651")]
    N100000073651,
    /// Eye drops, powder and solvent for solution
    #[serde(rename = "100000073774")]
    N100000073774,
    /// Eye drops, powder and solvent for suspension
    #[serde(rename = "100000073781")]
    N100000073781,
    /// Ear drops, powder and solvent for suspension
    #[serde(rename = "100000073801")]
    N100000073801,
    /// Powder and solvent for solution for infusion
    #[serde(rename = "100000073860")]
    N100000073860,
    /// Powder and solvent for solution for injection
    #[serde(rename = "100000073868")]
    N100000073868,
    /// Powder and solvent for suspension for injection
    #[serde(rename = "100000073869")]
    N100000073869,
    /// Powder and solvent for implantation paste
    #[serde(rename = "100000073884")]
    N100000073884,
    /// Endotracheopulmonary instillation, powder and solvent for solution
    #[serde(rename = "100000073891")]
    N100000073891,
    /// Powder and solvent for endocervical gel
    #[serde(rename = "100000073892")]
    N100000073892,
    /// Powder and solvent for sealant
    #[serde(rename = "100000073941")]
    N100000073941,
    /// Concentrate and solvent for concentrate for solution for infusion
    #[serde(rename = "100000073972")]
    N100000073972,
    /// Concentrate and solvent for cutaneous use
    #[serde(rename = "100000073973")]
    N100000073973,
    /// Concentrate and solvent for injection
    #[serde(rename = "100000073974")]
    N100000073974,
    /// Concentrate and solvent for solution for infusion
    #[serde(rename = "100000073975")]
    N100000073975,
    /// Concentrate and diluent for solution for infusion
    #[serde(rename = "100000073987")]
    N100000073987,
    /// Concentrate and solvent for cutaneous solution
    #[serde(rename = "100000073988")]
    N100000073988,
    /// Concentrate and solvent for solution for injection
    #[serde(rename = "100000073989")]
    N100000073989,
    /// Concentrate and solvent for suspension for injection
    #[serde(rename = "100000073990")]
    N100000073990,
    /// Granules and solvent for suspension for injection
    #[serde(rename = "100000073999")]
    N100000073999,
    /// Powder and solvent for concentrate for solution for infusion
    #[serde(rename = "100000074015")]
    N100000074015,
    /// Powder and solvent for cutaneous solution
    #[serde(rename = "100000074016")]
    N100000074016,
    /// Powder and solvent for gingival gel
    #[serde(rename = "100000074017")]
    N100000074017,
    /// Powder and solvent for prolonged-release suspension for injection
    #[serde(rename = "100000074018")]
    N100000074018,
    /// Powder and solvent for endosinusial solution
    #[serde(rename = "100000074030")]
    N100000074030,
    /// Powder and solvent for intraocular instillation solution
    #[serde(rename = "100000074031")]
    N100000074031,
    /// Powder and suspension for suspension for injection
    #[serde(rename = "100000074032")]
    N100000074032,
    /// Suspension and effervescent granules for oral suspension
    #[serde(rename = "100000074048")]
    N100000074048,
    /// Tablet and solvent for rectal suspension
    #[serde(rename = "100000074051")]
    N100000074051,
    /// Powder and solvent for dental gel
    #[serde(rename = "100000074053")]
    N100000074053,
    /// Gas and solvent for dispersion for injection/infusion
    #[serde(rename = "100000074056")]
    N100000074056,
    /// Powder and solvent for solution for injection/infusion
    #[serde(rename = "100000074057")]
    N100000074057,
    /// Suspension and solution for spray
    #[serde(rename = "100000074061")]
    N100000074061,
    /// Tablet and powder for oral solution
    #[serde(rename = "100000074064")]
    N100000074064,
    /// Emulsion and suspension for emulsion for injection
    #[serde(rename = "100000075580")]
    N100000075580,
    /// Powder and solvent for dispersion for injection
    #[serde(rename = "100000075584")]
    N100000075584,
    /// Powder for mouth wash
    #[serde(rename = "100000075587")]
    N100000075587,
    /// Lyophilisate and solvent for solution for injection
    #[serde(rename = "100000116137")]
    N100000116137,
    /// Fibrin sealant-powder and solvent for fibrin sealant
    #[serde(rename = "100000116141")]
    N100000116141,
    /// Granules and solvent for oral suspension
    #[serde(rename = "100000116155")]
    N100000116155,
    /// Lyophilisate and solvent for suspension for injection
    #[serde(rename = "100000116160")]
    N100000116160,
    /// Powder and gel for gel
    #[serde(rename = "100000116172")]
    N100000116172,
    /// Powder and solution for solution for injection
    #[serde(rename = "100000116173")]
    N100000116173,
    /// Powder and solvent for epilesional solution
    #[serde(rename = "100000116174")]
    N100000116174,
    /// Powder and solvent for intravesical solution
    #[serde(rename = "100000116175")]
    N100000116175,
    /// Powder and solvent for intravesical suspension
    #[serde(rename = "100000116176")]
    N100000116176,
    /// Powder and solvent for nebuliser solution
    #[serde(rename = "100000116177")]
    N100000116177,
    /// Powder, dispersion and solvent for concentrate for dispersion for
    /// infusion
    #[serde(rename = "100000116179")]
    N100000116179,
    /// Powder and solvent for emulsion for injection
    #[serde(rename = "100000125746")]
    N100000125746,
    /// Nasal drops, powder and solvent for solution
    #[serde(rename = "100000125747")]
    N100000125747,
    /// Suspension and solvent for suspension for injection
    #[serde(rename = "100000125777")]
    N100000125777,
    /// Concentrate and solvent for solution for injection/infusion
    #[serde(rename = "100000136318")]
    N100000136318,
    /// Powder and solvent for solution for injection/skin-prick test
    #[serde(rename = "100000136325")]
    N100000136325,
    /// Lyophilisate and solvent for suspension for nasal administration
    #[serde(rename = "100000136558")]
    N100000136558,
    /// Powder and solvent for solution for sealant
    #[serde(rename = "100000136560")]
    N100000136560,
    /// Solution for dispersion for injection/infusion
    #[serde(rename = "100000136907")]
    N100000136907,
    /// Powder and solution for dental cement
    #[serde(rename = "100000143502")]
    N100000143502,
    /// Endotracheopulmonary instillation, powder and solvent for suspension
    #[serde(rename = "100000143546")]
    N100000143546,
    /// Powder, solvent and matrix for implantation matrix
    #[serde(rename = "100000143552")]
    N100000143552,
    /// Nasal drops, lyophilisate and solvent for suspension
    #[serde(rename = "100000156068")]
    N100000156068,
    /// Lyophilisate and suspension for suspension for injection
    #[serde(rename = "100000157796")]
    N100000157796,
    /// Powder for concentrate and solution for solution for infusion
    #[serde(rename = "100000164467")]
    N100000164467,
    /// Powder and solution for bee-hive solution
    #[serde(rename = "100000169997")]
    N100000169997,
    /// Suspension and solvent for oral spray
    #[serde(rename = "100000170588")]
    N100000170588,
    /// Lyophilisate and solvent for oral suspension
    #[serde(rename = "100000171127")]
    N100000171127,
    /// Concentrate and solvent for concentrate for oral spray, suspension
    #[serde(rename = "100000171193")]
    N100000171193,
    /// Lyophilisate and solvent for oculonasal suspension
    #[serde(rename = "100000171238")]
    N100000171238,
    /// Emulsion and lyophilisate for suspension for injection
    #[serde(rename = "100000171935")]
    N100000171935,
    /// Powder and solvent for syrup
    #[serde(rename = "100000174065")]
    N100000174065,
    /// Nasal spray, lyophilisate and solvent for suspension
    #[serde(rename = "200000002161")]
    N200000002161,
    /// Powder and solution for bee-hive dispersion
    #[serde(rename = "200000002287")]
    N200000002287,
    /// Solution and dispersion for nebuliser dispersion
    #[serde(rename = "200000004201")]
    N200000004201,
    /// Effervescent powder and powder for oral suspension
    #[serde(rename = "200000004819")]
    N200000004819,
    /// Lyophilisate and solvent for emulsion for injection
    #[serde(rename = "200000004820")]
    N200000004820,
    /// Powder and solution for suspension for injection
    #[serde(rename = "200000005547")]
    N200000005547,
    /// Lyophilisate and solvent for suspension for nasal spray or injection
    #[serde(rename = "200000010382")]
    N200000010382,
}

/// Which type a compartment definition describes.
///
/// System: <http://hl7.org/fhir/compartment-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CompartmentType {
    /// Patient
    #[default]
    #[serde(rename = "Patient")]
    Patient,
    /// Encounter
    #[serde(rename = "Encounter")]
    Encounter,
    /// RelatedPerson
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    /// Practitioner
    #[serde(rename = "Practitioner")]
    Practitioner,
    /// Device
    #[serde(rename = "Device")]
    Device,
    /// EpisodeOfCare
    #[serde(rename = "EpisodeOfCare")]
    EpisodeOfCare,
}

/// The way in which a person authenticated a composition.
///
/// System: <http://hl7.org/fhir/composition-attestation-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CompositionAttestationMode {
    /// Personal
    #[default]
    #[serde(rename = "personal")]
    Personal,
    /// Professional
    #[serde(rename = "professional")]
    Professional,
    /// Legal
    #[serde(rename = "legal")]
    Legal,
    /// Official
    #[serde(rename = "official")]
    Official,
}

/// The workflow/clinical status of the composition.
///
/// System: <http://hl7.org/fhir/composition-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CompositionStatus {
    /// Registered
    #[default]
    #[serde(rename = "registered")]
    Registered,
    /// Partial
    #[serde(rename = "partial")]
    Partial,
    /// Preliminary
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Corrected
    #[serde(rename = "corrected")]
    Corrected,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Deprecated
    #[serde(rename = "deprecated")]
    Deprecated,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The relationship between concepts.
///
/// System: <http://hl7.org/fhir/concept-map-relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptMapRelationship {
    /// Related To
    #[default]
    #[serde(rename = "related-to")]
    RelatedTo,
    /// Equivalent
    #[serde(rename = "equivalent")]
    Equivalent,
    /// Source Is Narrower Than Target
    #[serde(rename = "source-is-narrower-than-target")]
    SourceIsNarrowerThanTarget,
    /// Source Is Broader Than Target
    #[serde(rename = "source-is-broader-than-target")]
    SourceIsBroaderThanTarget,
    /// Not Related To
    #[serde(rename = "not-related-to")]
    NotRelatedTo,
}

/// A set of common concept properties for use on coded systems throughout the
/// FHIR eco-system.
///
/// System: <http://hl7.org/fhir/concept-properties>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptProperties {
    /// Status
    #[default]
    #[serde(rename = "status")]
    Status,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// effectiveDate
    #[serde(rename = "effectiveDate")]
    EffectiveDate,
    /// Deprecated
    #[serde(rename = "deprecated")]
    Deprecated,
    /// Deprecation Date
    #[serde(rename = "deprecationDate")]
    DeprecationDate,
    /// Retirement Date
    #[serde(rename = "retirementDate")]
    RetirementDate,
    /// Not Selectable
    #[serde(rename = "notSelectable")]
    NotSelectable,
    /// Parent
    #[serde(rename = "parent")]
    Parent,
    /// Child
    #[serde(rename = "child")]
    Child,
    /// Part Of
    #[serde(rename = "partOf")]
    PartOf,
    /// Synonym
    #[serde(rename = "synonym")]
    Synonym,
    /// Comment
    #[serde(rename = "comment")]
    Comment,
    /// itemWeight
    #[serde(rename = "itemWeight")]
    ItemWeight,
}

/// The type of a property value.
///
/// System: <http://hl7.org/fhir/concept-property-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptPropertyType {
    /// code (internal reference)
    #[default]
    #[serde(rename = "code")]
    Code,
    /// Coding (external reference)
    #[serde(rename = "Coding")]
    Coding,
    /// string
    #[serde(rename = "string")]
    String,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
}

/// Codes indicating the results of a subsumption check between codes. In the
/// context of this CodeSystem, subsumption is defined in the FHIR
/// specification under Resource Types - CodeSystem.
///
/// System: <http://hl7.org/fhir/concept-subsumption-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptSubsumptionOutcome {
    /// Equivalent
    #[default]
    #[serde(rename = "equivalent")]
    Equivalent,
    /// Subsumes
    #[serde(rename = "subsumes")]
    Subsumes,
    /// Subsumed-By
    #[serde(rename = "subsumed-by")]
    SubsumedBy,
    /// Not-Subsumed
    #[serde(rename = "not-subsumed")]
    NotSubsumed,
}

/// The type of a ConceptMap map attribute value.
///
/// System: <http://hl7.org/fhir/conceptmap-attribute-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptmapAttributeType {
    /// code
    #[default]
    #[serde(rename = "code")]
    Code,
    /// Coding
    #[serde(rename = "Coding")]
    Coding,
    /// string
    #[serde(rename = "string")]
    String,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
}

/// A set of common concept properties for use on ConceptMap
///
/// System: <http://hl7.org/fhir/conceptmap-properties>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptmapProperties {
    /// Relationship Refinement
    #[default]
    #[serde(rename = "relationshipRefinement")]
    RelationshipRefinement,
}

/// The type of a ConceptMap mapping property value.
///
/// System: <http://hl7.org/fhir/conceptmap-property-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptmapPropertyType {
    /// Coding (external reference)
    #[default]
    #[serde(rename = "Coding")]
    Coding,
    /// string
    #[serde(rename = "string")]
    String,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// code
    #[serde(rename = "code")]
    Code,
}

/// Defines which action to take if there is no match in the group.
///
/// System: <http://hl7.org/fhir/conceptmap-unmapped-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptmapUnmappedMode {
    /// Use Provided Source Code
    #[default]
    #[serde(rename = "use-source-code")]
    UseSourceCode,
    /// Fixed Code
    #[serde(rename = "fixed")]
    Fixed,
    /// Other Map
    #[serde(rename = "other-map")]
    OtherMap,
}

/// Kind of precondition for the condition.
///
/// System: <http://hl7.org/fhir/condition-precondition-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionPreconditionType {
    /// Sensitive
    #[default]
    #[serde(rename = "sensitive")]
    Sensitive,
    /// Specific
    #[serde(rename = "specific")]
    Specific,
}

/// The use of a questionnaire.
///
/// System: <http://hl7.org/fhir/condition-questionnaire-purpose>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionQuestionnairePurpose {
    /// Pre-admit
    #[default]
    #[serde(rename = "preadmit")]
    Preadmit,
    /// Diff Diagnosis
    #[serde(rename = "diff-diagnosis")]
    DiffDiagnosis,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
}

/// A code that indicates how the server supports conditional delete.
///
/// System: <http://hl7.org/fhir/conditional-delete-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionalDeleteStatus {
    /// Not Supported
    #[default]
    #[serde(rename = "not-supported")]
    NotSupported,
    /// Single Deletes Supported
    #[serde(rename = "single")]
    Single,
    /// Multiple Deletes Supported
    #[serde(rename = "multiple")]
    Multiple,
}

/// A code that indicates how the server supports conditional read.
///
/// System: <http://hl7.org/fhir/conditional-read-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionalReadStatus {
    /// Not Supported
    #[default]
    #[serde(rename = "not-supported")]
    NotSupported,
    /// If-Modified-Since
    #[serde(rename = "modified-since")]
    ModifiedSince,
    /// If-None-Match
    #[serde(rename = "not-match")]
    NotMatch,
    /// Full Support
    #[serde(rename = "full-support")]
    FullSupport,
}

/// ConformanceExpectation
///
/// System: <http://hl7.org/fhir/conformance-expectation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConformanceExpectation {
    /// SHALL
    #[default]
    #[serde(rename = "SHALL")]
    Shall,
    /// SHOULD
    #[serde(rename = "SHOULD")]
    Should,
    /// MAY
    #[serde(rename = "MAY")]
    May,
    /// SHOULD-NOT
    #[serde(rename = "SHOULD-NOT")]
    ShouldNot,
}

/// How a resource reference is interpreted when testing consent restrictions.
///
/// System: <http://hl7.org/fhir/consent-data-meaning>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConsentDataMeaning {
    /// Instance
    #[default]
    #[serde(rename = "instance")]
    Instance,
    /// Related
    #[serde(rename = "related")]
    Related,
    /// Dependents
    #[serde(rename = "dependents")]
    Dependents,
    /// AuthoredBy
    #[serde(rename = "authoredby")]
    Authoredby,
}

/// How a rule statement is applied, such as adding additional consent or
/// removing consent.
///
/// System: <http://hl7.org/fhir/consent-provision-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConsentProvisionType {
    /// Deny
    #[default]
    #[serde(rename = "deny")]
    Deny,
    /// Permit
    #[serde(rename = "permit")]
    Permit,
}

/// Indicates the state of the consent.
///
/// System: <http://hl7.org/fhir/consent-state-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConsentStateCodes {
    /// Pending
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Abandoned
    #[serde(rename = "not-done")]
    NotDone,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// SHALL applications comply with this constraint?
///
/// System: <http://hl7.org/fhir/constraint-severity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConstraintSeverity {
    /// Error
    #[default]
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
}

/// Telecommunications form for contact point.
///
/// System: <http://hl7.org/fhir/contact-point-system>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContactPointSystem {
    /// Phone
    #[default]
    #[serde(rename = "phone")]
    Phone,
    /// Fax
    #[serde(rename = "fax")]
    Fax,
    /// Email
    #[serde(rename = "email")]
    Email,
    /// Pager
    #[serde(rename = "pager")]
    Pager,
    /// URL
    #[serde(rename = "url")]
    Url,
    /// SMS
    #[serde(rename = "sms")]
    Sms,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Use of contact point.
///
/// System: <http://hl7.org/fhir/contact-point-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContactPointUse {
    /// Home
    #[default]
    #[serde(rename = "home")]
    Home,
    /// Work
    #[serde(rename = "work")]
    Work,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Old
    #[serde(rename = "old")]
    Old,
    /// Mobile
    #[serde(rename = "mobile")]
    Mobile,
}

/// This CodeSystem contains FHIR-defined contract action status types.
///
/// System: <http://hl7.org/fhir/contract-action-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractActionStatus {
    /// Complete
    #[default]
    #[serde(rename = "complete")]
    Complete,
}

/// This CodeSystem contains FHIR-defined contract asset context types.
///
/// System: <http://hl7.org/fhir/contract-asset-context>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractAssetContext {
    /// Custodian
    #[default]
    #[serde(rename = "custodian")]
    Custodian,
}

/// This CodeSystem contains FHIR-defined contract asset scope types.
///
/// System: <http://hl7.org/fhir/contract-asset-scope>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractAssetScope {
    /// Thing
    #[default]
    #[serde(rename = "thing")]
    Thing,
}

/// This CodeSystem contains FHIR-defined contract asset type sub-types.
///
/// System: <http://hl7.org/fhir/contract-asset-subtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractAssetSubtype {
    /// Participation
    #[default]
    #[serde(rename = "participation")]
    Participation,
}

/// This CodeSystem contains FHIR-defined contract asset type types.
///
/// System: <http://hl7.org/fhir/contract-asset-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractAssetType {
    /// Participation
    #[default]
    #[serde(rename = "participation")]
    Participation,
}

/// This CodeSystem contains FHIR-defined contract decision mode types.
///
/// System: <http://hl7.org/fhir/contract-decision-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractDecisionMode {
    /// Policy
    #[default]
    #[serde(rename = "policy")]
    Policy,
}

/// This CodeSystem contains FHIR-defined contract definition subtypes.
///
/// System: <http://hl7.org/fhir/contract-definition-subtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractDefinitionSubtype {
    /// Temporary Value
    #[default]
    #[serde(rename = "temp")]
    Temp,
}

/// This CodeSystem contains FHIR-defined contract definition types.
///
/// System: <http://hl7.org/fhir/contract-definition-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractDefinitionType {
    /// Temporary Value
    #[default]
    #[serde(rename = "temp")]
    Temp,
}

/// This CodeSystem contains FHIR-defined contract Expiration types.
///
/// System: <http://hl7.org/fhir/contract-expiration-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractExpirationType {
    /// Breach
    #[default]
    #[serde(rename = "breach")]
    Breach,
}

/// This CodeSystem contains FHIR-defined contract status types. Each
/// definition includes usage notes explaining the precedence order in contract
/// lifecycle - i.e., while only some stages are required, the order in which
/// they may occur is deterministic; and a map to comparable FHIR and v.3
/// status codes. It follows guidance about use of status codes in FHIR in the
/// [Status Codes Grid](sc.html).
///
/// System: <http://hl7.org/fhir/contract-legalstate>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractLegalstate {
    /// Amended
    #[default]
    #[serde(rename = "amended")]
    Amended,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Disputed
    #[serde(rename = "disputed")]
    Disputed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Executable
    #[serde(rename = "executable")]
    Executable,
    /// Executed
    #[serde(rename = "executed")]
    Executed,
    /// Negotiable
    #[serde(rename = "negotiable")]
    Negotiable,
    /// Offered
    #[serde(rename = "offered")]
    Offered,
    /// Policy
    #[serde(rename = "policy")]
    Policy,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Renewed
    #[serde(rename = "renewed")]
    Renewed,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
}

/// This CodeSystem contains FHIR-defined contract party role types.
///
/// System: <http://hl7.org/fhir/contract-party-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractPartyRole {
    /// FLunky
    #[default]
    #[serde(rename = "flunky")]
    Flunky,
}

/// This CodeSystem contains FHIR-defined contract publication status types.
/// Each definition includes usage notes explaining the precedence order in
/// contract publication lifecycle - i.e., while only some stages are required,
/// the order in which they may occur is deterministic.
///
/// System: <http://hl7.org/fhir/contract-publicationstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractPublicationstatus {
    /// Amended
    #[default]
    #[serde(rename = "amended")]
    Amended,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Disputed
    #[serde(rename = "disputed")]
    Disputed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Executable
    #[serde(rename = "executable")]
    Executable,
    /// Executed
    #[serde(rename = "executed")]
    Executed,
    /// Negotiable
    #[serde(rename = "negotiable")]
    Negotiable,
    /// Offered
    #[serde(rename = "offered")]
    Offered,
    /// Policy
    #[serde(rename = "policy")]
    Policy,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Renewed
    #[serde(rename = "renewed")]
    Renewed,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
}

/// This CodeSystem contains FHIR-defined contract Expiration types.
///
/// System: <http://hl7.org/fhir/contract-scope>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractScope {
    /// Policy
    #[default]
    #[serde(rename = "policy")]
    Policy,
}

/// This CodeSystem contains FHIR-defined contract security category types.
///
/// System: <http://hl7.org/fhir/contract-security-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractSecurityCategory {
    /// Policy
    #[default]
    #[serde(rename = "policy")]
    Policy,
}

/// This CodeSystem contains FHIR-defined contract security classification
/// types.
///
/// System: <http://hl7.org/fhir/contract-security-classification>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractSecurityClassification {
    /// Policy
    #[default]
    #[serde(rename = "policy")]
    Policy,
}

/// This CodeSystem contains FHIR-defined contract security control types.
///
/// System: <http://hl7.org/fhir/contract-security-control>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractSecurityControl {
    /// Policy
    #[default]
    #[serde(rename = "policy")]
    Policy,
}

/// This CodeSystem contains FHIR-defined contract status types. Each
/// definition includes usage notes explaining the precedence order in contract
/// lifecycle - i.e., while only some stages are required, the order in which
/// they may occur is deterministic; and a map to comparable FHIR and v.3
/// status codes. It follows guidance about use of status codes in FHIR in [the
/// Status Codes Grid](sc.html).
///
/// System: <http://hl7.org/fhir/contract-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContractStatus {
    /// Amended
    #[default]
    #[serde(rename = "amended")]
    Amended,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Disputed
    #[serde(rename = "disputed")]
    Disputed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Executable
    #[serde(rename = "executable")]
    Executable,
    /// Executed
    #[serde(rename = "executed")]
    Executed,
    /// Negotiable
    #[serde(rename = "negotiable")]
    Negotiable,
    /// Offered
    #[serde(rename = "offered")]
    Offered,
    /// Policy
    #[serde(rename = "policy")]
    Policy,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Renewed
    #[serde(rename = "renewed")]
    Renewed,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
}

/// Used to code the format of the display string.
///
/// System: <http://hl7.org/fhir/contributor-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContributorRole {
    /// Publisher
    #[default]
    #[serde(rename = "publisher")]
    Publisher,
    /// Author/Creator
    #[serde(rename = "author")]
    Author,
    /// Reviewer
    #[serde(rename = "reviewer")]
    Reviewer,
    /// Endorser
    #[serde(rename = "endorser")]
    Endorser,
    /// Editor
    #[serde(rename = "editor")]
    Editor,
    /// Informant
    #[serde(rename = "informant")]
    Informant,
    /// Funder
    #[serde(rename = "funder")]
    Funder,
}

/// Used to code the producer or rule for creating the display string.
///
/// System: <http://hl7.org/fhir/contributor-summary-source>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContributorSummarySource {
    /// Publisher provided
    #[default]
    #[serde(rename = "publisher-data")]
    PublisherData,
    /// Copied from article
    #[serde(rename = "article-copy")]
    ArticleCopy,
    /// Reported by citation manager
    #[serde(rename = "citation-manager")]
    CitationManager,
    /// custom format
    #[serde(rename = "custom")]
    Custom,
}

/// Used to code the format of the display string.
///
/// System: <http://hl7.org/fhir/contributor-summary-style>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContributorSummaryStyle {
    /// First author (full name) et al
    #[default]
    #[serde(rename = "a1full")]
    A1Full,
    /// First author (last name first initials) et al
    #[serde(rename = "a1init")]
    A1Init,
    /// First 3 authors (full name) et al
    #[serde(rename = "a3full")]
    A3Full,
    /// First 3 authors (last name first initials) et al
    #[serde(rename = "a3init")]
    A3Init,
    /// First 6 authors (full name) et al
    #[serde(rename = "a6full")]
    A6Full,
    /// First 6 authors (last name first initials) et al
    #[serde(rename = "a6init")]
    A6Init,
    /// All authors (full name)
    #[serde(rename = "aallfull")]
    Aallfull,
    /// All authors (full name) with and before last author
    #[serde(rename = "aallfullwithand")]
    Aallfullwithand,
    /// All authors (full name) with an ampersand before last author
    #[serde(rename = "aallfullwithampersand")]
    Aallfullwithampersand,
    /// All authors (last name first initials)
    #[serde(rename = "aallinit")]
    Aallinit,
    /// All authors (last name first initials) with and before last author
    #[serde(rename = "aallinitwithand")]
    Aallinitwithand,
    /// All authors (last name first initials) with an ampersand before last
    /// author
    #[serde(rename = "aallinitwithampersand")]
    Aallinitwithampersand,
    /// Contributorship statement listed by person with full names
    #[serde(rename = "contr-full-by-person")]
    ContrFullByPerson,
    /// Contributorship statement listed by person with initials
    #[serde(rename = "contr-init-by-person")]
    ContrInitByPerson,
    /// Contributorship statement listed by contribution with full names
    #[serde(rename = "contr-full-by-contr")]
    ContrFullByContr,
    /// Contributorship statement listed by contribution with initials
    #[serde(rename = "contr-init-by-contr")]
    ContrInitByContr,
}

/// Used to code author list statement, contributorship statement, and such.
///
/// System: <http://hl7.org/fhir/contributor-summary-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContributorSummaryType {
    /// Author string
    #[default]
    #[serde(rename = "author-string")]
    AuthorString,
    /// Contributorship list
    #[serde(rename = "contributorship-list")]
    ContributorshipList,
    /// Contributorship statement
    #[serde(rename = "contributorship-statement")]
    ContributorshipStatement,
    /// Acknowledgment list
    #[serde(rename = "acknowledgement-list")]
    AcknowledgementList,
    /// Acknowledgment statement
    #[serde(rename = "acknowledgment-statement")]
    AcknowledgmentStatement,
    /// Funding statement
    #[serde(rename = "funding-statement")]
    FundingStatement,
    /// Competing interests statement
    #[serde(rename = "competing-interests-statement")]
    CompetingInterestsStatement,
}

/// The type of contributor.
///
/// System: <http://hl7.org/fhir/contributor-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContributorType {
    /// Author
    #[default]
    #[serde(rename = "author")]
    Author,
    /// Editor
    #[serde(rename = "editor")]
    Editor,
    /// Reviewer
    #[serde(rename = "reviewer")]
    Reviewer,
    /// Endorser
    #[serde(rename = "endorser")]
    Endorser,
}

/// The nature of the Coverage details which convey who is paying potentially
/// for health services.
///
/// System: <http://hl7.org/fhir/coverage-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CoverageKind {
    /// Insurance
    #[default]
    #[serde(rename = "insurance")]
    Insurance,
    /// Self-pay
    #[serde(rename = "self-pay")]
    SelfPay,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// This value set includes sample Dates Event Type codes.
///
/// System: <http://hl7.org/fhir/datestype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Datestype {
    /// Card Issued
    #[default]
    #[serde(rename = "card-issued")]
    CardIssued,
    /// Claim Received
    #[serde(rename = "claim-received")]
    ClaimReceived,
    /// Service Expected
    #[serde(rename = "service-expected")]
    ServiceExpected,
}

/// The days of the week.
///
/// System: <http://hl7.org/fhir/days-of-week>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DaysOfWeek {
    /// Monday
    #[default]
    #[serde(rename = "mon")]
    Mon,
    /// Tuesday
    #[serde(rename = "tue")]
    Tue,
    /// Wednesday
    #[serde(rename = "wed")]
    Wed,
    /// Thursday
    #[serde(rename = "thu")]
    Thu,
    /// Friday
    #[serde(rename = "fri")]
    Fri,
    /// Saturday
    #[serde(rename = "sat")]
    Sat,
    /// Sunday
    #[serde(rename = "sun")]
    Sun,
}

/// The method used to define, describe, or determine a characteristic value.
///
/// System: <http://hl7.org/fhir/definition-method>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DefinitionMethod {
    /// Systematic Assessment
    #[default]
    #[serde(rename = "systematic-assessment")]
    SystematicAssessment,
    /// Non-Systematic Assessment
    #[serde(rename = "non-systematic-assessment")]
    NonSystematicAssessment,
    /// Mean
    #[serde(rename = "mean")]
    Mean,
    /// Median
    #[serde(rename = "median")]
    Median,
    /// Mean of Means
    #[serde(rename = "mean-of-mean")]
    MeanOfMean,
    /// Mean of Medians
    #[serde(rename = "mean-of-median")]
    MeanOfMedian,
    /// Median of Means
    #[serde(rename = "median-of-mean")]
    MedianOfMean,
    /// Median of Medians
    #[serde(rename = "median-of-median")]
    MedianOfMedian,
}

/// Indicates the potential degree of impact of the identified issue on the
/// patient.
///
/// System: <http://hl7.org/fhir/detectedissue-severity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DetectedissueSeverity {
    /// High
    #[default]
    #[serde(rename = "high")]
    High,
    /// Moderate
    #[serde(rename = "moderate")]
    Moderate,
    /// Low
    #[serde(rename = "low")]
    Low,
}

/// Indicates the status of the detected issue. This code system contains only
/// status codes that are not already defined and used from the
/// ObservationStatus code system.
///
/// System: <http://hl7.org/fhir/detectedissue-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DetectedissueStatus {
    /// Preliminary
    #[default]
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Mitigated
    #[serde(rename = "mitigated")]
    Mitigated,
}

/// Example value set for Procedure Device Action code (what happened to a
/// device during a procedure.
///
/// System: <http://hl7.org/fhir/device-action>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceAction {
    /// Implanted
    #[default]
    #[serde(rename = "implanted")]
    Implanted,
    /// Explanted
    #[serde(rename = "explanted")]
    Explanted,
    /// Manipulated
    #[serde(rename = "manipulated")]
    Manipulated,
}

/// The record status of the device.
///
/// System: <http://hl7.org/fhir/device-availability-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceAvailabilityStatus {
    /// Lost
    #[default]
    #[serde(rename = "lost")]
    Lost,
    /// Damaged
    #[serde(rename = "damaged")]
    Damaged,
    /// Destroyed
    #[serde(rename = "destroyed")]
    Destroyed,
    /// Available
    #[serde(rename = "available")]
    Available,
}

/// The category of the device.
///
/// System: <http://hl7.org/fhir/device-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceCategory {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// communicating
    #[serde(rename = "communicating")]
    Communicating,
    /// Durable Medical Equipment
    #[serde(rename = "dme")]
    Dme,
    /// Maintenance
    #[serde(rename = "home-use")]
    HomeUse,
    /// Implantable
    #[serde(rename = "implantable")]
    Implantable,
    /// In vitro
    #[serde(rename = "in-vitro")]
    InVitro,
    /// Point of Care
    #[serde(rename = "point-of-care")]
    PointOfCare,
    /// Single Use
    #[serde(rename = "single-use")]
    SingleUse,
    /// Reusable
    #[serde(rename = "reusable")]
    Reusable,
    /// Software
    #[serde(rename = "software")]
    Software,
}

/// The type of relation between devices.
///
/// System: <http://hl7.org/fhir/device-correctiveactionscope>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceCorrectiveactionscope {
    /// Model
    #[default]
    #[serde(rename = "model")]
    Model,
    /// Lot Numbers
    #[serde(rename = "lot-numbers")]
    LotNumbers,
    /// Serial Numbers
    #[serde(rename = "serial-numbers")]
    SerialNumbers,
}

/// The type of name the device is referred by.
///
/// System: <http://hl7.org/fhir/device-nametype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceNametype {
    /// Registered name
    #[default]
    #[serde(rename = "registered-name")]
    RegisteredName,
    /// User Friendly name
    #[serde(rename = "user-friendly-name")]
    UserFriendlyName,
    /// Patient Reported name
    #[serde(rename = "patient-reported-name")]
    PatientReportedName,
}

/// The operation mode of the device.
///
/// System: <http://hl7.org/fhir/device-operation-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceOperationMode {
    /// Normal
    #[default]
    #[serde(rename = "normal")]
    Normal,
    /// Demo
    #[serde(rename = "demo")]
    Demo,
    /// Service
    #[serde(rename = "service")]
    Service,
    /// Maintenance
    #[serde(rename = "maintenance")]
    Maintenance,
    /// Test
    #[serde(rename = "test")]
    Test,
}

/// The production identifier(s) that are expected to appear in the UDI
/// carrier.
///
/// System: <http://hl7.org/fhir/device-productidentifierinudi>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceProductidentifierinudi {
    /// Lot Number
    #[default]
    #[serde(rename = "lot-number")]
    LotNumber,
    /// Manufactured date
    #[serde(rename = "manufactured-date")]
    ManufacturedDate,
    /// Serial Number
    #[serde(rename = "serial-number")]
    SerialNumber,
    /// Expiration date
    #[serde(rename = "expiration-date")]
    ExpirationDate,
    /// Biological source
    #[serde(rename = "biological-source")]
    BiologicalSource,
    /// Software Version
    #[serde(rename = "software-version")]
    SoftwareVersion,
}

/// The kind of standards used by the device.
///
/// System: <http://hl7.org/fhir/device-specification-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceSpecificationCategory {
    /// Communication
    #[default]
    #[serde(rename = "communication")]
    Communication,
    /// Performance
    #[serde(rename = "performance")]
    Performance,
    /// Measurement
    #[serde(rename = "measurement")]
    Measurement,
    /// Risk Class
    #[serde(rename = "risk-class")]
    RiskClass,
    /// Electrical
    #[serde(rename = "electrical")]
    Electrical,
    /// Material
    #[serde(rename = "material")]
    Material,
    /// Exchange
    #[serde(rename = "exchange")]
    Exchange,
}

/// The status of the Device record.
///
/// System: <http://hl7.org/fhir/device-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The operational status of the device.
///
/// System: <http://hl7.org/fhir/deviceassociation-operationstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceassociationOperationstatus {
    /// On
    #[default]
    #[serde(rename = "on")]
    On,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Stand By
    #[serde(rename = "standby")]
    Standby,
    /// Stand By
    #[serde(rename = "defective")]
    Defective,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// DeviceAssociation Status Codes
///
/// System: <http://hl7.org/fhir/deviceassociation-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceassociationStatus {
    /// Implanted
    #[default]
    #[serde(rename = "implanted")]
    Implanted,
    /// Explanted
    #[serde(rename = "explanted")]
    Explanted,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Attached
    #[serde(rename = "attached")]
    Attached,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// DeviceAssociation Status Reason Codes
///
/// System: <http://hl7.org/fhir/deviceassociation-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceassociationStatusReason {
    /// Attached
    #[default]
    #[serde(rename = "attached")]
    Attached,
    /// Disconnected
    #[serde(rename = "disconnected")]
    Disconnected,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
    /// placed
    #[serde(rename = "placed")]
    Placed,
    /// Replaced
    #[serde(rename = "replaced")]
    Replaced,
}

/// The type of regulatory identifier.
///
/// System: <http://hl7.org/fhir/devicedefinition-regulatory-identifier-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DevicedefinitionRegulatoryIdentifierType {
    /// Basic
    #[default]
    #[serde(rename = "basic")]
    Basic,
    /// Master
    #[serde(rename = "master")]
    Master,
    /// License
    #[serde(rename = "license")]
    License,
}

/// The type of relation between devices.
///
/// System: <http://hl7.org/fhir/devicedefinition-relationtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DevicedefinitionRelationtype {
    /// Gateway
    #[default]
    #[serde(rename = "gateway")]
    Gateway,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Previous
    #[serde(rename = "previous")]
    Previous,
}

/// DeviceDispense Status Codes
///
/// System: <http://hl7.org/fhir/devicedispense-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DevicedispenseStatus {
    /// Preparation
    #[default]
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// DeviceDispense Status Reason Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/devicedispense-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DevicedispenseStatusReason {
    /// Out of Stock
    #[default]
    #[serde(rename = "out-of-stock")]
    OutOfStock,
    /// Off market
    #[serde(rename = "off-market")]
    OffMarket,
    /// Contraindication
    #[serde(rename = "contraindication")]
    Contraindication,
    /// Incompatible device
    #[serde(rename = "incompatible-device")]
    IncompatibleDevice,
    /// Order expired
    #[serde(rename = "order-expired")]
    OrderExpired,
    /// Verbal order
    #[serde(rename = "verbal-order")]
    VerbalOrder,
}

/// A coded concept indicating the usage of the device.
///
/// System: <http://hl7.org/fhir/deviceusage-adherence-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceusageAdherenceCode {
    /// Always
    #[default]
    #[serde(rename = "always")]
    Always,
    /// Never
    #[serde(rename = "never")]
    Never,
    /// Sometimes
    #[serde(rename = "sometimes")]
    Sometimes,
}

/// A coded concept indicating the reason for the usage of the device.
///
/// System: <http://hl7.org/fhir/deviceusage-adherence-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceusageAdherenceReason {
    /// Lost
    #[default]
    #[serde(rename = "lost")]
    Lost,
    /// Stolen
    #[serde(rename = "stolen")]
    Stolen,
    /// Prescribed
    #[serde(rename = "prescribed")]
    Prescribed,
    /// Broken
    #[serde(rename = "broken")]
    Broken,
    /// Burned
    #[serde(rename = "burned")]
    Burned,
    /// Forgot
    #[serde(rename = "forgot")]
    Forgot,
}

/// A coded concept indicating the current status of the Device Usage.
///
/// System: <http://hl7.org/fhir/deviceusage-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceusageStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Not done
    #[serde(rename = "not-done")]
    NotDone,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Intended
    #[serde(rename = "intended")]
    Intended,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
}

/// The status of the diagnostic report.
///
/// System: <http://hl7.org/fhir/diagnostic-report-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DiagnosticReportStatus {
    /// Registered
    #[default]
    #[serde(rename = "registered")]
    Registered,
    /// Partial
    #[serde(rename = "partial")]
    Partial,
    /// Preliminary
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Modified
    #[serde(rename = "modified")]
    Modified,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Corrected
    #[serde(rename = "corrected")]
    Corrected,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// How an element value is interpreted when discrimination is evaluated.
///
/// System: <http://hl7.org/fhir/discriminator-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DiscriminatorType {
    /// Value
    #[default]
    #[serde(rename = "value")]
    Value,
    /// Exists
    #[serde(rename = "exists")]
    Exists,
    /// Pattern
    #[serde(rename = "pattern")]
    Pattern,
    /// Type
    #[serde(rename = "type")]
    Type,
    /// Profile
    #[serde(rename = "profile")]
    Profile,
    /// Position
    #[serde(rename = "position")]
    Position,
}

/// Whether the application produces or consumes documents.
///
/// System: <http://hl7.org/fhir/document-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DocumentMode {
    /// Producer
    #[default]
    #[serde(rename = "producer")]
    Producer,
    /// Consumer
    #[serde(rename = "consumer")]
    Consumer,
}

/// The status of the document reference.
///
/// System: <http://hl7.org/fhir/document-reference-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DocumentReferenceStatus {
    /// Current
    #[default]
    #[serde(rename = "current")]
    Current,
    /// Superseded
    #[serde(rename = "superseded")]
    Superseded,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The type of relationship between documents.
///
/// System: <http://hl7.org/fhir/document-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DocumentRelationshipType {
    /// Replaces
    #[default]
    #[serde(rename = "replaces")]
    Replaces,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Signs
    #[serde(rename = "signs")]
    Signs,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
    /// Incorporates
    #[serde(rename = "incorporates")]
    Incorporates,
    /// Summarizes
    #[serde(rename = "summarizes")]
    Summarizes,
}

/// This value set includes Claim Processing Outcome codes.
///
/// System: <http://hl7.org/fhir/eligibility-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EligibilityOutcome {
    /// Queued
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Processing Complete
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial Processing
    #[serde(rename = "partial")]
    Partial,
}

/// A code specifying the types of information being requested.
///
/// System: <http://hl7.org/fhir/eligibilityrequest-purpose>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EligibilityrequestPurpose {
    /// Coverage auth-requirements
    #[default]
    #[serde(rename = "auth-requirements")]
    AuthRequirements,
    /// Coverage benefits
    #[serde(rename = "benefits")]
    Benefits,
    /// Coverage Discovery
    #[serde(rename = "discovery")]
    Discovery,
    /// Coverage Validation
    #[serde(rename = "validation")]
    Validation,
}

/// A code specifying the types of information being requested.
///
/// System: <http://hl7.org/fhir/eligibilityresponse-purpose>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EligibilityresponsePurpose {
    /// Coverage auth-requirements
    #[default]
    #[serde(rename = "auth-requirements")]
    AuthRequirements,
    /// Coverage benefits
    #[serde(rename = "benefits")]
    Benefits,
    /// Coverage Discovery
    #[serde(rename = "discovery")]
    Discovery,
    /// Coverage Validation
    #[serde(rename = "validation")]
    Validation,
}

/// Encounter Condition Use
///
/// System: <http://hl7.org/fhir/encounter-diagnosis-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterDiagnosisUse {
    /// Working
    #[default]
    #[serde(rename = "working")]
    Working,
    /// Final
    #[serde(rename = "final")]
    Final,
}

/// The status of the location.
///
/// System: <http://hl7.org/fhir/encounter-location-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterLocationStatus {
    /// Planned
    #[default]
    #[serde(rename = "planned")]
    Planned,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Reserved
    #[serde(rename = "reserved")]
    Reserved,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
}

/// Encounter Reason Use
///
/// System: <http://hl7.org/fhir/encounter-reason-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterReasonUse {
    /// Chief Complaint
    #[default]
    #[serde(rename = "CC")]
    Cc,
    /// Health Concern
    #[serde(rename = "HC")]
    Hc,
    /// Admitting Diagnosis
    #[serde(rename = "AD")]
    Ad,
    /// Reason for Visit
    #[serde(rename = "RV")]
    Rv,
    /// Health Maintenance (including screening)
    #[serde(rename = "HM")]
    Hm,
}

/// Current state of the encounter.
///
/// System: <http://hl7.org/fhir/encounter-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterStatus {
    /// Planned
    #[default]
    #[serde(rename = "planned")]
    Planned,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Discharged
    #[serde(rename = "discharged")]
    Discharged,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Discontinued
    #[serde(rename = "discontinued")]
    Discontinued,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The environment type of the endpoint.
///
/// System: <http://hl7.org/fhir/endpoint-environment>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EndpointEnvironment {
    /// Production
    #[default]
    #[serde(rename = "prod")]
    Prod,
    /// Staging
    #[serde(rename = "staging")]
    Staging,
    /// Development
    #[serde(rename = "dev")]
    Dev,
    /// Test
    #[serde(rename = "test")]
    Test,
    /// Training
    #[serde(rename = "train")]
    Train,
}

/// The status of the endpoint.
///
/// System: <http://hl7.org/fhir/endpoint-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EndpointStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// This value set includes Claim Processing Outcome codes.
///
/// System: <http://hl7.org/fhir/enrollment-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EnrollmentOutcome {
    /// Queued
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Processing Complete
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial Processing
    #[serde(rename = "partial")]
    Partial,
}

/// The status of the episode of care.
///
/// System: <http://hl7.org/fhir/episode-of-care-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EpisodeOfCareStatus {
    /// Planned
    #[default]
    #[serde(rename = "planned")]
    Planned,
    /// Waitlist
    #[serde(rename = "waitlist")]
    Waitlist,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "onhold")]
    Onhold,
    /// Finished
    #[serde(rename = "finished")]
    Finished,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The mode of a message capability statement.
///
/// System: <http://hl7.org/fhir/event-capability-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EventCapabilityMode {
    /// Sender
    #[default]
    #[serde(rename = "sender")]
    Sender,
    /// Receiver
    #[serde(rename = "receiver")]
    Receiver,
}

/// Codes identifying the lifecycle stage of an event.
///
/// System: <http://hl7.org/fhir/event-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EventStatus {
    /// Preparation
    #[default]
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Not Done
    #[serde(rename = "not-done")]
    NotDone,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Real-world event relating to the schedule.
///
/// System: <http://hl7.org/fhir/event-timing>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EventTiming {
    /// Morning
    #[default]
    #[serde(rename = "MORN")]
    Morn,
    /// Early Morning
    #[serde(rename = "MORN.early")]
    MornEarly,
    /// Late Morning
    #[serde(rename = "MORN.late")]
    MornLate,
    /// Noon
    #[serde(rename = "NOON")]
    Noon,
    /// Afternoon
    #[serde(rename = "AFT")]
    Aft,
    /// Early Afternoon
    #[serde(rename = "AFT.early")]
    AftEarly,
    /// Late Afternoon
    #[serde(rename = "AFT.late")]
    AftLate,
    /// Evening
    #[serde(rename = "EVE")]
    Eve,
    /// Early Evening
    #[serde(rename = "EVE.early")]
    EveEarly,
    /// Late Evening
    #[serde(rename = "EVE.late")]
    EveLate,
    /// Night
    #[serde(rename = "NIGHT")]
    Night,
    /// After Sleep
    #[serde(rename = "PHS")]
    Phs,
    /// Immediate
    #[serde(rename = "IMD")]
    Imd,
    /// HS
    #[serde(rename = "HS")]
    Hs,
    /// WAKE
    #[serde(rename = "WAKE")]
    Wake,
    /// C
    #[serde(rename = "C")]
    C,
    /// CM
    #[serde(rename = "CM")]
    Cm,
    /// CD
    #[serde(rename = "CD")]
    Cd,
    /// CV
    #[serde(rename = "CV")]
    Cv,
    /// AC
    #[serde(rename = "AC")]
    Ac,
    /// ACM
    #[serde(rename = "ACM")]
    Acm,
    /// ACD
    #[serde(rename = "ACD")]
    Acd,
    /// ACV
    #[serde(rename = "ACV")]
    Acv,
    /// PC
    #[serde(rename = "PC")]
    Pc,
    /// PCM
    #[serde(rename = "PCM")]
    Pcm,
    /// PCD
    #[serde(rename = "PCD")]
    Pcd,
    /// PCV
    #[serde(rename = "PCV")]
    Pcv,
}

/// Commonly used classifiers for evidence sets.
///
/// System: <http://hl7.org/fhir/evidence-classifier-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EvidenceClassifierCode {
    /// COVID-19 specific article
    #[default]
    #[serde(rename = "COVID19Specific")]
    Covid19Specific,
    /// COVID-19 relevant (but not specific) article
    #[serde(rename = "COVID19Relevant")]
    Covid19Relevant,
    /// COVID-19 human data in population, exposure, or outcome
    #[serde(rename = "COVID19HumanResearch")]
    Covid19HumanResearch,
    /// Article includes original research
    #[serde(rename = "OriginalResearch")]
    OriginalResearch,
    /// Article includes synthesis of research
    #[serde(rename = "ResearchSynthesis")]
    ResearchSynthesis,
    /// Article includes guideline
    #[serde(rename = "Guideline")]
    Guideline,
    /// Article provides protocol without results
    #[serde(rename = "ResearchProtocol")]
    ResearchProtocol,
    /// Article is neither research nor guideline
    #[serde(rename = "NotResearchNotGuideline")]
    NotResearchNotGuideline,
    /// Article about treatment
    #[serde(rename = "Treatment")]
    Treatment,
    /// Article about prevention and control
    #[serde(rename = "PreventionAndControl")]
    PreventionAndControl,
    /// Article about diagnosis
    #[serde(rename = "Diagnosis")]
    Diagnosis,
    /// Article about prognosis or prediction
    #[serde(rename = "PrognosisPrediction")]
    PrognosisPrediction,
    /// Rated as yes, affirmative, positive, present, or include
    #[serde(rename = "RatedAsYes")]
    RatedAsYes,
    /// Rated as no, negative, absent, or exclude
    #[serde(rename = "RatedAsNo")]
    RatedAsNo,
    /// Not rated, not assessed
    #[serde(rename = "NotAssessed")]
    NotAssessed,
    /// classified as randomized controlled trial
    #[serde(rename = "RatedAsRCT")]
    RatedAsRct,
    /// classified as nonrandomized controlled trial (experimental)
    #[serde(rename = "RatedAsControlledTrial")]
    RatedAsControlledTrial,
    /// classified as comparative cohort study (observational)
    #[serde(rename = "RatedAsComparativeCohort")]
    RatedAsComparativeCohort,
    /// classified as case-control study
    #[serde(rename = "RatedAsCaseControl")]
    RatedAsCaseControl,
    /// classified as uncontrolled cohort (case series)
    #[serde(rename = "RatedAsUncontrolledSeries")]
    RatedAsUncontrolledSeries,
    /// classified as mixed-methods study
    #[serde(rename = "RatedAsMixedMethods")]
    RatedAsMixedMethods,
    /// classified as other concept (not elsewhere classified)
    #[serde(rename = "RatedAsOther")]
    RatedAsOther,
    /// Risk of bias assessment
    #[serde(rename = "RiskOfBias")]
    RiskOfBias,
    /// No blinding
    #[serde(rename = "NoBlinding")]
    NoBlinding,
    /// Allocation concealment not stated
    #[serde(rename = "AllocConcealNotStated")]
    AllocConcealNotStated,
    /// Early trial termination
    #[serde(rename = "EarlyTrialTermination")]
    EarlyTrialTermination,
    /// No intention-to-treat analysis
    #[serde(rename = "NoITT")]
    NoItt,
    /// Preprint (not final publication)
    #[serde(rename = "Preprint")]
    Preprint,
    /// Preliminary analysis
    #[serde(rename = "PreliminaryAnalysis")]
    PreliminaryAnalysis,
    /// Baseline imbalances
    #[serde(rename = "BaselineImbalance")]
    BaselineImbalance,
    /// Subgroup analysis
    #[serde(rename = "SubgroupAnalysis")]
    SubgroupAnalysis,
}

/// Evidence Report Section Type.
///
/// System: <http://hl7.org/fhir/evidence-report-section>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EvidenceReportSection {
    /// Evidence Results
    #[default]
    #[serde(rename = "Evidence")]
    Evidence,
    /// Evidence Results for the intervention exposure only
    #[serde(rename = "Intervention-group-alone-Evidence")]
    InterventionGroupAloneEvidence,
    /// Evidence Results for comparison of Intervention and Control
    #[serde(rename = "Intervention-vs-Control-Evidence")]
    InterventionVsControlEvidence,
    /// Evidence Results for the control exposure only
    #[serde(rename = "Control-group-alone-Evidence")]
    ControlGroupAloneEvidence,
    /// Evidence Variables used
    #[serde(rename = "EvidenceVariable")]
    EvidenceVariable,
    /// Evidence Variables actually observed
    #[serde(rename = "EvidenceVariable-observed")]
    EvidenceVariableObserved,
    /// Evidence Variables intended for interpretation
    #[serde(rename = "EvidenceVariable-intended")]
    EvidenceVariableIntended,
    /// Evidence Variable in variable role Population
    #[serde(rename = "EvidenceVariable-population")]
    EvidenceVariablePopulation,
    /// Evidence Variable in variable role Exposure
    #[serde(rename = "EvidenceVariable-exposure")]
    EvidenceVariableExposure,
    /// Evidence Variable in variable role Outcome (MeasuredVariable)
    #[serde(rename = "EvidenceVariable-outcome")]
    EvidenceVariableOutcome,
    /// Efficacy-outcomes
    #[serde(rename = "Efficacy-outcomes")]
    EfficacyOutcomes,
    /// Harms outcomes
    #[serde(rename = "Harms-outcomes")]
    HarmsOutcomes,
    /// Sample Size
    #[serde(rename = "SampleSize")]
    SampleSize,
    /// References
    #[serde(rename = "References")]
    References,
    /// Assertion
    #[serde(rename = "Assertion")]
    Assertion,
    /// Reasons
    #[serde(rename = "Reasons")]
    Reasons,
    /// Certainty of Evidence
    #[serde(rename = "Certainty-of-Evidence")]
    CertaintyOfEvidence,
    /// Evidence Classifier section
    #[serde(rename = "Evidence-Classifier")]
    EvidenceClassifier,
    /// Warnings
    #[serde(rename = "Warnings")]
    Warnings,
    /// Text Summary
    #[serde(rename = "Text-Summary")]
    TextSummary,
    /// Summary of Body of Evidence Findings
    #[serde(rename = "SummaryOfBodyOfEvidenceFindings")]
    SummaryOfBodyOfEvidenceFindings,
    /// Summary of Individual Study Findings
    #[serde(rename = "SummaryOfIndividualStudyFindings")]
    SummaryOfIndividualStudyFindings,
    /// Header
    #[serde(rename = "Header")]
    Header,
    /// Tables
    #[serde(rename = "Tables")]
    Tables,
    /// Table
    #[serde(rename = "Table")]
    Table,
    /// Row Headers
    #[serde(rename = "Row-Headers")]
    RowHeaders,
    /// Column Header
    #[serde(rename = "Column-Header")]
    ColumnHeader,
    /// Column Headers
    #[serde(rename = "Column-Headers")]
    ColumnHeaders,
}

/// The kind of report, such as grouping of classifiers, search results, or
/// human-compiled expression.
///
/// System: <http://hl7.org/fhir/evidence-report-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EvidenceReportType {
    /// Classification
    #[default]
    #[serde(rename = "classification")]
    Classification,
    /// Search Results
    #[serde(rename = "search-results")]
    SearchResults,
    /// Resource Compilation
    #[serde(rename = "resources-compiled")]
    ResourcesCompiled,
    /// Structured Text
    #[serde(rename = "text-structured")]
    TextStructured,
}

/// The event used as a base point (reference point) in time.
///
/// System: <http://hl7.org/fhir/evidence-variable-event>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EvidenceVariableEvent {
    /// Study Start
    #[default]
    #[serde(rename = "study-start")]
    StudyStart,
    /// Start of Treatment
    #[serde(rename = "treatment-start")]
    TreatmentStart,
    /// Detection of Condition
    #[serde(rename = "condition-detection")]
    ConditionDetection,
    /// Treatment of Condition
    #[serde(rename = "condition-treatment")]
    ConditionTreatment,
    /// Hospital Admission
    #[serde(rename = "hospital-admission")]
    HospitalAdmission,
    /// Hospital Discharge
    #[serde(rename = "hospital-discharge")]
    HospitalDischarge,
    /// Operative Procedure
    #[serde(rename = "operative-procedure")]
    OperativeProcedure,
}

/// This value set includes sample ICD-10 Procedure codes.
///
/// System: <http://hl7.org/fhir/sid/ex-icd-10-procedures>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExIcd10Procedures {
    /// PROC-1
    #[default]
    #[serde(rename = "123001")]
    N123001,
    /// PROC-2
    #[serde(rename = "123002")]
    N123002,
    /// PROC-3
    #[serde(rename = "123003")]
    N123003,
}

/// This is an example code system that includes all the ACME codes for
/// serum/plasma cholesterol from v2.36.
///
/// System: <http://hl7.org/fhir/CodeSystem/example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Example {
    /// Cholesterol [Moles/Volume]
    #[default]
    #[serde(rename = "14647-2")]
    N146472,
    /// Cholesterol [Mass/Volume]
    #[serde(rename = "2093-3")]
    N20933,
    /// Cholesterol [Mass Or Moles/Volume]
    #[serde(rename = "35200-5")]
    N352005,
    /// Cholesterol [Percentile]
    #[serde(rename = "9342-7")]
    N93427,
}

/// This is an example code system that illustrates usage of the metadata
/// resource elements introduced in R5
///
/// System: <http://hl7.org/fhir/CodeSystem/example-metadata>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExampleMetadata {
    /// A
    #[default]
    #[serde(rename = "A")]
    A,
    /// B
    #[serde(rename = "B")]
    B,
    /// C
    #[serde(rename = "C")]
    C,
}

/// This is an example code system that illustrates usage of the metadata
/// resource elements introduced in R5
///
/// System: <http://hl7.org/fhir/CodeSystem/example-metadata-2>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExampleMetadata2 {
    /// A
    #[default]
    #[serde(rename = "A")]
    A,
    /// B
    #[serde(rename = "B")]
    B,
    /// C
    #[serde(rename = "C")]
    C,
    /// D
    #[serde(rename = "D")]
    D,
}

/// The type of actor - system or human.
///
/// System: <http://hl7.org/fhir/examplescenario-actor-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExamplescenarioActorType {
    /// Person
    #[default]
    #[serde(rename = "person")]
    Person,
    /// System
    #[serde(rename = "system")]
    System,
}

/// A code specifying the state of the resource instance.
///
/// System: <http://hl7.org/fhir/explanationofbenefit-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExplanationofbenefitStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// How an extension context is interpreted.
///
/// System: <http://hl7.org/fhir/extension-context-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExtensionContextType {
    /// FHIRPath
    #[default]
    #[serde(rename = "fhirpath")]
    Fhirpath,
    /// Element ID
    #[serde(rename = "element")]
    Element,
    /// Extension URL
    #[serde(rename = "extension")]
    Extension,
}

/// This value set includes coded concepts not well covered in any of the
/// included valuesets.
///
/// System: <http://hl7.org/fhir/extra-activity-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExtraActivityType {
    /// aggregate
    #[default]
    #[serde(rename = "aggregate")]
    Aggregate,
    /// compose
    #[serde(rename = "compose")]
    Compose,
    /// label
    #[serde(rename = "label")]
    Label,
}

/// FHIR Format types
///
/// System: <http://hl7.org/fhir/fhir-format-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FhirFormatType {
    /// XML
    #[default]
    #[serde(rename = "xml")]
    Xml,
    /// JSON
    #[serde(rename = "json")]
    Json,
    /// TTL
    #[serde(rename = "ttl")]
    Ttl,
}

/// An old resource name no longer used in this version of FHIR (deleted or
/// renamed).
///
/// System: <http://hl7.org/fhir/fhir-old-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FhirOldTypes {
    /// BodySite
    #[default]
    #[serde(rename = "BodySite")]
    BodySite,
    /// CatalogEntry
    #[serde(rename = "CatalogEntry")]
    CatalogEntry,
    /// Conformance
    #[serde(rename = "Conformance")]
    Conformance,
    /// DataElement
    #[serde(rename = "DataElement")]
    DataElement,
    /// DeviceComponent
    #[serde(rename = "DeviceComponent")]
    DeviceComponent,
    /// DeviceUseRequest
    #[serde(rename = "DeviceUseRequest")]
    DeviceUseRequest,
    /// DeviceUseStatement
    #[serde(rename = "DeviceUseStatement")]
    DeviceUseStatement,
    /// DiagnosticOrder
    #[serde(rename = "DiagnosticOrder")]
    DiagnosticOrder,
    /// DocumentManifest
    #[serde(rename = "DocumentManifest")]
    DocumentManifest,
    /// EffectEvidenceSynthesis
    #[serde(rename = "EffectEvidenceSynthesis")]
    EffectEvidenceSynthesis,
    /// EligibilityRequest
    #[serde(rename = "EligibilityRequest")]
    EligibilityRequest,
    /// EligibilityResponse
    #[serde(rename = "EligibilityResponse")]
    EligibilityResponse,
    /// ExpansionProfile
    #[serde(rename = "ExpansionProfile")]
    ExpansionProfile,
    /// ImagingManifest
    #[serde(rename = "ImagingManifest")]
    ImagingManifest,
    /// ImagingObjectSelection
    #[serde(rename = "ImagingObjectSelection")]
    ImagingObjectSelection,
    /// Media
    #[serde(rename = "Media")]
    Media,
    /// MedicationOrder
    #[serde(rename = "MedicationOrder")]
    MedicationOrder,
    /// MedicationUsage
    #[serde(rename = "MedicationUsage")]
    MedicationUsage,
    /// MedicinalProduct
    #[serde(rename = "MedicinalProduct")]
    MedicinalProduct,
    /// MedicinalProductAuthorization
    #[serde(rename = "MedicinalProductAuthorization")]
    MedicinalProductAuthorization,
    /// MedicinalProductContraindication
    #[serde(rename = "MedicinalProductContraindication")]
    MedicinalProductContraindication,
    /// MedicinalProductIndication
    #[serde(rename = "MedicinalProductIndication")]
    MedicinalProductIndication,
    /// MedicinalProductIngredient
    #[serde(rename = "MedicinalProductIngredient")]
    MedicinalProductIngredient,
    /// MedicinalProductInteraction
    #[serde(rename = "MedicinalProductInteraction")]
    MedicinalProductInteraction,
    /// MedicinalProductManufactured
    #[serde(rename = "MedicinalProductManufactured")]
    MedicinalProductManufactured,
    /// MedicinalProductPackaged
    #[serde(rename = "MedicinalProductPackaged")]
    MedicinalProductPackaged,
    /// MedicinalProductPharmaceutical
    #[serde(rename = "MedicinalProductPharmaceutical")]
    MedicinalProductPharmaceutical,
    /// MedicinalProductUndesirableEffect
    #[serde(rename = "MedicinalProductUndesirableEffect")]
    MedicinalProductUndesirableEffect,
    /// Order
    #[serde(rename = "Order")]
    Order,
    /// OrderResponse
    #[serde(rename = "OrderResponse")]
    OrderResponse,
    /// ProcedureRequest
    #[serde(rename = "ProcedureRequest")]
    ProcedureRequest,
    /// ProcessRequest
    #[serde(rename = "ProcessRequest")]
    ProcessRequest,
    /// ProcessResponse
    #[serde(rename = "ProcessResponse")]
    ProcessResponse,
    /// ReferralRequest
    #[serde(rename = "ReferralRequest")]
    ReferralRequest,
    /// RequestGroup
    #[serde(rename = "RequestGroup")]
    RequestGroup,
    /// ResearchDefinition
    #[serde(rename = "ResearchDefinition")]
    ResearchDefinition,
    /// ResearchElementDefinition
    #[serde(rename = "ResearchElementDefinition")]
    ResearchElementDefinition,
    /// RiskEvidenceSynthesis
    #[serde(rename = "RiskEvidenceSynthesis")]
    RiskEvidenceSynthesis,
    /// Sequence
    #[serde(rename = "Sequence")]
    Sequence,
    /// ServiceDefinition
    #[serde(rename = "ServiceDefinition")]
    ServiceDefinition,
    /// SubstanceSpecification
    #[serde(rename = "SubstanceSpecification")]
    SubstanceSpecification,
}

/// One of the types defined as part of this version of FHIR.
///
/// System: <http://hl7.org/fhir/fhir-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FhirTypes {
    /// Base
    #[default]
    #[serde(rename = "Base")]
    Base,
    /// Element
    #[serde(rename = "Element")]
    Element,
    /// BackboneElement
    #[serde(rename = "BackboneElement")]
    BackboneElement,
    /// DataType
    #[serde(rename = "DataType")]
    DataType,
    /// Address
    #[serde(rename = "Address")]
    Address,
    /// Annotation
    #[serde(rename = "Annotation")]
    Annotation,
    /// Attachment
    #[serde(rename = "Attachment")]
    Attachment,
    /// Availability
    #[serde(rename = "Availability")]
    Availability,
    /// BackboneType
    #[serde(rename = "BackboneType")]
    BackboneType,
    /// Dosage
    #[serde(rename = "Dosage")]
    Dosage,
    /// ElementDefinition
    #[serde(rename = "ElementDefinition")]
    ElementDefinition,
    /// MarketingStatus
    #[serde(rename = "MarketingStatus")]
    MarketingStatus,
    /// ProductShelfLife
    #[serde(rename = "ProductShelfLife")]
    ProductShelfLife,
    /// Timing
    #[serde(rename = "Timing")]
    Timing,
    /// CodeableConcept
    #[serde(rename = "CodeableConcept")]
    CodeableConcept,
    /// CodeableReference
    #[serde(rename = "CodeableReference")]
    CodeableReference,
    /// Coding
    #[serde(rename = "Coding")]
    Coding,
    /// ContactDetail
    #[serde(rename = "ContactDetail")]
    ContactDetail,
    /// ContactPoint
    #[serde(rename = "ContactPoint")]
    ContactPoint,
    /// Contributor
    #[serde(rename = "Contributor")]
    Contributor,
    /// DataRequirement
    #[serde(rename = "DataRequirement")]
    DataRequirement,
    /// Expression
    #[serde(rename = "Expression")]
    Expression,
    /// ExtendedContactDetail
    #[serde(rename = "ExtendedContactDetail")]
    ExtendedContactDetail,
    /// Extension
    #[serde(rename = "Extension")]
    Extension,
    /// HumanName
    #[serde(rename = "HumanName")]
    HumanName,
    /// Identifier
    #[serde(rename = "Identifier")]
    Identifier,
    /// Meta
    #[serde(rename = "Meta")]
    Meta,
    /// MonetaryComponent
    #[serde(rename = "MonetaryComponent")]
    MonetaryComponent,
    /// Money
    #[serde(rename = "Money")]
    Money,
    /// Narrative
    #[serde(rename = "Narrative")]
    Narrative,
    /// ParameterDefinition
    #[serde(rename = "ParameterDefinition")]
    ParameterDefinition,
    /// Period
    #[serde(rename = "Period")]
    Period,
    /// PrimitiveType
    #[serde(rename = "PrimitiveType")]
    PrimitiveType,
    /// base64Binary
    #[serde(rename = "base64Binary")]
    Base64Binary,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// date
    #[serde(rename = "date")]
    Date,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// instant
    #[serde(rename = "instant")]
    Instant,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// positiveInt
    #[serde(rename = "positiveInt")]
    PositiveInt,
    /// unsignedInt
    #[serde(rename = "unsignedInt")]
    UnsignedInt,
    /// integer64
    #[serde(rename = "integer64")]
    Integer64,
    /// string
    #[serde(rename = "string")]
    String,
    /// code
    #[serde(rename = "code")]
    Code,
    /// id
    #[serde(rename = "id")]
    Id,
    /// markdown
    #[serde(rename = "markdown")]
    Markdown,
    /// time
    #[serde(rename = "time")]
    Time,
    /// uri
    #[serde(rename = "uri")]
    Uri,
    /// canonical
    #[serde(rename = "canonical")]
    Canonical,
    /// oid
    #[serde(rename = "oid")]
    Oid,
    /// url
    #[serde(rename = "url")]
    Url,
    /// uuid
    #[serde(rename = "uuid")]
    Uuid,
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
    /// Age
    #[serde(rename = "Age")]
    Age,
    /// Count
    #[serde(rename = "Count")]
    Count,
    /// Distance
    #[serde(rename = "Distance")]
    Distance,
    /// Duration
    #[serde(rename = "Duration")]
    Duration,
    /// Range
    #[serde(rename = "Range")]
    Range,
    /// Ratio
    #[serde(rename = "Ratio")]
    Ratio,
    /// RatioRange
    #[serde(rename = "RatioRange")]
    RatioRange,
    /// Reference
    #[serde(rename = "Reference")]
    Reference,
    /// RelatedArtifact
    #[serde(rename = "RelatedArtifact")]
    RelatedArtifact,
    /// SampledData
    #[serde(rename = "SampledData")]
    SampledData,
    /// Signature
    #[serde(rename = "Signature")]
    Signature,
    /// TriggerDefinition
    #[serde(rename = "TriggerDefinition")]
    TriggerDefinition,
    /// UsageContext
    #[serde(rename = "UsageContext")]
    UsageContext,
    /// VirtualServiceDetail
    #[serde(rename = "VirtualServiceDetail")]
    VirtualServiceDetail,
    /// xhtml
    #[serde(rename = "xhtml")]
    Xhtml,
    /// Resource
    #[serde(rename = "Resource")]
    Resource,
    /// Binary
    #[serde(rename = "Binary")]
    Binary,
    /// Bundle
    #[serde(rename = "Bundle")]
    Bundle,
    /// DomainResource
    #[serde(rename = "DomainResource")]
    DomainResource,
    /// Account
    #[serde(rename = "Account")]
    Account,
    /// ActivityDefinition
    #[serde(rename = "ActivityDefinition")]
    ActivityDefinition,
    /// ActorDefinition
    #[serde(rename = "ActorDefinition")]
    ActorDefinition,
    /// AdministrableProductDefinition
    #[serde(rename = "AdministrableProductDefinition")]
    AdministrableProductDefinition,
    /// AdverseEvent
    #[serde(rename = "AdverseEvent")]
    AdverseEvent,
    /// AllergyIntolerance
    #[serde(rename = "AllergyIntolerance")]
    AllergyIntolerance,
    /// Appointment
    #[serde(rename = "Appointment")]
    Appointment,
    /// AppointmentResponse
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    /// ArtifactAssessment
    #[serde(rename = "ArtifactAssessment")]
    ArtifactAssessment,
    /// AuditEvent
    #[serde(rename = "AuditEvent")]
    AuditEvent,
    /// Basic
    #[serde(rename = "Basic")]
    Basic,
    /// BiologicallyDerivedProduct
    #[serde(rename = "BiologicallyDerivedProduct")]
    BiologicallyDerivedProduct,
    /// BiologicallyDerivedProductDispense
    #[serde(rename = "BiologicallyDerivedProductDispense")]
    BiologicallyDerivedProductDispense,
    /// BodyStructure
    #[serde(rename = "BodyStructure")]
    BodyStructure,
    /// CanonicalResource
    #[serde(rename = "CanonicalResource")]
    CanonicalResource,
    /// CapabilityStatement
    #[serde(rename = "CapabilityStatement")]
    CapabilityStatement,
    /// CarePlan
    #[serde(rename = "CarePlan")]
    CarePlan,
    /// CareTeam
    #[serde(rename = "CareTeam")]
    CareTeam,
    /// ChargeItem
    #[serde(rename = "ChargeItem")]
    ChargeItem,
    /// ChargeItemDefinition
    #[serde(rename = "ChargeItemDefinition")]
    ChargeItemDefinition,
    /// Citation
    #[serde(rename = "Citation")]
    Citation,
    /// Claim
    #[serde(rename = "Claim")]
    Claim,
    /// ClaimResponse
    #[serde(rename = "ClaimResponse")]
    ClaimResponse,
    /// ClinicalImpression
    #[serde(rename = "ClinicalImpression")]
    ClinicalImpression,
    /// ClinicalUseDefinition
    #[serde(rename = "ClinicalUseDefinition")]
    ClinicalUseDefinition,
    /// CodeSystem
    #[serde(rename = "CodeSystem")]
    CodeSystem,
    /// Communication
    #[serde(rename = "Communication")]
    Communication,
    /// CommunicationRequest
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    /// CompartmentDefinition
    #[serde(rename = "CompartmentDefinition")]
    CompartmentDefinition,
    /// Composition
    #[serde(rename = "Composition")]
    Composition,
    /// ConceptMap
    #[serde(rename = "ConceptMap")]
    ConceptMap,
    /// Condition
    #[serde(rename = "Condition")]
    Condition,
    /// ConditionDefinition
    #[serde(rename = "ConditionDefinition")]
    ConditionDefinition,
    /// Consent
    #[serde(rename = "Consent")]
    Consent,
    /// Contract
    #[serde(rename = "Contract")]
    Contract,
    /// Coverage
    #[serde(rename = "Coverage")]
    Coverage,
    /// CoverageEligibilityRequest
    #[serde(rename = "CoverageEligibilityRequest")]
    CoverageEligibilityRequest,
    /// CoverageEligibilityResponse
    #[serde(rename = "CoverageEligibilityResponse")]
    CoverageEligibilityResponse,
    /// DetectedIssue
    #[serde(rename = "DetectedIssue")]
    DetectedIssue,
    /// Device
    #[serde(rename = "Device")]
    Device,
    /// DeviceAssociation
    #[serde(rename = "DeviceAssociation")]
    DeviceAssociation,
    /// DeviceDefinition
    #[serde(rename = "DeviceDefinition")]
    DeviceDefinition,
    /// DeviceDispense
    #[serde(rename = "DeviceDispense")]
    DeviceDispense,
    /// DeviceMetric
    #[serde(rename = "DeviceMetric")]
    DeviceMetric,
    /// DeviceRequest
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    /// DeviceUsage
    #[serde(rename = "DeviceUsage")]
    DeviceUsage,
    /// DiagnosticReport
    #[serde(rename = "DiagnosticReport")]
    DiagnosticReport,
    /// DocumentReference
    #[serde(rename = "DocumentReference")]
    DocumentReference,
    /// Encounter
    #[serde(rename = "Encounter")]
    Encounter,
    /// EncounterHistory
    #[serde(rename = "EncounterHistory")]
    EncounterHistory,
    /// Endpoint
    #[serde(rename = "Endpoint")]
    Endpoint,
    /// EnrollmentRequest
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    /// EnrollmentResponse
    #[serde(rename = "EnrollmentResponse")]
    EnrollmentResponse,
    /// EpisodeOfCare
    #[serde(rename = "EpisodeOfCare")]
    EpisodeOfCare,
    /// EventDefinition
    #[serde(rename = "EventDefinition")]
    EventDefinition,
    /// Evidence
    #[serde(rename = "Evidence")]
    Evidence,
    /// EvidenceReport
    #[serde(rename = "EvidenceReport")]
    EvidenceReport,
    /// EvidenceVariable
    #[serde(rename = "EvidenceVariable")]
    EvidenceVariable,
    /// ExampleScenario
    #[serde(rename = "ExampleScenario")]
    ExampleScenario,
    /// ExplanationOfBenefit
    #[serde(rename = "ExplanationOfBenefit")]
    ExplanationOfBenefit,
    /// FamilyMemberHistory
    #[serde(rename = "FamilyMemberHistory")]
    FamilyMemberHistory,
    /// Flag
    #[serde(rename = "Flag")]
    Flag,
    /// FormularyItem
    #[serde(rename = "FormularyItem")]
    FormularyItem,
    /// GenomicStudy
    #[serde(rename = "GenomicStudy")]
    GenomicStudy,
    /// Goal
    #[serde(rename = "Goal")]
    Goal,
    /// GraphDefinition
    #[serde(rename = "GraphDefinition")]
    GraphDefinition,
    /// Group
    #[serde(rename = "Group")]
    Group,
    /// GuidanceResponse
    #[serde(rename = "GuidanceResponse")]
    GuidanceResponse,
    /// HealthcareService
    #[serde(rename = "HealthcareService")]
    HealthcareService,
    /// ImagingSelection
    #[serde(rename = "ImagingSelection")]
    ImagingSelection,
    /// ImagingStudy
    #[serde(rename = "ImagingStudy")]
    ImagingStudy,
    /// Immunization
    #[serde(rename = "Immunization")]
    Immunization,
    /// ImmunizationEvaluation
    #[serde(rename = "ImmunizationEvaluation")]
    ImmunizationEvaluation,
    /// ImmunizationRecommendation
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    /// ImplementationGuide
    #[serde(rename = "ImplementationGuide")]
    ImplementationGuide,
    /// Ingredient
    #[serde(rename = "Ingredient")]
    Ingredient,
    /// InsurancePlan
    #[serde(rename = "InsurancePlan")]
    InsurancePlan,
    /// InventoryItem
    #[serde(rename = "InventoryItem")]
    InventoryItem,
    /// InventoryReport
    #[serde(rename = "InventoryReport")]
    InventoryReport,
    /// Invoice
    #[serde(rename = "Invoice")]
    Invoice,
    /// Library
    #[serde(rename = "Library")]
    Library,
    /// Linkage
    #[serde(rename = "Linkage")]
    Linkage,
    /// List
    #[serde(rename = "List")]
    List,
    /// Location
    #[serde(rename = "Location")]
    Location,
    /// ManufacturedItemDefinition
    #[serde(rename = "ManufacturedItemDefinition")]
    ManufacturedItemDefinition,
    /// Measure
    #[serde(rename = "Measure")]
    Measure,
    /// MeasureReport
    #[serde(rename = "MeasureReport")]
    MeasureReport,
    /// Medication
    #[serde(rename = "Medication")]
    Medication,
    /// MedicationAdministration
    #[serde(rename = "MedicationAdministration")]
    MedicationAdministration,
    /// MedicationDispense
    #[serde(rename = "MedicationDispense")]
    MedicationDispense,
    /// MedicationKnowledge
    #[serde(rename = "MedicationKnowledge")]
    MedicationKnowledge,
    /// MedicationRequest
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    /// MedicationStatement
    #[serde(rename = "MedicationStatement")]
    MedicationStatement,
    /// MedicinalProductDefinition
    #[serde(rename = "MedicinalProductDefinition")]
    MedicinalProductDefinition,
    /// MessageDefinition
    #[serde(rename = "MessageDefinition")]
    MessageDefinition,
    /// MessageHeader
    #[serde(rename = "MessageHeader")]
    MessageHeader,
    /// MetadataResource
    #[serde(rename = "MetadataResource")]
    MetadataResource,
    /// MolecularSequence
    #[serde(rename = "MolecularSequence")]
    MolecularSequence,
    /// NamingSystem
    #[serde(rename = "NamingSystem")]
    NamingSystem,
    /// NutritionIntake
    #[serde(rename = "NutritionIntake")]
    NutritionIntake,
    /// NutritionOrder
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    /// NutritionProduct
    #[serde(rename = "NutritionProduct")]
    NutritionProduct,
    /// Observation
    #[serde(rename = "Observation")]
    Observation,
    /// ObservationDefinition
    #[serde(rename = "ObservationDefinition")]
    ObservationDefinition,
    /// OperationDefinition
    #[serde(rename = "OperationDefinition")]
    OperationDefinition,
    /// OperationOutcome
    #[serde(rename = "OperationOutcome")]
    OperationOutcome,
    /// Organization
    #[serde(rename = "Organization")]
    Organization,
    /// OrganizationAffiliation
    #[serde(rename = "OrganizationAffiliation")]
    OrganizationAffiliation,
    /// PackagedProductDefinition
    #[serde(rename = "PackagedProductDefinition")]
    PackagedProductDefinition,
    /// Patient
    #[serde(rename = "Patient")]
    Patient,
    /// PaymentNotice
    #[serde(rename = "PaymentNotice")]
    PaymentNotice,
    /// PaymentReconciliation
    #[serde(rename = "PaymentReconciliation")]
    PaymentReconciliation,
    /// Permission
    #[serde(rename = "Permission")]
    Permission,
    /// Person
    #[serde(rename = "Person")]
    Person,
    /// PlanDefinition
    #[serde(rename = "PlanDefinition")]
    PlanDefinition,
    /// Practitioner
    #[serde(rename = "Practitioner")]
    Practitioner,
    /// PractitionerRole
    #[serde(rename = "PractitionerRole")]
    PractitionerRole,
    /// Procedure
    #[serde(rename = "Procedure")]
    Procedure,
    /// Provenance
    #[serde(rename = "Provenance")]
    Provenance,
    /// Questionnaire
    #[serde(rename = "Questionnaire")]
    Questionnaire,
    /// QuestionnaireResponse
    #[serde(rename = "QuestionnaireResponse")]
    QuestionnaireResponse,
    /// RegulatedAuthorization
    #[serde(rename = "RegulatedAuthorization")]
    RegulatedAuthorization,
    /// RelatedPerson
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    /// RequestOrchestration
    #[serde(rename = "RequestOrchestration")]
    RequestOrchestration,
    /// Requirements
    #[serde(rename = "Requirements")]
    Requirements,
    /// ResearchStudy
    #[serde(rename = "ResearchStudy")]
    ResearchStudy,
    /// ResearchSubject
    #[serde(rename = "ResearchSubject")]
    ResearchSubject,
    /// RiskAssessment
    #[serde(rename = "RiskAssessment")]
    RiskAssessment,
    /// Schedule
    #[serde(rename = "Schedule")]
    Schedule,
    /// SearchParameter
    #[serde(rename = "SearchParameter")]
    SearchParameter,
    /// ServiceRequest
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    /// Slot
    #[serde(rename = "Slot")]
    Slot,
    /// Specimen
    #[serde(rename = "Specimen")]
    Specimen,
    /// SpecimenDefinition
    #[serde(rename = "SpecimenDefinition")]
    SpecimenDefinition,
    /// StructureDefinition
    #[serde(rename = "StructureDefinition")]
    StructureDefinition,
    /// StructureMap
    #[serde(rename = "StructureMap")]
    StructureMap,
    /// Subscription
    #[serde(rename = "Subscription")]
    Subscription,
    /// SubscriptionStatus
    #[serde(rename = "SubscriptionStatus")]
    SubscriptionStatus,
    /// SubscriptionTopic
    #[serde(rename = "SubscriptionTopic")]
    SubscriptionTopic,
    /// Substance
    #[serde(rename = "Substance")]
    Substance,
    /// SubstanceDefinition
    #[serde(rename = "SubstanceDefinition")]
    SubstanceDefinition,
    /// SubstanceNucleicAcid
    #[serde(rename = "SubstanceNucleicAcid")]
    SubstanceNucleicAcid,
    /// SubstancePolymer
    #[serde(rename = "SubstancePolymer")]
    SubstancePolymer,
    /// SubstanceProtein
    #[serde(rename = "SubstanceProtein")]
    SubstanceProtein,
    /// SubstanceReferenceInformation
    #[serde(rename = "SubstanceReferenceInformation")]
    SubstanceReferenceInformation,
    /// SubstanceSourceMaterial
    #[serde(rename = "SubstanceSourceMaterial")]
    SubstanceSourceMaterial,
    /// SupplyDelivery
    #[serde(rename = "SupplyDelivery")]
    SupplyDelivery,
    /// SupplyRequest
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    /// Task
    #[serde(rename = "Task")]
    Task,
    /// TerminologyCapabilities
    #[serde(rename = "TerminologyCapabilities")]
    TerminologyCapabilities,
    /// TestPlan
    #[serde(rename = "TestPlan")]
    TestPlan,
    /// TestReport
    #[serde(rename = "TestReport")]
    TestReport,
    /// TestScript
    #[serde(rename = "TestScript")]
    TestScript,
    /// Transport
    #[serde(rename = "Transport")]
    Transport,
    /// ValueSet
    #[serde(rename = "ValueSet")]
    ValueSet,
    /// VerificationResult
    #[serde(rename = "VerificationResult")]
    VerificationResult,
    /// VisionPrescription
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
    /// Parameters
    #[serde(rename = "Parameters")]
    Parameters,
}

/// All published FHIR Versions.
///
/// System: <http://hl7.org/fhir/FHIR-version>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FhirVersion {
    /// 0.01
    #[default]
    #[serde(rename = "0.01")]
    N001,
    /// 0.05
    #[serde(rename = "0.05")]
    N005,
    /// 0.06
    #[serde(rename = "0.06")]
    N006,
    /// 0.11
    #[serde(rename = "0.11")]
    N011,
    /// 0.0
    #[serde(rename = "0.0")]
    N00,
    /// 0.0.80
    #[serde(rename = "0.0.80")]
    N0080,
    /// 0.0.81
    #[serde(rename = "0.0.81")]
    N0081,
    /// 0.0.82
    #[serde(rename = "0.0.82")]
    N0082,
    /// 0.4
    #[serde(rename = "0.4")]
    N04,
    /// 0.4.0
    #[serde(rename = "0.4.0")]
    N040,
    /// 0.5
    #[serde(rename = "0.5")]
    N05,
    /// 0.5.0
    #[serde(rename = "0.5.0")]
    N050,
    /// 1.0
    #[serde(rename = "1.0")]
    N10,
    /// 1.0.0
    #[serde(rename = "1.0.0")]
    N100,
    /// 1.0.1
    #[serde(rename = "1.0.1")]
    N101,
    /// 1.0.2
    #[serde(rename = "1.0.2")]
    N102,
    /// 1.1
    #[serde(rename = "1.1")]
    N11,
    /// 1.1.0
    #[serde(rename = "1.1.0")]
    N110,
    /// 1.4
    #[serde(rename = "1.4")]
    N14,
    /// 1.4.0
    #[serde(rename = "1.4.0")]
    N140,
    /// 1.6
    #[serde(rename = "1.6")]
    N16,
    /// 1.6.0
    #[serde(rename = "1.6.0")]
    N160,
    /// 1.8
    #[serde(rename = "1.8")]
    N18,
    /// 1.8.0
    #[serde(rename = "1.8.0")]
    N180,
    /// 3.0
    #[serde(rename = "3.0")]
    N30,
    /// 3.0.0
    #[serde(rename = "3.0.0")]
    N300,
    /// 3.0.1
    #[serde(rename = "3.0.1")]
    N301,
    /// 3.0.2
    #[serde(rename = "3.0.2")]
    N302,
    /// 3.3
    #[serde(rename = "3.3")]
    N33,
    /// 3.3.0
    #[serde(rename = "3.3.0")]
    N330,
    /// 3.5
    #[serde(rename = "3.5")]
    N35,
    /// 3.5.0
    #[serde(rename = "3.5.0")]
    N350,
    /// 4.0
    #[serde(rename = "4.0")]
    N40,
    /// 4.0.0
    #[serde(rename = "4.0.0")]
    N400,
    /// 4.0.1
    #[serde(rename = "4.0.1")]
    N401,
    /// 4.1
    #[serde(rename = "4.1")]
    N41,
    /// 4.1.0
    #[serde(rename = "4.1.0")]
    N410,
    /// 4.2
    #[serde(rename = "4.2")]
    N42,
    /// 4.2.0
    #[serde(rename = "4.2.0")]
    N420,
    /// 4.3
    #[serde(rename = "4.3")]
    N43,
    /// 4.3.0
    #[serde(rename = "4.3.0")]
    N430,
    /// 4.3.0-cibuild
    #[serde(rename = "4.3.0-cibuild")]
    N430Cibuild,
    /// 4.3.0-snapshot1
    #[serde(rename = "4.3.0-snapshot1")]
    N430Snapshot1,
    /// 4.4
    #[serde(rename = "4.4")]
    N44,
    /// 4.4.0
    #[serde(rename = "4.4.0")]
    N440,
    /// 4.5
    #[serde(rename = "4.5")]
    N45,
    /// 4.5.0
    #[serde(rename = "4.5.0")]
    N450,
    /// 4.6
    #[serde(rename = "4.6")]
    N46,
    /// 4.6.0
    #[serde(rename = "4.6.0")]
    N460,
    /// 5.0
    #[serde(rename = "5.0")]
    N50,
    /// 5.0.0
    #[serde(rename = "5.0.0")]
    N500,
    /// 5.0.0-cibuild
    #[serde(rename = "5.0.0-cibuild")]
    N500Cibuild,
    /// 5.0.0-snapshot1
    #[serde(rename = "5.0.0-snapshot1")]
    N500Snapshot1,
    /// 5.0.0-snapshot2
    #[serde(rename = "5.0.0-snapshot2")]
    N500Snapshot2,
    /// 5.0.0-ballot
    #[serde(rename = "5.0.0-ballot")]
    N500Ballot,
    /// 5.0.0-snapshot3
    #[serde(rename = "5.0.0-snapshot3")]
    N500Snapshot3,
    /// 5.0.0-draft-final
    #[serde(rename = "5.0.0-draft-final")]
    N500DraftFinal,
}

/// Data types defined by FHIRPath and used within the FHIR specification
///
/// System: <http://hl7.org/fhir/CodeSystem/fhirpath-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FhirpathTypes {
    /// String
    #[default]
    #[serde(rename = "http://hl7.org/fhirpath/System.String")]
    HttpHl7OrgFhirpathSystemString,
    /// Boolean
    #[serde(rename = "http://hl7.org/fhirpath/System.Boolean")]
    HttpHl7OrgFhirpathSystemBoolean,
    /// Date
    #[serde(rename = "http://hl7.org/fhirpath/System.Date")]
    HttpHl7OrgFhirpathSystemDate,
    /// DateTime
    #[serde(rename = "http://hl7.org/fhirpath/System.DateTime")]
    HttpHl7OrgFhirpathSystemDateTime,
    /// Decimal
    #[serde(rename = "http://hl7.org/fhirpath/System.Decimal")]
    HttpHl7OrgFhirpathSystemDecimal,
    /// Integer
    #[serde(rename = "http://hl7.org/fhirpath/System.Integer")]
    HttpHl7OrgFhirpathSystemInteger,
    /// Time
    #[serde(rename = "http://hl7.org/fhirpath/System.Time")]
    HttpHl7OrgFhirpathSystemTime,
}

/// The kind of operation to perform as a part of a property based filter.
///
/// System: <http://hl7.org/fhir/filter-operator>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FilterOperator {
    /// Equals
    #[default]
    #[serde(rename = "=")]
    Unnamed,
    /// Is A (by subsumption)
    #[serde(rename = "is-a")]
    IsA,
    /// Descendent Of (by subsumption)
    #[serde(rename = "descendent-of")]
    DescendentOf,
    /// Not (Is A) (by subsumption)
    #[serde(rename = "is-not-a")]
    IsNotA,
    /// Regular Expression
    #[serde(rename = "regex")]
    Regex,
    /// In Set
    #[serde(rename = "in")]
    In,
    /// Not in Set
    #[serde(rename = "not-in")]
    NotIn,
    /// Generalizes (by Subsumption)
    #[serde(rename = "generalizes")]
    Generalizes,
    /// Child Of
    #[serde(rename = "child-of")]
    ChildOf,
    /// Descendent Leaf
    #[serde(rename = "descendent-leaf")]
    DescendentLeaf,
    /// Exists
    #[serde(rename = "exists")]
    Exists,
}

/// Indicates whether this flag is active and needs to be displayed to a user,
/// or whether it is no longer needed or was entered in error.
///
/// System: <http://hl7.org/fhir/flag-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FlagStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// This value set includes Status codes.
///
/// System: <http://hl7.org/fhir/fm-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FmStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Evidence focus characteristic code.
///
/// System: <http://hl7.org/fhir/focus-characteristic-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FocusCharacteristicCode {
    /// Citation
    #[default]
    #[serde(rename = "citation")]
    Citation,
    /// Observed outcomes are clinical outcomes
    #[serde(rename = "clinical-outcomes-observed")]
    ClinicalOutcomesObserved,
    /// Population
    #[serde(rename = "population")]
    Population,
    /// Exposure
    #[serde(rename = "exposure")]
    Exposure,
    /// Comparator
    #[serde(rename = "comparator")]
    Comparator,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
    /// Medication exposures
    #[serde(rename = "medication-exposures")]
    MedicationExposures,
    /// Study type
    #[serde(rename = "study-type")]
    StudyType,
}

/// FormularyItem Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/formularyitem-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FormularyitemStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}

/// The change type relevant to GenomicStudy analysis.
///
/// System: <http://hl7.org/fhir/genomicstudy-changetype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GenomicstudyChangetype {
    /// DNA change
    #[default]
    #[serde(rename = "DNA")]
    Dna,
    /// RNA change
    #[serde(rename = "RNA")]
    Rna,
    /// Protein/amino Acids change
    #[serde(rename = "AA")]
    Aa,
    /// Chromosomal changes
    #[serde(rename = "CHR")]
    Chr,
    /// Copy number variations
    #[serde(rename = "CNV")]
    Cnv,
}

/// The data format relevant to genomics. These formats and relevant codes were
/// pulled from [Integrative Genomics Viewer
/// Documentation](https://software.broadinstitute.org/software/igv/FileFormats)
/// by Broad Institute.
///
/// System: <http://hl7.org/fhir/genomicstudy-dataformat>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GenomicstudyDataformat {
    /// BAM
    #[default]
    #[serde(rename = "bam")]
    Bam,
    /// BED
    #[serde(rename = "bed")]
    Bed,
    /// BEDPE
    #[serde(rename = "bedpe")]
    Bedpe,
    /// BedGraph
    #[serde(rename = "bedgraph")]
    Bedgraph,
    /// bigBed
    #[serde(rename = "bigbed")]
    Bigbed,
    /// bigWig
    #[serde(rename = "bigWig")]
    BigWig,
    /// Birdsuite-Files
    #[serde(rename = "birdsuite-files")]
    BirdsuiteFiles,
    /// broadPeak
    #[serde(rename = "broadpeak")]
    Broadpeak,
    /// CBS
    #[serde(rename = "cbs")]
    Cbs,
    /// Chemical-Reactivity-Probing-Profiles
    #[serde(rename = "chemical-reactivity-probing-profiles")]
    ChemicalReactivityProbingProfiles,
    /// chrom-sizes
    #[serde(rename = "chrom-sizes")]
    ChromSizes,
    /// CN
    #[serde(rename = "cn")]
    Cn,
    /// Custom-File-Formats
    #[serde(rename = "custom-file-formats")]
    CustomFileFormats,
    /// Cytoband
    #[serde(rename = "cytoband")]
    Cytoband,
    /// FASTA
    #[serde(rename = "fasta")]
    Fasta,
    /// GCT
    #[serde(rename = "gct")]
    Gct,
    /// CRAM
    #[serde(rename = "cram")]
    Cram,
    /// genePred
    #[serde(rename = "genepred")]
    Genepred,
    /// GFF/GTF
    #[serde(rename = "gff-gtf")]
    GffGtf,
    /// GISTIC
    #[serde(rename = "gistic")]
    Gistic,
    /// Goby
    #[serde(rename = "goby")]
    Goby,
    /// GWAS
    #[serde(rename = "gwas")]
    Gwas,
    /// IGV
    #[serde(rename = "igv")]
    Igv,
    /// LOH
    #[serde(rename = "loh")]
    Loh,
    /// MAF-Multiple Alignment Format
    #[serde(rename = "maf-multiple-alignment-format")]
    MafMultipleAlignmentFormat,
    /// MAF-Mutation-Annotation-Format
    #[serde(rename = "maf-mutation-annotation-format")]
    MafMutationAnnotationFormat,
    /// Merged BAM File
    #[serde(rename = "merged-bam-file")]
    MergedBamFile,
    /// MUT
    #[serde(rename = "mut")]
    Mut,
    /// narrowPeak
    #[serde(rename = "narrowpeak")]
    Narrowpeak,
    /// PSL
    #[serde(rename = "psl")]
    Psl,
    /// RES
    #[serde(rename = "res")]
    Res,
    /// RNA-Secondary-Structure-Formats
    #[serde(rename = "rna-secondary-structure-formats")]
    RnaSecondaryStructureFormats,
    /// SAM
    #[serde(rename = "sam")]
    Sam,
    /// Sample-Info-Attributes-file
    #[serde(rename = "sample-info-attributes-file")]
    SampleInfoAttributesFile,
    /// SEG
    #[serde(rename = "seg")]
    Seg,
    /// TDF
    #[serde(rename = "tdf")]
    Tdf,
    /// Track Line
    #[serde(rename = "track-line")]
    TrackLine,
    /// Type Line
    #[serde(rename = "type-line")]
    TypeLine,
    /// VCF
    #[serde(rename = "vcf")]
    Vcf,
    /// WIG
    #[serde(rename = "wig")]
    Wig,
}

/// The method type of the GenomicStudy analysis. These method types and
/// relevant codes were pulled from [National Library of Medicine-Genetic
/// Testing Registry](https://www.ncbi.nlm.nih.gov/gtr/) (NCBI-GTR) values of
/// describing different testing methods on various levels: [major method
/// category](https://ftp.ncbi.nlm.nih.gov/pub/GTR/standard_terms/Major_method_category.txt),
/// [method
/// category](https://ftp.ncbi.nlm.nih.gov/pub/GTR/standard_terms/Method_category.txt),
/// and [primary
/// methodology](https://ftp.ncbi.nlm.nih.gov/pub/GTR/standard_terms/Primary_test_methodology.txt)
///
/// System: <http://hl7.org/fhir/genomicstudy-methodtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GenomicstudyMethodtype {
    /// Biochemical Genetics
    #[default]
    #[serde(rename = "biochemical-genetics")]
    BiochemicalGenetics,
    /// Cytogenetics
    #[serde(rename = "cytogenetics")]
    Cytogenetics,
    /// Molecular Genetics
    #[serde(rename = "molecular-genetics")]
    MolecularGenetics,
    /// Analyte
    #[serde(rename = "analyte")]
    Analyte,
    /// Chromosome breakage studies
    #[serde(rename = "chromosome-breakage-studies")]
    ChromosomeBreakageStudies,
    /// Deletion/duplication analysis
    #[serde(rename = "deletion-duplication-analysis")]
    DeletionDuplicationAnalysis,
    /// Detection of homozygosity
    #[serde(rename = "detection-of-homozygosity")]
    DetectionOfHomozygosity,
    /// Enzyme assay
    #[serde(rename = "enzyme-assay")]
    EnzymeAssay,
    /// FISH-interphase
    #[serde(rename = "fish-interphase")]
    FishInterphase,
    /// FISH-metaphase
    #[serde(rename = "fish-metaphase")]
    FishMetaphase,
    /// Flow cytometry
    #[serde(rename = "flow-cytometry")]
    FlowCytometry,
    /// Fluorescence in situ hybridization (FISH)
    #[serde(rename = "fish")]
    Fish,
    /// Immunohistochemistry
    #[serde(rename = "immunohistochemistry")]
    Immunohistochemistry,
    /// Karyotyping
    #[serde(rename = "karyotyping")]
    Karyotyping,
    /// Linkage analysis
    #[serde(rename = "linkage-analysis")]
    LinkageAnalysis,
    /// Methylation analysis
    #[serde(rename = "methylation-analysis")]
    MethylationAnalysis,
    /// Microsatellite instability testing (MSI)
    #[serde(rename = "msi")]
    Msi,
    /// Multicolor FISH (M-FISH)
    #[serde(rename = "m-fish")]
    MFish,
    /// Mutation scanning of select exons
    #[serde(rename = "mutation-scanning-of-select-exons")]
    MutationScanningOfSelectExons,
    /// Mutation scanning of the entire coding region
    #[serde(rename = "mutation-scanning-of-the-entire-coding-region")]
    MutationScanningOfTheEntireCodingRegion,
    /// Protein analysis
    #[serde(rename = "protein-analysis")]
    ProteinAnalysis,
    /// Protein expression
    #[serde(rename = "protein-expression")]
    ProteinExpression,
    /// RNA analysis
    #[serde(rename = "rna-analysis")]
    RnaAnalysis,
    /// Sequence analysis of select exons
    #[serde(rename = "sequence-analysis-of-select-exons")]
    SequenceAnalysisOfSelectExons,
    /// Sequence analysis of the entire coding region
    #[serde(rename = "sequence-analysis-of-the-entire-coding-region")]
    SequenceAnalysisOfTheEntireCodingRegion,
    /// Sister chromatid exchange
    #[serde(rename = "sister-chromatid-exchange")]
    SisterChromatidExchange,
    /// Targeted variant analysis
    #[serde(rename = "targeted-variant-analysis")]
    TargetedVariantAnalysis,
    /// Uniparental disomy study (UPD)
    #[serde(rename = "udp")]
    Udp,
    /// Allele-specific primer extension (ASPE)
    #[serde(rename = "aspe")]
    Aspe,
    /// Alternative splicing detection
    #[serde(rename = "alternative-splicing-detection")]
    AlternativeSplicingDetection,
    /// Bi-directional Sanger Sequence Analysis
    #[serde(rename = "bi-directional-sanger-sequence-analysis")]
    BiDirectionalSangerSequenceAnalysis,
    /// C-banding
    #[serde(rename = "c-banding")]
    CBanding,
    /// Chemiluminescent Immunoassay (CIA)
    #[serde(rename = "cia")]
    Cia,
    /// Chromatin Immunoprecipitation on ChIP
    #[serde(rename = "chromatin-immunoprecipitation-on-chip")]
    ChromatinImmunoprecipitationOnChip,
    /// Comparative Genomic Hybridization
    #[serde(rename = "comparative-genomic-hybridization")]
    ComparativeGenomicHybridization,
    /// DamID
    #[serde(rename = "damid")]
    Damid,
    /// Digital / Virtual karyotyping
    #[serde(rename = "digital-virtual-karyotyping")]
    DigitalVirtualKaryotyping,
    /// Digital microfluidic microspheres
    #[serde(rename = "digital-microfluidic-microspheres")]
    DigitalMicrofluidicMicrospheres,
    /// Enzymatic levels
    #[serde(rename = "enzymatic-levels")]
    EnzymaticLevels,
    /// Enzyme activity
    #[serde(rename = "enzyme-activity")]
    EnzymeActivity,
    /// Enzyme-Linked Immunosorbent Assays (ELISA)
    #[serde(rename = "elisa")]
    Elisa,
    /// Fluorometry
    #[serde(rename = "fluorometry")]
    Fluorometry,
    /// Fusion genes microarrays
    #[serde(rename = "fusion-genes-microarrays")]
    FusionGenesMicroarrays,
    /// G-banding
    #[serde(rename = "g-banding")]
    GBanding,
    /// Gas chromatographyâ€“mass spectrometry (GC-MS)
    #[serde(rename = "gc-ms")]
    GcMs,
    /// Gene expression profiling
    #[serde(rename = "gene-expression-profiling")]
    GeneExpressionProfiling,
    /// GeneID
    #[serde(rename = "gene-id")]
    GeneId,
    /// Gold nanoparticle probe technology
    #[serde(rename = "gold-nanoparticle-probe-technology")]
    GoldNanoparticleProbeTechnology,
    /// High-performance liquid chromatography (HPLC)
    #[serde(rename = "hplc")]
    Hplc,
    /// Liquid chromatography mass spectrometry (LC-MS)
    #[serde(rename = "lc-ms")]
    LcMs,
    /// Liquid chromatography-tandem mass spectrometry (LC-MS/MS)
    #[serde(rename = "lc-ms-ms")]
    LcMsMs,
    /// Metabolite levels
    #[serde(rename = "metabolite-levels")]
    MetaboliteLevels,
    /// Methylation-specific PCR
    #[serde(rename = "methylation-specific-pcr")]
    MethylationSpecificPcr,
    /// Microarray
    #[serde(rename = "microarray")]
    Microarray,
    /// Multiplex Ligation-dependent Probe Amplification (MLPA)
    #[serde(rename = "mlpa")]
    Mlpa,
    /// Next-Generation (NGS)/Massively parallel sequencing (MPS)
    #[serde(rename = "ngs-mps")]
    NgsMps,
    /// Oligonucleotide Ligation Assay (OLA)
    #[serde(rename = "ola")]
    Ola,
    /// Oligonucleotide hybridization-based DNA sequencing
    #[serde(rename = "oligonucleotide-hybridization-based-dna-sequencing")]
    OligonucleotideHybridizationBasedDnaSequencing,
    /// Other
    #[serde(rename = "other")]
    Other,
    /// PCR
    #[serde(rename = "pcr")]
    Pcr,
    /// PCR with allele specific hybridization
    #[serde(rename = "pcr-with-allele-specific-hybridization")]
    PcrWithAlleleSpecificHybridization,
    /// PCR-RFLP with Southern hybridization
    #[serde(rename = "pcr-rflp-with-southern-hybridization")]
    PcrRflpWithSouthernHybridization,
    /// Protein truncation test
    #[serde(rename = "protein-truncation-test")]
    ProteinTruncationTest,
    /// Pyrosequencing
    #[serde(rename = "pyrosequencing")]
    Pyrosequencing,
    /// Q-banding
    #[serde(rename = "q-banding")]
    QBanding,
    /// Quantitative PCR (qPCR)
    #[serde(rename = "qpcr")]
    Qpcr,
    /// R-banding
    #[serde(rename = "r-banding")]
    RBanding,
    /// RFLP
    #[serde(rename = "rflp")]
    Rflp,
    /// RT-LAMP
    #[serde(rename = "rt-lamp")]
    RtLamp,
    /// RT-PCR
    #[serde(rename = "rt-pcr")]
    RtPcr,
    /// RT-PCR with gel analysis
    #[serde(rename = "rt-pcr-with-gel-analysis")]
    RtPcrWithGelAnalysis,
    /// RT-qPCR
    #[serde(rename = "rt-qpcr")]
    RtQpcr,
    /// SNP Detection
    #[serde(rename = "snp-detection")]
    SnpDetection,
    /// Silver staining
    #[serde(rename = "silver-staining")]
    SilverStaining,
    /// Spectral karyotyping (SKY)
    #[serde(rename = "sky")]
    Sky,
    /// T-banding
    #[serde(rename = "t-banding")]
    TBanding,
    /// Tandem mass spectrometry (MS/MS)
    #[serde(rename = "ms-ms")]
    MsMs,
    /// Tetra-nucleotide repeat by PCR or Southern Blot
    #[serde(rename = "tetra-nucleotide-repeat-by-pcr-or-southern-blot")]
    TetraNucleotideRepeatByPcrOrSouthernBlot,
    /// Tiling Arrays
    #[serde(rename = "tiling-arrays")]
    TilingArrays,
    /// Trinucleotide repeat by PCR or Southern Blot
    #[serde(rename = "trinucleotide-repeat-by-pcr-or-southern-blot")]
    TrinucleotideRepeatByPcrOrSouthernBlot,
    /// Uni-directional Sanger sequencing
    #[serde(rename = "uni-directional-sanger-sequencing")]
    UniDirectionalSangerSequencing,
}

/// The status of the GenomicStudy.
///
/// System: <http://hl7.org/fhir/genomicstudy-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GenomicstudyStatus {
    /// Registered
    #[default]
    #[serde(rename = "registered")]
    Registered,
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The type relevant to GenomicStudy.
///
/// System: <http://hl7.org/fhir/genomicstudy-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GenomicstudyType {
    /// Alternative splicing detection
    #[default]
    #[serde(rename = "alt-splc")]
    AltSplc,
    /// Chromatin conformation
    #[serde(rename = "chromatin")]
    Chromatin,
    /// CNV detection
    #[serde(rename = "cnv")]
    Cnv,
    /// Epigenetic Alterations - histone modifications
    #[serde(rename = "epi-alt-hist")]
    EpiAltHist,
    /// Epigenetic Alterations -DNA methylation
    #[serde(rename = "epi-alt-dna")]
    EpiAltDna,
    /// Familial variant segregation
    #[serde(rename = "fam-var-segr")]
    FamVarSegr,
    /// Functional variation detection
    #[serde(rename = "func-var")]
    FuncVar,
    /// Gene expression profiling
    #[serde(rename = "gene-expression")]
    GeneExpression,
    /// Post-translational Modification Identification
    #[serde(rename = "post-trans-mod")]
    PostTransMod,
    /// SNP Detection
    #[serde(rename = "snp")]
    Snp,
    /// STR count
    #[serde(rename = "str")]
    Str,
    /// Structural variation detection
    #[serde(rename = "struc-var")]
    StrucVar,
}

/// Codes that reflect the current state of a goal and whether the goal is
/// still being targeted.
///
/// System: <http://hl7.org/fhir/goal-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
}

/// How a compartment must be linked.
///
/// System: <http://hl7.org/fhir/graph-compartment-rule>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GraphCompartmentRule {
    /// Identical
    #[default]
    #[serde(rename = "identical")]
    Identical,
    /// Matching
    #[serde(rename = "matching")]
    Matching,
    /// Different
    #[serde(rename = "different")]
    Different,
    /// Custom
    #[serde(rename = "custom")]
    Custom,
}

/// Defines how a compartment rule is used.
///
/// System: <http://hl7.org/fhir/graph-compartment-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GraphCompartmentUse {
    /// Where
    #[default]
    #[serde(rename = "where")]
    Where,
    /// requires
    #[serde(rename = "requires")]
    Requires,
}

/// Basis for membership in a group
///
/// System: <http://hl7.org/fhir/group-membership-basis>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GroupMembershipBasis {
    /// Definitional
    #[default]
    #[serde(rename = "definitional")]
    Definitional,
    /// Enumerated
    #[serde(rename = "enumerated")]
    Enumerated,
}

/// Types of resources that are part of group.
///
/// System: <http://hl7.org/fhir/group-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GroupType {
    /// Person
    #[default]
    #[serde(rename = "person")]
    Person,
    /// Animal
    #[serde(rename = "animal")]
    Animal,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// CareTeam
    #[serde(rename = "careteam")]
    Careteam,
    /// HealthcareService
    #[serde(rename = "healthcareservice")]
    Healthcareservice,
    /// Location
    #[serde(rename = "location")]
    Location,
    /// Organization
    #[serde(rename = "organization")]
    Organization,
    /// RelatedPerson
    #[serde(rename = "relatedperson")]
    Relatedperson,
    /// Specimen
    #[serde(rename = "specimen")]
    Specimen,
}

/// Example guidance module codes.
///
/// System: <http://hl7.org/fhir/guidance-module-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuidanceModuleCode {
    /// BMI Calculator
    #[default]
    #[serde(rename = "bmi-calculator")]
    BmiCalculator,
    /// MME Calculator
    #[serde(rename = "mme-calculator")]
    MmeCalculator,
    /// Opioid CDS
    #[serde(rename = "opioid-cds")]
    OpioidCds,
    /// ANC CDS
    #[serde(rename = "anc-cds")]
    AncCds,
    /// CHF Pathway
    #[serde(rename = "chf-pathway")]
    ChfPathway,
    /// COVID-19 Severity Score
    #[serde(rename = "covid-19-severity")]
    Covid19Severity,
}

/// The status of a guidance response.
///
/// System: <http://hl7.org/fhir/guidance-response-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuidanceResponseStatus {
    /// Success
    #[default]
    #[serde(rename = "success")]
    Success,
    /// Data Requested
    #[serde(rename = "data-requested")]
    DataRequested,
    /// Data Required
    #[serde(rename = "data-required")]
    DataRequired,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Failure
    #[serde(rename = "failure")]
    Failure,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// A code that indicates how the page is generated.
///
/// System: <http://hl7.org/fhir/guide-page-generation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuidePageGeneration {
    /// HTML
    #[default]
    #[serde(rename = "html")]
    Html,
    /// Markdown
    #[serde(rename = "markdown")]
    Markdown,
    /// XML
    #[serde(rename = "xml")]
    Xml,
    /// Generated
    #[serde(rename = "generated")]
    Generated,
}

/// GuideParameterCode
///
/// System: <http://hl7.org/fhir/guide-parameter-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuideParameterCode {
    /// Apply Metadata Value
    #[default]
    #[serde(rename = "apply")]
    Apply,
    /// Resource Path
    #[serde(rename = "path-resource")]
    PathResource,
    /// Pages Path
    #[serde(rename = "path-pages")]
    PathPages,
    /// Terminology Cache Path
    #[serde(rename = "path-tx-cache")]
    PathTxCache,
    /// Expansion Profile
    #[serde(rename = "expansion-parameter")]
    ExpansionParameter,
    /// Broken Links Rule
    #[serde(rename = "rule-broken-links")]
    RuleBrokenLinks,
    /// Generate XML
    #[serde(rename = "generate-xml")]
    GenerateXml,
    /// Generate JSON
    #[serde(rename = "generate-json")]
    GenerateJson,
    /// Generate Turtle
    #[serde(rename = "generate-turtle")]
    GenerateTurtle,
    /// HTML Template
    #[serde(rename = "html-template")]
    HtmlTemplate,
}

/// A code that identifies the status of the family history record.
///
/// System: <http://hl7.org/fhir/history-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HistoryStatus {
    /// Partial
    #[default]
    #[serde(rename = "partial")]
    Partial,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Health Unknown
    #[serde(rename = "health-unknown")]
    HealthUnknown,
}

/// The allowable request method or HTTP operation codes.
///
/// System: <http://hl7.org/fhir/http-operations>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HttpOperations {
    /// DELETE
    #[default]
    #[serde(rename = "delete")]
    Delete,
    /// GET
    #[serde(rename = "get")]
    Get,
    /// OPTIONS
    #[serde(rename = "options")]
    Options,
    /// PATCH
    #[serde(rename = "patch")]
    Patch,
    /// POST
    #[serde(rename = "post")]
    Post,
    /// PUT
    #[serde(rename = "put")]
    Put,
    /// HEAD
    #[serde(rename = "head")]
    Head,
}

/// HTTP verbs (in the HTTP command line). See [HTTP
/// rfc](https://tools.ietf.org/html/rfc7231) for details.
///
/// System: <http://hl7.org/fhir/http-verb>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HttpVerb {
    /// GET
    #[default]
    #[serde(rename = "GET")]
    Get,
    /// HEAD
    #[serde(rename = "HEAD")]
    Head,
    /// POST
    #[serde(rename = "POST")]
    Post,
    /// PUT
    #[serde(rename = "PUT")]
    Put,
    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,
    /// PATCH
    #[serde(rename = "PATCH")]
    Patch,
}

/// Link Relation Types defined at
/// https://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
///
/// System: <http://hl7.org/fhir/CodeSystem/iana-link-relations>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IanaLinkRelations {
    /// Refers to a resource that is the subject of the link's context.
    #[default]
    #[serde(rename = "about")]
    About,
    /// Asserts that the link target provides an access control description for
    /// the link context.
    #[serde(rename = "acl")]
    Acl,
    /// Refers to a substitute for this context
    #[serde(rename = "alternate")]
    Alternate,
    /// Used to reference alternative content that uses the AMP profile of the
    /// HTML format.
    #[serde(rename = "amphtml")]
    Amphtml,
    /// Refers to an appendix.
    #[serde(rename = "appendix")]
    Appendix,
    /// Refers to an icon for the context. Synonym for icon.
    #[serde(rename = "apple-touch-icon")]
    AppleTouchIcon,
    /// Refers to a launch screen for the context.
    #[serde(rename = "apple-touch-startup-image")]
    AppleTouchStartupImage,
    /// Refers to a collection of records, documents, or other materials of
    /// historical interest.
    #[serde(rename = "archives")]
    Archives,
    /// Refers to the context's author.
    #[serde(rename = "author")]
    Author,
    /// Identifies the entity that blocks access to a resource following
    /// receipt of a legal demand.
    #[serde(rename = "blocked-by")]
    BlockedBy,
    /// Gives a permanent link to use for bookmarking purposes.
    #[serde(rename = "bookmark")]
    Bookmark,
    /// Designates the preferred version of a resource (the IRI and its
    /// contents).
    #[serde(rename = "canonical")]
    Canonical,
    /// Refers to a chapter in a collection of resources.
    #[serde(rename = "chapter")]
    Chapter,
    /// Indicates that the link target is preferred over the link context for
    /// the purpose of permanent citation.
    #[serde(rename = "cite-as")]
    CiteAs,
    /// The target IRI points to a resource which represents the collection
    /// resource for the context IRI.
    #[serde(rename = "collection")]
    Collection,
    /// Refers to a table of contents.
    #[serde(rename = "contents")]
    Contents,
    /// The document linked to was later converted to the document that
    /// contains this link relation. For example, an RFC can have a link to the
    /// Internet-Draft that became the RFC; in that case, the link relation
    /// would be "convertedFrom".
    #[serde(rename = "convertedFrom")]
    ConvertedFrom,
    /// Refers to a copyright statement that applies to the link's context.
    #[serde(rename = "copyright")]
    Copyright,
    /// The target IRI points to a resource where a submission form can be
    /// obtained.
    #[serde(rename = "create-form")]
    CreateForm,
    /// Refers to a resource containing the most recent item(s) in a collection
    /// of resources.
    #[serde(rename = "current")]
    Current,
    /// Refers to a resource providing information about the link's context.
    #[serde(rename = "describedby")]
    Describedby,
    /// The relationship A 'describes' B asserts that resource A provides a
    /// description of resource B. There are no constraints on the format or
    /// representation of either A or B, neither are there any further
    /// constraints on either resource.
    #[serde(rename = "describes")]
    Describes,
    /// Refers to a list of patent disclosures made with respect to material
    /// for which 'disclosure' relation is specified.
    #[serde(rename = "disclosure")]
    Disclosure,
    /// Used to indicate an origin that will be used to fetch required
    /// resources for the link context, and that the user agent ought to
    /// resolve as early as possible.
    #[serde(rename = "dns-prefetch")]
    DnsPrefetch,
    /// Refers to a resource whose available representations are byte-for-byte
    /// identical with the corresponding representations of the context IRI.
    #[serde(rename = "duplicate")]
    Duplicate,
    /// Refers to a resource that can be used to edit the link's context.
    #[serde(rename = "edit")]
    Edit,
    /// The target IRI points to a resource where a submission form for editing
    /// associated resource can be obtained.
    #[serde(rename = "edit-form")]
    EditForm,
    /// Refers to a resource that can be used to edit media associated with the
    /// link's context.
    #[serde(rename = "edit-media")]
    EditMedia,
    /// Identifies a related resource that is potentially large and might
    /// require special handling.
    #[serde(rename = "enclosure")]
    Enclosure,
    /// Refers to a resource that is not part of the same site as the current
    /// context.
    #[serde(rename = "external")]
    External,
    /// An IRI that refers to the furthest preceding resource in a series of
    /// resources.
    #[serde(rename = "first")]
    First,
    /// Refers to a glossary of terms.
    #[serde(rename = "glossary")]
    Glossary,
    /// Refers to context-sensitive help.
    #[serde(rename = "help")]
    Help,
    /// Refers to a resource hosted by the server indicated by the link
    /// context.
    #[serde(rename = "hosts")]
    Hosts,
    /// Refers to a hub that enables registration for notification of updates
    /// to the context.
    #[serde(rename = "hub")]
    Hub,
    /// Refers to an icon representing the link's context.
    #[serde(rename = "icon")]
    Icon,
    /// Refers to an index.
    #[serde(rename = "index")]
    Index,
    /// refers to a resource associated with a time interval that ends before
    /// the beginning of the time interval associated with the context resource
    #[serde(rename = "intervalAfter")]
    IntervalAfter,
    /// refers to a resource associated with a time interval that begins after
    /// the end of the time interval associated with the context resource
    #[serde(rename = "intervalBefore")]
    IntervalBefore,
    /// refers to a resource associated with a time interval that begins after
    /// the beginning of the time interval associated with the context
    /// resource, and ends before the end of the time interval associated with
    /// the context resource
    #[serde(rename = "intervalContains")]
    IntervalContains,
    /// refers to a resource associated with a time interval that begins after
    /// the end of the time interval associated with the context resource, or
    /// ends before the beginning of the time interval associated with the
    /// context resource
    #[serde(rename = "intervalDisjoint")]
    IntervalDisjoint,
    /// refers to a resource associated with a time interval that begins before
    /// the beginning of the time interval associated with the context
    /// resource, and ends after the end of the time interval associated with
    /// the context resource
    #[serde(rename = "intervalDuring")]
    IntervalDuring,
    /// refers to a resource associated with a time interval whose beginning
    /// coincides with the beginning of the time interval associated with the
    /// context resource, and whose end coincides with the end of the time
    /// interval associated with the context resource
    #[serde(rename = "intervalEquals")]
    IntervalEquals,
    /// refers to a resource associated with a time interval that begins after
    /// the beginning of the time interval associated with the context
    /// resource, and whose end coincides with the end of the time interval
    /// associated with the context resource
    #[serde(rename = "intervalFinishedBy")]
    IntervalFinishedBy,
    /// refers to a resource associated with a time interval that begins before
    /// the beginning of the time interval associated with the context
    /// resource, and whose end coincides with the end of the time interval
    /// associated with the context resource
    #[serde(rename = "intervalFinishes")]
    IntervalFinishes,
    /// refers to a resource associated with a time interval that begins before
    /// or is coincident with the beginning of the time interval associated
    /// with the context resource, and ends after or is coincident with the end
    /// of the time interval associated with the context resource
    #[serde(rename = "intervalIn")]
    IntervalIn,
    /// refers to a resource associated with a time interval whose beginning
    /// coincides with the end of the time interval associated with the context
    /// resource
    #[serde(rename = "intervalMeets")]
    IntervalMeets,
    /// refers to a resource associated with a time interval whose end
    /// coincides with the beginning of the time interval associated with the
    /// context resource
    #[serde(rename = "intervalMetBy")]
    IntervalMetBy,
    /// refers to a resource associated with a time interval that begins before
    /// the beginning of the time interval associated with the context
    /// resource, and ends after the beginning of the time interval associated
    /// with the context resource
    #[serde(rename = "intervalOverlappedBy")]
    IntervalOverlappedBy,
    /// refers to a resource associated with a time interval that begins before
    /// the end of the time interval associated with the context resource, and
    /// ends after the end of the time interval associated with the context
    /// resource
    #[serde(rename = "intervalOverlaps")]
    IntervalOverlaps,
    /// refers to a resource associated with a time interval whose beginning
    /// coincides with the beginning of the time interval associated with the
    /// context resource, and ends before the end of the time interval
    /// associated with the context resource
    #[serde(rename = "intervalStartedBy")]
    IntervalStartedBy,
    /// refers to a resource associated with a time interval whose beginning
    /// coincides with the beginning of the time interval associated with the
    /// context resource, and ends after the end of the time interval
    /// associated with the context resource
    #[serde(rename = "intervalStarts")]
    IntervalStarts,
    /// The target IRI points to a resource that is a member of the collection
    /// represented by the context IRI.
    #[serde(rename = "item")]
    Item,
    /// An IRI that refers to the furthest following resource in a series of
    /// resources.
    #[serde(rename = "last")]
    Last,
    /// Points to a resource containing the latest (e.g., current) version of
    /// the context.
    #[serde(rename = "latest-version")]
    LatestVersion,
    /// Refers to a license associated with this context.
    #[serde(rename = "license")]
    License,
    /// The link target of a link with the "linkset" relation type provides a
    /// set of links, including links in which the link context of the link
    /// participates.
    #[serde(rename = "linkset")]
    Linkset,
    /// Refers to further information about the link's context, expressed as a
    /// LRDD ("Link-based Resource Descriptor Document") resource. See for
    /// information about processing this relation type in host-meta documents.
    /// When used elsewhere, it refers to additional links and other metadata.
    /// Multiple instances indicate additional LRDD resources. LRDD resources
    /// MUST have an "application/xrd+xml" representation, and MAY have others.
    #[serde(rename = "lrdd")]
    Lrdd,
    /// Links to a manifest file for the context.
    #[serde(rename = "manifest")]
    Manifest,
    /// Refers to a mask that can be applied to the icon for the context.
    #[serde(rename = "mask-icon")]
    MaskIcon,
    /// Refers to a feed of personalised media recommendations relevant to the
    /// link context.
    #[serde(rename = "media-feed")]
    MediaFeed,
    /// The Target IRI points to a Memento, a fixed resource that will not
    /// change state anymore.
    #[serde(rename = "memento")]
    Memento,
    /// Links to the context's Micropub endpoint.
    #[serde(rename = "micropub")]
    Micropub,
    /// Refers to a module that the user agent is to preemptively fetch and
    /// store for use in the current context.
    #[serde(rename = "modulepreload")]
    Modulepreload,
    /// Refers to a resource that can be used to monitor changes in an HTTP
    /// resource.
    #[serde(rename = "monitor")]
    Monitor,
    /// Refers to a resource that can be used to monitor changes in a specified
    /// group of HTTP resources.
    #[serde(rename = "monitor-group")]
    MonitorGroup,
    /// Indicates that the link's context is a part of a series, and that the
    /// next in the series is the link target.
    #[serde(rename = "next")]
    Next,
    /// Refers to the immediately following archive resource.
    #[serde(rename = "next-archive")]
    NextArchive,
    /// Indicates that the context’s original author or publisher does not
    /// endorse the link target.
    #[serde(rename = "nofollow")]
    Nofollow,
    /// Indicates that any newly created top-level browsing context which
    /// results from following the link will not be an auxiliary browsing
    /// context.
    #[serde(rename = "noopener")]
    Noopener,
    /// Indicates that no referrer information is to be leaked when following
    /// the link.
    #[serde(rename = "noreferrer")]
    Noreferrer,
    /// Indicates that any newly created top-level browsing context which
    /// results from following the link will be an auxiliary browsing context.
    #[serde(rename = "opener")]
    Opener,
    /// Refers to an OpenID Authentication server on which the context relies
    /// for an assertion that the end user controls an Identifier.
    #[serde(rename = "openid2.local_id")]
    Openid2LocalId,
    /// Refers to a resource which accepts OpenID Authentication protocol
    /// messages for the context.
    #[serde(rename = "openid2.provider")]
    Openid2Provider,
    /// The Target IRI points to an Original Resource.
    #[serde(rename = "original")]
    Original,
    /// Refers to a P3P privacy policy for the context.
    #[serde(rename = "P3Pv1")]
    P3Pv1,
    /// Indicates a resource where payment is accepted.
    #[serde(rename = "payment")]
    Payment,
    /// Gives the address of the pingback resource for the link context.
    #[serde(rename = "pingback")]
    Pingback,
    /// Used to indicate an origin that will be used to fetch required
    /// resources for the link context. Initiating an early connection, which
    /// includes the DNS lookup, TCP handshake, and optional TLS negotiation,
    /// allows the user agent to mask the high latency costs of establishing a
    /// connection.
    #[serde(rename = "preconnect")]
    Preconnect,
    /// Points to a resource containing the predecessor version in the version
    /// history.
    #[serde(rename = "predecessor-version")]
    PredecessorVersion,
    /// The prefetch link relation type is used to identify a resource that
    /// might be required by the next navigation from the link context, and
    /// that the user agent ought to fetch, such that the user agent can
    /// deliver a faster response once the resource is requested in the future.
    #[serde(rename = "prefetch")]
    Prefetch,
    /// Refers to a resource that should be loaded early in the processing of
    /// the link's context, without blocking rendering.
    #[serde(rename = "preload")]
    Preload,
    /// Used to identify a resource that might be required by the next
    /// navigation from the link context, and that the user agent ought to
    /// fetch and execute, such that the user agent can deliver a faster
    /// response once the resource is requested in the future.
    #[serde(rename = "prerender")]
    Prerender,
    /// Indicates that the link's context is a part of a series, and that the
    /// previous in the series is the link target.
    #[serde(rename = "prev")]
    Prev,
    /// Refers to a resource that provides a preview of the link's context.
    #[serde(rename = "preview")]
    Preview,
    /// Refers to the previous resource in an ordered series of resources.
    /// Synonym for "prev".
    #[serde(rename = "previous")]
    Previous,
    /// Refers to the immediately preceding archive resource.
    #[serde(rename = "prev-archive")]
    PrevArchive,
    /// Refers to a privacy policy associated with the link's context.
    #[serde(rename = "privacy-policy")]
    PrivacyPolicy,
    /// Identifying that a resource representation conforms to a certain
    /// profile, without affecting the non-profile semantics of the resource
    /// representation.
    #[serde(rename = "profile")]
    Profile,
    /// Links to a publication manifest. A manifest represents structured
    /// information about a publication, such as informative metadata, a list
    /// of resources, and a default reading order.
    #[serde(rename = "publication")]
    Publication,
    /// Identifies a related resource.
    #[serde(rename = "related")]
    Related,
    /// Identifies the root of RESTCONF API as configured on this HTTP server.
    /// The "restconf" relation defines the root of the API defined in RFC8040.
    /// Subsequent revisions of RESTCONF will use alternate relation values to
    /// support protocol versioning.
    #[serde(rename = "restconf")]
    Restconf,
    /// Identifies a resource that is a reply to the context of the link.
    #[serde(rename = "replies")]
    Replies,
    /// The resource identified by the link target provides an input value to
    /// an instance of a rule, where the resource which represents the rule
    /// instance is identified by the link context.
    #[serde(rename = "ruleinput")]
    Ruleinput,
    /// Refers to a resource that can be used to search through the link's
    /// context and related resources.
    #[serde(rename = "search")]
    Search,
    /// Refers to a section in a collection of resources.
    #[serde(rename = "section")]
    Section,
    /// Conveys an identifier for the link's context.
    #[serde(rename = "self")]
    SelfCode,
    /// Indicates a URI that can be used to retrieve a service document.
    #[serde(rename = "service")]
    Service,
    /// Identifies service description for the context that is primarily
    /// intended for consumption by machines.
    #[serde(rename = "service-desc")]
    ServiceDesc,
    /// Identifies service documentation for the context that is primarily
    /// intended for human consumption.
    #[serde(rename = "service-doc")]
    ServiceDoc,
    /// Identifies general metadata for the context that is primarily intended
    /// for consumption by machines.
    #[serde(rename = "service-meta")]
    ServiceMeta,
    /// Refers to a resource that is within a context that is sponsored (such
    /// as advertising or another compensation agreement).
    #[serde(rename = "sponsored")]
    Sponsored,
    /// Refers to the first resource in a collection of resources.
    #[serde(rename = "start")]
    Start,
    /// Identifies a resource that represents the context's status.
    #[serde(rename = "status")]
    Status,
    /// Refers to a stylesheet.
    #[serde(rename = "stylesheet")]
    Stylesheet,
    /// Refers to a resource serving as a subsection in a collection of
    /// resources.
    #[serde(rename = "subsection")]
    Subsection,
    /// Points to a resource containing the successor version in the version
    /// history.
    #[serde(rename = "successor-version")]
    SuccessorVersion,
    /// Identifies a resource that provides information about the context's
    /// retirement policy.
    #[serde(rename = "sunset")]
    Sunset,
    /// Gives a tag (identified by the given address) that applies to the
    /// current document.
    #[serde(rename = "tag")]
    Tag,
    /// Refers to the terms of service associated with the link's context.
    #[serde(rename = "terms-of-service")]
    TermsOfService,
    /// The Target IRI points to a TimeGate for an Original Resource.
    #[serde(rename = "timegate")]
    Timegate,
    /// The Target IRI points to a TimeMap for an Original Resource.
    #[serde(rename = "timemap")]
    Timemap,
    /// Refers to a resource identifying the abstract semantic type of which
    /// the link's context is considered to be an instance.
    #[serde(rename = "type")]
    Type,
    /// Refers to a resource that is within a context that is User Generated
    /// Content.
    #[serde(rename = "ugc")]
    Ugc,
    /// Refers to a parent document in a hierarchy of documents.
    #[serde(rename = "up")]
    Up,
    /// Points to a resource containing the version history for the context.
    #[serde(rename = "version-history")]
    VersionHistory,
    /// Identifies a resource that is the source of the information in the
    /// link's context.
    #[serde(rename = "via")]
    Via,
    /// Identifies a target URI that supports the Webmention protocol. This
    /// allows clients that mention a resource in some form of publishing
    /// process to contact that endpoint and inform it that this resource has
    /// been mentioned.
    #[serde(rename = "webmention")]
    Webmention,
    /// Points to a working copy for this resource.
    #[serde(rename = "working-copy")]
    WorkingCopy,
    /// Points to the versioned resource from which this working copy was
    /// obtained.
    #[serde(rename = "working-copy-of")]
    WorkingCopyOf,
}

/// Identifies the purpose for this identifier, if known .
///
/// System: <http://hl7.org/fhir/identifier-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IdentifierUse {
    /// Usual
    #[default]
    #[serde(rename = "usual")]
    Usual,
    /// Official
    #[serde(rename = "official")]
    Official,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Secondary
    #[serde(rename = "secondary")]
    Secondary,
    /// Old
    #[serde(rename = "old")]
    Old,
}

/// The level of confidence that this link represents the same actual person,
/// based on NIST Authentication Levels.
///
/// System: <http://hl7.org/fhir/identity-assuranceLevel>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IdentityAssuranceLevel {
    /// Level 1
    #[default]
    #[serde(rename = "level1")]
    Level1,
    /// Level 2
    #[serde(rename = "level2")]
    Level2,
    /// Level 3
    #[serde(rename = "level3")]
    Level3,
    /// Level 4
    #[serde(rename = "level4")]
    Level4,
}

/// The type of coordinates describing a 2D image region.
///
/// System: <http://hl7.org/fhir/imagingselection-2dgraphictype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Imagingselection2Dgraphictype {
    /// POINT
    #[default]
    #[serde(rename = "point")]
    Point,
    /// POLYLINE
    #[serde(rename = "polyline")]
    Polyline,
    /// INTERPOLATED
    #[serde(rename = "interpolated")]
    Interpolated,
    /// CIRCLE
    #[serde(rename = "circle")]
    Circle,
    /// ELLIPSE
    #[serde(rename = "ellipse")]
    Ellipse,
}

/// The type of coordinates describing an image region.
///
/// System: <http://hl7.org/fhir/imagingselection-3dgraphictype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Imagingselection3Dgraphictype {
    /// POINT
    #[default]
    #[serde(rename = "point")]
    Point,
    /// MULTIPOINT
    #[serde(rename = "multipoint")]
    Multipoint,
    /// POLYLINE
    #[serde(rename = "polyline")]
    Polyline,
    /// POLYGON
    #[serde(rename = "polygon")]
    Polygon,
    /// ELLIPSE
    #[serde(rename = "ellipse")]
    Ellipse,
    /// ELLIPSOID
    #[serde(rename = "ellipsoid")]
    Ellipsoid,
}

/// The status of the ImagingSelection.
///
/// System: <http://hl7.org/fhir/imagingselection-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ImagingselectionStatus {
    /// Available
    #[default]
    #[serde(rename = "available")]
    Available,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The status of the ImagingStudy.
///
/// System: <http://hl7.org/fhir/imagingstudy-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ImagingstudyStatus {
    /// Registered
    #[default]
    #[serde(rename = "registered")]
    Registered,
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// This codesystem captures the indicator codes defined by the CDS Hooks
/// specification. The indicator is included as an element of the cards in a
/// CDS Hooks response and conveys the urgency and/or importance of the
/// information in each card. See [Card
/// Attributes](https://cds-hooks.hl7.org/1.0/#card-attributes) in the CDS
/// Hooks specification for more information.
///
/// System: <http://cds-hooks.hl7.org/CodeSystem/indicator>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Indicator {
    /// Information
    #[default]
    #[serde(rename = "info")]
    Info,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Critical
    #[serde(rename = "critical")]
    Critical,
}

/// A classification of the ingredient identifying its precise purpose(s) in
/// the drug product (beyond e.g. active/inactive).
///
/// System: <http://hl7.org/fhir/ingredient-function>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IngredientFunction {
    /// Antioxidant
    #[default]
    #[serde(rename = "Antioxidant")]
    Antioxidant,
    /// Alkalizing Agent
    #[serde(rename = "AlkalizingAgent")]
    AlkalizingAgent,
}

/// The way in which this manufacturer is associated with the ingredient. For
/// example whether it is a possible one (others allowed), or an exclusive
/// authorized one for this ingredient. Note that this is not the manufacturing
/// process role
///
/// System: <http://hl7.org/fhir/ingredient-manufacturer-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IngredientManufacturerRole {
    /// Manufacturer is specifically allowed for this ingredient
    #[default]
    #[serde(rename = "allowed")]
    Allowed,
    /// Manufacturer is known to make this ingredient in general
    #[serde(rename = "possible")]
    Possible,
    /// Manufacturer actually makes this particular ingredient
    #[serde(rename = "actual")]
    Actual,
}

/// A classification of the ingredient identifying its purpose within the
/// product, e.g. active, inactive.
///
/// System: <http://hl7.org/fhir/ingredient-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IngredientRole {
    /// Active
    #[default]
    #[serde(rename = "100000072072")]
    N100000072072,
    /// Adjuvant
    #[serde(rename = "100000072073")]
    N100000072073,
    /// Excipient
    #[serde(rename = "100000072082")]
    N100000072082,
    /// Starting material for excipient
    #[serde(rename = "100000136065")]
    N100000136065,
    /// Solvent / Diluent
    #[serde(rename = "100000136066")]
    N100000136066,
    /// Raw materials used in the manufacture of the product
    #[serde(rename = "100000136178")]
    N100000136178,
    /// Starting material for active substance
    #[serde(rename = "100000136179")]
    N100000136179,
    /// Overage
    #[serde(rename = "100000136561")]
    N100000136561,
    /// bioenhancer
    #[serde(rename = "200000003427")]
    N200000003427,
}

/// A categorisation for a frequency of occurence of an undesirable effect.
///
/// System: <http://hl7.org/fhir/interaction-incidence>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InteractionIncidence {
    /// Theoretical
    #[default]
    #[serde(rename = "Theoretical")]
    Theoretical,
    /// Observed
    #[serde(rename = "Observed")]
    Observed,
}

/// A categorisation for an interaction between two substances.
///
/// System: <http://hl7.org/fhir/interaction-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InteractionType {
    /// drug to drug interaction
    #[default]
    #[serde(rename = "drug-drug")]
    DrugDrug,
    /// drug to food interaction
    #[serde(rename = "drug-food")]
    DrugFood,
    /// drug to laboratory test interaction
    #[serde(rename = "drug-test")]
    DrugTest,
    /// other interaction
    #[serde(rename = "other")]
    Other,
}

/// InventoryItem Name Type
///
/// System: <http://hl7.org/fhir/inventoryitem-nametype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InventoryitemNametype {
    /// Trade Name
    #[default]
    #[serde(rename = "trade-name")]
    TradeName,
    /// Alias
    #[serde(rename = "alias")]
    Alias,
    /// Original Name
    #[serde(rename = "original-name")]
    OriginalName,
    /// Preferred
    #[serde(rename = "preferred")]
    Preferred,
}

/// InventoryItem Status Codes
///
/// System: <http://hl7.org/fhir/inventoryitem-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InventoryitemStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The type of count.
///
/// System: <http://hl7.org/fhir/inventoryreport-counttype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InventoryreportCounttype {
    /// Snapshot
    #[default]
    #[serde(rename = "snapshot")]
    Snapshot,
    /// Difference
    #[serde(rename = "difference")]
    Difference,
}

/// The status of the InventoryReport.
///
/// System: <http://hl7.org/fhir/inventoryreport-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InventoryreportStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Codes identifying the lifecycle stage of an Invoice.
///
/// System: <http://hl7.org/fhir/invoice-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InvoiceStatus {
    /// draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// issued
    #[serde(rename = "issued")]
    Issued,
    /// balanced
    #[serde(rename = "balanced")]
    Balanced,
    /// cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// How the issue affects the success of the action.
///
/// System: <http://hl7.org/fhir/issue-severity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IssueSeverity {
    /// Fatal
    #[default]
    #[serde(rename = "fatal")]
    Fatal,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Information
    #[serde(rename = "information")]
    Information,
    /// Operation Successful
    #[serde(rename = "success")]
    Success,
}

/// A code that describes the type of issue.
///
/// System: <http://hl7.org/fhir/issue-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IssueType {
    /// Invalid Content
    #[default]
    #[serde(rename = "invalid")]
    Invalid,
    /// Structural Issue
    #[serde(rename = "structure")]
    Structure,
    /// Required element missing
    #[serde(rename = "required")]
    Required,
    /// Element value invalid
    #[serde(rename = "value")]
    Value,
    /// Validation rule failed
    #[serde(rename = "invariant")]
    Invariant,
    /// Security Problem
    #[serde(rename = "security")]
    Security,
    /// Login Required
    #[serde(rename = "login")]
    Login,
    /// Unknown User
    #[serde(rename = "unknown")]
    Unknown,
    /// Session Expired
    #[serde(rename = "expired")]
    Expired,
    /// Forbidden
    #[serde(rename = "forbidden")]
    Forbidden,
    /// Information Suppressed
    #[serde(rename = "suppressed")]
    Suppressed,
    /// Processing Failure
    #[serde(rename = "processing")]
    Processing,
    /// Content not supported
    #[serde(rename = "not-supported")]
    NotSupported,
    /// Duplicate
    #[serde(rename = "duplicate")]
    Duplicate,
    /// Multiple Matches
    #[serde(rename = "multiple-matches")]
    MultipleMatches,
    /// Not Found
    #[serde(rename = "not-found")]
    NotFound,
    /// Deleted
    #[serde(rename = "deleted")]
    Deleted,
    /// Content Too Long
    #[serde(rename = "too-long")]
    TooLong,
    /// Invalid Code
    #[serde(rename = "code-invalid")]
    CodeInvalid,
    /// Unacceptable Extension
    #[serde(rename = "extension")]
    Extension,
    /// Operation Too Costly
    #[serde(rename = "too-costly")]
    TooCostly,
    /// Business Rule Violation
    #[serde(rename = "business-rule")]
    BusinessRule,
    /// Edit Version Conflict
    #[serde(rename = "conflict")]
    Conflict,
    /// Limited Filter Application
    #[serde(rename = "limited-filter")]
    LimitedFilter,
    /// Transient Issue
    #[serde(rename = "transient")]
    Transient,
    /// Lock Error
    #[serde(rename = "lock-error")]
    LockError,
    /// No Store Available
    #[serde(rename = "no-store")]
    NoStore,
    /// Exception
    #[serde(rename = "exception")]
    Exception,
    /// Timeout
    #[serde(rename = "timeout")]
    Timeout,
    /// Incomplete Results
    #[serde(rename = "incomplete")]
    Incomplete,
    /// Throttled
    #[serde(rename = "throttled")]
    Throttled,
    /// Informational Note
    #[serde(rename = "informational")]
    Informational,
    /// Operation Successful
    #[serde(rename = "success")]
    Success,
}

/// Distinguishes groups from questions and display text and indicates data
/// type for questions.
///
/// System: <http://hl7.org/fhir/item-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ItemType {
    /// Group
    #[default]
    #[serde(rename = "group")]
    Group,
    /// Display
    #[serde(rename = "display")]
    Display,
    /// Question
    #[serde(rename = "question")]
    Question,
    /// Boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// Decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// Integer
    #[serde(rename = "integer")]
    Integer,
    /// Date
    #[serde(rename = "date")]
    Date,
    /// Date Time
    #[serde(rename = "dateTime")]
    DateTime,
    /// Time
    #[serde(rename = "time")]
    Time,
    /// String
    #[serde(rename = "string")]
    String,
    /// Text
    #[serde(rename = "text")]
    Text,
    /// Url
    #[serde(rename = "url")]
    Url,
    /// Coding
    #[serde(rename = "coding")]
    Coding,
    /// Attachment
    #[serde(rename = "attachment")]
    Attachment,
    /// Reference
    #[serde(rename = "reference")]
    Reference,
    /// Quantity
    #[serde(rename = "quantity")]
    Quantity,
}

/// A knowledge representation level, narrative, semi-structured, structured,
/// and executable
///
/// System: <http://hl7.org/fhir/CodeSystem/knowledge-representation-level>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum KnowledgeRepresentationLevel {
    /// Narrative
    #[default]
    #[serde(rename = "narrative")]
    Narrative,
    /// Semi-Structured
    #[serde(rename = "semi-structured")]
    SemiStructured,
    /// Structured
    #[serde(rename = "structured")]
    Structured,
    /// Executable
    #[serde(rename = "executable")]
    Executable,
}

/// The prescription supply types appropriate to a medicinal product
///
/// System: <http://hl7.org/fhir/legal-status-of-supply>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LegalStatusOfSupply {
    /// Medicinal product not subject to medical prescription
    #[default]
    #[serde(rename = "100000072076")]
    N100000072076,
    /// Medicinal product on medical prescription for renewable or
    /// non-renewable delivery
    #[serde(rename = "100000072077")]
    N100000072077,
    /// Medicinal product subject to restricted medical prescription
    #[serde(rename = "100000072078")]
    N100000072078,
    /// Medicinal product on medical prescription for non-renewable delivery
    #[serde(rename = "100000072079")]
    N100000072079,
    /// Medicinal product subject to medical prescription
    #[serde(rename = "100000072084")]
    N100000072084,
    /// Medicinal product subject to special medical prescription
    #[serde(rename = "100000072085")]
    N100000072085,
    /// Medicinal product on medical prescription for renewable delivery
    #[serde(rename = "100000072086")]
    N100000072086,
    /// Medicinal product subject to special and restricted medical
    /// prescription
    #[serde(rename = "100000157313")]
    N100000157313,
}

/// The type of link between this Patient resource and other
/// Patient/RelatedPerson resource(s).
///
/// System: <http://hl7.org/fhir/link-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LinkType {
    /// Replaced-by
    #[default]
    #[serde(rename = "replaced-by")]
    ReplacedBy,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Refer
    #[serde(rename = "refer")]
    Refer,
    /// See also
    #[serde(rename = "seealso")]
    Seealso,
}

/// Used to distinguish different roles a resource can play within a set of
/// linked resources.
///
/// System: <http://hl7.org/fhir/linkage-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LinkageType {
    /// Source of Truth
    #[default]
    #[serde(rename = "source")]
    Source,
    /// Alternate Record
    #[serde(rename = "alternate")]
    Alternate,
    /// Historical/Obsolete Record
    #[serde(rename = "historical")]
    Historical,
}

/// The processing mode that applies to this list.
///
/// System: <http://hl7.org/fhir/list-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ListMode {
    /// Working List
    #[default]
    #[serde(rename = "working")]
    Working,
    /// Snapshot List
    #[serde(rename = "snapshot")]
    Snapshot,
    /// Change List
    #[serde(rename = "changes")]
    Changes,
}

/// The current state of the list.
///
/// System: <http://hl7.org/fhir/list-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ListStatus {
    /// Current
    #[default]
    #[serde(rename = "current")]
    Current,
    /// Retired
    #[serde(rename = "retired")]
    Retired,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Example Set of Location Characteristics.
///
/// System: <http://hl7.org/fhir/location-characteristic>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LocationCharacteristic {
    /// Wheelchair accessible
    #[default]
    #[serde(rename = "wheelchair")]
    Wheelchair,
    /// translation services available
    #[serde(rename = "has-translation")]
    HasTranslation,
    /// oxygen/nitrogen available
    #[serde(rename = "has-oxy-nitro")]
    HasOxyNitro,
    /// negative pressure rooms available
    #[serde(rename = "has-neg-press")]
    HasNegPress,
    /// isolation ward
    #[serde(rename = "has-iso-ward")]
    HasIsoWard,
    /// has ICU
    #[serde(rename = "has-icu")]
    HasIcu,
}

/// Indicates whether a resource instance represents a specific location or a
/// class of locations.
///
/// System: <http://hl7.org/fhir/location-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LocationMode {
    /// Instance
    #[default]
    #[serde(rename = "instance")]
    Instance,
    /// Kind
    #[serde(rename = "kind")]
    Kind,
}

/// Indicates whether the location is still in use.
///
/// System: <http://hl7.org/fhir/location-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LocationStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}

/// Dose form for a medication, as manufactured (and before any mixing etc.),
/// not necessarily ready for administration to the patient.
///
/// System: <http://hl7.org/fhir/manufactured-dose-form>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ManufacturedDoseForm {
    /// Oral suspension
    #[default]
    #[serde(rename = "100000073362")]
    N100000073362,
    /// Oral gel
    #[serde(rename = "100000073363")]
    N100000073363,
    /// Powder for oral solution
    #[serde(rename = "100000073364")]
    N100000073364,
    /// Granules for oral solution
    #[serde(rename = "100000073365")]
    N100000073365,
    /// Lyophilisate for suspension
    #[serde(rename = "100000073367")]
    N100000073367,
    /// Powder for syrup
    #[serde(rename = "100000073368")]
    N100000073368,
    /// Soluble tablet
    #[serde(rename = "100000073369")]
    N100000073369,
    /// Herbal tea
    #[serde(rename = "100000073370")]
    N100000073370,
    /// Instant herbal tea
    #[serde(rename = "100000073371")]
    N100000073371,
    /// Granules
    #[serde(rename = "100000073372")]
    N100000073372,
    /// Gastro-resistant granules
    #[serde(rename = "100000073373")]
    N100000073373,
    /// Modified-release granules
    #[serde(rename = "100000073374")]
    N100000073374,
    /// Capsule, hard
    #[serde(rename = "100000073375")]
    N100000073375,
    /// Gastro-resistant capsule, hard
    #[serde(rename = "100000073376")]
    N100000073376,
    /// Chewable capsule, soft
    #[serde(rename = "100000073377")]
    N100000073377,
    /// Prolonged-release capsule, soft
    #[serde(rename = "100000073378")]
    N100000073378,
    /// Modified-release capsule, soft
    #[serde(rename = "100000073379")]
    N100000073379,
    /// Coated tablet
    #[serde(rename = "100000073380")]
    N100000073380,
    /// Oral drops, solution
    #[serde(rename = "100000073642")]
    N100000073642,
    /// Oral drops, suspension
    #[serde(rename = "100000073643")]
    N100000073643,
    /// Oral drops, emulsion
    #[serde(rename = "100000073644")]
    N100000073644,
    /// Oral liquid
    #[serde(rename = "100000073645")]
    N100000073645,
    /// Oral solution
    #[serde(rename = "100000073646")]
    N100000073646,
    /// Oral emulsion
    #[serde(rename = "100000073647")]
    N100000073647,
    /// Oral paste
    #[serde(rename = "100000073648")]
    N100000073648,
    /// Powder for oral suspension
    #[serde(rename = "100000073649")]
    N100000073649,
    /// Granules for oral suspension
    #[serde(rename = "100000073650")]
    N100000073650,
    /// Syrup
    #[serde(rename = "100000073652")]
    N100000073652,
    /// Granules for syrup
    #[serde(rename = "100000073653")]
    N100000073653,
    /// Dispersible tablet
    #[serde(rename = "100000073654")]
    N100000073654,
    /// Oral powder
    #[serde(rename = "100000073655")]
    N100000073655,
    /// Effervescent powder
    #[serde(rename = "100000073656")]
    N100000073656,
    /// Effervescent granules
    #[serde(rename = "100000073657")]
    N100000073657,
    /// Prolonged-release granules
    #[serde(rename = "100000073658")]
    N100000073658,
    /// Cachet
    #[serde(rename = "100000073659")]
    N100000073659,
    /// Capsule, soft
    #[serde(rename = "100000073660")]
    N100000073660,
    /// Gastro-resistant capsule, soft
    #[serde(rename = "100000073661")]
    N100000073661,
    /// Prolonged-release capsule, hard
    #[serde(rename = "100000073662")]
    N100000073662,
    /// Modified-release capsule, hard
    #[serde(rename = "100000073663")]
    N100000073663,
    /// Tablet
    #[serde(rename = "100000073664")]
    N100000073664,
    /// Film-coated tablet
    #[serde(rename = "100000073665")]
    N100000073665,
    /// Orodispersible tablet
    #[serde(rename = "100000073666")]
    N100000073666,
    /// Gastro-resistant tablet
    #[serde(rename = "100000073667")]
    N100000073667,
    /// Modified-release tablet
    #[serde(rename = "100000073668")]
    N100000073668,
    /// Medicated chewing-gum
    #[serde(rename = "100000073669")]
    N100000073669,
    /// Pillules
    #[serde(rename = "100000073670")]
    N100000073670,
    /// Pulsatile-release intraruminal device
    #[serde(rename = "100000073671")]
    N100000073671,
    /// Premix for medicated feeding stuff
    #[serde(rename = "100000073672")]
    N100000073672,
    /// Gargle
    #[serde(rename = "100000073673")]
    N100000073673,
    /// Gargle, powder for solution
    #[serde(rename = "100000073674")]
    N100000073674,
    /// Oromucosal suspension
    #[serde(rename = "100000073675")]
    N100000073675,
    /// Oromucosal spray
    #[serde(rename = "100000073676")]
    N100000073676,
    /// Mouthwash
    #[serde(rename = "100000073677")]
    N100000073677,
    /// Gingival solution
    #[serde(rename = "100000073678")]
    N100000073678,
    /// Oromucosal paste
    #[serde(rename = "100000073679")]
    N100000073679,
    /// Gingival gel
    #[serde(rename = "100000073680")]
    N100000073680,
    /// Effervescent tablet
    #[serde(rename = "100000073681")]
    N100000073681,
    /// Oral lyophilisate
    #[serde(rename = "100000073682")]
    N100000073682,
    /// Prolonged-release tablet
    #[serde(rename = "100000073683")]
    N100000073683,
    /// Chewable tablet
    #[serde(rename = "100000073684")]
    N100000073684,
    /// Oral gum
    #[serde(rename = "100000073685")]
    N100000073685,
    /// Continuous-release intraruminal device
    #[serde(rename = "100000073686")]
    N100000073686,
    /// Lick block
    #[serde(rename = "100000073687")]
    N100000073687,
    /// Medicated pellets
    #[serde(rename = "100000073688")]
    N100000073688,
    /// Concentrate for gargle
    #[serde(rename = "100000073689")]
    N100000073689,
    /// Gargle, tablet for solution
    #[serde(rename = "100000073690")]
    N100000073690,
    /// Oromucosal solution
    #[serde(rename = "100000073691")]
    N100000073691,
    /// Oromucosal drops
    #[serde(rename = "100000073692")]
    N100000073692,
    /// Sublingual spray
    #[serde(rename = "100000073693")]
    N100000073693,
    /// Mouthwash, tablet for solution
    #[serde(rename = "100000073694")]
    N100000073694,
    /// Oromucosal gel
    #[serde(rename = "100000073695")]
    N100000073695,
    /// Oromucosal cream
    #[serde(rename = "100000073696")]
    N100000073696,
    /// Gingival paste
    #[serde(rename = "100000073697")]
    N100000073697,
    /// Sublingual tablet
    #[serde(rename = "100000073698")]
    N100000073698,
    /// Buccal tablet
    #[serde(rename = "100000073699")]
    N100000073699,
    /// Compressed lozenge
    #[serde(rename = "100000073700")]
    N100000073700,
    /// Oromucosal capsule
    #[serde(rename = "100000073701")]
    N100000073701,
    /// Muco-adhesive buccal tablet
    #[serde(rename = "100000073702")]
    N100000073702,
    /// Lozenge
    #[serde(rename = "100000073703")]
    N100000073703,
    /// Pastille
    #[serde(rename = "100000073704")]
    N100000073704,
    /// Dental gel
    #[serde(rename = "100000073705")]
    N100000073705,
    /// Dental insert
    #[serde(rename = "100000073706")]
    N100000073706,
    /// Dental powder
    #[serde(rename = "100000073707")]
    N100000073707,
    /// Dental suspension
    #[serde(rename = "100000073708")]
    N100000073708,
    /// Toothpaste
    #[serde(rename = "100000073709")]
    N100000073709,
    /// Periodontal gel
    #[serde(rename = "100000073710")]
    N100000073710,
    /// Bath additive
    #[serde(rename = "100000073711")]
    N100000073711,
    /// Cream
    #[serde(rename = "100000073712")]
    N100000073712,
    /// Ointment
    #[serde(rename = "100000073713")]
    N100000073713,
    /// Medicated plaster
    #[serde(rename = "100000073714")]
    N100000073714,
    /// Shampoo
    #[serde(rename = "100000073715")]
    N100000073715,
    /// Cutaneous spray, suspension
    #[serde(rename = "100000073716")]
    N100000073716,
    /// Cutaneous liquid
    #[serde(rename = "100000073717")]
    N100000073717,
    /// Concentrate for cutaneous solution
    #[serde(rename = "100000073718")]
    N100000073718,
    /// Cutaneous emulsion
    #[serde(rename = "100000073719")]
    N100000073719,
    /// Cutaneous patch
    #[serde(rename = "100000073720")]
    N100000073720,
    /// Periodontal powder
    #[serde(rename = "100000073721")]
    N100000073721,
    /// Dental stick
    #[serde(rename = "100000073722")]
    N100000073722,
    /// Dental solution
    #[serde(rename = "100000073723")]
    N100000073723,
    /// Dental emulsion
    #[serde(rename = "100000073724")]
    N100000073724,
    /// Periodontal insert
    #[serde(rename = "100000073725")]
    N100000073725,
    /// Gel
    #[serde(rename = "100000073726")]
    N100000073726,
    /// Cutaneous paste
    #[serde(rename = "100000073727")]
    N100000073727,
    /// Cutaneous foam
    #[serde(rename = "100000073728")]
    N100000073728,
    /// Cutaneous spray, solution
    #[serde(rename = "100000073729")]
    N100000073729,
    /// Cutaneous spray, powder
    #[serde(rename = "100000073730")]
    N100000073730,
    /// Cutaneous solution
    #[serde(rename = "100000073731")]
    N100000073731,
    /// Cutaneous suspension
    #[serde(rename = "100000073732")]
    N100000073732,
    /// Cutaneous powder
    #[serde(rename = "100000073733")]
    N100000073733,
    /// Solution for iontophoresis
    #[serde(rename = "100000073734")]
    N100000073734,
    /// Collodion
    #[serde(rename = "100000073735")]
    N100000073735,
    /// Poultice
    #[serde(rename = "100000073736")]
    N100000073736,
    /// Cutaneous sponge
    #[serde(rename = "100000073737")]
    N100000073737,
    /// Collar
    #[serde(rename = "100000073738")]
    N100000073738,
    /// Ear tag
    #[serde(rename = "100000073739")]
    N100000073739,
    /// Dip suspension
    #[serde(rename = "100000073740")]
    N100000073740,
    /// Transdermal patch
    #[serde(rename = "100000073741")]
    N100000073741,
    /// Medicated nail lacquer
    #[serde(rename = "100000073742")]
    N100000073742,
    /// Cutaneous stick
    #[serde(rename = "100000073743")]
    N100000073743,
    /// Impregnated dressing
    #[serde(rename = "100000073744")]
    N100000073744,
    /// Medicated pendant
    #[serde(rename = "100000073745")]
    N100000073745,
    /// Dip solution
    #[serde(rename = "100000073746")]
    N100000073746,
    /// Dip emulsion
    #[serde(rename = "100000073747")]
    N100000073747,
    /// Concentrate for dip suspension
    #[serde(rename = "100000073748")]
    N100000073748,
    /// Powder for dip solution
    #[serde(rename = "100000073749")]
    N100000073749,
    /// Powder for suspension for fish treatment
    #[serde(rename = "100000073750")]
    N100000073750,
    /// Pour-on suspension
    #[serde(rename = "100000073751")]
    N100000073751,
    /// Spot-on solution
    #[serde(rename = "100000073752")]
    N100000073752,
    /// Spot-on emulsion
    #[serde(rename = "100000073753")]
    N100000073753,
    /// Teat dip suspension
    #[serde(rename = "100000073754")]
    N100000073754,
    /// Teat spray solution
    #[serde(rename = "100000073755")]
    N100000073755,
    /// Solution for skin-prick test
    #[serde(rename = "100000073756")]
    N100000073756,
    /// Plaster for provocation test
    #[serde(rename = "100000073757")]
    N100000073757,
    /// Eye gel
    #[serde(rename = "100000073758")]
    N100000073758,
    /// Eye drops, solution
    #[serde(rename = "100000073759")]
    N100000073759,
    /// Eye drops, suspension
    #[serde(rename = "100000073760")]
    N100000073760,
    /// Concentrate for dip solution
    #[serde(rename = "100000073761")]
    N100000073761,
    /// Concentrate for dip emulsion
    #[serde(rename = "100000073762")]
    N100000073762,
    /// Concentrate for solution for fish treatment
    #[serde(rename = "100000073763")]
    N100000073763,
    /// Pour-on solution
    #[serde(rename = "100000073764")]
    N100000073764,
    /// Pour-on emulsion
    #[serde(rename = "100000073765")]
    N100000073765,
    /// Spot-on suspension
    #[serde(rename = "100000073766")]
    N100000073766,
    /// Teat dip solution
    #[serde(rename = "100000073767")]
    N100000073767,
    /// Teat dip emulsion
    #[serde(rename = "100000073768")]
    N100000073768,
    /// Transdermal system
    #[serde(rename = "100000073769")]
    N100000073769,
    /// Solution for skin-scratch test
    #[serde(rename = "100000073770")]
    N100000073770,
    /// Eye cream
    #[serde(rename = "100000073771")]
    N100000073771,
    /// Eye ointment
    #[serde(rename = "100000073772")]
    N100000073772,
    /// Eye drops, emulsion
    #[serde(rename = "100000073773")]
    N100000073773,
    /// Eye drops, solvent for reconstitution
    #[serde(rename = "100000073775")]
    N100000073775,
    /// Eye lotion
    #[serde(rename = "100000073776")]
    N100000073776,
    /// Ophthalmic insert
    #[serde(rename = "100000073777")]
    N100000073777,
    /// Ear cream
    #[serde(rename = "100000073778")]
    N100000073778,
    /// Ear ointment
    #[serde(rename = "100000073779")]
    N100000073779,
    /// Ear drops, suspension
    #[serde(rename = "100000073780")]
    N100000073780,
    /// Eye drops, prolonged-release
    #[serde(rename = "100000073782")]
    N100000073782,
    /// Eye lotion, solvent for reconstitution
    #[serde(rename = "100000073783")]
    N100000073783,
    /// Ophthalmic strip
    #[serde(rename = "100000073784")]
    N100000073784,
    /// Ear gel
    #[serde(rename = "100000073785")]
    N100000073785,
    /// Ear drops, solution
    #[serde(rename = "100000073786")]
    N100000073786,
    /// Ear drops, emulsion
    #[serde(rename = "100000073787")]
    N100000073787,
    /// Ear powder
    #[serde(rename = "100000073788")]
    N100000073788,
    /// Ear spray, suspension
    #[serde(rename = "100000073789")]
    N100000073789,
    /// Ear wash, solution
    #[serde(rename = "100000073790")]
    N100000073790,
    /// Ear tampon
    #[serde(rename = "100000073791")]
    N100000073791,
    /// Nasal cream
    #[serde(rename = "100000073792")]
    N100000073792,
    /// Nasal gel
    #[serde(rename = "100000073793")]
    N100000073793,
    /// Nasal drops, solution
    #[serde(rename = "100000073794")]
    N100000073794,
    /// Nasal drops, emulsion
    #[serde(rename = "100000073795")]
    N100000073795,
    /// Nasal spray, solution
    #[serde(rename = "100000073796")]
    N100000073796,
    /// Nasal spray, emulsion
    #[serde(rename = "100000073797")]
    N100000073797,
    /// Nasal stick
    #[serde(rename = "100000073798")]
    N100000073798,
    /// Vaginal gel
    #[serde(rename = "100000073799")]
    N100000073799,
    /// Vaginal foam
    #[serde(rename = "100000073800")]
    N100000073800,
    /// Ear spray, solution
    #[serde(rename = "100000073802")]
    N100000073802,
    /// Ear spray, emulsion
    #[serde(rename = "100000073803")]
    N100000073803,
    /// Ear wash, emulsion
    #[serde(rename = "100000073804")]
    N100000073804,
    /// Ear stick
    #[serde(rename = "100000073805")]
    N100000073805,
    /// Nasal ointment
    #[serde(rename = "100000073806")]
    N100000073806,
    /// Nasal drops, suspension
    #[serde(rename = "100000073807")]
    N100000073807,
    /// Nasal powder
    #[serde(rename = "100000073808")]
    N100000073808,
    /// Nasal spray, suspension
    #[serde(rename = "100000073809")]
    N100000073809,
    /// Nasal wash
    #[serde(rename = "100000073810")]
    N100000073810,
    /// Vaginal cream
    #[serde(rename = "100000073811")]
    N100000073811,
    /// Vaginal ointment
    #[serde(rename = "100000073812")]
    N100000073812,
    /// Vaginal solution
    #[serde(rename = "100000073813")]
    N100000073813,
    /// Vaginal emulsion
    #[serde(rename = "100000073814")]
    N100000073814,
    /// Pessary
    #[serde(rename = "100000073815")]
    N100000073815,
    /// Vaginal capsule, soft
    #[serde(rename = "100000073816")]
    N100000073816,
    /// Effervescent vaginal tablet
    #[serde(rename = "100000073817")]
    N100000073817,
    /// Vaginal delivery system
    #[serde(rename = "100000073818")]
    N100000073818,
    /// Rectal cream
    #[serde(rename = "100000073819")]
    N100000073819,
    /// Rectal foam
    #[serde(rename = "100000073820")]
    N100000073820,
    /// Vaginal suspension
    #[serde(rename = "100000073821")]
    N100000073821,
    /// Tablet for vaginal solution
    #[serde(rename = "100000073822")]
    N100000073822,
    /// Vaginal capsule, hard
    #[serde(rename = "100000073823")]
    N100000073823,
    /// Vaginal tablet
    #[serde(rename = "100000073824")]
    N100000073824,
    /// Medicated vaginal tampon
    #[serde(rename = "100000073825")]
    N100000073825,
    /// Vaginal sponge
    #[serde(rename = "100000073826")]
    N100000073826,
    /// Rectal gel
    #[serde(rename = "100000073827")]
    N100000073827,
    /// Solution for injection
    #[serde(rename = "100000073863")]
    N100000073863,
}

/// If this is the default rule set to apply for the source type, or this
/// combination of types.
///
/// System: <http://hl7.org/fhir/map-group-type-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapGroupTypeMode {
    /// Default for Type Combination
    #[default]
    #[serde(rename = "types")]
    Types,
    /// Default for type + combination
    #[serde(rename = "type-and-types")]
    TypeAndTypes,
}

/// Mode for this instance of data.
///
/// System: <http://hl7.org/fhir/map-input-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapInputMode {
    /// Source Instance
    #[default]
    #[serde(rename = "source")]
    Source,
    /// Target Instance
    #[serde(rename = "target")]
    Target,
}

/// How the referenced structure is used in this mapping.
///
/// System: <http://hl7.org/fhir/map-model-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapModelMode {
    /// Source Structure Definition
    #[default]
    #[serde(rename = "source")]
    Source,
    /// Queried Structure Definition
    #[serde(rename = "queried")]
    Queried,
    /// Target Structure Definition
    #[serde(rename = "target")]
    Target,
    /// Produced Structure Definition
    #[serde(rename = "produced")]
    Produced,
}

/// If field is a list, how to manage the source.
///
/// System: <http://hl7.org/fhir/map-source-list-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapSourceListMode {
    /// First
    #[default]
    #[serde(rename = "first")]
    First,
    /// All but the first
    #[serde(rename = "not_first")]
    NotFirst,
    /// Last
    #[serde(rename = "last")]
    Last,
    /// All but the last
    #[serde(rename = "not_last")]
    NotLast,
    /// Enforce only one
    #[serde(rename = "only_one")]
    OnlyOne,
}

/// If field is a list, how to manage the production.
///
/// System: <http://hl7.org/fhir/map-target-list-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapTargetListMode {
    /// First
    #[default]
    #[serde(rename = "first")]
    First,
    /// Share
    #[serde(rename = "share")]
    Share,
    /// Last
    #[serde(rename = "last")]
    Last,
    /// single
    #[serde(rename = "single")]
    Single,
}

/// How data is copied/created.
///
/// System: <http://hl7.org/fhir/map-transform>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapTransform {
    /// create
    #[default]
    #[serde(rename = "create")]
    Create,
    /// copy
    #[serde(rename = "copy")]
    Copy,
    /// truncate
    #[serde(rename = "truncate")]
    Truncate,
    /// escape
    #[serde(rename = "escape")]
    Escape,
    /// cast
    #[serde(rename = "cast")]
    Cast,
    /// append
    #[serde(rename = "append")]
    Append,
    /// translate
    #[serde(rename = "translate")]
    Translate,
    /// reference
    #[serde(rename = "reference")]
    Reference,
    /// dateOp
    #[serde(rename = "dateOp")]
    DateOp,
    /// uuid
    #[serde(rename = "uuid")]
    Uuid,
    /// pointer
    #[serde(rename = "pointer")]
    Pointer,
    /// evaluate
    #[serde(rename = "evaluate")]
    Evaluate,
    /// cc
    #[serde(rename = "cc")]
    Cc,
    /// c
    #[serde(rename = "c")]
    C,
    /// qty
    #[serde(rename = "qty")]
    Qty,
    /// id
    #[serde(rename = "id")]
    Id,
    /// cp
    #[serde(rename = "cp")]
    Cp,
}

/// Aggregation method for a measure (e.g. sum, average, median, minimum,
/// maximum, count)
///
/// System: <http://hl7.org/fhir/CodeSystem/measure-aggregate-method>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureAggregateMethod {
    /// Sum
    #[default]
    #[serde(rename = "sum")]
    Sum,
    /// Average
    #[serde(rename = "average")]
    Average,
    /// Median
    #[serde(rename = "median")]
    Median,
    /// Minimum
    #[serde(rename = "minimum")]
    Minimum,
    /// Maximum
    #[serde(rename = "maximum")]
    Maximum,
    /// Count
    #[serde(rename = "count")]
    Count,
}

/// Example Measure Definitions for the Measure Resource.
///
/// System: <http://hl7.org/fhir/measure-definition-example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureDefinitionExample {
    /// Screening
    #[default]
    #[serde(rename = "screening")]
    Screening,
    /// Standardized Depression Screening Tool
    #[serde(rename = "standardized-depression-screening-tool")]
    StandardizedDepressionScreeningTool,
}

/// Example Measure Groups for the Measure Resource.
///
/// System: <http://hl7.org/fhir/measure-group-example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureGroupExample {
    /// Primary Rate
    #[default]
    #[serde(rename = "primary-rate")]
    PrimaryRate,
    /// Secondary Rate
    #[serde(rename = "secondary-rate")]
    SecondaryRate,
}

/// The status of the measure report.
///
/// System: <http://hl7.org/fhir/measure-report-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureReportStatus {
    /// Complete
    #[default]
    #[serde(rename = "complete")]
    Complete,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
    /// Error
    #[serde(rename = "error")]
    Error,
}

/// The type of the measure report.
///
/// System: <http://hl7.org/fhir/measure-report-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureReportType {
    /// Individual
    #[default]
    #[serde(rename = "individual")]
    Individual,
    /// Subject List
    #[serde(rename = "subject-list")]
    SubjectList,
    /// Summary
    #[serde(rename = "summary")]
    Summary,
    /// Data Exchange
    #[serde(rename = "data-exchange")]
    DataExchange,
}

/// Identifier subgroups in a population for measuring purposes.
///
/// System: <http://hl7.org/fhir/measure-stratifier-example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureStratifierExample {
    /// Age
    #[default]
    #[serde(rename = "age")]
    Age,
    /// Gender
    #[serde(rename = "gender")]
    Gender,
    /// Region
    #[serde(rename = "region")]
    Region,
}

/// Identifier supplemental data in a population for measuring purposes.
///
/// System: <http://hl7.org/fhir/measure-supplemental-data-example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasureSupplementalDataExample {
    /// Age
    #[default]
    #[serde(rename = "age")]
    Age,
    /// Gender
    #[serde(rename = "gender")]
    Gender,
    /// Ethnicity
    #[serde(rename = "ethnicity")]
    Ethnicity,
    /// Payer
    #[serde(rename = "payer")]
    Payer,
}

/// Example Region Value Measure Groups for the Measure Resource.
///
/// System: <http://hl7.org/fhir/measurereport-stratifier-value-example>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasurereportStratifierValueExample {
    /// Northwest
    #[default]
    #[serde(rename = "northwest")]
    Northwest,
    /// Northeast
    #[serde(rename = "northeast")]
    Northeast,
    /// Soutwest
    #[serde(rename = "southwest")]
    Southwest,
    /// Southeast
    #[serde(rename = "southeast")]
    Southeast,
}

/// MedicationAdministration Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-admin-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationAdminStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Not Done
    #[serde(rename = "not-done")]
    NotDone,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Medication Cost Category Codes
///
/// System: <http://hl7.org/fhir/medication-cost-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationCostCategory {
    /// Band A
    #[default]
    #[serde(rename = "banda")]
    Banda,
    /// Band B
    #[serde(rename = "bandb")]
    Bandb,
}

/// Medication dose aid
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-dose-aid>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationDoseAid {
    /// Blister Pack
    #[default]
    #[serde(rename = "blisterpack")]
    Blisterpack,
    /// Dosette
    #[serde(rename = "dosette")]
    Dosette,
    /// Sachets
    #[serde(rename = "sachets")]
    Sachets,
}

/// Medication Ingredient Strength Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-ingredientstrength>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationIngredientstrength {
    /// QS
    #[default]
    #[serde(rename = "qs")]
    Qs,
    /// Trace
    #[serde(rename = "trace")]
    Trace,
}

/// Medication Intended Performer Role
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-intended-performer-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationIntendedPerformerRole {
    /// Registered Nurse
    #[default]
    #[serde(rename = "registerednurse")]
    Registerednurse,
    /// Oncology Nurse
    #[serde(rename = "oncologynurse")]
    Oncologynurse,
    /// Pain Control Nurse
    #[serde(rename = "paincontrolnurse")]
    Paincontrolnurse,
    /// Physician
    #[serde(rename = "physician")]
    Physician,
    /// Pharmacist
    #[serde(rename = "pharmacist")]
    Pharmacist,
}

/// MedicationStatement Adherence Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-statement-adherence>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationStatementAdherence {
    /// Taking
    #[default]
    #[serde(rename = "taking")]
    Taking,
    /// Taking As Directed
    #[serde(rename = "taking-as-directed")]
    TakingAsDirected,
    /// Taking Not As Directed
    #[serde(rename = "taking-not-as-directed")]
    TakingNotAsDirected,
    /// Not Taking
    #[serde(rename = "not-taking")]
    NotTaking,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// On Hold As Directed
    #[serde(rename = "on-hold-as-directed")]
    OnHoldAsDirected,
    /// On Hold Not As Directed
    #[serde(rename = "on-hold-not-as-directed")]
    OnHoldNotAsDirected,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Stopped As Directed
    #[serde(rename = "stopped-as-directed")]
    StoppedAsDirected,
    /// Stopped Not As Directed
    #[serde(rename = "stopped-not-as-directed")]
    StoppedNotAsDirected,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// MedicationStatement Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-statement-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationStatementStatus {
    /// Recorded
    #[default]
    #[serde(rename = "recorded")]
    Recorded,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
}

/// Medication Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medication-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Medication Dispense Administration Location Codes
///
/// System: <http://hl7.org/fhir/medicationdispense-admin-location>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationdispenseAdminLocation {
    /// Inpatient
    #[default]
    #[serde(rename = "inpatient")]
    Inpatient,
    /// Outpatient
    #[serde(rename = "outpatient")]
    Outpatient,
    /// Community
    #[serde(rename = "community")]
    Community,
}

/// Medication Dispense Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medicationdispense-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationdispenseStatus {
    /// Preparation
    #[default]
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Medication Dispense Status Reason Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medicationdispense-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationdispenseStatusReason {
    /// Order Stopped
    #[default]
    #[serde(rename = "frr01")]
    Frr01,
    /// Stale-dated Order
    #[serde(rename = "frr02")]
    Frr02,
    /// Incomplete data
    #[serde(rename = "frr03")]
    Frr03,
    /// Product unavailable
    #[serde(rename = "frr04")]
    Frr04,
    /// Ethical/religious
    #[serde(rename = "frr05")]
    Frr05,
    /// Unable to provide care
    #[serde(rename = "frr06")]
    Frr06,
    /// Try another treatment first
    #[serde(rename = "altchoice")]
    Altchoice,
    /// Prescription/Request requires clarification
    #[serde(rename = "clarif")]
    Clarif,
    /// Drug level too high
    #[serde(rename = "drughigh")]
    Drughigh,
    /// Admission to hospital
    #[serde(rename = "hospadm")]
    Hospadm,
    /// Lab interference issues
    #[serde(rename = "labint")]
    Labint,
    /// Patient not available
    #[serde(rename = "non-avail")]
    NonAvail,
    /// Patient is pregnant or breastfeeding
    #[serde(rename = "preg")]
    Preg,
    /// Allergy
    #[serde(rename = "saig")]
    Saig,
    /// Drug interacts with another drug
    #[serde(rename = "sddi")]
    Sddi,
    /// Duplicate therapy
    #[serde(rename = "sdupther")]
    Sdupther,
    /// Suspected intolerance
    #[serde(rename = "sintol")]
    Sintol,
    /// Patient scheduled for surgery
    #[serde(rename = "surg")]
    Surg,
    /// Washout
    #[serde(rename = "washout")]
    Washout,
    /// Drug not available - out of stock
    #[serde(rename = "outofstock")]
    Outofstock,
    /// Drug not available - off market
    #[serde(rename = "offmarket")]
    Offmarket,
}

/// MedicationKnowledge Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medicationknowledge-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationknowledgeStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}

/// MedicationRequest Intent Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medicationrequest-intent>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationrequestIntent {
    /// Proposal
    #[default]
    #[serde(rename = "proposal")]
    Proposal,
    /// Plan
    #[serde(rename = "plan")]
    Plan,
    /// Order
    #[serde(rename = "order")]
    Order,
    /// Original Order
    #[serde(rename = "original-order")]
    OriginalOrder,
    /// Reflex Order
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    /// Filler Order
    #[serde(rename = "filler-order")]
    FillerOrder,
    /// Instance Order
    #[serde(rename = "instance-order")]
    InstanceOrder,
    /// Option
    #[serde(rename = "option")]
    Option,
}

/// MedicationRequest Status Codes
///
/// System: <http://hl7.org/fhir/CodeSystem/medicationrequest-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationrequestStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Ended
    #[serde(rename = "ended")]
    Ended,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Extra monitoring defined for a Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-additional-monitoring>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductAdditionalMonitoring {
    /// Requirement for Black Triangle Monitoring
    #[default]
    #[serde(rename = "BlackTriangleMonitoring")]
    BlackTriangleMonitoring,
}

/// Confidentiality rating, e.g. commercial sensitivity for a Medicinal
/// Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-confidentiality>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductConfidentiality {
    /// Commercially Sensitive
    #[default]
    #[serde(rename = "CommerciallySensitive")]
    CommerciallySensitive,
    /// Not Commercially Sensitive
    #[serde(rename = "NotCommerciallySensitive")]
    NotCommerciallySensitive,
}

/// Contact type for a Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-contact-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductContactType {
    /// Proposed Marketing Authorization Holder/Person
    #[default]
    #[serde(rename = "ProposedMAH")]
    ProposedMah,
    /// Person/Company authorised for Communication during procedure
    #[serde(rename = "ProcedureContactDuring")]
    ProcedureContactDuring,
    /// Person/Company authorised for Communication after procedure
    #[serde(rename = "ProcedureContactAfter")]
    ProcedureContactAfter,
    /// Qualified Person Responsible for Pharmacovigilance
    #[serde(rename = "QPPV")]
    Qppv,
    /// Pharmacovigilance Enquiry Information
    #[serde(rename = "PVEnquiries")]
    PvEnquiries,
}

/// Relationship to another Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-cross-reference-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductCrossReferenceType {
    /// Link to Investigational Product
    #[default]
    #[serde(rename = "InvestigationalProduct")]
    InvestigationalProduct,
    /// Link Actual to Virtual Product
    #[serde(rename = "VirtualProduct")]
    VirtualProduct,
    /// Link Virtual to Actual Product
    #[serde(rename = "ActualProduct")]
    ActualProduct,
    /// Link Generic to Branded Product
    #[serde(rename = "BrandedProduct")]
    BrandedProduct,
    /// Link Branded to Generic Product
    #[serde(rename = "GenericProduct")]
    GenericProduct,
    /// Link to Parallel Import Product
    #[serde(rename = "Parallel")]
    Parallel,
}

/// Applicable domain for this product (e.g. human, veterinary).
///
/// System: <http://hl7.org/fhir/medicinal-product-domain>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductDomain {
    /// Human use
    #[default]
    #[serde(rename = "Human")]
    Human,
    /// Veterinary use
    #[serde(rename = "Veterinary")]
    Veterinary,
    /// Human and Veterinary use
    #[serde(rename = "HumanAndVeterinary")]
    HumanAndVeterinary,
}

/// Type of part of a name for a Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-name-part-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductNamePartType {
    /// Full name
    #[default]
    #[serde(rename = "FullName")]
    FullName,
    /// Invented name part
    #[serde(rename = "InventedNamePart")]
    InventedNamePart,
    /// Scientific name part
    #[serde(rename = "ScientificNamePart")]
    ScientificNamePart,
    /// Strength part
    #[serde(rename = "StrengthPart")]
    StrengthPart,
    /// Pharmaceutical dose form part
    #[serde(rename = "DoseFormPart")]
    DoseFormPart,
    /// Formulation part
    #[serde(rename = "FormulationPart")]
    FormulationPart,
    /// Intended use part
    #[serde(rename = "IntendedUsePart")]
    IntendedUsePart,
    /// Target population part
    #[serde(rename = "PopulationPart")]
    PopulationPart,
    /// Container or pack part
    #[serde(rename = "ContainerPart")]
    ContainerPart,
    /// Device part
    #[serde(rename = "DevicePart")]
    DevicePart,
    /// Trademark or company name part
    #[serde(rename = "TrademarkOrCompanyPart")]
    TrademarkOrCompanyPart,
    /// Time/Period part
    #[serde(rename = "TimeOrPeriodPart")]
    TimeOrPeriodPart,
    /// Flavor part
    #[serde(rename = "FlavorPart")]
    FlavorPart,
    /// Delimiter part
    #[serde(rename = "DelimiterPart")]
    DelimiterPart,
    /// Legacy name
    #[serde(rename = "LegacyNamePart")]
    LegacyNamePart,
    /// Target species name part
    #[serde(rename = "SpeciesNamePart")]
    SpeciesNamePart,
}

/// Type of a name for a Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-name-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductNameType {
    /// British Approved Name
    #[default]
    #[serde(rename = "BAN")]
    Ban,
    /// International Non-Proprietary Name
    #[serde(rename = "INN")]
    Inn,
    /// Modified International Non-Proprietary Name
    #[serde(rename = "INNM")]
    Innm,
    /// Proposed International Non-Proprietary Name
    #[serde(rename = "pINN")]
    PInn,
    /// Recommended International Non-Proprietary Name
    #[serde(rename = "rINN")]
    RInn,
}

/// Types of medicinal product packs
///
/// System: <http://hl7.org/fhir/medicinal-product-package-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductPackageType {
    /// Ampoule
    #[default]
    #[serde(rename = "100000073490")]
    N100000073490,
    /// Applicator
    #[serde(rename = "100000073491")]
    N100000073491,
    /// Automatic injection device
    #[serde(rename = "100000073492")]
    N100000073492,
    /// Bag
    #[serde(rename = "100000073493")]
    N100000073493,
    /// Balling gun
    #[serde(rename = "100000073494")]
    N100000073494,
    /// Barrel
    #[serde(rename = "100000073495")]
    N100000073495,
    /// Blister
    #[serde(rename = "100000073496")]
    N100000073496,
    /// Bottle
    #[serde(rename = "100000073497")]
    N100000073497,
    /// Box
    #[serde(rename = "100000073498")]
    N100000073498,
    /// Sachet
    #[serde(rename = "100000073547")]
    N100000073547,
    /// Vial
    #[serde(rename = "100000073563")]
    N100000073563,
    /// Pack
    #[serde(rename = "100000143555")]
    N100000143555,
}

/// Suitability for age groups, in particular children.
///
/// System: <http://hl7.org/fhir/medicinal-product-pediatric-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductPediatricUse {
    /// In utero
    #[default]
    #[serde(rename = "InUtero")]
    InUtero,
    /// Preterm newborn infants (0 – 27 days)
    #[serde(rename = "PretermNewborn")]
    PretermNewborn,
    /// Term newborn infants (0 – 27 days)
    #[serde(rename = "TermNewborn")]
    TermNewborn,
    /// Infants and toddlers (28 days – 23 months)
    #[serde(rename = "Infants")]
    Infants,
    /// Children (2 to < 12 years)
    #[serde(rename = "Children")]
    Children,
    /// Adolescents (12 to < 18 years)
    #[serde(rename = "Adolescents")]
    Adolescents,
    /// Adults (18 to < 65 years)
    #[serde(rename = "Adults")]
    Adults,
    /// Elderly (≥ 65 years)
    #[serde(rename = "Elderly")]
    Elderly,
    /// Neonate
    #[serde(rename = "Neonate")]
    Neonate,
    /// Pediatric Population (< 18 years)
    #[serde(rename = "PediatricPopulation")]
    PediatricPopulation,
    /// All
    #[serde(rename = "All")]
    All,
    /// Prepubertal children (2 years to onset of puberty)
    #[serde(rename = "Prepubertal")]
    Prepubertal,
    /// Adult and elderly population (> 18 years)
    #[serde(rename = "AdultsAndElderly")]
    AdultsAndElderly,
    /// Pubertal and postpubertal adolescents (onset of puberty to < 18 years)
    #[serde(rename = "PubertalAndPostpubertal")]
    PubertalAndPostpubertal,
}

/// Extra measures defined for a Medicinal Product, such as requirement to
/// conduct post-authorization studies.
///
/// System: <http://hl7.org/fhir/medicinal-product-special-measures>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductSpecialMeasures {
    /// Requirement to conduct post-authorization studies
    #[default]
    #[serde(rename = "Post-authorizationStudies")]
    PostAuthorizationStudies,
}

/// Overall defining type of this Medicinal Product.
///
/// System: <http://hl7.org/fhir/medicinal-product-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicinalProductType {
    /// Medicinal Product
    #[default]
    #[serde(rename = "MedicinalProduct")]
    MedicinalProduct,
    /// Investigational Medicinal Product
    #[serde(rename = "InvestigationalProduct")]
    InvestigationalProduct,
}

/// The impact of the content of a message.
///
/// System: <http://hl7.org/fhir/message-significance-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageSignificanceCategory {
    /// Consequence
    #[default]
    #[serde(rename = "consequence")]
    Consequence,
    /// Currency
    #[serde(rename = "currency")]
    Currency,
    /// Notification
    #[serde(rename = "notification")]
    Notification,
}

/// The protocol used for message transport.
///
/// System: <http://hl7.org/fhir/message-transport>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageTransport {
    /// HTTP
    #[default]
    #[serde(rename = "http")]
    Http,
    /// FTP
    #[serde(rename = "ftp")]
    Ftp,
    /// MLLP
    #[serde(rename = "mllp")]
    Mllp,
}

/// HL7-defined table of codes which identify conditions under which
/// acknowledgments are required to be returned in response to a message.
///
/// System: <http://hl7.org/fhir/messageheader-response-request>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageheaderResponseRequest {
    /// Always
    #[default]
    #[serde(rename = "always")]
    Always,
    /// Error/reject conditions only
    #[serde(rename = "on-error")]
    OnError,
    /// Never
    #[serde(rename = "never")]
    Never,
    /// Successful completion only
    #[serde(rename = "on-success")]
    OnSuccess,
}

/// Describes the state of a metric calibration.
///
/// System: <http://hl7.org/fhir/metric-calibration-state>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MetricCalibrationState {
    /// Not Calibrated
    #[default]
    #[serde(rename = "not-calibrated")]
    NotCalibrated,
    /// Calibration Required
    #[serde(rename = "calibration-required")]
    CalibrationRequired,
    /// Calibrated
    #[serde(rename = "calibrated")]
    Calibrated,
    /// Unspecified
    #[serde(rename = "unspecified")]
    Unspecified,
}

/// Describes the type of a metric calibration.
///
/// System: <http://hl7.org/fhir/metric-calibration-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MetricCalibrationType {
    /// Unspecified
    #[default]
    #[serde(rename = "unspecified")]
    Unspecified,
    /// Offset
    #[serde(rename = "offset")]
    Offset,
    /// Gain
    #[serde(rename = "gain")]
    Gain,
    /// Two Point
    #[serde(rename = "two-point")]
    TwoPoint,
}

/// Describes the category of the metric.
///
/// System: <http://hl7.org/fhir/metric-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MetricCategory {
    /// Measurement
    #[default]
    #[serde(rename = "measurement")]
    Measurement,
    /// Setting
    #[serde(rename = "setting")]
    Setting,
    /// Calculation
    #[serde(rename = "calculation")]
    Calculation,
    /// Unspecified
    #[serde(rename = "unspecified")]
    Unspecified,
}

/// Describes the operational status of the DeviceMetric.
///
/// System: <http://hl7.org/fhir/metric-operational-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MetricOperationalStatus {
    /// On
    #[default]
    #[serde(rename = "on")]
    On,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Standby
    #[serde(rename = "standby")]
    Standby,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The use of a human name.
///
/// System: <http://hl7.org/fhir/name-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NameUse {
    /// Usual
    #[default]
    #[serde(rename = "usual")]
    Usual,
    /// Official
    #[serde(rename = "official")]
    Official,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Nickname
    #[serde(rename = "nickname")]
    Nickname,
    /// Anonymous
    #[serde(rename = "anonymous")]
    Anonymous,
    /// Old
    #[serde(rename = "old")]
    Old,
    /// Name changed for Marriage
    #[serde(rename = "maiden")]
    Maiden,
}

/// Identifies the style of unique identifier used to identify a namespace.
///
/// System: <http://hl7.org/fhir/namingsystem-identifier-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NamingsystemIdentifierType {
    /// OID
    #[default]
    #[serde(rename = "oid")]
    Oid,
    /// UUID
    #[serde(rename = "uuid")]
    Uuid,
    /// URI
    #[serde(rename = "uri")]
    Uri,
    /// IRI stem
    #[serde(rename = "iri-stem")]
    IriStem,
    /// V2CSMNemonic
    #[serde(rename = "v2csmnemonic")]
    V2Csmnemonic,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Identifies the purpose of the naming system.
///
/// System: <http://hl7.org/fhir/namingsystem-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NamingsystemType {
    /// Code System
    #[default]
    #[serde(rename = "codesystem")]
    Codesystem,
    /// Identifier
    #[serde(rename = "identifier")]
    Identifier,
    /// Root
    #[serde(rename = "root")]
    Root,
}

/// The status of a resource narrative.
///
/// System: <http://hl7.org/fhir/narrative-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NarrativeStatus {
    /// Generated
    #[default]
    #[serde(rename = "generated")]
    Generated,
    /// Extensions
    #[serde(rename = "extensions")]
    Extensions,
    /// Additional
    #[serde(rename = "additional")]
    Additional,
    /// Empty
    #[serde(rename = "empty")]
    Empty,
}

/// The presentation types of notes.
///
/// System: <http://hl7.org/fhir/note-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NoteType {
    /// Display
    #[default]
    #[serde(rename = "display")]
    Display,
    /// Print (Form)
    #[serde(rename = "print")]
    Print,
    /// Print (Operator)
    #[serde(rename = "printoper")]
    Printoper,
}

/// Codes identifying the lifecycle stage of a product.
///
/// System: <http://hl7.org/fhir/nutritionproduct-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NutritionproductStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Codes identifying the category of observation range.
///
/// System: <http://hl7.org/fhir/observation-range-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationRangeCategory {
    /// reference range
    #[default]
    #[serde(rename = "reference")]
    Reference,
    /// critical range
    #[serde(rename = "critical")]
    Critical,
    /// absolute range
    #[serde(rename = "absolute")]
    Absolute,
}

/// Codes that describe the normal value in the reference range.
///
/// System: <http://hl7.org/fhir/observation-referencerange-normalvalue>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationReferencerangeNormalvalue {
    /// Negative
    #[default]
    #[serde(rename = "negative")]
    Negative,
    /// Absent
    #[serde(rename = "absent")]
    Absent,
}

/// The statistical operation parameter -"statistic" codes.
///
/// System: <http://hl7.org/fhir/observation-statistics>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationStatistics {
    /// Average
    #[default]
    #[serde(rename = "average")]
    Average,
    /// Maximum
    #[serde(rename = "maximum")]
    Maximum,
    /// Minimum
    #[serde(rename = "minimum")]
    Minimum,
    /// Count
    #[serde(rename = "count")]
    Count,
    /// Total Count
    #[serde(rename = "total-count")]
    TotalCount,
    /// Median
    #[serde(rename = "median")]
    Median,
    /// Standard Deviation
    #[serde(rename = "std-dev")]
    StdDev,
    /// Sum
    #[serde(rename = "sum")]
    Sum,
    /// Variance
    #[serde(rename = "variance")]
    Variance,
    /// 20th Percentile
    #[serde(rename = "20-percent")]
    N20Percent,
    /// 80th Percentile
    #[serde(rename = "80-percent")]
    N80Percent,
    /// Lower Quartile
    #[serde(rename = "4-lower")]
    N4Lower,
    /// Upper Quartile
    #[serde(rename = "4-upper")]
    N4Upper,
    /// Quartile Deviation
    #[serde(rename = "4-dev")]
    N4Dev,
    /// 1st Quintile
    #[serde(rename = "5-1")]
    N51,
    /// 2nd Quintile
    #[serde(rename = "5-2")]
    N52,
    /// 3rd Quintile
    #[serde(rename = "5-3")]
    N53,
    /// 4th Quintile
    #[serde(rename = "5-4")]
    N54,
    /// Skew
    #[serde(rename = "skew")]
    Skew,
    /// Kurtosis
    #[serde(rename = "kurtosis")]
    Kurtosis,
    /// Regression
    #[serde(rename = "regression")]
    Regression,
}

/// Codes providing the status of an observation.
///
/// System: <http://hl7.org/fhir/observation-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationStatus {
    /// Registered
    #[default]
    #[serde(rename = "registered")]
    Registered,
    /// Preliminary
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Corrected
    #[serde(rename = "corrected")]
    Corrected,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Codes providing the triggeredBy type of observation.
///
/// System: <http://hl7.org/fhir/observation-triggeredbytype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationTriggeredbytype {
    /// Reflex
    #[default]
    #[serde(rename = "reflex")]
    Reflex,
    /// Repeat (per policy)
    #[serde(rename = "repeat")]
    Repeat,
    /// Re-run (per policy)
    #[serde(rename = "re-run")]
    ReRun,
}

/// Whether an operation is a normal operation or a query.
///
/// System: <http://hl7.org/fhir/operation-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationKind {
    /// Operation
    #[default]
    #[serde(rename = "operation")]
    Operation,
    /// Query
    #[serde(rename = "query")]
    Query,
}

/// Operation Outcome codes for translatable phrases used by FHIR test servers
/// (see Implementation file translations.xml)
///
/// System: <http://hl7.org/fhir/operation-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationOutcome {
    /// Error: Multiple matches exist for the conditional delete
    #[default]
    #[serde(rename = "DELETE_MULTIPLE_MATCHES")]
    DeleteMultipleMatches,
    /// You must authenticate before you can use this service
    #[serde(rename = "MSG_AUTH_REQUIRED")]
    MsgAuthRequired,
    /// Bad Syntax: "%s" must be a %s'
    #[serde(rename = "MSG_BAD_FORMAT")]
    MsgBadFormat,
    /// Bad Syntax in %s
    #[serde(rename = "MSG_BAD_SYNTAX")]
    MsgBadSyntax,
    /// Unable to parse feed (entry content type = "%s")
    #[serde(rename = "MSG_CANT_PARSE_CONTENT")]
    MsgCantParseContent,
    /// Unable to parse feed (root element name = "%s")
    #[serde(rename = "MSG_CANT_PARSE_ROOT")]
    MsgCantParseRoot,
    /// New resource created
    #[serde(rename = "MSG_CREATED")]
    MsgCreated,
    /// The Date value %s is not in the correct format (Xml Date Format
    /// required)
    #[serde(rename = "MSG_DATE_FORMAT")]
    MsgDateFormat,
    /// This resource has been deleted
    #[serde(rename = "MSG_DELETED")]
    MsgDeleted,
    /// Resource deleted
    #[serde(rename = "MSG_DELETED_DONE")]
    MsgDeletedDone,
    /// The resource "%s" has been deleted
    #[serde(rename = "MSG_DELETED_ID")]
    MsgDeletedId,
    /// Duplicate Id %s for resource type %s
    #[serde(rename = "MSG_DUPLICATE_ID")]
    MsgDuplicateId,
    /// Error parsing resource Xml (%s)
    #[serde(rename = "MSG_ERROR_PARSING")]
    MsgErrorParsing,
    /// Id "%s" has an invalid character "%s"
    #[serde(rename = "MSG_ID_INVALID")]
    MsgIdInvalid,
    /// Id "%s" too long (length limit 36)
    #[serde(rename = "MSG_ID_TOO_LONG")]
    MsgIdTooLong,
    /// Id not accepted
    #[serde(rename = "MSG_INVALID_ID")]
    MsgInvalidId,
    /// Json Source for a resource should start with an object
    #[serde(rename = "MSG_JSON_OBJECT")]
    MsgJsonObject,
    /// Unable to resolve local reference to resource %s
    #[serde(rename = "MSG_LOCAL_FAIL")]
    MsgLocalFail,
    /// Resource Id "%s" does not exist
    #[serde(rename = "MSG_NO_EXIST")]
    MsgNoExist,
    /// No Resource found matching the query "%s"
    #[serde(rename = "MSG_NO_MATCH")]
    MsgNoMatch,
    /// No module could be found to handle the request "%s"
    #[serde(rename = "MSG_NO_MODULE")]
    MsgNoModule,
    /// No Summary for this resource
    #[serde(rename = "MSG_NO_SUMMARY")]
    MsgNoSummary,
    /// Operation %s not allowed for resource %s (due to local configuration)
    #[serde(rename = "MSG_OP_NOT_ALLOWED")]
    MsgOpNotAllowed,
    /// Unknown chained parameter name "%s"
    #[serde(rename = "MSG_PARAM_CHAINED")]
    MsgParamChained,
    /// Parameter "%s" content is invalid
    #[serde(rename = "MSG_PARAM_INVALID")]
    MsgParamInvalid,
    /// Parameter "%s" modifier is invalid
    #[serde(rename = "MSG_PARAM_MODIFIER_INVALID")]
    MsgParamModifierInvalid,
    /// Parameter "%s" is not allowed to repeat
    #[serde(rename = "MSG_PARAM_NO_REPEAT")]
    MsgParamNoRepeat,
    /// Parameter "%s" not understood
    #[serde(rename = "MSG_PARAM_UNKNOWN")]
    MsgParamUnknown,
    /// Unable to resolve local reference to resource %s
    #[serde(rename = "MSG_REMOTE_FAIL")]
    MsgRemoteFail,
    /// Resources with identity "example" cannot be deleted (for
    /// testing/training purposes)
    #[serde(rename = "MSG_RESOURCE_EXAMPLE_PROTECTED")]
    MsgResourceExampleProtected,
    /// unable to allocate resource id
    #[serde(rename = "MSG_RESOURCE_ID_FAIL")]
    MsgResourceIdFail,
    /// Resource Id Mismatch
    #[serde(rename = "MSG_RESOURCE_ID_MISMATCH")]
    MsgResourceIdMismatch,
    /// Resource Id Missing
    #[serde(rename = "MSG_RESOURCE_ID_MISSING")]
    MsgResourceIdMissing,
    /// Not allowed to submit a resource for this operation
    #[serde(rename = "MSG_RESOURCE_NOT_ALLOWED")]
    MsgResourceNotAllowed,
    /// A resource is required
    #[serde(rename = "MSG_RESOURCE_REQUIRED")]
    MsgResourceRequired,
    /// Resource Type Mismatch
    #[serde(rename = "MSG_RESOURCE_TYPE_MISMATCH")]
    MsgResourceTypeMismatch,
    /// Unknown sort parameter name "%s"
    #[serde(rename = "MSG_SORT_UNKNOWN")]
    MsgSortUnknown,
    /// Duplicate Identifier in transaction: %s
    #[serde(rename = "MSG_TRANSACTION_DUPLICATE_ID")]
    MsgTransactionDuplicateId,
    /// Missing Identifier in transaction - an entry.id must be provided
    #[serde(rename = "MSG_TRANSACTION_MISSING_ID")]
    MsgTransactionMissingId,
    /// Unhandled xml node type "%s"
    #[serde(rename = "MSG_UNHANDLED_NODE_TYPE")]
    MsgUnhandledNodeType,
    /// Unknown Content (%s) at %s
    #[serde(rename = "MSG_UNKNOWN_CONTENT")]
    MsgUnknownContent,
    /// unknown FHIR http operation
    #[serde(rename = "MSG_UNKNOWN_OPERATION")]
    MsgUnknownOperation,
    /// Resource Type "%s" not recognised
    #[serde(rename = "MSG_UNKNOWN_TYPE")]
    MsgUnknownType,
    /// existing resource updated
    #[serde(rename = "MSG_UPDATED")]
    MsgUpdated,
    /// Version aware updates are required for this resource
    #[serde(rename = "MSG_VERSION_AWARE")]
    MsgVersionAware,
    /// Update Conflict (server current version = "%s", client version
    /// referenced = "%s")
    #[serde(rename = "MSG_VERSION_AWARE_CONFLICT")]
    MsgVersionAwareConflict,
    /// Version specific URL not recognised
    #[serde(rename = "MSG_VERSION_AWARE_URL")]
    MsgVersionAwareUrl,
    /// This does not appear to be a FHIR element or resource (wrong namespace
    /// "%s")
    #[serde(rename = "MSG_WRONG_NS")]
    MsgWrongNs,
    /// Error: Multiple matches exist for %s search parameters "%s"
    #[serde(rename = "SEARCH_MULTIPLE")]
    SearchMultiple,
    /// Error: no processable search found for %s search parameters "%s"
    #[serde(rename = "SEARCH_NONE")]
    SearchNone,
    /// Error: Multiple matches exist for the conditional update
    #[serde(rename = "UPDATE_MULTIPLE_MATCHES")]
    UpdateMultipleMatches,
}

/// Indicates that a parameter applies when the operation is being invoked at
/// the specified level
///
/// System: <http://hl7.org/fhir/operation-parameter-scope>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationParameterScope {
    /// Instance
    #[default]
    #[serde(rename = "instance")]
    Instance,
    /// Type
    #[serde(rename = "type")]
    Type,
    /// System
    #[serde(rename = "system")]
    System,
}

/// Whether an operation parameter is an input or an output parameter.
///
/// System: <http://hl7.org/fhir/operation-parameter-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationParameterUse {
    /// In
    #[default]
    #[serde(rename = "in")]
    In,
    /// Out
    #[serde(rename = "out")]
    Out,
}

/// This example value set defines a set of codes that can be used to indicate
/// the role of one Organization in relation to its affiliation with another.
///
/// System: <http://hl7.org/fhir/organization-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OrganizationRole {
    /// Provider
    #[default]
    #[serde(rename = "provider")]
    Provider,
    /// Agency
    #[serde(rename = "agency")]
    Agency,
    /// Research
    #[serde(rename = "research")]
    Research,
    /// Payer
    #[serde(rename = "payer")]
    Payer,
    /// Diagnostics
    #[serde(rename = "diagnostics")]
    Diagnostics,
    /// Supplier
    #[serde(rename = "supplier")]
    Supplier,
    /// HIE/HIO
    #[serde(rename = "HIE/HIO")]
    HieHio,
    /// Member
    #[serde(rename = "member")]
    Member,
}

/// Type for orientation.
///
/// System: <http://hl7.org/fhir/orientation-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OrientationType {
    /// Sense orientation of referenceSeq
    #[default]
    #[serde(rename = "sense")]
    Sense,
    /// Antisense orientation of referenceSeq
    #[serde(rename = "antisense")]
    Antisense,
}

/// A material used in the construction of packages and their components.
///
/// System: <http://hl7.org/fhir/package-material>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PackageMaterial {
    /// Aluminium
    #[default]
    #[serde(rename = "200000003200")]
    N200000003200,
    /// Cyclic Olefin Copolymer
    #[serde(rename = "200000003201")]
    N200000003201,
    /// Epoxyphenol
    #[serde(rename = "200000003202")]
    N200000003202,
    /// Glass
    #[serde(rename = "200000003203")]
    N200000003203,
    /// Glass type I
    #[serde(rename = "200000003204")]
    N200000003204,
    /// Glass type II
    #[serde(rename = "200000003205")]
    N200000003205,
    /// Glass type III
    #[serde(rename = "200000003206")]
    N200000003206,
    /// Paper
    #[serde(rename = "200000003207")]
    N200000003207,
    /// Plastic
    #[serde(rename = "200000003208")]
    N200000003208,
    /// PolyAmide
    #[serde(rename = "200000003209")]
    N200000003209,
    /// Orientated PolyAmide
    #[serde(rename = "200000003210")]
    N200000003210,
    /// PolyCarbonate
    #[serde(rename = "200000003211")]
    N200000003211,
    /// PolyChloroTriFluoroEthylene
    #[serde(rename = "200000003212")]
    N200000003212,
    /// Polyester
    #[serde(rename = "200000003213")]
    N200000003213,
    /// PolyEthylene
    #[serde(rename = "200000003214")]
    N200000003214,
    /// High Density PolyEthylene
    #[serde(rename = "200000003215")]
    N200000003215,
    /// Low Density PolyEthylene
    #[serde(rename = "200000003216")]
    N200000003216,
    /// PolyEthylene TerePhthalate
    #[serde(rename = "200000003217")]
    N200000003217,
    /// Polyolefin
    #[serde(rename = "200000003218")]
    N200000003218,
    /// PolyPropylene
    #[serde(rename = "200000003219")]
    N200000003219,
    /// PolyStyrene
    #[serde(rename = "200000003220")]
    N200000003220,
    /// PolyVinyl Acetate
    #[serde(rename = "200000003221")]
    N200000003221,
    /// PolyVinyl Chloride
    #[serde(rename = "200000003222")]
    N200000003222,
    /// Plasticised PolyVinyl Chloride
    #[serde(rename = "200000003223")]
    N200000003223,
    /// Non-plasticised PolyVinyl Chloride / Unplasticised PolyVinyl Chloride
    #[serde(rename = "200000003224")]
    N200000003224,
    /// PolyVinylidene Chloride
    #[serde(rename = "200000003225")]
    N200000003225,
    /// Rubber
    #[serde(rename = "200000003226")]
    N200000003226,
    /// Silicone oil
    #[serde(rename = "200000003227")]
    N200000003227,
    /// Silicone elastomer
    #[serde(rename = "200000003228")]
    N200000003228,
    /// Steel
    #[serde(rename = "200000003229")]
    N200000003229,
    /// Cardboard
    #[serde(rename = "200000003529")]
    N200000003529,
    /// PolyAcryloNitrile
    #[serde(rename = "200000012514")]
    N200000012514,
    /// Ethylene acrylic acid copolymer
    #[serde(rename = "200000012515")]
    N200000012515,
    /// Ethylene meta-acrylic acid
    #[serde(rename = "200000012521")]
    N200000012521,
    /// Ethylene-Vinyl Alcohol copolymer
    #[serde(rename = "200000012522")]
    N200000012522,
    /// PolyVinylidene Fluoride
    #[serde(rename = "200000012523")]
    N200000012523,
    /// Medium Density PolyEthylene
    #[serde(rename = "200000012524")]
    N200000012524,
    /// Syndiotactic Polypropylene
    #[serde(rename = "200000012538")]
    N200000012538,
    /// PolyEthylene copolymer
    #[serde(rename = "200000015521")]
    N200000015521,
    /// Expanded Polyethylene
    #[serde(rename = "200000023330")]
    N200000023330,
    /// Cyclic Olefin Polymer
    #[serde(rename = "200000023332")]
    N200000023332,
    /// Silica gel
    #[serde(rename = "200000025255")]
    N200000025255,
    /// Linear Low Density PolyEthylene
    #[serde(rename = "200000025257")]
    N200000025257,
}

/// A high level categorisation of a package.
///
/// System: <http://hl7.org/fhir/package-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PackageType {
    /// Medicinal product pack
    #[default]
    #[serde(rename = "MedicinalProductPack")]
    MedicinalProductPack,
    /// Raw material package
    #[serde(rename = "RawMaterialPackage")]
    RawMaterialPackage,
    /// Shipping or transport container
    #[serde(rename = "Shipping-TransportContainer")]
    ShippingTransportContainer,
}

/// A type of packaging.
///
/// System: <http://hl7.org/fhir/packaging-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PackagingType {
    /// Ampoule
    #[default]
    #[serde(rename = "100000073490")]
    N100000073490,
    /// Applicator
    #[serde(rename = "100000073491")]
    N100000073491,
    /// Automatic injection device
    #[serde(rename = "100000073492")]
    N100000073492,
    /// Bag
    #[serde(rename = "100000073493")]
    N100000073493,
    /// Balling gun
    #[serde(rename = "100000073494")]
    N100000073494,
    /// Barrel
    #[serde(rename = "100000073495")]
    N100000073495,
    /// Blister
    #[serde(rename = "100000073496")]
    N100000073496,
    /// Bottle
    #[serde(rename = "100000073497")]
    N100000073497,
    /// Box
    #[serde(rename = "100000073498")]
    N100000073498,
    /// Brush
    #[serde(rename = "100000073499")]
    N100000073499,
    /// Brush applicator
    #[serde(rename = "100000073500")]
    N100000073500,
    /// Cannula
    #[serde(rename = "100000073501")]
    N100000073501,
    /// Cap
    #[serde(rename = "100000073502")]
    N100000073502,
    /// Cartridge
    #[serde(rename = "100000073503")]
    N100000073503,
    /// Child-resistant closure
    #[serde(rename = "100000073504")]
    N100000073504,
    /// Cup
    #[serde(rename = "100000073505")]
    N100000073505,
    /// Dabbing applicator
    #[serde(rename = "100000073506")]
    N100000073506,
    /// Dart
    #[serde(rename = "100000073507")]
    N100000073507,
    /// Dredging applicator
    #[serde(rename = "100000073508")]
    N100000073508,
    /// Dredging container
    #[serde(rename = "100000073509")]
    N100000073509,
    /// Drench gun
    #[serde(rename = "100000073510")]
    N100000073510,
    /// Dropper applicator
    #[serde(rename = "100000073511")]
    N100000073511,
    /// Dropper container
    #[serde(rename = "100000073512")]
    N100000073512,
    /// Fixed cryogenic vessel
    #[serde(rename = "100000073513")]
    N100000073513,
    /// Gas cylinder
    #[serde(rename = "100000073514")]
    N100000073514,
    /// High pressure transdermal delivery device
    #[serde(rename = "100000073515")]
    N100000073515,
    /// Implanter
    #[serde(rename = "100000073516")]
    N100000073516,
    /// Inhaler
    #[serde(rename = "100000073517")]
    N100000073517,
    /// In-ovo injection device
    #[serde(rename = "100000073518")]
    N100000073518,
    /// Injection needle
    #[serde(rename = "100000073519")]
    N100000073519,
    /// Injection syringe
    #[serde(rename = "100000073520")]
    N100000073520,
    /// Internal graduated calibration chamber
    #[serde(rename = "100000073521")]
    N100000073521,
    /// Intramammary syringe
    #[serde(rename = "100000073522")]
    N100000073522,
    /// Jar
    #[serde(rename = "100000073523")]
    N100000073523,
    /// Measuring device
    #[serde(rename = "100000073524")]
    N100000073524,
    /// Measuring spoon
    #[serde(rename = "100000073525")]
    N100000073525,
    /// Metering pump
    #[serde(rename = "100000073526")]
    N100000073526,
    /// Metering valve
    #[serde(rename = "100000073527")]
    N100000073527,
    /// Mobile cryogenic vessel
    #[serde(rename = "100000073528")]
    N100000073528,
    /// Mouthpiece
    #[serde(rename = "100000073529")]
    N100000073529,
    /// Multidose container
    #[serde(rename = "100000073530")]
    N100000073530,
    /// Multidose container with airless pump
    #[serde(rename = "100000073531")]
    N100000073531,
    /// Multipuncturer
    #[serde(rename = "100000073532")]
    N100000073532,
    /// Nasal applicator
    #[serde(rename = "100000073533")]
    N100000073533,
    /// Nebuliser
    #[serde(rename = "100000073534")]
    N100000073534,
    /// Needle applicator
    #[serde(rename = "100000073535")]
    N100000073535,
    /// Nozzle
    #[serde(rename = "100000073536")]
    N100000073536,
    /// Oral syringe
    #[serde(rename = "100000073537")]
    N100000073537,
    /// Pipette
    #[serde(rename = "100000073538")]
    N100000073538,
    /// Pipette applicator
    #[serde(rename = "100000073539")]
    N100000073539,
    /// Pouch
    #[serde(rename = "100000073540")]
    N100000073540,
    /// Pour-on container
    #[serde(rename = "100000073541")]
    N100000073541,
    /// Pre-filled gastroenteral tube
    #[serde(rename = "100000073542")]
    N100000073542,
    /// Pre-filled pen
    #[serde(rename = "100000073543")]
    N100000073543,
    /// Pre-filled syringe
    #[serde(rename = "100000073544")]
    N100000073544,
    /// Pressurised container
    #[serde(rename = "100000073545")]
    N100000073545,
    /// Prick test applicator
    #[serde(rename = "100000073546")]
    N100000073546,
    /// Sachet
    #[serde(rename = "100000073547")]
    N100000073547,
    /// Scarifier
    #[serde(rename = "100000073548")]
    N100000073548,
    /// Screw cap
    #[serde(rename = "100000073549")]
    N100000073549,
    /// Single-dose container
    #[serde(rename = "100000073550")]
    N100000073550,
    /// Spatula
    #[serde(rename = "100000073551")]
    N100000073551,
    /// Spot-on applicator
    #[serde(rename = "100000073552")]
    N100000073552,
    /// Spray container
    #[serde(rename = "100000073553")]
    N100000073553,
    /// Spray pump
    #[serde(rename = "100000073554")]
    N100000073554,
    /// Spray valve
    #[serde(rename = "100000073555")]
    N100000073555,
    /// Stab vaccinator
    #[serde(rename = "100000073556")]
    N100000073556,
    /// Stopper
    #[serde(rename = "100000073557")]
    N100000073557,
    /// Straw
    #[serde(rename = "100000073558")]
    N100000073558,
    /// Strip
    #[serde(rename = "100000073559")]
    N100000073559,
    /// Tablet container
    #[serde(rename = "100000073560")]
    N100000073560,
    /// Tube
    #[serde(rename = "100000073561")]
    N100000073561,
    /// Vaginal sponge applicator
    #[serde(rename = "100000073562")]
    N100000073562,
    /// Vial
    #[serde(rename = "100000073563")]
    N100000073563,
    /// Administration system
    #[serde(rename = "100000075664")]
    N100000075664,
    /// Calendar package
    #[serde(rename = "100000116195")]
    N100000116195,
    /// Needle-free injector
    #[serde(rename = "100000116196")]
    N100000116196,
    /// Roll-on container
    #[serde(rename = "100000116197")]
    N100000116197,
    /// Multidose container with pump
    #[serde(rename = "100000125779")]
    N100000125779,
    /// Container
    #[serde(rename = "100000137702")]
    N100000137702,
    /// Oral applicator
    #[serde(rename = "100000137703")]
    N100000137703,
    /// Multidose container with metering pump
    #[serde(rename = "100000143554")]
    N100000143554,
    /// Pack
    #[serde(rename = "100000143555")]
    N100000143555,
    /// disk
    #[serde(rename = "100000163233")]
    N100000163233,
    /// plunger
    #[serde(rename = "100000163234")]
    N100000163234,
    /// infusion port
    #[serde(rename = "100000164143")]
    N100000164143,
    /// Valve
    #[serde(rename = "100000166980")]
    N100000166980,
    /// Jerrycan
    #[serde(rename = "100000169899")]
    N100000169899,
    /// Oral applicator
    #[serde(rename = "100000173982")]
    N100000173982,
    /// Dose dispenser
    #[serde(rename = "100000173983")]
    N100000173983,
    /// Unit-dose blister
    #[serde(rename = "100000174066")]
    N100000174066,
    /// Pre-filled injector
    #[serde(rename = "100000174067")]
    N100000174067,
    /// Pre-filled oral syringe
    #[serde(rename = "100000174068")]
    N100000174068,
    /// Pre-filled oral applicator
    #[serde(rename = "100000174069")]
    N100000174069,
    /// Dose-dispenser cartridge
    #[serde(rename = "100000174070")]
    N100000174070,
    /// Pen
    #[serde(rename = "200000005068")]
    N200000005068,
    /// Wrapper
    #[serde(rename = "200000005585")]
    N200000005585,
    /// Lid
    #[serde(rename = "200000010647")]
    N200000010647,
    /// Capsule for opening
    #[serde(rename = "200000011726")]
    N200000011726,
    /// Child-resistant sachet
    #[serde(rename = "200000012539")]
    N200000012539,
    /// Tamper-evident closure
    #[serde(rename = "200000013191")]
    N200000013191,
    /// Tablet tube
    #[serde(rename = "200000024874")]
    N200000024874,
}

/// The Participation status of an appointment.
///
/// System: <http://hl7.org/fhir/participationstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Participationstatus {
    /// Accepted
    #[default]
    #[serde(rename = "accepted")]
    Accepted,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Tentative
    #[serde(rename = "tentative")]
    Tentative,
    /// Needs Action
    #[serde(rename = "needs-action")]
    NeedsAction,
}

/// This value set contains codes for the type of payment issuers.
///
/// System: <http://hl7.org/fhir/payment-issuertype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PaymentIssuertype {
    /// Patient
    #[default]
    #[serde(rename = "patient")]
    Patient,
    /// Insurance
    #[serde(rename = "insurance")]
    Insurance,
}

/// This value set contains codes for the type of workflow from which payments
/// arise.
///
/// System: <http://hl7.org/fhir/payment-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PaymentKind {
    /// Deposit on Account
    #[default]
    #[serde(rename = "deposit")]
    Deposit,
    /// Periodic Payment
    #[serde(rename = "periodic-payment")]
    PeriodicPayment,
    /// Online Bill Payment
    #[serde(rename = "online")]
    Online,
    /// Kiosk Payment
    #[serde(rename = "kiosk")]
    Kiosk,
}

/// This value set includes Claim Processing Outcome codes.
///
/// System: <http://hl7.org/fhir/payment-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PaymentOutcome {
    /// Queued
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Processing Complete
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial Processing
    #[serde(rename = "partial")]
    Partial,
}

/// Codes identifying the rule combining. See XACML Combining algorithms
/// http://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-cos01-en.html
///
/// System: <http://hl7.org/fhir/permission-rule-combining>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PermissionRuleCombining {
    /// Deny-overrides
    #[default]
    #[serde(rename = "deny-overrides")]
    DenyOverrides,
    /// Permit-overrides
    #[serde(rename = "permit-overrides")]
    PermitOverrides,
    /// Ordered-deny-overrides
    #[serde(rename = "ordered-deny-overrides")]
    OrderedDenyOverrides,
    /// Ordered-permit-overrides
    #[serde(rename = "ordered-permit-overrides")]
    OrderedPermitOverrides,
    /// Deny-unless-permit
    #[serde(rename = "deny-unless-permit")]
    DenyUnlessPermit,
    /// Permit-unless-deny
    #[serde(rename = "permit-unless-deny")]
    PermitUnlessDeny,
}

/// Codes identifying the lifecycle stage of a product.
///
/// System: <http://hl7.org/fhir/permission-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PermissionStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
}

/// Permitted data type for observation value.
///
/// System: <http://hl7.org/fhir/permitted-data-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PermittedDataType {
    /// Quantity
    #[default]
    #[serde(rename = "Quantity")]
    Quantity,
    /// CodeableConcept
    #[serde(rename = "CodeableConcept")]
    CodeableConcept,
    /// string
    #[serde(rename = "string")]
    String,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// Range
    #[serde(rename = "Range")]
    Range,
    /// Ratio
    #[serde(rename = "Ratio")]
    Ratio,
    /// SampledData
    #[serde(rename = "SampledData")]
    SampledData,
    /// time
    #[serde(rename = "time")]
    Time,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// Period
    #[serde(rename = "Period")]
    Period,
}

/// Codes indicating the kind of the price component.
///
/// System: <http://hl7.org/fhir/price-component-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PriceComponentType {
    /// base price
    #[default]
    #[serde(rename = "base")]
    Base,
    /// surcharge
    #[serde(rename = "surcharge")]
    Surcharge,
    /// deduction
    #[serde(rename = "deduction")]
    Deduction,
    /// discount
    #[serde(rename = "discount")]
    Discount,
    /// tax
    #[serde(rename = "tax")]
    Tax,
    /// informational
    #[serde(rename = "informational")]
    Informational,
}

/// Biologically Derived Product Category.
///
/// System: <http://hl7.org/fhir/product-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProductCategory {
    /// Organ
    #[default]
    #[serde(rename = "organ")]
    Organ,
    /// Tissue
    #[serde(rename = "tissue")]
    Tissue,
    /// Fluid
    #[serde(rename = "fluid")]
    Fluid,
    /// Cells
    #[serde(rename = "cells")]
    Cells,
    /// BiologicalAgent
    #[serde(rename = "biologicalAgent")]
    BiologicalAgent,
}

/// ProductIntendedUse
///
/// System: <http://hl7.org/fhir/product-intended-use>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProductIntendedUse {
    /// Prevention
    #[default]
    #[serde(rename = "Prevention")]
    Prevention,
    /// Treatment
    #[serde(rename = "Treatment")]
    Treatment,
    /// Alleviation
    #[serde(rename = "Alleviation")]
    Alleviation,
    /// Diagnosis
    #[serde(rename = "Diagnosis")]
    Diagnosis,
    /// Monitoring
    #[serde(rename = "Monitoring")]
    Monitoring,
}

/// Codes identifying the lifecycle stage of a product.
///
/// System: <http://hl7.org/fhir/product-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProductStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// How a property is represented when serialized.
///
/// System: <http://hl7.org/fhir/property-representation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PropertyRepresentation {
    /// XML Attribute
    #[default]
    #[serde(rename = "xmlAttr")]
    XmlAttr,
    /// XML Text
    #[serde(rename = "xmlText")]
    XmlText,
    /// Type Attribute
    #[serde(rename = "typeAttr")]
    TypeAttr,
    /// CDA Text Format
    #[serde(rename = "cdaText")]
    CdaText,
    /// XHTML
    #[serde(rename = "xhtml")]
    Xhtml,
}

/// How an entity was used in an activity.
///
/// System: <http://hl7.org/fhir/provenance-entity-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProvenanceEntityRole {
    /// Revision
    #[default]
    #[serde(rename = "revision")]
    Revision,
    /// Quotation
    #[serde(rename = "quotation")]
    Quotation,
    /// Source
    #[serde(rename = "source")]
    Source,
    /// Instantiates
    #[serde(rename = "instantiates")]
    Instantiates,
    /// Removal
    #[serde(rename = "removal")]
    Removal,
}

/// The lifecycle status of an artifact.
///
/// System: <http://hl7.org/fhir/publication-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PublicationStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Retired
    #[serde(rename = "retired")]
    Retired,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// The type of publication such as book, database, or journal.
///
/// System: <http://hl7.org/fhir/published-in-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PublishedInType {
    /// Periodical
    #[default]
    #[serde(rename = "D020492")]
    D020492,
    /// Database
    #[serde(rename = "D019991")]
    D019991,
    /// Book
    #[serde(rename = "D001877")]
    D001877,
    /// Dataset
    #[serde(rename = "D064886")]
    D064886,
}

/// This value set is suitable for use with the provenance resource. It is
/// derived from, but not compatible with, the HL7 v3 Purpose of use Code
/// system.
///
/// System: <http://healthit.gov/nhin/purposeofuse>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Purposeofuse {
    /// Treatment
    #[default]
    #[serde(rename = "TREATMENT")]
    Treatment,
    /// Payment
    #[serde(rename = "PAYMENT")]
    Payment,
    /// Operations
    #[serde(rename = "OPERATIONS")]
    Operations,
    /// Sysadmin
    #[serde(rename = "SYSADMIN")]
    Sysadmin,
    /// Fraud
    #[serde(rename = "FRAUD")]
    Fraud,
    /// Psychotherapy
    #[serde(rename = "PSYCHOTHERAPY")]
    Psychotherapy,
    /// Training
    #[serde(rename = "TRAINING")]
    Training,
    /// Legal
    #[serde(rename = "LEGAL")]
    Legal,
    /// Marketing
    #[serde(rename = "MARKETING")]
    Marketing,
    /// Directory
    #[serde(rename = "DIRECTORY")]
    Directory,
    /// Family
    #[serde(rename = "FAMILY")]
    Family,
    /// Present
    #[serde(rename = "PRESENT")]
    Present,
    /// Emergency
    #[serde(rename = "EMERGENCY")]
    Emergency,
    /// Disaster
    #[serde(rename = "DISASTER")]
    Disaster,
    /// Public Health
    #[serde(rename = "PUBLICHEALTH")]
    Publichealth,
    /// Abuse
    #[serde(rename = "ABUSE")]
    Abuse,
    /// Oversight
    #[serde(rename = "OVERSIGHT")]
    Oversight,
    /// Judicial
    #[serde(rename = "JUDICIAL")]
    Judicial,
    /// Law Enforcement
    #[serde(rename = "LAW")]
    Law,
    /// Deceased
    #[serde(rename = "DECEASED")]
    Deceased,
    /// Donation
    #[serde(rename = "DONATION")]
    Donation,
    /// Research
    #[serde(rename = "RESEARCH")]
    Research,
    /// Threat
    #[serde(rename = "THREAT")]
    Threat,
    /// Government
    #[serde(rename = "GOVERNMENT")]
    Government,
    /// Worker's Comp
    #[serde(rename = "WORKERSCOMP")]
    Workerscomp,
    /// Coverage
    #[serde(rename = "COVERAGE")]
    Coverage,
    /// Request
    #[serde(rename = "REQUEST")]
    Request,
}

/// How the Quantity should be understood and represented.
///
/// System: <http://hl7.org/fhir/quantity-comparator>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuantityComparator {
    /// Less than
    #[default]
    #[serde(rename = "<")]
    Unnamed,
    /// Less or Equal to
    #[serde(rename = "<=")]
    Unnamed2,
    /// Greater or Equal to
    #[serde(rename = ">=")]
    Unnamed3,
    /// Greater than
    #[serde(rename = ">")]
    Unnamed4,
    /// Sufficient to achieve this total quantity
    #[serde(rename = "ad")]
    Ad,
}

/// Codes that describe the types of constraints possible on a question item
/// that has a list of permitted answers
///
/// System: <http://hl7.org/fhir/questionnaire-answer-constraint>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireAnswerConstraint {
    /// Options only
    #[default]
    #[serde(rename = "optionsOnly")]
    OptionsOnly,
    /// Options or 'type'
    #[serde(rename = "optionsOrType")]
    OptionsOrType,
    /// Options or string
    #[serde(rename = "optionsOrString")]
    OptionsOrString,
}

/// Lifecycle status of the questionnaire response.
///
/// System: <http://hl7.org/fhir/questionnaire-answers-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireAnswersStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
}

/// QuestionnaireItemDisabledDisplay
///
/// System: <http://hl7.org/fhir/questionnaire-disabled-display>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireDisabledDisplay {
    /// Hidden
    #[default]
    #[serde(rename = "hidden")]
    Hidden,
    /// Protected
    #[serde(rename = "protected")]
    Protected,
}

/// Controls how multiple enableWhen values are interpreted - whether all or
/// any must be true.
///
/// System: <http://hl7.org/fhir/questionnaire-enable-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireEnableBehavior {
    /// All
    #[default]
    #[serde(rename = "all")]
    All,
    /// Any
    #[serde(rename = "any")]
    Any,
}

/// The criteria by which a question is enabled.
///
/// System: <http://hl7.org/fhir/questionnaire-enable-operator>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireEnableOperator {
    /// Exists
    #[default]
    #[serde(rename = "exists")]
    Exists,
    /// Equals
    #[serde(rename = "=")]
    Unnamed,
    /// Not Equals
    #[serde(rename = "!=")]
    Unnamed2,
    /// Greater Than
    #[serde(rename = ">")]
    Unnamed3,
    /// Less Than
    #[serde(rename = "<")]
    Unnamed4,
    /// Greater or Equals
    #[serde(rename = ">=")]
    Unnamed5,
    /// Less or Equals
    #[serde(rename = "<=")]
    Unnamed6,
}

/// Clinical assessment of the severity of a reaction event as a whole,
/// potentially considering multiple different manifestations.
///
/// System: <http://hl7.org/fhir/reaction-event-severity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReactionEventSeverity {
    /// Mild
    #[default]
    #[serde(rename = "mild")]
    Mild,
    /// Moderate
    #[serde(rename = "moderate")]
    Moderate,
    /// Severe
    #[serde(rename = "severe")]
    Severe,
}

/// This value set is provided as an example. The value set to instantiate this
/// attribute should be drawn from a robust terminology code system that
/// consists of or contains concepts to support the medication process.
///
/// System: <http://hl7.org/fhir/reason-medication-not-given>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReasonMedicationNotGiven {
    /// None
    #[default]
    #[serde(rename = "a")]
    A,
    /// Away
    #[serde(rename = "b")]
    B,
    /// Asleep
    #[serde(rename = "c")]
    C,
    /// Vomit
    #[serde(rename = "d")]
    D,
}

/// A set of flags that defines how references are supported.
///
/// System: <http://hl7.org/fhir/reference-handling-policy>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReferenceHandlingPolicy {
    /// Literal References
    #[default]
    #[serde(rename = "literal")]
    Literal,
    /// Logical References
    #[serde(rename = "logical")]
    Logical,
    /// Resolves References
    #[serde(rename = "resolves")]
    Resolves,
    /// Reference Integrity Enforced
    #[serde(rename = "enforced")]
    Enforced,
    /// Local References Only
    #[serde(rename = "local")]
    Local,
}

/// Whether a reference needs to be version specific or version independent, or
/// whether either can be used.
///
/// System: <http://hl7.org/fhir/reference-version-rules>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReferenceVersionRules {
    /// Either Specific or independent
    #[default]
    #[serde(rename = "either")]
    Either,
    /// Version independent
    #[serde(rename = "independent")]
    Independent,
    /// Version Specific
    #[serde(rename = "specific")]
    Specific,
}

/// RegulatedAuthorizationBasis
///
/// System: <http://hl7.org/fhir/regulated-authorization-basis>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RegulatedAuthorizationBasis {
    /// Full application
    #[default]
    #[serde(rename = "Full")]
    Full,
    /// New active substance
    #[serde(rename = "NewSubstance")]
    NewSubstance,
    /// Known active substance
    #[serde(rename = "KnownSubstance")]
    KnownSubstance,
    /// Similar biological application
    #[serde(rename = "SimilarBiological")]
    SimilarBiological,
    /// Well-established use application
    #[serde(rename = "Well-establishedUse")]
    WellEstablishedUse,
    /// Traditional use registration for herbal medicinal product application
    #[serde(rename = "TraditionalUse")]
    TraditionalUse,
    /// Bibliographical application (stand-alone)
    #[serde(rename = "Bibliographical")]
    Bibliographical,
    /// Known human blood/plasma derived ancillary medicinal substance
    #[serde(rename = "KnownHumanBlood")]
    KnownHumanBlood,
    /// Authorizations for temporary use
    #[serde(rename = "TemporaryUse")]
    TemporaryUse,
    /// Parallel traded products
    #[serde(rename = "ParallelTrade")]
    ParallelTrade,
}

/// RegulatedAuthorizationCaseType
///
/// System: <http://hl7.org/fhir/regulated-authorization-case-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RegulatedAuthorizationCaseType {
    /// Initial Marketing Authorization Application
    #[default]
    #[serde(rename = "InitialMAA")]
    InitialMaa,
    /// Variation
    #[serde(rename = "Variation")]
    Variation,
    /// Line Extension
    #[serde(rename = "LineExtension")]
    LineExtension,
    /// Periodic Safety Update Report
    #[serde(rename = "PSUR")]
    Psur,
    /// Renewal
    #[serde(rename = "Renewal")]
    Renewal,
    /// Follow-up Measure
    #[serde(rename = "Follow-up")]
    FollowUp,
    /// Specific Obligation
    #[serde(rename = "100000155699")]
    N100000155699,
    /// Annual Reassessment
    #[serde(rename = "AnnualReassessment")]
    AnnualReassessment,
    /// Urgent Safety Restriction
    #[serde(rename = "UrgentSafetyRestriction")]
    UrgentSafetyRestriction,
    /// Paediatric Submission
    #[serde(rename = "PaediatricSubmission")]
    PaediatricSubmission,
    /// Transfer of a marketing authorization
    #[serde(rename = "TransferMA")]
    TransferMa,
    /// Lifting of a Suspension
    #[serde(rename = "LiftingSuspension")]
    LiftingSuspension,
    /// Withdrawal
    #[serde(rename = "Withdrawal")]
    Withdrawal,
    /// Reformatting
    #[serde(rename = "Reformatting")]
    Reformatting,
    /// Risk Management Plan
    #[serde(rename = "RMP")]
    Rmp,
    /// Review of a Suspension of MA
    #[serde(rename = "ReviewSuspension")]
    ReviewSuspension,
    /// Supplemental Information
    #[serde(rename = "SupplementalInformation")]
    SupplementalInformation,
    /// Repeat Use Procedure
    #[serde(rename = "RepeatUse")]
    RepeatUse,
    /// Signal detection
    #[serde(rename = "SignalDetection")]
    SignalDetection,
    /// FLU STRAIN UPDATE
    #[serde(rename = "FLU")]
    Flu,
    /// PANDEMIC UPDATE
    #[serde(rename = "PANDEMIC")]
    Pandemic,
    /// Orphan Designation Application
    #[serde(rename = "Orphan")]
    Orphan,
}

/// RegulatedAuthorizationType
///
/// System: <http://hl7.org/fhir/regulated-authorization-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RegulatedAuthorizationType {
    /// Marketing Authorization
    #[default]
    #[serde(rename = "MarketingAuth")]
    MarketingAuth,
    /// Orphan Drug Authorization
    #[serde(rename = "Orphan")]
    Orphan,
    /// Pediatric Use Drug Authorization
    #[serde(rename = "Pediatric")]
    Pediatric,
}

/// The type of relationship to the related artifact.
///
/// System: <http://hl7.org/fhir/related-artifact-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RelatedArtifactType {
    /// Documentation
    #[default]
    #[serde(rename = "documentation")]
    Documentation,
    /// Justification
    #[serde(rename = "justification")]
    Justification,
    /// Citation
    #[serde(rename = "citation")]
    Citation,
    /// Predecessor
    #[serde(rename = "predecessor")]
    Predecessor,
    /// Successor
    #[serde(rename = "successor")]
    Successor,
    /// Derived From
    #[serde(rename = "derived-from")]
    DerivedFrom,
    /// Depends On
    #[serde(rename = "depends-on")]
    DependsOn,
    /// Composed Of
    #[serde(rename = "composed-of")]
    ComposedOf,
    /// Part Of
    #[serde(rename = "part-of")]
    PartOf,
    /// Amends
    #[serde(rename = "amends")]
    Amends,
    /// Amended With
    #[serde(rename = "amended-with")]
    AmendedWith,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
    /// Appended With
    #[serde(rename = "appended-with")]
    AppendedWith,
    /// Cites
    #[serde(rename = "cites")]
    Cites,
    /// Cited By
    #[serde(rename = "cited-by")]
    CitedBy,
    /// Is Comment On
    #[serde(rename = "comments-on")]
    CommentsOn,
    /// Has Comment In
    #[serde(rename = "comment-in")]
    CommentIn,
    /// Contains
    #[serde(rename = "contains")]
    Contains,
    /// Contained In
    #[serde(rename = "contained-in")]
    ContainedIn,
    /// Corrects
    #[serde(rename = "corrects")]
    Corrects,
    /// Correction In
    #[serde(rename = "correction-in")]
    CorrectionIn,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Replaced With
    #[serde(rename = "replaced-with")]
    ReplacedWith,
    /// Retracts
    #[serde(rename = "retracts")]
    Retracts,
    /// Retracted By
    #[serde(rename = "retracted-by")]
    RetractedBy,
    /// Signs
    #[serde(rename = "signs")]
    Signs,
    /// Similar To
    #[serde(rename = "similar-to")]
    SimilarTo,
    /// Supports
    #[serde(rename = "supports")]
    Supports,
    /// Supported With
    #[serde(rename = "supported-with")]
    SupportedWith,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Transformed Into
    #[serde(rename = "transformed-into")]
    TransformedInto,
    /// Transformed With
    #[serde(rename = "transformed-with")]
    TransformedWith,
    /// Documents
    #[serde(rename = "documents")]
    Documents,
    /// Specification Of
    #[serde(rename = "specification-of")]
    SpecificationOf,
    /// Created With
    #[serde(rename = "created-with")]
    CreatedWith,
    /// Cite As
    #[serde(rename = "cite-as")]
    CiteAs,
}

/// The type of relationship to the cited artifact.
///
/// System: <http://hl7.org/fhir/related-artifact-type-expanded>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RelatedArtifactTypeExpanded {
    /// Reprint
    #[default]
    #[serde(rename = "reprint")]
    Reprint,
    /// Reprint Of
    #[serde(rename = "reprint-of")]
    ReprintOf,
}

/// This value set includes the Patient to subscriber relationship codes.
///
/// System: <http://hl7.org/fhir/relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Relationship {
    /// Self
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Spouse
    #[serde(rename = "2")]
    N2,
    /// Child
    #[serde(rename = "3")]
    N3,
    /// Common Law Spouse
    #[serde(rename = "4")]
    N4,
    /// Other
    #[serde(rename = "5")]
    N5,
}

/// The outcome of the processing.
///
/// System: <http://hl7.org/fhir/remittance-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RemittanceOutcome {
    /// Complete
    #[default]
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial
    #[serde(rename = "partial")]
    Partial,
}

/// The results of executing an action.
///
/// System: <http://hl7.org/fhir/report-action-result-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReportActionResultCodes {
    /// Pass
    #[default]
    #[serde(rename = "pass")]
    Pass,
    /// Skip
    #[serde(rename = "skip")]
    Skip,
    /// Fail
    #[serde(rename = "fail")]
    Fail,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Error
    #[serde(rename = "error")]
    Error,
}

/// The type of participant.
///
/// System: <http://hl7.org/fhir/report-participant-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReportParticipantType {
    /// Test Engine
    #[default]
    #[serde(rename = "test-engine")]
    TestEngine,
    /// Client
    #[serde(rename = "client")]
    Client,
    /// Server
    #[serde(rename = "server")]
    Server,
}

/// The type of relationship between reports.
///
/// System: <http://hl7.org/fhir/report-relation-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReportRelationType {
    /// Replaces
    #[default]
    #[serde(rename = "replaces")]
    Replaces,
    /// Amends
    #[serde(rename = "amends")]
    Amends,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Replaced With
    #[serde(rename = "replacedWith")]
    ReplacedWith,
    /// Amended With
    #[serde(rename = "amendedWith")]
    AmendedWith,
    /// Appended With
    #[serde(rename = "appendedWith")]
    AppendedWith,
    /// Transformed With
    #[serde(rename = "transformedWith")]
    TransformedWith,
}

/// The reported execution result.
///
/// System: <http://hl7.org/fhir/report-result-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReportResultCodes {
    /// Pass
    #[default]
    #[serde(rename = "pass")]
    Pass,
    /// Fail
    #[serde(rename = "fail")]
    Fail,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
}

/// The current status of the test report.
///
/// System: <http://hl7.org/fhir/report-status-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReportStatusCodes {
    /// Completed
    #[default]
    #[serde(rename = "completed")]
    Completed,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Waiting
    #[serde(rename = "waiting")]
    Waiting,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Codes indicating the degree of authority/intentionality associated with a
/// request.
///
/// System: <http://hl7.org/fhir/request-intent>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RequestIntent {
    /// Proposal
    #[default]
    #[serde(rename = "proposal")]
    Proposal,
    /// Plan
    #[serde(rename = "plan")]
    Plan,
    /// Directive
    #[serde(rename = "directive")]
    Directive,
    /// Order
    #[serde(rename = "order")]
    Order,
    /// Original Order
    #[serde(rename = "original-order")]
    OriginalOrder,
    /// Reflex Order
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    /// Filler Order
    #[serde(rename = "filler-order")]
    FillerOrder,
    /// Instance Order
    #[serde(rename = "instance-order")]
    InstanceOrder,
    /// Option
    #[serde(rename = "option")]
    Option,
}

/// Identifies the level of importance to be assigned to actioning the request.
///
/// System: <http://hl7.org/fhir/request-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RequestPriority {
    /// Routine
    #[default]
    #[serde(rename = "routine")]
    Routine,
    /// Urgent
    #[serde(rename = "urgent")]
    Urgent,
    /// ASAP
    #[serde(rename = "asap")]
    Asap,
    /// STAT
    #[serde(rename = "stat")]
    Stat,
}

/// Codes identifying the lifecycle stage of a request.
///
/// System: <http://hl7.org/fhir/request-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RequestStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Codes for the main intent of the study.
///
/// System: <http://hl7.org/fhir/research-study-arm-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyArmType {
    /// Active Comparator
    #[default]
    #[serde(rename = "active-comparator")]
    ActiveComparator,
    /// Placebo Comparator
    #[serde(rename = "placebo-comparator")]
    PlaceboComparator,
    /// Sham Comparator
    #[serde(rename = "sham-comparator")]
    ShamComparator,
    /// No Intervention
    #[serde(rename = "no-intervention")]
    NoIntervention,
    /// Experimental
    #[serde(rename = "experimental")]
    Experimental,
    /// Other Arm Type
    #[serde(rename = "other-arm-type")]
    OtherArmType,
}

/// Codes for use in ResearchStudy Resource. This resource (this entire set of
/// content) is being used for active development of a ResearchStudyClassifiers
/// CodeSystem for use for supporting multiple value sets in the FHIR
/// ResearchStudy StructureDefinition.
///
/// System: <http://hl7.org/fhir/research-study-classifiers>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyClassifiers {
    /// FDA regulated drug
    #[default]
    #[serde(rename = "fda-regulated-drug")]
    FdaRegulatedDrug,
    /// FDA regulated device
    #[serde(rename = "fda-regulated-device")]
    FdaRegulatedDevice,
    /// MPG Paragraph 23b
    #[serde(rename = "mpg-paragraph-23b")]
    MpgParagraph23B,
    /// IRB-exempt
    #[serde(rename = "irb-exempt")]
    IrbExempt,
}

/// Codes for the main intent of the study.
///
/// System: <http://hl7.org/fhir/research-study-focus-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyFocusType {
    /// Medication
    #[default]
    #[serde(rename = "medication")]
    Medication,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Intervention
    #[serde(rename = "intervention")]
    Intervention,
    /// Factor
    #[serde(rename = "factor")]
    Factor,
}

/// Codes for the kind of study objective.
///
/// System: <http://hl7.org/fhir/research-study-objective-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyObjectiveType {
    /// Primary
    #[default]
    #[serde(rename = "primary")]
    Primary,
    /// Secondary
    #[serde(rename = "secondary")]
    Secondary,
    /// Exploratory
    #[serde(rename = "exploratory")]
    Exploratory,
}

/// This is a ResearchStudy's party organization type.
///
/// System: <http://hl7.org/fhir/research-study-party-organization-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyPartyOrganizationType {
    /// NIH
    #[default]
    #[serde(rename = "nih")]
    Nih,
    /// FDA
    #[serde(rename = "fda")]
    Fda,
    /// Government
    #[serde(rename = "government")]
    Government,
    /// Nonprofit
    #[serde(rename = "nonprofit")]
    Nonprofit,
    /// Academic
    #[serde(rename = "academic")]
    Academic,
    /// Industry
    #[serde(rename = "industry")]
    Industry,
}

/// This is a ResearchStudy's party role.
///
/// System: <http://hl7.org/fhir/research-study-party-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyPartyRole {
    /// sponsor
    #[default]
    #[serde(rename = "sponsor")]
    Sponsor,
    /// lead-sponsor
    #[serde(rename = "lead-sponsor")]
    LeadSponsor,
    /// sponsor-investigator
    #[serde(rename = "sponsor-investigator")]
    SponsorInvestigator,
    /// primary-investigator
    #[serde(rename = "primary-investigator")]
    PrimaryInvestigator,
    /// collaborator
    #[serde(rename = "collaborator")]
    Collaborator,
    /// funding-source
    #[serde(rename = "funding-source")]
    FundingSource,
    /// general-contact
    #[serde(rename = "general-contact")]
    GeneralContact,
    /// recruitment-contact
    #[serde(rename = "recruitment-contact")]
    RecruitmentContact,
    /// sub-investigator
    #[serde(rename = "sub-investigator")]
    SubInvestigator,
    /// study-director
    #[serde(rename = "study-director")]
    StudyDirector,
    /// study-chair
    #[serde(rename = "study-chair")]
    StudyChair,
    /// Institutional Review Board
    #[serde(rename = "irb")]
    Irb,
}

/// Codes for the stage in the progression of a therapy from initial
/// experimental use in humans in clinical trials to post-market evaluation.
///
/// System: <http://hl7.org/fhir/research-study-phase>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyPhase {
    /// N/A
    #[default]
    #[serde(rename = "n-a")]
    NA,
    /// Early Phase 1
    #[serde(rename = "early-phase-1")]
    EarlyPhase1,
    /// Phase 1
    #[serde(rename = "phase-1")]
    Phase1,
    /// Phase 1/Phase 2
    #[serde(rename = "phase-1-phase-2")]
    Phase1Phase2,
    /// Phase 2
    #[serde(rename = "phase-2")]
    Phase2,
    /// Phase 2/Phase 3
    #[serde(rename = "phase-2-phase-3")]
    Phase2Phase3,
    /// Phase 3
    #[serde(rename = "phase-3")]
    Phase3,
    /// Phase 4
    #[serde(rename = "phase-4")]
    Phase4,
}

/// Codes for the main intent of a research study.
///
/// System: <http://hl7.org/fhir/research-study-prim-purp-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyPrimPurpType {
    /// Treatment
    #[default]
    #[serde(rename = "treatment")]
    Treatment,
    /// Prevention
    #[serde(rename = "prevention")]
    Prevention,
    /// Diagnostic
    #[serde(rename = "diagnostic")]
    Diagnostic,
    /// Supportive Care
    #[serde(rename = "supportive-care")]
    SupportiveCare,
    /// Screening
    #[serde(rename = "screening")]
    Screening,
    /// Health Services Research
    #[serde(rename = "health-services-research")]
    HealthServicesResearch,
    /// Basic Science
    #[serde(rename = "basic-science")]
    BasicScience,
    /// Device Feasibility
    #[serde(rename = "device-feasibility")]
    DeviceFeasibility,
}

/// Codes for why the study ended prematurely.
///
/// System: <http://hl7.org/fhir/research-study-reason-stopped>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyReasonStopped {
    /// Accrual Goal Met
    #[default]
    #[serde(rename = "accrual-goal-met")]
    AccrualGoalMet,
    /// Closed due to toxicity
    #[serde(rename = "closed-due-to-toxicity")]
    ClosedDueToToxicity,
    /// Closed due to lack of study progress
    #[serde(rename = "closed-due-to-lack-of-study-progress")]
    ClosedDueToLackOfStudyProgress,
    /// Temporarily closed per study design
    #[serde(rename = "temporarily-closed-per-study-design")]
    TemporarilyClosedPerStudyDesign,
}

/// Codes that convey the current status of the research study.
///
/// System: <http://hl7.org/fhir/research-study-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResearchStudyStatus {
    /// Overall study
    #[default]
    #[serde(rename = "overall-study")]
    OverallStudy,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Active, not recruiting
    #[serde(rename = "active-but-not-recruiting")]
    ActiveButNotRecruiting,
    /// Administratively Completed
    #[serde(rename = "administratively-completed")]
    AdministrativelyCompleted,
    /// Approved
    #[serde(rename = "approved")]
    Approved,
    /// Closed to Accrual
    #[serde(rename = "closed-to-accrual")]
    ClosedToAccrual,
    /// Closed to Accrual and Intervention
    #[serde(rename = "closed-to-accrual-and-intervention")]
    ClosedToAccrualAndIntervention,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Disapproved
    #[serde(rename = "disapproved")]
    Disapproved,
    /// Enrolling by invitation
    #[serde(rename = "enrolling-by-invitation")]
    EnrollingByInvitation,
    /// In Review
    #[serde(rename = "in-review")]
    InReview,
    /// Not yet recruiting
    #[serde(rename = "not-yet-recruiting")]
    NotYetRecruiting,
    /// Recruiting
    #[serde(rename = "recruiting")]
    Recruiting,
    /// Temporarily Closed to Accrual
    #[serde(rename = "temporarily-closed-to-accrual")]
    TemporarilyClosedToAccrual,
    /// Temporarily Closed to Accrual and Intervention
    #[serde(rename = "temporarily-closed-to-accrual-and-intervention")]
    TemporarilyClosedToAccrualAndIntervention,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
    /// Withdrawn
    #[serde(rename = "withdrawn")]
    Withdrawn,
}

/// How resource references can be aggregated.
///
/// System: <http://hl7.org/fhir/resource-aggregation-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceAggregationMode {
    /// Contained
    #[default]
    #[serde(rename = "contained")]
    Contained,
    /// Referenced
    #[serde(rename = "referenced")]
    Referenced,
    /// Bundled
    #[serde(rename = "bundled")]
    Bundled,
}

/// How slices are interpreted when evaluating an instance.
///
/// System: <http://hl7.org/fhir/resource-slicing-rules>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceSlicingRules {
    /// Closed
    #[default]
    #[serde(rename = "closed")]
    Closed,
    /// Open
    #[serde(rename = "open")]
    Open,
    /// Open at End
    #[serde(rename = "openAtEnd")]
    OpenAtEnd,
}

/// The master set of status codes used throughout FHIR. All status codes are
/// mapped to one of these codes.
///
/// System: <http://hl7.org/fhir/resource-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceStatus {
    /// error
    #[default]
    #[serde(rename = "error")]
    Error,
    /// proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// planned
    #[serde(rename = "planned")]
    Planned,
    /// draft
    #[serde(rename = "draft")]
    Draft,
    /// requested
    #[serde(rename = "requested")]
    Requested,
    /// received
    #[serde(rename = "received")]
    Received,
    /// declined
    #[serde(rename = "declined")]
    Declined,
    /// accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// arrived
    #[serde(rename = "arrived")]
    Arrived,
    /// active
    #[serde(rename = "active")]
    Active,
    /// suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// failed
    #[serde(rename = "failed")]
    Failed,
    /// replaced
    #[serde(rename = "replaced")]
    Replaced,
    /// complete
    #[serde(rename = "complete")]
    Complete,
    /// inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// abandoned
    #[serde(rename = "abandoned")]
    Abandoned,
    /// unknown
    #[serde(rename = "unknown")]
    Unknown,
    /// unconfirmed
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// confirmed
    #[serde(rename = "confirmed")]
    Confirmed,
    /// resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// refuted
    #[serde(rename = "refuted")]
    Refuted,
    /// differential
    #[serde(rename = "differential")]
    Differential,
    /// partial
    #[serde(rename = "partial")]
    Partial,
    /// busy-unavailable
    #[serde(rename = "busy-unavailable")]
    BusyUnavailable,
    /// free
    #[serde(rename = "free")]
    Free,
    /// on-target
    #[serde(rename = "on-target")]
    OnTarget,
    /// ahead-of-target
    #[serde(rename = "ahead-of-target")]
    AheadOfTarget,
    /// behind-target
    #[serde(rename = "behind-target")]
    BehindTarget,
    /// not-ready
    #[serde(rename = "not-ready")]
    NotReady,
    /// transduc-discon
    #[serde(rename = "transduc-discon")]
    TransducDiscon,
    /// hw-discon
    #[serde(rename = "hw-discon")]
    HwDiscon,
}

/// ResourceValidationMode
///
/// System: <http://hl7.org/fhir/resource-validation-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceValidationMode {
    /// Validate for Create
    #[default]
    #[serde(rename = "create")]
    Create,
    /// Validate for Update
    #[serde(rename = "update")]
    Update,
    /// Validate for Delete
    #[serde(rename = "delete")]
    Delete,
    /// Validate Against a Profile
    #[serde(rename = "profile")]
    Profile,
}

/// The kind of response to a message.
///
/// System: <http://hl7.org/fhir/response-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResponseCode {
    /// OK
    #[default]
    #[serde(rename = "ok")]
    Ok,
    /// Transient Error
    #[serde(rename = "transient-error")]
    TransientError,
    /// Fatal Error
    #[serde(rename = "fatal-error")]
    FatalError,
}

/// The mode of a RESTful capability statement.
///
/// System: <http://hl7.org/fhir/restful-capability-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RestfulCapabilityMode {
    /// Client
    #[default]
    #[serde(rename = "client")]
    Client,
    /// Server
    #[serde(rename = "server")]
    Server,
}

/// The set of interactions defined by the RESTful part of the FHIR
/// specification.
///
/// System: <http://hl7.org/fhir/restful-interaction>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RestfulInteraction {
    /// read
    #[default]
    #[serde(rename = "read")]
    Read,
    /// vread
    #[serde(rename = "vread")]
    Vread,
    /// update
    #[serde(rename = "update")]
    Update,
    /// patch
    #[serde(rename = "patch")]
    Patch,
    /// delete
    #[serde(rename = "delete")]
    Delete,
    /// history
    #[serde(rename = "history")]
    History,
    /// history-instance
    #[serde(rename = "history-instance")]
    HistoryInstance,
    /// history-type
    #[serde(rename = "history-type")]
    HistoryType,
    /// history-system
    #[serde(rename = "history-system")]
    HistorySystem,
    /// create
    #[serde(rename = "create")]
    Create,
    /// search
    #[serde(rename = "search")]
    Search,
    /// search-type
    #[serde(rename = "search-type")]
    SearchType,
    /// search-system
    #[serde(rename = "search-system")]
    SearchSystem,
    /// search-compartment
    #[serde(rename = "search-compartment")]
    SearchCompartment,
    /// capabilities
    #[serde(rename = "capabilities")]
    Capabilities,
    /// transaction
    #[serde(rename = "transaction")]
    Transaction,
    /// batch
    #[serde(rename = "batch")]
    Batch,
    /// operation
    #[serde(rename = "operation")]
    Operation,
}

/// Types of security services used with FHIR.
///
/// System: <http://hl7.org/fhir/restful-security-service>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RestfulSecurityService {
    /// OAuth
    #[default]
    #[serde(rename = "OAuth")]
    OAuth,
    /// SMART-on-FHIR
    #[serde(rename = "SMART-on-FHIR")]
    SmartOnFhir,
    /// NTLM
    #[serde(rename = "NTLM")]
    Ntlm,
    /// Basic
    #[serde(rename = "Basic")]
    Basic,
    /// Kerberos
    #[serde(rename = "Kerberos")]
    Kerberos,
    /// Certificates
    #[serde(rename = "Certificates")]
    Certificates,
}

/// The [checklist items](http://hl7.org/fhir/safety.html) defined as part of
/// the FHIR specification.
///
/// System: <http://hl7.org/fhir/safety-entries>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SafetyEntries {
    /// For each resource that my system handles, my system handles the full
    /// [Life cycle](lifecycle.html) (status codes, currency issues, and
    /// erroneous entry status)
    #[default]
    #[serde(rename = "life-cycle")]
    LifeCycle,
    /// For each resource that my system handles, I've reviewed the [Modifier
    /// elements](conformance-rules.html#isModifier)
    #[serde(rename = "modifiers")]
    Modifiers,
    /// My system checks for
    /// [modifierExtension](extensibility.html#modifierExtension) elements
    #[serde(rename = "modifier-extensions")]
    ModifierExtensions,
    /// My system supports [elements labeled as
    /// 'MustSupport'](conformance-rules.html#mustSupport) in the
    /// [profiles](profiling.html) that apply to my system
    #[serde(rename = "must-support")]
    MustSupport,
    /// My system has documented how [distributed resource
    /// identification](managing.html#distributed) works in its relevant
    /// contexts of use, and where (and why)
    /// [contained](references.html#contained) resources are used
    #[serde(rename = "identity")]
    Identity,
    /// My system manages lists of [current resources](lifecycle.html#current)
    /// correctly
    #[serde(rename = "current")]
    Current,
    /// When other systems [return http errors from the RESTful
    /// API](http.html#summary) and [Operations](operations.html) (perhaps
    /// using [Operation Outcome](operationoutcome.html)), my system checks for
    /// them and handles them appropriately
    #[serde(rename = "error-checks")]
    ErrorChecks,
    /// My system ensures checks for patient links (and/or merges) and handles
    /// data that is linked to patients accordingly
    #[serde(rename = "link-merge")]
    LinkMerge,
    /// My system publishes a [Capability Statement](capabilitystatement.html)
    /// with [StructureDefinitions](structuredefinition.html),
    /// [ValueSets](valueset.html), and
    /// [OperationDefinitions](operationdefinition.html), etc., so other
    /// implementers know how the system functions
    #[serde(rename = "cs-declare")]
    CsDeclare,
    /// All resources in use are [valid](validation.html) against the base
    /// specification and the [profiles](profiling.html) that apply to my
    /// system (see note about the [correct run-time use of
    /// validation](validation.html#correct-use))
    #[serde(rename = "valid-checked")]
    ValidChecked,
    /// I've reviewed the [Observation](observation.html) resource, and
    /// understand how ```focus``` is a mechanism for observations to be about
    /// someone or something other than the patient or subject of record.
    #[serde(rename = "obs-focus")]
    ObsFocus,
    /// My system checks for timezones and adjusts times appropriately. (note:
    /// timezones are extremely difficult to get correct - see [W3C Timezone
    /// Advice](https://www.w3.org/TR/timezone/), and note that some fields
    /// should be timezone corrected, and others should not be)
    #[serde(rename = "time-zone")]
    TimeZone,
    /// My system renders dates safely for changes in culture and language (the
    /// date formats D-M-Y and M-D-Y are not differentiated for many dates, and
    /// this is a well-known source of confusion. Systems should use the month
    /// name, or otherwise be specific for each date when rendering, unless
    /// there is solid confidence that such confusion cannot arise, even in the
    /// future when information/narrative from resources will be shared much
    /// more widely)
    #[serde(rename = "date-rendering")]
    DateRendering,
    /// My system takes care to ensure that clients can (for servers) or will
    /// (for clients) find the information they need when content that might
    /// reasonably be exposed using more than one FHIR resource. Possible
    /// patterns: Support a single search across the applicable resources, or
    /// expose data through each applicable resource. See discussion on [Wiki
    /// Page](https://confluence.hl7.org/display/FHIR/Managing+Overlap+Between+Resources)
    /// for further information
    #[serde(rename = "cross-resource")]
    CrossResource,
    /// My system will display warnings returned by the server to the user
    #[serde(rename = "display-warnings")]
    DisplayWarnings,
    /// My system checks whether the server processed all the requested search
    /// parameter, and is safe if servers ignore parameters (typically, either
    /// filters locally or warns the user)
    #[serde(rename = "search-parameters")]
    SearchParameters,
    /// My system caters for [parameters that have missing
    /// values](search.html#missing) when doing search operations, and responds
    /// correctly to the client with regard to [erroneous search
    /// parameters](search.html#errors)
    #[serde(rename = "missing-values")]
    MissingValues,
    /// My system includes appropriate default filters when searching based on
    /// patient context - e.g. filtering out entered-in-error records,
    /// filtering to only include active, living patients if appropriate, and
    /// clearly documents these (preferably including them in the self link for
    /// a search
    #[serde(rename = "default-filters")]
    DefaultFilters,
    /// For each resource, I have checked whether resources can be deleted,
    /// and/or how records are marked as incorrect/no longer relevant
    #[serde(rename = "deletion-check")]
    DeletionCheck,
    /// Deletion of records (or equivalent updates in status) flow through the
    /// system so any replicated copies are deleted/updated
    #[serde(rename = "deletion-replication")]
    DeletionReplication,
    /// (If a server) my documentation about deleted resources is clear, and my
    /// test sandbox (if exists) has deleted/error record cases in the test
    /// data
    #[serde(rename = "deletion-support")]
    DeletionSupport,
    /// My system checks that the right [Patient consent](consent.html) has
    /// been granted (where applicable)
    #[serde(rename = "check-consent")]
    CheckConsent,
    /// My system sends an [Accounting of Disclosure](secpriv-module.html#AoD)
    /// to the consenter as requested when permitted actions on resources are
    /// performed using an [AuditEvent](auditevent.html) Resource
    #[serde(rename = "distribute-aod")]
    DistributeAod,
    /// My system ensures that system clocks are synchronized using a protocol
    /// like NTP or SNTP, or my server is robust against clients that have the
    /// wrong clock set
    #[serde(rename = "check-clocks")]
    CheckClocks,
    /// My system uses security methods for an API to authenticate where Domain
    /// Name System (DNS) responses are coming from and ensure that they are
    /// valid
    #[serde(rename = "check-dns-responses")]
    CheckDnsResponses,
    /// Production exchange of patient or other sensitive data will always use
    /// some form of [encryption on the wire](security.html#http)
    #[serde(rename = "use-encryption")]
    UseEncryption,
    /// Where resources are exchanged using [HTTP](security.html#http),
    /// [TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security) should be
    /// utilized to protect the communications channel
    #[serde(rename = "use-tls")]
    UseTls,
    /// Where resources are exchanged using email,
    /// [S/MIME](https://en.wikipedia.org/wiki/S/MIME) should be used to
    /// protect the end-to-end communication
    #[serde(rename = "use-smime")]
    UseSmime,
    /// Production exchange should utilize recommendations for
    /// [Best-Current-Practice on TLS in BCP
    /// 195](https://tools.ietf.org/html/bcp195)
    #[serde(rename = "use-tls-per-bcp195")]
    UseTlsPerBcp195,
    /// My system utilizes a risk and use case [appropriate OAuth
    /// profile](security.html#oauth) (preferably [Smart App
    /// Launch](http://hl7.org/fhir/smart-app-launch)), with a [clear policy on
    /// authentication strength](security.html#authentication)
    #[serde(rename = "use-ouath")]
    UseOuath,
    /// My system uses [OpenID Connect](https://openid.net/connect/) (or other
    /// suitable authentication protocol) to verify identity of end user, where
    /// it is necessary that end-users be identified to the client application,
    /// and has a clear policy on [identity proofing](secpriv-module.html#user)
    #[serde(rename = "use-openidconnect")]
    UseOpenidconnect,
    /// My system applies appropriate access control to every request, using a
    /// combination of requester’s clearance (ABAC) and/or roles (RBAC)
    #[serde(rename = "use-rbac")]
    UseRbac,
    /// My system considers [security labels](security-labels.html) on the
    /// affected resources when making access control decisions
    #[serde(rename = "use-labels")]
    UseLabels,
    /// My system can [render narratives properly](narrative.html#css) and
    /// [securely](security.html#narrative)(where they are used)
    #[serde(rename = "render-narratives")]
    RenderNarratives,
    /// My system [validates all input received](validation.html) (whether in
    /// resource format or other) from other actors so that it data is
    /// well-formed and does not contain content that would cause unwanted
    /// system behavior
    #[serde(rename = "check=validation")]
    CheckValidation,
    /// My system makes the right [Provenance](provenance.html) statements and
    /// [AuditEvent](auditevent.html) logs, and uses the right [security
    /// labels](security-labels.html#core) where appropriate
    #[serde(rename = "use-provenance")]
    UseProvenance,
    /// Server: CORS ([cross-origin resource sharing](http://enable-cors.org/))
    /// is appropriately enabled (many clients are Javascript apps running in a
    /// browser)
    #[serde(rename = "enable-cors")]
    EnableCors,
    /// JSON is supported (many clients are Javascript apps running in a
    /// browser; XML is inconvenient at best)
    #[serde(rename = "use-json")]
    UseJson,
    /// JSON is returned correctly when errors happen (clients often don't
    /// handle HTML errors well)
    #[serde(rename = "json-for-errors")]
    JsonForErrors,
    /// The _format header is supported correctly
    #[serde(rename = "use-format-header")]
    UseFormatHeader,
    /// Errors are trapped and an OperationOutcome returned
    #[serde(rename = "use-operation-outcome")]
    UseOperationOutcome,
}

/// This codeSystem contains example structural roles. In general, two types of
/// roles can be distinguished: structural roles and functional roles.
/// Structural Roles reflect human or organizational categories (hierarchies),
/// and describe prerequisites, feasibilities, or competences for actions.
/// Functional roles are bound to the realization or performance of actions..
///
/// System: <http://hl7.org/fhir/sample-security-structural-roles>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SampleSecurityStructuralRoles {
    /// Regulated Health Professionals
    #[default]
    #[serde(rename = "regulated-health-professionals")]
    RegulatedHealthProfessionals,
    /// General Medicine
    #[serde(rename = "general-medicine")]
    GeneralMedicine,
    /// General Nursing
    #[serde(rename = "general-nursing")]
    GeneralNursing,
    /// Dentist
    #[serde(rename = "dentist")]
    Dentist,
    /// Veterinarian
    #[serde(rename = "veterinarian")]
    Veterinarian,
    /// Pharmacy
    #[serde(rename = "pharmacy")]
    Pharmacy,
    /// Dietician
    #[serde(rename = "dietician")]
    Dietician,
    /// Pediatrics
    #[serde(rename = "pediatrics")]
    Pediatrics,
    /// Non-Regulated health Professionals
    #[serde(rename = "non-regulated-health-professionals")]
    NonRegulatedHealthProfessionals,
    /// Receptionist
    #[serde(rename = "receptionist")]
    Receptionist,
    /// Business Manager
    #[serde(rename = "business-manager")]
    BusinessManager,
    /// Transcriptionist
    #[serde(rename = "transcriptionist")]
    Transcriptionist,
    /// Claims Adjudicator
    #[serde(rename = "claims-adjudicator")]
    ClaimsAdjudicator,
}

/// What Search Comparator Codes are supported in search.
///
/// System: <http://hl7.org/fhir/search-comparator>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchComparator {
    /// Equals
    #[default]
    #[serde(rename = "eq")]
    Eq,
    /// Not Equals
    #[serde(rename = "ne")]
    Ne,
    /// Greater Than
    #[serde(rename = "gt")]
    Gt,
    /// Less Than
    #[serde(rename = "lt")]
    Lt,
    /// Greater or Equals
    #[serde(rename = "ge")]
    Ge,
    /// Less of Equal
    #[serde(rename = "le")]
    Le,
    /// Starts After
    #[serde(rename = "sa")]
    Sa,
    /// Ends Before
    #[serde(rename = "eb")]
    Eb,
    /// Approximately
    #[serde(rename = "ap")]
    Ap,
}

/// Why an entry is in the result set - whether it's included as a match or
/// because of an _include requirement, or to convey information or warning
/// information about the search process.
///
/// System: <http://hl7.org/fhir/search-entry-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchEntryMode {
    /// Match
    #[default]
    #[serde(rename = "match")]
    Match,
    /// Include
    #[serde(rename = "include")]
    Include,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
}

/// A supported modifier for a search parameter.
///
/// System: <http://hl7.org/fhir/search-modifier-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchModifierCode {
    /// Missing
    #[default]
    #[serde(rename = "missing")]
    Missing,
    /// Exact
    #[serde(rename = "exact")]
    Exact,
    /// Contains
    #[serde(rename = "contains")]
    Contains,
    /// Not
    #[serde(rename = "not")]
    Not,
    /// Text
    #[serde(rename = "text")]
    Text,
    /// In
    #[serde(rename = "in")]
    In,
    /// Not In
    #[serde(rename = "not-in")]
    NotIn,
    /// Below
    #[serde(rename = "below")]
    Below,
    /// Above
    #[serde(rename = "above")]
    Above,
    /// Type
    #[serde(rename = "type")]
    Type,
    /// Identifier
    #[serde(rename = "identifier")]
    Identifier,
    /// Of Type
    #[serde(rename = "of-type")]
    OfType,
    /// Code Text
    #[serde(rename = "code-text")]
    CodeText,
    /// Text Advanced
    #[serde(rename = "text-advanced")]
    TextAdvanced,
    /// Iterate
    #[serde(rename = "iterate")]
    Iterate,
}

/// Data types allowed to be used for search parameters.
///
/// System: <http://hl7.org/fhir/search-param-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchParamType {
    /// Number
    #[default]
    #[serde(rename = "number")]
    Number,
    /// Date/DateTime
    #[serde(rename = "date")]
    Date,
    /// String
    #[serde(rename = "string")]
    String,
    /// Token
    #[serde(rename = "token")]
    Token,
    /// Reference
    #[serde(rename = "reference")]
    Reference,
    /// Composite
    #[serde(rename = "composite")]
    Composite,
    /// Quantity
    #[serde(rename = "quantity")]
    Quantity,
    /// URI
    #[serde(rename = "uri")]
    Uri,
    /// Special
    #[serde(rename = "special")]
    Special,
}

/// How a search parameter relates to the set of elements returned by
/// evaluating its expression query.
///
/// System: <http://hl7.org/fhir/search-processingmode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchProcessingmode {
    /// Normal
    #[default]
    #[serde(rename = "normal")]
    Normal,
    /// Phonetic
    #[serde(rename = "phonetic")]
    Phonetic,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Type if a sequence -- DNA, RNA, or amino acid sequence.
///
/// System: <http://hl7.org/fhir/sequence-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SequenceType {
    /// AA Sequence
    #[default]
    #[serde(rename = "aa")]
    Aa,
    /// DNA Sequence
    #[serde(rename = "dna")]
    Dna,
    /// RNA Sequence
    #[serde(rename = "rna")]
    Rna,
}

/// An example set of Service Modes that could be applicable to use to
/// characterize HealthcareServices or PractitionerRoles while searching
///
/// System: <http://hl7.org/fhir/service-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ServiceMode {
    /// In Person
    #[default]
    #[serde(rename = "in-person")]
    InPerson,
    /// Telephone
    #[serde(rename = "telephone")]
    Telephone,
    /// Video Conference
    #[serde(rename = "videoconference")]
    Videoconference,
    /// Chat/Messaging
    #[serde(rename = "chat")]
    Chat,
}

/// Codes providing the parameter codes for service request details.
///
/// System: <http://hl7.org/fhir/servicerequest-orderdetail-parameter-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ServicerequestOrderdetailParameterCode {
    /// Catheter Insertion
    #[default]
    #[serde(rename = "catheter-insertion")]
    CatheterInsertion,
    /// Body Elevation
    #[serde(rename = "body-elevation")]
    BodyElevation,
    /// Device Configuration
    #[serde(rename = "device-configuration")]
    DeviceConfiguration,
    /// Device Settings
    #[serde(rename = "device-settings")]
    DeviceSettings,
}

/// The free/busy status of the slot.
///
/// System: <http://hl7.org/fhir/slotstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Slotstatus {
    /// Busy
    #[default]
    #[serde(rename = "busy")]
    Busy,
    /// Free
    #[serde(rename = "free")]
    Free,
    /// Busy (Unavailable)
    #[serde(rename = "busy-unavailable")]
    BusyUnavailable,
    /// Busy (Tentative)
    #[serde(rename = "busy-tentative")]
    BusyTentative,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The possible sort directions, ascending or descending.
///
/// System: <http://hl7.org/fhir/sort-direction>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SortDirection {
    /// Ascending
    #[default]
    #[serde(rename = "ascending")]
    Ascending,
    /// Descending
    #[serde(rename = "descending")]
    Descending,
}

/// The license that applies to an Implementation Guide (using an SPDX license
/// Identifiers, or 'not-open-source'). The binding is required but new SPDX
/// license Identifiers are allowed to be used (https://spdx.org/licenses/).
///
/// System: <http://hl7.org/fhir/spdx-license>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpdxLicense {
    /// Not open source
    #[default]
    #[serde(rename = "not-open-source")]
    NotOpenSource,
    /// BSD Zero Clause License
    #[serde(rename = "0BSD")]
    N0Bsd,
    /// Attribution Assurance License
    #[serde(rename = "AAL")]
    Aal,
    /// Abstyles License
    #[serde(rename = "Abstyles")]
    Abstyles,
    /// Adobe Systems Incorporated Source Code License Agreement
    #[serde(rename = "Adobe-2006")]
    Adobe2006,
    /// Adobe Glyph List License
    #[serde(rename = "Adobe-Glyph")]
    AdobeGlyph,
    /// Amazon Digital Services License
    #[serde(rename = "ADSL")]
    Adsl,
    /// Academic Free License v1.1
    #[serde(rename = "AFL-1.1")]
    Afl11,
    /// Academic Free License v1.2
    #[serde(rename = "AFL-1.2")]
    Afl12,
    /// Academic Free License v2.0
    #[serde(rename = "AFL-2.0")]
    Afl20,
    /// Academic Free License v2.1
    #[serde(rename = "AFL-2.1")]
    Afl21,
    /// Academic Free License v3.0
    #[serde(rename = "AFL-3.0")]
    Afl30,
    /// Afmparse License
    #[serde(rename = "Afmparse")]
    Afmparse,
    /// Affero General Public License v1.0 only
    #[serde(rename = "AGPL-1.0-only")]
    Agpl10Only,
    /// Affero General Public License v1.0 or later
    #[serde(rename = "AGPL-1.0-or-later")]
    Agpl10OrLater,
    /// GNU Affero General Public License v3.0 only
    #[serde(rename = "AGPL-3.0-only")]
    Agpl30Only,
    /// GNU Affero General Public License v3.0 or later
    #[serde(rename = "AGPL-3.0-or-later")]
    Agpl30OrLater,
    /// Aladdin Free Public License
    #[serde(rename = "Aladdin")]
    Aladdin,
    /// AMD's plpa_map.c License
    #[serde(rename = "AMDPLPA")]
    Amdplpa,
    /// Apple MIT License
    #[serde(rename = "AML")]
    Aml,
    /// Academy of Motion Picture Arts and Sciences BSD
    #[serde(rename = "AMPAS")]
    Ampas,
    /// ANTLR Software Rights Notice
    #[serde(rename = "ANTLR-PD")]
    AntlrPd,
    /// Apache License 1.0
    #[serde(rename = "Apache-1.0")]
    Apache10,
    /// Apache License 1.1
    #[serde(rename = "Apache-1.1")]
    Apache11,
    /// Apache License 2.0
    #[serde(rename = "Apache-2.0")]
    Apache20,
    /// Adobe Postscript AFM License
    #[serde(rename = "APAFML")]
    Apafml,
    /// Adaptive Public License 1.0
    #[serde(rename = "APL-1.0")]
    Apl10,
    /// Apple Public Source License 1.0
    #[serde(rename = "APSL-1.0")]
    Apsl10,
    /// Apple Public Source License 1.1
    #[serde(rename = "APSL-1.1")]
    Apsl11,
    /// Apple Public Source License 1.2
    #[serde(rename = "APSL-1.2")]
    Apsl12,
    /// Apple Public Source License 2.0
    #[serde(rename = "APSL-2.0")]
    Apsl20,
    /// Artistic License 1.0 w/clause 8
    #[serde(rename = "Artistic-1.0-cl8")]
    Artistic10Cl8,
    /// Artistic License 1.0 (Perl)
    #[serde(rename = "Artistic-1.0-Perl")]
    Artistic10Perl,
    /// Artistic License 1.0
    #[serde(rename = "Artistic-1.0")]
    Artistic10,
    /// Artistic License 2.0
    #[serde(rename = "Artistic-2.0")]
    Artistic20,
    /// Bahyph License
    #[serde(rename = "Bahyph")]
    Bahyph,
    /// Barr License
    #[serde(rename = "Barr")]
    Barr,
    /// Beerware License
    #[serde(rename = "Beerware")]
    Beerware,
    /// BitTorrent Open Source License v1.0
    #[serde(rename = "BitTorrent-1.0")]
    BitTorrent10,
    /// BitTorrent Open Source License v1.1
    #[serde(rename = "BitTorrent-1.1")]
    BitTorrent11,
    /// Borceux license
    #[serde(rename = "Borceux")]
    Borceux,
    /// BSD 1-Clause License
    #[serde(rename = "BSD-1-Clause")]
    Bsd1Clause,
    /// BSD 2-Clause FreeBSD License
    #[serde(rename = "BSD-2-Clause-FreeBSD")]
    Bsd2ClauseFreeBsd,
    /// BSD 2-Clause NetBSD License
    #[serde(rename = "BSD-2-Clause-NetBSD")]
    Bsd2ClauseNetBsd,
    /// BSD-2-Clause Plus Patent License
    #[serde(rename = "BSD-2-Clause-Patent")]
    Bsd2ClausePatent,
    /// BSD 2-Clause "Simplified" License
    #[serde(rename = "BSD-2-Clause")]
    Bsd2Clause,
    /// BSD with attribution
    #[serde(rename = "BSD-3-Clause-Attribution")]
    Bsd3ClauseAttribution,
    /// BSD 3-Clause Clear License
    #[serde(rename = "BSD-3-Clause-Clear")]
    Bsd3ClauseClear,
    /// Lawrence Berkeley National Labs BSD variant license
    #[serde(rename = "BSD-3-Clause-LBNL")]
    Bsd3ClauseLbnl,
    /// BSD 3-Clause No Nuclear License 2014
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License-2014")]
    Bsd3ClauseNoNuclearLicense2014,
    /// BSD 3-Clause No Nuclear License
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License")]
    Bsd3ClauseNoNuclearLicense,
    /// BSD 3-Clause No Nuclear Warranty
    #[serde(rename = "BSD-3-Clause-No-Nuclear-Warranty")]
    Bsd3ClauseNoNuclearWarranty,
    /// BSD 3-Clause "New" or "Revised" License
    #[serde(rename = "BSD-3-Clause")]
    Bsd3Clause,
    /// BSD-4-Clause (University of California-Specific)
    #[serde(rename = "BSD-4-Clause-UC")]
    Bsd4ClauseUc,
    /// BSD 4-Clause "Original" or "Old" License
    #[serde(rename = "BSD-4-Clause")]
    Bsd4Clause,
    /// BSD Protection License
    #[serde(rename = "BSD-Protection")]
    BsdProtection,
    /// BSD Source Code Attribution
    #[serde(rename = "BSD-Source-Code")]
    BsdSourceCode,
    /// Boost Software License 1.0
    #[serde(rename = "BSL-1.0")]
    Bsl10,
    /// bzip2 and libbzip2 License v1.0.5
    #[serde(rename = "bzip2-1.0.5")]
    Bzip2105,
    /// bzip2 and libbzip2 License v1.0.6
    #[serde(rename = "bzip2-1.0.6")]
    Bzip2106,
    /// Caldera License
    #[serde(rename = "Caldera")]
    Caldera,
    /// Computer Associates Trusted Open Source License 1.1
    #[serde(rename = "CATOSL-1.1")]
    Catosl11,
    /// Creative Commons Attribution 1.0 Generic
    #[serde(rename = "CC-BY-1.0")]
    CcBy10,
    /// Creative Commons Attribution 2.0 Generic
    #[serde(rename = "CC-BY-2.0")]
    CcBy20,
    /// Creative Commons Attribution 2.5 Generic
    #[serde(rename = "CC-BY-2.5")]
    CcBy25,
    /// Creative Commons Attribution 3.0 Unported
    #[serde(rename = "CC-BY-3.0")]
    CcBy30,
    /// Creative Commons Attribution 4.0 International
    #[serde(rename = "CC-BY-4.0")]
    CcBy40,
    /// Creative Commons Attribution Non Commercial 1.0 Generic
    #[serde(rename = "CC-BY-NC-1.0")]
    CcByNc10,
    /// Creative Commons Attribution Non Commercial 2.0 Generic
    #[serde(rename = "CC-BY-NC-2.0")]
    CcByNc20,
    /// Creative Commons Attribution Non Commercial 2.5 Generic
    #[serde(rename = "CC-BY-NC-2.5")]
    CcByNc25,
    /// Creative Commons Attribution Non Commercial 3.0 Unported
    #[serde(rename = "CC-BY-NC-3.0")]
    CcByNc30,
    /// Creative Commons Attribution Non Commercial 4.0 International
    #[serde(rename = "CC-BY-NC-4.0")]
    CcByNc40,
    /// Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic
    #[serde(rename = "CC-BY-NC-ND-1.0")]
    CcByNcNd10,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic
    #[serde(rename = "CC-BY-NC-ND-2.0")]
    CcByNcNd20,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic
    #[serde(rename = "CC-BY-NC-ND-2.5")]
    CcByNcNd25,
    /// Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported
    #[serde(rename = "CC-BY-NC-ND-3.0")]
    CcByNcNd30,
    /// Creative Commons Attribution Non Commercial No Derivatives 4.0
    /// International
    #[serde(rename = "CC-BY-NC-ND-4.0")]
    CcByNcNd40,
    /// Creative Commons Attribution Non Commercial Share Alike 1.0 Generic
    #[serde(rename = "CC-BY-NC-SA-1.0")]
    CcByNcSa10,
    /// Creative Commons Attribution Non Commercial Share Alike 2.0 Generic
    #[serde(rename = "CC-BY-NC-SA-2.0")]
    CcByNcSa20,
    /// Creative Commons Attribution Non Commercial Share Alike 2.5 Generic
    #[serde(rename = "CC-BY-NC-SA-2.5")]
    CcByNcSa25,
    /// Creative Commons Attribution Non Commercial Share Alike 3.0 Unported
    #[serde(rename = "CC-BY-NC-SA-3.0")]
    CcByNcSa30,
    /// Creative Commons Attribution Non Commercial Share Alike 4.0
    /// International
    #[serde(rename = "CC-BY-NC-SA-4.0")]
    CcByNcSa40,
    /// Creative Commons Attribution No Derivatives 1.0 Generic
    #[serde(rename = "CC-BY-ND-1.0")]
    CcByNd10,
    /// Creative Commons Attribution No Derivatives 2.0 Generic
    #[serde(rename = "CC-BY-ND-2.0")]
    CcByNd20,
    /// Creative Commons Attribution No Derivatives 2.5 Generic
    #[serde(rename = "CC-BY-ND-2.5")]
    CcByNd25,
    /// Creative Commons Attribution No Derivatives 3.0 Unported
    #[serde(rename = "CC-BY-ND-3.0")]
    CcByNd30,
    /// Creative Commons Attribution No Derivatives 4.0 International
    #[serde(rename = "CC-BY-ND-4.0")]
    CcByNd40,
    /// Creative Commons Attribution Share Alike 1.0 Generic
    #[serde(rename = "CC-BY-SA-1.0")]
    CcBySa10,
    /// Creative Commons Attribution Share Alike 2.0 Generic
    #[serde(rename = "CC-BY-SA-2.0")]
    CcBySa20,
    /// Creative Commons Attribution Share Alike 2.5 Generic
    #[serde(rename = "CC-BY-SA-2.5")]
    CcBySa25,
    /// Creative Commons Attribution Share Alike 3.0 Unported
    #[serde(rename = "CC-BY-SA-3.0")]
    CcBySa30,
    /// Creative Commons Attribution Share Alike 4.0 International
    #[serde(rename = "CC-BY-SA-4.0")]
    CcBySa40,
    /// Creative Commons Zero v1.0 Universal
    #[serde(rename = "CC0-1.0")]
    Cc010,
    /// Common Development and Distribution License 1.0
    #[serde(rename = "CDDL-1.0")]
    Cddl10,
    /// Common Development and Distribution License 1.1
    #[serde(rename = "CDDL-1.1")]
    Cddl11,
    /// Community Data License Agreement Permissive 1.0
    #[serde(rename = "CDLA-Permissive-1.0")]
    CdlaPermissive10,
    /// Community Data License Agreement Sharing 1.0
    #[serde(rename = "CDLA-Sharing-1.0")]
    CdlaSharing10,
    /// CeCILL Free Software License Agreement v1.0
    #[serde(rename = "CECILL-1.0")]
    Cecill10,
    /// CeCILL Free Software License Agreement v1.1
    #[serde(rename = "CECILL-1.1")]
    Cecill11,
    /// CeCILL Free Software License Agreement v2.0
    #[serde(rename = "CECILL-2.0")]
    Cecill20,
    /// CeCILL Free Software License Agreement v2.1
    #[serde(rename = "CECILL-2.1")]
    Cecill21,
    /// CeCILL-B Free Software License Agreement
    #[serde(rename = "CECILL-B")]
    CecillB,
    /// CeCILL-C Free Software License Agreement
    #[serde(rename = "CECILL-C")]
    CecillC,
    /// Clarified Artistic License
    #[serde(rename = "ClArtistic")]
    ClArtistic,
    /// CNRI Jython License
    #[serde(rename = "CNRI-Jython")]
    CnriJython,
    /// CNRI Python Open Source GPL Compatible License Agreement
    #[serde(rename = "CNRI-Python-GPL-Compatible")]
    CnriPythonGplCompatible,
    /// CNRI Python License
    #[serde(rename = "CNRI-Python")]
    CnriPython,
    /// Condor Public License v1.1
    #[serde(rename = "Condor-1.1")]
    Condor11,
    /// Common Public Attribution License 1.0
    #[serde(rename = "CPAL-1.0")]
    Cpal10,
    /// Common Public License 1.0
    #[serde(rename = "CPL-1.0")]
    Cpl10,
    /// Code Project Open License 1.02
    #[serde(rename = "CPOL-1.02")]
    Cpol102,
    /// Crossword License
    #[serde(rename = "Crossword")]
    Crossword,
    /// CrystalStacker License
    #[serde(rename = "CrystalStacker")]
    CrystalStacker,
    /// CUA Office Public License v1.0
    #[serde(rename = "CUA-OPL-1.0")]
    CuaOpl10,
    /// Cube License
    #[serde(rename = "Cube")]
    Cube,
    /// curl License
    #[serde(rename = "curl")]
    Curl,
    /// Deutsche Freie Software Lizenz
    #[serde(rename = "D-FSL-1.0")]
    DFsl10,
    /// diffmark license
    #[serde(rename = "diffmark")]
    Diffmark,
    /// DOC License
    #[serde(rename = "DOC")]
    Doc,
    /// Dotseqn License
    #[serde(rename = "Dotseqn")]
    Dotseqn,
    /// DSDP License
    #[serde(rename = "DSDP")]
    Dsdp,
    /// dvipdfm License
    #[serde(rename = "dvipdfm")]
    Dvipdfm,
    /// Educational Community License v1.0
    #[serde(rename = "ECL-1.0")]
    Ecl10,
    /// Educational Community License v2.0
    #[serde(rename = "ECL-2.0")]
    Ecl20,
    /// Eiffel Forum License v1.0
    #[serde(rename = "EFL-1.0")]
    Efl10,
    /// Eiffel Forum License v2.0
    #[serde(rename = "EFL-2.0")]
    Efl20,
    /// eGenix.com Public License 1.1.0
    #[serde(rename = "eGenix")]
    EGenix,
    /// Entessa Public License v1.0
    #[serde(rename = "Entessa")]
    Entessa,
    /// Eclipse Public License 1.0
    #[serde(rename = "EPL-1.0")]
    Epl10,
    /// Eclipse Public License 2.0
    #[serde(rename = "EPL-2.0")]
    Epl20,
    /// Erlang Public License v1.1
    #[serde(rename = "ErlPL-1.1")]
    ErlPl11,
    /// EU DataGrid Software License
    #[serde(rename = "EUDatagrid")]
    EuDatagrid,
    /// European Union Public License 1.0
    #[serde(rename = "EUPL-1.0")]
    Eupl10,
    /// European Union Public License 1.1
    #[serde(rename = "EUPL-1.1")]
    Eupl11,
    /// European Union Public License 1.2
    #[serde(rename = "EUPL-1.2")]
    Eupl12,
    /// Eurosym License
    #[serde(rename = "Eurosym")]
    Eurosym,
    /// Fair License
    #[serde(rename = "Fair")]
    Fair,
    /// Frameworx Open License 1.0
    #[serde(rename = "Frameworx-1.0")]
    Frameworx10,
    /// FreeImage Public License v1.0
    #[serde(rename = "FreeImage")]
    FreeImage,
    /// FSF All Permissive License
    #[serde(rename = "FSFAP")]
    Fsfap,
    /// FSF Unlimited License
    #[serde(rename = "FSFUL")]
    Fsful,
    /// FSF Unlimited License (with License Retention)
    #[serde(rename = "FSFULLR")]
    Fsfullr,
    /// Freetype Project License
    #[serde(rename = "FTL")]
    Ftl,
    /// GNU Free Documentation License v1.1 only
    #[serde(rename = "GFDL-1.1-only")]
    Gfdl11Only,
    /// GNU Free Documentation License v1.1 or later
    #[serde(rename = "GFDL-1.1-or-later")]
    Gfdl11OrLater,
    /// GNU Free Documentation License v1.2 only
    #[serde(rename = "GFDL-1.2-only")]
    Gfdl12Only,
    /// GNU Free Documentation License v1.2 or later
    #[serde(rename = "GFDL-1.2-or-later")]
    Gfdl12OrLater,
    /// GNU Free Documentation License v1.3 only
    #[serde(rename = "GFDL-1.3-only")]
    Gfdl13Only,
    /// GNU Free Documentation License v1.3 or later
    #[serde(rename = "GFDL-1.3-or-later")]
    Gfdl13OrLater,
    /// Giftware License
    #[serde(rename = "Giftware")]
    Giftware,
    /// GL2PS License
    #[serde(rename = "GL2PS")]
    Gl2Ps,
    /// 3dfx Glide License
    #[serde(rename = "Glide")]
    Glide,
    /// Glulxe License
    #[serde(rename = "Glulxe")]
    Glulxe,
    /// gnuplot License
    #[serde(rename = "gnuplot")]
    Gnuplot,
    /// GNU General Public License v1.0 only
    #[serde(rename = "GPL-1.0-only")]
    Gpl10Only,
    /// GNU General Public License v1.0 or later
    #[serde(rename = "GPL-1.0-or-later")]
    Gpl10OrLater,
    /// GNU General Public License v2.0 only
    #[serde(rename = "GPL-2.0-only")]
    Gpl20Only,
    /// GNU General Public License v2.0 or later
    #[serde(rename = "GPL-2.0-or-later")]
    Gpl20OrLater,
    /// GNU General Public License v3.0 only
    #[serde(rename = "GPL-3.0-only")]
    Gpl30Only,
    /// GNU General Public License v3.0 or later
    #[serde(rename = "GPL-3.0-or-later")]
    Gpl30OrLater,
    /// gSOAP Public License v1.3b
    #[serde(rename = "gSOAP-1.3b")]
    GSoap13B,
    /// Haskell Language Report License
    #[serde(rename = "HaskellReport")]
    HaskellReport,
    /// Historical Permission Notice and Disclaimer
    #[serde(rename = "HPND")]
    Hpnd,
    /// IBM PowerPC Initialization and Boot Software
    #[serde(rename = "IBM-pibs")]
    IbmPibs,
    /// ICU License
    #[serde(rename = "ICU")]
    Icu,
    /// Independent JPEG Group License
    #[serde(rename = "IJG")]
    Ijg,
    /// ImageMagick License
    #[serde(rename = "ImageMagick")]
    ImageMagick,
    /// iMatix Standard Function Library Agreement
    #[serde(rename = "iMatix")]
    IMatix,
    /// Imlib2 License
    #[serde(rename = "Imlib2")]
    Imlib2,
    /// Info-ZIP License
    #[serde(rename = "Info-ZIP")]
    InfoZip,
    /// Intel ACPI Software License Agreement
    #[serde(rename = "Intel-ACPI")]
    IntelAcpi,
    /// Intel Open Source License
    #[serde(rename = "Intel")]
    Intel,
    /// Interbase Public License v1.0
    #[serde(rename = "Interbase-1.0")]
    Interbase10,
    /// IPA Font License
    #[serde(rename = "IPA")]
    Ipa,
    /// IBM Public License v1.0
    #[serde(rename = "IPL-1.0")]
    Ipl10,
    /// ISC License
    #[serde(rename = "ISC")]
    Isc,
    /// JasPer License
    #[serde(rename = "JasPer-2.0")]
    JasPer20,
    /// JSON License
    #[serde(rename = "JSON")]
    Json,
    /// Licence Art Libre 1.2
    #[serde(rename = "LAL-1.2")]
    Lal12,
    /// Licence Art Libre 1.3
    #[serde(rename = "LAL-1.3")]
    Lal13,
    /// Latex2e License
    #[serde(rename = "Latex2e")]
    Latex2E,
    /// Leptonica License
    #[serde(rename = "Leptonica")]
    Leptonica,
    /// GNU Library General Public License v2 only
    #[serde(rename = "LGPL-2.0-only")]
    Lgpl20Only,
    /// GNU Library General Public License v2 or later
    #[serde(rename = "LGPL-2.0-or-later")]
    Lgpl20OrLater,
    /// GNU Lesser General Public License v2.1 only
    #[serde(rename = "LGPL-2.1-only")]
    Lgpl21Only,
    /// GNU Lesser General Public License v2.1 or later
    #[serde(rename = "LGPL-2.1-or-later")]
    Lgpl21OrLater,
    /// GNU Lesser General Public License v3.0 only
    #[serde(rename = "LGPL-3.0-only")]
    Lgpl30Only,
    /// GNU Lesser General Public License v3.0 or later
    #[serde(rename = "LGPL-3.0-or-later")]
    Lgpl30OrLater,
    /// Lesser General Public License For Linguistic Resources
    #[serde(rename = "LGPLLR")]
    Lgpllr,
    /// libpng License
    #[serde(rename = "Libpng")]
    Libpng,
    /// libtiff License
    #[serde(rename = "libtiff")]
    Libtiff,
    /// Licence Libre du Québec – Permissive version 1.1
    #[serde(rename = "LiLiQ-P-1.1")]
    LiLiQP11,
    /// Licence Libre du Québec – Réciprocité version 1.1
    #[serde(rename = "LiLiQ-R-1.1")]
    LiLiQR11,
    /// Licence Libre du Québec – Réciprocité forte version 1.1
    #[serde(rename = "LiLiQ-Rplus-1.1")]
    LiLiQRplus11,
    /// Linux Kernel Variant of OpenIB.org license
    #[serde(rename = "Linux-OpenIB")]
    LinuxOpenIb,
    /// Lucent Public License Version 1.0
    #[serde(rename = "LPL-1.0")]
    Lpl10,
    /// Lucent Public License v1.02
    #[serde(rename = "LPL-1.02")]
    Lpl102,
    /// LaTeX Project Public License v1.0
    #[serde(rename = "LPPL-1.0")]
    Lppl10,
    /// LaTeX Project Public License v1.1
    #[serde(rename = "LPPL-1.1")]
    Lppl11,
    /// LaTeX Project Public License v1.2
    #[serde(rename = "LPPL-1.2")]
    Lppl12,
    /// LaTeX Project Public License v1.3a
    #[serde(rename = "LPPL-1.3a")]
    Lppl13A,
    /// LaTeX Project Public License v1.3c
    #[serde(rename = "LPPL-1.3c")]
    Lppl13C,
    /// MakeIndex License
    #[serde(rename = "MakeIndex")]
    MakeIndex,
    /// MirOS License
    #[serde(rename = "MirOS")]
    MirOs,
    /// MIT No Attribution
    #[serde(rename = "MIT-0")]
    Mit0,
    /// Enlightenment License (e16)
    #[serde(rename = "MIT-advertising")]
    MitAdvertising,
    /// CMU License
    #[serde(rename = "MIT-CMU")]
    MitCmu,
    /// enna License
    #[serde(rename = "MIT-enna")]
    MitEnna,
    /// feh License
    #[serde(rename = "MIT-feh")]
    MitFeh,
    /// MIT License
    #[serde(rename = "MIT")]
    Mit,
    /// MIT +no-false-attribs license
    #[serde(rename = "MITNFA")]
    Mitnfa,
    /// Motosoto License
    #[serde(rename = "Motosoto")]
    Motosoto,
    /// mpich2 License
    #[serde(rename = "mpich2")]
    Mpich2,
    /// Mozilla Public License 1.0
    #[serde(rename = "MPL-1.0")]
    Mpl10,
    /// Mozilla Public License 1.1
    #[serde(rename = "MPL-1.1")]
    Mpl11,
    /// Mozilla Public License 2.0 (no copyleft exception)
    #[serde(rename = "MPL-2.0-no-copyleft-exception")]
    Mpl20NoCopyleftException,
    /// Mozilla Public License 2.0
    #[serde(rename = "MPL-2.0")]
    Mpl20,
    /// Microsoft Public License
    #[serde(rename = "MS-PL")]
    MsPl,
    /// Microsoft Reciprocal License
    #[serde(rename = "MS-RL")]
    MsRl,
    /// Matrix Template Library License
    #[serde(rename = "MTLL")]
    Mtll,
    /// Multics License
    #[serde(rename = "Multics")]
    Multics,
    /// Mup License
    #[serde(rename = "Mup")]
    Mup,
    /// NASA Open Source Agreement 1.3
    #[serde(rename = "NASA-1.3")]
    Nasa13,
    /// Naumen Public License
    #[serde(rename = "Naumen")]
    Naumen,
    /// Net Boolean Public License v1
    #[serde(rename = "NBPL-1.0")]
    Nbpl10,
    /// University of Illinois/NCSA Open Source License
    #[serde(rename = "NCSA")]
    Ncsa,
    /// Net-SNMP License
    #[serde(rename = "Net-SNMP")]
    NetSnmp,
    /// NetCDF license
    #[serde(rename = "NetCDF")]
    NetCdf,
    /// Newsletr License
    #[serde(rename = "Newsletr")]
    Newsletr,
    /// Nethack General Public License
    #[serde(rename = "NGPL")]
    Ngpl,
    /// Norwegian Licence for Open Government Data
    #[serde(rename = "NLOD-1.0")]
    Nlod10,
    /// No Limit Public License
    #[serde(rename = "NLPL")]
    Nlpl,
    /// Nokia Open Source License
    #[serde(rename = "Nokia")]
    Nokia,
    /// Netizen Open Source License
    #[serde(rename = "NOSL")]
    Nosl,
    /// Noweb License
    #[serde(rename = "Noweb")]
    Noweb,
    /// Netscape Public License v1.0
    #[serde(rename = "NPL-1.0")]
    Npl10,
    /// Netscape Public License v1.1
    #[serde(rename = "NPL-1.1")]
    Npl11,
    /// Non-Profit Open Software License 3.0
    #[serde(rename = "NPOSL-3.0")]
    Nposl30,
    /// NRL License
    #[serde(rename = "NRL")]
    Nrl,
    /// NTP License
    #[serde(rename = "NTP")]
    Ntp,
    /// Open CASCADE Technology Public License
    #[serde(rename = "OCCT-PL")]
    OcctPl,
    /// OCLC Research Public License 2.0
    #[serde(rename = "OCLC-2.0")]
    Oclc20,
    /// ODC Open Database License v1.0
    #[serde(rename = "ODbL-1.0")]
    ODbL10,
    /// SIL Open Font License 1.0
    #[serde(rename = "OFL-1.0")]
    Ofl10,
    /// SIL Open Font License 1.1
    #[serde(rename = "OFL-1.1")]
    Ofl11,
    /// Open Group Test Suite License
    #[serde(rename = "OGTSL")]
    Ogtsl,
    /// Open LDAP Public License v1.1
    #[serde(rename = "OLDAP-1.1")]
    Oldap11,
    /// Open LDAP Public License v1.2
    #[serde(rename = "OLDAP-1.2")]
    Oldap12,
    /// Open LDAP Public License v1.3
    #[serde(rename = "OLDAP-1.3")]
    Oldap13,
    /// Open LDAP Public License v1.4
    #[serde(rename = "OLDAP-1.4")]
    Oldap14,
    /// Open LDAP Public License v2.0.1
    #[serde(rename = "OLDAP-2.0.1")]
    Oldap201,
    /// Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)
    #[serde(rename = "OLDAP-2.0")]
    Oldap20,
    /// Open LDAP Public License v2.1
    #[serde(rename = "OLDAP-2.1")]
    Oldap21,
    /// Open LDAP Public License v2.2.1
    #[serde(rename = "OLDAP-2.2.1")]
    Oldap221,
    /// Open LDAP Public License 2.2.2
    #[serde(rename = "OLDAP-2.2.2")]
    Oldap222,
    /// Open LDAP Public License v2.2
    #[serde(rename = "OLDAP-2.2")]
    Oldap22,
    /// Open LDAP Public License v2.3
    #[serde(rename = "OLDAP-2.3")]
    Oldap23,
    /// Open LDAP Public License v2.4
    #[serde(rename = "OLDAP-2.4")]
    Oldap24,
    /// Open LDAP Public License v2.5
    #[serde(rename = "OLDAP-2.5")]
    Oldap25,
    /// Open LDAP Public License v2.6
    #[serde(rename = "OLDAP-2.6")]
    Oldap26,
    /// Open LDAP Public License v2.7
    #[serde(rename = "OLDAP-2.7")]
    Oldap27,
    /// Open LDAP Public License v2.8
    #[serde(rename = "OLDAP-2.8")]
    Oldap28,
    /// Open Market License
    #[serde(rename = "OML")]
    Oml,
    /// OpenSSL License
    #[serde(rename = "OpenSSL")]
    OpenSsl,
    /// Open Public License v1.0
    #[serde(rename = "OPL-1.0")]
    Opl10,
    /// OSET Public License version 2.1
    #[serde(rename = "OSET-PL-2.1")]
    OsetPl21,
    /// Open Software License 1.0
    #[serde(rename = "OSL-1.0")]
    Osl10,
    /// Open Software License 1.1
    #[serde(rename = "OSL-1.1")]
    Osl11,
    /// Open Software License 2.0
    #[serde(rename = "OSL-2.0")]
    Osl20,
    /// Open Software License 2.1
    #[serde(rename = "OSL-2.1")]
    Osl21,
    /// Open Software License 3.0
    #[serde(rename = "OSL-3.0")]
    Osl30,
    /// ODC Public Domain Dedication & License 1.0
    #[serde(rename = "PDDL-1.0")]
    Pddl10,
    /// PHP License v3.0
    #[serde(rename = "PHP-3.0")]
    Php30,
    /// PHP License v3.01
    #[serde(rename = "PHP-3.01")]
    Php301,
    /// Plexus Classworlds License
    #[serde(rename = "Plexus")]
    Plexus,
    /// PostgreSQL License
    #[serde(rename = "PostgreSQL")]
    PostgreSql,
    /// psfrag License
    #[serde(rename = "psfrag")]
    Psfrag,
    /// psutils License
    #[serde(rename = "psutils")]
    Psutils,
    /// Python License 2.0
    #[serde(rename = "Python-2.0")]
    Python20,
    /// Qhull License
    #[serde(rename = "Qhull")]
    Qhull,
    /// Q Public License 1.0
    #[serde(rename = "QPL-1.0")]
    Qpl10,
    /// Rdisc License
    #[serde(rename = "Rdisc")]
    Rdisc,
    /// Red Hat eCos Public License v1.1
    #[serde(rename = "RHeCos-1.1")]
    RHeCos11,
    /// Reciprocal Public License 1.1
    #[serde(rename = "RPL-1.1")]
    Rpl11,
    /// Reciprocal Public License 1.5
    #[serde(rename = "RPL-1.5")]
    Rpl15,
    /// RealNetworks Public Source License v1.0
    #[serde(rename = "RPSL-1.0")]
    Rpsl10,
    /// RSA Message-Digest License
    #[serde(rename = "RSA-MD")]
    RsaMd,
    /// Ricoh Source Code Public License
    #[serde(rename = "RSCPL")]
    Rscpl,
    /// Ruby License
    #[serde(rename = "Ruby")]
    Ruby,
    /// Sax Public Domain Notice
    #[serde(rename = "SAX-PD")]
    SaxPd,
    /// Saxpath License
    #[serde(rename = "Saxpath")]
    Saxpath,
    /// SCEA Shared Source License
    #[serde(rename = "SCEA")]
    Scea,
    /// Sendmail License
    #[serde(rename = "Sendmail")]
    Sendmail,
    /// SGI Free Software License B v1.0
    #[serde(rename = "SGI-B-1.0")]
    SgiB10,
    /// SGI Free Software License B v1.1
    #[serde(rename = "SGI-B-1.1")]
    SgiB11,
    /// SGI Free Software License B v2.0
    #[serde(rename = "SGI-B-2.0")]
    SgiB20,
    /// Simple Public License 2.0
    #[serde(rename = "SimPL-2.0")]
    SimPl20,
    /// Sun Industry Standards Source License v1.2
    #[serde(rename = "SISSL-1.2")]
    Sissl12,
    /// Sun Industry Standards Source License v1.1
    #[serde(rename = "SISSL")]
    Sissl,
    /// Sleepycat License
    #[serde(rename = "Sleepycat")]
    Sleepycat,
    /// Standard ML of New Jersey License
    #[serde(rename = "SMLNJ")]
    Smlnj,
    /// Secure Messaging Protocol Public License
    #[serde(rename = "SMPPL")]
    Smppl,
    /// SNIA Public License 1.1
    #[serde(rename = "SNIA")]
    Snia,
    /// Spencer License 86
    #[serde(rename = "Spencer-86")]
    Spencer86,
    /// Spencer License 94
    #[serde(rename = "Spencer-94")]
    Spencer94,
    /// Spencer License 99
    #[serde(rename = "Spencer-99")]
    Spencer99,
    /// Sun Public License v1.0
    #[serde(rename = "SPL-1.0")]
    Spl10,
    /// SugarCRM Public License v1.1.3
    #[serde(rename = "SugarCRM-1.1.3")]
    SugarCrm113,
    /// Scheme Widget Library (SWL) Software License Agreement
    #[serde(rename = "SWL")]
    Swl,
    /// TCL/TK License
    #[serde(rename = "TCL")]
    Tcl,
    /// TCP Wrappers License
    #[serde(rename = "TCP-wrappers")]
    TcpWrappers,
    /// TMate Open Source License
    #[serde(rename = "TMate")]
    TMate,
    /// TORQUE v2.5+ Software License v1.1
    #[serde(rename = "TORQUE-1.1")]
    Torque11,
    /// Trusster Open Source License
    #[serde(rename = "TOSL")]
    Tosl,
    /// Unicode License Agreement - Data Files and Software (2015)
    #[serde(rename = "Unicode-DFS-2015")]
    UnicodeDfs2015,
    /// Unicode License Agreement - Data Files and Software (2016)
    #[serde(rename = "Unicode-DFS-2016")]
    UnicodeDfs2016,
    /// Unicode Terms of Use
    #[serde(rename = "Unicode-TOU")]
    UnicodeTou,
    /// The Unlicense
    #[serde(rename = "Unlicense")]
    Unlicense,
    /// Universal Permissive License v1.0
    #[serde(rename = "UPL-1.0")]
    Upl10,
    /// Vim License
    #[serde(rename = "Vim")]
    Vim,
    /// VOSTROM Public License for Open Source
    #[serde(rename = "VOSTROM")]
    Vostrom,
    /// Vovida Software License v1.0
    #[serde(rename = "VSL-1.0")]
    Vsl10,
    /// W3C Software Notice and License (1998-07-20)
    #[serde(rename = "W3C-19980720")]
    W3C19980720,
    /// W3C Software Notice and Document License (2015-05-13)
    #[serde(rename = "W3C-20150513")]
    W3C20150513,
    /// W3C Software Notice and License (2002-12-31)
    #[serde(rename = "W3C")]
    W3C,
    /// Sybase Open Watcom Public License 1.0
    #[serde(rename = "Watcom-1.0")]
    Watcom10,
    /// Wsuipa License
    #[serde(rename = "Wsuipa")]
    Wsuipa,
    /// Do What The F*ck You Want To Public License
    #[serde(rename = "WTFPL")]
    Wtfpl,
    /// X11 License
    #[serde(rename = "X11")]
    X11,
    /// Xerox License
    #[serde(rename = "Xerox")]
    Xerox,
    /// XFree86 License 1.1
    #[serde(rename = "XFree86-1.1")]
    XFree8611,
    /// xinetd License
    #[serde(rename = "xinetd")]
    Xinetd,
    /// X.Net License
    #[serde(rename = "Xnet")]
    Xnet,
    /// XPP License
    #[serde(rename = "xpp")]
    Xpp,
    /// XSkat License
    #[serde(rename = "XSkat")]
    XSkat,
    /// Yahoo! Public License v1.0
    #[serde(rename = "YPL-1.0")]
    Ypl10,
    /// Yahoo! Public License v1.1
    #[serde(rename = "YPL-1.1")]
    Ypl11,
    /// Zed License
    #[serde(rename = "Zed")]
    Zed,
    /// Zend License v2.0
    #[serde(rename = "Zend-2.0")]
    Zend20,
    /// Zimbra Public License v1.3
    #[serde(rename = "Zimbra-1.3")]
    Zimbra13,
    /// Zimbra Public License v1.4
    #[serde(rename = "Zimbra-1.4")]
    Zimbra14,
    /// zlib/libpng License with Acknowledgement
    #[serde(rename = "zlib-acknowledgement")]
    ZlibAcknowledgement,
    /// zlib License
    #[serde(rename = "Zlib")]
    Zlib,
    /// Zope Public License 1.1
    #[serde(rename = "ZPL-1.1")]
    Zpl11,
    /// Zope Public License 2.0
    #[serde(rename = "ZPL-2.0")]
    Zpl20,
    /// Zope Public License 2.1
    #[serde(rename = "ZPL-2.1")]
    Zpl21,
}

/// Codes providing the combined status of the specimen.
///
/// System: <http://hl7.org/fhir/specimen-combined>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpecimenCombined {
    /// Grouped
    #[default]
    #[serde(rename = "grouped")]
    Grouped,
    /// Pooled
    #[serde(rename = "pooled")]
    Pooled,
}

/// Degree of preference of a type of conditioned specimen.
///
/// System: <http://hl7.org/fhir/specimen-contained-preference>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpecimenContainedPreference {
    /// Preferred
    #[default]
    #[serde(rename = "preferred")]
    Preferred,
    /// Alternate
    #[serde(rename = "alternate")]
    Alternate,
}

/// Codes providing the combined status of the specimen.
///
/// System: <http://hl7.org/fhir/specimen-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpecimenRole {
    /// Blind Sample
    #[default]
    #[serde(rename = "b")]
    B,
    /// Calibrator
    #[serde(rename = "c")]
    C,
    /// Electronic QC
    #[serde(rename = "e")]
    E,
    /// Filler Organization Proficiency
    #[serde(rename = "f")]
    F,
    /// Operator Proficiency
    #[serde(rename = "o")]
    O,
    /// Patient
    #[serde(rename = "p")]
    P,
    /// Control specimen
    #[serde(rename = "q")]
    Q,
    /// Replicate (of patient sample as a control)
    #[serde(rename = "r")]
    R,
    /// Verifying Calibrator
    #[serde(rename = "v")]
    V,
}

/// Codes providing the status/availability of a specimen.
///
/// System: <http://hl7.org/fhir/specimen-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpecimenStatus {
    /// Available
    #[default]
    #[serde(rename = "available")]
    Available,
    /// Unavailable
    #[serde(rename = "unavailable")]
    Unavailable,
    /// Unsatisfactory
    #[serde(rename = "unsatisfactory")]
    Unsatisfactory,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The role that the assertion variable plays.
///
/// System: <http://hl7.org/fhir/statistic-model-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StatisticModelCode {
    /// one-tailed test (1 threshold)
    #[default]
    #[serde(rename = "oneTailedTest")]
    OneTailedTest,
    /// two-tailed test (2 thresholds)
    #[serde(rename = "twoTailedTest")]
    TwoTailedTest,
    /// z-test
    #[serde(rename = "zTest")]
    ZTest,
    /// 1-sample t-test
    #[serde(rename = "oneSampleTTest")]
    OneSampleTTest,
    /// 2-sample t-test
    #[serde(rename = "twoSampleTTest")]
    TwoSampleTTest,
    /// paired t-test
    #[serde(rename = "pairedTTest")]
    PairedTTest,
    /// Chi-square test
    #[serde(rename = "chiSquareTest")]
    ChiSquareTest,
    /// Chi-square test for trend
    #[serde(rename = "chiSquareTestTrend")]
    ChiSquareTestTrend,
    /// Pearson correlation
    #[serde(rename = "pearsonCorrelation")]
    PearsonCorrelation,
    /// ANOVA (ANalysis Of VAriance)
    #[serde(rename = "anova")]
    Anova,
    /// one-way ANOVA
    #[serde(rename = "anovaOneWay")]
    AnovaOneWay,
    /// 2-way ANOVA without replication
    #[serde(rename = "anovaTwoWay")]
    AnovaTwoWay,
    /// 2-way ANOVA with replication
    #[serde(rename = "anovaTwoWayReplication")]
    AnovaTwoWayReplication,
    /// multivariate ANOVA (MANOVA)
    #[serde(rename = "manova")]
    Manova,
    /// 3-way ANOVA
    #[serde(rename = "anovaThreeWay")]
    AnovaThreeWay,
    /// sign test
    #[serde(rename = "signTest")]
    SignTest,
    /// Wilcoxon signed-rank test
    #[serde(rename = "wilcoxonSignedRankTest")]
    WilcoxonSignedRankTest,
    /// Wilcoxon rank-sum test
    #[serde(rename = "wilcoxonRankSumTest")]
    WilcoxonRankSumTest,
    /// Mann-Whitney U test
    #[serde(rename = "mannWhitneyUTest")]
    MannWhitneyUTest,
    /// Fisher’s exact test
    #[serde(rename = "fishersExactTest")]
    FishersExactTest,
    /// McNemar’s test
    #[serde(rename = "mcnemarsTest")]
    McnemarsTest,
    /// Kruskal Wallis test
    #[serde(rename = "kruskalWallisTest")]
    KruskalWallisTest,
    /// Spearman correlation
    #[serde(rename = "spearmanCorrelation")]
    SpearmanCorrelation,
    /// Kendall correlation
    #[serde(rename = "kendallCorrelation")]
    KendallCorrelation,
    /// Friedman test
    #[serde(rename = "friedmanTest")]
    FriedmanTest,
    /// Goodman Kruska’s Gamma
    #[serde(rename = "goodmanKruskasGamma")]
    GoodmanKruskasGamma,
    /// GLM (Generalized Linear Model)
    #[serde(rename = "glm")]
    Glm,
    /// GLM with probit link
    #[serde(rename = "glmProbit")]
    GlmProbit,
    /// GLM with logit link
    #[serde(rename = "glmLogit")]
    GlmLogit,
    /// GLM with identity link
    #[serde(rename = "glmIdentity")]
    GlmIdentity,
    /// GLM with log link
    #[serde(rename = "glmLog")]
    GlmLog,
    /// GLM with generalized logit link
    #[serde(rename = "glmGeneralizedLogit")]
    GlmGeneralizedLogit,
    /// Generalized linear mixed model (GLMM)
    #[serde(rename = "glmm")]
    Glmm,
    /// GLMM with probit link
    #[serde(rename = "glmmProbit")]
    GlmmProbit,
    /// GLMM with logit link
    #[serde(rename = "glmmLogit")]
    GlmmLogit,
    /// GLMM with identity link
    #[serde(rename = "glmmIdentity")]
    GlmmIdentity,
    /// GLMM with log link
    #[serde(rename = "glmmLog")]
    GlmmLog,
    /// GLMM with generalized logit link
    #[serde(rename = "glmmGeneralizedLogit")]
    GlmmGeneralizedLogit,
    /// Linear Regression
    #[serde(rename = "linearRegression")]
    LinearRegression,
    /// Logistic Regression
    #[serde(rename = "logisticRegression")]
    LogisticRegression,
    /// Polynomial Regression
    #[serde(rename = "polynomialRegression")]
    PolynomialRegression,
    /// Cox Proportional Hazards
    #[serde(rename = "coxProportionalHazards")]
    CoxProportionalHazards,
    /// Binomial Distribution for Regression
    #[serde(rename = "binomialDistributionRegression")]
    BinomialDistributionRegression,
    /// Multinomial Distribution for Regression
    #[serde(rename = "multinomialDistributionRegression")]
    MultinomialDistributionRegression,
    /// Poisson Regression
    #[serde(rename = "poissonRegression")]
    PoissonRegression,
    /// Negative Binomial Regression
    #[serde(rename = "negativeBinomialRegression")]
    NegativeBinomialRegression,
    /// Zero-cell adjustment with constant
    #[serde(rename = "zeroCellConstant")]
    ZeroCellConstant,
    /// Zero-cell adjustment with continuity correction
    #[serde(rename = "zeroCellContinuityCorrection")]
    ZeroCellContinuityCorrection,
    /// Adjusted analysis
    #[serde(rename = "adjusted")]
    Adjusted,
    /// Interaction term
    #[serde(rename = "interactionTerm")]
    InteractionTerm,
    /// Mantel-Haenszel method
    #[serde(rename = "manteHaenszelMethod")]
    ManteHaenszelMethod,
    /// Meta-analysis
    #[serde(rename = "metaAnalysis")]
    MetaAnalysis,
    /// Inverse variance method
    #[serde(rename = "inverseVariance")]
    InverseVariance,
    /// Peto method
    #[serde(rename = "petoMethod")]
    PetoMethod,
    /// Hartung-Knapp adjustment
    #[serde(rename = "hartungKnapp")]
    HartungKnapp,
    /// Modified Hartung-Knapp adjustment
    #[serde(rename = "modifiedHartungKnapp")]
    ModifiedHartungKnapp,
    /// Fixed-effects
    #[serde(rename = "effectsFixed")]
    EffectsFixed,
    /// Random-effects
    #[serde(rename = "effectsRandom")]
    EffectsRandom,
    /// Chi-square test for homogeneity
    #[serde(rename = "chiSquareTestHomogeneity")]
    ChiSquareTestHomogeneity,
    /// Dersimonian-Laird method
    #[serde(rename = "dersimonianLairdMethod")]
    DersimonianLairdMethod,
    /// Paule-Mandel method
    #[serde(rename = "pauleMandelMethod")]
    PauleMandelMethod,
    /// Restricted Maximum Likelihood method
    #[serde(rename = "restrictedLikelihood")]
    RestrictedLikelihood,
    /// Maximum Likelihood method
    #[serde(rename = "maximumLikelihood")]
    MaximumLikelihood,
    /// Empirical Bayes method
    #[serde(rename = "empiricalBayes")]
    EmpiricalBayes,
    /// Hunter-Schmidt method
    #[serde(rename = "hunterSchmidt")]
    HunterSchmidt,
    /// Sidik-Jonkman method
    #[serde(rename = "sidikJonkman")]
    SidikJonkman,
    /// Hedges method
    #[serde(rename = "hedgesMethod")]
    HedgesMethod,
    /// Dersimonian-Laird method
    #[serde(rename = "tauDersimonianLaird")]
    TauDersimonianLaird,
    /// Paule-Mandel method
    #[serde(rename = "tauPauleMandel")]
    TauPauleMandel,
    /// Restricted Maximum Likelihood method
    #[serde(rename = "tauRestrictedMaximumLikelihood")]
    TauRestrictedMaximumLikelihood,
    /// Maximum Likelihood method
    #[serde(rename = "tauMaximumLikelihood")]
    TauMaximumLikelihood,
    /// Empirical Bayes method
    #[serde(rename = "tauEmpiricalBayes")]
    TauEmpiricalBayes,
    /// Hunter-Schmidt method
    #[serde(rename = "tauHunterSchmidt")]
    TauHunterSchmidt,
    /// Sidik-Jonkman method
    #[serde(rename = "tauSidikJonkman")]
    TauSidikJonkman,
    /// Hedges method
    #[serde(rename = "tauHedges")]
    TauHedges,
    /// Mantel-Haenszel method
    #[serde(rename = "poolMantelHaenzsel")]
    PoolMantelHaenzsel,
    /// Inverse variance method
    #[serde(rename = "poolInverseVariance")]
    PoolInverseVariance,
    /// Peto method
    #[serde(rename = "poolPeto")]
    PoolPeto,
    /// Generalized linear mixed model (GLMM)
    #[serde(rename = "poolGeneralizedLinearMixedModel")]
    PoolGeneralizedLinearMixedModel,
}

/// Type for strand.
///
/// System: <http://hl7.org/fhir/strand-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StrandType {
    /// Watson strand of starting sequence
    #[default]
    #[serde(rename = "watson")]
    Watson,
    /// Crick strand of starting sequence
    #[serde(rename = "crick")]
    Crick,
}

/// Defines the type of structure that a definition is describing.
///
/// System: <http://hl7.org/fhir/structure-definition-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StructureDefinitionKind {
    /// Primitive Data Type
    #[default]
    #[serde(rename = "primitive-type")]
    PrimitiveType,
    /// Complex Data Type
    #[serde(rename = "complex-type")]
    ComplexType,
    /// Resource
    #[serde(rename = "resource")]
    Resource,
    /// Logical
    #[serde(rename = "logical")]
    Logical,
}

/// This is a set of terms for study design characteristics.
///
/// System: <http://hl7.org/fhir/study-design>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StudyDesign {
    /// Interventional research
    #[default]
    #[serde(rename = "SEVCO:01001")]
    Sevco01001,
    /// randomized assignment
    #[serde(rename = "SEVCO:01003")]
    Sevco01003,
    /// simple randomization
    #[serde(rename = "SEVCO:01006")]
    Sevco01006,
    /// stratified randomization
    #[serde(rename = "SEVCO:01007")]
    Sevco01007,
    /// block randomization
    #[serde(rename = "SEVCO:01008")]
    Sevco01008,
    /// adaptive randomization
    #[serde(rename = "SEVCO:01009")]
    Sevco01009,
    /// Non-randomized assignment
    #[serde(rename = "SEVCO:01005")]
    Sevco01005,
    /// Quasi-Randomized assignment
    #[serde(rename = "SEVCO:01004")]
    Sevco01004,
    /// Clinical trial
    #[serde(rename = "SEVCO:01029")]
    Sevco01029,
    /// Pragmatic clinical trial
    #[serde(rename = "SEVCO:01041")]
    Sevco01041,
    /// Expanded Access study
    #[serde(rename = "SEVCO:01038")]
    Sevco01038,
    /// Phase 1 trial
    #[serde(rename = "SEVCO:01030")]
    Sevco01030,
    /// Exploratory investigational new drug study
    #[serde(rename = "SEVCO:01031")]
    Sevco01031,
    /// Phase 1/Phase 2 trial
    #[serde(rename = "SEVCO:01032")]
    Sevco01032,
    /// Phase 2 trial
    #[serde(rename = "SEVCO:01033")]
    Sevco01033,
    /// Phase 2/Phase 3 trial
    #[serde(rename = "SEVCO:01034")]
    Sevco01034,
    /// Phase 3 Trial
    #[serde(rename = "SEVCO:01035")]
    Sevco01035,
    /// Post-marketing study
    #[serde(rename = "SEVCO:01036")]
    Sevco01036,
    /// Observational research
    #[serde(rename = "SEVCO:01002")]
    Sevco01002,
    /// Post-Marketing Surveillance study
    #[serde(rename = "SEVCO:01037")]
    Sevco01037,
    /// Comparative study design
    #[serde(rename = "SEVCO:01010")]
    Sevco01010,
    /// Parallel cohort design
    #[serde(rename = "SEVCO:01011")]
    Sevco01011,
    /// Crossover cohort design
    #[serde(rename = "SEVCO:01012")]
    Sevco01012,
    /// Controlled crossover cohort design
    #[serde(rename = "SEVCO:01024")]
    Sevco01024,
    /// Single-arm crossover design
    #[serde(rename = "SEVCO:01025")]
    Sevco01025,
    /// Case control design
    #[serde(rename = "SEVCO:01013")]
    Sevco01013,
    /// Matching for comparison
    #[serde(rename = "SEVCO:01014")]
    Sevco01014,
    /// Family study design
    #[serde(rename = "SEVCO:01020")]
    Sevco01020,
    /// Twin study design
    #[serde(rename = "SEVCO:01021")]
    Sevco01021,
    /// Cluster as unit of allocation
    #[serde(rename = "SEVCO:01015")]
    Sevco01015,
    /// Non-comparative study design
    #[serde(rename = "SEVCO:01023")]
    Sevco01023,
    /// Uncontrolled cohort design
    #[serde(rename = "SEVCO:01016")]
    Sevco01016,
    /// Case report
    #[serde(rename = "SEVCO:01017")]
    Sevco01017,
    /// Population-based design
    #[serde(rename = "SEVCO:01022")]
    Sevco01022,
    /// Ecological design
    #[serde(rename = "SEVCO:01044")]
    Sevco01044,
    /// Cross sectional data collection
    #[serde(rename = "SEVCO:01027")]
    Sevco01027,
    /// Longitudinal data collection
    #[serde(rename = "SEVCO:01028")]
    Sevco01028,
    /// Time series design
    #[serde(rename = "SEVCO:01018")]
    Sevco01018,
    /// Before and after comparison
    #[serde(rename = "SEVCO:01019")]
    Sevco01019,
    /// Primary data collection
    #[serde(rename = "SEVCO:01045")]
    Sevco01045,
    /// Real world data collection
    #[serde(rename = "SEVCO:01026")]
    Sevco01026,
    /// Real world data collection from healthcare records
    #[serde(rename = "SEVCO:01039")]
    Sevco01039,
    /// Real world data collection from personal health records
    #[serde(rename = "SEVCO:01050")]
    Sevco01050,
    /// Real world data collection from healthcare financing records
    #[serde(rename = "SEVCO:01040")]
    Sevco01040,
    /// Real world data collection from testing procedures
    #[serde(rename = "SEVCO:01048")]
    Sevco01048,
    /// Real world data collection from monitoring procedures
    #[serde(rename = "SEVCO:01046")]
    Sevco01046,
    /// Secondary data collection from prior research
    #[serde(rename = "SEVCO:01049")]
    Sevco01049,
    /// Secondary data collection from a registry
    #[serde(rename = "SEVCO:01042")]
    Sevco01042,
    /// Multisite data collection
    #[serde(rename = "SEVCO:01051")]
    Sevco01051,
    /// Quantitative analysis
    #[serde(rename = "SEVCO:01086")]
    Sevco01086,
    /// Qualitative analysis
    #[serde(rename = "SEVCO:01087")]
    Sevco01087,
    /// Blinding of study participants
    #[serde(rename = "SEVCO:01060")]
    Sevco01060,
    /// Blinding of intervention providers
    #[serde(rename = "SEVCO:01061")]
    Sevco01061,
    /// Blinding of outcome assessors
    #[serde(rename = "SEVCO:01062")]
    Sevco01062,
    /// Blinding of data analysts
    #[serde(rename = "SEVCO:01063")]
    Sevco01063,
    /// Allocation concealment
    #[serde(rename = "SEVCO:01064")]
    Sevco01064,
    /// Multicentric
    #[serde(rename = "SEVCO:01043")]
    Sevco01043,
    /// Includes patient-reported outcome
    #[serde(rename = "SEVCO:01052")]
    Sevco01052,
    /// Includes patient-centered outcome
    #[serde(rename = "SEVCO:01053")]
    Sevco01053,
    /// Includes disease-oriented outcome
    #[serde(rename = "SEVCO:01054")]
    Sevco01054,
    /// Includes process measure
    #[serde(rename = "SEVCO:01085")]
    Sevco01085,
    /// Study Goal
    #[serde(rename = "SEVCO:01089")]
    Sevco01089,
    /// Evaluation Goal
    #[serde(rename = "SEVCO:01096")]
    Sevco01096,
    /// Derivation Goal
    #[serde(rename = "SEVCO:01097")]
    Sevco01097,
    /// Validation Goal
    #[serde(rename = "SEVCO:01098")]
    Sevco01098,
    /// Comparison Goal
    #[serde(rename = "SEVCO:01088")]
    Sevco01088,
    /// Comparative Effectiveness Goal
    #[serde(rename = "SEVCO:01091")]
    Sevco01091,
    /// Comparative Efficacy Goal
    #[serde(rename = "SEVCO:01090")]
    Sevco01090,
    /// Comparative Safety Goal
    #[serde(rename = "SEVCO:01092")]
    Sevco01092,
    /// Equivalence Goal
    #[serde(rename = "SEVCO:01093")]
    Sevco01093,
    /// Non-inferiority Goal
    #[serde(rename = "SEVCO:01094")]
    Sevco01094,
    /// Superiority Goal
    #[serde(rename = "SEVCO:01095")]
    Sevco01095,
}

/// Concepts for how a measure report consumer and receiver coordinate data
/// exchange updates. The choices are snapshot or incremental updates
///
/// System: <http://hl7.org/fhir/CodeSystem/submit-data-update-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubmitDataUpdateType {
    /// Incremental
    #[default]
    #[serde(rename = "incremental")]
    Incremental,
    /// Snapshot
    #[serde(rename = "snapshot")]
    Snapshot,
}

/// The type of notification represented by the status message.
///
/// System: <http://hl7.org/fhir/subscription-notification-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptionNotificationType {
    /// Handshake
    #[default]
    #[serde(rename = "handshake")]
    Handshake,
    /// Heartbeat
    #[serde(rename = "heartbeat")]
    Heartbeat,
    /// Event Notification
    #[serde(rename = "event-notification")]
    EventNotification,
    /// Query Status
    #[serde(rename = "query-status")]
    QueryStatus,
    /// Query Event
    #[serde(rename = "query-event")]
    QueryEvent,
}

/// Codes to represent how much resource content to send in the notification
/// payload.
///
/// System: <http://hl7.org/fhir/subscription-payload-content>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptionPayloadContent {
    /// Empty
    #[default]
    #[serde(rename = "empty")]
    Empty,
    /// Id-only
    #[serde(rename = "id-only")]
    IdOnly,
    /// Full-resource
    #[serde(rename = "full-resource")]
    FullResource,
}

/// This codesystem defines a set of codes that can be used to filter
/// Subscription triggers.
///
/// System: <http://hl7.org/fhir/subscription-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptionStatus {
    /// Requested
    #[default]
    #[serde(rename = "requested")]
    Requested,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Behavior a server can exhibit when a criteria state does not exist (e.g.,
/// state prior to a create or after a delete).
///
/// System: <http://hl7.org/fhir/subscriptiontopic-cr-behavior>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptiontopicCrBehavior {
    /// Test passes
    #[default]
    #[serde(rename = "test-passes")]
    TestPasses,
    /// Test fails
    #[serde(rename = "test-fails")]
    TestFails,
}

/// The type of a numeric quantity measurement.
///
/// System: <http://hl7.org/fhir/substance-amount-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceAmountType {
    /// Average
    #[default]
    #[serde(rename = "Average")]
    Average,
    /// Approximately
    #[serde(rename = "Approximately")]
    Approximately,
    /// Less Than
    #[serde(rename = "LessThan")]
    LessThan,
    /// More Than
    #[serde(rename = "MoreThan")]
    MoreThan,
}

/// SubstanceForm
///
/// System: <http://hl7.org/fhir/substance-form>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceForm {
    /// Salt of substance
    #[default]
    #[serde(rename = "salt")]
    Salt,
    /// Base of substance
    #[serde(rename = "base")]
    Base,
}

/// SubstanceGrade
///
/// System: <http://hl7.org/fhir/substance-grade>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceGrade {
    /// USP/NF United States Pharmacopeia (USP) and the National Formulary (NF)
    #[default]
    #[serde(rename = "USP-NF")]
    UspNf,
    /// European Pharmacopoeia
    #[serde(rename = "Ph.Eur")]
    PhEur,
    /// Japanese Pharmacopoeia
    #[serde(rename = "JP")]
    Jp,
    /// British Pharmacopoeia
    #[serde(rename = "BP")]
    Bp,
    /// Company Standard
    #[serde(rename = "CompanyStandard")]
    CompanyStandard,
}

/// SubstanceNameAuthority
///
/// System: <http://hl7.org/fhir/substance-name-authority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceNameAuthority {
    /// BAN
    #[default]
    #[serde(rename = "BAN")]
    Ban,
    /// COSING
    #[serde(rename = "COSING")]
    Cosing,
    /// Ph.Eur.
    #[serde(rename = "Ph.Eur.")]
    PhEur,
    /// FCC
    #[serde(rename = "FCC")]
    Fcc,
    /// INCI
    #[serde(rename = "INCI")]
    Inci,
    /// INN
    #[serde(rename = "INN")]
    Inn,
    /// JAN
    #[serde(rename = "JAN")]
    Jan,
    /// JECFA
    #[serde(rename = "JECFA")]
    Jecfa,
    /// MARTINDALE
    #[serde(rename = "MARTINDALE")]
    Martindale,
    /// USAN
    #[serde(rename = "USAN")]
    Usan,
    /// USP
    #[serde(rename = "USP")]
    Usp,
    /// PHF
    #[serde(rename = "PHF")]
    Phf,
    /// HAB
    #[serde(rename = "HAB")]
    Hab,
    /// PhF (Pharmacopée française)
    #[serde(rename = "PhF")]
    PhF,
    /// IUIS
    #[serde(rename = "IUIS")]
    Iuis,
}

/// SubstanceNameDomain
///
/// System: <http://hl7.org/fhir/substance-name-domain>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceNameDomain {
    /// Active Ingredient
    #[default]
    #[serde(rename = "ActiveIngredient")]
    ActiveIngredient,
    /// Food Color Additive
    #[serde(rename = "FoodColorAdditive")]
    FoodColorAdditive,
}

/// SubstanceNameType
///
/// System: <http://hl7.org/fhir/substance-name-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceNameType {
    /// systematic
    #[default]
    #[serde(rename = "Systematic")]
    Systematic,
    /// scientific
    #[serde(rename = "Scientific")]
    Scientific,
    /// brand
    #[serde(rename = "Brand")]
    Brand,
}

/// The optical rotation type of a substance.
///
/// System: <http://hl7.org/fhir/substance-optical-activity>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceOpticalActivity {
    /// dextrorotary
    #[default]
    #[serde(rename = "+")]
    Unnamed,
    /// levorotary
    #[serde(rename = "-")]
    Unnamed2,
}

/// The relationship between two substance types.
///
/// System: <http://hl7.org/fhir/substance-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceRelationshipType {
    /// Salt to parent
    #[default]
    #[serde(rename = "Salt")]
    Salt,
    /// Active moiety
    #[serde(rename = "ActiveMoiety")]
    ActiveMoiety,
    /// Starting material for
    #[serde(rename = "StartingMaterial")]
    StartingMaterial,
    /// Polymorph of
    #[serde(rename = "Polymorph")]
    Polymorph,
    /// Impurity of
    #[serde(rename = "Impurity")]
    Impurity,
}

/// SubstanceRepresentationFormat
///
/// System: <http://hl7.org/fhir/substance-representation-format>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceRepresentationFormat {
    /// InChI
    #[default]
    #[serde(rename = "InChI")]
    InChI,
    /// SMILES
    #[serde(rename = "SMILES")]
    Smiles,
    /// MOLFILE
    #[serde(rename = "MOLFILE")]
    Molfile,
    /// CDX
    #[serde(rename = "CDX")]
    Cdx,
    /// SDF
    #[serde(rename = "SDF")]
    Sdf,
    /// PDB
    #[serde(rename = "PDB")]
    Pdb,
    /// mmCIF
    #[serde(rename = "mmCIF")]
    MmCif,
}

/// SubstanceRepresentationType
///
/// System: <http://hl7.org/fhir/substance-representation-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceRepresentationType {
    /// systematic
    #[default]
    #[serde(rename = "Systematic")]
    Systematic,
    /// scientific
    #[serde(rename = "Scientific")]
    Scientific,
    /// brand
    #[serde(rename = "Brand")]
    Brand,
}

/// SubstanceSourceMaterialGenus
///
/// System: <http://hl7.org/fhir/substance-source-material-genus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceSourceMaterialGenus {
    /// Mycobacterium
    #[default]
    #[serde(rename = "Mycobacterium")]
    Mycobacterium,
    /// Influenza A virus
    #[serde(rename = "InfluenzavirusA")]
    InfluenzavirusA,
    /// Ginkgo
    #[serde(rename = "Ginkgo")]
    Ginkgo,
}

/// SubstanceSourceMaterialPart
///
/// System: <http://hl7.org/fhir/substance-source-material-part>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceSourceMaterialPart {
    /// animal
    #[default]
    #[serde(rename = "Animal")]
    Animal,
    /// plant
    #[serde(rename = "Plant")]
    Plant,
    /// mineral
    #[serde(rename = "Mineral")]
    Mineral,
}

/// SubstanceSourceMaterialSpecies
///
/// System: <http://hl7.org/fhir/substance-source-material-species>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceSourceMaterialSpecies {
    /// Ginkgo biloba
    #[default]
    #[serde(rename = "GinkgoBiloba")]
    GinkgoBiloba,
    /// Olea europaea
    #[serde(rename = "OleaEuropaea")]
    OleaEuropaea,
}

/// SubstanceSourceMaterialType
///
/// System: <http://hl7.org/fhir/substance-source-material-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceSourceMaterialType {
    /// animal
    #[default]
    #[serde(rename = "Animal")]
    Animal,
    /// plant
    #[serde(rename = "Plant")]
    Plant,
    /// mineral
    #[serde(rename = "Mineral")]
    Mineral,
}

/// A code to indicate if the substance is actively used.
///
/// System: <http://hl7.org/fhir/substance-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The stereochemistry type of a substance.
///
/// System: <http://hl7.org/fhir/substance-stereochemistry>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceStereochemistry {
    /// constitutional isomer
    #[default]
    #[serde(rename = "ConstitutionalIsomer")]
    ConstitutionalIsomer,
    /// stereoisomer
    #[serde(rename = "Stereoisomer")]
    Stereoisomer,
    /// enantiomer
    #[serde(rename = "Enantiomer")]
    Enantiomer,
}

/// SubstanceStructureTechnique
///
/// System: <http://hl7.org/fhir/substance-structure-technique>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceStructureTechnique {
    /// X-ray
    #[default]
    #[serde(rename = "X-Ray")]
    XRay,
    /// HPLC
    #[serde(rename = "HPLC")]
    Hplc,
    /// NMR
    #[serde(rename = "NMR")]
    Nmr,
    /// Peptide mapping
    #[serde(rename = "PeptideMapping")]
    PeptideMapping,
    /// Ligand binding assay
    #[serde(rename = "LigandBindingAssay")]
    LigandBindingAssay,
}

/// SubstanceWeightMethod
///
/// System: <http://hl7.org/fhir/substance-weight-method>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceWeightMethod {
    /// SDS-PAGE (sodium dodecyl sulfate-polyacrylamide gel electrophoresis)
    #[default]
    #[serde(rename = "SDS-PAGE")]
    SdsPage,
    /// calculated
    #[serde(rename = "Calculated")]
    Calculated,
    /// light scattering
    #[serde(rename = "LighScattering")]
    LighScattering,
    /// viscosity
    #[serde(rename = "Viscosity")]
    Viscosity,
    /// gel permeation centrifugation
    #[serde(rename = "GelPermeationCentrifugation")]
    GelPermeationCentrifugation,
    /// End-group analysis
    #[serde(rename = "End-groupAnalysis")]
    EndGroupAnalysis,
    /// End-group titration
    #[serde(rename = "End-groupTitration")]
    EndGroupTitration,
    /// Size-exclusion chromatography
    #[serde(rename = "Size-ExclusionChromatography")]
    SizeExclusionChromatography,
}

/// SubstanceWeightType
///
/// System: <http://hl7.org/fhir/substance-weight-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceWeightType {
    /// exact
    #[default]
    #[serde(rename = "Exact")]
    Exact,
    /// number average
    #[serde(rename = "Average")]
    Average,
    /// weight average
    #[serde(rename = "WeightAverage")]
    WeightAverage,
}

/// Status of the supply delivery.
///
/// System: <http://hl7.org/fhir/supplydelivery-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplydeliveryStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Delivered
    #[serde(rename = "completed")]
    Completed,
    /// Abandoned
    #[serde(rename = "abandoned")]
    Abandoned,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Status of the supply delivery.
///
/// System: <http://hl7.org/fhir/supplydelivery-supplyitemtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplydeliverySupplyitemtype {
    /// Medication
    #[default]
    #[serde(rename = "medication")]
    Medication,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Biologically Derived Product
    #[serde(rename = "biologicallyderivedproduct")]
    Biologicallyderivedproduct,
}

/// Status of the supply request.
///
/// System: <http://hl7.org/fhir/supplyrequest-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplyrequestStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// A species for which a medicinal product is intended.
///
/// System: <http://hl7.org/fhir/target-species>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TargetSpecies {
    /// Dove
    #[default]
    #[serde(rename = "100000108874")]
    N100000108874,
    /// Quail
    #[serde(rename = "100000108875")]
    N100000108875,
    /// Snipe
    #[serde(rename = "100000108876")]
    N100000108876,
    /// Sparrow
    #[serde(rename = "100000108877")]
    N100000108877,
    /// Starling
    #[serde(rename = "100000108878")]
    N100000108878,
    /// Swan
    #[serde(rename = "100000108879")]
    N100000108879,
    /// Turkey hen
    #[serde(rename = "100000108880")]
    N100000108880,
    /// Turkey cock
    #[serde(rename = "100000108881")]
    N100000108881,
    /// Turkeys
    #[serde(rename = "100000108882")]
    N100000108882,
    /// Turkey for reproduction
    #[serde(rename = "100000108883")]
    N100000108883,
    /// Poult
    #[serde(rename = "100000108884")]
    N100000108884,
    /// Turtle dove
    #[serde(rename = "100000108885")]
    N100000108885,
    /// Bison
    #[serde(rename = "100000108886")]
    N100000108886,
    /// Buffalo
    #[serde(rename = "100000108887")]
    N100000108887,
    /// Cows
    #[serde(rename = "100000108888")]
    N100000108888,
    /// Cow for reproduction
    #[serde(rename = "100000108889")]
    N100000108889,
    /// Bull for reproduction
    #[serde(rename = "100000108890")]
    N100000108890,
    /// Ox
    #[serde(rename = "100000108891")]
    N100000108891,
    /// Bullock
    #[serde(rename = "100000108892")]
    N100000108892,
    /// Cattle
    #[serde(rename = "100000108893")]
    N100000108893,
    /// Beef cattle
    #[serde(rename = "100000108894")]
    N100000108894,
    /// Dairy cattle
    #[serde(rename = "100000108895")]
    N100000108895,
    /// Dry cow
    #[serde(rename = "100000108896")]
    N100000108896,
    /// Bull
    #[serde(rename = "100000108897")]
    N100000108897,
    /// Lactating cow
    #[serde(rename = "100000108898")]
    N100000108898,
    /// All other food producing species
    #[serde(rename = "100000108899")]
    N100000108899,
    /// All non-food producing species
    #[serde(rename = "100000108900")]
    N100000108900,
    /// All species
    #[serde(rename = "100000108901")]
    N100000108901,
    /// Carnivores
    #[serde(rename = "100000108902")]
    N100000108902,
    /// Domestic animals
    #[serde(rename = "100000108903")]
    N100000108903,
    /// Fur animals
    #[serde(rename = "100000108904")]
    N100000108904,
    /// Game animals
    #[serde(rename = "100000108905")]
    N100000108905,
    /// Laboratory animals
    #[serde(rename = "100000108906")]
    N100000108906,
    /// Major species
    #[serde(rename = "100000108907")]
    N100000108907,
    /// Minor species
    #[serde(rename = "100000108908")]
    N100000108908,
    /// Ruminant
    #[serde(rename = "100000108909")]
    N100000108909,
    /// Ruminant and porcine
    #[serde(rename = "100000108910")]
    N100000108910,
    /// Small animals
    #[serde(rename = "100000108911")]
    N100000108911,
    /// Crocodile
    #[serde(rename = "100000108912")]
    N100000108912,
    /// Frog
    #[serde(rename = "100000108913")]
    N100000108913,
    /// Iguana
    #[serde(rename = "100000108914")]
    N100000108914,
    /// Lizard
    #[serde(rename = "100000108915")]
    N100000108915,
    /// Amphibians
    #[serde(rename = "100000108916")]
    N100000108916,
    /// Reptiles
    #[serde(rename = "100000108917")]
    N100000108917,
    /// Reptiles for production
    #[serde(rename = "100000108918")]
    N100000108918,
    /// Snake
    #[serde(rename = "100000108919")]
    N100000108919,
    /// Tortoise
    #[serde(rename = "100000108920")]
    N100000108920,
    /// Turtle
    #[serde(rename = "100000108921")]
    N100000108921,
    /// Honey bees
    #[serde(rename = "100000108922")]
    N100000108922,
    /// Crustacean
    #[serde(rename = "100000108923")]
    N100000108923,
    /// Spider
    #[serde(rename = "100000108924")]
    N100000108924,
    /// Indian hen
    #[serde(rename = "100000108925")]
    N100000108925,
    /// African Goshawk
    #[serde(rename = "100000108926")]
    N100000108926,
    /// Black Kite
    #[serde(rename = "100000108927")]
    N100000108927,
    /// Budgerigar
    #[serde(rename = "100000108928")]
    N100000108928,
    /// Bustard
    #[serde(rename = "100000108929")]
    N100000108929,
    /// Buzzard
    #[serde(rename = "100000108930")]
    N100000108930,
    /// Hen
    #[serde(rename = "100000108931")]
    N100000108931,
    /// Layer hen
    #[serde(rename = "100000108932")]
    N100000108932,
    /// Cock
    #[serde(rename = "100000108933")]
    N100000108933,
    /// Broiler
    #[serde(rename = "100000108934")]
    N100000108934,
    /// Chickens
    #[serde(rename = "100000108935")]
    N100000108935,
    /// Chicken embryonated eggs
    #[serde(rename = "100000108936")]
    N100000108936,
    /// Chicken for reproduction
    #[serde(rename = "100000108937")]
    N100000108937,
    /// Replacement chick
    #[serde(rename = "100000108938")]
    N100000108938,
    /// Chick
    #[serde(rename = "100000108939")]
    N100000108939,
    /// Pullet
    #[serde(rename = "100000108940")]
    N100000108940,
    /// Cockatiel
    #[serde(rename = "100000108941")]
    N100000108941,
    /// Cockatoo
    #[serde(rename = "100000108942")]
    N100000108942,
    /// Common canary
    #[serde(rename = "100000108943")]
    N100000108943,
    /// Crow
    #[serde(rename = "100000108944")]
    N100000108944,
    /// Duck
    #[serde(rename = "100000108945")]
    N100000108945,
    /// Duck broiler
    #[serde(rename = "100000108946")]
    N100000108946,
    /// Duckling
    #[serde(rename = "100000108947")]
    N100000108947,
    /// Eagle
    #[serde(rename = "100000108948")]
    N100000108948,
    /// Emu
    #[serde(rename = "100000108949")]
    N100000108949,
    /// Goose
    #[serde(rename = "100000108950")]
    N100000108950,
    /// Guinea fowl
    #[serde(rename = "100000108951")]
    N100000108951,
    /// Kestrel
    #[serde(rename = "100000108952")]
    N100000108952,
    /// Kite
    #[serde(rename = "100000108953")]
    N100000108953,
    /// Macaw
    #[serde(rename = "100000108954")]
    N100000108954,
    /// Ostrich
    #[serde(rename = "100000108955")]
    N100000108955,
    /// Birds
    #[serde(rename = "100000108956")]
    N100000108956,
    /// Fowls
    #[serde(rename = "100000108957")]
    N100000108957,
    /// Finch
    #[serde(rename = "100000108958")]
    N100000108958,
    /// Galliformes
    #[serde(rename = "100000108959")]
    N100000108959,
    /// Game birds
    #[serde(rename = "100000108960")]
    N100000108960,
    /// Pekin duck
    #[serde(rename = "100000108961")]
    N100000108961,
    /// Ornamental birds
    #[serde(rename = "100000108962")]
    N100000108962,
    /// Poultry
    #[serde(rename = "100000108963")]
    N100000108963,
    /// Owl
    #[serde(rename = "100000108964")]
    N100000108964,
    /// Parakeet
    #[serde(rename = "100000108965")]
    N100000108965,
    /// Parrot
    #[serde(rename = "100000108966")]
    N100000108966,
    /// Partridge
    #[serde(rename = "100000108967")]
    N100000108967,
    /// Peregrine Falcon
    #[serde(rename = "100000108968")]
    N100000108968,
    /// Pheasants
    #[serde(rename = "100000108969")]
    N100000108969,
    /// Carrier pigeon
    #[serde(rename = "100000108970")]
    N100000108970,
    /// Newborn calves
    #[serde(rename = "100000108971")]
    N100000108971,
    /// Pregnant cow
    #[serde(rename = "100000108972")]
    N100000108972,
    /// Pregnant heifer
    #[serde(rename = "100000108973")]
    N100000108973,
    /// Pre-ruminant cattle
    #[serde(rename = "100000108974")]
    N100000108974,
    /// Ruminant cattle
    #[serde(rename = "100000108975")]
    N100000108975,
    /// Wild cattle
    #[serde(rename = "100000108976")]
    N100000108976,
    /// Calf
    #[serde(rename = "100000108977")]
    N100000108977,
    /// Heifers
    #[serde(rename = "100000108978")]
    N100000108978,
    /// Other Bovids
    #[serde(rename = "100000108979")]
    N100000108979,
    /// Alpaca
    #[serde(rename = "100000108980")]
    N100000108980,
    /// Camel
    #[serde(rename = "100000108981")]
    N100000108981,
    /// Llama
    #[serde(rename = "100000108982")]
    N100000108982,
    /// Other Camelids
    #[serde(rename = "100000108983")]
    N100000108983,
    /// Bitch
    #[serde(rename = "100000108984")]
    N100000108984,
    /// Bitch for reproduction
    #[serde(rename = "100000108985")]
    N100000108985,
    /// Adult male dog
    #[serde(rename = "100000108986")]
    N100000108986,
    /// Adult male dog for reproduction
    #[serde(rename = "100000108987")]
    N100000108987,
    /// Dogs
    #[serde(rename = "100000108988")]
    N100000108988,
    /// Lactating bitch
    #[serde(rename = "100000108989")]
    N100000108989,
    /// Large dog
    #[serde(rename = "100000108990")]
    N100000108990,
    /// Medium dog
    #[serde(rename = "100000108991")]
    N100000108991,
    /// Pregnant bitch
    #[serde(rename = "100000108992")]
    N100000108992,
    /// Small dog
    #[serde(rename = "100000108993")]
    N100000108993,
    /// Very large dog
    #[serde(rename = "100000108994")]
    N100000108994,
    /// Very small dog
    #[serde(rename = "100000108995")]
    N100000108995,
    /// Puppy
    #[serde(rename = "100000108996")]
    N100000108996,
    /// Foxes
    #[serde(rename = "100000108997")]
    N100000108997,
    /// Jackal
    #[serde(rename = "100000108998")]
    N100000108998,
    /// Other Canids
    #[serde(rename = "100000108999")]
    N100000108999,
    /// Raccoon dogs
    #[serde(rename = "100000109000")]
    N100000109000,
    /// Wolf
    #[serde(rename = "100000109001")]
    N100000109001,
    /// Chamois
    #[serde(rename = "100000109002")]
    N100000109002,
    /// Other Caprines
    #[serde(rename = "100000109003")]
    N100000109003,
    /// Adult female goat
    #[serde(rename = "100000109004")]
    N100000109004,
    /// Adult male goat
    #[serde(rename = "100000109005")]
    N100000109005,
    /// Dry adult female goat
    #[serde(rename = "100000109006")]
    N100000109006,
    /// Goats
    #[serde(rename = "100000109007")]
    N100000109007,
    /// Lactating adult female goat
    #[serde(rename = "100000109008")]
    N100000109008,
    /// Pregnant adult female goat
    #[serde(rename = "100000109009")]
    N100000109009,
    /// Pre-ruminant goat
    #[serde(rename = "100000109010")]
    N100000109010,
    /// Ruminant goat
    #[serde(rename = "100000109011")]
    N100000109011,
    /// Wild goat
    #[serde(rename = "100000109012")]
    N100000109012,
    /// Kid
    #[serde(rename = "100000109013")]
    N100000109013,
    /// Other Ovids
    #[serde(rename = "100000109014")]
    N100000109014,
    /// Ewe
    #[serde(rename = "100000109015")]
    N100000109015,
    /// Ram
    #[serde(rename = "100000109016")]
    N100000109016,
    /// Dry ewe
    #[serde(rename = "100000109017")]
    N100000109017,
    /// Lactating ewe
    #[serde(rename = "100000109018")]
    N100000109018,
    /// Pregnant ewe
    #[serde(rename = "100000109019")]
    N100000109019,
    /// Pre-ruminant sheep
    #[serde(rename = "100000109020")]
    N100000109020,
    /// Ruminant sheep
    #[serde(rename = "100000109021")]
    N100000109021,
    /// Sheep
    #[serde(rename = "100000109022")]
    N100000109022,
    /// Sheep for meat production
    #[serde(rename = "100000109023")]
    N100000109023,
    /// Dairy sheep
    #[serde(rename = "100000109024")]
    N100000109024,
    /// Wild sheep
    #[serde(rename = "100000109025")]
    N100000109025,
    /// Lamb
    #[serde(rename = "100000109026")]
    N100000109026,
    /// Elk
    #[serde(rename = "100000109027")]
    N100000109027,
    /// Fallow deer
    #[serde(rename = "100000109028")]
    N100000109028,
    /// Moose
    #[serde(rename = "100000109029")]
    N100000109029,
    /// Antelope
    #[serde(rename = "100000109030")]
    N100000109030,
    /// Cervid
    #[serde(rename = "100000109031")]
    N100000109031,
    /// Deer
    #[serde(rename = "100000109032")]
    N100000109032,
    /// Other Deer
    #[serde(rename = "100000109033")]
    N100000109033,
    /// Red deer
    #[serde(rename = "100000109034")]
    N100000109034,
    /// Reindeer
    #[serde(rename = "100000109035")]
    N100000109035,
    /// Roe deer
    #[serde(rename = "100000109036")]
    N100000109036,
    /// Donkey
    #[serde(rename = "100000109037")]
    N100000109037,
    /// Mare
    #[serde(rename = "100000109038")]
    N100000109038,
    /// Gelding
    #[serde(rename = "100000109039")]
    N100000109039,
    /// Horses
    #[serde(rename = "100000109040")]
    N100000109040,
    /// Stallion
    #[serde(rename = "100000109041")]
    N100000109041,
    /// Pony
    #[serde(rename = "100000109042")]
    N100000109042,
    /// Pregnant mare
    #[serde(rename = "100000109043")]
    N100000109043,
    /// Non food-producing horse
    #[serde(rename = "100000109044")]
    N100000109044,
    /// Suckling colt
    #[serde(rename = "100000109045")]
    N100000109045,
    /// Colt
    #[serde(rename = "100000109046")]
    N100000109046,
    /// Mule
    #[serde(rename = "100000109047")]
    N100000109047,
    /// Equid
    #[serde(rename = "100000109048")]
    N100000109048,
    /// Female equid
    #[serde(rename = "100000109049")]
    N100000109049,
    /// Other Equids
    #[serde(rename = "100000109050")]
    N100000109050,
    /// Zebra
    #[serde(rename = "100000109051")]
    N100000109051,
    /// Bobcat
    #[serde(rename = "100000109052")]
    N100000109052,
    /// Adult female cat
    #[serde(rename = "100000109053")]
    N100000109053,
    /// Adult female cat for reproduction
    #[serde(rename = "100000109054")]
    N100000109054,
    /// Adult male cat
    #[serde(rename = "100000109055")]
    N100000109055,
    /// Cats
    #[serde(rename = "100000109056")]
    N100000109056,
    /// Lactating cat
    #[serde(rename = "100000109057")]
    N100000109057,
    /// Large cat
    #[serde(rename = "100000109058")]
    N100000109058,
    /// Medium cat
    #[serde(rename = "100000109059")]
    N100000109059,
    /// Pregnant cat
    #[serde(rename = "100000109060")]
    N100000109060,
    /// Small cat
    #[serde(rename = "100000109061")]
    N100000109061,
    /// Kitten
    #[serde(rename = "100000109062")]
    N100000109062,
    /// Cougar
    #[serde(rename = "100000109063")]
    N100000109063,
    /// Jaguar
    #[serde(rename = "100000109064")]
    N100000109064,
    /// Leopard
    #[serde(rename = "100000109065")]
    N100000109065,
    /// Lion
    #[serde(rename = "100000109066")]
    N100000109066,
    /// Lynx
    #[serde(rename = "100000109067")]
    N100000109067,
    /// Other Felids
    #[serde(rename = "100000109068")]
    N100000109068,
    /// Tiger
    #[serde(rename = "100000109069")]
    N100000109069,
    /// Tsushima wild cat
    #[serde(rename = "100000109070")]
    N100000109070,
    /// Other Leporids
    #[serde(rename = "100000109071")]
    N100000109071,
    /// Hare
    #[serde(rename = "100000109072")]
    N100000109072,
    /// Female rabbit for reproduction
    #[serde(rename = "100000109073")]
    N100000109073,
}

/// Codes indicating the type of action that is expected to be performed
///
/// System: <http://hl7.org/fhir/CodeSystem/task-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TaskCode {
    /// Activate/approve the focal resource
    #[default]
    #[serde(rename = "approve")]
    Approve,
    /// Fulfill the focal request
    #[serde(rename = "fulfill")]
    Fulfill,
    /// Instantiate the focal definition
    #[serde(rename = "instantiate")]
    Instantiate,
    /// Mark the focal resource as no longer active
    #[serde(rename = "abort")]
    Abort,
    /// Replace the focal resource with the input resource
    #[serde(rename = "replace")]
    Replace,
    /// Change the focal resource
    #[serde(rename = "change")]
    Change,
    /// Suspend the focal resource
    #[serde(rename = "suspend")]
    Suspend,
    /// Re-activate the focal resource
    #[serde(rename = "resume")]
    Resume,
}

/// Distinguishes whether the task is a proposal, plan or full order.
///
/// System: <http://hl7.org/fhir/task-intent>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TaskIntent {
    /// Unknown
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    /// proposal
    #[serde(rename = "proposal")]
    Proposal,
    /// plan
    #[serde(rename = "plan")]
    Plan,
    /// order
    #[serde(rename = "order")]
    Order,
    /// original-order
    #[serde(rename = "original-order")]
    OriginalOrder,
    /// reflex-order
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    /// filler-order
    #[serde(rename = "filler-order")]
    FillerOrder,
    /// instance-order
    #[serde(rename = "instance-order")]
    InstanceOrder,
    /// option
    #[serde(rename = "option")]
    Option,
}

/// The current status of the task.
///
/// System: <http://hl7.org/fhir/task-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TaskStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "received")]
    Received,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Ready
    #[serde(rename = "ready")]
    Ready,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The current status reason of the task.
///
/// System: <http://hl7.org/fhir/task-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TaskStatusReason {
    /// Missing
    #[default]
    #[serde(rename = "missing")]
    Missing,
    /// Misidentified
    #[serde(rename = "misidentified")]
    Misidentified,
    /// Equipment-issue
    #[serde(rename = "equipment-issue")]
    EquipmentIssue,
    /// Environmental-issue
    #[serde(rename = "environmental-issue")]
    EnvironmentalIssue,
    /// Personnel-issue
    #[serde(rename = "personnel-issue")]
    PersonnelIssue,
}

/// The expectation of whether the test must pass for the system to be
/// considered conformant with the artifact.
///
/// System: <http://hl7.org/fhir/testscript-scope-conformance-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TestscriptScopeConformanceCodes {
    /// Required
    #[default]
    #[serde(rename = "required")]
    Required,
    /// Optional
    #[serde(rename = "optional")]
    Optional,
    /// Strict
    #[serde(rename = "strict")]
    Strict,
}

/// The phase of testing for this artifact.
///
/// System: <http://hl7.org/fhir/testscript-scope-phase-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TestscriptScopePhaseCodes {
    /// Unit
    #[default]
    #[serde(rename = "unit")]
    Unit,
    /// Integration
    #[serde(rename = "integration")]
    Integration,
    /// Production
    #[serde(rename = "production")]
    Production,
}

/// Classification of relationship between a therapy and a contraindication or
/// an indication.
///
/// System: <http://hl7.org/fhir/therapy-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TherapyRelationshipType {
    /// Only contraindicated if the other therapy is given
    #[default]
    #[serde(rename = "contraindicated-only-with")]
    ContraindicatedOnlyWith,
    /// Contraindicated unless the other therapy is given
    #[serde(rename = "contraindicated-except-with")]
    ContraindicatedExceptWith,
    /// Indicated only when the other therapy is given (co-occurrent)
    #[serde(rename = "indicated-only-with")]
    IndicatedOnlyWith,
    /// Indicated except when the other therapy is given
    #[serde(rename = "indicated-except-with")]
    IndicatedExceptWith,
    /// Indicated only if the other therapy is planned to be given afterwards
    /// (prep)
    #[serde(rename = "indicated-only-after")]
    IndicatedOnlyAfter,
    /// Indicated only if the other therapy was given before (follow-up)
    #[serde(rename = "indicated-only-before")]
    IndicatedOnlyBefore,
    /// Indicated to replace the other therapy
    #[serde(rename = "replace-other-therapy")]
    ReplaceOtherTherapy,
    /// Indicated to replace the other contraindicated therapy
    #[serde(rename = "replace-other-therapy-contraindicated")]
    ReplaceOtherTherapyContraindicated,
    /// Indicated to replace the other therapy not well tolerated by patient
    #[serde(rename = "replace-other-therapy-not-tolerated")]
    ReplaceOtherTherapyNotTolerated,
    /// Indicated to replace the other therapy not effective on patient
    #[serde(rename = "replace-other-therapy-not-effective")]
    ReplaceOtherTherapyNotEffective,
}

/// Code for a known / defined timing pattern.
///
/// System: <http://terminology.hl7.org/CodeSystem/timing-abbreviation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TimingAbbreviation {
    /// Continuous (frequency)
    #[default]
    #[serde(rename = "C")]
    C,
    /// BID
    #[serde(rename = "BID")]
    Bid,
    /// TID
    #[serde(rename = "TID")]
    Tid,
    /// QID
    #[serde(rename = "QID")]
    Qid,
    /// AM
    #[serde(rename = "AM")]
    Am,
    /// PM
    #[serde(rename = "PM")]
    Pm,
    /// QD
    #[serde(rename = "QD")]
    Qd,
    /// QOD
    #[serde(rename = "QOD")]
    Qod,
    /// every hour
    #[serde(rename = "Q1H")]
    Q1H,
    /// every 2 hours
    #[serde(rename = "Q2H")]
    Q2H,
    /// every 3 hours
    #[serde(rename = "Q3H")]
    Q3H,
    /// Q4H
    #[serde(rename = "Q4H")]
    Q4H,
    /// Q6H
    #[serde(rename = "Q6H")]
    Q6H,
    /// every 8 hours
    #[serde(rename = "Q8H")]
    Q8H,
    /// at bedtime
    #[serde(rename = "BED")]
    Bed,
    /// weekly
    #[serde(rename = "WK")]
    Wk,
    /// monthly
    #[serde(rename = "MO")]
    Mo,
}

/// Used to express the reason and specific aspect for the variant title, such
/// as language and specific language.
///
/// System: <http://hl7.org/fhir/title-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TitleType {
    /// Primary title
    #[default]
    #[serde(rename = "primary")]
    Primary,
    /// Official title
    #[serde(rename = "official")]
    Official,
    /// Scientific title
    #[serde(rename = "scientific")]
    Scientific,
    /// Plain language title
    #[serde(rename = "plain-language")]
    PlainLanguage,
    /// Subtitle
    #[serde(rename = "subtitle")]
    Subtitle,
    /// Short title
    #[serde(rename = "short-title")]
    ShortTitle,
    /// Acronym
    #[serde(rename = "acronym")]
    Acronym,
    /// Different text in an earlier version
    #[serde(rename = "earlier-title")]
    EarlierTitle,
    /// Different language
    #[serde(rename = "language")]
    Language,
    /// Different language derived from autotranslation
    #[serde(rename = "autotranslated")]
    Autotranslated,
    /// Human use
    #[serde(rename = "human-use")]
    HumanUse,
    /// Machine use
    #[serde(rename = "machine-use")]
    MachineUse,
    /// Different text for the same object with a different identifier
    #[serde(rename = "duplicate-uid")]
    DuplicateUid,
}

/// Codes indicating the type of action that is expected to be performed
///
/// System: <http://hl7.org/fhir/CodeSystem/transport-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TransportCode {
    /// Activate/approve the focal resource
    #[default]
    #[serde(rename = "approve")]
    Approve,
    /// Fulfill the focal request
    #[serde(rename = "fulfill")]
    Fulfill,
    /// Instantiate the focal definition
    #[serde(rename = "instantiate")]
    Instantiate,
    /// Mark the focal resource as no longer active
    #[serde(rename = "abort")]
    Abort,
    /// Replace the focal resource with the input resource
    #[serde(rename = "replace")]
    Replace,
    /// Change the focal resource
    #[serde(rename = "change")]
    Change,
    /// Suspend the focal resource
    #[serde(rename = "suspend")]
    Suspend,
    /// Re-activate the focal resource
    #[serde(rename = "resume")]
    Resume,
}

/// Distinguishes whether the transport is a proposal, plan or full order.
///
/// System: <http://hl7.org/fhir/transport-intent>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TransportIntent {
    /// Unknown
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    /// proposal
    #[serde(rename = "proposal")]
    Proposal,
    /// plan
    #[serde(rename = "plan")]
    Plan,
    /// order
    #[serde(rename = "order")]
    Order,
    /// original-order
    #[serde(rename = "original-order")]
    OriginalOrder,
    /// reflex-order
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    /// filler-order
    #[serde(rename = "filler-order")]
    FillerOrder,
    /// instance-order
    #[serde(rename = "instance-order")]
    InstanceOrder,
    /// option
    #[serde(rename = "option")]
    Option,
}

/// Status of transport.
///
/// System: <http://hl7.org/fhir/transport-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TransportStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Abandoned
    #[serde(rename = "abandoned")]
    Abandoned,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Status of transport.
///
/// System: <http://hl7.org/fhir/transport-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TransportStatusReason {
    /// Declined by patient
    #[default]
    #[serde(rename = "declined-by-patient")]
    DeclinedByPatient,
    /// Refused by recipient
    #[serde(rename = "refused-by-recipient")]
    RefusedByRecipient,
    /// Patient not available
    #[serde(rename = "patient-not-available")]
    PatientNotAvailable,
    /// Specimen not available
    #[serde(rename = "specimen-not-available")]
    SpecimenNotAvailable,
    /// Wrong request data
    #[serde(rename = "wrong-request-data")]
    WrongRequestData,
    /// In route accident
    #[serde(rename = "in-route-accident")]
    InRouteAccident,
    /// Request not acknowledged
    #[serde(rename = "request-not-acknowledged")]
    RequestNotAcknowledged,
}

/// The type of trigger.
///
/// System: <http://hl7.org/fhir/trigger-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TriggerType {
    /// Named Event
    #[default]
    #[serde(rename = "named-event")]
    NamedEvent,
    /// Periodic
    #[serde(rename = "periodic")]
    Periodic,
    /// Data Changed
    #[serde(rename = "data-changed")]
    DataChanged,
    /// Data Added
    #[serde(rename = "data-added")]
    DataAdded,
    /// Data Updated
    #[serde(rename = "data-modified")]
    DataModified,
    /// Data Removed
    #[serde(rename = "data-removed")]
    DataRemoved,
    /// Data Accessed
    #[serde(rename = "data-accessed")]
    DataAccessed,
    /// Data Access Ended
    #[serde(rename = "data-access-ended")]
    DataAccessEnded,
}

/// How a type relates to its baseDefinition.
///
/// System: <http://hl7.org/fhir/type-derivation-rule>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TypeDerivationRule {
    /// Specialization
    #[default]
    #[serde(rename = "specialization")]
    Specialization,
    /// Constraint
    #[serde(rename = "constraint")]
    Constraint,
}

/// Codes to identify how UDI data was entered.
///
/// System: <http://hl7.org/fhir/udi-entry-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UdiEntryType {
    /// Barcode
    #[default]
    #[serde(rename = "barcode")]
    Barcode,
    /// RFID
    #[serde(rename = "rfid")]
    Rfid,
    /// Manual
    #[serde(rename = "manual")]
    Manual,
    /// Card
    #[serde(rename = "card")]
    Card,
    /// Self Reported
    #[serde(rename = "self-reported")]
    SelfReported,
    /// Electronic Transmission
    #[serde(rename = "electronic-transmission")]
    ElectronicTransmission,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// A categorisation for a frequency of occurence of an undesirable effect.
///
/// System: <http://hl7.org/fhir/undesirable-effect-frequency>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UndesirableEffectFrequency {
    /// Common
    #[default]
    #[serde(rename = "Common")]
    Common,
    /// Uncommon
    #[serde(rename = "Uncommon")]
    Uncommon,
    /// Rare
    #[serde(rename = "Rare")]
    Rare,
}

/// The presentation type in which an administrable medicinal product is given
/// to a patient.
///
/// System: <http://hl7.org/fhir/unit-of-presentation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UnitOfPresentation {
    /// Barrel
    #[default]
    #[serde(rename = "200000002108")]
    N200000002108,
    /// Blister
    #[serde(rename = "200000002109")]
    N200000002109,
    /// Block
    #[serde(rename = "200000002110")]
    N200000002110,
    /// Bottle
    #[serde(rename = "200000002111")]
    N200000002111,
    /// Cachet
    #[serde(rename = "200000002112")]
    N200000002112,
    /// Capsule
    #[serde(rename = "200000002113")]
    N200000002113,
    /// Cartridge
    #[serde(rename = "200000002114")]
    N200000002114,
    /// Collar
    #[serde(rename = "200000002115")]
    N200000002115,
    /// Container
    #[serde(rename = "200000002116")]
    N200000002116,
    /// Cup
    #[serde(rename = "200000002117")]
    N200000002117,
    /// Cylinder
    #[serde(rename = "200000002118")]
    N200000002118,
    /// Dart
    #[serde(rename = "200000002119")]
    N200000002119,
    /// Dressing
    #[serde(rename = "200000002120")]
    N200000002120,
    /// Drop
    #[serde(rename = "200000002121")]
    N200000002121,
    /// Film
    #[serde(rename = "200000002122")]
    N200000002122,
    /// Chewing gum
    #[serde(rename = "200000002123")]
    N200000002123,
    /// Implant
    #[serde(rename = "200000002124")]
    N200000002124,
    /// Inhaler
    #[serde(rename = "200000002125")]
    N200000002125,
    /// Insert
    #[serde(rename = "200000002126")]
    N200000002126,
    /// Jar
    #[serde(rename = "200000002127")]
    N200000002127,
    /// Lozenge
    #[serde(rename = "200000002128")]
    N200000002128,
    /// Lyophilisate
    #[serde(rename = "200000002129")]
    N200000002129,
    /// Matrix
    #[serde(rename = "200000002130")]
    N200000002130,
    /// Pad
    #[serde(rename = "200000002131")]
    N200000002131,
    /// Paper
    #[serde(rename = "200000002132")]
    N200000002132,
    /// Pastille
    #[serde(rename = "200000002133")]
    N200000002133,
    /// Patch
    #[serde(rename = "200000002134")]
    N200000002134,
    /// Pen
    #[serde(rename = "200000002135")]
    N200000002135,
    /// Pendant
    #[serde(rename = "200000002136")]
    N200000002136,
    /// Pessary
    #[serde(rename = "200000002137")]
    N200000002137,
    /// Pillule
    #[serde(rename = "200000002138")]
    N200000002138,
    /// Pipette
    #[serde(rename = "200000002139")]
    N200000002139,
    /// Plaster
    #[serde(rename = "200000002140")]
    N200000002140,
    /// Plug
    #[serde(rename = "200000002141")]
    N200000002141,
    /// Pouch
    #[serde(rename = "200000002142")]
    N200000002142,
    /// Sachet
    #[serde(rename = "200000002143")]
    N200000002143,
    /// Sponge
    #[serde(rename = "200000002144")]
    N200000002144,
    /// Spoonful
    #[serde(rename = "200000002145")]
    N200000002145,
    /// Stick
    #[serde(rename = "200000002146")]
    N200000002146,
    /// Straw
    #[serde(rename = "200000002147")]
    N200000002147,
    /// Strip
    #[serde(rename = "200000002148")]
    N200000002148,
    /// Suppository
    #[serde(rename = "200000002149")]
    N200000002149,
    /// Syringe
    #[serde(rename = "200000002150")]
    N200000002150,
    /// System
    #[serde(rename = "200000002151")]
    N200000002151,
    /// Tablet
    #[serde(rename = "200000002152")]
    N200000002152,
    /// Tag
    #[serde(rename = "200000002153")]
    N200000002153,
    /// Tampon
    #[serde(rename = "200000002154")]
    N200000002154,
    /// Thread
    #[serde(rename = "200000002155")]
    N200000002155,
    /// Tube
    #[serde(rename = "200000002156")]
    N200000002156,
    /// Vessel
    #[serde(rename = "200000002157")]
    N200000002157,
    /// Vial
    #[serde(rename = "200000002158")]
    N200000002158,
    /// Puff
    #[serde(rename = "200000002159")]
    N200000002159,
    /// Actuation
    #[serde(rename = "200000002163")]
    N200000002163,
    /// Ampoule
    #[serde(rename = "200000002164")]
    N200000002164,
    /// Applicator
    #[serde(rename = "200000002165")]
    N200000002165,
    /// Bag
    #[serde(rename = "200000002166")]
    N200000002166,
}

/// The Digital Signature Purposes, an indication of the reason an entity signs
/// a document. This is included in the signed information and can be used when
/// determining accountability for various actions concerning the document.
/// Examples include: author, transcriptionist/recorder, and witness.
///
/// System: <urn:iso-astm:E1762-95:2013>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UrnIsoAstmE1762952013 {
    /// Author's Signature
    #[default]
    #[serde(rename = "1.2.840.10065.1.12.1.1")]
    N128401006511211,
    /// Coauthor's Signature
    #[serde(rename = "1.2.840.10065.1.12.1.2")]
    N128401006511212,
    /// Co-participant's Signature
    #[serde(rename = "1.2.840.10065.1.12.1.3")]
    N128401006511213,
    /// Transcriptionist/Recorder Signature
    #[serde(rename = "1.2.840.10065.1.12.1.4")]
    N128401006511214,
    /// Verification Signature
    #[serde(rename = "1.2.840.10065.1.12.1.5")]
    N128401006511215,
    /// Validation Signature
    #[serde(rename = "1.2.840.10065.1.12.1.6")]
    N128401006511216,
    /// Consent Signature
    #[serde(rename = "1.2.840.10065.1.12.1.7")]
    N128401006511217,
    /// Signature Witness Signature
    #[serde(rename = "1.2.840.10065.1.12.1.8")]
    N128401006511218,
    /// Event Witness Signature
    #[serde(rename = "1.2.840.10065.1.12.1.9")]
    N128401006511219,
    /// Identity Witness Signature
    #[serde(rename = "1.2.840.10065.1.12.1.10")]
    N1284010065112110,
    /// Consent Witness Signature
    #[serde(rename = "1.2.840.10065.1.12.1.11")]
    N1284010065112111,
    /// Interpreter Signature
    #[serde(rename = "1.2.840.10065.1.12.1.12")]
    N1284010065112112,
    /// Review Signature
    #[serde(rename = "1.2.840.10065.1.12.1.13")]
    N1284010065112113,
    /// Source Signature
    #[serde(rename = "1.2.840.10065.1.12.1.14")]
    N1284010065112114,
    /// Addendum Signature
    #[serde(rename = "1.2.840.10065.1.12.1.15")]
    N1284010065112115,
    /// Modification Signature
    #[serde(rename = "1.2.840.10065.1.12.1.16")]
    N1284010065112116,
    /// Administrative (Error/Edit) Signature
    #[serde(rename = "1.2.840.10065.1.12.1.17")]
    N1284010065112117,
    /// Timestamp Signature
    #[serde(rename = "1.2.840.10065.1.12.1.18")]
    N1284010065112118,
}

/// Example Item Flags for the List Resource. In this case, these are the kind
/// of flags that would be used on a medication list at the end of a
/// consultation.
///
/// System: <urn:oid:1.2.36.1.2001.1001.101.104.16592>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UrnOid123612001100110110416592 {
    /// Unchanged
    #[default]
    #[serde(rename = "01")]
    N01,
    /// Changed
    #[serde(rename = "02")]
    N02,
    /// Cancelled
    #[serde(rename = "03")]
    N03,
    /// Prescribed
    #[serde(rename = "04")]
    N04,
    /// Ceased
    #[serde(rename = "05")]
    N05,
    /// Suspended
    #[serde(rename = "06")]
    N06,
}

/// Life cycle of the Status Code of a Template Design (Version)
///
/// System: <urn:oid:2.16.840.1.113883.3.1937.98.5.8>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UrnOid2168401113883319379858 {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Under pre-publication review
    #[serde(rename = "pending")]
    Pending,
    /// active
    #[serde(rename = "active")]
    Active,
    /// In Review
    #[serde(rename = "review")]
    Review,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// retired
    #[serde(rename = "retired")]
    Retired,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
}

/// This codesystem defines codes describing the type of agreement represented
/// by an artifact, for example for use in CanonicalResource.usageContext.
///
/// System: <http://hl7.org/fhir/CodeSystem/usage-context-agreement-scope>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UsageContextAgreementScope {
    /// Realm Base
    #[default]
    #[serde(rename = "realm-base")]
    RealmBase,
    /// Knowledge
    #[serde(rename = "knowledge")]
    Knowledge,
    /// Domain
    #[serde(rename = "domain")]
    Domain,
    /// Community
    #[serde(rename = "community")]
    Community,
    /// System Design
    #[serde(rename = "system-design")]
    SystemDesign,
    /// System Implementation
    #[serde(rename = "system-implementation")]
    SystemImplementation,
}

/// The ETSI TS 101 733 V2.2.1 (2013-04) - Electronic Signatures and
/// Infrastructures (ESI) - defines a set of Commitment Types (Purpose of
/// Signature). ETSI TS 101 903 V1.2.2 defines vocabulary identifiers for these
/// Commitment Types. Digital Signature Purposes, an indication of the reason
/// an entity signs a document. This is included in the signed information and
/// can be used when determining accountability for various actions concerning
/// the document.
///
/// System: <http://uri.etsi.org/01903/v1.2.2>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum V122 {
    /// Proof of origin
    #[default]
    #[serde(rename = "ProofOfOrigin")]
    ProofOfOrigin,
    /// Proof of receipt
    #[serde(rename = "ProofOfReceipt")]
    ProofOfReceipt,
    /// Proof of delivery
    #[serde(rename = "ProofOfDelivery")]
    ProofOfDelivery,
    /// Proof of sender
    #[serde(rename = "ProofOfSender")]
    ProofOfSender,
    /// Proof of approval
    #[serde(rename = "ProofOfapproval")]
    ProofOfapproval,
    /// Proof of creation
    #[serde(rename = "ProofOfCreation")]
    ProofOfCreation,
}

/// The handling of the variable in statistical analysis for exposures or
/// outcomes (E.g. Dichotomous, Continuous, Descriptive).
///
/// System: <http://hl7.org/fhir/variable-handling>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VariableHandling {
    /// continuous variable
    #[default]
    #[serde(rename = "continuous")]
    Continuous,
    /// dichotomous variable
    #[serde(rename = "dichotomous")]
    Dichotomous,
    /// ordinal variable
    #[serde(rename = "ordinal")]
    Ordinal,
    /// polychotomous variable
    #[serde(rename = "polychotomous")]
    Polychotomous,
}

/// The validation status of the target
///
/// System: <http://hl7.org/fhir/CodeSystem/verificationresult-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VerificationresultStatus {
    /// Attested
    #[default]
    #[serde(rename = "attested")]
    Attested,
    /// Validated
    #[serde(rename = "validated")]
    Validated,
    /// In process
    #[serde(rename = "in-process")]
    InProcess,
    /// Requires revalidation
    #[serde(rename = "req-revalid")]
    ReqRevalid,
    /// Validation failed
    #[serde(rename = "val-fail")]
    ValFail,
    /// Re-Validation failed
    #[serde(rename = "reval-fail")]
    RevalFail,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Indicates the mechanism used to compare versions to determine which is more
/// current.
///
/// System: <http://hl7.org/fhir/version-algorithm>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VersionAlgorithm {
    /// SemVer
    #[default]
    #[serde(rename = "semver")]
    Semver,
    /// Integer
    #[serde(rename = "integer")]
    Integer,
    /// Alphabetical
    #[serde(rename = "alpha")]
    Alpha,
    /// Date
    #[serde(rename = "date")]
    Date,
    /// Natural
    #[serde(rename = "natural")]
    Natural,
}

/// How the system supports versioning for a resource.
///
/// System: <http://hl7.org/fhir/versioning-policy>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VersioningPolicy {
    /// No VersionId Support
    #[default]
    #[serde(rename = "no-version")]
    NoVersion,
    /// Versioned
    #[serde(rename = "versioned")]
    Versioned,
    /// VersionId tracked fully
    #[serde(rename = "versioned-update")]
    VersionedUpdate,
}

/// Example codes for possible virtual service connection types.
///
/// System: <http://hl7.org/fhir/virtual-service-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VirtualServiceType {
    /// Zoom web conferencing
    #[default]
    #[serde(rename = "zoom")]
    Zoom,
    /// Microsoft Teams
    #[serde(rename = "ms-teams")]
    MsTeams,
    /// WhatsApp conference call
    #[serde(rename = "whatsapp")]
    Whatsapp,
}

/// A coded concept listing the base codes.
///
/// System: <http://hl7.org/fhir/vision-base-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VisionBaseCodes {
    /// Up
    #[default]
    #[serde(rename = "up")]
    Up,
    /// Down
    #[serde(rename = "down")]
    Down,
    /// In
    #[serde(rename = "in")]
    In,
    /// Out
    #[serde(rename = "out")]
    Out,
}

/// A coded concept listing the eye codes.
///
/// System: <http://hl7.org/fhir/vision-eye-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VisionEyeCodes {
    /// Right Eye
    #[default]
    #[serde(rename = "right")]
    Right,
    /// Left Eye
    #[serde(rename = "left")]
    Left,
}

/// This value set includes W3C PROV Data Model Activity concepts, which are
/// treated as codes in this valueset. Some adaptations were made to make these
/// concepts suitable values for the Provenance.activity element. Coded
/// concepts are from PROV-DM and the display names are their counterparts in
/// PROV-N (human readable notation syntax specification).[code system OID:
/// http://www.w3.org/TR/2013/REC-prov-dm-20130430/ and
/// http://www.w3.org/TR/2013/REC-prov-n-20130430/]
///
/// System: <http://hl7.org/fhir/w3c-provenance-activity-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum W3CProvenanceActivityType {
    /// wasGeneratedBy
    #[default]
    #[serde(rename = "Generation")]
    Generation,
    /// used
    #[serde(rename = "Usage")]
    Usage,
    /// wasInformedBy
    #[serde(rename = "Communication")]
    Communication,
    /// wasStartedBy
    #[serde(rename = "Start")]
    Start,
    /// wasEndedBy
    #[serde(rename = "End")]
    End,
    /// wasInvalidatedBy
    #[serde(rename = "Invalidation")]
    Invalidation,
    /// wasDerivedFrom
    #[serde(rename = "Derivation")]
    Derivation,
    /// wasRevisionOf
    #[serde(rename = "Revision")]
    Revision,
    /// wasQuotedFrom
    #[serde(rename = "Quotation")]
    Quotation,
    /// wasPrimarySourceOf
    #[serde(rename = "Primary-Source")]
    PrimarySource,
    /// wasAttributedTo
    #[serde(rename = "Attribution")]
    Attribution,
    /// isCollectionOf
    #[serde(rename = "Collection")]
    Collection,
}

/// Classification of warning type.
///
/// System: <http://hl7.org/fhir/warning-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WarningType {
    /// Get medical advice/attention.
    #[default]
    #[serde(rename = "P313")]
    P313,
    /// Get medical advice/attention if you feel unwell.
    #[serde(rename = "P314")]
    P314,
    /// Get immediate medical advice/attention.
    #[serde(rename = "P315")]
    P315,
    /// Specific treatment is urgent (see ... on this label).
    #[serde(rename = "P320")]
    P320,
    /// Specific treatment (see ... on this label).
    #[serde(rename = "P321")]
    P321,
    /// Specific measures (see ... on this label).
    #[serde(rename = "P322")]
    P322,
    /// Rinse mouth.
    #[serde(rename = "P330")]
    P330,
    /// Do NOT induce vomiting.
    #[serde(rename = "P331")]
    P331,
    /// Remove/Take off immediately all contaminated clothing.
    #[serde(rename = "P361")]
    P361,
    /// Wash contaminated clothing before reuse..
    #[serde(rename = "P363")]
    P363,
}

/// The set of weeks with in a month.
///
/// System: <http://hl7.org/fhir/week-of-month>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WeekOfMonth {
    /// First
    #[default]
    #[serde(rename = "first")]
    First,
    /// Second
    #[serde(rename = "second")]
    Second,
    /// Third
    #[serde(rename = "third")]
    Third,
    /// Fourth
    #[serde(rename = "fourth")]
    Fourth,
    /// Last
    #[serde(rename = "last")]
    Last,
}
