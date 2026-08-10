//! FHIR R2 code systems as type-safe Rust enums.
//!
//! Each enum corresponds to a complete FHIR R2 `CodeSystem`. Variants
//! serialize to and from their canonical FHIR code strings via serde renames.
//!
//! # Examples
//!
//! ```
//! use fhir::r2::codes::AdministrativeGender;
//!
//! let code = AdministrativeGender::Male;
//! assert_eq!(::serde_json::to_value(&code).unwrap(), ::serde_json::json!("male"));
//! ```

use ::serde::{Deserialize, Serialize};

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
}

/// List of allowable action which this resource can request.
///
/// System: <http://hl7.org/fhir/actionlist>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Actionlist {
    /// Cancel, Reverse or Nullify
    #[default]
    #[serde(rename = "cancel")]
    Cancel,
    /// Poll
    #[serde(rename = "poll")]
    Poll,
    /// Re-Process
    #[serde(rename = "reprocess")]
    Reprocess,
    /// Status Check
    #[serde(rename = "status")]
    Status,
}

/// This value set includes sample additional material type codes.
///
/// System: <http://hl7.org/fhir/additionalmaterials>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Additionalmaterials {
    /// XRay
    #[default]
    #[serde(rename = "xray")]
    Xray,
    /// Image
    #[serde(rename = "image")]
    Image,
    /// Email
    #[serde(rename = "email")]
    Email,
    /// Model
    #[serde(rename = "model")]
    Model,
    /// Document
    #[serde(rename = "document")]
    Document,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// The type of an address (physical / postal) The type of an address (physical
/// / postal).
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

/// The use of an address The use of an address (home / work / etc.).
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
}

/// This value set includes a smattering of Adjudication codes.
///
/// System: <http://hl7.org/fhir/adjudication>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Adjudication {
    /// total
    #[default]
    #[serde(rename = "total")]
    Total,
    /// copay
    #[serde(rename = "copay")]
    Copay,
    /// eligible
    #[serde(rename = "eligible")]
    Eligible,
    /// deductible
    #[serde(rename = "deductible")]
    Deductible,
    /// eligpercent
    #[serde(rename = "eligpercent")]
    Eligpercent,
    /// tax
    #[serde(rename = "tax")]
    Tax,
    /// benefit
    #[serde(rename = "benefit")]
    Benefit,
}

/// This value set includes a smattering of adjudication codes.
///
/// System: <http://hl7.org/fhir/adjudication-error>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdjudicationError {
    /// A001
    #[default]
    #[serde(rename = "A001")]
    A001,
    /// A002
    #[serde(rename = "A002")]
    A002,
}

/// This value set includes smattering of a Adjustment Reason codes.
///
/// System: <http://hl7.org/fhir/adjustment-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdjustmentReason {
    /// A001
    #[default]
    #[serde(rename = "A001")]
    A001,
    /// A002
    #[serde(rename = "A002")]
    A002,
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

/// This value set defines a set of codes that can be used to indicate from
/// where the patient came in.
///
/// System: <http://hl7.org/fhir/admit-source>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AdmitSource {
    /// Transferred from other hospital
    #[default]
    #[serde(rename = "hosp-trans")]
    HospTrans,
    /// From accident/emergency department
    #[serde(rename = "emd")]
    Emd,
    /// From outpatient department
    #[serde(rename = "outp")]
    Outp,
    /// Born in hospital
    #[serde(rename = "born")]
    Born,
    /// General Practitioner referral
    #[serde(rename = "gp")]
    Gp,
    /// Medical Practitioner/physician referral
    #[serde(rename = "mp")]
    Mp,
    /// From nursing home
    #[serde(rename = "nursing")]
    Nursing,
    /// From psychiatric hospital
    #[serde(rename = "psych")]
    Psych,
    /// From rehabilitation facility
    #[serde(rename = "rehab")]
    Rehab,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Category of an identified Substance.
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
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Estimate of the potential clinical harm, or seriousness, of a reaction to
/// an identified Substance.
///
/// System: <http://hl7.org/fhir/allergy-intolerance-criticality>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AllergyIntoleranceCriticality {
    /// Low Risk
    #[default]
    #[serde(rename = "CRITL")]
    Critl,
    /// High Risk
    #[serde(rename = "CRITH")]
    Crith,
    /// Unable to determine
    #[serde(rename = "CRITU")]
    Critu,
}

/// Assertion about certainty associated with a propensity, or potential risk,
/// of a reaction to the identified Substance.
///
/// System: <http://hl7.org/fhir/allergy-intolerance-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AllergyIntoleranceStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Unconfirmed
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// Confirmed
    #[serde(rename = "confirmed")]
    Confirmed,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// Refuted
    #[serde(rename = "refuted")]
    Refuted,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
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

/// This example value set defines a set of codes that can be used to indicate
/// breeds of species.
///
/// System: <http://hl7.org/fhir/animal-breed>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AnimalBreed {
    /// German Shepherd Dog
    #[default]
    #[serde(rename = "gsd")]
    Gsd,
    /// Irish Terrier
    #[serde(rename = "irt")]
    Irt,
    /// Tibetan Mastiff
    #[serde(rename = "tibmas")]
    Tibmas,
    /// Golden Retriever
    #[serde(rename = "gret")]
    Gret,
}

/// This example value set defines a set of codes that can be used to indicate
/// the current state of the animal's reproductive organs.
///
/// System: <http://hl7.org/fhir/animal-genderstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AnimalGenderstatus {
    /// Neutered
    #[default]
    #[serde(rename = "neutered")]
    Neutered,
    /// Intact
    #[serde(rename = "intact")]
    Intact,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// This example value set defines a set of codes that can be used to indicate
/// species of animal patients.
///
/// System: <http://hl7.org/fhir/animal-species>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AnimalSpecies {
    /// Dog
    #[default]
    #[serde(rename = "canislf")]
    Canislf,
    /// Sheep
    #[serde(rename = "ovisa")]
    Ovisa,
    /// Domestic Canary
    #[serde(rename = "serinuscd")]
    Serinuscd,
}

/// The expected format of an answer.
///
/// System: <http://hl7.org/fhir/answer-format>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AnswerFormat {
    /// Boolean
    #[default]
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
    /// Instant
    #[serde(rename = "instant")]
    Instant,
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
    /// Choice
    #[serde(rename = "choice")]
    Choice,
    /// Open Choice
    #[serde(rename = "open-choice")]
    OpenChoice,
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
}

/// The RAND scoring for appropriateness of the procedure.
///
/// System: <http://hl7.org/fhir/appropriateness-score>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AppropriatenessScore {
    /// Extremely Inappropriate
    #[default]
    #[serde(rename = "extremely-inappropriate")]
    ExtremelyInappropriate,
    /// Inappropriate
    #[serde(rename = "inappropriate")]
    Inappropriate,
    /// Probably Inappropriate
    #[serde(rename = "probably-inappropriate")]
    ProbablyInappropriate,
    /// Uncertain Inappropriate
    #[serde(rename = "uncertain-inappropriate")]
    UncertainInappropriate,
    /// Uncertain
    #[serde(rename = "uncertain")]
    Uncertain,
    /// Uncertain Appropriate
    #[serde(rename = "uncertain-appropriate")]
    UncertainAppropriate,
    /// Probably Appropriate
    #[serde(rename = "probably-appropriate")]
    ProbablyAppropriate,
    /// Appropriate
    #[serde(rename = "appropriate")]
    Appropriate,
    /// Extremely Appropriate
    #[serde(rename = "extremely-appropriate")]
    ExtremelyAppropriate,
}

/// The type of direction to use for assertion. The direction to use for
/// assertions.
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

/// The type of operator to use for assertion. The type of operator to use for
/// assertions.
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
}

/// The type of response code to use for assertion. The response code to expect
/// in the response.
///
/// System: <http://hl7.org/fhir/assert-response-code-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AssertResponseCodeTypes {
    /// okay
    #[default]
    #[serde(rename = "okay")]
    Okay,
    /// created
    #[serde(rename = "created")]
    Created,
    /// noContent
    #[serde(rename = "noContent")]
    NoContent,
    /// notModified
    #[serde(rename = "notModified")]
    NotModified,
    /// bad
    #[serde(rename = "bad")]
    Bad,
    /// forbidden
    #[serde(rename = "forbidden")]
    Forbidden,
    /// notFound
    #[serde(rename = "notFound")]
    NotFound,
    /// methodNotAllowed
    #[serde(rename = "methodNotAllowed")]
    MethodNotAllowed,
    /// conflict
    #[serde(rename = "conflict")]
    Conflict,
    /// gone
    #[serde(rename = "gone")]
    Gone,
    /// preconditionFailed
    #[serde(rename = "preconditionFailed")]
    PreconditionFailed,
    /// unprocessable
    #[serde(rename = "unprocessable")]
    Unprocessable,
}

/// Indicator for type of action performed during the event that generated the
/// audit.
///
/// System: <http://hl7.org/fhir/audit-event-action>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AuditEventAction {
    /// Create
    #[default]
    #[serde(rename = "C")]
    C,
    /// Read/View/Print
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

/// Indicates whether the event succeeded or failed
///
/// System: <http://hl7.org/fhir/audit-event-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AuditEventOutcome {
    /// Success
    #[default]
    #[serde(rename = "0")]
    N0,
    /// Minor failure
    #[serde(rename = "4")]
    N4,
    /// Serious failure
    #[serde(rename = "8")]
    N8,
    /// Major failure
    #[serde(rename = "12")]
    N12,
}

/// Event Types for Audit Events - defined by DICOM with some FHIR specific
/// additions.
///
/// System: <http://hl7.org/fhir/audit-event-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AuditEventType {
    /// 110100
    #[default]
    #[serde(rename = "110100")]
    N110100,
    /// 110101
    #[serde(rename = "110101")]
    N110101,
    /// 110102
    #[serde(rename = "110102")]
    N110102,
    /// 110103
    #[serde(rename = "110103")]
    N110103,
    /// 110104
    #[serde(rename = "110104")]
    N110104,
    /// 110105
    #[serde(rename = "110105")]
    N110105,
    /// 110106
    #[serde(rename = "110106")]
    N110106,
    /// 110107
    #[serde(rename = "110107")]
    N110107,
    /// 110108
    #[serde(rename = "110108")]
    N110108,
    /// 110109
    #[serde(rename = "110109")]
    N110109,
    /// 110110
    #[serde(rename = "110110")]
    N110110,
    /// 110111
    #[serde(rename = "110111")]
    N110111,
    /// 110112
    #[serde(rename = "110112")]
    N110112,
    /// 110113
    #[serde(rename = "110113")]
    N110113,
    /// 110114
    #[serde(rename = "110114")]
    N110114,
}

/// This value set defines codes for resources not yet supported by (or which
/// will never be supported by) FHIR. Many of the codes listed here will
/// eventually be turned into official resources. However, there is no
/// guarantee that any particular resource will be created nor that the scope
/// will be exactly as defined by the codes presented here. Codes in this set
/// will be deprecated if/when formal resources are defined that encompass
/// these concepts.
///
/// System: <http://hl7.org/fhir/basic-resource-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BasicResourceType {
    /// Consent
    #[default]
    #[serde(rename = "consent")]
    Consent,
    /// Referral
    #[serde(rename = "referral")]
    Referral,
    /// Adverse Event
    #[serde(rename = "advevent")]
    Advevent,
    /// Appointment Request
    #[serde(rename = "aptmtreq")]
    Aptmtreq,
    /// Transfer
    #[serde(rename = "transfer")]
    Transfer,
    /// Diet
    #[serde(rename = "diet")]
    Diet,
    /// Administrative Activity
    #[serde(rename = "adminact")]
    Adminact,
    /// Exposure
    #[serde(rename = "exposure")]
    Exposure,
    /// Investigation
    #[serde(rename = "investigation")]
    Investigation,
    /// Account
    #[serde(rename = "account")]
    Account,
    /// Invoice
    #[serde(rename = "invoice")]
    Invoice,
    /// Invoice Adjudication
    #[serde(rename = "adjudicat")]
    Adjudicat,
    /// Pre-determination Request
    #[serde(rename = "predetreq")]
    Predetreq,
    /// Predetermination
    #[serde(rename = "predetermine")]
    Predetermine,
    /// Study
    #[serde(rename = "study")]
    Study,
    /// Protocol
    #[serde(rename = "protocol")]
    Protocol,
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

/// Indicates the purpose of a bundle - how it was intended to be used.
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
}

/// High-level categorization of the type of activity in a care plan.
///
/// System: <http://hl7.org/fhir/care-plan-activity-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CarePlanActivityCategory {
    /// Diet
    #[default]
    #[serde(rename = "diet")]
    Diet,
    /// Drug
    #[serde(rename = "drug")]
    Drug,
    /// Encounter
    #[serde(rename = "encounter")]
    Encounter,
    /// Observation
    #[serde(rename = "observation")]
    Observation,
    /// Procedure
    #[serde(rename = "procedure")]
    Procedure,
    /// Supply
    #[serde(rename = "supply")]
    Supply,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Indicates where the activity is at in its overall life cycle.
///
/// System: <http://hl7.org/fhir/care-plan-activity-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CarePlanActivityStatus {
    /// Not Started
    #[default]
    #[serde(rename = "not-started")]
    NotStarted,
    /// Scheduled
    #[serde(rename = "scheduled")]
    Scheduled,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Codes identifying the types of relationships between two plans.
///
/// System: <http://hl7.org/fhir/care-plan-relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CarePlanRelationship {
    /// Includes
    #[default]
    #[serde(rename = "includes")]
    Includes,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Fulfills
    #[serde(rename = "fulfills")]
    Fulfills,
}

/// Indicates whether the plan is currently being acted upon, represents future
/// intentions or is now a historical record.
///
/// System: <http://hl7.org/fhir/care-plan-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CarePlanStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Pending
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Direction in which lists of question options should be displayed
///
/// System: <http://hl7.org/fhir/choice-list-orientation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ChoiceListOrientation {
    /// Horizontal
    #[default]
    #[serde(rename = "horizontal")]
    Horizontal,
    /// Vertical
    #[serde(rename = "vertical")]
    Vertical,
}

/// This is an example value set that includes all the ACME codes for
/// serum/plasma cholesterol from v2.36.
///
/// System: <http://acme.com/config/fhir/codesystems/cholesterol>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Cholesterol {
    /// SChol (mmol/L)
    #[default]
    #[serde(rename = "chol-mmol")]
    CholMmol,
    /// SChol (mg/L)
    #[serde(rename = "chol-mass")]
    CholMass,
    /// SChol
    #[serde(rename = "chol")]
    Chol,
}

/// The type or discipline-style of the claim.
///
/// System: <http://hl7.org/fhir/claim-type-link>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimTypeLink {
    /// Institutional
    #[default]
    #[serde(rename = "institutional")]
    Institutional,
    /// Oral Health
    #[serde(rename = "oral")]
    Oral,
    /// Pharmacy
    #[serde(rename = "pharmacy")]
    Pharmacy,
    /// Professional
    #[serde(rename = "professional")]
    Professional,
    /// Vision
    #[serde(rename = "vision")]
    Vision,
}

/// Complete, proposed, exploratory, other.
///
/// System: <http://hl7.org/fhir/claim-use-link>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClaimUseLink {
    /// Complete
    #[default]
    #[serde(rename = "complete")]
    Complete,
    /// Proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// Exploratory
    #[serde(rename = "exploratory")]
    Exploratory,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Identifies whether a useContext represents a context or classification for
/// the element
///
/// System: <http://hl7.org/fhir/classification-or-context>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClassificationOrContext {
    /// Classification
    #[default]
    #[serde(rename = "classification")]
    Classification,
    /// Context
    #[serde(rename = "context")]
    Context,
}

/// The workflow state of a clinical impression.
///
/// System: <http://hl7.org/fhir/clinical-impression-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ClinicalImpressionStatus {
    /// In progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The status of the communication.
///
/// System: <http://hl7.org/fhir/communication-request-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CommunicationRequestStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "received")]
    Received,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
}

/// The status of the communication.
///
/// System: <http://hl7.org/fhir/communication-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CommunicationStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
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
    /// Preliminary
    #[default]
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The degree of equivalence between concepts.
///
/// System: <http://hl7.org/fhir/concept-map-equivalence>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConceptMapEquivalence {
    /// Equivalent
    #[default]
    #[serde(rename = "equivalent")]
    Equivalent,
    /// Equal
    #[serde(rename = "equal")]
    Equal,
    /// Wider
    #[serde(rename = "wider")]
    Wider,
    /// Subsumes
    #[serde(rename = "subsumes")]
    Subsumes,
    /// Narrower
    #[serde(rename = "narrower")]
    Narrower,
    /// Specializes
    #[serde(rename = "specializes")]
    Specializes,
    /// Inexact
    #[serde(rename = "inexact")]
    Inexact,
    /// Unmatched
    #[serde(rename = "unmatched")]
    Unmatched,
    /// Disjoint
    #[serde(rename = "disjoint")]
    Disjoint,
}

/// Preferred value set for Condition Categories.
///
/// System: <http://hl7.org/fhir/condition-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionCategory {
    /// Complaint
    #[default]
    #[serde(rename = "complaint")]
    Complaint,
    /// Symptom
    #[serde(rename = "symptom")]
    Symptom,
    /// Finding
    #[serde(rename = "finding")]
    Finding,
    /// Diagnosis
    #[serde(rename = "diagnosis")]
    Diagnosis,
}

/// Preferred value set for Condition Clinical Status.
///
/// System: <http://hl7.org/fhir/condition-clinical>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionClinical {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Relapse
    #[serde(rename = "relapse")]
    Relapse,
    /// Remission
    #[serde(rename = "remission")]
    Remission,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
}

/// Enumeration indicating whether the condition is currently active, inactive,
/// or has been resolved. active| inactive| resolved
///
/// System: <http://hl7.org/fhir/condition-state>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionState {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
}

/// The verification status to support or decline the clinical status of the
/// condition or diagnosis.
///
/// System: <http://hl7.org/fhir/condition-ver-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConditionVerStatus {
    /// Provisional
    #[default]
    #[serde(rename = "provisional")]
    Provisional,
    /// Differential
    #[serde(rename = "differential")]
    Differential,
    /// Confirmed
    #[serde(rename = "confirmed")]
    Confirmed,
    /// Refuted
    #[serde(rename = "refuted")]
    Refuted,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
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

/// Indicates the degree of adherence to a specified behavior or capability
/// expected in order for a system to be deemed conformant with a
/// specification.
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

/// The lifecycle status of a Value Set or Concept Map.
///
/// System: <http://hl7.org/fhir/conformance-resource-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConformanceResourceStatus {
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
}

/// How a conformance statement is intended to be used.
///
/// System: <http://hl7.org/fhir/conformance-statement-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConformanceStatementKind {
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

/// Telecommunications form for contact point
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
    #[serde(rename = "other")]
    Other,
}

/// Use of contact point
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

/// This example value set defines a set of codes that can be used to indicate
/// the purpose for which you would contact a contact party.
///
/// System: <http://hl7.org/fhir/contactentity-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContactentityType {
    /// Billing
    #[default]
    #[serde(rename = "BILL")]
    Bill,
    /// Administrative
    #[serde(rename = "ADMIN")]
    Admin,
    /// Human Resource
    #[serde(rename = "HR")]
    Hr,
    /// Payor
    #[serde(rename = "PAYOR")]
    Payor,
    /// Patient
    #[serde(rename = "PATINF")]
    Patinf,
    /// Press
    #[serde(rename = "PRESS")]
    Press,
}

/// The content or mime type. The content type or mime type to be specified in
/// Accept or Content-Type header.
///
/// System: <http://hl7.org/fhir/content-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ContentType {
    /// xml
    #[default]
    #[serde(rename = "xml")]
    Xml,
    /// json
    #[serde(rename = "json")]
    Json,
}

/// This value set includes sample Contract Action codes.
///
/// System: <http://www.hl7.org/fhir/contractaction>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contractaction {
    /// action-a
    #[default]
    #[serde(rename = "action-a")]
    ActionA,
    /// action-b
    #[serde(rename = "action-b")]
    ActionB,
}

/// This value set includes sample Contract Actor Role codes.
///
/// System: <http://www.hl7.org/fhir/contractactorrole>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contractactorrole {
    /// Practitioner
    #[default]
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Patient
    #[serde(rename = "patient")]
    Patient,
}

/// The Digital Signature Purposes, an indication of the reason an entity signs
/// a document. This is included in the signed information and can be used when
/// determining accountability for various actions concerning the document.
/// Examples include: author, transcriptionist/recorder, and witness.
///
/// System: <http://hl7.org/fhir/contractsignertypecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contractsignertypecodes {
    /// AuthorID
    #[default]
    #[serde(rename = "1.2.840.10065.1.12.1.1")]
    N128401006511211,
    /// Co-AuthorID
    #[serde(rename = "1.2.840.10065.1.12.1.2")]
    N128401006511212,
    /// Co-Participated
    #[serde(rename = "1.2.840.10065.1.12.1.3")]
    N128401006511213,
    /// Transcriptionist
    #[serde(rename = "1.2.840.10065.1.12.1.4")]
    N128401006511214,
    /// Verification
    #[serde(rename = "1.2.840.10065.1.12.1.5")]
    N128401006511215,
    /// Validation
    #[serde(rename = "1.2.840.10065.1.12.1.6")]
    N128401006511216,
    /// Consent
    #[serde(rename = "1.2.840.10065.1.12.1.7")]
    N128401006511217,
    /// Witness
    #[serde(rename = "1.2.840.10065.1.12.1.8")]
    N128401006511218,
    /// Event-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.9")]
    N128401006511219,
    /// Identity-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.10")]
    N1284010065112110,
    /// Consent-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.11")]
    N1284010065112111,
    /// Interpreter
    #[serde(rename = "1.2.840.10065.1.12.1.12")]
    N1284010065112112,
    /// Review
    #[serde(rename = "1.2.840.10065.1.12.1.13")]
    N1284010065112113,
    /// Source
    #[serde(rename = "1.2.840.10065.1.12.1.14")]
    N1284010065112114,
    /// Addendum
    #[serde(rename = "1.2.840.10065.1.12.1.15")]
    N1284010065112115,
    /// Administrative
    #[serde(rename = "1.2.840.10065.1.12.1.16")]
    N1284010065112116,
    /// Timestamp
    #[serde(rename = "1.2.840.10065.1.12.1.17")]
    N1284010065112117,
}

/// This value set includes sample Contract Subtype codes.
///
/// System: <http://hl7.org/fhir/contractsubtypecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contractsubtypecodes {
    /// disclosure-CA
    #[default]
    #[serde(rename = "disclosure-CA")]
    DisclosureCa,
    /// disclosure-US
    #[serde(rename = "disclosure-US")]
    DisclosureUs,
}

/// This value set includes sample Contract Term SubType codes.
///
/// System: <http://hl7.org/fhir/contracttermsubtypecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contracttermsubtypecodes {
    /// OralHealth-Basic
    #[default]
    #[serde(rename = "OralHealth-Basic")]
    OralHealthBasic,
    /// OralHealth-Major
    #[serde(rename = "OralHealth-Major")]
    OralHealthMajor,
    /// OralHealth-Orthodontic
    #[serde(rename = "OralHealth-Orthodontic")]
    OralHealthOrthodontic,
}

/// This value set includes sample Contract Term Type codes.
///
/// System: <http://hl7.org/fhir/contracttermtypecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contracttermtypecodes {
    /// OralHealth
    #[default]
    #[serde(rename = "OralHealth")]
    OralHealth,
    /// Vision
    #[serde(rename = "Vision")]
    Vision,
}

/// This value set includes sample Contract Type codes.
///
/// System: <http://hl7.org/fhir/contracttypecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Contracttypecodes {
    /// privacy
    #[default]
    #[serde(rename = "privacy")]
    Privacy,
    /// disclosure
    #[serde(rename = "disclosure")]
    Disclosure,
}

/// Used to specify why the normally expected content of the data element is
/// missing.
///
/// System: <http://hl7.org/fhir/data-absent-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DataAbsentReason {
    /// Unknown
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    /// Asked
    #[serde(rename = "asked")]
    Asked,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Not Asked
    #[serde(rename = "not-asked")]
    NotAsked,
    /// Masked
    #[serde(rename = "masked")]
    Masked,
    /// Unsupported
    #[serde(rename = "unsupported")]
    Unsupported,
    /// As Text
    #[serde(rename = "astext")]
    Astext,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Not a Number
    #[serde(rename = "NaN")]
    NaN,
}

/// The type of an element - one of the FHIR data types.
///
/// System: <http://hl7.org/fhir/data-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DataTypes {
    /// Address
    #[default]
    #[serde(rename = "Address")]
    Address,
    /// Age
    #[serde(rename = "Age")]
    Age,
    /// Annotation
    #[serde(rename = "Annotation")]
    Annotation,
    /// Attachment
    #[serde(rename = "Attachment")]
    Attachment,
    /// BackboneElement
    #[serde(rename = "BackboneElement")]
    BackboneElement,
    /// CodeableConcept
    #[serde(rename = "CodeableConcept")]
    CodeableConcept,
    /// Coding
    #[serde(rename = "Coding")]
    Coding,
    /// ContactPoint
    #[serde(rename = "ContactPoint")]
    ContactPoint,
    /// Count
    #[serde(rename = "Count")]
    Count,
    /// Distance
    #[serde(rename = "Distance")]
    Distance,
    /// Duration
    #[serde(rename = "Duration")]
    Duration,
    /// Element
    #[serde(rename = "Element")]
    Element,
    /// ElementDefinition
    #[serde(rename = "ElementDefinition")]
    ElementDefinition,
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
    /// Money
    #[serde(rename = "Money")]
    Money,
    /// Narrative
    #[serde(rename = "Narrative")]
    Narrative,
    /// Period
    #[serde(rename = "Period")]
    Period,
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
    /// Range
    #[serde(rename = "Range")]
    Range,
    /// Ratio
    #[serde(rename = "Ratio")]
    Ratio,
    /// Reference
    #[serde(rename = "Reference")]
    Reference,
    /// SampledData
    #[serde(rename = "SampledData")]
    SampledData,
    /// Signature
    #[serde(rename = "Signature")]
    Signature,
    /// SimpleQuantity
    #[serde(rename = "SimpleQuantity")]
    SimpleQuantity,
    /// Timing
    #[serde(rename = "Timing")]
    Timing,
    /// base64Binary
    #[serde(rename = "base64Binary")]
    Base64Binary,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// code
    #[serde(rename = "code")]
    Code,
    /// date
    #[serde(rename = "date")]
    Date,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// id
    #[serde(rename = "id")]
    Id,
    /// instant
    #[serde(rename = "instant")]
    Instant,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// markdown
    #[serde(rename = "markdown")]
    Markdown,
    /// oid
    #[serde(rename = "oid")]
    Oid,
    /// positiveInt
    #[serde(rename = "positiveInt")]
    PositiveInt,
    /// string
    #[serde(rename = "string")]
    String,
    /// time
    #[serde(rename = "time")]
    Time,
    /// unsignedInt
    #[serde(rename = "unsignedInt")]
    UnsignedInt,
    /// uri
    #[serde(rename = "uri")]
    Uri,
    /// uuid
    #[serde(rename = "uuid")]
    Uuid,
    /// XHTML
    #[serde(rename = "xhtml")]
    Xhtml,
}

/// Indicates the degree of precision of the data element definition.
///
/// System: <http://hl7.org/fhir/dataelement-stringency>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DataelementStringency {
    /// Comparable
    #[default]
    #[serde(rename = "comparable")]
    Comparable,
    /// Fully Specified
    #[serde(rename = "fully-specified")]
    FullySpecified,
    /// Equivalent
    #[serde(rename = "equivalent")]
    Equivalent,
    /// Convertable
    #[serde(rename = "convertable")]
    Convertable,
    /// Scaleable
    #[serde(rename = "scaleable")]
    Scaleable,
    /// Flexible
    #[serde(rename = "flexible")]
    Flexible,
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

/// Codes representing the priority of the request.
///
/// System: <http://hl7.org/fhir/device-use-request-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceUseRequestPriority {
    /// Routine
    #[default]
    #[serde(rename = "routine")]
    Routine,
    /// Urgent
    #[serde(rename = "urgent")]
    Urgent,
    /// Stat
    #[serde(rename = "stat")]
    Stat,
    /// ASAP
    #[serde(rename = "asap")]
    Asap,
}

/// Codes representing the status of the request.
///
/// System: <http://hl7.org/fhir/device-use-request-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceUseRequestStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "received")]
    Received,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Aborted
    #[serde(rename = "aborted")]
    Aborted,
}

/// The availability status of the device.
///
/// System: <http://hl7.org/fhir/devicestatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Devicestatus {
    /// Available
    #[default]
    #[serde(rename = "available")]
    Available,
    /// Not Available
    #[serde(rename = "not-available")]
    NotAvailable,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// The clinical priority of a diagnostic order.
///
/// System: <http://hl7.org/fhir/diagnostic-order-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DiagnosticOrderPriority {
    /// Routine
    #[default]
    #[serde(rename = "routine")]
    Routine,
    /// Urgent
    #[serde(rename = "urgent")]
    Urgent,
    /// Stat
    #[serde(rename = "stat")]
    Stat,
    /// ASAP
    #[serde(rename = "asap")]
    Asap,
}

/// The status of a diagnostic order.
///
/// System: <http://hl7.org/fhir/diagnostic-order-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DiagnosticOrderStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "received")]
    Received,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// In-Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Review
    #[serde(rename = "review")]
    Review,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
}

/// The status of the diagnostic report as a whole.
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
    /// Final
    #[serde(rename = "final")]
    Final,
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
}

/// DICOM Code Definitions (Coding Scheme Designator "DCM" Coding Scheme
/// Version "01")
///
/// System: <http://nema.org/dicom/dicm>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Dicm {
    /// Archive
    #[default]
    #[serde(rename = "ARCHIVE")]
    Archive,
    /// Autorefraction
    #[serde(rename = "AR")]
    Ar,
    /// Angioscopy
    #[serde(rename = "AS")]
    As,
    /// Audio
    #[serde(rename = "AU")]
    Au,
    /// Ultrasound Bone Densitometry
    #[serde(rename = "BDUS")]
    Bdus,
    /// Biomagnetic imaging
    #[serde(rename = "BI")]
    Bi,
    /// Bone Mineral Densitometry
    #[serde(rename = "BMD")]
    Bmd,
    /// Computer Assisted Detection/Diagnosis
    #[serde(rename = "CAD")]
    Cad,
    /// Image Capture
    #[serde(rename = "CAPTURE")]
    Capture,
    /// Color flow Doppler
    #[serde(rename = "CD")]
    Cd,
    /// Cinefluorography
    #[serde(rename = "CF")]
    Cf,
    /// Computation Server
    #[serde(rename = "COMP")]
    Comp,
    /// Culposcopy
    #[serde(rename = "CP")]
    Cp,
    /// Computed Radiography
    #[serde(rename = "CR")]
    Cr,
    /// Cystoscopy
    #[serde(rename = "CS")]
    Cs,
    /// Computed Tomography
    #[serde(rename = "CT")]
    Ct,
    /// Duplex Doppler
    #[serde(rename = "DD")]
    Dd,
    /// Digital fluoroscopy
    #[serde(rename = "DF")]
    Df,
    /// Diaphanography
    #[serde(rename = "DG")]
    Dg,
    /// Digital microscopy
    #[serde(rename = "DM")]
    Dm,
    /// Document Digitizer Equipment
    #[serde(rename = "DOCD")]
    Docd,
    /// Digital Subtraction Angiography
    #[serde(rename = "DS")]
    Ds,
    /// Department System Scheduler
    #[serde(rename = "DSS")]
    Dss,
    /// Digital Radiography
    #[serde(rename = "DX")]
    Dx,
    /// Echocardiography
    #[serde(rename = "EC")]
    Ec,
    /// Electrocardiography
    #[serde(rename = "ECG")]
    Ecg,
    /// Cardiac Electrophysiology
    #[serde(rename = "EPS")]
    Eps,
    /// Endoscopy
    #[serde(rename = "ES")]
    Es,
    /// Female
    #[serde(rename = "F")]
    F,
    /// Fluorescein angiography
    #[serde(rename = "FA")]
    Fa,
    /// Female changed to Male
    #[serde(rename = "FC")]
    Fc,
    /// Film Digitizer
    #[serde(rename = "FILMD")]
    Filmd,
    /// Female Pseudohermaphrodite
    #[serde(rename = "FP")]
    Fp,
    /// Fundoscopy
    #[serde(rename = "FS")]
    Fs,
    /// General Microscopy
    #[serde(rename = "GM")]
    Gm,
    /// Hermaphrodite
    #[serde(rename = "H")]
    H,
    /// Hard Copy
    #[serde(rename = "HC")]
    Hc,
    /// Hemodynamic Waveform
    #[serde(rename = "HD")]
    Hd,
    /// Intra-oral Radiography
    #[serde(rename = "IO")]
    Io,
    /// Intravascular Ultrasound
    #[serde(rename = "IVUS")]
    Ivus,
    /// Keratometry
    #[serde(rename = "KER")]
    Ker,
    /// Key Object Selection
    #[serde(rename = "KO")]
    Ko,
    /// Lensometry
    #[serde(rename = "LEN")]
    Len,
    /// Procedure Logging
    #[serde(rename = "LOG")]
    Log,
    /// Laparoscopy
    #[serde(rename = "LP")]
    Lp,
    /// Laser surface scan
    #[serde(rename = "LS")]
    Ls,
    /// Male
    #[serde(rename = "M")]
    M,
    /// Magnetic resonance angiography
    #[serde(rename = "MA")]
    Ma,
    /// Male changed to Female
    #[serde(rename = "MC")]
    Mc,
    /// Media Creation Device
    #[serde(rename = "MCD")]
    Mcd,
    /// Portable Media Importer Equipment
    #[serde(rename = "MEDIM")]
    Medim,
    /// Mammography
    #[serde(rename = "MG")]
    Mg,
    /// Male Pseudohermaphrodite
    #[serde(rename = "MP")]
    Mp,
    /// Magnetic Resonance
    #[serde(rename = "MR")]
    Mr,
    /// Magnetic resonance spectroscopy
    #[serde(rename = "MS")]
    Ms,
    /// Nearline
    #[serde(rename = "NEARLINE")]
    Nearline,
    /// Nuclear Medicine
    #[serde(rename = "NM")]
    Nm,
    /// Ophthalmic Axial Measurements
    #[serde(rename = "OAM")]
    Oam,
    /// Optical Coherence Tomography
    #[serde(rename = "OCT")]
    Oct,
    /// Offline
    #[serde(rename = "OFFLINE")]
    Offline,
    /// Online
    #[serde(rename = "ONLINE")]
    Online,
    /// Ophthalmic photography
    #[serde(rename = "OP")]
    Op,
    /// Ophthalmic Mapping
    #[serde(rename = "OPM")]
    Opm,
    /// Ophthalmic Refraction
    #[serde(rename = "OPR")]
    Opr,
    /// Ophthalmic Tomography
    #[serde(rename = "OPT")]
    Opt,
    /// Ophthalmic Visual Field
    #[serde(rename = "OPV")]
    Opv,
    /// Other Modality
    #[serde(rename = "OT")]
    Ot,
    /// Presentation State
    #[serde(rename = "PR")]
    Pr,
    /// Hard Copy Print Server
    #[serde(rename = "PRINT")]
    Print,
    /// Positron emission tomography
    #[serde(rename = "PT")]
    Pt,
    /// Panoramic X-Ray
    #[serde(rename = "PX")]
    Px,
    /// Registration
    #[serde(rename = "REG")]
    Reg,
    /// Radiofluoroscopy
    #[serde(rename = "RF")]
    Rf,
    /// Radiographic imaging
    #[serde(rename = "RG")]
    Rg,
    /// Radiation Therapy Device
    #[serde(rename = "RT")]
    Rt,
    /// Radiotherapy Dose
    #[serde(rename = "RTDOSE")]
    Rtdose,
    /// Radiotherapy Image
    #[serde(rename = "RTIMAGE")]
    Rtimage,
    /// Radiotherapy Plan
    #[serde(rename = "RTPLAN")]
    Rtplan,
    /// Radiotherapy Treatment Record
    #[serde(rename = "RTRECORD")]
    Rtrecord,
    /// Radiotherapy Structure Set
    #[serde(rename = "RTSTRUCT")]
    Rtstruct,
    /// Segmentation
    #[serde(rename = "SEG")]
    Seg,
    /// Slide Microscopy
    #[serde(rename = "SM")]
    Sm,
    /// Stereometric Relationship
    #[serde(rename = "SMR")]
    Smr,
    /// Structured Report Document
    #[serde(rename = "SR")]
    Sr,
    /// Subjective Refraction
    #[serde(rename = "SRF")]
    Srf,
    /// Single-photon emission computed tomography
    #[serde(rename = "ST")]
    St,
    /// Thermography
    #[serde(rename = "TG")]
    Tg,
    /// Unknown Sex
    #[serde(rename = "U")]
    U,
    /// Unavailable
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
    /// Ultrasound
    #[serde(rename = "US")]
    Us,
    /// Visual Acuity
    #[serde(rename = "VA")]
    Va,
    /// Videofluorography
    #[serde(rename = "VF")]
    Vf,
    /// Video Tape Digitizer Equipment
    #[serde(rename = "VIDD")]
    Vidd,
    /// Workstation
    #[serde(rename = "WSD")]
    Wsd,
    /// X-Ray Angiography
    #[serde(rename = "XA")]
    Xa,
    /// External-camera Photography
    #[serde(rename = "XC")]
    Xc,
    /// Digital timecode (NOS)
    #[serde(rename = "109001")]
    N109001,
    /// ECG-based gating signal, processed
    #[serde(rename = "109002")]
    N109002,
    /// IRIG-B timecode
    #[serde(rename = "109003")]
    N109003,
    /// X-Ray Fluoroscopy On Signal
    #[serde(rename = "109004")]
    N109004,
    /// X-Ray On Trigger
    #[serde(rename = "109005")]
    N109005,
    /// Differential signal
    #[serde(rename = "109006")]
    N109006,
    /// His bundle electrogram
    #[serde(rename = "109007")]
    N109007,
    /// Monopole signal
    #[serde(rename = "109008")]
    N109008,
    /// Pacing (electrical) stimulus, voltage
    #[serde(rename = "109009")]
    N109009,
    /// Radio frequency ablation, power
    #[serde(rename = "109010")]
    N109010,
    /// Voltage measurement by basket catheter
    #[serde(rename = "109011")]
    N109011,
    /// Voltage measurement by mapping catheter
    #[serde(rename = "109012")]
    N109012,
    /// Voltage measurement, NOS
    #[serde(rename = "109013")]
    N109013,
    /// 35% of thermal CO
    #[serde(rename = "109014")]
    N109014,
    /// 70% of thermal CO
    #[serde(rename = "109015")]
    N109015,
    /// A wave peak pressure
    #[serde(rename = "109016")]
    N109016,
    /// A wave pressure, average
    #[serde(rename = "109017")]
    N109017,
    /// Beat detected (accepted)
    #[serde(rename = "109018")]
    N109018,
    /// Beat detected (rejected)
    #[serde(rename = "109019")]
    N109019,
    /// Diastolic pressure, average
    #[serde(rename = "109020")]
    N109020,
    /// Diastolic pressure nadir
    #[serde(rename = "109021")]
    N109021,
    /// End diastole
    #[serde(rename = "109022")]
    N109022,
    /// End of expiration
    #[serde(rename = "109023")]
    N109023,
    /// End of inspiration
    #[serde(rename = "109024")]
    N109024,
    /// Max dp/dt
    #[serde(rename = "109025")]
    N109025,
    /// Max neg dp/dt
    #[serde(rename = "109026")]
    N109026,
    /// Mean blood pressure
    #[serde(rename = "109027")]
    N109027,
    /// Peak of thermal cardiac output bolus
    #[serde(rename = "109028")]
    N109028,
    /// Start of expiration
    #[serde(rename = "109029")]
    N109029,
    /// Start of inspiration
    #[serde(rename = "109030")]
    N109030,
    /// Start of thermal cardiac output bolus
    #[serde(rename = "109031")]
    N109031,
    /// Systolic pressure, average
    #[serde(rename = "109032")]
    N109032,
    /// Systolic peak pressure
    #[serde(rename = "109033")]
    N109033,
    /// V wave peak pressure
    #[serde(rename = "109034")]
    N109034,
    /// V wave pressure, average
    #[serde(rename = "109035")]
    N109035,
    /// Valve close
    #[serde(rename = "109036")]
    N109036,
    /// Valve open
    #[serde(rename = "109037")]
    N109037,
    /// Ablation off
    #[serde(rename = "109038")]
    N109038,
    /// Ablation on
    #[serde(rename = "109039")]
    N109039,
    /// HIS bundle wave
    #[serde(rename = "109040")]
    N109040,
    /// P wave
    #[serde(rename = "109041")]
    N109041,
    /// Q wave
    #[serde(rename = "109042")]
    N109042,
    /// R wave
    #[serde(rename = "109043")]
    N109043,
    /// S wave
    #[serde(rename = "109044")]
    N109044,
    /// Start of atrial contraction
    #[serde(rename = "109045")]
    N109045,
    /// Start of atrial contraction (subsequent)
    #[serde(rename = "109046")]
    N109046,
    /// Stimulation at rate 1 interval
    #[serde(rename = "109047")]
    N109047,
    /// Stimulation at rate 2 interval
    #[serde(rename = "109048")]
    N109048,
    /// Stimulation at rate 3 interval
    #[serde(rename = "109049")]
    N109049,
    /// Stimulation at rate 4 interval
    #[serde(rename = "109050")]
    N109050,
    /// T wave
    #[serde(rename = "109051")]
    N109051,
    /// V wave
    #[serde(rename = "109052")]
    N109052,
    /// V wave of next beat
    #[serde(rename = "109053")]
    N109053,
    /// Patient State
    #[serde(rename = "109054")]
    N109054,
    /// Protocol Stage
    #[serde(rename = "109055")]
    N109055,
    /// Stress Protocol
    #[serde(rename = "109056")]
    N109056,
    /// Catheterization Procedure Phase
    #[serde(rename = "109057")]
    N109057,
    /// Contrast Phase
    #[serde(rename = "109058")]
    N109058,
    /// Physiological challenges
    #[serde(rename = "109059")]
    N109059,
    /// Procedure Step Number
    #[serde(rename = "109060")]
    N109060,
    /// EP Procedure Phase
    #[serde(rename = "109061")]
    N109061,
    /// Pulse train definition
    #[serde(rename = "109063")]
    N109063,
    /// End of systole
    #[serde(rename = "109070")]
    N109070,
    /// Indicator mean transit time
    #[serde(rename = "109071")]
    N109071,
    /// Tau
    #[serde(rename = "109072")]
    N109072,
    /// V max myocardial
    #[serde(rename = "109073")]
    N109073,
    /// Real time acquisition
    #[serde(rename = "109080")]
    N109080,
    /// Prospective gating
    #[serde(rename = "109081")]
    N109081,
    /// Retrospective gating
    #[serde(rename = "109082")]
    N109082,
    /// Paced
    #[serde(rename = "109083")]
    N109083,
    /// Cardiac Stress State
    #[serde(rename = "109091")]
    N109091,
    /// Reinjection State
    #[serde(rename = "109092")]
    N109092,
    /// Redistribution State
    #[serde(rename = "109093")]
    N109093,
    /// Delayed Redistribution State
    #[serde(rename = "109094")]
    N109094,
    /// Peak stress state
    #[serde(rename = "109095")]
    N109095,
    /// Recovery state
    #[serde(rename = "109096")]
    N109096,
    /// Acquisition Equipment
    #[serde(rename = "109101")]
    N109101,
    /// Processing Equipment
    #[serde(rename = "109102")]
    N109102,
    /// Modifying Equipment
    #[serde(rename = "109103")]
    N109103,
    /// De-identifying Equipment
    #[serde(rename = "109104")]
    N109104,
    /// Frame Extracting Equipment
    #[serde(rename = "109105")]
    N109105,
    /// Enhanced Multi-frame Conversion Equipment
    #[serde(rename = "109106")]
    N109106,
    /// Voice
    #[serde(rename = "109110")]
    N109110,
    /// Operator's narrative
    #[serde(rename = "109111")]
    N109111,
    /// Ambient room environment
    #[serde(rename = "109112")]
    N109112,
    /// Doppler audio
    #[serde(rename = "109113")]
    N109113,
    /// Phonocardiogram
    #[serde(rename = "109114")]
    N109114,
    /// Physiological audio signal
    #[serde(rename = "109115")]
    N109115,
    /// Arterial Pulse Waveform
    #[serde(rename = "109116")]
    N109116,
    /// Respiration Waveform
    #[serde(rename = "109117")]
    N109117,
    /// On admission to unit
    #[serde(rename = "109120")]
    N109120,
    /// On discharge
    #[serde(rename = "109121")]
    N109121,
    /// On discharge from unit
    #[serde(rename = "109122")]
    N109122,
    /// Pre-intervention
    #[serde(rename = "109123")]
    N109123,
    /// Post-intervention
    #[serde(rename = "109124")]
    N109124,
    /// At last appointment
    #[serde(rename = "109125")]
    N109125,
    /// Joint position method
    #[serde(rename = "109132")]
    N109132,
    /// Physical force
    #[serde(rename = "109133")]
    N109133,
    /// Prior to voiding
    #[serde(rename = "109134")]
    N109134,
    /// Post voiding
    #[serde(rename = "109135")]
    N109135,
    /// Neutral musculoskeletal position
    #[serde(rename = "109136")]
    N109136,
    /// America Kennel Club
    #[serde(rename = "109200")]
    N109200,
    /// America's Pet Registry Inc.
    #[serde(rename = "109201")]
    N109201,
    /// American Canine Association
    #[serde(rename = "109202")]
    N109202,
    /// American Purebred Registry
    #[serde(rename = "109203")]
    N109203,
    /// American Rare Breed Association
    #[serde(rename = "109204")]
    N109204,
    /// Animal Registry Unlimited
    #[serde(rename = "109205")]
    N109205,
    /// Animal Research Foundation
    #[serde(rename = "109206")]
    N109206,
    /// Canadian Border Collie Association
    #[serde(rename = "109207")]
    N109207,
    /// Canadian Kennel Club
    #[serde(rename = "109208")]
    N109208,
    /// Canadian Livestock Records Association
    #[serde(rename = "109209")]
    N109209,
    /// Canine Federation of Canada
    #[serde(rename = "109210")]
    N109210,
    /// Continental Kennel Club
    #[serde(rename = "109211")]
    N109211,
    /// Dog Registry of America
    #[serde(rename = "109212")]
    N109212,
    /// Federation of International Canines
    #[serde(rename = "109213")]
    N109213,
    /// International Progressive Dog Breeders' Alliance
    #[serde(rename = "109214")]
    N109214,
    /// National Kennel Club
    #[serde(rename = "109215")]
    N109215,
    /// North American Purebred Dog Registry
    #[serde(rename = "109216")]
    N109216,
    /// United All Breed Registry
    #[serde(rename = "109217")]
    N109217,
    /// United Kennel Club
    #[serde(rename = "109218")]
    N109218,
    /// Universal Kennel Club International
    #[serde(rename = "109219")]
    N109219,
    /// Working Canine Association of Canada
    #[serde(rename = "109220")]
    N109220,
    /// World Kennel Club
    #[serde(rename = "109221")]
    N109221,
    /// World Wide Kennel Club
    #[serde(rename = "109222")]
    N109222,
    /// Overall image quality evaluation
    #[serde(rename = "109701")]
    N109701,
    /// Grayscale resolution evaluation
    #[serde(rename = "109702")]
    N109702,
    /// Luminance response evaluation
    #[serde(rename = "109703")]
    N109703,
    /// Luminance uniformity evaluation
    #[serde(rename = "109704")]
    N109704,
    /// Chromaticity evaluation
    #[serde(rename = "109705")]
    N109705,
    /// Pixel faults evaluation
    #[serde(rename = "109706")]
    N109706,
    /// Veiling glare evaluation
    #[serde(rename = "109707")]
    N109707,
    /// Geometrical image evaluation
    #[serde(rename = "109708")]
    N109708,
    /// Angular viewing evaluation
    #[serde(rename = "109709")]
    N109709,
    /// Clinical evaluation
    #[serde(rename = "109710")]
    N109710,
    /// TG18-QC Pattern
    #[serde(rename = "109801")]
    N109801,
    /// TG18-BR Pattern
    #[serde(rename = "109802")]
    N109802,
    /// TG18-PQC Pattern
    #[serde(rename = "109803")]
    N109803,
    /// TG18-CT Pattern
    #[serde(rename = "109804")]
    N109804,
    /// TG18-LN8-01 Pattern
    #[serde(rename = "109805")]
    N109805,
    /// TG18-LN8-02 Pattern
    #[serde(rename = "109806")]
    N109806,
    /// TG18-LN8-03 Pattern
    #[serde(rename = "109807")]
    N109807,
    /// TG18-LN8-04 Pattern
    #[serde(rename = "109808")]
    N109808,
    /// TG18-LN8-05 Pattern
    #[serde(rename = "109809")]
    N109809,
    /// TG18-LN8-06 Pattern
    #[serde(rename = "109810")]
    N109810,
    /// TG18-LN8-07 Pattern
    #[serde(rename = "109811")]
    N109811,
    /// TG18-LN8-08 Pattern
    #[serde(rename = "109812")]
    N109812,
    /// TG18-LN8-09 Pattern
    #[serde(rename = "109813")]
    N109813,
    /// TG18-LN8-10 Pattern
    #[serde(rename = "109814")]
    N109814,
    /// TG18-LN8-11 Pattern
    #[serde(rename = "109815")]
    N109815,
    /// TG18-LN8-12 Pattern
    #[serde(rename = "109816")]
    N109816,
    /// TG18-LN8-13 Pattern
    #[serde(rename = "109817")]
    N109817,
    /// TG18-LN8-14 Pattern
    #[serde(rename = "109818")]
    N109818,
    /// TG18-LN8-15 Pattern
    #[serde(rename = "109819")]
    N109819,
    /// TG18-LN8-16 Pattern
    #[serde(rename = "109820")]
    N109820,
    /// TG18-LN8-17 Pattern
    #[serde(rename = "109821")]
    N109821,
    /// TG18-LN8-18 Pattern
    #[serde(rename = "109822")]
    N109822,
    /// TG18-LN12-01 Pattern
    #[serde(rename = "109823")]
    N109823,
    /// TG18-LN12-02 Pattern
    #[serde(rename = "109824")]
    N109824,
    /// TG18-LN12-03 Pattern
    #[serde(rename = "109825")]
    N109825,
    /// TG18-LN12-04 Pattern
    #[serde(rename = "109826")]
    N109826,
    /// TG18-LN12-05 Pattern
    #[serde(rename = "109827")]
    N109827,
    /// TG18-LN12-06 Pattern
    #[serde(rename = "109828")]
    N109828,
    /// TG18-LN12-07 Pattern
    #[serde(rename = "109829")]
    N109829,
    /// TG18-LN12-08 Pattern
    #[serde(rename = "109830")]
    N109830,
    /// TG18-LN12-09 Pattern
    #[serde(rename = "109831")]
    N109831,
    /// TG18-LN12-10 Pattern
    #[serde(rename = "109832")]
    N109832,
    /// TG18-LN12-11 Pattern
    #[serde(rename = "109833")]
    N109833,
    /// TG18-LN12-12 Pattern
    #[serde(rename = "109834")]
    N109834,
    /// TG18-LN12-13 Pattern
    #[serde(rename = "109835")]
    N109835,
    /// TG18-LN12-14 Pattern
    #[serde(rename = "109836")]
    N109836,
    /// TG18-LN12-15 Pattern
    #[serde(rename = "109837")]
    N109837,
    /// TG18-LN12-16 Pattern
    #[serde(rename = "109838")]
    N109838,
    /// TG18-LN12-17 Pattern
    #[serde(rename = "109839")]
    N109839,
    /// TG18-LN12-18 Pattern
    #[serde(rename = "109840")]
    N109840,
    /// TG18-UN10 Pattern
    #[serde(rename = "109841")]
    N109841,
    /// TG18-UN80 Pattern
    #[serde(rename = "109842")]
    N109842,
    /// TG18-UNL10 Pattern
    #[serde(rename = "109843")]
    N109843,
    /// TG18-UNL80 Pattern
    #[serde(rename = "109844")]
    N109844,
    /// TG18-AD Pattern
    #[serde(rename = "109845")]
    N109845,
    /// TG18-MP Pattern
    #[serde(rename = "109846")]
    N109846,
    /// TG18-RH10 Pattern
    #[serde(rename = "109847")]
    N109847,
    /// TG18-RH50 Pattern
    #[serde(rename = "109848")]
    N109848,
    /// TG18-RH89 Pattern
    #[serde(rename = "109849")]
    N109849,
    /// TG18-RV10 Pattern
    #[serde(rename = "109850")]
    N109850,
    /// TG18-RV50 Pattern
    #[serde(rename = "109851")]
    N109851,
    /// TG18-RV89 Pattern
    #[serde(rename = "109852")]
    N109852,
    /// TG18-PX Pattern
    #[serde(rename = "109853")]
    N109853,
    /// TG18-CX Pattern
    #[serde(rename = "109854")]
    N109854,
    /// TG18-LPH10 Pattern
    #[serde(rename = "109855")]
    N109855,
    /// TG18-LPH50 Pattern
    #[serde(rename = "109856")]
    N109856,
    /// TG18-LPH89 Pattern
    #[serde(rename = "109857")]
    N109857,
    /// TG18-LPV10 Pattern
    #[serde(rename = "109858")]
    N109858,
    /// TG18-LPV50 Pattern
    #[serde(rename = "109859")]
    N109859,
    /// TG18-LPV89 Pattern
    #[serde(rename = "109860")]
    N109860,
    /// TG18-AFC Pattern
    #[serde(rename = "109861")]
    N109861,
    /// TG18-NS10 Pattern
    #[serde(rename = "109862")]
    N109862,
    /// TG18-NS50 Pattern
    #[serde(rename = "109863")]
    N109863,
    /// TG18-NS89 Pattern
    #[serde(rename = "109864")]
    N109864,
    /// TG18-GV Pattern
    #[serde(rename = "109865")]
    N109865,
    /// TG18-GVN Pattern
    #[serde(rename = "109866")]
    N109866,
    /// TG18-GQ Pattern
    #[serde(rename = "109867")]
    N109867,
    /// TG18-GQN Pattern
    #[serde(rename = "109868")]
    N109868,
    /// TG18-GQB Pattern
    #[serde(rename = "109869")]
    N109869,
    /// TG18-GA03 Pattern
    #[serde(rename = "109870")]
    N109870,
    /// TG18-GA05 Pattern
    #[serde(rename = "109871")]
    N109871,
    /// TG18-GA08 Pattern
    #[serde(rename = "109872")]
    N109872,
    /// TG18-GA10 Pattern
    #[serde(rename = "109873")]
    N109873,
    /// TG18-GA15 Pattern
    #[serde(rename = "109874")]
    N109874,
    /// TG18-GA20 Pattern
    #[serde(rename = "109875")]
    N109875,
    /// TG18-GA25 Pattern
    #[serde(rename = "109876")]
    N109876,
    /// TG18-GA30 Pattern
    #[serde(rename = "109877")]
    N109877,
    /// TG18-CH Image
    #[serde(rename = "109878")]
    N109878,
    /// TG18-KN Image
    #[serde(rename = "109879")]
    N109879,
    /// TG18-MM1 Image
    #[serde(rename = "109880")]
    N109880,
    /// TG18-MM2 Image
    #[serde(rename = "109881")]
    N109881,
    /// OIQ Pattern
    #[serde(rename = "109901")]
    N109901,
    /// ANG Pattern
    #[serde(rename = "109902")]
    N109902,
    /// GD Pattern
    #[serde(rename = "109903")]
    N109903,
    /// BN01 Pattern
    #[serde(rename = "109904")]
    N109904,
    /// BN02 Pattern
    #[serde(rename = "109905")]
    N109905,
    /// BN03 Pattern
    #[serde(rename = "109906")]
    N109906,
    /// BN04 Pattern
    #[serde(rename = "109907")]
    N109907,
    /// BN05 Pattern
    #[serde(rename = "109908")]
    N109908,
    /// BN06 Pattern
    #[serde(rename = "109909")]
    N109909,
    /// BN07 Pattern
    #[serde(rename = "109910")]
    N109910,
    /// BN08 Pattern
    #[serde(rename = "109911")]
    N109911,
    /// BN09 Pattern
    #[serde(rename = "109912")]
    N109912,
    /// BN10 Pattern
    #[serde(rename = "109913")]
    N109913,
    /// BN11 Pattern
    #[serde(rename = "109914")]
    N109914,
    /// BN12 Pattern
    #[serde(rename = "109915")]
    N109915,
    /// BN13 Pattern
    #[serde(rename = "109916")]
    N109916,
    /// BN14 Pattern
    #[serde(rename = "109917")]
    N109917,
    /// BN15 Pattern
    #[serde(rename = "109918")]
    N109918,
    /// BN16 Pattern
    #[serde(rename = "109919")]
    N109919,
    /// BN17 Pattern
    #[serde(rename = "109920")]
    N109920,
    /// BN18 Pattern
    #[serde(rename = "109921")]
    N109921,
    /// DIN Grayscale Pattern
    #[serde(rename = "109931")]
    N109931,
    /// DIN Geometry Pattern
    #[serde(rename = "109932")]
    N109932,
    /// DIN Resolution Pattern
    #[serde(rename = "109933")]
    N109933,
    /// White Pattern
    #[serde(rename = "109941")]
    N109941,
    /// SMPTE Pattern
    #[serde(rename = "109943")]
    N109943,
    /// CRT Display
    #[serde(rename = "109991")]
    N109991,
    /// Liquid Crystal Display
    #[serde(rename = "109992")]
    N109992,
    /// Plasma Display
    #[serde(rename = "109993")]
    N109993,
    /// OLED
    #[serde(rename = "109994")]
    N109994,
    /// DLP Rear Projection System
    #[serde(rename = "109995")]
    N109995,
    /// DLP Front Projection System
    #[serde(rename = "109996")]
    N109996,
    /// CRT Rear Projection System
    #[serde(rename = "109997")]
    N109997,
    /// CRT Front Projection System
    #[serde(rename = "109998")]
    N109998,
    /// Other Projection System
    #[serde(rename = "109999")]
    N109999,
    /// Image Processing
    #[serde(rename = "110001")]
    N110001,
    /// Quality Control
    #[serde(rename = "110002")]
    N110002,
    /// Computer Aided Diagnosis
    #[serde(rename = "110003")]
    N110003,
    /// Computer Aided Detection
    #[serde(rename = "110004")]
    N110004,
    /// Interpretation
    #[serde(rename = "110005")]
    N110005,
    /// Transcription
    #[serde(rename = "110006")]
    N110006,
    /// Report Verification
    #[serde(rename = "110007")]
    N110007,
    /// Print
    #[serde(rename = "110008")]
    N110008,
    /// No subsequent Workitems
    #[serde(rename = "110009")]
    N110009,
    /// Film
    #[serde(rename = "110010")]
    N110010,
    /// Dictation
    #[serde(rename = "110011")]
    N110011,
    /// Transcription
    #[serde(rename = "110012")]
    N110012,
    /// Media Import
    #[serde(rename = "110013")]
    N110013,
    /// Sheet Film Digitized
    #[serde(rename = "110020")]
    N110020,
    /// Cine Film Digitized
    #[serde(rename = "110021")]
    N110021,
    /// Video Tape Digitized
    #[serde(rename = "110022")]
    N110022,
    /// Paper Digitized
    #[serde(rename = "110023")]
    N110023,
    /// CD Imported
    #[serde(rename = "110024")]
    N110024,
    /// DVD Imported
    #[serde(rename = "110025")]
    N110025,
    /// MOD Imported
    #[serde(rename = "110026")]
    N110026,
    /// Studies Imported
    #[serde(rename = "110027")]
    N110027,
    /// Instances Imported
    #[serde(rename = "110028")]
    N110028,
    /// USB Disk Emulation
    #[serde(rename = "110030")]
    N110030,
    /// Email
    #[serde(rename = "110031")]
    N110031,
    /// CD
    #[serde(rename = "110032")]
    N110032,
    /// DVD
    #[serde(rename = "110033")]
    N110033,
    /// Compact Flash
    #[serde(rename = "110034")]
    N110034,
    /// Multi-media Card
    #[serde(rename = "110035")]
    N110035,
    /// Secure Digital Card
    #[serde(rename = "110036")]
    N110036,
    /// URI
    #[serde(rename = "110037")]
    N110037,
    /// Paper Document
    #[serde(rename = "110038")]
    N110038,
    /// Application Activity
    #[serde(rename = "110100")]
    N110100,
    /// Audit Log Used
    #[serde(rename = "110101")]
    N110101,
    /// Begin Transferring DICOM Instances
    #[serde(rename = "110102")]
    N110102,
    /// DICOM Instances Accessed
    #[serde(rename = "110103")]
    N110103,
    /// DICOM Instances Transferred
    #[serde(rename = "110104")]
    N110104,
    /// DICOM Study Deleted
    #[serde(rename = "110105")]
    N110105,
    /// Export
    #[serde(rename = "110106")]
    N110106,
    /// Import
    #[serde(rename = "110107")]
    N110107,
    /// Network Entry
    #[serde(rename = "110108")]
    N110108,
    /// Order Record
    #[serde(rename = "110109")]
    N110109,
    /// Patient Record
    #[serde(rename = "110110")]
    N110110,
    /// Procedure Record
    #[serde(rename = "110111")]
    N110111,
    /// Query
    #[serde(rename = "110112")]
    N110112,
    /// Security Alert
    #[serde(rename = "110113")]
    N110113,
    /// User Authentication
    #[serde(rename = "110114")]
    N110114,
    /// Application Start
    #[serde(rename = "110120")]
    N110120,
    /// Application Stop
    #[serde(rename = "110121")]
    N110121,
    /// Login
    #[serde(rename = "110122")]
    N110122,
    /// Logout
    #[serde(rename = "110123")]
    N110123,
    /// Attach
    #[serde(rename = "110124")]
    N110124,
    /// Detach
    #[serde(rename = "110125")]
    N110125,
    /// Node Authentication
    #[serde(rename = "110126")]
    N110126,
    /// Emergency Override Started
    #[serde(rename = "110127")]
    N110127,
    /// Network Configuration
    #[serde(rename = "110128")]
    N110128,
    /// Security Configuration
    #[serde(rename = "110129")]
    N110129,
    /// Hardware Configuration
    #[serde(rename = "110130")]
    N110130,
    /// Software Configuration
    #[serde(rename = "110131")]
    N110131,
    /// Use of Restricted Function
    #[serde(rename = "110132")]
    N110132,
    /// Audit Recording Stopped
    #[serde(rename = "110133")]
    N110133,
    /// Audit Recording Started
    #[serde(rename = "110134")]
    N110134,
    /// Object Security Attributes Changed
    #[serde(rename = "110135")]
    N110135,
    /// Security Roles Changed
    #[serde(rename = "110136")]
    N110136,
    /// User security Attributes Changed
    #[serde(rename = "110137")]
    N110137,
    /// Emergency Override Stopped
    #[serde(rename = "110138")]
    N110138,
    /// Remote Service Operation Started
    #[serde(rename = "110139")]
    N110139,
    /// Remote Service Operation Stopped
    #[serde(rename = "110140")]
    N110140,
    /// Local Service Operation Started
    #[serde(rename = "110141")]
    N110141,
    /// Local Service Operation Stopped
    #[serde(rename = "110142")]
    N110142,
    /// Application
    #[serde(rename = "110150")]
    N110150,
    /// Application Launcher
    #[serde(rename = "110151")]
    N110151,
    /// Destination Role ID
    #[serde(rename = "110152")]
    N110152,
    /// Source Role ID
    #[serde(rename = "110153")]
    N110153,
    /// Destination Media
    #[serde(rename = "110154")]
    N110154,
    /// Source Media
    #[serde(rename = "110155")]
    N110155,
    /// Study Instance UID
    #[serde(rename = "110180")]
    N110180,
    /// SOP Class UID
    #[serde(rename = "110181")]
    N110181,
    /// Node ID
    #[serde(rename = "110182")]
    N110182,
    /// Issuer of Identifier
    #[serde(rename = "110190")]
    N110190,
    /// Doctor canceled procedure
    #[serde(rename = "110500")]
    N110500,
    /// Equipment failure
    #[serde(rename = "110501")]
    N110501,
    /// Incorrect procedure ordered
    #[serde(rename = "110502")]
    N110502,
    /// Patient allergic to media/contrast
    #[serde(rename = "110503")]
    N110503,
    /// Patient died
    #[serde(rename = "110504")]
    N110504,
    /// Patient refused to continue procedure
    #[serde(rename = "110505")]
    N110505,
    /// Patient taken for treatment or surgery
    #[serde(rename = "110506")]
    N110506,
    /// Patient did not arrive
    #[serde(rename = "110507")]
    N110507,
    /// Patient pregnant
    #[serde(rename = "110508")]
    N110508,
    /// Change of procedure for correct charging
    #[serde(rename = "110509")]
    N110509,
    /// Duplicate order
    #[serde(rename = "110510")]
    N110510,
    /// Nursing unit cancel
    #[serde(rename = "110511")]
    N110511,
    /// Incorrect side ordered
    #[serde(rename = "110512")]
    N110512,
    /// Discontinued for unspecified reason
    #[serde(rename = "110513")]
    N110513,
    /// Incorrect worklist entry selected
    #[serde(rename = "110514")]
    N110514,
    /// Patient condition prevented continuing
    #[serde(rename = "110515")]
    N110515,
    /// Equipment change
    #[serde(rename = "110516")]
    N110516,
    /// Patient Movement
    #[serde(rename = "110518")]
    N110518,
    /// Operator Error
    #[serde(rename = "110519")]
    N110519,
    /// Objects incorrectly formatted
    #[serde(rename = "110521")]
    N110521,
    /// Object Types not supported
    #[serde(rename = "110522")]
    N110522,
    /// Object Set incomplete
    #[serde(rename = "110523")]
    N110523,
    /// Media Failure
    #[serde(rename = "110524")]
    N110524,
    /// Resource pre-empted
    #[serde(rename = "110526")]
    N110526,
    /// Resource inadequate
    #[serde(rename = "110527")]
    N110527,
    /// Discontinued Procedure Step rescheduled
    #[serde(rename = "110528")]
    N110528,
    /// Discontinued Procedure Step rescheduling recommended
    #[serde(rename = "110529")]
    N110529,
    /// Ventral Diencephalon
    #[serde(rename = "110700")]
    N110700,
    /// White Matter T1 Hypointensity
    #[serde(rename = "110701")]
    N110701,
    /// White Matter T2 Hyperintensity
    #[serde(rename = "110702")]
    N110702,
    /// superior longitudinal fasciculus I
    #[serde(rename = "110703")]
    N110703,
    /// superior longitudinal fasciculus II
    #[serde(rename = "110704")]
    N110704,
    /// superior longitudinal fasciculus III
    #[serde(rename = "110705")]
    N110705,
    /// Perilesional White Matter
    #[serde(rename = "110706")]
    N110706,
    /// Spin Tagging Perfusion MR Signal Intensity
    #[serde(rename = "110800")]
    N110800,
    /// Contrast Agent Angio MR Signal Intensity
    #[serde(rename = "110801")]
    N110801,
    /// Time Of Flight Angio MR Signal Intensity
    #[serde(rename = "110802")]
    N110802,
    /// Proton Density Weighted MR Signal Intensity
    #[serde(rename = "110803")]
    N110803,
    /// T1 Weighted MR Signal Intensity
    #[serde(rename = "110804")]
    N110804,
    /// T2 Weighted MR Signal Intensity
    #[serde(rename = "110805")]
    N110805,
    /// T2* Weighted MR Signal Intensity
    #[serde(rename = "110806")]
    N110806,
    /// Field Map MR Signal Intensity
    #[serde(rename = "110807")]
    N110807,
    /// Fractional Anisotropy
    #[serde(rename = "110808")]
    N110808,
    /// Relative Anisotropy
    #[serde(rename = "110809")]
    N110809,
    /// Volumetric Diffusion Dxx Component
    #[serde(rename = "110810")]
    N110810,
    /// Volumetric Diffusion Dxy Component
    #[serde(rename = "110811")]
    N110811,
    /// Volumetric Diffusion Dxz Component
    #[serde(rename = "110812")]
    N110812,
    /// Volumetric Diffusion Dyy Component
    #[serde(rename = "110813")]
    N110813,
    /// Volumetric Diffusion Dyz Component
    #[serde(rename = "110814")]
    N110814,
    /// Volumetric Diffusion Dzz Component
    #[serde(rename = "110815")]
    N110815,
    /// T1 Weighted Dynamic Contrast Enhanced MR Signal Intensity
    #[serde(rename = "110816")]
    N110816,
    /// T2 Weighted Dynamic Contrast Enhanced MR Signal Intensity
    #[serde(rename = "110817")]
    N110817,
    /// T2* Weighted Dynamic Contrast Enhanced MR Signal Intensity
    #[serde(rename = "110818")]
    N110818,
    /// Blood Oxygenation Level
    #[serde(rename = "110819")]
    N110819,
    /// Nuclear Medicine Projection Activity
    #[serde(rename = "110820")]
    N110820,
    /// Nuclear Medicine Tomographic Activity
    #[serde(rename = "110821")]
    N110821,
    /// Spatial Displacement X Component
    #[serde(rename = "110822")]
    N110822,
    /// Spatial Displacement Y Component
    #[serde(rename = "110823")]
    N110823,
    /// Spatial Displacement Z Component
    #[serde(rename = "110824")]
    N110824,
    /// Hemodynamic Resistance
    #[serde(rename = "110825")]
    N110825,
    /// Indexed Hemodynamic Resistance
    #[serde(rename = "110826")]
    N110826,
    /// Tissue Velocity
    #[serde(rename = "110827")]
    N110827,
    /// Flow Velocity
    #[serde(rename = "110828")]
    N110828,
    /// Flow Variance
    #[serde(rename = "110829")]
    N110829,
    /// Elasticity
    #[serde(rename = "110830")]
    N110830,
    /// Perfusion
    #[serde(rename = "110831")]
    N110831,
    /// Speed of sound
    #[serde(rename = "110832")]
    N110832,
    /// Ultrasound Attenuation
    #[serde(rename = "110833")]
    N110833,
    /// RGB R Component
    #[serde(rename = "110834")]
    N110834,
    /// RGB G Component
    #[serde(rename = "110835")]
    N110835,
    /// RGB B Component
    #[serde(rename = "110836")]
    N110836,
    /// YBR FULL Y Component
    #[serde(rename = "110837")]
    N110837,
    /// YBR FULL CB Component
    #[serde(rename = "110838")]
    N110838,
    /// YBR FULL CR Component
    #[serde(rename = "110839")]
    N110839,
    /// YBR PARTIAL Y Component
    #[serde(rename = "110840")]
    N110840,
    /// YBR PARTIAL CB Component
    #[serde(rename = "110841")]
    N110841,
    /// YBR PARTIAL CR Component
    #[serde(rename = "110842")]
    N110842,
    /// YBR ICT Y Component
    #[serde(rename = "110843")]
    N110843,
    /// YBR ICT CB Component
    #[serde(rename = "110844")]
    N110844,
    /// YBR ICT CR Component
    #[serde(rename = "110845")]
    N110845,
    /// YBR RCT Y Component
    #[serde(rename = "110846")]
    N110846,
    /// YBR RCT CB Component
    #[serde(rename = "110847")]
    N110847,
    /// YBR RCT CR Component
    #[serde(rename = "110848")]
    N110848,
    /// Echogenicity
    #[serde(rename = "110849")]
    N110849,
    /// X-Ray Attenuation
    #[serde(rename = "110850")]
    N110850,
    /// X-Ray Attenuation Coefficient
    #[serde(rename = "110851")]
    N110851,
    /// MR signal intensity
    #[serde(rename = "110852")]
    N110852,
    /// Binary Segmentation
    #[serde(rename = "110853")]
    N110853,
    /// Fractional Probabilistic Segmentation
    #[serde(rename = "110854")]
    N110854,
    /// Fractional Occupancy Segmentation
    #[serde(rename = "110855")]
    N110855,
    /// Linear Displacement
    #[serde(rename = "110856")]
    N110856,
    /// Photon Energy
    #[serde(rename = "110857")]
    N110857,
    /// Time
    #[serde(rename = "110858")]
    N110858,
    /// Angle
    #[serde(rename = "110859")]
    N110859,
    /// Left-Right Axis
    #[serde(rename = "110860")]
    N110860,
    /// Head-Foot Axis
    #[serde(rename = "110861")]
    N110861,
    /// Anterior-Posterior Axis
    #[serde(rename = "110862")]
    N110862,
    /// Apex-Base Axis
    #[serde(rename = "110863")]
    N110863,
    /// Anterior-Inferior Axis
    #[serde(rename = "110864")]
    N110864,
    /// Septum-Wall Axis
    #[serde(rename = "110865")]
    N110865,
    /// Right To Left
    #[serde(rename = "110866")]
    N110866,
    /// Left To Right
    #[serde(rename = "110867")]
    N110867,
    /// Head To Foot
    #[serde(rename = "110868")]
    N110868,
    /// Foot To Head
    #[serde(rename = "110869")]
    N110869,
    /// Anterior To Posterior
    #[serde(rename = "110870")]
    N110870,
    /// Posterior To Anterior
    #[serde(rename = "110871")]
    N110871,
    /// Apex To Base
    #[serde(rename = "110872")]
    N110872,
    /// Base To Apex
    #[serde(rename = "110873")]
    N110873,
    /// Anterior To Inferior
    #[serde(rename = "110874")]
    N110874,
    /// Inferior To Anterior
    #[serde(rename = "110875")]
    N110875,
    /// Septum To Wall
    #[serde(rename = "110876")]
    N110876,
    /// Wall To Septum
    #[serde(rename = "110877")]
    N110877,
    /// Image Position (Patient) X
    #[serde(rename = "110901")]
    N110901,
    /// Image Position (Patient) Y
    #[serde(rename = "110902")]
    N110902,
    /// Image Position (Patient) Z
    #[serde(rename = "110903")]
    N110903,
    /// Image Orientation (Patient) Row X
    #[serde(rename = "110904")]
    N110904,
    /// Image Orientation (Patient) Row Y
    #[serde(rename = "110905")]
    N110905,
    /// Image Orientation (Patient) Row Z
    #[serde(rename = "110906")]
    N110906,
    /// Image Orientation (Patient) Column X
    #[serde(rename = "110907")]
    N110907,
    /// Image Orientation (Patient) Column Y
    #[serde(rename = "110908")]
    N110908,
    /// Image Orientation (Patient) Column Z
    #[serde(rename = "110909")]
    N110909,
    /// Pixel Data Rows
    #[serde(rename = "110910")]
    N110910,
    /// Pixel Data Columns
    #[serde(rename = "110911")]
    N110911,
    /// Algorithm Name
    #[serde(rename = "111001")]
    N111001,
    /// Algorithm Parameters
    #[serde(rename = "111002")]
    N111002,
    /// Algorithm Version
    #[serde(rename = "111003")]
    N111003,
    /// Analysis Performed
    #[serde(rename = "111004")]
    N111004,
    /// Assessment Category
    #[serde(rename = "111005")]
    N111005,
    /// Breast composition
    #[serde(rename = "111006")]
    N111006,
    /// Breast Outline including Pectoral Muscle Tissue
    #[serde(rename = "111007")]
    N111007,
    /// Calcification Distribution
    #[serde(rename = "111008")]
    N111008,
    /// Calcification Type
    #[serde(rename = "111009")]
    N111009,
    /// Center
    #[serde(rename = "111010")]
    N111010,
    /// Certainty of Feature
    #[serde(rename = "111011")]
    N111011,
    /// Certainty of Finding
    #[serde(rename = "111012")]
    N111012,
    /// Certainty of Impression
    #[serde(rename = "111013")]
    N111013,
    /// Clockface or region
    #[serde(rename = "111014")]
    N111014,
    /// Composite Feature
    #[serde(rename = "111015")]
    N111015,
    /// Composite type
    #[serde(rename = "111016")]
    N111016,
    /// CAD Processing and Findings Summary
    #[serde(rename = "111017")]
    N111017,
    /// Content Date
    #[serde(rename = "111018")]
    N111018,
    /// Content Time
    #[serde(rename = "111019")]
    N111019,
    /// Depth
    #[serde(rename = "111020")]
    N111020,
    /// Description of Change
    #[serde(rename = "111021")]
    N111021,
    /// Detection Performed
    #[serde(rename = "111022")]
    N111022,
    /// Differential Diagnosis/Impression
    #[serde(rename = "111023")]
    N111023,
    /// Failed Analyses
    #[serde(rename = "111024")]
    N111024,
    /// Failed Detections
    #[serde(rename = "111025")]
    N111025,
    /// Horizontal Pixel Spacing
    #[serde(rename = "111026")]
    N111026,
    /// Image Laterality
    #[serde(rename = "111027")]
    N111027,
    /// Image Library
    #[serde(rename = "111028")]
    N111028,
    /// Image Quality Rating
    #[serde(rename = "111029")]
    N111029,
    /// Image Region
    #[serde(rename = "111030")]
    N111030,
    /// Image View
    #[serde(rename = "111031")]
    N111031,
    /// Image View Modifier
    #[serde(rename = "111032")]
    N111032,
    /// Impression Description
    #[serde(rename = "111033")]
    N111033,
    /// Individual Impression/Recommendation
    #[serde(rename = "111034")]
    N111034,
    /// Lesion Density
    #[serde(rename = "111035")]
    N111035,
    /// Mammography CAD Report
    #[serde(rename = "111036")]
    N111036,
    /// Margins
    #[serde(rename = "111037")]
    N111037,
    /// Number of calcifications
    #[serde(rename = "111038")]
    N111038,
    /// Object type
    #[serde(rename = "111039")]
    N111039,
    /// Original Source
    #[serde(rename = "111040")]
    N111040,
    /// Outline
    #[serde(rename = "111041")]
    N111041,
    /// Pathology
    #[serde(rename = "111042")]
    N111042,
    /// Patient Orientation Column
    #[serde(rename = "111043")]
    N111043,
    /// Patient Orientation Row
    #[serde(rename = "111044")]
    N111044,
    /// Pectoral Muscle Outline
    #[serde(rename = "111045")]
    N111045,
    /// Percent Fibroglandular Tissue
    #[serde(rename = "111046")]
    N111046,
    /// Probability of cancer
    #[serde(rename = "111047")]
    N111047,
    /// Quadrant location
    #[serde(rename = "111048")]
    N111048,
    /// Qualitative Difference
    #[serde(rename = "111049")]
    N111049,
    /// Quality Assessment
    #[serde(rename = "111050")]
    N111050,
    /// Quality Control Standard
    #[serde(rename = "111051")]
    N111051,
    /// Quality Finding
    #[serde(rename = "111052")]
    N111052,
    /// Recommended Follow-up
    #[serde(rename = "111053")]
    N111053,
    /// Recommended Follow-up Date
    #[serde(rename = "111054")]
    N111054,
    /// Recommended Follow-up Interval
    #[serde(rename = "111055")]
    N111055,
    /// Rendering Intent
    #[serde(rename = "111056")]
    N111056,
    /// Scope of Feature
    #[serde(rename = "111057")]
    N111057,
    /// Selected Region Description
    #[serde(rename = "111058")]
    N111058,
    /// Single Image Finding
    #[serde(rename = "111059")]
    N111059,
    /// Study Date
    #[serde(rename = "111060")]
    N111060,
    /// Study Time
    #[serde(rename = "111061")]
    N111061,
    /// Successful Analyses
    #[serde(rename = "111062")]
    N111062,
    /// Successful Detections
    #[serde(rename = "111063")]
    N111063,
    /// Summary of Detections
    #[serde(rename = "111064")]
    N111064,
    /// Summary of Analyses
    #[serde(rename = "111065")]
    N111065,
    /// Vertical Pixel Spacing
    #[serde(rename = "111066")]
    N111066,
    /// Crosstable
    #[serde(rename = "111069")]
    N111069,
    /// CAD Operating Point
    #[serde(rename = "111071")]
    N111071,
    /// Maximum CAD Operating Point
    #[serde(rename = "111072")]
    N111072,
    /// CAD Operating Point Description
    #[serde(rename = "111081")]
    N111081,
    /// False Markers per Image
    #[serde(rename = "111086")]
    N111086,
    /// False Markers per Case
    #[serde(rename = "111087")]
    N111087,
    /// Case Sensitivity
    #[serde(rename = "111088")]
    N111088,
    /// Lesion Sensitivity
    #[serde(rename = "111089")]
    N111089,
    /// Case Specificity
    #[serde(rename = "111090")]
    N111090,
    /// Image Specificity
    #[serde(rename = "111091")]
    N111091,
    /// Recommended CAD Operating Point
    #[serde(rename = "111092")]
    N111092,
    /// CAD Operating Point Table
    #[serde(rename = "111093")]
    N111093,
    /// Selected region
    #[serde(rename = "111099")]
    N111099,
    /// Breast geometry
    #[serde(rename = "111100")]
    N111100,
    /// Image Quality
    #[serde(rename = "111101")]
    N111101,
    /// Non-lesion
    #[serde(rename = "111102")]
    N111102,
    /// Density
    #[serde(rename = "111103")]
    N111103,
    /// Individual Calcification
    #[serde(rename = "111104")]
    N111104,
    /// Calcification Cluster
    #[serde(rename = "111105")]
    N111105,
    /// Cooper's ligament changes
    #[serde(rename = "111111")]
    N111111,
    /// Mass in the skin
    #[serde(rename = "111112")]
    N111112,
    /// Mass on the skin
    #[serde(rename = "111113")]
    N111113,
    /// Post Procedure Mammograms for Marker Placement
    #[serde(rename = "111120")]
    N111120,
    /// Follow-up post biopsy as directed by clinician
    #[serde(rename = "111121")]
    N111121,
    /// Known biopsy proven malignancy - take appropriate action
    #[serde(rename = "111122")]
    N111122,
    /// Marker placement
    #[serde(rename = "111123")]
    N111123,
    /// Personal history of breast cancer with mastectomy
    #[serde(rename = "111124")]
    N111124,
    /// Known biopsy proven malignancy
    #[serde(rename = "111125")]
    N111125,
    /// Image detected mass
    #[serde(rename = "111126")]
    N111126,
    /// Targeted
    #[serde(rename = "111127")]
    N111127,
    /// Survey
    #[serde(rename = "111128")]
    N111128,
    /// Clustered microcysts
    #[serde(rename = "111129")]
    N111129,
    /// Complicated cyst
    #[serde(rename = "111130")]
    N111130,
    /// Additional projections
    #[serde(rename = "111135")]
    N111135,
    /// Spot magnification view(s)
    #[serde(rename = "111136")]
    N111136,
    /// Ultrasound
    #[serde(rename = "111137")]
    N111137,
    /// Old films for comparison
    #[serde(rename = "111138")]
    N111138,
    /// Ductography
    #[serde(rename = "111139")]
    N111139,
    /// Normal interval follow-up
    #[serde(rename = "111140")]
    N111140,
    /// Any decision to biopsy should be based on clinical assessment
    #[serde(rename = "111141")]
    N111141,
    /// Follow-up at short interval (1-11 months)
    #[serde(rename = "111142")]
    N111142,
    /// Biopsy should be considered
    #[serde(rename = "111143")]
    N111143,
    /// Needle localization and biopsy
    #[serde(rename = "111144")]
    N111144,
    /// Histology using core biopsy
    #[serde(rename = "111145")]
    N111145,
    /// Suggestive of malignancy - take appropriate action
    #[serde(rename = "111146")]
    N111146,
    /// Cytologic analysis
    #[serde(rename = "111147")]
    N111147,
    /// Biopsy should be strongly considered
    #[serde(rename = "111148")]
    N111148,
    /// Highly suggestive of malignancy - take appropriate action
    #[serde(rename = "111149")]
    N111149,
    /// Presentation Required: Rendering device is expected to present
    #[serde(rename = "111150")]
    N111150,
    /// Presentation Optional: Rendering device may present
    #[serde(rename = "111151")]
    N111151,
    /// Not for Presentation: Rendering device expected not to present
    #[serde(rename = "111152")]
    N111152,
    /// Target content items are related temporally
    #[serde(rename = "111153")]
    N111153,
    /// Target content items are related spatially
    #[serde(rename = "111154")]
    N111154,
    /// Target content items are related contra-laterally
    #[serde(rename = "111155")]
    N111155,
    /// Feature detected on the only image
    #[serde(rename = "111156")]
    N111156,
    /// Feature detected on only one of the images
    #[serde(rename = "111157")]
    N111157,
    /// Feature detected on multiple images
    #[serde(rename = "111158")]
    N111158,
    /// Feature detected on images from multiple modalities
    #[serde(rename = "111159")]
    N111159,
    /// Scar tissue
    #[serde(rename = "111168")]
    N111168,
    /// J Wire
    #[serde(rename = "111170")]
    N111170,
    /// Pacemaker
    #[serde(rename = "111171")]
    N111171,
    /// Paddle
    #[serde(rename = "111172")]
    N111172,
    /// Collimator
    #[serde(rename = "111173")]
    N111173,
    /// ID Plate
    #[serde(rename = "111174")]
    N111174,
    /// Other Marker
    #[serde(rename = "111175")]
    N111175,
    /// Unspecified
    #[serde(rename = "111176")]
    N111176,
    /// View and Laterality Marker is missing
    #[serde(rename = "111177")]
    N111177,
    /// View and Laterality Marker does not have both view and laterality
    #[serde(rename = "111178")]
    N111178,
    /// View and Laterality Marker does not have approved codes
    #[serde(rename = "111179")]
    N111179,
    /// View and Laterality Marker is not near the axilla
    #[serde(rename = "111180")]
    N111180,
    /// View and Laterality Marker overlaps breast tissue
    #[serde(rename = "111181")]
    N111181,
    /// View and Laterality Marker is partially obscured
    #[serde(rename = "111182")]
    N111182,
    /// View and Laterality Marker is incorrect
    #[serde(rename = "111183")]
    N111183,
    /// View and Laterality Marker is off image
    #[serde(rename = "111184")]
    N111184,
    /// Flash is not near edge of film
    #[serde(rename = "111185")]
    N111185,
    /// Flash is illigible, does not fit, or is lopsided
    #[serde(rename = "111186")]
    N111186,
    /// Flash doesn't include patient name and additional patient id
    #[serde(rename = "111187")]
    N111187,
    /// Flash doesn't include date of examination
    #[serde(rename = "111188")]
    N111188,
    /// Flash doesn't include facility name and location
    #[serde(rename = "111189")]
    N111189,
    /// Flash doesn't include technologist identification
    #[serde(rename = "111190")]
    N111190,
    /// Flash doesn't include cassette/screen/detector identification
    #[serde(rename = "111191")]
    N111191,
    /// Flash doesn't include mammography unit identification
    #[serde(rename = "111192")]
    N111192,
    /// Date sticker is missing
    #[serde(rename = "111193")]
    N111193,
    /// Technical factors missing
    #[serde(rename = "111194")]
    N111194,
    /// Collimation too close to breast
    #[serde(rename = "111195")]
    N111195,
    /// Inadequate compression
    #[serde(rename = "111196")]
    N111196,
    /// MLO Insufficient pectoral muscle
    #[serde(rename = "111197")]
    N111197,
    /// MLO No fat is visualized posterior to fibroglandular tissues
    #[serde(rename = "111198")]
    N111198,
    /// MLO Poor separation of deep and superficial breast tissues
    #[serde(rename = "111199")]
    N111199,
    /// MLO Evidence of motion blur
    #[serde(rename = "111200")]
    N111200,
    /// MLO Inframammary fold is not open
    #[serde(rename = "111201")]
    N111201,
    /// CC Not all medial tissue visualized
    #[serde(rename = "111202")]
    N111202,
    /// CC Nipple not centered on image
    #[serde(rename = "111203")]
    N111203,
    /// CC Posterior nipple line does not measure within 1 cm of MLO
    #[serde(rename = "111204")]
    N111204,
    /// Nipple not in profile
    #[serde(rename = "111205")]
    N111205,
    /// Insufficient implant displacement incorrect
    #[serde(rename = "111206")]
    N111206,
    /// Image artifact(s)
    #[serde(rename = "111207")]
    N111207,
    /// Grid artifact(s)
    #[serde(rename = "111208")]
    N111208,
    /// Positioning
    #[serde(rename = "111209")]
    N111209,
    /// Motion blur
    #[serde(rename = "111210")]
    N111210,
    /// Under exposed
    #[serde(rename = "111211")]
    N111211,
    /// Over exposed
    #[serde(rename = "111212")]
    N111212,
    /// No image
    #[serde(rename = "111213")]
    N111213,
    /// Detector artifact(s)
    #[serde(rename = "111214")]
    N111214,
    /// Artifact(s) other than grid or detector artifact
    #[serde(rename = "111215")]
    N111215,
    /// Mechanical failure
    #[serde(rename = "111216")]
    N111216,
    /// Electrical failure
    #[serde(rename = "111217")]
    N111217,
    /// Software failure
    #[serde(rename = "111218")]
    N111218,
    /// Inappropriate image processing
    #[serde(rename = "111219")]
    N111219,
    /// Other failure
    #[serde(rename = "111220")]
    N111220,
    /// Unknown failure
    #[serde(rename = "111221")]
    N111221,
    /// Succeeded
    #[serde(rename = "111222")]
    N111222,
    /// Partially Succeeded
    #[serde(rename = "111223")]
    N111223,
    /// Failed
    #[serde(rename = "111224")]
    N111224,
    /// Not Attempted
    #[serde(rename = "111225")]
    N111225,
    /// Individual Impression / Recommendation Analysis
    #[serde(rename = "111233")]
    N111233,
    /// Overall Impression / Recommendation Analysis
    #[serde(rename = "111234")]
    N111234,
    /// Unusable - Quality renders image unusable
    #[serde(rename = "111235")]
    N111235,
    /// Usable - Does not meet the quality control standard
    #[serde(rename = "111236")]
    N111236,
    /// Usable - Meets the quality control standard
    #[serde(rename = "111237")]
    N111237,
    /// Mammography Quality Control Manual 1999, ACR
    #[serde(rename = "111238")]
    N111238,
    /// Title 21 CFR Section 900, Subpart B
    #[serde(rename = "111239")]
    N111239,
    /// Institutionally defined quality control standard
    #[serde(rename = "111240")]
    N111240,
    /// All algorithms succeeded; without findings
    #[serde(rename = "111241")]
    N111241,
    /// All algorithms succeeded; with findings
    #[serde(rename = "111242")]
    N111242,
    /// Not all algorithms succeeded; without findings
    #[serde(rename = "111243")]
    N111243,
    /// Not all algorithms succeeded; with findings
    #[serde(rename = "111244")]
    N111244,
    /// No algorithms succeeded; without findings
    #[serde(rename = "111245")]
    N111245,
    /// Adenolipoma
    #[serde(rename = "111248")]
    N111248,
    /// Ductal hyperplasia
    #[serde(rename = "111249")]
    N111249,
    /// Adenomyoepithelioma
    #[serde(rename = "111250")]
    N111250,
    /// Normal axillary node
    #[serde(rename = "111251")]
    N111251,
    /// Axillary node with calcifications
    #[serde(rename = "111252")]
    N111252,
    /// Axillary node hyperplasia
    #[serde(rename = "111253")]
    N111253,
    /// Asynchronous involution
    #[serde(rename = "111254")]
    N111254,
    /// Benign cyst with blood
    #[serde(rename = "111255")]
    N111255,
    /// Benign Calcifications
    #[serde(rename = "111256")]
    N111256,
    /// Intracystic papilloma
    #[serde(rename = "111257")]
    N111257,
    /// Ductal adenoma
    #[serde(rename = "111258")]
    N111258,
    /// Diabetic fibrous mastopathy
    #[serde(rename = "111259")]
    N111259,
    /// Extra abdominal desmoid
    #[serde(rename = "111260")]
    N111260,
    /// Epidermal inclusion cyst
    #[serde(rename = "111262")]
    N111262,
    /// Fibroadenomatoid hyperplasia
    #[serde(rename = "111263")]
    N111263,
    /// Fibroadenolipoma
    #[serde(rename = "111264")]
    N111264,
    /// Foreign body (reaction)
    #[serde(rename = "111265")]
    N111265,
    /// Galactocele
    #[serde(rename = "111269")]
    N111269,
    /// Hemangioma - nonparenchymal, subcutaneous
    #[serde(rename = "111271")]
    N111271,
    /// Hyperplasia, usual
    #[serde(rename = "111273")]
    N111273,
    /// Juvenile papillomatosis
    #[serde(rename = "111277")]
    N111277,
    /// Lactating adenoma
    #[serde(rename = "111278")]
    N111278,
    /// Lactational change
    #[serde(rename = "111279")]
    N111279,
    /// Large duct papilloma
    #[serde(rename = "111281")]
    N111281,
    /// Myofibroblastoma
    #[serde(rename = "111283")]
    N111283,
    /// Microglandular adenosis
    #[serde(rename = "111284")]
    N111284,
    /// Multiple Intraductal Papillomas
    #[serde(rename = "111285")]
    N111285,
    /// No abnormality
    #[serde(rename = "111286")]
    N111286,
    /// Normal breast tissue
    #[serde(rename = "111287")]
    N111287,
    /// Neurofibromatosis
    #[serde(rename = "111288")]
    N111288,
    /// Oil cyst (fat necrosis cyst)
    #[serde(rename = "111290")]
    N111290,
    /// Post reduction mammoplasty
    #[serde(rename = "111291")]
    N111291,
    /// Pseudoangiomatous stromal hyperplasia
    #[serde(rename = "111292")]
    N111292,
    /// Radial scar
    #[serde(rename = "111293")]
    N111293,
    /// Sclerosing adenosis
    #[serde(rename = "111294")]
    N111294,
    /// Silicone granuloma
    #[serde(rename = "111296")]
    N111296,
    /// Nipple Characteristic
    #[serde(rename = "111297")]
    N111297,
    /// Virginal hyperplasia
    #[serde(rename = "111298")]
    N111298,
    /// Peripheral duct papillomas
    #[serde(rename = "111299")]
    N111299,
    /// Axillary node with lymphoma
    #[serde(rename = "111300")]
    N111300,
    /// Axillary nodal metastases
    #[serde(rename = "111301")]
    N111301,
    /// Angiosarcoma
    #[serde(rename = "111302")]
    N111302,
    /// Blood vessel (vascular) invasion
    #[serde(rename = "111303")]
    N111303,
    /// Carcinoma in children
    #[serde(rename = "111304")]
    N111304,
    /// Carcinoma in ectopic breast
    #[serde(rename = "111305")]
    N111305,
    /// Carcinoma with endocrine differentiation
    #[serde(rename = "111306")]
    N111306,
    /// Basal cell carcinoma of nipple
    #[serde(rename = "111307")]
    N111307,
    /// Carcinoma with metaplasia
    #[serde(rename = "111308")]
    N111308,
    /// Cartilaginous and osseous change
    #[serde(rename = "111309")]
    N111309,
    /// Carcinoma in pregnancy and lactation
    #[serde(rename = "111310")]
    N111310,
    /// Carcinosarcoma
    #[serde(rename = "111311")]
    N111311,
    /// Intraductal comedocarcinoma with necrosis
    #[serde(rename = "111312")]
    N111312,
    /// Intraductal carcinoma, low grade
    #[serde(rename = "111313")]
    N111313,
    /// Intraductal carcinoma micro-papillary
    #[serde(rename = "111314")]
    N111314,
    /// Intracystic papillary carcinoma
    #[serde(rename = "111315")]
    N111315,
    /// Invasive and in-situ carcinoma
    #[serde(rename = "111316")]
    N111316,
    /// Invasive lobular carcinoma
    #[serde(rename = "111317")]
    N111317,
    /// Leukemic infiltration
    #[serde(rename = "111318")]
    N111318,
    /// Lympathic vessel invasion
    #[serde(rename = "111320")]
    N111320,
    /// Lymphoma
    #[serde(rename = "111321")]
    N111321,
    /// Occult carcinoma presenting with axillary lymph node metastases
    #[serde(rename = "111322")]
    N111322,
    /// Metastatic cancer to the breast
    #[serde(rename = "111323")]
    N111323,
    /// Metastatic cancer to the breast from the colon
    #[serde(rename = "111324")]
    N111324,
    /// Metastatic cancer to the breast from the lung
    #[serde(rename = "111325")]
    N111325,
    /// Metastatic melanoma to the breast
    #[serde(rename = "111326")]
    N111326,
    /// Metastatic cancer to the breast from the ovary
    #[serde(rename = "111327")]
    N111327,
    /// Metastatic sarcoma to the breast
    #[serde(rename = "111328")]
    N111328,
    /// Multifocal intraductal carcinoma
    #[serde(rename = "111329")]
    N111329,
    /// Metastatic disease to axillary node
    #[serde(rename = "111330")]
    N111330,
    /// Malignant fibrous histiocytoma
    #[serde(rename = "111331")]
    N111331,
    /// Multifocal invasive ductal carcinoma
    #[serde(rename = "111332")]
    N111332,
    /// Metastasis to an intramammary lymph node
    #[serde(rename = "111333")]
    N111333,
    /// Malignant melanoma of nipple
    #[serde(rename = "111334")]
    N111334,
    /// Neoplasm of the mammary skin
    #[serde(rename = "111335")]
    N111335,
    /// Papillary carcinoma in-situ
    #[serde(rename = "111336")]
    N111336,
    /// Recurrent malignancy
    #[serde(rename = "111338")]
    N111338,
    /// Squamous cell carcinoma of the nipple
    #[serde(rename = "111340")]
    N111340,
    /// Intraductal carcinoma, high grade
    #[serde(rename = "111341")]
    N111341,
    /// Invasive cribriform carcinoma
    #[serde(rename = "111342")]
    N111342,
    /// Angular margins
    #[serde(rename = "111343")]
    N111343,
    /// Fine pleomorphic calcification
    #[serde(rename = "111344")]
    N111344,
    /// Macrocalcifications
    #[serde(rename = "111345")]
    N111345,
    /// Calcifications within a mass
    #[serde(rename = "111346")]
    N111346,
    /// Calcifications outside of a mass
    #[serde(rename = "111347")]
    N111347,
    /// Breast background echotexture
    #[serde(rename = "111350")]
    N111350,
    /// Homogeneous fat echotexture
    #[serde(rename = "111351")]
    N111351,
    /// Homogeneous fibroglandular echotexture
    #[serde(rename = "111352")]
    N111352,
    /// Heterogeneous echotexture
    #[serde(rename = "111353")]
    N111353,
    /// Orientation
    #[serde(rename = "111354")]
    N111354,
    /// Parallel
    #[serde(rename = "111355")]
    N111355,
    /// Not parallel
    #[serde(rename = "111356")]
    N111356,
    /// Lesion boundary
    #[serde(rename = "111357")]
    N111357,
    /// Abrupt interface
    #[serde(rename = "111358")]
    N111358,
    /// Echogenic halo
    #[serde(rename = "111359")]
    N111359,
    /// Echo pattern
    #[serde(rename = "111360")]
    N111360,
    /// Anechoic
    #[serde(rename = "111361")]
    N111361,
    /// Hyperechoic
    #[serde(rename = "111362")]
    N111362,
    /// Complex
    #[serde(rename = "111363")]
    N111363,
    /// Hypoechoic
    #[serde(rename = "111364")]
    N111364,
    /// Isoechoic
    #[serde(rename = "111365")]
    N111365,
    /// Posterior acoustic features
    #[serde(rename = "111366")]
    N111366,
    /// No posterior acoustic features
    #[serde(rename = "111367")]
    N111367,
    /// Posterior enhancement
    #[serde(rename = "111368")]
    N111368,
    /// Posterior shadowing
    #[serde(rename = "111369")]
    N111369,
    /// Combined posterior enhancement and shadowing
    #[serde(rename = "111370")]
    N111370,
    /// Identifiable effect on surrounding tissues
    #[serde(rename = "111371")]
    N111371,
    /// Vascularity
    #[serde(rename = "111372")]
    N111372,
    /// Vascularity not present
    #[serde(rename = "111373")]
    N111373,
    /// Vascularity not assessed
    #[serde(rename = "111374")]
    N111374,
    /// Vascularity present in lesion
    #[serde(rename = "111375")]
    N111375,
    /// Vascularity present immediately adjacent to lesion
    #[serde(rename = "111376")]
    N111376,
    /// Diffusely increased vascularity in surrounding tissue
    #[serde(rename = "111377")]
    N111377,
    /// Correlation to other Findings
    #[serde(rename = "111380")]
    N111380,
    /// Correlates to physical exam findings
    #[serde(rename = "111381")]
    N111381,
    /// Correlates to mammography findings
    #[serde(rename = "111382")]
    N111382,
    /// Correlates to MRI findings
    #[serde(rename = "111383")]
    N111383,
    /// Correlates to ultrasound findings
    #[serde(rename = "111384")]
    N111384,
    /// Correlates to other imaging findings
    #[serde(rename = "111385")]
    N111385,
    /// No correlation to other imaging findings
    #[serde(rename = "111386")]
    N111386,
    /// No correlation to clinical findings
    #[serde(rename = "111387")]
    N111387,
    /// Malignancy Type
    #[serde(rename = "111388")]
    N111388,
    /// Invasive breast carcinoma
    #[serde(rename = "111389")]
    N111389,
    /// Other malignancy type
    #[serde(rename = "111390")]
    N111390,
    /// Menstrual Cycle Phase
    #[serde(rename = "111391")]
    N111391,
    /// 1st week
    #[serde(rename = "111392")]
    N111392,
    /// 2nd week
    #[serde(rename = "111393")]
    N111393,
    /// 3rd week
    #[serde(rename = "111394")]
    N111394,
    /// Estimated Timeframe
    #[serde(rename = "111395")]
    N111395,
    /// < 3 months ago
    #[serde(rename = "111396")]
    N111396,
    /// 4 months to 1 year ago
    #[serde(rename = "111397")]
    N111397,
    /// \> 1 year ago
    #[serde(rename = "111398")]
    N111398,
    /// Timeframe uncertain
    #[serde(rename = "111399")]
    N111399,
    /// Breast Imaging Report
    #[serde(rename = "111400")]
    N111400,
    /// Reason for procedure
    #[serde(rename = "111401")]
    N111401,
    /// Clinical Finding
    #[serde(rename = "111402")]
    N111402,
    /// Baseline screening mammogram
    #[serde(rename = "111403")]
    N111403,
    /// First mammogram ever
    #[serde(rename = "111404")]
    N111404,
    /// Implant type
    #[serde(rename = "111405")]
    N111405,
    /// Number of similar findings
    #[serde(rename = "111406")]
    N111406,
    /// Implant finding
    #[serde(rename = "111407")]
    N111407,
    /// Film Screen Mammography
    #[serde(rename = "111408")]
    N111408,
    /// Digital Mammography
    #[serde(rename = "111409")]
    N111409,
    /// Surgical consult
    #[serde(rename = "111410")]
    N111410,
    /// Mammography CAD
    #[serde(rename = "111411")]
    N111411,
    /// Narrative Summary
    #[serde(rename = "111412")]
    N111412,
    /// Overall Assessment
    #[serde(rename = "111413")]
    N111413,
    /// Supplementary Data
    #[serde(rename = "111414")]
    N111414,
    /// Additional evaluation requested from prior study
    #[serde(rename = "111415")]
    N111415,
    /// Follow-up at short interval from prior study
    #[serde(rename = "111416")]
    N111416,
    /// History of breast augmentation, asymptomatic
    #[serde(rename = "111417")]
    N111417,
    /// Review of an outside study
    #[serde(rename = "111418")]
    N111418,
    /// Additional evaluation requested from abnormal screening exam
    #[serde(rename = "111419")]
    N111419,
    /// History of benign breast biopsy
    #[serde(rename = "111420")]
    N111420,
    /// Personal history of breast cancer with breast conservation therapy
    #[serde(rename = "111421")]
    N111421,
    /// Physical Examination Results
    #[serde(rename = "111423")]
    N111423,
    /// Comparison to previous findings
    #[serde(rename = "111424")]
    N111424,
    /// Intraluminal filling defect
    #[serde(rename = "111425")]
    N111425,
    /// Multiple filling defect
    #[serde(rename = "111426")]
    N111426,
    /// Abrupt duct termination
    #[serde(rename = "111427")]
    N111427,
    /// Extravasation
    #[serde(rename = "111428")]
    N111428,
    /// Duct narrowing
    #[serde(rename = "111429")]
    N111429,
    /// Cyst fill
    #[serde(rename = "111430")]
    N111430,
    /// Instrument Approach
    #[serde(rename = "111431")]
    N111431,
    /// Inferolateral to superomedial
    #[serde(rename = "111432")]
    N111432,
    /// Inferomedial to superolateral
    #[serde(rename = "111433")]
    N111433,
    /// Superolateral to inferomedial
    #[serde(rename = "111434")]
    N111434,
    /// Superomedial to inferolateral
    #[serde(rename = "111435")]
    N111435,
    /// Number of passes
    #[serde(rename = "111436")]
    N111436,
    /// Number of specimens
    #[serde(rename = "111437")]
    N111437,
    /// Needle in target
    #[serde(rename = "111438")]
    N111438,
    /// Number of needles around target
    #[serde(rename = "111439")]
    N111439,
    /// Incision made
    #[serde(rename = "111440")]
    N111440,
    /// Microclip placed
    #[serde(rename = "111441")]
    N111441,
    /// Confirmation of target
    #[serde(rename = "111442")]
    N111442,
    /// Target completely contained in the specimen
    #[serde(rename = "111443")]
    N111443,
    /// Target partially obtained in the specimen
    #[serde(rename = "111444")]
    N111444,
    /// Target not in the specimen
    #[serde(rename = "111445")]
    N111445,
    /// Calcifications seen in the core
    #[serde(rename = "111446")]
    N111446,
    /// Lesion completely removed
    #[serde(rename = "111447")]
    N111447,
    /// Lesion partially removed
    #[serde(rename = "111448")]
    N111448,
    /// Fluid obtained
    #[serde(rename = "111449")]
    N111449,
    /// Light brown color
    #[serde(rename = "111450")]
    N111450,
    /// Dark red color
    #[serde(rename = "111451")]
    N111451,
    /// Dark brown color
    #[serde(rename = "111452")]
    N111452,
    /// Bright red color
    #[serde(rename = "111453")]
    N111453,
    /// Blood tinged color
    #[serde(rename = "111454")]
    N111454,
    /// Occult blood test result
    #[serde(rename = "111455")]
    N111455,
    /// Action on fluid
    #[serde(rename = "111456")]
    N111456,
    /// Sent for analysis
    #[serde(rename = "111457")]
    N111457,
    /// Discarded
    #[serde(rename = "111458")]
    N111458,
    /// Mass with calcifications
    #[serde(rename = "111459")]
    N111459,
    /// Complex cyst
    #[serde(rename = "111460")]
    N111460,
    /// Intracystic lesion
    #[serde(rename = "111461")]
    N111461,
    /// Solid mass
    #[serde(rename = "111462")]
    N111462,
    /// Supplementary Data for Intervention
    #[serde(rename = "111463")]
    N111463,
    /// Procedure Modifier
    #[serde(rename = "111464")]
    N111464,
    /// Needle Gauge
    #[serde(rename = "111465")]
    N111465,
    /// Severity of Complication
    #[serde(rename = "111466")]
    N111466,
    /// Needle Length
    #[serde(rename = "111467")]
    N111467,
    /// Pathology Results
    #[serde(rename = "111468")]
    N111468,
    /// Sampling DateTime
    #[serde(rename = "111469")]
    N111469,
    /// Uninvolved
    #[serde(rename = "111470")]
    N111470,
    /// Involved
    #[serde(rename = "111471")]
    N111471,
    /// Nipple involved
    #[serde(rename = "111472")]
    N111472,
    /// Number of nodes removed
    #[serde(rename = "111473")]
    N111473,
    /// Number of nodes positive
    #[serde(rename = "111474")]
    N111474,
    /// Estrogen receptor
    #[serde(rename = "111475")]
    N111475,
    /// Progesterone receptor
    #[serde(rename = "111476")]
    N111476,
    /// S Phase
    #[serde(rename = "111477")]
    N111477,
    /// Non-bloody discharge (from nipple)
    #[serde(rename = "111478")]
    N111478,
    /// Difficult physical/clinical examination
    #[serde(rename = "111479")]
    N111479,
    /// Cancer elsewhere
    #[serde(rename = "111480")]
    N111480,
    /// Saline implant
    #[serde(rename = "111481")]
    N111481,
    /// Polyurethane implant
    #[serde(rename = "111482")]
    N111482,
    /// Percutaneous silicone injection
    #[serde(rename = "111483")]
    N111483,
    /// Combination implant
    #[serde(rename = "111484")]
    N111484,
    /// Pre-pectoral implant
    #[serde(rename = "111485")]
    N111485,
    /// Retro-pectoral implant
    #[serde(rename = "111486")]
    N111486,
    /// Mammographic (crosshair)
    #[serde(rename = "111487")]
    N111487,
    /// Mammographic (grid)
    #[serde(rename = "111488")]
    N111488,
    /// Palpation guided
    #[serde(rename = "111489")]
    N111489,
    /// Vacuum assisted
    #[serde(rename = "111490")]
    N111490,
    /// Abnormal discharge
    #[serde(rename = "111491")]
    N111491,
    /// No complications
    #[serde(rename = "111492")]
    N111492,
    /// Stage 0
    #[serde(rename = "111494")]
    N111494,
    /// Stage I
    #[serde(rename = "111495")]
    N111495,
    /// Stage IIA
    #[serde(rename = "111496")]
    N111496,
    /// Stage IIB
    #[serde(rename = "111497")]
    N111497,
    /// Stage IIIA
    #[serde(rename = "111498")]
    N111498,
    /// Stage IIIB
    #[serde(rename = "111499")]
    N111499,
    /// Stage IIIC
    #[serde(rename = "111500")]
    N111500,
    /// Stage IV
    #[serde(rename = "111501")]
    N111501,
    /// Bloom-Richardson Grade
    #[serde(rename = "111502")]
    N111502,
    /// Normal implants
    #[serde(rename = "111503")]
    N111503,
    /// Asymmetric implants
    #[serde(rename = "111504")]
    N111504,
    /// Calcified implant
    #[serde(rename = "111505")]
    N111505,
    /// Distorted implant
    #[serde(rename = "111506")]
    N111506,
    /// Silicone-laden lymph nodes
    #[serde(rename = "111507")]
    N111507,
    /// Free silicone
    #[serde(rename = "111508")]
    N111508,
    /// Herniated implant
    #[serde(rename = "111509")]
    N111509,
    /// Explantation
    #[serde(rename = "111510")]
    N111510,
    /// Relevant Patient Information for Breast Imaging
    #[serde(rename = "111511")]
    N111511,
    /// Medication History
    #[serde(rename = "111512")]
    N111512,
    /// Relevant Previous Procedures
    #[serde(rename = "111513")]
    N111513,
    /// Relevant Indicated Problems
    #[serde(rename = "111514")]
    N111514,
    /// Relevant Risk Factors
    #[serde(rename = "111515")]
    N111515,
    /// Medication Type
    #[serde(rename = "111516")]
    N111516,
    /// Relevant Patient Information
    #[serde(rename = "111517")]
    N111517,
    /// Age when first menstrual period occurred
    #[serde(rename = "111518")]
    N111518,
    /// Age at First Full Term Pregnancy
    #[serde(rename = "111519")]
    N111519,
    /// Age at Menopause
    #[serde(rename = "111520")]
    N111520,
    /// Age when hysterectomy performed
    #[serde(rename = "111521")]
    N111521,
    /// Age when left ovary removed
    #[serde(rename = "111522")]
    N111522,
    /// Age when right ovary removed
    #[serde(rename = "111523")]
    N111523,
    /// Age Started
    #[serde(rename = "111524")]
    N111524,
    /// Age Ended
    #[serde(rename = "111525")]
    N111525,
    /// DateTime Started
    #[serde(rename = "111526")]
    N111526,
    /// DateTime Ended
    #[serde(rename = "111527")]
    N111527,
    /// Ongoing
    #[serde(rename = "111528")]
    N111528,
    /// Brand Name
    #[serde(rename = "111529")]
    N111529,
    /// Risk Factor modifier
    #[serde(rename = "111530")]
    N111530,
    /// Previous Procedure
    #[serde(rename = "111531")]
    N111531,
    /// Pregnancy Status
    #[serde(rename = "111532")]
    N111532,
    /// Indicated Problem
    #[serde(rename = "111533")]
    N111533,
    /// Role of person reporting
    #[serde(rename = "111534")]
    N111534,
    /// DateTime problem observed
    #[serde(rename = "111535")]
    N111535,
    /// DateTime of last evaluation
    #[serde(rename = "111536")]
    N111536,
    /// Family Member with Risk Factor
    #[serde(rename = "111537")]
    N111537,
    /// Age at Occurrence
    #[serde(rename = "111538")]
    N111538,
    /// Menopausal phase
    #[serde(rename = "111539")]
    N111539,
    /// Side of Family
    #[serde(rename = "111540")]
    N111540,
    /// Maternal
    #[serde(rename = "111541")]
    N111541,
    /// Unspecified gynecological hormone
    #[serde(rename = "111542")]
    N111542,
    /// Breast feeding history
    #[serde(rename = "111543")]
    N111543,
    /// Average breast feeding period
    #[serde(rename = "111544")]
    N111544,
    /// Substance Use History
    #[serde(rename = "111545")]
    N111545,
    /// Used Substance Type
    #[serde(rename = "111546")]
    N111546,
    /// Environmental Exposure History
    #[serde(rename = "111547")]
    N111547,
    /// Environmental Factor
    #[serde(rename = "111548")]
    N111548,
    /// Previous Reports
    #[serde(rename = "111549")]
    N111549,
    /// Personal breast cancer history
    #[serde(rename = "111550")]
    N111550,
    /// History of endometrial cancer
    #[serde(rename = "111551")]
    N111551,
    /// History of ovarian cancer
    #[serde(rename = "111552")]
    N111552,
    /// History of high risk lesion on previous biopsy
    #[serde(rename = "111553")]
    N111553,
    /// Post menopausal patient
    #[serde(rename = "111554")]
    N111554,
    /// Late child bearing (after 30)
    #[serde(rename = "111555")]
    N111555,
    /// BRCA1 breast cancer gene
    #[serde(rename = "111556")]
    N111556,
    /// BRCA2 breast cancer gene
    #[serde(rename = "111557")]
    N111557,
    /// BRCA3 breast cancer gene
    #[serde(rename = "111558")]
    N111558,
    /// Weak family history of breast cancer
    #[serde(rename = "111559")]
    N111559,
    /// Intermediate family history of breast cancer
    #[serde(rename = "111560")]
    N111560,
    /// Very strong family history of breast cancer
    #[serde(rename = "111561")]
    N111561,
    /// Family history of prostate cancer
    #[serde(rename = "111562")]
    N111562,
    /// Family history unknown
    #[serde(rename = "111563")]
    N111563,
    /// Nipple discharge cytology
    #[serde(rename = "111564")]
    N111564,
    /// Uterine malformations
    #[serde(rename = "111565")]
    N111565,
    /// Spontaneous Abortion
    #[serde(rename = "111566")]
    N111566,
    /// Gynecologic condition
    #[serde(rename = "111567")]
    N111567,
    /// Gynecologic surgery
    #[serde(rename = "111568")]
    N111568,
    /// Previous LBW or IUGR birth
    #[serde(rename = "111569")]
    N111569,
    /// Previous fetal malformation/syndrome
    #[serde(rename = "111570")]
    N111570,
    /// Previous RH negative or blood dyscrasia at birth
    #[serde(rename = "111571")]
    N111571,
    /// History of multiple fetuses
    #[serde(rename = "111572")]
    N111572,
    /// Current pregnancy, known or suspected malformations/syndromes
    #[serde(rename = "111573")]
    N111573,
    /// Family history, fetal malformation/syndrome
    #[serde(rename = "111574")]
    N111574,
    /// High
    #[serde(rename = "111575")]
    N111575,
    /// Medium
    #[serde(rename = "111576")]
    N111576,
    /// Low
    #[serde(rename = "111577")]
    N111577,
    /// Dose frequency
    #[serde(rename = "111578")]
    N111578,
    /// Rate of exposure
    #[serde(rename = "111579")]
    N111579,
    /// Volume of use
    #[serde(rename = "111580")]
    N111580,
    /// Relative dose amount
    #[serde(rename = "111581")]
    N111581,
    /// Relative amount of exposure
    #[serde(rename = "111582")]
    N111582,
    /// Relative amount of use
    #[serde(rename = "111583")]
    N111583,
    /// Relative dose frequency
    #[serde(rename = "111584")]
    N111584,
    /// Relative frequency of exposure
    #[serde(rename = "111585")]
    N111585,
    /// Relative frequency of use
    #[serde(rename = "111586")]
    N111586,
    /// No known exposure
    #[serde(rename = "111587")]
    N111587,
    /// Recall for technical reasons
    #[serde(rename = "111590")]
    N111590,
    /// Recall for imaging findings
    #[serde(rename = "111591")]
    N111591,
    /// Recall for patient symptoms/ clinical findings
    #[serde(rename = "111592")]
    N111592,
    /// LBW or IUGR
    #[serde(rename = "111593")]
    N111593,
    /// Green filter
    #[serde(rename = "111601")]
    N111601,
    /// Red filter
    #[serde(rename = "111602")]
    N111602,
    /// Blue filter
    #[serde(rename = "111603")]
    N111603,
    /// Yellow-green filter
    #[serde(rename = "111604")]
    N111604,
    /// Blue-green filter
    #[serde(rename = "111605")]
    N111605,
    /// Infrared filter
    #[serde(rename = "111606")]
    N111606,
    /// Polarizing filter
    #[serde(rename = "111607")]
    N111607,
    /// No filter
    #[serde(rename = "111609")]
    N111609,
    /// Field 1 for Joslin 3 field
    #[serde(rename = "111621")]
    N111621,
    /// Field 2 for Joslin 3 field
    #[serde(rename = "111622")]
    N111622,
    /// Field 3 for Joslin 3 field
    #[serde(rename = "111623")]
    N111623,
    /// Diffuse direct illumination
    #[serde(rename = "111625")]
    N111625,
    /// Scheimpflug Camera
    #[serde(rename = "111626")]
    N111626,
    /// Scotopic light
    #[serde(rename = "111627")]
    N111627,
    /// Mesopic light
    #[serde(rename = "111628")]
    N111628,
    /// Photopic light
    #[serde(rename = "111629")]
    N111629,
    /// Dynamic light
    #[serde(rename = "111630")]
    N111630,
    /// Average Glandular Dose
    #[serde(rename = "111631")]
    N111631,
    /// Anode Target Material
    #[serde(rename = "111632")]
    N111632,
    /// Compression Thickness
    #[serde(rename = "111633")]
    N111633,
    /// Half Value Layer
    #[serde(rename = "111634")]
    N111634,
    /// X-Ray Grid
    #[serde(rename = "111635")]
    N111635,
    /// Entrance Exposure at RP
    #[serde(rename = "111636")]
    N111636,
    /// Accumulated Average Glandular Dose
    #[serde(rename = "111637")]
    N111637,
    /// Patient Equivalent Thickness
    #[serde(rename = "111638")]
    N111638,
    /// Fixed grid
    #[serde(rename = "111641")]
    N111641,
    /// Focused grid
    #[serde(rename = "111642")]
    N111642,
    /// Reciprocating grid
    #[serde(rename = "111643")]
    N111643,
    /// Parallel grid
    #[serde(rename = "111644")]
    N111644,
    /// Crossed grid
    #[serde(rename = "111645")]
    N111645,
    /// No grid
    #[serde(rename = "111646")]
    N111646,
    /// Spectacle Prescription Report
    #[serde(rename = "111671")]
    N111671,
    /// Add Near
    #[serde(rename = "111672")]
    N111672,
    /// Add Intermediate
    #[serde(rename = "111673")]
    N111673,
    /// Add Other
    #[serde(rename = "111674")]
    N111674,
    /// Horizontal Prism Power
    #[serde(rename = "111675")]
    N111675,
    /// Horizontal Prism Base
    #[serde(rename = "111676")]
    N111676,
    /// Vertical Prism Power
    #[serde(rename = "111677")]
    N111677,
    /// Vertical Prism Base
    #[serde(rename = "111678")]
    N111678,
    /// Distance Pupillary Distance
    #[serde(rename = "111679")]
    N111679,
    /// Near Pupillary Distance
    #[serde(rename = "111680")]
    N111680,
    /// Autorefraction Visual Acuity
    #[serde(rename = "111685")]
    N111685,
    /// Habitual Visual Acuity
    #[serde(rename = "111686")]
    N111686,
    /// Prescription Visual Acuity
    #[serde(rename = "111687")]
    N111687,
    /// Right Eye Rx
    #[serde(rename = "111688")]
    N111688,
    /// Left Eye Rx
    #[serde(rename = "111689")]
    N111689,
    /// Macular Grid Thickness and Volume Report
    #[serde(rename = "111690")]
    N111690,
    /// Number of Images Used for Macular Measurements
    #[serde(rename = "111691")]
    N111691,
    /// Number of Samples Used per Image
    #[serde(rename = "111692")]
    N111692,
    /// Analysis Quality Rating
    #[serde(rename = "111693")]
    N111693,
    /// Image Set Quality Rating
    #[serde(rename = "111694")]
    N111694,
    /// Interfering Tears or Drops
    #[serde(rename = "111695")]
    N111695,
    /// Visual Fixation Quality During Acquisition
    #[serde(rename = "111696")]
    N111696,
    /// Visual Fixation Quality Problem
    #[serde(rename = "111697")]
    N111697,
    /// Ophthalmic Macular Grid Problem
    #[serde(rename = "111698")]
    N111698,
    /// Specimen Container Identifier
    #[serde(rename = "111700")]
    N111700,
    /// Processing type
    #[serde(rename = "111701")]
    N111701,
    /// DateTime of processing
    #[serde(rename = "111702")]
    N111702,
    /// Processing step description
    #[serde(rename = "111703")]
    N111703,
    /// Sampling Method
    #[serde(rename = "111704")]
    N111704,
    /// Parent Specimen Identifier
    #[serde(rename = "111705")]
    N111705,
    /// Issuer of Parent Specimen Identifier
    #[serde(rename = "111706")]
    N111706,
    /// Parent specimen type
    #[serde(rename = "111707")]
    N111707,
    /// Position Frame of Reference
    #[serde(rename = "111708")]
    N111708,
    /// Location of sampling site
    #[serde(rename = "111709")]
    N111709,
    /// Location of sampling site X offset
    #[serde(rename = "111710")]
    N111710,
    /// Location of sampling site Y offset
    #[serde(rename = "111711")]
    N111711,
    /// Location of sampling site Z offset
    #[serde(rename = "111712")]
    N111712,
    /// Location of Specimen
    #[serde(rename = "111718")]
    N111718,
    /// Location of Specimen X offset
    #[serde(rename = "111719")]
    N111719,
    /// Location of Specimen Y offset
    #[serde(rename = "111720")]
    N111720,
    /// Location of Specimen Z offset
    #[serde(rename = "111721")]
    N111721,
    /// Visual Marking of Specimen
    #[serde(rename = "111723")]
    N111723,
    /// Issuer of Specimen Identifier
    #[serde(rename = "111724")]
    N111724,
    /// Dissection with entire specimen submission
    #[serde(rename = "111726")]
    N111726,
    /// Dissection with representative sections submission
    #[serde(rename = "111727")]
    N111727,
    /// Specimen storage
    #[serde(rename = "111729")]
    N111729,
    /// Transmission illumination
    #[serde(rename = "111741")]
    N111741,
    /// Reflection illumination
    #[serde(rename = "111742")]
    N111742,
    /// Epifluorescence illumination
    #[serde(rename = "111743")]
    N111743,
    /// Brightfield illumination
    #[serde(rename = "111744")]
    N111744,
    /// Darkfield illumination
    #[serde(rename = "111745")]
    N111745,
    /// Oblique illumination
    #[serde(rename = "111746")]
    N111746,
    /// Phase contrast illumination
    #[serde(rename = "111747")]
    N111747,
    /// Differential interference contrast
    #[serde(rename = "111748")]
    N111748,
    /// Total internal reflection fluorescence
    #[serde(rename = "111749")]
    N111749,
    /// Ultrasound Contact
    #[serde(rename = "111750")]
    N111750,
    /// Ultrasound Immersion
    #[serde(rename = "111751")]
    N111751,
    /// Optical
    #[serde(rename = "111752")]
    N111752,
    /// Manual Keratometry
    #[serde(rename = "111753")]
    N111753,
    /// Auto Keratometry
    #[serde(rename = "111754")]
    N111754,
    /// Simulated Keratometry
    #[serde(rename = "111755")]
    N111755,
    /// Equivalent K-reading
    #[serde(rename = "111756")]
    N111756,
    /// Haigis
    #[serde(rename = "111760")]
    N111760,
    /// Haigis-L
    #[serde(rename = "111761")]
    N111761,
    /// Holladay 1
    #[serde(rename = "111762")]
    N111762,
    /// Holladay 2
    #[serde(rename = "111763")]
    N111763,
    /// Hoffer Q
    #[serde(rename = "111764")]
    N111764,
    /// Olsen
    #[serde(rename = "111765")]
    N111765,
    /// SRKII
    #[serde(rename = "111766")]
    N111766,
    /// SRK-T
    #[serde(rename = "111767")]
    N111767,
    /// ACD Constant
    #[serde(rename = "111768")]
    N111768,
    /// Haigis a0
    #[serde(rename = "111769")]
    N111769,
    /// Haigis a1
    #[serde(rename = "111770")]
    N111770,
    /// Haigis a2
    #[serde(rename = "111771")]
    N111771,
    /// Hoffer pACD Constant
    #[serde(rename = "111772")]
    N111772,
    /// Surgeon Factor
    #[serde(rename = "111773")]
    N111773,
    /// Front Of Cornea To Front Of Lens
    #[serde(rename = "111776")]
    N111776,
    /// Back Of Cornea To Front Of Lens
    #[serde(rename = "111777")]
    N111777,
    /// Single or Anterior Lens
    #[serde(rename = "111778")]
    N111778,
    /// Posterior Lens
    #[serde(rename = "111779")]
    N111779,
    /// Measurement From This Device
    #[serde(rename = "111780")]
    N111780,
    /// External Data Source
    #[serde(rename = "111781")]
    N111781,
    /// Axial Measurements SOP Instance
    #[serde(rename = "111782")]
    N111782,
    /// Refractive Measurements SOP Instance
    #[serde(rename = "111783")]
    N111783,
    /// Standard Deviation of measurements used
    #[serde(rename = "111786")]
    N111786,
    /// Signal to Noise Ratio
    #[serde(rename = "111787")]
    N111787,
    /// Spherical projection
    #[serde(rename = "111791")]
    N111791,
    /// Surface contour mapping
    #[serde(rename = "111792")]
    N111792,
    /// Visual Field 24-2 Test Pattern
    #[serde(rename = "111800")]
    N111800,
    /// Visual Field 10-2 Test Pattern
    #[serde(rename = "111801")]
    N111801,
    /// Visual Field 30-2 Test Pattern
    #[serde(rename = "111802")]
    N111802,
    /// Visual Field 60-4 Test Pattern
    #[serde(rename = "111803")]
    N111803,
    /// Visual Field Macula Test Pattern
    #[serde(rename = "111804")]
    N111804,
    /// Visual Field Central 40 Point Test Pattern
    #[serde(rename = "111805")]
    N111805,
    /// Visual Field Central 76 Point Test Pattern
    #[serde(rename = "111806")]
    N111806,
    /// Visual Field Peripheral 60 Point Test Pattern
    #[serde(rename = "111807")]
    N111807,
    /// Visual Field Full Field 81 Point Test Pattern
    #[serde(rename = "111808")]
    N111808,
    /// Visual Field Full Field 120 Point Test Pattern
    #[serde(rename = "111809")]
    N111809,
    /// Visual Field G Test Pattern
    #[serde(rename = "111810")]
    N111810,
    /// Visual Field M Test Pattern
    #[serde(rename = "111811")]
    N111811,
    /// Visual Field 07 Test Pattern
    #[serde(rename = "111812")]
    N111812,
    /// Visual Field LVC Test Pattern
    #[serde(rename = "111813")]
    N111813,
    /// Visual Field Central Test Pattern
    #[serde(rename = "111814")]
    N111814,
    /// Visual Field SITA-Standard Test Strategy
    #[serde(rename = "111815")]
    N111815,
    /// Visual Field SITA-SWAP Test Strategy
    #[serde(rename = "111816")]
    N111816,
    /// Visual Field SITA-Fast Test Strategy
    #[serde(rename = "111817")]
    N111817,
    /// Visual Field Full Threshold Test Strategy
    #[serde(rename = "111818")]
    N111818,
    /// Visual Field FastPac Test Strategy
    #[serde(rename = "111819")]
    N111819,
    /// Visual Field Full From Prior Test Strategy
    #[serde(rename = "111820")]
    N111820,
    /// Visual Field Optima Test Strategy
    #[serde(rename = "111821")]
    N111821,
    /// Visual Field Two-Zone Test Strategy
    #[serde(rename = "111822")]
    N111822,
    /// Visual Field Three-Zone Test Strategy
    #[serde(rename = "111823")]
    N111823,
    /// Visual Field Quantify-Defects Test Strategy
    #[serde(rename = "111824")]
    N111824,
    /// Visual Field TOP Test Strategy
    #[serde(rename = "111825")]
    N111825,
    /// Visual Field Dynamic Test Strategy
    #[serde(rename = "111826")]
    N111826,
    /// Visual Field Normal Test Strategy
    #[serde(rename = "111827")]
    N111827,
    /// Visual Field 1-LT Test Strategy
    #[serde(rename = "111828")]
    N111828,
    /// Visual Field 2-LT Test Strategy
    #[serde(rename = "111829")]
    N111829,
    /// Visual Field LVS Test Strategy
    #[serde(rename = "111830")]
    N111830,
    /// Visual Field GATE Test Strategy
    #[serde(rename = "111831")]
    N111831,
    /// Visual Field GATEi Test Strategy
    #[serde(rename = "111832")]
    N111832,
    /// Visual Field 2LT-Dynamic Test Strategy
    #[serde(rename = "111833")]
    N111833,
    /// Visual Field 2LT-Normal Test Strategy
    #[serde(rename = "111834")]
    N111834,
    /// Visual Field Fast Threshold Test Strategy
    #[serde(rename = "111835")]
    N111835,
    /// Visual Field CLIP Test Strategy
    #[serde(rename = "111836")]
    N111836,
    /// Visual Field CLASS Strategy
    #[serde(rename = "111837")]
    N111837,
    /// Age corrected
    #[serde(rename = "111838")]
    N111838,
    /// Threshold related
    #[serde(rename = "111839")]
    N111839,
    /// Single luminance
    #[serde(rename = "111840")]
    N111840,
    /// Foveal sensitivity related
    #[serde(rename = "111841")]
    N111841,
    /// Related to non macular sensitivity
    #[serde(rename = "111842")]
    N111842,
    /// Automated Optical
    #[serde(rename = "111843")]
    N111843,
    /// Blind Spot Monitoring
    #[serde(rename = "111844")]
    N111844,
    /// Macular Fixation Testing
    #[serde(rename = "111845")]
    N111845,
    /// Observation by Examiner
    #[serde(rename = "111846")]
    N111846,
    /// Outside normal limits
    #[serde(rename = "111847")]
    N111847,
    /// Borderline
    #[serde(rename = "111848")]
    N111848,
    /// Abnormally high sensitivity
    #[serde(rename = "111849")]
    N111849,
    /// General reduction in sensitivity
    #[serde(rename = "111850")]
    N111850,
    /// Borderline and general reduction in sensitivity
    #[serde(rename = "111851")]
    N111851,
    /// Visual Field Index
    #[serde(rename = "111852")]
    N111852,
    /// Visual Field Loss Due to Diffuse Defect
    #[serde(rename = "111853")]
    N111853,
    /// Visual Field Loss Due to Local Defect
    #[serde(rename = "111854")]
    N111854,
    /// Glaucoma Hemifield Test Analysis
    #[serde(rename = "111855")]
    N111855,
    /// Optical Fixation Measurements
    #[serde(rename = "111856")]
    N111856,
    /// Macula centered
    #[serde(rename = "111900")]
    N111900,
    /// Disc centered
    #[serde(rename = "111901")]
    N111901,
    /// Lesion centered
    #[serde(rename = "111902")]
    N111902,
    /// Disc-macula centered
    #[serde(rename = "111903")]
    N111903,
    /// Mid-peripheral-superior
    #[serde(rename = "111904")]
    N111904,
    /// Mid-peripheral-superior temporal
    #[serde(rename = "111905")]
    N111905,
    /// Mid-peripheral-temporal
    #[serde(rename = "111906")]
    N111906,
    /// Mid-peripheral-inferior temporal
    #[serde(rename = "111907")]
    N111907,
    /// Mid-peripheral-inferior
    #[serde(rename = "111908")]
    N111908,
    /// Mid-peripheral-inferior nasal
    #[serde(rename = "111909")]
    N111909,
    /// Mid-peripheral-nasal
    #[serde(rename = "111910")]
    N111910,
    /// Mid-peripheral-superior nasal
    #[serde(rename = "111911")]
    N111911,
    /// Peripheral-superior
    #[serde(rename = "111912")]
    N111912,
    /// Peripheral-superior temporal
    #[serde(rename = "111913")]
    N111913,
    /// Peripheral-temporal
    #[serde(rename = "111914")]
    N111914,
    /// Peripheral-inferior temporal
    #[serde(rename = "111915")]
    N111915,
    /// Peripheral-inferior
    #[serde(rename = "111916")]
    N111916,
    /// Peripheral-inferior nasal
    #[serde(rename = "111917")]
    N111917,
    /// Peripheral-nasal
    #[serde(rename = "111918")]
    N111918,
    /// Peripheral-superior nasal
    #[serde(rename = "111919")]
    N111919,
    /// Time domain
    #[serde(rename = "111920")]
    N111920,
    /// Spectral domain
    #[serde(rename = "111921")]
    N111921,
    /// No corneal compensation
    #[serde(rename = "111922")]
    N111922,
    /// Corneal birefringence compensation
    #[serde(rename = "111923")]
    N111923,
    /// Retinal topography
    #[serde(rename = "111924")]
    N111924,
    /// Retinal nerve fiber layer thickness
    #[serde(rename = "111925")]
    N111925,
    /// Ganglion cell complex thickness
    #[serde(rename = "111926")]
    N111926,
    /// Total retinal thickness (ILM to IS-OS)
    #[serde(rename = "111927")]
    N111927,
    /// Total retinal thickness (ILM to RPE)
    #[serde(rename = "111928")]
    N111928,
    /// Total retinal thickness (ILM to BM)
    #[serde(rename = "111929")]
    N111929,
    /// Absolute ophthalmic thickness
    #[serde(rename = "111930")]
    N111930,
    /// Thickness deviation category from normative data
    #[serde(rename = "111931")]
    N111931,
    /// Thickness deviation from normative data
    #[serde(rename = "111932")]
    N111932,
    /// Related ophthalmic thickness map
    #[serde(rename = "111933")]
    N111933,
    /// Disc-Fovea
    #[serde(rename = "111934")]
    N111934,
    /// p>5%
    #[serde(rename = "111935")]
    N111935,
    /// p<5%
    #[serde(rename = "111936")]
    N111936,
    /// p<2%
    #[serde(rename = "111937")]
    N111937,
    /// p<1%
    #[serde(rename = "111938")]
    N111938,
    /// p<0.5%
    #[serde(rename = "111939")]
    N111939,
    /// Corneal axial power map
    #[serde(rename = "111940")]
    N111940,
    /// Corneal instantaneous power map
    #[serde(rename = "111941")]
    N111941,
    /// Corneal refractive power map
    #[serde(rename = "111942")]
    N111942,
    /// Corneal elevation map
    #[serde(rename = "111943")]
    N111943,
    /// Corneal wavefront map
    #[serde(rename = "111944")]
    N111944,
    /// Elevation-based corneal tomographer
    #[serde(rename = "111945")]
    N111945,
    /// Reflection-based corneal topographer
    #[serde(rename = "111946")]
    N111946,
    /// Interferometry-based corneal tomographer
    #[serde(rename = "111947")]
    N111947,
    /// Chest CAD Report
    #[serde(rename = "112000")]
    N112000,
    /// Opacity
    #[serde(rename = "112001")]
    N112001,
    /// Series Instance UID
    #[serde(rename = "112002")]
    N112002,
    /// Associated Chest Component
    #[serde(rename = "112003")]
    N112003,
    /// Abnormal interstitial pattern
    #[serde(rename = "112004")]
    N112004,
    /// Radiographic anatomy
    #[serde(rename = "112005")]
    N112005,
    /// Distribution Descriptor
    #[serde(rename = "112006")]
    N112006,
    /// Border definition
    #[serde(rename = "112007")]
    N112007,
    /// Site involvement
    #[serde(rename = "112008")]
    N112008,
    /// Type of Content
    #[serde(rename = "112009")]
    N112009,
    /// Texture Descriptor
    #[serde(rename = "112010")]
    N112010,
    /// Positioner Primary Angle
    #[serde(rename = "112011")]
    N112011,
    /// Positioner Secondary Angle
    #[serde(rename = "112012")]
    N112012,
    /// Location in Chest
    #[serde(rename = "112013")]
    N112013,
    /// Orientation Descriptor
    #[serde(rename = "112014")]
    N112014,
    /// Border shape
    #[serde(rename = "112015")]
    N112015,
    /// Baseline Category
    #[serde(rename = "112016")]
    N112016,
    /// Cavity extent as percent of volume
    #[serde(rename = "112017")]
    N112017,
    /// Calcification extent as percent of surface
    #[serde(rename = "112018")]
    N112018,
    /// Calcification extent as percent of volume
    #[serde(rename = "112019")]
    N112019,
    /// Response Evaluation
    #[serde(rename = "112020")]
    N112020,
    /// Response Evaluation Method
    #[serde(rename = "112021")]
    N112021,
    /// RECIST
    #[serde(rename = "112022")]
    N112022,
    /// Composite Feature Modifier
    #[serde(rename = "112023")]
    N112023,
    /// Single Image Finding Modifier
    #[serde(rename = "112024")]
    N112024,
    /// Size Descriptor
    #[serde(rename = "112025")]
    N112025,
    /// Width Descriptor
    #[serde(rename = "112026")]
    N112026,
    /// Opacity Descriptor
    #[serde(rename = "112027")]
    N112027,
    /// Abnormal Distribution of Anatomic Structure
    #[serde(rename = "112028")]
    N112028,
    /// WHO
    #[serde(rename = "112029")]
    N112029,
    /// Calcification Descriptor
    #[serde(rename = "112030")]
    N112030,
    /// Attenuation Coefficient
    #[serde(rename = "112031")]
    N112031,
    /// Threshold Attenuation Coefficient
    #[serde(rename = "112032")]
    N112032,
    /// Abnormal opacity
    #[serde(rename = "112033")]
    N112033,
    /// Calculation Description
    #[serde(rename = "112034")]
    N112034,
    /// Performance of Pediatric and Adult Chest Radiography, ACR
    #[serde(rename = "112035")]
    N112035,
    /// ACR Position Statement
    #[serde(rename = "112036")]
    N112036,
    /// Non-lesion Modifier
    #[serde(rename = "112037")]
    N112037,
    /// Osseous Modifier
    #[serde(rename = "112038")]
    N112038,
    /// Tracking Identifier
    #[serde(rename = "112039")]
    N112039,
    /// Tracking Unique Identifier
    #[serde(rename = "112040")]
    N112040,
    /// Target Lesion Complete Response
    #[serde(rename = "112041")]
    N112041,
    /// Target Lesion Partial Response
    #[serde(rename = "112042")]
    N112042,
    /// Target Lesion Progressive Disease
    #[serde(rename = "112043")]
    N112043,
    /// Target Lesion Stable Disease
    #[serde(rename = "112044")]
    N112044,
    /// Non-Target Lesion Complete Response
    #[serde(rename = "112045")]
    N112045,
    /// Non-Target Lesion Incomplete Response or Stable Disease
    #[serde(rename = "112046")]
    N112046,
    /// Non-Target Lesion Progressive Disease
    #[serde(rename = "112047")]
    N112047,
    /// Current Response
    #[serde(rename = "112048")]
    N112048,
    /// Best Overall Response
    #[serde(rename = "112049")]
    N112049,
    /// Anatomic Identifier
    #[serde(rename = "112050")]
    N112050,
    /// Measurement of Response
    #[serde(rename = "112051")]
    N112051,
    /// Bronchovascular
    #[serde(rename = "112052")]
    N112052,
    /// Osseous
    #[serde(rename = "112053")]
    N112053,
    /// Secondary pulmonary lobule
    #[serde(rename = "112054")]
    N112054,
    /// Agatston scoring method
    #[serde(rename = "112055")]
    N112055,
    /// Volume scoring method
    #[serde(rename = "112056")]
    N112056,
    /// Mass scoring method
    #[serde(rename = "112057")]
    N112057,
    /// Calcium score
    #[serde(rename = "112058")]
    N112058,
    /// Primary complex
    #[serde(rename = "112059")]
    N112059,
    /// Oligemia
    #[serde(rename = "112060")]
    N112060,
    /// Abnormal lines (1D)
    #[serde(rename = "112061")]
    N112061,
    /// Abnormal lucency
    #[serde(rename = "112062")]
    N112062,
    /// Abnormal calcifications
    #[serde(rename = "112063")]
    N112063,
    /// Abnormal texture
    #[serde(rename = "112064")]
    N112064,
    /// Reticulonodular pattern
    #[serde(rename = "112065")]
    N112065,
    /// Beaded septum sign
    #[serde(rename = "112066")]
    N112066,
    /// Nodular pattern
    #[serde(rename = "112067")]
    N112067,
    /// Pseudoplaque
    #[serde(rename = "112068")]
    N112068,
    /// Signet-ring sign
    #[serde(rename = "112069")]
    N112069,
    /// Air bronchiologram
    #[serde(rename = "112070")]
    N112070,
    /// Air bronchogram
    #[serde(rename = "112071")]
    N112071,
    /// Air crescent
    #[serde(rename = "112072")]
    N112072,
    /// Halo sign
    #[serde(rename = "112073")]
    N112073,
    /// Target Lesion at Baseline
    #[serde(rename = "112074")]
    N112074,
    /// Non-Target Lesion at Baseline
    #[serde(rename = "112075")]
    N112075,
    /// Non-Lesion at Baseline
    #[serde(rename = "112076")]
    N112076,
    /// Vasoconstriction
    #[serde(rename = "112077")]
    N112077,
    /// Vasodilation
    #[serde(rename = "112078")]
    N112078,
    /// Architectural distortion
    #[serde(rename = "112079")]
    N112079,
    /// Mosaic perfusion
    #[serde(rename = "112080")]
    N112080,
    /// Pleonemia
    #[serde(rename = "112081")]
    N112081,
    /// Interface
    #[serde(rename = "112082")]
    N112082,
    /// Line
    #[serde(rename = "112083")]
    N112083,
    /// Lucency
    #[serde(rename = "112084")]
    N112084,
    /// Midlung window
    #[serde(rename = "112085")]
    N112085,
    /// Carina angle
    #[serde(rename = "112086")]
    N112086,
    /// Centrilobular structures
    #[serde(rename = "112087")]
    N112087,
    /// Anterior junction line
    #[serde(rename = "112088")]
    N112088,
    /// Posterior junction line
    #[serde(rename = "112089")]
    N112089,
    /// Azygoesophageal recess interface
    #[serde(rename = "112090")]
    N112090,
    /// Paraspinal line
    #[serde(rename = "112091")]
    N112091,
    /// Posterior tracheal stripe
    #[serde(rename = "112092")]
    N112092,
    /// Right tracheal stripe
    #[serde(rename = "112093")]
    N112093,
    /// Stripe
    #[serde(rename = "112094")]
    N112094,
    /// Hiatus
    #[serde(rename = "112095")]
    N112095,
    /// Rib Scalene Tubercle
    #[serde(rename = "112096")]
    N112096,
    /// Vertebral Intervertebral Notch
    #[serde(rename = "112097")]
    N112097,
    /// Subscapular Fossa
    #[serde(rename = "112098")]
    N112098,
    /// Scapular Spine
    #[serde(rename = "112099")]
    N112099,
    /// Scapular Supraspinatus Fossa
    #[serde(rename = "112100")]
    N112100,
    /// Scapular Infraspinatus Fossa
    #[serde(rename = "112101")]
    N112101,
    /// Aortic knob
    #[serde(rename = "112102")]
    N112102,
    /// Arch of the Azygos vein
    #[serde(rename = "112103")]
    N112103,
    /// Air-fluid level
    #[serde(rename = "112104")]
    N112104,
    /// Corona radiata
    #[serde(rename = "112105")]
    N112105,
    /// Honeycomb pattern
    #[serde(rename = "112106")]
    N112106,
    /// Fleischner's line(s)
    #[serde(rename = "112107")]
    N112107,
    /// Intralobular lines
    #[serde(rename = "112108")]
    N112108,
    /// Kerley A line
    #[serde(rename = "112109")]
    N112109,
    /// Kerley B line
    #[serde(rename = "112110")]
    N112110,
    /// Kerley C lines
    #[serde(rename = "112111")]
    N112111,
    /// Parenchymal band
    #[serde(rename = "112112")]
    N112112,
    /// Reticular pattern
    #[serde(rename = "112113")]
    N112113,
    /// Septal line(s)
    #[serde(rename = "112114")]
    N112114,
    /// Subpleural line
    #[serde(rename = "112115")]
    N112115,
    /// Tramline shadow
    #[serde(rename = "112116")]
    N112116,
    /// Tubular shadow
    #[serde(rename = "112117")]
    N112117,
    /// Density
    #[serde(rename = "112118")]
    N112118,
    /// Dependent opacity
    #[serde(rename = "112119")]
    N112119,
    /// Ground glass opacity
    #[serde(rename = "112120")]
    N112120,
    /// Infiltrate
    #[serde(rename = "112121")]
    N112121,
    /// Micronodule
    #[serde(rename = "112122")]
    N112122,
    /// Phantom tumor (pseudotumor)
    #[serde(rename = "112123")]
    N112123,
    /// Shadow
    #[serde(rename = "112124")]
    N112124,
    /// Small irregular opacities
    #[serde(rename = "112125")]
    N112125,
    /// Small rounded opacities
    #[serde(rename = "112126")]
    N112126,
    /// Tree-in-bud sign
    #[serde(rename = "112127")]
    N112127,
    /// Granular pattern
    #[serde(rename = "112128")]
    N112128,
    /// Miliary pattern
    #[serde(rename = "112129")]
    N112129,
    /// Mosaic pattern
    #[serde(rename = "112130")]
    N112130,
    /// Extremely small
    #[serde(rename = "112131")]
    N112131,
    /// Very small
    #[serde(rename = "112132")]
    N112132,
    /// Too small
    #[serde(rename = "112133")]
    N112133,
    /// Elliptic
    #[serde(rename = "112134")]
    N112134,
    /// Lobulated
    #[serde(rename = "112135")]
    N112135,
    /// Spiculated
    #[serde(rename = "112136")]
    N112136,
    /// Sharply defined
    #[serde(rename = "112137")]
    N112137,
    /// Distinctly defined
    #[serde(rename = "112138")]
    N112138,
    /// Well demarcated
    #[serde(rename = "112139")]
    N112139,
    /// Sharply demarcated
    #[serde(rename = "112140")]
    N112140,
    /// Poorly demarcated
    #[serde(rename = "112141")]
    N112141,
    /// Circumscribed
    #[serde(rename = "112142")]
    N112142,
    /// Air
    #[serde(rename = "112143")]
    N112143,
    /// Soft tissue
    #[serde(rename = "112144")]
    N112144,
    /// Calcium
    #[serde(rename = "112145")]
    N112145,
    /// Acinar
    #[serde(rename = "112146")]
    N112146,
    /// Air space
    #[serde(rename = "112147")]
    N112147,
    /// Fibronodular
    #[serde(rename = "112148")]
    N112148,
    /// Fluffy
    #[serde(rename = "112149")]
    N112149,
    /// Linear
    #[serde(rename = "112150")]
    N112150,
    /// Profusion
    #[serde(rename = "112151")]
    N112151,
    /// Silhouette sign
    #[serde(rename = "112152")]
    N112152,
    /// Subpleural
    #[serde(rename = "112153")]
    N112153,
    /// Bat's wing distribution
    #[serde(rename = "112154")]
    N112154,
    /// Butterfly distribution
    #[serde(rename = "112155")]
    N112155,
    /// Centrilobular
    #[serde(rename = "112156")]
    N112156,
    /// Coalescent
    #[serde(rename = "112157")]
    N112157,
    /// Lobar
    #[serde(rename = "112158")]
    N112158,
    /// Hyper-acute
    #[serde(rename = "112159")]
    N112159,
    /// Homogeneous (uniform opacity)
    #[serde(rename = "112160")]
    N112160,
    /// Inhomogeneous
    #[serde(rename = "112161")]
    N112161,
    /// Target
    #[serde(rename = "112162")]
    N112162,
    /// Fibrocalcific
    #[serde(rename = "112163")]
    N112163,
    /// Flocculent
    #[serde(rename = "112164")]
    N112164,
    /// Difference in border shape
    #[serde(rename = "112165")]
    N112165,
    /// Difference in border definition
    #[serde(rename = "112166")]
    N112166,
    /// Difference in distribution
    #[serde(rename = "112167")]
    N112167,
    /// Difference in site involvement
    #[serde(rename = "112168")]
    N112168,
    /// Difference in Type of Content
    #[serde(rename = "112169")]
    N112169,
    /// Difference in Texture
    #[serde(rename = "112170")]
    N112170,
    /// Fiducial mark
    #[serde(rename = "112171")]
    N112171,
    /// Portacath
    #[serde(rename = "112172")]
    N112172,
    /// Chest tube
    #[serde(rename = "112173")]
    N112173,
    /// Central line
    #[serde(rename = "112174")]
    N112174,
    /// Kidney stent
    #[serde(rename = "112175")]
    N112175,
    /// Pancreatic stent
    #[serde(rename = "112176")]
    N112176,
    /// Nipple ring
    #[serde(rename = "112177")]
    N112177,
    /// Coin
    #[serde(rename = "112178")]
    N112178,
    /// Minimum Attenuation Coefficient
    #[serde(rename = "112179")]
    N112179,
    /// Maximum Attenuation Coefficient
    #[serde(rename = "112180")]
    N112180,
    /// Mean Attenuation Coefficient
    #[serde(rename = "112181")]
    N112181,
    /// Median Attenuation Coefficient
    #[serde(rename = "112182")]
    N112182,
    /// Standard Deviation of Attenuation Coefficient
    #[serde(rename = "112183")]
    N112183,
    /// Performance of Pediatric and Adult Thoracic CT
    #[serde(rename = "112184")]
    N112184,
    /// Performance of CT for Detection of Pulmonary Embolism in Adults
    #[serde(rename = "112185")]
    N112185,
    /// Performance of High-Resolution CT of the Lungs in Adults
    #[serde(rename = "112186")]
    N112186,
    /// Unspecified method of calculation
    #[serde(rename = "112187")]
    N112187,
    /// Two-dimensional method
    #[serde(rename = "112188")]
    N112188,
    /// Three-dimensional method
    #[serde(rename = "112189")]
    N112189,
    /// Breast tissue density
    #[serde(rename = "112191")]
    N112191,
    /// Volume of parenchymal tissue
    #[serde(rename = "112192")]
    N112192,
    /// Volume of breast
    #[serde(rename = "112193")]
    N112193,
    /// Mass of parenchymal tissue
    #[serde(rename = "112194")]
    N112194,
    /// Mass of breast
    #[serde(rename = "112195")]
    N112195,
    /// Area of Vascular Calcification
    #[serde(rename = "112196")]
    N112196,
    /// Volume of Vascular Calcification
    #[serde(rename = "112197")]
    N112197,
    /// Percentage of Vascular Calcification
    #[serde(rename = "112198")]
    N112198,
    /// Mass of Vascular Calcification
    #[serde(rename = "112199")]
    N112199,
    /// Average calcification distance in a calcification cluster
    #[serde(rename = "112200")]
    N112200,
    /// Standard deviation distance of calcifications in a cluster
    #[serde(rename = "112201")]
    N112201,
    /// Colon CAD Report
    #[serde(rename = "112220")]
    N112220,
    /// Colon Overall Assessment
    #[serde(rename = "112222")]
    N112222,
    /// Image Set Properties
    #[serde(rename = "112224")]
    N112224,
    /// Slice Thickness
    #[serde(rename = "112225")]
    N112225,
    /// Spacing between slices
    #[serde(rename = "112226")]
    N112226,
    /// Frame of Reference UID
    #[serde(rename = "112227")]
    N112227,
    /// Recumbent Patient Position with respect to gravity
    #[serde(rename = "112228")]
    N112228,
    /// Identifying Segment
    #[serde(rename = "112229")]
    N112229,
    /// Polyp stalk width
    #[serde(rename = "112232")]
    N112232,
    /// Distance from anus
    #[serde(rename = "112233")]
    N112233,
    /// Anatomic non-colon
    #[serde(rename = "112238")]
    N112238,
    /// C0 - Inadequate Study/Awaiting Prior Comparisons
    #[serde(rename = "112240")]
    N112240,
    /// C1 - Normal Colon or Benign Lesion
    #[serde(rename = "112241")]
    N112241,
    /// C2 - Intermediate Polyp or Indeterminate Finding
    #[serde(rename = "112242")]
    N112242,
    /// C3 - Polyp, Possibly Advanced Adenoma
    #[serde(rename = "112243")]
    N112243,
    /// C4 - Colonic Mass, Likely Malignant
    #[serde(rename = "112244")]
    N112244,
    /// ACR Guideline, Performance of Adult CT Colonography
    #[serde(rename = "112248")]
    N112248,
    /// ACR Standard, CT medical physics performance monitoring
    #[serde(rename = "112249")]
    N112249,
    /// AP+45
    #[serde(rename = "112300")]
    N112300,
    /// AP-45
    #[serde(rename = "112301")]
    N112301,
    /// Anatomical axis of femur
    #[serde(rename = "112302")]
    N112302,
    /// Acetabular Center of Rotation
    #[serde(rename = "112303")]
    N112303,
    /// Femur Head Center of Rotation
    #[serde(rename = "112304")]
    N112304,
    /// Acetabular Cup Shell
    #[serde(rename = "112305")]
    N112305,
    /// Acetabular Cup Insert
    #[serde(rename = "112306")]
    N112306,
    /// Acetabular Cup Monoblock
    #[serde(rename = "112307")]
    N112307,
    /// Femoral Head Ball Component
    #[serde(rename = "112308")]
    N112308,
    /// Femoral Head Cone Taper Component
    #[serde(rename = "112309")]
    N112309,
    /// Femoral Stem
    #[serde(rename = "112310")]
    N112310,
    /// Femoral Stem Distal Component
    #[serde(rename = "112311")]
    N112311,
    /// Femoral Stem Proximal Component
    #[serde(rename = "112312")]
    N112312,
    /// Femoral Stem Component
    #[serde(rename = "112313")]
    N112313,
    /// Neck Component
    #[serde(rename = "112314")]
    N112314,
    /// Monoblock Stem
    #[serde(rename = "112315")]
    N112315,
    /// Prosthetic Shaft Augment
    #[serde(rename = "112316")]
    N112316,
    /// Femoral Head Resurfacing Component
    #[serde(rename = "112317")]
    N112317,
    /// Pinning
    #[serde(rename = "112318")]
    N112318,
    /// Sewing
    #[serde(rename = "112319")]
    N112319,
    /// Bolting
    #[serde(rename = "112320")]
    N112320,
    /// Wedging
    #[serde(rename = "112321")]
    N112321,
    /// Distal Centralizer
    #[serde(rename = "112325")]
    N112325,
    /// Generic 2D Planning
    #[serde(rename = "112340")]
    N112340,
    /// Generic 3D Planning
    #[serde(rename = "112341")]
    N112341,
    /// Generic Planning for Hip Replacement
    #[serde(rename = "112342")]
    N112342,
    /// Generic Planning for Knee Replacement
    #[serde(rename = "112343")]
    N112343,
    /// Müller Method Planning for Hip Replacement
    #[serde(rename = "112344")]
    N112344,
    /// Implantation Plan
    #[serde(rename = "112345")]
    N112345,
    /// Selected Implant Component
    #[serde(rename = "112346")]
    N112346,
    /// Component ID
    #[serde(rename = "112347")]
    N112347,
    /// Implant Template
    #[serde(rename = "112348")]
    N112348,
    /// Component Connection
    #[serde(rename = "112350")]
    N112350,
    /// Mating Feature Set ID
    #[serde(rename = "112351")]
    N112351,
    /// Mating Feature ID
    #[serde(rename = "112352")]
    N112352,
    /// Spatial Registration
    #[serde(rename = "112353")]
    N112353,
    /// Patient Image
    #[serde(rename = "112354")]
    N112354,
    /// Assembly
    #[serde(rename = "112355")]
    N112355,
    /// User Selected Fiducial
    #[serde(rename = "112356")]
    N112356,
    /// Derived Fiducial
    #[serde(rename = "112357")]
    N112357,
    /// Information used for planning
    #[serde(rename = "112358")]
    N112358,
    /// Supporting Information
    #[serde(rename = "112359")]
    N112359,
    /// Implant Component List
    #[serde(rename = "112360")]
    N112360,
    /// Patient Data Used During Planning
    #[serde(rename = "112361")]
    N112361,
    /// Degrees of Freedom Specification
    #[serde(rename = "112362")]
    N112362,
    /// Degree of Freedom ID
    #[serde(rename = "112363")]
    N112363,
    /// Related Patient Data Not Used During Planning
    #[serde(rename = "112364")]
    N112364,
    /// Related Implantation Reports
    #[serde(rename = "112365")]
    N112365,
    /// Implant Assembly Template
    #[serde(rename = "112366")]
    N112366,
    /// Planning Information for Intraoperative Usage
    #[serde(rename = "112367")]
    N112367,
    /// Implantation Patient Positioning
    #[serde(rename = "112368")]
    N112368,
    /// Fiducial Intent
    #[serde(rename = "112369")]
    N112369,
    /// Component Type
    #[serde(rename = "112370")]
    N112370,
    /// Manufacturer Implant Template
    #[serde(rename = "112371")]
    N112371,
    /// Derived Planning Images
    #[serde(rename = "112372")]
    N112372,
    /// Other Derived Planning Data
    #[serde(rename = "112373")]
    N112373,
    /// Connected Implantation Plan Component
    #[serde(rename = "112374")]
    N112374,
    /// Planning Method
    #[serde(rename = "112375")]
    N112375,
    /// Degree of Freedom Exact Translational Value
    #[serde(rename = "112376")]
    N112376,
    /// Degree of Freedom Minimum Translational Value
    #[serde(rename = "112377")]
    N112377,
    /// Degree of Freedom Maximum Translational Value
    #[serde(rename = "112378")]
    N112378,
    /// Degree of Freedom Exact Rotational Translation Value
    #[serde(rename = "112379")]
    N112379,
    /// Degree of Freedom Minimum Rotational Value
    #[serde(rename = "112380")]
    N112380,
    /// Degree of Freedom Maximum Rotational Value
    #[serde(rename = "112381")]
    N112381,
    /// Peri-operative Photographic Imaging
    #[serde(rename = "112700")]
    N112700,
    /// Gross Specimen Imaging
    #[serde(rename = "112701")]
    N112701,
    /// Slide Microscopy
    #[serde(rename = "112702")]
    N112702,
    /// Whole Slide Imaging
    #[serde(rename = "112703")]
    N112703,
    /// WSI 20X RGB
    #[serde(rename = "112704")]
    N112704,
    /// WSI 40X RGB
    #[serde(rename = "112705")]
    N112705,
    /// Illumination Method
    #[serde(rename = "112706")]
    N112706,
    /// Number of focal planes
    #[serde(rename = "112707")]
    N112707,
    /// Focal plane Z offset
    #[serde(rename = "112708")]
    N112708,
    /// Magnification selection
    #[serde(rename = "112709")]
    N112709,
    /// Illumination wavelength
    #[serde(rename = "112710")]
    N112710,
    /// Illumination spectral band
    #[serde(rename = "112711")]
    N112711,
    /// Optical filter type
    #[serde(rename = "112712")]
    N112712,
    /// Tissue selection method
    #[serde(rename = "112713")]
    N112713,
    /// Multiple planes
    #[serde(rename = "112714")]
    N112714,
    /// 5X
    #[serde(rename = "112715")]
    N112715,
    /// 10X
    #[serde(rename = "112716")]
    N112716,
    /// 20X
    #[serde(rename = "112717")]
    N112717,
    /// 40X
    #[serde(rename = "112718")]
    N112718,
    /// Nominal empty tile suppression
    #[serde(rename = "112719")]
    N112719,
    /// High threshold empty tile suppression
    #[serde(rename = "112720")]
    N112720,
    /// No empty tile suppression
    #[serde(rename = "112721")]
    N112721,
    /// Of Interest
    #[serde(rename = "113000")]
    N113000,
    /// Rejected for Quality Reasons
    #[serde(rename = "113001")]
    N113001,
    /// For Referring Provider
    #[serde(rename = "113002")]
    N113002,
    /// For Surgery
    #[serde(rename = "113003")]
    N113003,
    /// For Teaching
    #[serde(rename = "113004")]
    N113004,
    /// For Conference
    #[serde(rename = "113005")]
    N113005,
    /// For Therapy
    #[serde(rename = "113006")]
    N113006,
    /// For Patient
    #[serde(rename = "113007")]
    N113007,
    /// For Peer Review
    #[serde(rename = "113008")]
    N113008,
    /// For Research
    #[serde(rename = "113009")]
    N113009,
    /// Quality Issue
    #[serde(rename = "113010")]
    N113010,
    /// Document Title Modifier
    #[serde(rename = "113011")]
    N113011,
    /// Key Object Description
    #[serde(rename = "113012")]
    N113012,
    /// Best In Set
    #[serde(rename = "113013")]
    N113013,
    /// Study
    #[serde(rename = "113014")]
    N113014,
    /// Series
    #[serde(rename = "113015")]
    N113015,
    /// Performed Procedure Step
    #[serde(rename = "113016")]
    N113016,
    /// Stage-View
    #[serde(rename = "113017")]
    N113017,
    /// For Printing
    #[serde(rename = "113018")]
    N113018,
    /// For Report Attachment
    #[serde(rename = "113020")]
    N113020,
    /// For Litigation
    #[serde(rename = "113021")]
    N113021,
    /// Double exposure
    #[serde(rename = "113026")]
    N113026,
    /// Manifest
    #[serde(rename = "113030")]
    N113030,
    /// Signed Manifest
    #[serde(rename = "113031")]
    N113031,
    /// Complete Study Content
    #[serde(rename = "113032")]
    N113032,
    /// Signed Complete Study Content
    #[serde(rename = "113033")]
    N113033,
    /// Complete Acquisition Content
    #[serde(rename = "113034")]
    N113034,
    /// Signed Complete Acquisition Content
    #[serde(rename = "113035")]
    N113035,
    /// Group of Frames for Display
    #[serde(rename = "113036")]
    N113036,
    /// Rejected for Patient Safety Reasons
    #[serde(rename = "113037")]
    N113037,
    /// Incorrect Modality Worklist Entry
    #[serde(rename = "113038")]
    N113038,
    /// Data Retention Policy Expired
    #[serde(rename = "113039")]
    N113039,
    /// Lossy Compression
    #[serde(rename = "113040")]
    N113040,
    /// Apparent Diffusion Coefficient
    #[serde(rename = "113041")]
    N113041,
    /// Pixel by pixel addition
    #[serde(rename = "113042")]
    N113042,
    /// Diffusion weighted
    #[serde(rename = "113043")]
    N113043,
    /// Diffusion Anisotropy
    #[serde(rename = "113044")]
    N113044,
    /// Diffusion Attenuated
    #[serde(rename = "113045")]
    N113045,
    /// Pixel by pixel division
    #[serde(rename = "113046")]
    N113046,
    /// Pixel by pixel mask
    #[serde(rename = "113047")]
    N113047,
    /// Pixel by pixel Maximum
    #[serde(rename = "113048")]
    N113048,
    /// Pixel by pixel mean
    #[serde(rename = "113049")]
    N113049,
    /// Metabolite Maps from spectroscopy data
    #[serde(rename = "113050")]
    N113050,
    /// Pixel by pixel Minimum
    #[serde(rename = "113051")]
    N113051,
    /// Mean Transit Time
    #[serde(rename = "113052")]
    N113052,
    /// Pixel by pixel multiplication
    #[serde(rename = "113053")]
    N113053,
    /// Negative Enhancement Integral
    #[serde(rename = "113054")]
    N113054,
    /// Regional Cerebral Blood Flow
    #[serde(rename = "113055")]
    N113055,
    /// Regional Cerebral Blood Volume
    #[serde(rename = "113056")]
    N113056,
    /// R-Coefficient
    #[serde(rename = "113057")]
    N113057,
    /// Proton Density
    #[serde(rename = "113058")]
    N113058,
    /// Signal Change
    #[serde(rename = "113059")]
    N113059,
    /// Signal to Noise
    #[serde(rename = "113060")]
    N113060,
    /// Standard Deviation
    #[serde(rename = "113061")]
    N113061,
    /// Pixel by pixel subtraction
    #[serde(rename = "113062")]
    N113062,
    /// T1
    #[serde(rename = "113063")]
    N113063,
    /// T2*
    #[serde(rename = "113064")]
    N113064,
    /// T2
    #[serde(rename = "113065")]
    N113065,
    /// Time Course of Signal
    #[serde(rename = "113066")]
    N113066,
    /// Temperature encoded
    #[serde(rename = "113067")]
    N113067,
    /// Student's T-Test
    #[serde(rename = "113068")]
    N113068,
    /// Time To Peak
    #[serde(rename = "113069")]
    N113069,
    /// Velocity encoded
    #[serde(rename = "113070")]
    N113070,
    /// Z-Score
    #[serde(rename = "113071")]
    N113071,
    /// Multiplanar reformatting
    #[serde(rename = "113072")]
    N113072,
    /// Curved multiplanar reformatting
    #[serde(rename = "113073")]
    N113073,
    /// Volume rendering
    #[serde(rename = "113074")]
    N113074,
    /// Surface rendering
    #[serde(rename = "113075")]
    N113075,
    /// Segmentation
    #[serde(rename = "113076")]
    N113076,
    /// Volume editing
    #[serde(rename = "113077")]
    N113077,
    /// Maximum intensity projection
    #[serde(rename = "113078")]
    N113078,
    /// Minimum intensity projection
    #[serde(rename = "113079")]
    N113079,
    /// Glutamate and glutamine
    #[serde(rename = "113080")]
    N113080,
    /// Choline/Creatine Ratio
    #[serde(rename = "113081")]
    N113081,
    /// N-acetylaspartate /Creatine Ratio
    #[serde(rename = "113082")]
    N113082,
    /// N-acetylaspartate /Choline Ratio
    #[serde(rename = "113083")]
    N113083,
    /// Spatial resampling
    #[serde(rename = "113085")]
    N113085,
    /// Edge enhancement
    #[serde(rename = "113086")]
    N113086,
    /// Smoothing
    #[serde(rename = "113087")]
    N113087,
    /// Gaussian blur
    #[serde(rename = "113088")]
    N113088,
    /// Unsharp mask
    #[serde(rename = "113089")]
    N113089,
    /// Image stitching
    #[serde(rename = "113090")]
    N113090,
    /// Spatially-related frames extracted from the volume
    #[serde(rename = "113091")]
    N113091,
    /// Temporally-related frames extracted from the set of volumes
    #[serde(rename = "113092")]
    N113092,
    /// Polar to Rectangular Scan Conversion
    #[serde(rename = "113093")]
    N113093,
    /// Creatine and Choline
    #[serde(rename = "113094")]
    N113094,
    /// Lipid and Lactate
    #[serde(rename = "113095")]
    N113095,
    /// Creatine+Choline/ Citrate Ratio
    #[serde(rename = "113096")]
    N113096,
    /// Multi-energy proportional weighting
    #[serde(rename = "113097")]
    N113097,
    /// Basic Application Confidentiality Profile
    #[serde(rename = "113100")]
    N113100,
    /// Clean Pixel Data Option
    #[serde(rename = "113101")]
    N113101,
    /// Clean Recognizable Visual Features Option
    #[serde(rename = "113102")]
    N113102,
    /// Clean Graphics Option
    #[serde(rename = "113103")]
    N113103,
    /// Clean Structured Content Option
    #[serde(rename = "113104")]
    N113104,
    /// Clean Descriptors Option
    #[serde(rename = "113105")]
    N113105,
    /// Retain Longitudinal Temporal Information Full Dates Option
    #[serde(rename = "113106")]
    N113106,
    /// Retain Longitudinal Temporal Information Modified Dates Option
    #[serde(rename = "113107")]
    N113107,
    /// Retain Patient Characteristics Option
    #[serde(rename = "113108")]
    N113108,
    /// Retain Device Identity Option
    #[serde(rename = "113109")]
    N113109,
    /// Retain UIDs Option
    #[serde(rename = "113110")]
    N113110,
    /// Retain Safe Private Option
    #[serde(rename = "113111")]
    N113111,
    /// Radiopharmaceutical Radiation Dose Report
    #[serde(rename = "113500")]
    N113500,
    /// Radiopharmaceutical Administration
    #[serde(rename = "113502")]
    N113502,
    /// Radiopharmaceutical Administration Event UID
    #[serde(rename = "113503")]
    N113503,
    /// Intravenous Extravasation Symptoms
    #[serde(rename = "113505")]
    N113505,
    /// Estimated Extravasation Activity
    #[serde(rename = "113506")]
    N113506,
    /// Administered activity
    #[serde(rename = "113507")]
    N113507,
    /// Pre-Administration Measured Activity
    #[serde(rename = "113508")]
    N113508,
    /// Post-Administration Measured Activity
    #[serde(rename = "113509")]
    N113509,
    /// Drug Product Identifier
    #[serde(rename = "113510")]
    N113510,
    /// Radiopharmaceutical Dispense Unit Identifier
    #[serde(rename = "113511")]
    N113511,
    /// Radiopharmaceutical Lot Identifier
    #[serde(rename = "113512")]
    N113512,
    /// Reagent Vial Identifier
    #[serde(rename = "113513")]
    N113513,
    /// Radionuclide Vial Identifier
    #[serde(rename = "113514")]
    N113514,
    /// Prescription Identifier
    #[serde(rename = "113516")]
    N113516,
    /// Organ Dose Information
    #[serde(rename = "113517")]
    N113517,
    /// Organ Dose
    #[serde(rename = "113518")]
    N113518,
    /// MIRD Pamphlet 1
    #[serde(rename = "113520")]
    N113520,
    /// ICRP Publication 53
    #[serde(rename = "113521")]
    N113521,
    /// ICRP Publication 80
    #[serde(rename = "113522")]
    N113522,
    /// ICRP Publication 106
    #[serde(rename = "113523")]
    N113523,
    /// MIRDOSE
    #[serde(rename = "113526")]
    N113526,
    /// OLINDA-EXM
    #[serde(rename = "113527")]
    N113527,
    /// Package Insert
    #[serde(rename = "113528")]
    N113528,
    /// Institutionally Approved Estimates
    #[serde(rename = "113529")]
    N113529,
    /// Investigational New Drug
    #[serde(rename = "113530")]
    N113530,
    /// Activity Measurement Device
    #[serde(rename = "113540")]
    N113540,
    /// Dose Calibrator
    #[serde(rename = "113541")]
    N113541,
    /// Infusion System
    #[serde(rename = "113542")]
    N113542,
    /// Generator
    #[serde(rename = "113543")]
    N113543,
    /// Fasting Duration
    #[serde(rename = "113550")]
    N113550,
    /// Hydration Volume
    #[serde(rename = "113551")]
    N113551,
    /// Recent Physical Activity
    #[serde(rename = "113552")]
    N113552,
    /// Acute unilateral renal blockage
    #[serde(rename = "113560")]
    N113560,
    /// Low Thyroid Uptake
    #[serde(rename = "113561")]
    N113561,
    /// High Thyroid Uptake
    #[serde(rename = "113562")]
    N113562,
    /// Severely Jaundiced
    #[serde(rename = "113563")]
    N113563,
    /// Extravasation visible in image
    #[serde(rename = "113568")]
    N113568,
    /// Cockroft-Gault Formula estimation of GFR
    #[serde(rename = "113570")]
    N113570,
    /// CKD-EPI Formula estimation of GFR
    #[serde(rename = "113571")]
    N113571,
    /// Glomerular Filtration Rate (MDRD)
    #[serde(rename = "113572")]
    N113572,
    /// Glomerular Filtration Rate non-black (MDRD)
    #[serde(rename = "113573")]
    N113573,
    /// Glomerular Filtration Rate black (MDRD)
    #[serde(rename = "113574")]
    N113574,
    /// Glomerular Filtration Rate female (MDRD)
    #[serde(rename = "113575")]
    N113575,
    /// Glomerular Filtration Rate Cystatin-based formula
    #[serde(rename = "113576")]
    N113576,
    /// Glomerular Filtration Rate Creatinine-based formula (Schwartz)
    #[serde(rename = "113577")]
    N113577,
    /// Small: < 32.0 cm lateral thickness
    #[serde(rename = "113601")]
    N113601,
    /// Medium: 32.0-38.0 cm lateral thickness
    #[serde(rename = "113602")]
    N113602,
    /// Large: > 38.0 cm lateral thickness
    #[serde(rename = "113603")]
    N113603,
    /// Irradiation Event Label
    #[serde(rename = "113605")]
    N113605,
    /// Label Type
    #[serde(rename = "113606")]
    N113606,
    /// Series Number
    #[serde(rename = "113607")]
    N113607,
    /// Acquisition Number
    #[serde(rename = "113608")]
    N113608,
    /// Instance Number
    #[serde(rename = "113609")]
    N113609,
    /// Stationary Acquisition
    #[serde(rename = "113611")]
    N113611,
    /// Stepping Acquisition
    #[serde(rename = "113612")]
    N113612,
    /// Rotational Acquisition
    #[serde(rename = "113613")]
    N113613,
    /// Plane A
    #[serde(rename = "113620")]
    N113620,
    /// Plane B
    #[serde(rename = "113621")]
    N113621,
    /// Single Plane
    #[serde(rename = "113622")]
    N113622,
    /// Continuous
    #[serde(rename = "113630")]
    N113630,
    /// Pulsed
    #[serde(rename = "113631")]
    N113631,
    /// Strip filter
    #[serde(rename = "113650")]
    N113650,
    /// Wedge filter
    #[serde(rename = "113651")]
    N113651,
    /// Butterfly filter
    #[serde(rename = "113652")]
    N113652,
    /// Flat filter
    #[serde(rename = "113653")]
    N113653,
    /// Outline of lobulations
    #[serde(rename = "113661")]
    N113661,
    /// Inner limits of fuzzy margin
    #[serde(rename = "113662")]
    N113662,
    /// Outer limits of fuzzy margin
    #[serde(rename = "113663")]
    N113663,
    /// Outline of spiculations
    #[serde(rename = "113664")]
    N113664,
    /// Linear spiculation
    #[serde(rename = "113665")]
    N113665,
    /// Pixelated spiculations
    #[serde(rename = "113666")]
    N113666,
    /// Orthogonal location arc
    #[serde(rename = "113669")]
    N113669,
    /// Orthogonal location arc inner margin
    #[serde(rename = "113670")]
    N113670,
    /// Orthogonal location arc outer margin
    #[serde(rename = "113671")]
    N113671,
    /// Quality Control Intent
    #[serde(rename = "113680")]
    N113680,
    /// Phantom
    #[serde(rename = "113681")]
    N113681,
    /// ACR Accreditation Phantom - CT
    #[serde(rename = "113682")]
    N113682,
    /// ACR Accreditation Phantom - MR
    #[serde(rename = "113683")]
    N113683,
    /// ACR Accreditation Phantom - Mammography
    #[serde(rename = "113684")]
    N113684,
    /// ACR Accreditation Phantom - Stereotactic Breast Biopsy
    #[serde(rename = "113685")]
    N113685,
    /// ACR Accreditation Phantom - ECT
    #[serde(rename = "113686")]
    N113686,
    /// ACR Accreditation Phantom - PET
    #[serde(rename = "113687")]
    N113687,
    /// ACR Accreditation Phantom - ECT/PET
    #[serde(rename = "113688")]
    N113688,
    /// ACR Accreditation Phantom - PET Faceplate
    #[serde(rename = "113689")]
    N113689,
    /// IEC Head Dosimetry Phantom
    #[serde(rename = "113690")]
    N113690,
    /// IEC Body Dosimetry Phantom
    #[serde(rename = "113691")]
    N113691,
    /// NEMA XR21-2000 Phantom
    #[serde(rename = "113692")]
    N113692,
    /// X-Ray Radiation Dose Report
    #[serde(rename = "113701")]
    N113701,
    /// Accumulated X-Ray Dose Data
    #[serde(rename = "113702")]
    N113702,
    /// Projection X-Ray
    #[serde(rename = "113704")]
    N113704,
    /// Scope of Accumulation
    #[serde(rename = "113705")]
    N113705,
    /// Irradiation Event X-Ray Data
    #[serde(rename = "113706")]
    N113706,
    /// Niobium or Niobium compound
    #[serde(rename = "113710")]
    N113710,
    /// Europium or Europium compound
    #[serde(rename = "113711")]
    N113711,
    /// Calibration Protocol
    #[serde(rename = "113720")]
    N113720,
    /// Irradiation Event Type
    #[serde(rename = "113721")]
    N113721,
    /// Dose Area Product Total
    #[serde(rename = "113722")]
    N113722,
    /// Calibration Date
    #[serde(rename = "113723")]
    N113723,
    /// Calibration Responsible Party
    #[serde(rename = "113724")]
    N113724,
    /// Dose (RP) Total
    #[serde(rename = "113725")]
    N113725,
    /// Fluoro Dose Area Product Total
    #[serde(rename = "113726")]
    N113726,
    /// Acquisition Dose Area Product Total
    #[serde(rename = "113727")]
    N113727,
    /// Fluoro Dose (RP) Total
    #[serde(rename = "113728")]
    N113728,
    /// Acquisition Dose (RP) Total
    #[serde(rename = "113729")]
    N113729,
    /// Total Fluoro Time
    #[serde(rename = "113730")]
    N113730,
    /// Total Number of Radiographic Frames
    #[serde(rename = "113731")]
    N113731,
    /// Fluoro Mode
    #[serde(rename = "113732")]
    N113732,
    /// KVP
    #[serde(rename = "113733")]
    N113733,
    /// X-Ray Tube Current
    #[serde(rename = "113734")]
    N113734,
    /// Exposure Time
    #[serde(rename = "113735")]
    N113735,
    /// Exposure
    #[serde(rename = "113736")]
    N113736,
    /// Distance Source to Reference Point
    #[serde(rename = "113737")]
    N113737,
    /// Dose (RP)
    #[serde(rename = "113738")]
    N113738,
    /// Positioner Primary End Angle
    #[serde(rename = "113739")]
    N113739,
    /// Positioner Secondary End Angle
    #[serde(rename = "113740")]
    N113740,
    /// Irradiation Duration
    #[serde(rename = "113742")]
    N113742,
    /// Patient Orientation
    #[serde(rename = "113743")]
    N113743,
    /// Patient Orientation Modifier
    #[serde(rename = "113744")]
    N113744,
    /// Patient Table Relationship
    #[serde(rename = "113745")]
    N113745,
    /// Distance Source to Isocenter
    #[serde(rename = "113748")]
    N113748,
    /// Distance Source to Detector
    #[serde(rename = "113750")]
    N113750,
    /// Table Longitudinal Position
    #[serde(rename = "113751")]
    N113751,
    /// Table Lateral Position
    #[serde(rename = "113752")]
    N113752,
    /// Table Height Position
    #[serde(rename = "113753")]
    N113753,
    /// Table Head Tilt Angle
    #[serde(rename = "113754")]
    N113754,
    /// Table Horizontal Rotation Angle
    #[serde(rename = "113755")]
    N113755,
    /// Table Cradle Tilt Angle
    #[serde(rename = "113756")]
    N113756,
    /// X-Ray Filter Material
    #[serde(rename = "113757")]
    N113757,
    /// X-Ray Filter Thickness Minimum
    #[serde(rename = "113758")]
    N113758,
    /// Table Longitudinal End Position
    #[serde(rename = "113759")]
    N113759,
    /// Table Lateral End Position
    #[serde(rename = "113760")]
    N113760,
    /// Table Height End Position
    #[serde(rename = "113761")]
    N113761,
    /// Calibration Uncertainty
    #[serde(rename = "113763")]
    N113763,
    /// Acquisition Plane
    #[serde(rename = "113764")]
    N113764,
    /// Focal Spot Size
    #[serde(rename = "113766")]
    N113766,
    /// Average X-Ray Tube Current
    #[serde(rename = "113767")]
    N113767,
    /// Number of Pulses
    #[serde(rename = "113768")]
    N113768,
    /// Irradiation Event UID
    #[serde(rename = "113769")]
    N113769,
    /// Column Angulation
    #[serde(rename = "113770")]
    N113770,
    /// X-Ray Filters
    #[serde(rename = "113771")]
    N113771,
    /// X-Ray Filter Type
    #[serde(rename = "113772")]
    N113772,
    /// X-Ray Filter Thickness Maximum
    #[serde(rename = "113773")]
    N113773,
    /// Reference Point Definition
    #[serde(rename = "113780")]
    N113780,
    /// Collimated Field Height
    #[serde(rename = "113788")]
    N113788,
    /// Collimated Field Width
    #[serde(rename = "113789")]
    N113789,
    /// Collimated Field Area
    #[serde(rename = "113790")]
    N113790,
    /// Pulse Rate
    #[serde(rename = "113791")]
    N113791,
    /// Distance Source to Table Plane
    #[serde(rename = "113792")]
    N113792,
    /// Pulse Width
    #[serde(rename = "113793")]
    N113793,
    /// Dose Measurement Device
    #[serde(rename = "113794")]
    N113794,
    /// Acquired Image
    #[serde(rename = "113795")]
    N113795,
    /// DLP to E conversion via MC computation
    #[serde(rename = "113800")]
    N113800,
    /// CTDIfreeair to E conversion via MC computation
    #[serde(rename = "113801")]
    N113801,
    /// DLP to E conversion via measurement
    #[serde(rename = "113802")]
    N113802,
    /// CTDIfreeair to E conversion via measurement
    #[serde(rename = "113803")]
    N113803,
    /// Sequenced Acquisition
    #[serde(rename = "113804")]
    N113804,
    /// Constant Angle Acquisition
    #[serde(rename = "113805")]
    N113805,
    /// Stationary Acquisition
    #[serde(rename = "113806")]
    N113806,
    /// Free Acquisition
    #[serde(rename = "113807")]
    N113807,
    /// ICRP Pub 60
    #[serde(rename = "113808")]
    N113808,
    /// Start of X-Ray Irradiation
    #[serde(rename = "113809")]
    N113809,
    /// End of X-Ray Irradiation
    #[serde(rename = "113810")]
    N113810,
    /// CT Accumulated Dose Data
    #[serde(rename = "113811")]
    N113811,
    /// Total Number of Irradiation Events
    #[serde(rename = "113812")]
    N113812,
    /// CT Dose Length Product Total
    #[serde(rename = "113813")]
    N113813,
    /// CT Effective Dose Total
    #[serde(rename = "113814")]
    N113814,
    /// Patient Model
    #[serde(rename = "113815")]
    N113815,
    /// Condition Effective Dose measured
    #[serde(rename = "113816")]
    N113816,
    /// Effective Dose Phantom Type
    #[serde(rename = "113817")]
    N113817,
    /// Dosimeter Type
    #[serde(rename = "113818")]
    N113818,
    /// CT Acquisition
    #[serde(rename = "113819")]
    N113819,
    /// CT Acquisition Type
    #[serde(rename = "113820")]
    N113820,
    /// X-Ray Filter Aluminum Equivalent
    #[serde(rename = "113821")]
    N113821,
    /// CT Acquisition Parameters
    #[serde(rename = "113822")]
    N113822,
    /// Number of X-Ray Sources
    #[serde(rename = "113823")]
    N113823,
    /// Exposure Time
    #[serde(rename = "113824")]
    N113824,
    /// Scanning Length
    #[serde(rename = "113825")]
    N113825,
    /// Nominal Single Collimation Width
    #[serde(rename = "113826")]
    N113826,
    /// Nominal Total Collimation Width
    #[serde(rename = "113827")]
    N113827,
    /// Pitch Factor
    #[serde(rename = "113828")]
    N113828,
    /// CT Dose
    #[serde(rename = "113829")]
    N113829,
    /// Mean CTDIvol
    #[serde(rename = "113830")]
    N113830,
    /// CT X-Ray Source Parameters
    #[serde(rename = "113831")]
    N113831,
    /// Identification of the X-Ray Source
    #[serde(rename = "113832")]
    N113832,
    /// Maximum X-Ray Tube Current
    #[serde(rename = "113833")]
    N113833,
    /// Exposure Time per Rotation
    #[serde(rename = "113834")]
    N113834,
    /// CTDIw Phantom Type
    #[serde(rename = "113835")]
    N113835,
    /// CTDIfreeair Calculation Factor
    #[serde(rename = "113836")]
    N113836,
    /// Mean CTDIfreeair
    #[serde(rename = "113837")]
    N113837,
    /// DLP
    #[serde(rename = "113838")]
    N113838,
    /// Effective Dose
    #[serde(rename = "113839")]
    N113839,
    /// Effective Dose Conversion Factor
    #[serde(rename = "113840")]
    N113840,
    /// ICRP Pub 103
    #[serde(rename = "113841")]
    N113841,
    /// X-Ray Modulation Type
    #[serde(rename = "113842")]
    N113842,
    /// Exposure Index
    #[serde(rename = "113845")]
    N113845,
    /// Target Exposure Index
    #[serde(rename = "113846")]
    N113846,
    /// Deviation Index
    #[serde(rename = "113847")]
    N113847,
    /// Irradiation Authorizing
    #[serde(rename = "113850")]
    N113850,
    /// Irradiation Administering
    #[serde(rename = "113851")]
    N113851,
    /// Irradiation Event
    #[serde(rename = "113852")]
    N113852,
    /// Irradiation Event UID
    #[serde(rename = "113853")]
    N113853,
    /// Source of Dose Information
    #[serde(rename = "113854")]
    N113854,
    /// Total Acquisition Time
    #[serde(rename = "113855")]
    N113855,
    /// Automated Data Collection
    #[serde(rename = "113856")]
    N113856,
    /// Manual Entry
    #[serde(rename = "113857")]
    N113857,
    /// MPPS Content
    #[serde(rename = "113858")]
    N113858,
    /// Irradiating Device
    #[serde(rename = "113859")]
    N113859,
    /// 15cm from Isocenter toward Source
    #[serde(rename = "113860")]
    N113860,
    /// 30cm in Front of Image Input Surface
    #[serde(rename = "113861")]
    N113861,
    /// 1cm above Tabletop
    #[serde(rename = "113862")]
    N113862,
    /// 30cm above Tabletop
    #[serde(rename = "113863")]
    N113863,
    /// 15cm from Table Centerline
    #[serde(rename = "113864")]
    N113864,
    /// Entrance exposure to a 4.2 cm breast thickness
    #[serde(rename = "113865")]
    N113865,
    /// Copied From Image Attributes
    #[serde(rename = "113866")]
    N113866,
    /// Computed From Image Attributes
    #[serde(rename = "113867")]
    N113867,
    /// Derived From Human-Readable Reports
    #[serde(rename = "113868")]
    N113868,
    /// Person Name
    #[serde(rename = "113870")]
    N113870,
    /// Person ID
    #[serde(rename = "113871")]
    N113871,
    /// Person ID Issuer
    #[serde(rename = "113872")]
    N113872,
    /// Organization Name
    #[serde(rename = "113873")]
    N113873,
    /// Person Role in Organization
    #[serde(rename = "113874")]
    N113874,
    /// Person Role in Procedure
    #[serde(rename = "113875")]
    N113875,
    /// Device Role in Procedure
    #[serde(rename = "113876")]
    N113876,
    /// Device Name
    #[serde(rename = "113877")]
    N113877,
    /// Device Manufacturer
    #[serde(rename = "113878")]
    N113878,
    /// Device Model Name
    #[serde(rename = "113879")]
    N113879,
    /// Device Serial Number
    #[serde(rename = "113880")]
    N113880,
    /// All Planes
    #[serde(rename = "113890")]
    N113890,
    /// Length of Reconstructable Volume
    #[serde(rename = "113893")]
    N113893,
    /// Top Z Location of Reconstructable Volume
    #[serde(rename = "113895")]
    N113895,
    /// Bottom Z Location of Reconstructable Volume
    #[serde(rename = "113896")]
    N113896,
    /// Top Z Location of Scanning Length
    #[serde(rename = "113897")]
    N113897,
    /// Bottom Z Location of Scanning Length
    #[serde(rename = "113898")]
    N113898,
    /// Exposed Range
    #[serde(rename = "113899")]
    N113899,
    /// Dose Check Alert Details
    #[serde(rename = "113900")]
    N113900,
    /// DLP Alert Value Configured
    #[serde(rename = "113901")]
    N113901,
    /// CTDIvol Alert Value Configured
    #[serde(rename = "113902")]
    N113902,
    /// DLP Alert Value
    #[serde(rename = "113903")]
    N113903,
    /// CTDIvol Alert Value
    #[serde(rename = "113904")]
    N113904,
    /// Accumulated DLP Forward Estimate
    #[serde(rename = "113905")]
    N113905,
    /// Accumulated CTDIvol Forward Estimate
    #[serde(rename = "113906")]
    N113906,
    /// Reason for Proceeding
    #[serde(rename = "113907")]
    N113907,
    /// Dose Check Notification Details
    #[serde(rename = "113908")]
    N113908,
    /// DLP Notification Value Configured
    #[serde(rename = "113909")]
    N113909,
    /// CTDIvol Notification Value Configured
    #[serde(rename = "113910")]
    N113910,
    /// DLP Notification Value
    #[serde(rename = "113911")]
    N113911,
    /// CTDIvol Notification Value
    #[serde(rename = "113912")]
    N113912,
    /// DLP Forward Estimate
    #[serde(rename = "113913")]
    N113913,
    /// CTDIvol Forward Estimate
    #[serde(rename = "113914")]
    N113914,
    /// Radiation Exposure
    #[serde(rename = "113921")]
    N113921,
    /// Radioactive Substance Administered
    #[serde(rename = "113922")]
    N113922,
    /// Radiation Exposure and Protection Information
    #[serde(rename = "113923")]
    N113923,
    /// Size Specific Dose Estimation
    #[serde(rename = "113930")]
    N113930,
    /// Measured Lateral Dimension
    #[serde(rename = "113931")]
    N113931,
    /// Measured AP Dimension
    #[serde(rename = "113932")]
    N113932,
    /// Derived Effective Diameter
    #[serde(rename = "113933")]
    N113933,
    /// AAPM 204 Lateral Dimension
    #[serde(rename = "113934")]
    N113934,
    /// AAPM 204 AP Dimension
    #[serde(rename = "113935")]
    N113935,
    /// AAPM 204 Sum of Lateral and AP Dimension
    #[serde(rename = "113936")]
    N113936,
    /// AAPM 204 Effective Diameter Estimated From Patient Age
    #[serde(rename = "113937")]
    N113937,
    /// System Calculated
    #[serde(rename = "113940")]
    N113940,
    /// In Detector Plane
    #[serde(rename = "113941")]
    N113941,
    /// X-Ray Reading Device
    #[serde(rename = "113942")]
    N113942,
    /// X-Ray Source Data Available
    #[serde(rename = "113943")]
    N113943,
    /// X-Ray Mechanical Data Available
    #[serde(rename = "113944")]
    N113944,
    /// X-Ray Detector Data Available
    #[serde(rename = "113945")]
    N113945,
    /// Projection Eponymous Name
    #[serde(rename = "113946")]
    N113946,
    /// Detector Type
    #[serde(rename = "113947")]
    N113947,
    /// Direct Detector
    #[serde(rename = "113948")]
    N113948,
    /// Indirect Detector
    #[serde(rename = "113949")]
    N113949,
    /// Storage Detector
    #[serde(rename = "113950")]
    N113950,
    /// Film
    #[serde(rename = "113951")]
    N113951,
    /// Table Mount
    #[serde(rename = "113952")]
    N113952,
    /// Unmounted Detector
    #[serde(rename = "113953")]
    N113953,
    /// Upright Stand Mount
    #[serde(rename = "113954")]
    N113954,
    /// C-Arm Mount
    #[serde(rename = "113955")]
    N113955,
    /// CR/DR Mechanical Configuration
    #[serde(rename = "113956")]
    N113956,
    /// Fluoroscopy-Guided Projection Radiography System
    #[serde(rename = "113957")]
    N113957,
    /// Integrated Projection Radiography System
    #[serde(rename = "113958")]
    N113958,
    /// Cassette-based Projection Radiography System
    #[serde(rename = "113959")]
    N113959,
    /// Reconstruction Algorithm
    #[serde(rename = "113961")]
    N113961,
    /// Filtered Back Projection
    #[serde(rename = "113962")]
    N113962,
    /// Iterative Reconstruction
    #[serde(rename = "113963")]
    N113963,
    /// Procedure Step To This Point
    #[serde(rename = "113970")]
    N113970,
    /// Not a number
    #[serde(rename = "114000")]
    N114000,
    /// Negative Infinity
    #[serde(rename = "114001")]
    N114001,
    /// Positive Infinity
    #[serde(rename = "114002")]
    N114002,
    /// Divide by zero
    #[serde(rename = "114003")]
    N114003,
    /// Underflow
    #[serde(rename = "114004")]
    N114004,
    /// Overflow
    #[serde(rename = "114005")]
    N114005,
    /// Measurement failure
    #[serde(rename = "114006")]
    N114006,
    /// Measurement not attempted
    #[serde(rename = "114007")]
    N114007,
    /// Calculation failure
    #[serde(rename = "114008")]
    N114008,
    /// Value out of range
    #[serde(rename = "114009")]
    N114009,
    /// Value unknown
    #[serde(rename = "114010")]
    N114010,
    /// Value indeterminate
    #[serde(rename = "114011")]
    N114011,
    /// Time of flight
    #[serde(rename = "114201")]
    N114201,
    /// Interferometry
    #[serde(rename = "114202")]
    N114202,
    /// Laser scanning
    #[serde(rename = "114203")]
    N114203,
    /// Pattern projection
    #[serde(rename = "114204")]
    N114204,
    /// Shape from shading
    #[serde(rename = "114205")]
    N114205,
    /// Shape from motion
    #[serde(rename = "114206")]
    N114206,
    /// Confocal imaging
    #[serde(rename = "114207")]
    N114207,
    /// Point Cloud Algorithmic
    #[serde(rename = "114208")]
    N114208,
    /// Turntable Scan Method
    #[serde(rename = "114209")]
    N114209,
    /// High resolution
    #[serde(rename = "114210")]
    N114210,
    /// Fast mode
    #[serde(rename = "114211")]
    N114211,
    /// Iterative Closest Point
    #[serde(rename = "114213")]
    N114213,
    /// Freehand
    #[serde(rename = "114215")]
    N114215,
    /// Checkerboard
    #[serde(rename = "114216")]
    N114216,
    /// Quotation Mode
    #[serde(rename = "121001")]
    N121001,
    /// Quoted Source
    #[serde(rename = "121002")]
    N121002,
    /// Document
    #[serde(rename = "121003")]
    N121003,
    /// Verbal
    #[serde(rename = "121004")]
    N121004,
    /// Observer Type
    #[serde(rename = "121005")]
    N121005,
    /// Person
    #[serde(rename = "121006")]
    N121006,
    /// Device
    #[serde(rename = "121007")]
    N121007,
    /// Person Observer Name
    #[serde(rename = "121008")]
    N121008,
    /// Person Observer's Organization Name
    #[serde(rename = "121009")]
    N121009,
    /// Person Observer's Role in the Organization
    #[serde(rename = "121010")]
    N121010,
    /// Person Observer's Role in this Procedure
    #[serde(rename = "121011")]
    N121011,
    /// Device Observer UID
    #[serde(rename = "121012")]
    N121012,
    /// Device Observer Name
    #[serde(rename = "121013")]
    N121013,
    /// Device Observer Manufacturer
    #[serde(rename = "121014")]
    N121014,
    /// Device Observer Model Name
    #[serde(rename = "121015")]
    N121015,
    /// Device Observer Serial Number
    #[serde(rename = "121016")]
    N121016,
    /// Device Observer Physical Location During Observation
    #[serde(rename = "121017")]
    N121017,
    /// Procedure Study Instance UID
    #[serde(rename = "121018")]
    N121018,
    /// Procedure Study Component UID
    #[serde(rename = "121019")]
    N121019,
    /// Placer Number
    #[serde(rename = "121020")]
    N121020,
    /// Filler Number
    #[serde(rename = "121021")]
    N121021,
    /// Accession Number
    #[serde(rename = "121022")]
    N121022,
    /// Procedure Code
    #[serde(rename = "121023")]
    N121023,
    /// Subject Class
    #[serde(rename = "121024")]
    N121024,
    /// Patient
    #[serde(rename = "121025")]
    N121025,
    /// Fetus
    #[serde(rename = "121026")]
    N121026,
    /// Specimen
    #[serde(rename = "121027")]
    N121027,
    /// Subject UID
    #[serde(rename = "121028")]
    N121028,
    /// Subject Name
    #[serde(rename = "121029")]
    N121029,
    /// Subject ID
    #[serde(rename = "121030")]
    N121030,
    /// Subject Birth Date
    #[serde(rename = "121031")]
    N121031,
    /// Subject Sex
    #[serde(rename = "121032")]
    N121032,
    /// Subject Age
    #[serde(rename = "121033")]
    N121033,
    /// Subject Species
    #[serde(rename = "121034")]
    N121034,
    /// Subject Breed
    #[serde(rename = "121035")]
    N121035,
    /// Mother of fetus
    #[serde(rename = "121036")]
    N121036,
    /// Fetus number
    #[serde(rename = "121037")]
    N121037,
    /// Number of Fetuses
    #[serde(rename = "121038")]
    N121038,
    /// Specimen UID
    #[serde(rename = "121039")]
    N121039,
    /// Specimen Accession Number
    #[serde(rename = "121040")]
    N121040,
    /// Specimen Identifier
    #[serde(rename = "121041")]
    N121041,
    /// Specimen Type
    #[serde(rename = "121042")]
    N121042,
    /// Slide Identifier
    #[serde(rename = "121043")]
    N121043,
    /// Slide UID
    #[serde(rename = "121044")]
    N121044,
    /// Language
    #[serde(rename = "121045")]
    N121045,
    /// Country of Language
    #[serde(rename = "121046")]
    N121046,
    /// Language of Value
    #[serde(rename = "121047")]
    N121047,
    /// Language of Name and Value
    #[serde(rename = "121048")]
    N121048,
    /// Language of Content Item and Descendants
    #[serde(rename = "121049")]
    N121049,
    /// Equivalent Meaning of Concept Name
    #[serde(rename = "121050")]
    N121050,
    /// Equivalent Meaning of Value
    #[serde(rename = "121051")]
    N121051,
    /// Presence of property
    #[serde(rename = "121052")]
    N121052,
    /// Present
    #[serde(rename = "121053")]
    N121053,
    /// Absent
    #[serde(rename = "121054")]
    N121054,
    /// Path
    #[serde(rename = "121055")]
    N121055,
    /// Area outline
    #[serde(rename = "121056")]
    N121056,
    /// Perimeter outline
    #[serde(rename = "121057")]
    N121057,
    /// Procedure reported
    #[serde(rename = "121058")]
    N121058,
    /// Presence Undetermined
    #[serde(rename = "121059")]
    N121059,
    /// History
    #[serde(rename = "121060")]
    N121060,
    /// Request
    #[serde(rename = "121062")]
    N121062,
    /// Current Procedure Descriptions
    #[serde(rename = "121064")]
    N121064,
    /// Procedure Description
    #[serde(rename = "121065")]
    N121065,
    /// Prior Procedure Descriptions
    #[serde(rename = "121066")]
    N121066,
    /// Previous Findings
    #[serde(rename = "121068")]
    N121068,
    /// Previous Finding
    #[serde(rename = "121069")]
    N121069,
    /// Findings
    #[serde(rename = "121070")]
    N121070,
    /// Finding
    #[serde(rename = "121071")]
    N121071,
    /// Impressions
    #[serde(rename = "121072")]
    N121072,
    /// Impression
    #[serde(rename = "121073")]
    N121073,
    /// Recommendations
    #[serde(rename = "121074")]
    N121074,
    /// Recommendation
    #[serde(rename = "121075")]
    N121075,
    /// Conclusions
    #[serde(rename = "121076")]
    N121076,
    /// Conclusion
    #[serde(rename = "121077")]
    N121077,
    /// Addendum
    #[serde(rename = "121078")]
    N121078,
    /// Baseline
    #[serde(rename = "121079")]
    N121079,
    /// Best illustration of finding
    #[serde(rename = "121080")]
    N121080,
    /// Physician
    #[serde(rename = "121081")]
    N121081,
    /// Nurse
    #[serde(rename = "121082")]
    N121082,
    /// Technologist
    #[serde(rename = "121083")]
    N121083,
    /// Radiographer
    #[serde(rename = "121084")]
    N121084,
    /// Intern
    #[serde(rename = "121085")]
    N121085,
    /// Resident
    #[serde(rename = "121086")]
    N121086,
    /// Registrar
    #[serde(rename = "121087")]
    N121087,
    /// Fellow
    #[serde(rename = "121088")]
    N121088,
    /// Attending [Consultant]
    #[serde(rename = "121089")]
    N121089,
    /// Scrub nurse
    #[serde(rename = "121090")]
    N121090,
    /// Surgeon
    #[serde(rename = "121091")]
    N121091,
    /// Sonologist
    #[serde(rename = "121092")]
    N121092,
    /// Sonographer
    #[serde(rename = "121093")]
    N121093,
    /// Performing
    #[serde(rename = "121094")]
    N121094,
    /// Referring
    #[serde(rename = "121095")]
    N121095,
    /// Requesting
    #[serde(rename = "121096")]
    N121096,
    /// Recording
    #[serde(rename = "121097")]
    N121097,
    /// Verifying
    #[serde(rename = "121098")]
    N121098,
    /// Assisting
    #[serde(rename = "121099")]
    N121099,
    /// Circulating
    #[serde(rename = "121100")]
    N121100,
    /// Standby
    #[serde(rename = "121101")]
    N121101,
    /// Other sex
    #[serde(rename = "121102")]
    N121102,
    /// Undetermined sex
    #[serde(rename = "121103")]
    N121103,
    /// Ambiguous sex
    #[serde(rename = "121104")]
    N121104,
    /// Radiation Physicist
    #[serde(rename = "121105")]
    N121105,
    /// Comment
    #[serde(rename = "121106")]
    N121106,
    /// Indications for Procedure
    #[serde(rename = "121109")]
    N121109,
    /// Patient Presentation
    #[serde(rename = "121110")]
    N121110,
    /// Summary
    #[serde(rename = "121111")]
    N121111,
    /// Source of Measurement
    #[serde(rename = "121112")]
    N121112,
    /// Complications
    #[serde(rename = "121113")]
    N121113,
    /// Performing Physician
    #[serde(rename = "121114")]
    N121114,
    /// Discharge Summary
    #[serde(rename = "121115")]
    N121115,
    /// Proximal Finding Site
    #[serde(rename = "121116")]
    N121116,
    /// Distal Finding Site
    #[serde(rename = "121117")]
    N121117,
    /// Patient Characteristics
    #[serde(rename = "121118")]
    N121118,
    /// Cath Lab Procedure Log
    #[serde(rename = "121120")]
    N121120,
    /// Room identification
    #[serde(rename = "121121")]
    N121121,
    /// Equipment Identification
    #[serde(rename = "121122")]
    N121122,
    /// Patient Status or Event
    #[serde(rename = "121123")]
    N121123,
    /// Procedure Action Item ID
    #[serde(rename = "121124")]
    N121124,
    /// DateTime of Recording of Log Entry
    #[serde(rename = "121125")]
    N121125,
    /// Performed Procedure Step SOP Instance UID
    #[serde(rename = "121126")]
    N121126,
    /// Performed Procedure Step SOP Class UID
    #[serde(rename = "121127")]
    N121127,
    /// Procedure Action Duration
    #[serde(rename = "121128")]
    N121128,
    /// Start Procedure Action Item
    #[serde(rename = "121130")]
    N121130,
    /// End Procedure Action Item
    #[serde(rename = "121131")]
    N121131,
    /// Suspend Procedure Action Item
    #[serde(rename = "121132")]
    N121132,
    /// Resume Procedure Action Item
    #[serde(rename = "121133")]
    N121133,
    /// Observation DateTime Qualifier
    #[serde(rename = "121135")]
    N121135,
    /// DateTime Unsynchronized
    #[serde(rename = "121136")]
    N121136,
    /// DateTime Estimated
    #[serde(rename = "121137")]
    N121137,
    /// Image Acquired
    #[serde(rename = "121138")]
    N121138,
    /// Modality
    #[serde(rename = "121139")]
    N121139,
    /// Number of Frames
    #[serde(rename = "121140")]
    N121140,
    /// Image Type
    #[serde(rename = "121141")]
    N121141,
    /// Acquisition Duration
    #[serde(rename = "121142")]
    N121142,
    /// Waveform Acquired
    #[serde(rename = "121143")]
    N121143,
    /// Document Title
    #[serde(rename = "121144")]
    N121144,
    /// Description of Material
    #[serde(rename = "121145")]
    N121145,
    /// Quantity of Material
    #[serde(rename = "121146")]
    N121146,
    /// Billing Code
    #[serde(rename = "121147")]
    N121147,
    /// Unit Serial Identifier
    #[serde(rename = "121148")]
    N121148,
    /// Lot Identifier
    #[serde(rename = "121149")]
    N121149,
    /// Device Code
    #[serde(rename = "121150")]
    N121150,
    /// Lesion Identifier
    #[serde(rename = "121151")]
    N121151,
    /// Person administering drug/contrast
    #[serde(rename = "121152")]
    N121152,
    /// Lesion Risk
    #[serde(rename = "121153")]
    N121153,
    /// Intervention attempt identifier
    #[serde(rename = "121154")]
    N121154,
    /// Deployment
    #[serde(rename = "121155")]
    N121155,
    /// Percutaneous Entry Action
    #[serde(rename = "121156")]
    N121156,
    /// Begin Circulatory Support
    #[serde(rename = "121157")]
    N121157,
    /// End Circulatory Support
    #[serde(rename = "121158")]
    N121158,
    /// Oxygen Administration Rate
    #[serde(rename = "121160")]
    N121160,
    /// Begin Oxygen Administration
    #[serde(rename = "121161")]
    N121161,
    /// End oxygen administration
    #[serde(rename = "121162")]
    N121162,
    /// By ventilator
    #[serde(rename = "121163")]
    N121163,
    /// Patient Assessment Performed
    #[serde(rename = "121165")]
    N121165,
    /// Begin Pacing
    #[serde(rename = "121166")]
    N121166,
    /// End Pacing
    #[serde(rename = "121167")]
    N121167,
    /// Begin Ventilation
    #[serde(rename = "121168")]
    N121168,
    /// End Ventilation
    #[serde(rename = "121169")]
    N121169,
    /// Tech Note
    #[serde(rename = "121171")]
    N121171,
    /// Nursing Note
    #[serde(rename = "121172")]
    N121172,
    /// Physician Note
    #[serde(rename = "121173")]
    N121173,
    /// Procedure Note
    #[serde(rename = "121174")]
    N121174,
    /// Key Images
    #[serde(rename = "121180")]
    N121180,
    /// DICOM Object Catalog
    #[serde(rename = "121181")]
    N121181,
    /// Referenced Frames
    #[serde(rename = "121190")]
    N121190,
    /// Referenced Segment
    #[serde(rename = "121191")]
    N121191,
    /// Device Subject
    #[serde(rename = "121192")]
    N121192,
    /// Device Subject Name
    #[serde(rename = "121193")]
    N121193,
    /// Device Subject Manufacturer
    #[serde(rename = "121194")]
    N121194,
    /// Device Subject Model Name
    #[serde(rename = "121195")]
    N121195,
    /// Device Subject Serial Number
    #[serde(rename = "121196")]
    N121196,
    /// Device Subject Physical Location during observation
    #[serde(rename = "121197")]
    N121197,
    /// Device Subject UID
    #[serde(rename = "121198")]
    N121198,
    /// Illustration of ROI
    #[serde(rename = "121200")]
    N121200,
    /// Area Outline
    #[serde(rename = "121201")]
    N121201,
    /// Area of Defined Region
    #[serde(rename = "121202")]
    N121202,
    /// Distance
    #[serde(rename = "121206")]
    N121206,
    /// Height
    #[serde(rename = "121207")]
    N121207,
    /// Inter-Marker Distance
    #[serde(rename = "121208")]
    N121208,
    /// Path
    #[serde(rename = "121210")]
    N121210,
    /// Path length
    #[serde(rename = "121211")]
    N121211,
    /// Perimeter Outline
    #[serde(rename = "121213")]
    N121213,
    /// Referenced Segmentation Frame
    #[serde(rename = "121214")]
    N121214,
    /// Volume estimated from single 2D region
    #[serde(rename = "121216")]
    N121216,
    /// Volume estimated from three or more non-coplanar 2D regions
    #[serde(rename = "121217")]
    N121217,
    /// Volume estimated from two non-coplanar 2D regions
    #[serde(rename = "121218")]
    N121218,
    /// Volume of bounding three dimensional region
    #[serde(rename = "121219")]
    N121219,
    /// Volume of circumscribed sphere
    #[serde(rename = "121220")]
    N121220,
    /// Volume of ellipsoid
    #[serde(rename = "121221")]
    N121221,
    /// Volume of sphere
    #[serde(rename = "121222")]
    N121222,
    /// Path Vertex
    #[serde(rename = "121230")]
    N121230,
    /// Volume Surface
    #[serde(rename = "121231")]
    N121231,
    /// Source series for segmentation
    #[serde(rename = "121232")]
    N121232,
    /// Source image for segmentation
    #[serde(rename = "121233")]
    N121233,
    /// Distance from nipple
    #[serde(rename = "121242")]
    N121242,
    /// Distance from skin
    #[serde(rename = "121243")]
    N121243,
    /// Distance from chest wall
    #[serde(rename = "121244")]
    N121244,
    /// Patient exposure to ionizing radiation
    #[serde(rename = "121290")]
    N121290,
    /// Results communicated
    #[serde(rename = "121291")]
    N121291,
    /// Simultaneous Doppler
    #[serde(rename = "121301")]
    N121301,
    /// Simultaneous Hemodynamic
    #[serde(rename = "121302")]
    N121302,
    /// Simultaneous ECG
    #[serde(rename = "121303")]
    N121303,
    /// Simultaneous Voice Narrative
    #[serde(rename = "121304")]
    N121304,
    /// Simultaneous Respiratory Waveform
    #[serde(rename = "121305")]
    N121305,
    /// Simultaneous Arterial Pulse Waveform
    #[serde(rename = "121306")]
    N121306,
    /// Simultaneous Phonocardiographic Waveform
    #[serde(rename = "121307")]
    N121307,
    /// Localizer
    #[serde(rename = "121311")]
    N121311,
    /// Biopsy localizer
    #[serde(rename = "121312")]
    N121312,
    /// Other partial views
    #[serde(rename = "121313")]
    N121313,
    /// Other image of biplane pair
    #[serde(rename = "121314")]
    N121314,
    /// Other image of stereoscopic pair
    #[serde(rename = "121315")]
    N121315,
    /// Images related to standalone object
    #[serde(rename = "121316")]
    N121316,
    /// Spectroscopy
    #[serde(rename = "121317")]
    N121317,
    /// Spectroscopy Data for Water Phase Correction
    #[serde(rename = "121318")]
    N121318,
    /// Uncompressed predecessor
    #[serde(rename = "121320")]
    N121320,
    /// Mask image for image processing operation
    #[serde(rename = "121321")]
    N121321,
    /// Source image for image processing operation
    #[serde(rename = "121322")]
    N121322,
    /// Source series for image processing operation
    #[serde(rename = "121323")]
    N121323,
    /// Source Image
    #[serde(rename = "121324")]
    N121324,
    /// Lossy compressed image
    #[serde(rename = "121325")]
    N121325,
    /// Alternate SOP Class instance
    #[serde(rename = "121326")]
    N121326,
    /// Full fidelity image
    #[serde(rename = "121327")]
    N121327,
    /// Alternate Photometric Interpretation image
    #[serde(rename = "121328")]
    N121328,
    /// Source image for montage
    #[serde(rename = "121329")]
    N121329,
    /// Lossy compressed predecessor
    #[serde(rename = "121330")]
    N121330,
    /// Equivalent CDA Document
    #[serde(rename = "121331")]
    N121331,
    /// Complete Rendering for Presentation
    #[serde(rename = "121332")]
    N121332,
    /// Partial Rendering for Presentation
    #[serde(rename = "121333")]
    N121333,
    /// Extended Rendering for Presentation
    #[serde(rename = "121334")]
    N121334,
    /// Source Document
    #[serde(rename = "121335")]
    N121335,
    /// Anatomic image
    #[serde(rename = "121338")]
    N121338,
    /// Functional image
    #[serde(rename = "121339")]
    N121339,
    /// Spectral filtered image
    #[serde(rename = "121340")]
    N121340,
    /// Device localizer
    #[serde(rename = "121341")]
    N121341,
    /// Dose Image
    #[serde(rename = "121342")]
    N121342,
    /// Acquisition frames corresponding to volume
    #[serde(rename = "121346")]
    N121346,
    /// Volume corresponding to spatially-related acquisition frames
    #[serde(rename = "121347")]
    N121347,
    /// Temporal Predecessor
    #[serde(rename = "121348")]
    N121348,
    /// Temporal Successor
    #[serde(rename = "121349")]
    N121349,
    /// Same acquisition at lower resolution
    #[serde(rename = "121350")]
    N121350,
    /// Same acquisition at higher resolution
    #[serde(rename = "121351")]
    N121351,
    /// Same acquisition at different focal depth
    #[serde(rename = "121352")]
    N121352,
    /// Same acquisition at different spectral band
    #[serde(rename = "121353")]
    N121353,
    /// Imaged container label
    #[serde(rename = "121354")]
    N121354,
    /// For Processing predecessor
    #[serde(rename = "121358")]
    N121358,
    /// Replaced report
    #[serde(rename = "121360")]
    N121360,
    /// Addended report
    #[serde(rename = "121361")]
    N121361,
    /// Preliminary report
    #[serde(rename = "121362")]
    N121362,
    /// Partial report
    #[serde(rename = "121363")]
    N121363,
    /// Composed from prior doses
    #[serde(rename = "121370")]
    N121370,
    /// Composed from prior doses and current plan
    #[serde(rename = "121371")]
    N121371,
    /// Source dose for composing current dose
    #[serde(rename = "121372")]
    N121372,
    /// Active Ingredient Undiluted Concentration
    #[serde(rename = "121380")]
    N121380,
    /// Contrast/Bolus Ingredient Opaque
    #[serde(rename = "121381")]
    N121381,
    /// Quantity administered
    #[serde(rename = "121382")]
    N121382,
    /// Mass administered
    #[serde(rename = "121383")]
    N121383,
    /// Derivation
    #[serde(rename = "121401")]
    N121401,
    /// Normality
    #[serde(rename = "121402")]
    N121402,
    /// Level of Significance
    #[serde(rename = "121403")]
    N121403,
    /// Selection Status
    #[serde(rename = "121404")]
    N121404,
    /// Population description
    #[serde(rename = "121405")]
    N121405,
    /// Reference Authority
    #[serde(rename = "121406")]
    N121406,
    /// Normal Range description
    #[serde(rename = "121407")]
    N121407,
    /// Normal Range Authority
    #[serde(rename = "121408")]
    N121408,
    /// User chosen value
    #[serde(rename = "121410")]
    N121410,
    /// Most recent value chosen
    #[serde(rename = "121411")]
    N121411,
    /// Mean value chosen
    #[serde(rename = "121412")]
    N121412,
    /// Standard deviation of population
    #[serde(rename = "121414")]
    N121414,
    /// Percentile Ranking of measurement
    #[serde(rename = "121415")]
    N121415,
    /// Z-Score of measurement
    #[serde(rename = "121416")]
    N121416,
    /// 2 Sigma deviation of population
    #[serde(rename = "121417")]
    N121417,
    /// Equation
    #[serde(rename = "121420")]
    N121420,
    /// Equation Citation
    #[serde(rename = "121421")]
    N121421,
    /// Table of Values Citation
    #[serde(rename = "121422")]
    N121422,
    /// Method Citation
    #[serde(rename = "121423")]
    N121423,
    /// Table of Values
    #[serde(rename = "121424")]
    N121424,
    /// Index
    #[serde(rename = "121425")]
    N121425,
    /// Estimated
    #[serde(rename = "121427")]
    N121427,
    /// Calculated
    #[serde(rename = "121428")]
    N121428,
    /// Concern
    #[serde(rename = "121430")]
    N121430,
    /// DateTime Concern Noted
    #[serde(rename = "121431")]
    N121431,
    /// DateTime Concern Resolved
    #[serde(rename = "121432")]
    N121432,
    /// DateTime Problem Resolved
    #[serde(rename = "121433")]
    N121433,
    /// Service Delivery Location
    #[serde(rename = "121434")]
    N121434,
    /// Service Performer
    #[serde(rename = "121435")]
    N121435,
    /// Medical Device Used
    #[serde(rename = "121436")]
    N121436,
    /// Pharmacologic and exercise stress test
    #[serde(rename = "121437")]
    N121437,
    /// Paced stress test
    #[serde(rename = "121438")]
    N121438,
    /// Correction of congenital cardiovascular deformity
    #[serde(rename = "121439")]
    N121439,
    /// RT Patient Setup
    #[serde(rename = "121701")]
    N121701,
    /// RT Patient Position Acquisition, single plane MV
    #[serde(rename = "121702")]
    N121702,
    /// RT Patient Position Acquisition, dual plane MV
    #[serde(rename = "121703")]
    N121703,
    /// RT Patient Position Acquisition, single plane kV
    #[serde(rename = "121704")]
    N121704,
    /// RT Patient Position Acquisition, dual plane kV
    #[serde(rename = "121705")]
    N121705,
    /// RT Patient Position Acquisition, dual plane kV/MV
    #[serde(rename = "121706")]
    N121706,
    /// RT Patient Position Acquisition, CT kV
    #[serde(rename = "121707")]
    N121707,
    /// RT Patient Position Acquisition, CT MV
    #[serde(rename = "121708")]
    N121708,
    /// RT Patient Position Acquisition, Optical
    #[serde(rename = "121709")]
    N121709,
    /// RT Patient Position Acquisition, Ultrasound
    #[serde(rename = "121710")]
    N121710,
    /// RT Patient Position Acquisition, Spatial Fiducials
    #[serde(rename = "121711")]
    N121711,
    /// RT Patient Position Registration, single plane
    #[serde(rename = "121712")]
    N121712,
    /// RT Patient Position Registration, dual plane
    #[serde(rename = "121713")]
    N121713,
    /// RT Patient Position Registration, 3D CT general
    #[serde(rename = "121714")]
    N121714,
    /// RT Patient Position Registration, 3D CT marker-based
    #[serde(rename = "121715")]
    N121715,
    /// RT Patient Position Registration, 3D CT volume-based
    #[serde(rename = "121716")]
    N121716,
    /// RT Patient Position Registration, 3D on 2D reference
    #[serde(rename = "121717")]
    N121717,
    /// RT Patient Position Registration, 2D on 3D reference
    #[serde(rename = "121718")]
    N121718,
    /// RT Patient Position Registration, Optical
    #[serde(rename = "121719")]
    N121719,
    /// RT Patient Position Registration, Ultrasound
    #[serde(rename = "121720")]
    N121720,
    /// RT Patient Position Registration, Spatial Fiducials
    #[serde(rename = "121721")]
    N121721,
    /// RT Patient Position Adjustment
    #[serde(rename = "121722")]
    N121722,
    /// RT Patient Position In-treatment-session Review
    #[serde(rename = "121723")]
    N121723,
    /// RT Treatment Simulation with Internal Verification
    #[serde(rename = "121724")]
    N121724,
    /// RT Treatment Simulation with External Verification
    #[serde(rename = "121725")]
    N121725,
    /// RT Treatment with Internal Verification
    #[serde(rename = "121726")]
    N121726,
    /// RT Treatment with External Verification
    #[serde(rename = "121727")]
    N121727,
    /// RT Treatment QA with Internal Verification
    #[serde(rename = "121728")]
    N121728,
    /// RT Treatment QA with External Verification
    #[serde(rename = "121729")]
    N121729,
    /// RT Machine QA
    #[serde(rename = "121730")]
    N121730,
    /// RT Treatment QA by RT Plan Dose Check
    #[serde(rename = "121731")]
    N121731,
    /// RT Treatment QA by RT Plan Difference Check
    #[serde(rename = "121732")]
    N121732,
    /// RT Treatment QA by RT Ion Plan Dose Check
    #[serde(rename = "121733")]
    N121733,
    /// RT Treatment QA with RT Ion Plan Difference Check
    #[serde(rename = "121734")]
    N121734,
    /// Treatment Delivery Type
    #[serde(rename = "121740")]
    N121740,
    /// Patient called to procedure room
    #[serde(rename = "122001")]
    N122001,
    /// Patient admitted to procedure room
    #[serde(rename = "122002")]
    N122002,
    /// Patient given pre-procedure instruction
    #[serde(rename = "122003")]
    N122003,
    /// Patient informed consent given
    #[serde(rename = "122004")]
    N122004,
    /// Patient advance directive given
    #[serde(rename = "122005")]
    N122005,
    /// Nil Per Os (NPO) status confirmed
    #[serde(rename = "122006")]
    N122006,
    /// Patient assisted to table
    #[serde(rename = "122007")]
    N122007,
    /// Patient prepped and draped
    #[serde(rename = "122008")]
    N122008,
    /// Patient connected to continuous monitoring
    #[serde(rename = "122009")]
    N122009,
    /// Patient transferred to holding area
    #[serde(rename = "122010")]
    N122010,
    /// Patient transferred to surgery
    #[serde(rename = "122011")]
    N122011,
    /// Patient transferred to CCU
    #[serde(rename = "122012")]
    N122012,
    /// Patient disoriented
    #[serde(rename = "122020")]
    N122020,
    /// Patient reports nausea
    #[serde(rename = "122021")]
    N122021,
    /// Patient reports discomfort
    #[serde(rename = "122022")]
    N122022,
    /// Patient reports chest pain
    #[serde(rename = "122023")]
    N122023,
    /// Patient reports no pain
    #[serde(rename = "122024")]
    N122024,
    /// Patient alert
    #[serde(rename = "122025")]
    N122025,
    /// Patient restless
    #[serde(rename = "122026")]
    N122026,
    /// Patient sedated
    #[serde(rename = "122027")]
    N122027,
    /// Patient asleep
    #[serde(rename = "122028")]
    N122028,
    /// Patient unresponsive
    #[serde(rename = "122029")]
    N122029,
    /// Patient has respiratory difficulty
    #[serde(rename = "122030")]
    N122030,
    /// Patient coughed
    #[serde(rename = "122031")]
    N122031,
    /// Patient disconnected from continuous monitoring
    #[serde(rename = "122032")]
    N122032,
    /// Hemostasis achieved
    #[serde(rename = "122033")]
    N122033,
    /// Hemostasis not achieved - oozing
    #[serde(rename = "122034")]
    N122034,
    /// Hemostasis not achieved - actively bleeding
    #[serde(rename = "122035")]
    N122035,
    /// Patient given post-procedure instruction
    #[serde(rename = "122036")]
    N122036,
    /// Patient discharged from department
    #[serde(rename = "122037")]
    N122037,
    /// Patient pronounced dead
    #[serde(rename = "122038")]
    N122038,
    /// Patient transferred to morgue
    #[serde(rename = "122039")]
    N122039,
    /// Personnel Arrived
    #[serde(rename = "122041")]
    N122041,
    /// Personnel Departed
    #[serde(rename = "122042")]
    N122042,
    /// Page Sent To
    #[serde(rename = "122043")]
    N122043,
    /// Consultation With
    #[serde(rename = "122044")]
    N122044,
    /// Office called
    #[serde(rename = "122045")]
    N122045,
    /// Equipment failure
    #[serde(rename = "122046")]
    N122046,
    /// Equipment brought to procedure room
    #[serde(rename = "122047")]
    N122047,
    /// Equipment ready
    #[serde(rename = "122048")]
    N122048,
    /// Equipment removed
    #[serde(rename = "122049")]
    N122049,
    /// Bioptome
    #[serde(rename = "122052")]
    N122052,
    /// Valvular Intervention
    #[serde(rename = "122053")]
    N122053,
    /// Aortic Intervention
    #[serde(rename = "122054")]
    N122054,
    /// Septal Defect Intervention
    #[serde(rename = "122055")]
    N122055,
    /// Vascular Intervention
    #[serde(rename = "122056")]
    N122056,
    /// Myocardial biopsy
    #[serde(rename = "122057")]
    N122057,
    /// Arterial conduit angiography
    #[serde(rename = "122058")]
    N122058,
    /// Single plane Angiography
    #[serde(rename = "122059")]
    N122059,
    /// Bi-plane Angiography
    #[serde(rename = "122060")]
    N122060,
    /// Percutaneous Coronary Intervention
    #[serde(rename = "122061")]
    N122061,
    /// 15-Lead ECG
    #[serde(rename = "122062")]
    N122062,
    /// Pre-procedure log
    #[serde(rename = "122072")]
    N122072,
    /// Current procedure evidence
    #[serde(rename = "122073")]
    N122073,
    /// Prior report for current patient
    #[serde(rename = "122075")]
    N122075,
    /// Consumable taken from inventory
    #[serde(rename = "122076")]
    N122076,
    /// Consumable returned to inventory
    #[serde(rename = "122077")]
    N122077,
    /// Remaining consumable disposed
    #[serde(rename = "122078")]
    N122078,
    /// Consumable unusable
    #[serde(rename = "122079")]
    N122079,
    /// Drug start
    #[serde(rename = "122081")]
    N122081,
    /// Drug end
    #[serde(rename = "122082")]
    N122082,
    /// Drug administered
    #[serde(rename = "122083")]
    N122083,
    /// Contrast start
    #[serde(rename = "122084")]
    N122084,
    /// Contrast end
    #[serde(rename = "122085")]
    N122085,
    /// Contrast administered
    #[serde(rename = "122086")]
    N122086,
    /// Infusate start
    #[serde(rename = "122087")]
    N122087,
    /// Infusate end
    #[serde(rename = "122088")]
    N122088,
    /// Device crossed lesion
    #[serde(rename = "122089")]
    N122089,
    /// Intervention Action
    #[serde(rename = "122090")]
    N122090,
    /// Volume administered
    #[serde(rename = "122091")]
    N122091,
    /// Undiluted dose administered
    #[serde(rename = "122092")]
    N122092,
    /// Concentration
    #[serde(rename = "122093")]
    N122093,
    /// Rate of administration
    #[serde(rename = "122094")]
    N122094,
    /// Duration of administration
    #[serde(rename = "122095")]
    N122095,
    /// Volume unadministered or discarded
    #[serde(rename = "122096")]
    N122096,
    /// Catheter Curve
    #[serde(rename = "122097")]
    N122097,
    /// Transmit Frequency
    #[serde(rename = "122098")]
    N122098,
    /// ST change from baseline
    #[serde(rename = "122099")]
    N122099,
    /// Aneurysm on cited vessel
    #[serde(rename = "122101")]
    N122101,
    /// Graft to cited segment, proximal section
    #[serde(rename = "122102")]
    N122102,
    /// Graft to cited segment, mid section
    #[serde(rename = "122103")]
    N122103,
    /// Graft to cited segment, distal section
    #[serde(rename = "122104")]
    N122104,
    /// DateTime of Intervention
    #[serde(rename = "122105")]
    N122105,
    /// Duration of Intervention
    #[serde(rename = "122106")]
    N122106,
    /// Baseline Stenosis Measurement
    #[serde(rename = "122107")]
    N122107,
    /// Post-Intervention Stenosis Measurement
    #[serde(rename = "122108")]
    N122108,
    /// Baseline TIMI Flow
    #[serde(rename = "122109")]
    N122109,
    /// Post-Intervention TIMI Flow
    #[serde(rename = "122110")]
    N122110,
    /// Primary Intervention Device
    #[serde(rename = "122111")]
    N122111,
    /// Normal Myocardium
    #[serde(rename = "122112")]
    N122112,
    /// Sacrred Myocardial
    #[serde(rename = "122113")]
    N122113,
    /// Thinning Myocardium
    #[serde(rename = "122114")]
    N122114,
    /// Hemodynamics Report
    #[serde(rename = "122120")]
    N122120,
    /// Atrial pressure measurements
    #[serde(rename = "122121")]
    N122121,
    /// Ventricular pressure measurements
    #[serde(rename = "122122")]
    N122122,
    /// Gradient assessment
    #[serde(rename = "122123")]
    N122123,
    /// Blood velocity measurements
    #[serde(rename = "122124")]
    N122124,
    /// Blood lab measurements
    #[serde(rename = "122125")]
    N122125,
    /// Derived Hemodynamic Measurements
    #[serde(rename = "122126")]
    N122126,
    /// Clinical Context
    #[serde(rename = "122127")]
    N122127,
    /// Patient Transferred From
    #[serde(rename = "122128")]
    N122128,
    /// PCI during this procedure
    #[serde(rename = "122129")]
    N122129,
    /// Dose Area Product
    #[serde(rename = "122130")]
    N122130,
    /// Degree of Thrombus
    #[serde(rename = "122131")]
    N122131,
    /// Severity of Calcification
    #[serde(rename = "122132")]
    N122132,
    /// Lesion Morphology
    #[serde(rename = "122133")]
    N122133,
    /// Vessel Morphology
    #[serde(rename = "122134")]
    N122134,
    /// Circulatory Support
    #[serde(rename = "122138")]
    N122138,
    /// Reason for Exam
    #[serde(rename = "122139")]
    N122139,
    /// Comparison with Prior Exam Done
    #[serde(rename = "122140")]
    N122140,
    /// Electrode Placement
    #[serde(rename = "122141")]
    N122141,
    /// Acquisition Device Type
    #[serde(rename = "122142")]
    N122142,
    /// Acquisition Device ID
    #[serde(rename = "122143")]
    N122143,
    /// Quantitative Analysis
    #[serde(rename = "122144")]
    N122144,
    /// Qualitative Analysis
    #[serde(rename = "122145")]
    N122145,
    /// Procedure DateTime
    #[serde(rename = "122146")]
    N122146,
    /// Clinical Interpretation
    #[serde(rename = "122147")]
    N122147,
    /// Lead ID
    #[serde(rename = "122148")]
    N122148,
    /// Beat Number
    #[serde(rename = "122149")]
    N122149,
    /// Compound Statement
    #[serde(rename = "122150")]
    N122150,
    /// Trend
    #[serde(rename = "122151")]
    N122151,
    /// Statement
    #[serde(rename = "122152")]
    N122152,
    /// Statement Modifier
    #[serde(rename = "122153")]
    N122153,
    /// Conjunctive Term
    #[serde(rename = "122154")]
    N122154,
    /// Probability
    #[serde(rename = "122157")]
    N122157,
    /// ECG Global Measurements
    #[serde(rename = "122158")]
    N122158,
    /// ECG Lead Measurements
    #[serde(rename = "122159")]
    N122159,
    /// Derived Area, Non-Valve
    #[serde(rename = "122160")]
    N122160,
    /// Pulmonary Flow
    #[serde(rename = "122161")]
    N122161,
    /// Systemic Flow
    #[serde(rename = "122162")]
    N122162,
    /// Discharge DateTime
    #[serde(rename = "122163")]
    N122163,
    /// Coronary Artery Bypass During This Admission
    #[serde(rename = "122164")]
    N122164,
    /// Date of Death
    #[serde(rename = "122165")]
    N122165,
    /// Death During This Admission
    #[serde(rename = "122166")]
    N122166,
    /// Death During Catheterization
    #[serde(rename = "122167")]
    N122167,
    /// Type of Myocardial Infarction
    #[serde(rename = "122170")]
    N122170,
    /// Coronary lesion > = 50% stenosis
    #[serde(rename = "122171")]
    N122171,
    /// Acute MI Present
    #[serde(rename = "122172")]
    N122172,
    /// ST Elevation Onset DateTime
    #[serde(rename = "122173")]
    N122173,
    /// Number of lesion interventions attempted
    #[serde(rename = "122175")]
    N122175,
    /// Number of lesion interventions successful
    #[serde(rename = "122176")]
    N122176,
    /// Procedure Result
    #[serde(rename = "122177")]
    N122177,
    /// Lesion Intervention Information
    #[serde(rename = "122178")]
    N122178,
    /// Peri-procedural MI occurred
    #[serde(rename = "122179")]
    N122179,
    /// CK-MB baseline
    #[serde(rename = "122180")]
    N122180,
    /// CK-MB peak
    #[serde(rename = "122181")]
    N122181,
    /// R-R interval
    #[serde(rename = "122182")]
    N122182,
    /// Blood temperature
    #[serde(rename = "122183")]
    N122183,
    /// Blood Oxygen content
    #[serde(rename = "122185")]
    N122185,
    /// Blood Carbon dioxide saturation
    #[serde(rename = "122187")]
    N122187,
    /// Pulmonary Arterial Content (FCpa)
    #[serde(rename = "122188")]
    N122188,
    /// Pulmonary Venous Content (FCpv)
    #[serde(rename = "122189")]
    N122189,
    /// Max dp/dt/P
    #[serde(rename = "122190")]
    N122190,
    /// Ventricular End Diastolic pressure
    #[serde(rename = "122191")]
    N122191,
    /// Indicator appearance time
    #[serde(rename = "122192")]
    N122192,
    /// Maximum pressure acceleration
    #[serde(rename = "122193")]
    N122193,
    /// Ventricular Systolic blood pressure
    #[serde(rename = "122194")]
    N122194,
    /// Pulse Strength
    #[serde(rename = "122195")]
    N122195,
    /// C wave pressure
    #[serde(rename = "122196")]
    N122196,
    /// Gradient pressure, average
    #[serde(rename = "122197")]
    N122197,
    /// Gradient pressure, peak
    #[serde(rename = "122198")]
    N122198,
    /// Pressure at dp/dt max
    #[serde(rename = "122199")]
    N122199,
    /// Diastolic blood velocity, mean
    #[serde(rename = "122201")]
    N122201,
    /// Diastolic blood velocity, peak
    #[serde(rename = "122202")]
    N122202,
    /// Systolic blood velocity, mean
    #[serde(rename = "122203")]
    N122203,
    /// Systolic blood velocity, peak
    #[serde(rename = "122204")]
    N122204,
    /// Blood velocity, mean
    #[serde(rename = "122205")]
    N122205,
    /// Blood velocity, minimum
    #[serde(rename = "122206")]
    N122206,
    /// Blood velocity, peak
    #[serde(rename = "122207")]
    N122207,
    /// x-descent pressure
    #[serde(rename = "122208")]
    N122208,
    /// y-descent pressure
    #[serde(rename = "122209")]
    N122209,
    /// z-point pressure
    #[serde(rename = "122210")]
    N122210,
    /// Left Ventricular ejection time
    #[serde(rename = "122211")]
    N122211,
    /// Left Ventricular filling time
    #[serde(rename = "122212")]
    N122212,
    /// Right Ventricular ejection time
    #[serde(rename = "122213")]
    N122213,
    /// Right Ventricular filling time
    #[serde(rename = "122214")]
    N122214,
    /// Total Pulmonary Resistance
    #[serde(rename = "122215")]
    N122215,
    /// Total Vascular Resistance
    #[serde(rename = "122216")]
    N122216,
    /// Coronary Flow reserve
    #[serde(rename = "122217")]
    N122217,
    /// Diastolic/Systolic velocity ratio
    #[serde(rename = "122218")]
    N122218,
    /// Hyperemic ratio
    #[serde(rename = "122219")]
    N122219,
    /// Hemodynamic Resistance Index
    #[serde(rename = "122220")]
    N122220,
    /// Thorax diameter, sagittal
    #[serde(rename = "122221")]
    N122221,
    /// Procedure Environmental Characteristics
    #[serde(rename = "122222")]
    N122222,
    /// Room oxygen concentration
    #[serde(rename = "122223")]
    N122223,
    /// Room temperature
    #[serde(rename = "122224")]
    N122224,
    /// Room Barometric pressure
    #[serde(rename = "122225")]
    N122225,
    /// Left to Right Flow
    #[serde(rename = "122227")]
    N122227,
    /// Right to Left Flow
    #[serde(rename = "122228")]
    N122228,
    /// Arteriovenous difference
    #[serde(rename = "122229")]
    N122229,
    /// 10 Year CHD Risk
    #[serde(rename = "122230")]
    N122230,
    /// Comparative Average10 Year CHD Risk
    #[serde(rename = "122231")]
    N122231,
    /// Comparative Low10 Year CHD Risk
    #[serde(rename = "122232")]
    N122232,
    /// LDL Cholesterol Score Sheet for Men
    #[serde(rename = "122233")]
    N122233,
    /// LDL Cholesterol Score Sheet for Women
    #[serde(rename = "122234")]
    N122234,
    /// Total Cholesterol Score Sheet for Men
    #[serde(rename = "122235")]
    N122235,
    /// Total Cholesterol Score Sheet for Women
    #[serde(rename = "122236")]
    N122236,
    /// Corrected Sinus Node Recovery Time
    #[serde(rename = "122237")]
    N122237,
    /// Max volume normalized to 50mmHg pulse pressure
    #[serde(rename = "122238")]
    N122238,
    /// Oxygen Consumption
    #[serde(rename = "122239")]
    N122239,
    /// BSA = 3.207*WT^(0.7285-0.0188 log (WT)) *HT^0.3
    #[serde(rename = "122240")]
    N122240,
    /// BSA = 0.007184*WT^ 0.425*HT^0.725
    #[serde(rename = "122241")]
    N122241,
    /// BSA = 0.0235*WT^0.51456*HT^ 0.42246
    #[serde(rename = "122242")]
    N122242,
    /// BSA = 0.024265*WT^0.5378*HT^0.3964
    #[serde(rename = "122243")]
    N122243,
    /// BSA = (HT * WT/36) ^0.5
    #[serde(rename = "122244")]
    N122244,
    /// BSA = 1321+0.3433*WT
    #[serde(rename = "122245")]
    N122245,
    /// BSA = 0.0004688 * WT ^ (0.8168 - 0.0154 * log(WT))
    #[serde(rename = "122246")]
    N122246,
    /// VO2male = BSA (138.1 - 11.49 * loge(age) + 0.378 * HRf)
    #[serde(rename = "122247")]
    N122247,
    /// VO2female = BSA (138.1 - 17.04 * loge(age) + 0.378 * HRf)
    #[serde(rename = "122248")]
    N122248,
    /// VO2 = VeSTPD * 10 * (FIO2 - FE02)
    #[serde(rename = "122249")]
    N122249,
    /// VO2 = 152 * BSA
    #[serde(rename = "122250")]
    N122250,
    /// VO2 = 175 * BSA
    #[serde(rename = "122251")]
    N122251,
    /// VO2 = 176 * BSA
    #[serde(rename = "122252")]
    N122252,
    /// Robertson & Reid table
    #[serde(rename = "122253")]
    N122253,
    /// Fleisch table
    #[serde(rename = "122254")]
    N122254,
    /// Boothby table
    #[serde(rename = "122255")]
    N122255,
    /// if (prem age< 3days) P50 = 19.9
    #[serde(rename = "122256")]
    N122256,
    /// if (age < 1day) P50 = 21.6
    #[serde(rename = "122257")]
    N122257,
    /// if (age < 30day) P50 = 24.6
    #[serde(rename = "122258")]
    N122258,
    /// if (age < 18y) P50 = 27.2
    #[serde(rename = "122259")]
    N122259,
    /// if (age < 40y) P50 = 27.4
    #[serde(rename = "122260")]
    N122260,
    /// if (age > 60y) P50 = 29.3
    #[serde(rename = "122261")]
    N122261,
    /// Area = Flow / 44.5 * sqrt(Gradient[mmHg])
    #[serde(rename = "122262")]
    N122262,
    /// MVA = Flow / 38.0 * sqrt(Gradient[mmHg])
    #[serde(rename = "122263")]
    N122263,
    /// BMI = Wt / Ht ^ 2
    #[serde(rename = "122265")]
    N122265,
    /// BSA = 0.007358 * WT ^ 0.425 * HT ^ 0.725
    #[serde(rename = "122266")]
    N122266,
    /// BSA = 0.010265 * WT ^ 0.423 * HT ^ 0.651
    #[serde(rename = "122267")]
    N122267,
    /// BSA = 0.008883 * WT ^ 0.444 * HT ^ 0.663
    #[serde(rename = "122268")]
    N122268,
    /// BSA = 0.038189 * WT ^ 0.423 * HT ^ 0.362
    #[serde(rename = "122269")]
    N122269,
    /// BSA = 0.009568 * WT ^ 0.473 * HT ^ 0.655
    #[serde(rename = "122270")]
    N122270,
    /// Skin Condition Warm
    #[serde(rename = "122271")]
    N122271,
    /// Skin Condition Cool
    #[serde(rename = "122272")]
    N122272,
    /// Skin Condition Cold
    #[serde(rename = "122273")]
    N122273,
    /// Skin Condition Dry
    #[serde(rename = "122274")]
    N122274,
    /// Skin Condition Clammy
    #[serde(rename = "122275")]
    N122275,
    /// Skin Condition Diaphoretic
    #[serde(rename = "122276")]
    N122276,
    /// Skin Condition Flush
    #[serde(rename = "122277")]
    N122277,
    /// Skin Condition Mottled
    #[serde(rename = "122278")]
    N122278,
    /// Skin Condition Pale
    #[serde(rename = "122279")]
    N122279,
    /// Airway unobstructed
    #[serde(rename = "122281")]
    N122281,
    /// Airway partially obstructed
    #[serde(rename = "122282")]
    N122282,
    /// Airway severely obstructed
    #[serde(rename = "122283")]
    N122283,
    /// Not Visualized
    #[serde(rename = "122288")]
    N122288,
    /// Quantitative Arteriography Report
    #[serde(rename = "122291")]
    N122291,
    /// Quantitative Ventriculography Report
    #[serde(rename = "122292")]
    N122292,
    /// Guidewire crossing lesion unsuccessful
    #[serde(rename = "122301")]
    N122301,
    /// Guidewire crossing lesion successful
    #[serde(rename = "122302")]
    N122302,
    /// Angioplasty balloon inflated
    #[serde(rename = "122303")]
    N122303,
    /// Angioplasty balloon deflated
    #[serde(rename = "122304")]
    N122304,
    /// Device deployed
    #[serde(rename = "122305")]
    N122305,
    /// Stent re-expanded
    #[serde(rename = "122306")]
    N122306,
    /// Object removed
    #[serde(rename = "122307")]
    N122307,
    /// Radiation applied
    #[serde(rename = "122308")]
    N122308,
    /// Radiation removed
    #[serde(rename = "122309")]
    N122309,
    /// Interventional device placement unsuccessful
    #[serde(rename = "122310")]
    N122310,
    /// Interventional device placed
    #[serde(rename = "122311")]
    N122311,
    /// Intervention performed
    #[serde(rename = "122312")]
    N122312,
    /// Interventional device withdrawn
    #[serde(rename = "122313")]
    N122313,
    /// Catheter Size
    #[serde(rename = "122319")]
    N122319,
    /// Injectate Temperature
    #[serde(rename = "122320")]
    N122320,
    /// Injectate Volume
    #[serde(rename = "122321")]
    N122321,
    /// Calibration Factor
    #[serde(rename = "122322")]
    N122322,
    /// IVUS Report
    #[serde(rename = "122325")]
    N122325,
    /// EEM Diameter
    #[serde(rename = "122330")]
    N122330,
    /// Plaque Plus Media Thickness
    #[serde(rename = "122331")]
    N122331,
    /// Lumen Perimeter
    #[serde(rename = "122332")]
    N122332,
    /// EEM Cross-Sectional Area
    #[serde(rename = "122333")]
    N122333,
    /// Plaque plus Media Cross-Sectional Area
    #[serde(rename = "122334")]
    N122334,
    /// In-Stent Neointimal Cross-Sectional Area
    #[serde(rename = "122335")]
    N122335,
    /// Vascular Volume measurement length
    #[serde(rename = "122336")]
    N122336,
    /// Relative position
    #[serde(rename = "122337")]
    N122337,
    /// Stent Volume Obstruction
    #[serde(rename = "122339")]
    N122339,
    /// Fiducial feature
    #[serde(rename = "122340")]
    N122340,
    /// Calcium Length
    #[serde(rename = "122341")]
    N122341,
    /// Lumen Eccentricity Index
    #[serde(rename = "122343")]
    N122343,
    /// Plaque plus Media Eccentricity Index
    #[serde(rename = "122344")]
    N122344,
    /// Remodeling Index
    #[serde(rename = "122345")]
    N122345,
    /// Stent Symmetry Index
    #[serde(rename = "122346")]
    N122346,
    /// Stent Expansion Index
    #[serde(rename = "122347")]
    N122347,
    /// Lumen Shape Index
    #[serde(rename = "122348")]
    N122348,
    /// Lumen Diameter Ratio
    #[serde(rename = "122350")]
    N122350,
    /// Stent Diameter Ratio
    #[serde(rename = "122351")]
    N122351,
    /// EEM Diameter Ratio
    #[serde(rename = "122352")]
    N122352,
    /// Plaque Burden
    #[serde(rename = "122354")]
    N122354,
    /// Arc of Calcium
    #[serde(rename = "122355")]
    N122355,
    /// Soft plaque
    #[serde(rename = "122356")]
    N122356,
    /// In-Stent Neointima
    #[serde(rename = "122357")]
    N122357,
    /// True Lumen
    #[serde(rename = "122360")]
    N122360,
    /// False Lumen
    #[serde(rename = "122361")]
    N122361,
    /// Plaque Rupture
    #[serde(rename = "122363")]
    N122363,
    /// Stent Gap
    #[serde(rename = "122364")]
    N122364,
    /// T-1 Worst
    #[serde(rename = "122367")]
    N122367,
    /// T-2 Secondary
    #[serde(rename = "122368")]
    N122368,
    /// T-3 Secondary
    #[serde(rename = "122369")]
    N122369,
    /// T-4 Secondary
    #[serde(rename = "122370")]
    N122370,
    /// EEM Volume
    #[serde(rename = "122371")]
    N122371,
    /// Lumen Volume
    #[serde(rename = "122372")]
    N122372,
    /// In-Stent Neointimal Volume
    #[serde(rename = "122374")]
    N122374,
    /// Native Plaque Volume
    #[serde(rename = "122375")]
    N122375,
    /// Total Plaque Volume
    #[serde(rename = "122376")]
    N122376,
    /// Proximal Reference
    #[serde(rename = "122380")]
    N122380,
    /// Distal Reference
    #[serde(rename = "122381")]
    N122381,
    /// Site of Lumen Minimum
    #[serde(rename = "122382")]
    N122382,
    /// Entire Pullback
    #[serde(rename = "122383")]
    N122383,
    /// Stented Region
    #[serde(rename = "122384")]
    N122384,
    /// Proximal Stent Margin
    #[serde(rename = "122385")]
    N122385,
    /// Distal Stent Margin
    #[serde(rename = "122386")]
    N122386,
    /// Dissection Classification
    #[serde(rename = "122387")]
    N122387,
    /// Intra-stent Dissection
    #[serde(rename = "122388")]
    N122388,
    /// Vulnerable Plaque
    #[serde(rename = "122389")]
    N122389,
    /// Eroded Plaque
    #[serde(rename = "122390")]
    N122390,
    /// Relative Stenosis Severity
    #[serde(rename = "122391")]
    N122391,
    /// Restenotic Lesion
    #[serde(rename = "122393")]
    N122393,
    /// Fibro-Lipidic Plaque
    #[serde(rename = "122394")]
    N122394,
    /// Necrotic-Lipidic Plaque
    #[serde(rename = "122395")]
    N122395,
    /// Intimal Dissection
    #[serde(rename = "122398")]
    N122398,
    /// Medial Dissection
    #[serde(rename = "122399")]
    N122399,
    /// Simultaneously Acquired
    #[serde(rename = "122400")]
    N122400,
    /// Same Anatomy
    #[serde(rename = "122401")]
    N122401,
    /// Same Indication
    #[serde(rename = "122402")]
    N122402,
    /// For Attenuation Correction
    #[serde(rename = "122403")]
    N122403,
    /// Reconstructed
    #[serde(rename = "122404")]
    N122404,
    /// Algorithm Manufacturer
    #[serde(rename = "122405")]
    N122405,
    /// Left Atrial Ejection Fraction by Angiography
    #[serde(rename = "122406")]
    N122406,
    /// Left Atrial ED Volume
    #[serde(rename = "122407")]
    N122407,
    /// Left Atrial ES Volume
    #[serde(rename = "122408")]
    N122408,
    /// Contour Realignment
    #[serde(rename = "122410")]
    N122410,
    /// Threshold Value
    #[serde(rename = "122411")]
    N122411,
    /// Regional Abnormal Wall Motion
    #[serde(rename = "122417")]
    N122417,
    /// Calibration Object
    #[serde(rename = "122421")]
    N122421,
    /// Calibration Method
    #[serde(rename = "122422")]
    N122422,
    /// Calibration Object Size
    #[serde(rename = "122423")]
    N122423,
    /// Area Length Method
    #[serde(rename = "122428")]
    N122428,
    /// Volume Method
    #[serde(rename = "122429")]
    N122429,
    /// Reference Method
    #[serde(rename = "122430")]
    N122430,
    /// Regression Slope ED
    #[serde(rename = "122431")]
    N122431,
    /// Regression Offset ED
    #[serde(rename = "122432")]
    N122432,
    /// Regression Slope ES
    #[serde(rename = "122433")]
    N122433,
    /// Regression Offset ES
    #[serde(rename = "122434")]
    N122434,
    /// Regression Volume Exponent
    #[serde(rename = "122435")]
    N122435,
    /// Reference Points
    #[serde(rename = "122438")]
    N122438,
    /// Wall Thickness
    #[serde(rename = "122445")]
    N122445,
    /// Wall Volume
    #[serde(rename = "122446")]
    N122446,
    /// Wall Mass
    #[serde(rename = "122447")]
    N122447,
    /// Wall Stress
    #[serde(rename = "122448")]
    N122448,
    /// Centerline Wall Motion Analysis
    #[serde(rename = "122449")]
    N122449,
    /// Normalized Chord Length
    #[serde(rename = "122450")]
    N122450,
    /// Abnormal Region
    #[serde(rename = "122451")]
    N122451,
    /// First Chord of Abnormal Region
    #[serde(rename = "122452")]
    N122452,
    /// Last Chord of Abnormal Region
    #[serde(rename = "122453")]
    N122453,
    /// Territory Region Severity
    #[serde(rename = "122459")]
    N122459,
    /// Opposite Region Severity
    #[serde(rename = "122461")]
    N122461,
    /// LAD Region in RAO Projection
    #[serde(rename = "122464")]
    N122464,
    /// RCA Region in ROA Projection
    #[serde(rename = "122465")]
    N122465,
    /// Single LAD Region in RAO Projection
    #[serde(rename = "122466")]
    N122466,
    /// Single RCA Region in RAO Projection
    #[serde(rename = "122467")]
    N122467,
    /// Multiple LAD Region in RAO Projection
    #[serde(rename = "122468")]
    N122468,
    /// Multiple RCA Region in RAO Projection
    #[serde(rename = "122469")]
    N122469,
    /// LAD Region in LAO Projection
    #[serde(rename = "122470")]
    N122470,
    /// RCA Region in LAO Projection
    #[serde(rename = "122471")]
    N122471,
    /// CFX Region in LAO Projection
    #[serde(rename = "122472")]
    N122472,
    /// Circular Method
    #[serde(rename = "122473")]
    N122473,
    /// Densitometric Method
    #[serde(rename = "122474")]
    N122474,
    /// Center of Gravity
    #[serde(rename = "122475")]
    N122475,
    /// Long Axis Based
    #[serde(rename = "122476")]
    N122476,
    /// No Realignment
    #[serde(rename = "122477")]
    N122477,
    /// Vessel Lumen Cross-Sectional Area
    #[serde(rename = "122480")]
    N122480,
    /// Contour Start
    #[serde(rename = "122481")]
    N122481,
    /// Contour End
    #[serde(rename = "122482")]
    N122482,
    /// Sphere
    #[serde(rename = "122485")]
    N122485,
    /// Geometric Isocenter
    #[serde(rename = "122486")]
    N122486,
    /// Geometric Non-Isocenter
    #[serde(rename = "122487")]
    N122487,
    /// Calibration Object Used
    #[serde(rename = "122488")]
    N122488,
    /// Curve Fitted Reference
    #[serde(rename = "122489")]
    N122489,
    /// Interpolated Local Reference
    #[serde(rename = "122490")]
    N122490,
    /// Mean Local Reference
    #[serde(rename = "122491")]
    N122491,
    /// Radial Based Wall Motion Analysis
    #[serde(rename = "122493")]
    N122493,
    /// Regional Contribution to Ejection Fraction
    #[serde(rename = "122495")]
    N122495,
    /// Radial Shortening
    #[serde(rename = "122496")]
    N122496,
    /// Landmark Based Wall Motion Analysis
    #[serde(rename = "122497")]
    N122497,
    /// Slice Contribution to Ejection Fraction
    #[serde(rename = "122498")]
    N122498,
    /// Frame to Frame Analysis
    #[serde(rename = "122499")]
    N122499,
    /// Area of closed irregular polygon
    #[serde(rename = "122501")]
    N122501,
    /// Area of a closed NURBS
    #[serde(rename = "122502")]
    N122502,
    /// Integration of sum of closed areas on contiguous slices
    #[serde(rename = "122503")]
    N122503,
    /// Calibration
    #[serde(rename = "122505")]
    N122505,
    /// Left Contour
    #[serde(rename = "122507")]
    N122507,
    /// Right Contour
    #[serde(rename = "122508")]
    N122508,
    /// Diameter Graph
    #[serde(rename = "122509")]
    N122509,
    /// Length Luminal Segment
    #[serde(rename = "122510")]
    N122510,
    /// Graph Increment
    #[serde(rename = "122511")]
    N122511,
    /// Site of Maximum Luminal
    #[serde(rename = "122516")]
    N122516,
    /// Densitometric Luminal Cross-sectional Area Graph
    #[serde(rename = "122517")]
    N122517,
    /// Position of Proximal Border
    #[serde(rename = "122528")]
    N122528,
    /// Position of Distal Border
    #[serde(rename = "122529")]
    N122529,
    /// Plaque Area
    #[serde(rename = "122542")]
    N122542,
    /// Diameter Symmetry
    #[serde(rename = "122544")]
    N122544,
    /// Area Symmetry
    #[serde(rename = "122545")]
    N122545,
    /// Inflow Angle
    #[serde(rename = "122546")]
    N122546,
    /// Outflow Angle
    #[serde(rename = "122547")]
    N122547,
    /// Stenotic Flow Reserve
    #[serde(rename = "122548")]
    N122548,
    /// Poiseuille Resistance
    #[serde(rename = "122549")]
    N122549,
    /// Turbulence Resistance
    #[serde(rename = "122550")]
    N122550,
    /// Pressure Drop at SFR
    #[serde(rename = "122551")]
    N122551,
    /// Segmentation Method
    #[serde(rename = "122554")]
    N122554,
    /// Estimated Normal Flow
    #[serde(rename = "122555")]
    N122555,
    /// Area Length Kennedy
    #[serde(rename = "122558")]
    N122558,
    /// Area Length Dodge
    #[serde(rename = "122559")]
    N122559,
    /// Area Length Wynne
    #[serde(rename = "122560")]
    N122560,
    /// Multiple Slices
    #[serde(rename = "122562")]
    N122562,
    /// Boak
    #[serde(rename = "122563")]
    N122563,
    /// TS Pyramid
    #[serde(rename = "122564")]
    N122564,
    /// Two Chamber
    #[serde(rename = "122565")]
    N122565,
    /// Parallelepiped
    #[serde(rename = "122566")]
    N122566,
    /// BSA^1.219
    #[serde(rename = "122572")]
    N122572,
    /// Equidistant method
    #[serde(rename = "122574")]
    N122574,
    /// User selected method
    #[serde(rename = "122575")]
    N122575,
    /// Left ventricular posterobasal segment
    #[serde(rename = "122582")]
    N122582,
    /// Cardiovascular Analysis Report
    #[serde(rename = "122600")]
    N122600,
    /// Ventricular Analysis
    #[serde(rename = "122601")]
    N122601,
    /// Myocardial Perfusion Analysis
    #[serde(rename = "122602")]
    N122602,
    /// Calcium Scoring Analysis
    #[serde(rename = "122603")]
    N122603,
    /// Flow Quantification
    #[serde(rename = "122604")]
    N122604,
    /// Vascular Morphological Analysis
    #[serde(rename = "122605")]
    N122605,
    /// Vascular Functional Analysis
    #[serde(rename = "122606")]
    N122606,
    /// Thickening Analysis
    #[serde(rename = "122607")]
    N122607,
    /// Absolute Values Of Ventricular Measurements
    #[serde(rename = "122608")]
    N122608,
    /// Normalized Values Of Ventricular Measurements
    #[serde(rename = "122609")]
    N122609,
    /// Reference Point
    #[serde(rename = "122611")]
    N122611,
    /// Central breathing position
    #[serde(rename = "122612")]
    N122612,
    /// Peak Ejection Rate
    #[serde(rename = "122616")]
    N122616,
    /// Peak Ejection Time
    #[serde(rename = "122617")]
    N122617,
    /// Peak Filling Rate
    #[serde(rename = "122618")]
    N122618,
    /// Peak Filling Time
    #[serde(rename = "122619")]
    N122619,
    /// Papillary Muscle Excluded
    #[serde(rename = "122620")]
    N122620,
    /// Papillary Muscle Included
    #[serde(rename = "122621")]
    N122621,
    /// Wall Thickness Ratio end-systolic to end-diastolic
    #[serde(rename = "122624")]
    N122624,
    /// Curve Fit Method
    #[serde(rename = "122627")]
    N122627,
    /// Baseline Result Correction
    #[serde(rename = "122628")]
    N122628,
    /// Signal Earliest Peak Time
    #[serde(rename = "122631")]
    N122631,
    /// Signal Increase Start Time
    #[serde(rename = "122633")]
    N122633,
    /// Signal Time to Peak
    #[serde(rename = "122634")]
    N122634,
    /// MR Perfusion Peak
    #[serde(rename = "122635")]
    N122635,
    /// MR Perfusion Slope
    #[serde(rename = "122636")]
    N122636,
    /// MR Perfusion Time Integral
    #[serde(rename = "122637")]
    N122637,
    /// Signal Baseline Start
    #[serde(rename = "122638")]
    N122638,
    /// Signal Baseline End
    #[serde(rename = "122639")]
    N122639,
    /// Image Interval
    #[serde(rename = "122640")]
    N122640,
    /// Velocity Encoding Minimum Value
    #[serde(rename = "122642")]
    N122642,
    /// Velocity Encoding Maximum Value
    #[serde(rename = "122643")]
    N122643,
    /// Net Forward Volume
    #[serde(rename = "122645")]
    N122645,
    /// Area Based Method
    #[serde(rename = "122650")]
    N122650,
    /// Diameter Based Method
    #[serde(rename = "122651")]
    N122651,
    /// Volume Based Method
    #[serde(rename = "122652")]
    N122652,
    /// NASCET
    #[serde(rename = "122655")]
    N122655,
    /// ECST
    #[serde(rename = "122656")]
    N122656,
    /// Agatston Score Threshold
    #[serde(rename = "122657")]
    N122657,
    /// Calcium Mass Threshold
    #[serde(rename = "122658")]
    N122658,
    /// Calcium Scoring Calibration
    #[serde(rename = "122659")]
    N122659,
    /// Calcium Volume
    #[serde(rename = "122660")]
    N122660,
    /// Calcium Mass
    #[serde(rename = "122661")]
    N122661,
    /// Late Contrast Enhancement
    #[serde(rename = "122664")]
    N122664,
    /// Time interval since injection of contrast media
    #[serde(rename = "122665")]
    N122665,
    /// Time relative to R-wave peak
    #[serde(rename = "122666")]
    N122666,
    /// Blood velocity vs. time of cardiac cycle
    #[serde(rename = "122667")]
    N122667,
    /// Time interval since detection of contrast bolus
    #[serde(rename = "122668")]
    N122668,
    /// Papillary Muscle Included/Excluded
    #[serde(rename = "122670")]
    N122670,
    /// Anterior-Posterior
    #[serde(rename = "122675")]
    N122675,
    /// endoleak
    #[serde(rename = "122680")]
    N122680,
    /// Stent Fracture
    #[serde(rename = "122683")]
    N122683,
    /// Stent Disintegration
    #[serde(rename = "122684")]
    N122684,
    /// Stent Composition
    #[serde(rename = "122685")]
    N122685,
    /// Parent Vessel Finding
    #[serde(rename = "122686")]
    N122686,
    /// Site of Lumen Maximum
    #[serde(rename = "122687")]
    N122687,
    /// X-Concept
    #[serde(rename = "122698")]
    N122698,
    /// Y-Concept
    #[serde(rename = "122699")]
    N122699,
    /// Indications for Pharmacological Stress
    #[serde(rename = "122700")]
    N122700,
    /// Procedure time base
    #[serde(rename = "122701")]
    N122701,
    /// Treadmill speed
    #[serde(rename = "122702")]
    N122702,
    /// Treadmill gradient
    #[serde(rename = "122703")]
    N122703,
    /// Ergometer power
    #[serde(rename = "122704")]
    N122704,
    /// Pharmacological Stress Agent Dose Rate
    #[serde(rename = "122705")]
    N122705,
    /// Rating of Perceived Exertion
    #[serde(rename = "122706")]
    N122706,
    /// Number of Ectopic Beats
    #[serde(rename = "122707")]
    N122707,
    /// Double Product
    #[serde(rename = "122708")]
    N122708,
    /// Activity workload
    #[serde(rename = "122709")]
    N122709,
    /// Time since start of stage
    #[serde(rename = "122710")]
    N122710,
    /// Exercise duration after stress agent injection
    #[serde(rename = "122711")]
    N122711,
    /// Imaging Start Time
    #[serde(rename = "122712")]
    N122712,
    /// Attenuation correction method
    #[serde(rename = "122713")]
    N122713,
    /// Pharmacological Stress Agent Dose
    #[serde(rename = "122715")]
    N122715,
    /// Maximum Power Output Achieved
    #[serde(rename = "122716")]
    N122716,
    /// Peak activity workload
    #[serde(rename = "122717")]
    N122717,
    /// Peak Double Product
    #[serde(rename = "122718")]
    N122718,
    /// OSEM algorithm
    #[serde(rename = "122720")]
    N122720,
    /// Chang method
    #[serde(rename = "122721")]
    N122721,
    /// Algorithmic attenuation correction
    #[serde(rename = "122726")]
    N122726,
    /// NM transmission attenuation correction
    #[serde(rename = "122727")]
    N122727,
    /// CT-based attenuation correction
    #[serde(rename = "122728")]
    N122728,
    /// No Attenuation Correction
    #[serde(rename = "122729")]
    N122729,
    /// Bazett QTc Algorithm
    #[serde(rename = "122730")]
    N122730,
    /// Hodges QTc Algorithm
    #[serde(rename = "122731")]
    N122731,
    /// Fridericia QTc Algorithm
    #[serde(rename = "122732")]
    N122732,
    /// Framingham QTc Algorithm
    #[serde(rename = "122733")]
    N122733,
    /// Borg RPE Scale
    #[serde(rename = "122734")]
    N122734,
    /// Borg CR10 Scale
    #[serde(rename = "122735")]
    N122735,
    /// Overall study quality
    #[serde(rename = "122739")]
    N122739,
    /// Excellent image quality
    #[serde(rename = "122740")]
    N122740,
    /// Good image quality
    #[serde(rename = "122741")]
    N122741,
    /// Poor image quality
    #[serde(rename = "122742")]
    N122742,
    /// Body habitus attenuation
    #[serde(rename = "122743")]
    N122743,
    /// Breast attenuation
    #[serde(rename = "122744")]
    N122744,
    /// Diaphragmatic attenuation
    #[serde(rename = "122745")]
    N122745,
    /// False positive defect finding
    #[serde(rename = "122748")]
    N122748,
    /// Non-diagnostic - low heart rate
    #[serde(rename = "122750")]
    N122750,
    /// Non-diagnostic - resting ST abnormalities
    #[serde(rename = "122751")]
    N122751,
    /// Non-diagnostic - ventricular pacing or LBBB
    #[serde(rename = "122752")]
    N122752,
    /// Non-diagnostic ECG
    #[serde(rename = "122753")]
    N122753,
    /// Strongly positive
    #[serde(rename = "122755")]
    N122755,
    /// Strongly positive - ST elevation
    #[serde(rename = "122756")]
    N122756,
    /// ST Depression - Horizontal
    #[serde(rename = "122757")]
    N122757,
    /// ST Depression - Upsloping
    #[serde(rename = "122758")]
    N122758,
    /// ST Depression - Downsloping
    #[serde(rename = "122759")]
    N122759,
    /// Stress test score
    #[serde(rename = "122760")]
    N122760,
    /// Number of diseased vessel territories
    #[serde(rename = "122762")]
    N122762,
    /// Weight exceeds equipment limit
    #[serde(rename = "122764")]
    N122764,
    /// Difference in Ejection Fraction
    #[serde(rename = "122768")]
    N122768,
    /// Difference in ED LV Volume
    #[serde(rename = "122769")]
    N122769,
    /// Ratio of achieved to predicted maximal oxygen consumption
    #[serde(rename = "122770")]
    N122770,
    /// Ratio of achieved to predicted functional capacity
    #[serde(rename = "122771")]
    N122771,
    /// Aerobic index
    #[serde(rename = "122772")]
    N122772,
    /// ST/HR Index
    #[serde(rename = "122773")]
    N122773,
    /// Agreement with prior findings
    #[serde(rename = "122775")]
    N122775,
    /// Disagreement with prior findings
    #[serde(rename = "122776")]
    N122776,
    /// Rest thallium/stress technetium procedure
    #[serde(rename = "122781")]
    N122781,
    /// Rest technetium/stress technetium 1 day procedure
    #[serde(rename = "122782")]
    N122782,
    /// Rest technetium/stress technetium 2 day procedure
    #[serde(rename = "122783")]
    N122783,
    /// Stress technetium/rest technetium 1 day procedure
    #[serde(rename = "122784")]
    N122784,
    /// NM Myocardial Viability procedure
    #[serde(rename = "122785")]
    N122785,
    /// PET Myocardial Perfusion, Rest only
    #[serde(rename = "122791")]
    N122791,
    /// PET Myocardial Perfusion, Stress only
    #[serde(rename = "122792")]
    N122792,
    /// PET Myocardial Perfusion, Rest and Stress
    #[serde(rename = "122793")]
    N122793,
    /// PET Myocardial Viability, Rest only
    #[serde(rename = "122795")]
    N122795,
    /// PET Myocardial Viability, Stress only
    #[serde(rename = "122796")]
    N122796,
    /// PET Myocardial Viability, Rest and Stress
    #[serde(rename = "122797")]
    N122797,
    /// Anginal Equivalent
    #[serde(rename = "122799")]
    N122799,
    /// Radiopharmaceutical
    #[serde(rename = "123001")]
    N123001,
    /// Radiopharmaceutical Start Time
    #[serde(rename = "123003")]
    N123003,
    /// Radiopharmaceutical Stop Time
    #[serde(rename = "123004")]
    N123004,
    /// Radiopharmaceutical Volume
    #[serde(rename = "123005")]
    N123005,
    /// Radionuclide Total Dose
    #[serde(rename = "123006")]
    N123006,
    /// Radiopharmaceutical Specific Activity
    #[serde(rename = "123007")]
    N123007,
    /// Radionuclide Syringe Counts
    #[serde(rename = "123009")]
    N123009,
    /// Radionuclide Residual Syringe Counts
    #[serde(rename = "123010")]
    N123010,
    /// Contrast/Bolus Agent
    #[serde(rename = "123011")]
    N123011,
    /// Pre-Medication
    #[serde(rename = "123012")]
    N123012,
    /// Target Region
    #[serde(rename = "123014")]
    N123014,
    /// Imaging Direction
    #[serde(rename = "123015")]
    N123015,
    /// Imaging Conditions
    #[serde(rename = "123016")]
    N123016,
    /// Caudal 10 degree distal-cranioproximal oblique
    #[serde(rename = "123019")]
    N123019,
    /// Neighborhood Analysis
    #[serde(rename = "123101")]
    N123101,
    /// Adaptive Filtering
    #[serde(rename = "123102")]
    N123102,
    /// Edge Detection
    #[serde(rename = "123103")]
    N123103,
    /// Morphological Operations
    #[serde(rename = "123104")]
    N123104,
    /// Histogram Analysis
    #[serde(rename = "123105")]
    N123105,
    /// Multi-Scale/Resolution Filtering
    #[serde(rename = "123106")]
    N123106,
    /// Cluster Analysis
    #[serde(rename = "123107")]
    N123107,
    /// Multispectral Processing
    #[serde(rename = "123108")]
    N123108,
    /// Manual Processing
    #[serde(rename = "123109")]
    N123109,
    /// Artificial Intelligence
    #[serde(rename = "123110")]
    N123110,
    /// Deformable Models
    #[serde(rename = "123111")]
    N123111,
    /// OB-GYN Ultrasound Procedure Report
    #[serde(rename = "125000")]
    N125000,
    /// Fetal Biometry Ratios
    #[serde(rename = "125001")]
    N125001,
    /// Fetal Biometry
    #[serde(rename = "125002")]
    N125002,
    /// Fetal Long Bones
    #[serde(rename = "125003")]
    N125003,
    /// Fetal Cranium
    #[serde(rename = "125004")]
    N125004,
    /// Biometry Group
    #[serde(rename = "125005")]
    N125005,
    /// Biophysical Profile
    #[serde(rename = "125006")]
    N125006,
    /// Measurement Group
    #[serde(rename = "125007")]
    N125007,
    /// Fetus Summary
    #[serde(rename = "125008")]
    N125008,
    /// Early Gestation
    #[serde(rename = "125009")]
    N125009,
    /// Identifier
    #[serde(rename = "125010")]
    N125010,
    /// Pelvis and Uterus
    #[serde(rename = "125011")]
    N125011,
    /// Growth Percentile rank
    #[serde(rename = "125012")]
    N125012,
    /// Growth Z-score
    #[serde(rename = "125013")]
    N125013,
    /// Fetus Characteristics
    #[serde(rename = "125015")]
    N125015,
    /// Fetal Measurements
    #[serde(rename = "125016")]
    N125016,
    /// Frame of Reference Identity
    #[serde(rename = "125021")]
    N125021,
    /// Fiducial Alignment
    #[serde(rename = "125022")]
    N125022,
    /// Acquisition Equipment Alignment
    #[serde(rename = "125023")]
    N125023,
    /// Image Content-based Alignment
    #[serde(rename = "125024")]
    N125024,
    /// Visual Alignment
    #[serde(rename = "125025")]
    N125025,
    /// Inter-Hemispheric Plane
    #[serde(rename = "125030")]
    N125030,
    /// Right Hemisphere Most Anterior
    #[serde(rename = "125031")]
    N125031,
    /// Right Hemisphere Most Posterior
    #[serde(rename = "125032")]
    N125032,
    /// Right Hemisphere Most Superior
    #[serde(rename = "125033")]
    N125033,
    /// Right Hemisphere Most Inferior
    #[serde(rename = "125034")]
    N125034,
    /// Left Hemisphere Most Anterior
    #[serde(rename = "125035")]
    N125035,
    /// Left Hemisphere Most Posterior
    #[serde(rename = "125036")]
    N125036,
    /// Left Hemisphere Most Superior
    #[serde(rename = "125037")]
    N125037,
    /// Left Hemisphere Most Inferior
    #[serde(rename = "125038")]
    N125038,
    /// Background
    #[serde(rename = "125040")]
    N125040,
    /// Registration Input
    #[serde(rename = "125041")]
    N125041,
    /// Vascular Ultrasound Procedure Report
    #[serde(rename = "125100")]
    N125100,
    /// Vessel Branch
    #[serde(rename = "125101")]
    N125101,
    /// Graft Type
    #[serde(rename = "125102")]
    N125102,
    /// Measurement Orientation
    #[serde(rename = "125105")]
    N125105,
    /// Doppler Angle
    #[serde(rename = "125106")]
    N125106,
    /// Sample Volume Depth
    #[serde(rename = "125107")]
    N125107,
    /// Pediatric Cardiac Ultrasound Report
    #[serde(rename = "125195")]
    N125195,
    /// Fetal Cardiac Ultrasound Report
    #[serde(rename = "125196")]
    N125196,
    /// Adult Congenital Cardiac Ultrasound Report
    #[serde(rename = "125197")]
    N125197,
    /// Adult Echocardiography Procedure Report
    #[serde(rename = "125200")]
    N125200,
    /// Illustration of Finding
    #[serde(rename = "125201")]
    N125201,
    /// LV Wall Motion Score Index
    #[serde(rename = "125202")]
    N125202,
    /// Acquisition Protocol
    #[serde(rename = "125203")]
    N125203,
    /// Area-length biplane
    #[serde(rename = "125204")]
    N125204,
    /// Area-Length Single Plane
    #[serde(rename = "125205")]
    N125205,
    /// Cube
    #[serde(rename = "125206")]
    N125206,
    /// Method of Disks, Biplane
    #[serde(rename = "125207")]
    N125207,
    /// Method of Disks, Single Plane
    #[serde(rename = "125208")]
    N125208,
    /// Teichholz
    #[serde(rename = "125209")]
    N125209,
    /// Area by Pressure Half-Time
    #[serde(rename = "125210")]
    N125210,
    /// Biplane Ellipse
    #[serde(rename = "125211")]
    N125211,
    /// Continuity Equation
    #[serde(rename = "125212")]
    N125212,
    /// Continuity Equation by Mean Velocity
    #[serde(rename = "125213")]
    N125213,
    /// Continuity Equation by Peak Velocity
    #[serde(rename = "125214")]
    N125214,
    /// Continuity Equation by Velocity Time Integral
    #[serde(rename = "125215")]
    N125215,
    /// Proximal Isovelocity Surface Area
    #[serde(rename = "125216")]
    N125216,
    /// Full Bernoulli
    #[serde(rename = "125217")]
    N125217,
    /// Simplified Bernoulli
    #[serde(rename = "125218")]
    N125218,
    /// Doppler Volume Flow
    #[serde(rename = "125219")]
    N125219,
    /// Planimetry
    #[serde(rename = "125220")]
    N125220,
    /// Left Ventricle Mass by M-mode
    #[serde(rename = "125221")]
    N125221,
    /// Left Ventricle Mass by Truncated Ellipse
    #[serde(rename = "125222")]
    N125222,
    /// 4 Point Segment Finding Scale
    #[serde(rename = "125223")]
    N125223,
    /// 5 Point Segment Finding Scale
    #[serde(rename = "125224")]
    N125224,
    /// 5 Point Segment Finding Scale With Graded Hypokinesis
    #[serde(rename = "125225")]
    N125225,
    /// Single Plane Ellipse
    #[serde(rename = "125226")]
    N125226,
    /// Modified Simpson
    #[serde(rename = "125227")]
    N125227,
    /// Bullet Method
    #[serde(rename = "125228")]
    N125228,
    /// Power Doppler
    #[serde(rename = "125230")]
    N125230,
    /// 3D mode
    #[serde(rename = "125231")]
    N125231,
    /// Start of drug dose administration
    #[serde(rename = "125233")]
    N125233,
    /// Start of contrast agent administration
    #[serde(rename = "125234")]
    N125234,
    /// Destruction of microbubbles
    #[serde(rename = "125235")]
    N125235,
    /// Onset of exercise
    #[serde(rename = "125236")]
    N125236,
    /// Cessation of exercise
    #[serde(rename = "125237")]
    N125237,
    /// Onset of stimulation
    #[serde(rename = "125238")]
    N125238,
    /// Cessation of stimulation
    #[serde(rename = "125239")]
    N125239,
    /// Line scan pattern
    #[serde(rename = "125240")]
    N125240,
    /// Plane scan pattern
    #[serde(rename = "125241")]
    N125241,
    /// Volume scan pattern
    #[serde(rename = "125242")]
    N125242,
    /// Non-imaging Doppler ultrasound transducer geometry
    #[serde(rename = "125251")]
    N125251,
    /// Linear ultrasound transducer geometry
    #[serde(rename = "125252")]
    N125252,
    /// Curved linear ultrasound transducer geometry
    #[serde(rename = "125253")]
    N125253,
    /// Sector ultrasound transducer geometry
    #[serde(rename = "125254")]
    N125254,
    /// Radial ultrasound transducer geometry
    #[serde(rename = "125255")]
    N125255,
    /// Ring ultrasound transducer geometry
    #[serde(rename = "125256")]
    N125256,
    /// Fixed beam direction
    #[serde(rename = "125257")]
    N125257,
    /// Mechanical beam steering
    #[serde(rename = "125258")]
    N125258,
    /// Phased beam steering
    #[serde(rename = "125259")]
    N125259,
    /// External Transducer
    #[serde(rename = "125261")]
    N125261,
    /// Transesophageal Transducer
    #[serde(rename = "125262")]
    N125262,
    /// Endovaginal Transducer
    #[serde(rename = "125263")]
    N125263,
    /// Endorectal Transducer
    #[serde(rename = "125264")]
    N125264,
    /// Intravascular Transducer
    #[serde(rename = "125265")]
    N125265,
    /// Left Ventricle Mass by Area Length
    #[serde(rename = "125270")]
    N125270,
    /// Left Ventricle Mass by M-mode - adjusted by Height
    #[serde(rename = "125271")]
    N125271,
    /// Left Ventricle Mass by Truncated Ellipse - adjusted by Height
    #[serde(rename = "125272")]
    N125272,
    /// Left Ventricle Mass by Area Length - adjusted by Height
    #[serde(rename = "125273")]
    N125273,
    /// CARDIOsphere
    #[serde(rename = "125901")]
    N125901,
    /// Echovist
    #[serde(rename = "125902")]
    N125902,
    /// Imagify
    #[serde(rename = "125903")]
    N125903,
    /// Levovist
    #[serde(rename = "125904")]
    N125904,
    /// Sonazoid
    #[serde(rename = "125905")]
    N125905,
    /// SonoVue
    #[serde(rename = "125906")]
    N125906,
    /// Targestar-B
    #[serde(rename = "125907")]
    N125907,
    /// Targestar-P
    #[serde(rename = "125908")]
    N125908,
    /// Imaging Measurement Report
    #[serde(rename = "126000")]
    N126000,
    /// Oncology Measurement Report
    #[serde(rename = "126001")]
    N126001,
    /// Dynamic Contrast MR Measurement Report
    #[serde(rename = "126002")]
    N126002,
    /// PET Measurement Report
    #[serde(rename = "126003")]
    N126003,
    /// Imaging Measurements
    #[serde(rename = "126010")]
    N126010,
    /// Derived Imaging Measurements
    #[serde(rename = "126011")]
    N126011,
    /// Multiparametric MRI
    #[serde(rename = "126020")]
    N126020,
    /// Multiparametric MRI of prostate
    #[serde(rename = "126021")]
    N126021,
    /// Multiparametric MRI of whole body
    #[serde(rename = "126022")]
    N126022,
    /// Sum of segmented voxel volumes
    #[serde(rename = "126030")]
    N126030,
    /// Peak Value Within ROI
    #[serde(rename = "126031")]
    N126031,
    /// Metabolic Volume
    #[serde(rename = "126032")]
    N126032,
    /// Total Lesion Glycolysis
    #[serde(rename = "126033")]
    N126033,
    /// Glycolysis
    #[serde(rename = "126034")]
    N126034,
    /// Total Lesion Proliferation
    #[serde(rename = "126035")]
    N126035,
    /// Proliferative Activity
    #[serde(rename = "126036")]
    N126036,
    /// Standardized Added Metabolic Activity (SAM)
    #[serde(rename = "126037")]
    N126037,
    /// Standardized Added Metabolic Activity (SAM) Background
    #[serde(rename = "126038")]
    N126038,
    /// Lesion to Background SUV Ratio
    #[serde(rename = "126039")]
    N126039,
    /// Background for Lesion to Background SUV Ratio
    #[serde(rename = "126040")]
    N126040,
    /// Fractal Dimension
    #[serde(rename = "126050")]
    N126050,
    /// Skewness
    #[serde(rename = "126051")]
    N126051,
    /// Kurtosis
    #[serde(rename = "126052")]
    N126052,
    /// Entropy of GLCM
    #[serde(rename = "126060")]
    N126060,
    /// Energy of GLCM
    #[serde(rename = "126061")]
    N126061,
    /// Homogeneity of GLCM
    #[serde(rename = "126062")]
    N126062,
    /// Contrast of GLCM
    #[serde(rename = "126063")]
    N126063,
    /// Dissimilarity of GLCM
    #[serde(rename = "126064")]
    N126064,
    /// ASM of GLCM
    #[serde(rename = "126065")]
    N126065,
    /// Correlation of GLCM
    #[serde(rename = "126066")]
    N126066,
    /// Gray Level Co-occurrence Matrix (GLCM)
    #[serde(rename = "126067")]
    N126067,
    /// Subject Time Point Identifier
    #[serde(rename = "126070")]
    N126070,
    /// Protocol Time Point Identifier
    #[serde(rename = "126071")]
    N126071,
    /// Time Point Type
    #[serde(rename = "126072")]
    N126072,
    /// Time Point Order
    #[serde(rename = "126073")]
    N126073,
    /// Posttreatment
    #[serde(rename = "126074")]
    N126074,
    /// Eligibility
    #[serde(rename = "126075")]
    N126075,
    /// RECIST 1.0
    #[serde(rename = "126080")]
    N126080,
    /// RECIST 1.1
    #[serde(rename = "126081")]
    N126081,
    /// Real World Value Map used for measurement
    #[serde(rename = "126100")]
    N126100,
    /// Image Library Group
    #[serde(rename = "126200")]
    N126200,
    /// Acquisition Date
    #[serde(rename = "126201")]
    N126201,
    /// Acquisition Time
    #[serde(rename = "126202")]
    N126202,
    /// PET Radionuclide Incubation Time
    #[serde(rename = "126203")]
    N126203,
    /// R2-Coefficient
    #[serde(rename = "126220")]
    N126220,
    /// Perfusion analysis by Stable Xenon CT technique
    #[serde(rename = "126300")]
    N126300,
    /// Perfusion analysis by IV Iodinated Contrast CT technique
    #[serde(rename = "126301")]
    N126301,
    /// Perfusion analysis by Arterial Spin Labeling MR technique
    #[serde(rename = "126302")]
    N126302,
    /// Perfusion analysis by Susceptibility MR technique
    #[serde(rename = "126303")]
    N126303,
    /// Least Mean Square (LMS) deconvolution
    #[serde(rename = "126310")]
    N126310,
    /// Singular Value Decomposition (SVD) deconvolution
    #[serde(rename = "126311")]
    N126311,
    /// Ktrans
    #[serde(rename = "126312")]
    N126312,
    /// kep
    #[serde(rename = "126313")]
    N126313,
    /// ve
    #[serde(rename = "126314")]
    N126314,
    /// IAUC
    #[serde(rename = "126320")]
    N126320,
    /// IAUC60
    #[serde(rename = "126321")]
    N126321,
    /// IAUC90
    #[serde(rename = "126322")]
    N126322,
    /// tau_m
    #[serde(rename = "126330")]
    N126330,
    /// vp
    #[serde(rename = "126331")]
    N126331,
    /// Standard Tofts Model
    #[serde(rename = "126340")]
    N126340,
    /// Extended Tofts Model
    #[serde(rename = "126341")]
    N126341,
    /// Model-free concentration-time quantitification
    #[serde(rename = "126342")]
    N126342,
    /// First Pass Leakage Profile (FPLP)
    #[serde(rename = "126343")]
    N126343,
    /// Shutter-Speed Model (SSM)
    #[serde(rename = "126344")]
    N126344,
    /// T1 by Multiple Flip Angles
    #[serde(rename = "126350")]
    N126350,
    /// T1 by Inversion Recovery
    #[serde(rename = "126351")]
    N126351,
    /// T1 by Fixed Value
    #[serde(rename = "126352")]
    N126352,
    /// T1 Used For Calculation
    #[serde(rename = "126353")]
    N126353,
    /// AIF Ignored
    #[serde(rename = "126360")]
    N126360,
    /// Population Averaged AIF
    #[serde(rename = "126361")]
    N126361,
    /// User-defined AIF ROI
    #[serde(rename = "126362")]
    N126362,
    /// Automatically Detected AIF ROI
    #[serde(rename = "126363")]
    N126363,
    /// Blind Estimation of AIF
    #[serde(rename = "126364")]
    N126364,
    /// Time of Peak Concentration
    #[serde(rename = "126370")]
    N126370,
    /// Bolus Arrival Time
    #[serde(rename = "126371")]
    N126371,
    /// Time of Leading Half-Peak Concentration
    #[serde(rename = "126372")]
    N126372,
    /// Temporal Derivative Exceeds Threshold
    #[serde(rename = "126373")]
    N126373,
    /// Temporal Derivative Threshold
    #[serde(rename = "126374")]
    N126374,
    /// Maximum Slope
    #[serde(rename = "126375")]
    N126375,
    /// Maximum Difference
    #[serde(rename = "126376")]
    N126376,
    /// Tracer Concentration
    #[serde(rename = "126377")]
    N126377,
    /// Contrast Longitudinal Relaxivity
    #[serde(rename = "126380")]
    N126380,
    /// Regional Blood Flow
    #[serde(rename = "126390")]
    N126390,
    /// Regional Blood Volume
    #[serde(rename = "126391")]
    N126391,
    /// Oxygen Extraction Fraction
    #[serde(rename = "126392")]
    N126392,
    /// R1
    #[serde(rename = "126393")]
    N126393,
    /// R2
    #[serde(rename = "126394")]
    N126394,
    /// Standardized Uptake Value
    #[serde(rename = "126400")]
    N126400,
    /// SUVbw
    #[serde(rename = "126401")]
    N126401,
    /// SUVlbm
    #[serde(rename = "126402")]
    N126402,
    /// SUVbsa
    #[serde(rename = "126403")]
    N126403,
    /// SUVibw
    #[serde(rename = "126404")]
    N126404,
    /// SUV body weight calculation method
    #[serde(rename = "126410")]
    N126410,
    /// SUV lean body mass calculation method
    #[serde(rename = "126411")]
    N126411,
    /// SUV body surface area calculation method
    #[serde(rename = "126412")]
    N126412,
    /// SUV ideal body weight calculation method
    #[serde(rename = "126413")]
    N126413,
    /// Pittsburgh compound B C^11^
    #[serde(rename = "126500")]
    N126500,
    /// Florbetaben F^18^
    #[serde(rename = "126501")]
    N126501,
    /// T807 F^18^
    #[serde(rename = "126502")]
    N126502,
    /// Flubatine F^18^
    #[serde(rename = "126503")]
    N126503,
    /// Monoclonal Antibody (mAb) ^64^Cu
    #[serde(rename = "126510")]
    N126510,
    /// Monoclonal Antibody (mAb) ^89^Zr
    #[serde(rename = "126511")]
    N126511,
    /// Trastuzumab ^89^Zr
    #[serde(rename = "126512")]
    N126512,
    /// Cetuximab ^89^Zr
    #[serde(rename = "126513")]
    N126513,
    /// J591 ^89^Zr
    #[serde(rename = "126514")]
    N126514,
    /// cU36 ^89^Zr
    #[serde(rename = "126515")]
    N126515,
    /// Bevacizumab ^89^Zr
    #[serde(rename = "126516")]
    N126516,
    /// cG250-F(ab')(2) ^89^Zr
    #[serde(rename = "126517")]
    N126517,
    /// R1507 ^89^Zr
    #[serde(rename = "126518")]
    N126518,
    /// E4G10 ^89^Zr
    #[serde(rename = "126519")]
    N126519,
    /// Df-CD45 ^89^Zr
    #[serde(rename = "126520")]
    N126520,
    /// ^44^Scandium
    #[serde(rename = "126600")]
    N126600,
    /// ^51^Manganese
    #[serde(rename = "126601")]
    N126601,
    /// ^70^Arsenic
    #[serde(rename = "126602")]
    N126602,
    /// ^90^Niobium
    #[serde(rename = "126603")]
    N126603,
    /// ^191m^Iridium
    #[serde(rename = "126604")]
    N126604,
    /// ^43^Scandium
    #[serde(rename = "126605")]
    N126605,
    /// ^152^Terbium
    #[serde(rename = "126606")]
    N126606,
    /// ATSM Cu^60^
    #[serde(rename = "126700")]
    N126700,
    /// ATSM Cu^61^
    #[serde(rename = "126701")]
    N126701,
    /// ATSM Cu^62^
    #[serde(rename = "126702")]
    N126702,
    /// Choline C^11^
    #[serde(rename = "126703")]
    N126703,
    /// Fallypride C^11^
    #[serde(rename = "126704")]
    N126704,
    /// Fallypride F^18^
    #[serde(rename = "126705")]
    N126705,
    /// FLB 457 C^11^
    #[serde(rename = "126706")]
    N126706,
    /// Fluorotriopride F^18^
    #[serde(rename = "126707")]
    N126707,
    /// Fluoromisonidazole (FMISO) F^18^
    #[serde(rename = "126708")]
    N126708,
    /// Glutamine C^11^
    #[serde(rename = "126709")]
    N126709,
    /// Glutamine C^14^
    #[serde(rename = "126710")]
    N126710,
    /// Glutamine F^18^
    #[serde(rename = "126711")]
    N126711,
    /// Flubatine F^18^
    #[serde(rename = "126712")]
    N126712,
    /// 2FA F^18^
    #[serde(rename = "126713")]
    N126713,
    /// Nifene F^18^
    #[serde(rename = "126714")]
    N126714,
    /// CLR1404 I^124^
    #[serde(rename = "126715")]
    N126715,
    /// CLR1404 I^131^
    #[serde(rename = "126716")]
    N126716,
    /// IEC6127 Patient Support Continuous Angle
    #[serde(rename = "126801")]
    N126801,
    /// IEC6127 Table Top Continuous Pitch Angle
    #[serde(rename = "126802")]
    N126802,
    /// IEC6127 Table Top Continuous Roll Angle
    #[serde(rename = "126803")]
    N126803,
    /// IEC6127 Table Top Eccentric Axis Distance
    #[serde(rename = "126804")]
    N126804,
    /// IEC6127 Table Top Continuous Eccentric Angle
    #[serde(rename = "126805")]
    N126805,
    /// IEC6127 Table Top Lateral Position
    #[serde(rename = "126806")]
    N126806,
    /// IEC6127 Table Top Longitudinal Position
    #[serde(rename = "126807")]
    N126807,
    /// IEC6127 Table Top Vertical Position
    #[serde(rename = "126808")]
    N126808,
    /// IEC6127 Gantry Continuous Roll Angle
    #[serde(rename = "126809")]
    N126809,
    /// IEC6127 Gantry Continuous Pitch Angle
    #[serde(rename = "126810")]
    N126810,
    /// IEC6127 Gantry Continuous Yaw Angle
    #[serde(rename = "126811")]
    N126811,
}

/// This value set defines a set of codes that can be used to indicate dietary
/// preferences or restrictions a patient may have.
///
/// System: <http://hl7.org/fhir/diet>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Diet {
    /// Vegetarian
    #[default]
    #[serde(rename = "vegetarian")]
    Vegetarian,
    /// Dairy Free
    #[serde(rename = "dairy-free")]
    DairyFree,
    /// Nut Free
    #[serde(rename = "nut-free")]
    NutFree,
    /// Gluten Free
    #[serde(rename = "gluten-free")]
    GlutenFree,
    /// Vegan
    #[serde(rename = "vegan")]
    Vegan,
    /// Halal
    #[serde(rename = "halal")]
    Halal,
    /// Kosher
    #[serde(rename = "kosher")]
    Kosher,
}

/// Whether the Media is a photo, video, or audio
///
/// System: <http://hl7.org/fhir/digital-media-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DigitalMediaType {
    /// Photo
    #[default]
    #[serde(rename = "photo")]
    Photo,
    /// Video
    #[serde(rename = "video")]
    Video,
    /// Audio
    #[serde(rename = "audio")]
    Audio,
}

/// This value set defines a set of codes that can be used to where the patient
/// left the hospital.
///
/// System: <http://hl7.org/fhir/discharge-disposition>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DischargeDisposition {
    /// Home
    #[default]
    #[serde(rename = "home")]
    Home,
    /// Other healthcare facility
    #[serde(rename = "other-hcf")]
    OtherHcf,
    /// Hospice
    #[serde(rename = "hosp")]
    Hosp,
    /// Long-term care
    #[serde(rename = "long")]
    Long,
    /// Left against advice
    #[serde(rename = "aadvice")]
    Aadvice,
    /// Expired
    #[serde(rename = "exp")]
    Exp,
    /// Psychiatric hospital
    #[serde(rename = "psy")]
    Psy,
    /// Rehabilitation
    #[serde(rename = "rehab")]
    Rehab,
    /// Other
    #[serde(rename = "oth")]
    Oth,
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
}

/// Classification of the encounter
///
/// System: <http://hl7.org/fhir/encounter-class>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterClass {
    /// Inpatient
    #[default]
    #[serde(rename = "inpatient")]
    Inpatient,
    /// Outpatient
    #[serde(rename = "outpatient")]
    Outpatient,
    /// Ambulatory
    #[serde(rename = "ambulatory")]
    Ambulatory,
    /// Emergency
    #[serde(rename = "emergency")]
    Emergency,
    /// Home
    #[serde(rename = "home")]
    Home,
    /// Field
    #[serde(rename = "field")]
    Field,
    /// Daytime
    #[serde(rename = "daytime")]
    Daytime,
    /// Virtual
    #[serde(rename = "virtual")]
    Virtual,
    /// Other
    #[serde(rename = "other")]
    Other,
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

/// This is an example value set defined by the FHIR project, that could be
/// used in Emergency to indicate the encounter priority as determined by
/// triage.
///
/// System: <http://hl7.org/fhir/encounter-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterPriority {
    /// Immediate
    #[default]
    #[serde(rename = "imm")]
    Imm,
    /// Emergency
    #[serde(rename = "emg")]
    Emg,
    /// Urgent
    #[serde(rename = "urg")]
    Urg,
    /// Semi-urgent
    #[serde(rename = "s-urg")]
    SUrg,
    /// Non-urgent
    #[serde(rename = "no-urg")]
    NoUrg,
}

/// This value set defines a set of codes that can be used to indicate the
/// kinds of special arrangements in place for a patients visit.
///
/// System: <http://hl7.org/fhir/encounter-special-arrangements>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterSpecialArrangements {
    /// Wheelchair
    #[default]
    #[serde(rename = "wheel")]
    Wheel,
    /// Stretcher
    #[serde(rename = "stret")]
    Stret,
    /// Interpreter
    #[serde(rename = "int")]
    Int,
    /// Attendant
    #[serde(rename = "att")]
    Att,
    /// Guide dog
    #[serde(rename = "dog")]
    Dog,
}

/// Current state of the encounter
///
/// System: <http://hl7.org/fhir/encounter-state>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterState {
    /// Planned
    #[default]
    #[serde(rename = "planned")]
    Planned,
    /// Arrived
    #[serde(rename = "arrived")]
    Arrived,
    /// in Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Leave
    #[serde(rename = "onleave")]
    Onleave,
    /// Finished
    #[serde(rename = "finished")]
    Finished,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// This example value set defines a set of codes that can be used to indicate
/// the type of encounter: a specific code indicating type of service provided.
///
/// System: <http://hl7.org/fhir/encounter-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EncounterType {
    /// Annual diabetes mellitus screening
    #[default]
    #[serde(rename = "ADMS")]
    Adms,
    /// Bone drilling/bone marrow punction in clinic
    #[serde(rename = "BD/BM-clin")]
    BdBmClin,
    /// Infant colon screening - 60 minutes
    #[serde(rename = "CCS60")]
    Ccs60,
    /// Outpatient Kenacort injection
    #[serde(rename = "OKI")]
    Oki,
}

/// EnteralFormulaAdditiveType: Codes for the type of modular component such as
/// protein, carbohydrate or fiber to be provided in addition to or mixed with
/// the base formula. This value set is provided as a suggestive example.
///
/// System: <http://hl7.org/fhir/entformula-additive>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EntformulaAdditive {
    /// Modular lipid enteral formula component
    #[default]
    #[serde(rename = "lipid")]
    Lipid,
    /// Modular protein enteral formula component
    #[serde(rename = "protein")]
    Protein,
    /// Modular carbohydrate enteral formula component
    #[serde(rename = "carbohydrate")]
    Carbohydrate,
    /// Modular fiber enteral formula component
    #[serde(rename = "fiber")]
    Fiber,
    /// Added water
    #[serde(rename = "water")]
    Water,
}

/// The status of the encounter.
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
}

/// This value set includes the FDI Teeth codes.
///
/// System: <http://hl7.org/fhir/ex-fdi>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExFdi {
    /// 11
    #[default]
    #[serde(rename = "11")]
    N11,
    /// 12
    #[serde(rename = "12")]
    N12,
    /// 13
    #[serde(rename = "13")]
    N13,
    /// 14
    #[serde(rename = "14")]
    N14,
    /// 15
    #[serde(rename = "15")]
    N15,
    /// 16
    #[serde(rename = "16")]
    N16,
    /// 17
    #[serde(rename = "17")]
    N17,
    /// 18
    #[serde(rename = "18")]
    N18,
    /// 21
    #[serde(rename = "21")]
    N21,
    /// 22
    #[serde(rename = "22")]
    N22,
    /// 23
    #[serde(rename = "23")]
    N23,
    /// 24
    #[serde(rename = "24")]
    N24,
    /// 25
    #[serde(rename = "25")]
    N25,
    /// 26
    #[serde(rename = "26")]
    N26,
    /// 27
    #[serde(rename = "27")]
    N27,
    /// 28
    #[serde(rename = "28")]
    N28,
    /// 31
    #[serde(rename = "31")]
    N31,
    /// 32
    #[serde(rename = "32")]
    N32,
    /// 33
    #[serde(rename = "33")]
    N33,
    /// 34
    #[serde(rename = "34")]
    N34,
    /// 35
    #[serde(rename = "35")]
    N35,
    /// 36
    #[serde(rename = "36")]
    N36,
    /// 37
    #[serde(rename = "37")]
    N37,
    /// 38
    #[serde(rename = "38")]
    N38,
    /// 41
    #[serde(rename = "41")]
    N41,
    /// 42
    #[serde(rename = "42")]
    N42,
    /// 43
    #[serde(rename = "43")]
    N43,
    /// 44
    #[serde(rename = "44")]
    N44,
    /// 45
    #[serde(rename = "45")]
    N45,
    /// 46
    #[serde(rename = "46")]
    N46,
    /// 47
    #[serde(rename = "47")]
    N47,
    /// 48
    #[serde(rename = "48")]
    N48,
}

/// This value set includes sample Oral Prosthodontic Material type codes.
///
/// System: <http://hl7.org/fhir/ex-oralprostho>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExOralprostho {
    /// Fixed Bridge
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Maryland Bridge
    #[serde(rename = "2")]
    N2,
    /// Denture Acrylic
    #[serde(rename = "3")]
    N3,
    /// Denture Chrome Cobalt
    #[serde(rename = "4")]
    N4,
}

/// This value set includes a smattering of Pharmacy Service codes.
///
/// System: <http://hl7.org/fhir/ex-pharmaservice>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExPharmaservice {
    /// Smoking cessation
    #[default]
    #[serde(rename = "smokecess")]
    Smokecess,
    /// Flu Shot
    #[serde(rename = "flushot")]
    Flushot,
}

/// This value set includes a smattering of Service/Product codes.
///
/// System: <http://hl7.org/fhir/ex-serviceproduct>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExServiceproduct {
    /// Exam
    #[default]
    #[serde(rename = "exam")]
    Exam,
    /// Flu shot
    #[serde(rename = "flushot")]
    Flushot,
}

/// This value set includes a smattering of FDI tooth surface codes.
///
/// System: <http://hl7.org/fhir/ex-surface>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExSurface {
    /// 0
    #[default]
    #[serde(rename = "0")]
    N0,
    /// 1
    #[serde(rename = "1")]
    N1,
    /// 2
    #[serde(rename = "2")]
    N2,
    /// 3
    #[serde(rename = "3")]
    N3,
    /// 4
    #[serde(rename = "4")]
    N4,
    /// 5
    #[serde(rename = "5")]
    N5,
    /// 6
    #[serde(rename = "6")]
    N6,
    /// 7
    #[serde(rename = "7")]
    N7,
    /// 8
    #[serde(rename = "8")]
    N8,
    /// 11
    #[serde(rename = "11")]
    N11,
    /// 12
    #[serde(rename = "12")]
    N12,
    /// 13
    #[serde(rename = "13")]
    N13,
    /// 14
    #[serde(rename = "14")]
    N14,
    /// 15
    #[serde(rename = "15")]
    N15,
    /// 16
    #[serde(rename = "16")]
    N16,
    /// 17
    #[serde(rename = "17")]
    N17,
    /// 18
    #[serde(rename = "18")]
    N18,
    /// 21
    #[serde(rename = "21")]
    N21,
    /// 22
    #[serde(rename = "22")]
    N22,
    /// 23
    #[serde(rename = "23")]
    N23,
    /// 24
    #[serde(rename = "24")]
    N24,
    /// 25
    #[serde(rename = "25")]
    N25,
    /// 26
    #[serde(rename = "26")]
    N26,
    /// 27
    #[serde(rename = "27")]
    N27,
    /// 28
    #[serde(rename = "28")]
    N28,
    /// 31
    #[serde(rename = "31")]
    N31,
    /// 32
    #[serde(rename = "32")]
    N32,
    /// 33
    #[serde(rename = "33")]
    N33,
    /// 34
    #[serde(rename = "34")]
    N34,
    /// 35
    #[serde(rename = "35")]
    N35,
    /// 36
    #[serde(rename = "36")]
    N36,
    /// 37
    #[serde(rename = "37")]
    N37,
    /// 38
    #[serde(rename = "38")]
    N38,
    /// 41
    #[serde(rename = "41")]
    N41,
    /// 42
    #[serde(rename = "42")]
    N42,
    /// 43
    #[serde(rename = "43")]
    N43,
    /// 44
    #[serde(rename = "44")]
    N44,
    /// 45
    #[serde(rename = "45")]
    N45,
    /// 46
    #[serde(rename = "46")]
    N46,
    /// 47
    #[serde(rename = "47")]
    N47,
    /// 48
    #[serde(rename = "48")]
    N48,
}

/// This value set includes sample UDI codes.
///
/// System: <http://hl7.org/fhir/ex-udi>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExUdi {
    /// {01}123456789
    #[default]
    #[serde(rename = "{01}123456789")]
    N01123456789,
}

/// This value set includes a smattering of USCLS codes.
///
/// System: <http://hl7.org/fhir/ex-USCLS>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExUscls {
    /// Exam, comp, primary
    #[default]
    #[serde(rename = "1101")]
    N1101,
    /// Exam, comp, mixed
    #[serde(rename = "1102")]
    N1102,
    /// Exam, comp, permanent
    #[serde(rename = "1103")]
    N1103,
    /// Exam, recall
    #[serde(rename = "1201")]
    N1201,
    /// Exam, emergency
    #[serde(rename = "1205")]
    N1205,
    /// Radiograph, series (12)
    #[serde(rename = "2101")]
    N2101,
    /// Radiograph, series (16)
    #[serde(rename = "2102")]
    N2102,
    /// Radiograph, bytewing
    #[serde(rename = "2141")]
    N2141,
    /// Radiograph, panoramic
    #[serde(rename = "2601")]
    N2601,
    /// Polishing, 1 unit
    #[serde(rename = "11101")]
    N11101,
    /// Polishing, 2 unit
    #[serde(rename = "11102")]
    N11102,
    /// Polishing, 3 unit
    #[serde(rename = "11103")]
    N11103,
    /// Polishing, 4 unit
    #[serde(rename = "11104")]
    N11104,
    /// Amalgam, 1 surface
    #[serde(rename = "21211")]
    N21211,
    /// Amalgam, 2 surface
    #[serde(rename = "21212")]
    N21212,
    /// Crown, PFM
    #[serde(rename = "27211")]
    N27211,
    /// Lab, commercial
    #[serde(rename = "99111")]
    N99111,
    /// Lab, in office
    #[serde(rename = "99333")]
    N99333,
    /// Expense
    #[serde(rename = "99555")]
    N99555,
}

/// This value set includes a smattering of Prescription Product codes.
///
/// System: <http://hl7.org/fhir/ex-visionprescriptionproduct>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExVisionprescriptionproduct {
    /// lens
    #[default]
    #[serde(rename = "lens")]
    Lens,
    /// contact
    #[serde(rename = "contact")]
    Contact,
}

/// This value set includes sample Exception codes.
///
/// System: <http://hl7.org/fhir/exception>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Exception {
    /// Student (Fulltime)
    #[default]
    #[serde(rename = "student")]
    Student,
    /// Disabled
    #[serde(rename = "disabled")]
    Disabled,
}

/// How an extension context is interpreted.
///
/// System: <http://hl7.org/fhir/extension-context>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExtensionContext {
    /// Resource
    #[default]
    #[serde(rename = "resource")]
    Resource,
    /// Datatype
    #[serde(rename = "datatype")]
    Datatype,
    /// Mapping
    #[serde(rename = "mapping")]
    Mapping,
    /// Extension
    #[serde(rename = "extension")]
    Extension,
}

/// This value set includes a smattering of FDI tooth surface codes.
///
/// System: <http://hl7.org/fhir/FDI-surface>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FdiSurface {
    /// Mesial
    #[default]
    #[serde(rename = "M")]
    M,
    /// Occlusal
    #[serde(rename = "O")]
    O,
    /// Incisal
    #[serde(rename = "I")]
    I,
    /// Distal
    #[serde(rename = "D")]
    D,
    /// Buccal
    #[serde(rename = "B")]
    B,
    /// Ventral
    #[serde(rename = "V")]
    V,
    /// Lingual
    #[serde(rename = "L")]
    L,
    /// Mesioclusal
    #[serde(rename = "MO")]
    Mo,
    /// Distoclusal
    #[serde(rename = "DO")]
    Do,
    /// Distoincisal
    #[serde(rename = "DI")]
    Di,
    /// Mesioclusodistal
    #[serde(rename = "MOD")]
    Mod,
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
}

/// Example list of detail codes for flagged issues. (Not complete or
/// necessarily appropriate.)
///
/// System: <http://hl7.org/fhir/flag-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FlagCategory {
    /// Diet
    #[default]
    #[serde(rename = "diet")]
    Diet,
    /// Drug
    #[serde(rename = "drug")]
    Drug,
    /// Lab
    #[serde(rename = "lab")]
    Lab,
    /// Administrative
    #[serde(rename = "admin")]
    Admin,
    /// Subject contact
    #[serde(rename = "contact")]
    Contact,
}

/// This value set is provided as an exemplar. The value set is driven by IHE
/// Table B.8-4: Abnormal Flags, Alert Priority.
///
/// System: <http://hl7.org/fhir/flag-priority-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FlagPriorityCode {
    /// No alarm
    #[default]
    #[serde(rename = "PN")]
    Pn,
    /// Low priority
    #[serde(rename = "PL")]
    Pl,
    /// Medium priority
    #[serde(rename = "PM")]
    Pm,
    /// High priority
    #[serde(rename = "PH")]
    Ph,
}

/// Indicates whether this flag is active and needs to be displayed to a user,
/// or whether it is no longer needed or entered in error.
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

/// This value set includes sample Conditions codes.
///
/// System: <http://hl7.org/fhir/fm-conditions>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FmConditions {
    /// Headache
    #[default]
    #[serde(rename = "123987")]
    N123987,
}

/// This value set includes a sample set of Forms codes.
///
/// System: <http://hl7.org/fhir/forms-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FormsCodes {
    /// 1
    #[default]
    #[serde(rename = "1")]
    N1,
    /// 2
    #[serde(rename = "2")]
    N2,
}

/// This value set includes sample funds reservation type codes.
///
/// System: <http://hl7.org/fhir/fundsreserve>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Fundsreserve {
    /// patient
    #[default]
    #[serde(rename = "patient")]
    Patient,
    /// provider
    #[serde(rename = "provider")]
    Provider,
    /// none
    #[serde(rename = "none")]
    None,
}

/// Codes indicating whether the goal has been accepted by a stakeholder
///
/// System: <http://hl7.org/fhir/goal-acceptance-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalAcceptanceStatus {
    /// Agree
    #[default]
    #[serde(rename = "agree")]
    Agree,
    /// Disagree
    #[serde(rename = "disagree")]
    Disagree,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
}

/// Example codes for grouping goals for filtering or presentation.
///
/// System: <http://hl7.org/fhir/goal-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalCategory {
    /// dietary
    #[default]
    #[serde(rename = "dietary")]
    Dietary,
    /// safety
    #[serde(rename = "safety")]
    Safety,
    /// behavioral
    #[serde(rename = "behavioral")]
    Behavioral,
    /// nursing
    #[serde(rename = "nursing")]
    Nursing,
    /// physiotherapy
    #[serde(rename = "physiotherapy")]
    Physiotherapy,
}

/// Indicates the level of importance associated with reaching or sustaining a
/// goal.
///
/// System: <http://hl7.org/fhir/goal-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalPriority {
    /// high
    #[default]
    #[serde(rename = "high")]
    High,
    /// medium
    #[serde(rename = "medium")]
    Medium,
    /// low
    #[serde(rename = "low")]
    Low,
}

/// Types of relationships between two goals
///
/// System: <http://hl7.org/fhir/goal-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalRelationshipType {
    /// Predecessor
    #[default]
    #[serde(rename = "predecessor")]
    Predecessor,
    /// Successor
    #[serde(rename = "successor")]
    Successor,
    /// Replacement
    #[serde(rename = "replacement")]
    Replacement,
    /// Component
    #[serde(rename = "component")]
    Component,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// Indicates whether the goal has been met and is still being targeted
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
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Achieved
    #[serde(rename = "achieved")]
    Achieved,
    /// Sustaining
    #[serde(rename = "sustaining")]
    Sustaining,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Example codes indicating the reason for a current status. Note that these
/// are in no way complete and may not even be appropriate for some uses.
///
/// System: <http://hl7.org/fhir/goal-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GoalStatusReason {
    /// surgery
    #[default]
    #[serde(rename = "surgery")]
    Surgery,
    /// life event
    #[serde(rename = "life-event")]
    LifeEvent,
    /// replaced
    #[serde(rename = "replaced")]
    Replaced,
    /// patient request
    #[serde(rename = "patient-request")]
    PatientRequest,
}

/// Identifies special types of groups within questionnaires
///
/// System: <http://hl7.org/fhir/group-special-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GroupSpecialType {
    /// Header
    #[default]
    #[serde(rename = "header")]
    Header,
    /// Footer
    #[serde(rename = "footer")]
    Footer,
}

/// Types of resources that are part of group
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
    /// Medication
    #[serde(rename = "medication")]
    Medication,
    /// Substance
    #[serde(rename = "substance")]
    Substance,
}

/// How a dependency is represented when the guide is published.
///
/// System: <http://hl7.org/fhir/guide-dependency-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuideDependencyType {
    /// Reference
    #[default]
    #[serde(rename = "reference")]
    Reference,
    /// Inclusion
    #[serde(rename = "inclusion")]
    Inclusion,
}

/// The kind of an included page.
///
/// System: <http://hl7.org/fhir/guide-page-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuidePageKind {
    /// Page
    #[default]
    #[serde(rename = "page")]
    Page,
    /// Example
    #[serde(rename = "example")]
    Example,
    /// List
    #[serde(rename = "list")]
    List,
    /// Include
    #[serde(rename = "include")]
    Include,
    /// Directory
    #[serde(rename = "directory")]
    Directory,
    /// Dictionary
    #[serde(rename = "dictionary")]
    Dictionary,
    /// Table Of Contents
    #[serde(rename = "toc")]
    Toc,
    /// Resource
    #[serde(rename = "resource")]
    Resource,
}

/// Why a resource is included in the guide.
///
/// System: <http://hl7.org/fhir/guide-resource-purpose>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GuideResourcePurpose {
    /// Example
    #[default]
    #[serde(rename = "example")]
    Example,
    /// Terminology
    #[serde(rename = "terminology")]
    Terminology,
    /// Profile
    #[serde(rename = "profile")]
    Profile,
    /// Extension
    #[serde(rename = "extension")]
    Extension,
    /// Dictionary
    #[serde(rename = "dictionary")]
    Dictionary,
    /// Logical Model
    #[serde(rename = "logical")]
    Logical,
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
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Health unknown
    #[serde(rename = "health-unknown")]
    HealthUnknown,
}

/// HTTP verbs (in the HTTP command line).
///
/// System: <http://hl7.org/fhir/http-verb>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HttpVerb {
    /// GET
    #[default]
    #[serde(rename = "GET")]
    Get,
    /// POST
    #[serde(rename = "POST")]
    Post,
    /// PUT
    #[serde(rename = "PUT")]
    Put,
    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,
}

/// A coded type for an identifier that can be used to determine which
/// identifier to use for a specific purpose.
///
/// System: <http://hl7.org/fhir/identifier-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IdentifierType {
    /// DL
    #[default]
    #[serde(rename = "DL")]
    Dl,
    /// PPN
    #[serde(rename = "PPN")]
    Ppn,
    /// BRN
    #[serde(rename = "BRN")]
    Brn,
    /// MR
    #[serde(rename = "MR")]
    Mr,
    /// MCN
    #[serde(rename = "MCN")]
    Mcn,
    /// EN
    #[serde(rename = "EN")]
    En,
    /// TAX
    #[serde(rename = "TAX")]
    Tax,
    /// NIIP
    #[serde(rename = "NIIP")]
    Niip,
    /// PRN
    #[serde(rename = "PRN")]
    Prn,
    /// MD
    #[serde(rename = "MD")]
    Md,
    /// DR
    #[serde(rename = "DR")]
    Dr,
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

/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts
/// to support the medication process, in particular the process and reasons
/// for dispensing. This value set is provided as a suggestive example.
///
/// System: <http://hl7.org/fhir/immunization-recommendation-date-criterion>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ImmunizationRecommendationDateCriterion {
    /// Due
    #[default]
    #[serde(rename = "due")]
    Due,
    /// Recommended
    #[serde(rename = "recommended")]
    Recommended,
    /// Earliest Date
    #[serde(rename = "earliest")]
    Earliest,
    /// Past Due Date
    #[serde(rename = "overdue")]
    Overdue,
    /// Latest
    #[serde(rename = "latest")]
    Latest,
}

/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts
/// to support the medication process, in particular the process and reasons
/// for dispensing. This value set is provided as a suggestive example.
///
/// System: <http://hl7.org/fhir/immunization-recommendation-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ImmunizationRecommendationStatus {
    /// Due
    #[default]
    #[serde(rename = "due")]
    Due,
    /// Overdue
    #[serde(rename = "overdue")]
    Overdue,
}

/// This value set includes sample Intervention codes.
///
/// System: <http://hl7.org/fhir/intervention>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Intervention {
    /// Unknown
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    /// Other
    #[serde(rename = "other")]
    Other,
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
    /// Not Found
    #[serde(rename = "not-found")]
    NotFound,
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
    /// Incomplete Results
    #[serde(rename = "incomplete")]
    Incomplete,
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
    /// Throttled
    #[serde(rename = "throttled")]
    Throttled,
    /// Informational Note
    #[serde(rename = "informational")]
    Informational,
}

/// The type of link between this patient resource and another patient
/// resource.
///
/// System: <http://hl7.org/fhir/link-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LinkType {
    /// Replace
    #[default]
    #[serde(rename = "replace")]
    Replace,
    /// Refer
    #[serde(rename = "refer")]
    Refer,
    /// See also
    #[serde(rename = "seealso")]
    Seealso,
}

/// General reasons for a list to be empty. Reasons are either related to a
/// summary list (i.e. problem or medication list) or to a workflow related
/// list (i.e. consultation list).
///
/// System: <http://hl7.org/fhir/list-empty-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ListEmptyReason {
    /// Nil Known
    #[default]
    #[serde(rename = "nilknown")]
    Nilknown,
    /// Not Asked
    #[serde(rename = "notasked")]
    Notasked,
    /// Information Withheld
    #[serde(rename = "withheld")]
    Withheld,
    /// Unavailable
    #[serde(rename = "unavailable")]
    Unavailable,
    /// Not Started
    #[serde(rename = "notstarted")]
    Notstarted,
    /// Closed
    #[serde(rename = "closed")]
    Closed,
}

/// Example use codes for the List resource - typical kinds of use.
///
/// System: <http://hl7.org/fhir/list-example-use-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ListExampleUseCodes {
    /// Alerts
    #[default]
    #[serde(rename = "alerts")]
    Alerts,
    /// Adverse Reactions
    #[serde(rename = "adverserxns")]
    Adverserxns,
    /// Allergies
    #[serde(rename = "allergies")]
    Allergies,
    /// Medication List
    #[serde(rename = "medications")]
    Medications,
    /// Problem List
    #[serde(rename = "problems")]
    Problems,
    /// Worklist
    #[serde(rename = "worklist")]
    Worklist,
    /// Waiting List
    #[serde(rename = "waiting")]
    Waiting,
    /// Protocols
    #[serde(rename = "protocols")]
    Protocols,
    /// Care Plans
    #[serde(rename = "plans")]
    Plans,
}

/// The processing mode that applies to this list
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

/// Base values for the order of the items in a list resource.
///
/// System: <http://hl7.org/fhir/list-order>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ListOrder {
    /// Sorted by User
    #[default]
    #[serde(rename = "user")]
    User,
    /// Sorted by System
    #[serde(rename = "system")]
    System,
    /// Sorted by Event Date
    #[serde(rename = "event-date")]
    EventDate,
    /// Sorted by Item Date
    #[serde(rename = "entry-date")]
    EntryDate,
    /// Sorted by Priority
    #[serde(rename = "priority")]
    Priority,
    /// Sorted Alphabetically
    #[serde(rename = "alphabetic")]
    Alphabetic,
    /// Sorted by Category
    #[serde(rename = "category")]
    Category,
    /// Sorted by Patient
    #[serde(rename = "patient")]
    Patient,
}

/// The current state of the list
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

/// The answer list for question 1 on the AHRQ 'Medication or Other Substance'
/// form
///
/// System: <http://loinc.org/vs/LL2654-3>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Ll26543 {
    /// Medications
    #[default]
    #[serde(rename = "LA20271-5")]
    La202715,
    /// Biological products
    #[serde(rename = "LA20335-8")]
    La203358,
    /// Nutritional products
    #[serde(rename = "LA20336-6")]
    La203366,
    /// Expressed human breast milk
    #[serde(rename = "LA20337-4")]
    La203374,
    /// Medical gases (e.g., oxygen, nitrogen, nitrous oxide)
    #[serde(rename = "LA20338-2")]
    La203382,
    /// Contrast media
    #[serde(rename = "LA20339-0")]
    La203390,
    /// Radiopharmaceuticals
    #[serde(rename = "LA20340-8")]
    La203408,
    /// Patient food (not suspected in drug-food interactions
    #[serde(rename = "LA20341-6")]
    La203416,
    /// Drug-drug, drug-food, or adverse drug reaction as a result of a
    /// prescription and/or administration of a drug and/or food frior to
    /// admission
    #[serde(rename = "LA20342-4")]
    La203424,
    /// Other substance: PLEASE SPECIFY
    #[serde(rename = "LA20343-2")]
    La203432,
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

/// This example value set defines a set of codes that can be used to indicate
/// the physical form of the Location.
///
/// System: <http://hl7.org/fhir/location-physical-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LocationPhysicalType {
    /// Building
    #[default]
    #[serde(rename = "bu")]
    Bu,
    /// Wing
    #[serde(rename = "wi")]
    Wi,
    /// Level
    #[serde(rename = "lvl")]
    Lvl,
    /// Corridor
    #[serde(rename = "co")]
    Co,
    /// Room
    #[serde(rename = "ro")]
    Ro,
    /// Bed
    #[serde(rename = "bd")]
    Bd,
    /// Vehicle
    #[serde(rename = "ve")]
    Ve,
    /// House
    #[serde(rename = "ho")]
    Ho,
    /// Cabinet
    #[serde(rename = "ca")]
    Ca,
    /// Road
    #[serde(rename = "rd")]
    Rd,
    /// Jurisdiction
    #[serde(rename = "jdn")]
    Jdn,
    /// Area
    #[serde(rename = "area")]
    Area,
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

/// This value set defines the set of codes that can be used to indicate the
/// marital status of a person.
///
/// System: <http://hl7.org/fhir/marital-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MaritalStatus {
    /// A
    #[default]
    #[serde(rename = "A")]
    A,
    /// D
    #[serde(rename = "D")]
    D,
    /// I
    #[serde(rename = "I")]
    I,
    /// L
    #[serde(rename = "L")]
    L,
    /// M
    #[serde(rename = "M")]
    M,
    /// P
    #[serde(rename = "P")]
    P,
    /// S
    #[serde(rename = "S")]
    S,
    /// T
    #[serde(rename = "T")]
    T,
    /// W
    #[serde(rename = "W")]
    W,
    /// UNK
    #[serde(rename = "UNK")]
    Unk,
}

/// Different measurement principle supported by the device.
///
/// System: <http://hl7.org/fhir/measurement-principle>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MeasurementPrinciple {
    /// MSP Other
    #[default]
    #[serde(rename = "other")]
    Other,
    /// MSP Chemical
    #[serde(rename = "chemical")]
    Chemical,
    /// MSP Electrical
    #[serde(rename = "electrical")]
    Electrical,
    /// MSP Impedance
    #[serde(rename = "impedance")]
    Impedance,
    /// MSP Nuclear
    #[serde(rename = "nuclear")]
    Nuclear,
    /// MSP Optical
    #[serde(rename = "optical")]
    Optical,
    /// MSP Thermal
    #[serde(rename = "thermal")]
    Thermal,
    /// MSP Biological
    #[serde(rename = "biological")]
    Biological,
    /// MSP Mechanical
    #[serde(rename = "mechanical")]
    Mechanical,
    /// MSP Acoustical
    #[serde(rename = "acoustical")]
    Acoustical,
    /// MSP Manual
    #[serde(rename = "manual")]
    Manual,
}

/// Detailed information about the type of the image - its kind, purpose, or
/// the kind of equipment used to generate it.
///
/// System: <http://hl7.org/fhir/media-subtype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MediaSubtype {
    /// Diagram
    #[default]
    #[serde(rename = "diagram")]
    Diagram,
    /// Fax
    #[serde(rename = "fax")]
    Fax,
    /// Scanned Document
    #[serde(rename = "scan")]
    Scan,
    /// Retina scan
    #[serde(rename = "retina")]
    Retina,
    /// Fingerprint
    #[serde(rename = "fingerprint")]
    Fingerprint,
    /// Iris
    #[serde(rename = "iris")]
    Iris,
    /// Palm
    #[serde(rename = "palm")]
    Palm,
    /// Face
    #[serde(rename = "face")]
    Face,
}

/// A set of codes indicating the current status of a MedicationAdministration.
///
/// System: <http://hl7.org/fhir/medication-admin-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationAdminStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
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
}

/// A code specifying the state of the dispense event. Describes the lifecycle
/// of the dispense.
///
/// System: <http://hl7.org/fhir/medication-dispense-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationDispenseStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in-Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
}

/// A code specifying the state of the prescribing event. Describes the
/// lifecycle of the prescription.
///
/// System: <http://hl7.org/fhir/medication-order-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationOrderStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
}

/// A set of codes indicating the current status of a MedicationStatement.
///
/// System: <http://hl7.org/fhir/medication-statement-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MedicationStatementStatus {
    /// Active
    #[default]
    #[serde(rename = "active")]
    Active,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Intended
    #[serde(rename = "intended")]
    Intended,
}

/// The mode of a message conformance statement.
///
/// System: <http://hl7.org/fhir/message-conformance-event-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageConformanceEventMode {
    /// Sender
    #[default]
    #[serde(rename = "sender")]
    Sender,
    /// Receiver
    #[serde(rename = "receiver")]
    Receiver,
}

/// One of the message events defined as part of FHIR.
///
/// System: <http://hl7.org/fhir/message-events>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageEvents {
    /// MedicationAdministration-Complete
    #[default]
    #[serde(rename = "MedicationAdministration-Complete")]
    MedicationAdministrationComplete,
    /// MedicationAdministration-Nullification
    #[serde(rename = "MedicationAdministration-Nullification")]
    MedicationAdministrationNullification,
    /// MedicationAdministration-Recording
    #[serde(rename = "MedicationAdministration-Recording")]
    MedicationAdministrationRecording,
    /// MedicationAdministration-Update
    #[serde(rename = "MedicationAdministration-Update")]
    MedicationAdministrationUpdate,
    /// admin-notify
    #[serde(rename = "admin-notify")]
    AdminNotify,
    /// diagnosticreport-provide
    #[serde(rename = "diagnosticreport-provide")]
    DiagnosticreportProvide,
    /// observation-provide
    #[serde(rename = "observation-provide")]
    ObservationProvide,
    /// patient-link
    #[serde(rename = "patient-link")]
    PatientLink,
    /// patient-unlink
    #[serde(rename = "patient-unlink")]
    PatientUnlink,
    /// valueset-expand
    #[serde(rename = "valueset-expand")]
    ValuesetExpand,
}

/// Example Message Reasons. These are the set of codes that might be used an
/// updating an encounter using admin-update.
///
/// System: <http://hl7.org/fhir/message-reasons-encounter>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageReasonsEncounter {
    /// Admit
    #[default]
    #[serde(rename = "admit")]
    Admit,
    /// Discharge
    #[serde(rename = "discharge")]
    Discharge,
    /// Absent
    #[serde(rename = "absent")]
    Absent,
    /// Returned
    #[serde(rename = "return")]
    Return,
    /// Moved
    #[serde(rename = "moved")]
    Moved,
    /// Edit
    #[serde(rename = "edit")]
    Edit,
}

/// The impact of the content of a message.
///
/// System: <http://hl7.org/fhir/message-significance-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MessageSignificanceCategory {
    /// Consequence
    #[default]
    #[serde(rename = "Consequence")]
    Consequence,
    /// Currency
    #[serde(rename = "Currency")]
    Currency,
    /// Notification
    #[serde(rename = "Notification")]
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

/// Describes the typical color of representation.
///
/// System: <http://hl7.org/fhir/metric-color>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MetricColor {
    /// Color Black
    #[default]
    #[serde(rename = "black")]
    Black,
    /// Color Red
    #[serde(rename = "red")]
    Red,
    /// Color Green
    #[serde(rename = "green")]
    Green,
    /// Color Yellow
    #[serde(rename = "yellow")]
    Yellow,
    /// Color Blue
    #[serde(rename = "blue")]
    Blue,
    /// Color Magenta
    #[serde(rename = "magenta")]
    Magenta,
    /// Color Cyan
    #[serde(rename = "cyan")]
    Cyan,
    /// Color White
    #[serde(rename = "white")]
    White,
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
}

/// This value set includes sample Missing Tooth Reason codes.
///
/// System: <http://hl7.org/fhir/missingtoothreason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Missingtoothreason {
    /// E
    #[default]
    #[serde(rename = "E")]
    E,
    /// C
    #[serde(rename = "C")]
    C,
    /// U
    #[serde(rename = "U")]
    U,
    /// O
    #[serde(rename = "O")]
    O,
}

/// This value set includes sample Modifier type codes.
///
/// System: <http://hl7.org/fhir/modifiers>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Modifiers {
    /// Repair of prior service or installation
    #[default]
    #[serde(rename = "A")]
    A,
    /// Temporary service or installation
    #[serde(rename = "B")]
    B,
    /// TMJ treatment
    #[serde(rename = "C")]
    C,
    /// Implant or associated with an implant
    #[serde(rename = "E")]
    E,
    /// None
    #[serde(rename = "X")]
    X,
}

/// Developed for use in the collection, analysis and dissemination of
/// occupation statistics in Australia and New Zealand.
///
/// System: <http://www.abs.gov.au/ausstats/abs@.nsf/mf/1220.0>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum N12200 {
    /// Managers nfd
    #[default]
    #[serde(rename = "100000")]
    N100000,
    /// Chief Executives, General Managers and Legislators nfd
    #[serde(rename = "111000")]
    N111000,
    /// Chief Executive or Managing Director
    #[serde(rename = "111111")]
    N111111,
    /// General Managers nfd
    #[serde(rename = "111200")]
    N111200,
    /// Corporate General Manager
    #[serde(rename = "111211")]
    N111211,
    /// Defence Force Senior Officer
    #[serde(rename = "111212")]
    N111212,
    /// Legislators nfd
    #[serde(rename = "111300")]
    N111300,
    /// Local Government Legislator
    #[serde(rename = "111311")]
    N111311,
    /// Member of Parliament
    #[serde(rename = "111312")]
    N111312,
    /// Legislators nec
    #[serde(rename = "111399")]
    N111399,
    /// Farmers and Farm Managers nfd
    #[serde(rename = "121000")]
    N121000,
    /// Aquaculture Farmer
    #[serde(rename = "121111")]
    N121111,
    /// Crop Farmers nfd
    #[serde(rename = "121200")]
    N121200,
    /// Cotton Grower
    #[serde(rename = "121211")]
    N121211,
    /// Flower Grower
    #[serde(rename = "121212")]
    N121212,
    /// Fruit or Nut Grower
    #[serde(rename = "121213")]
    N121213,
    /// Grain, Oilseed or Pasture Grower
    #[serde(rename = "121214")]
    N121214,
    /// Grape Grower
    #[serde(rename = "121215")]
    N121215,
    /// Mixed Crop Farmer
    #[serde(rename = "121216")]
    N121216,
    /// Sugar Cane Grower
    #[serde(rename = "121217")]
    N121217,
    /// Turf Grower
    #[serde(rename = "121218")]
    N121218,
    /// Vegetable Grower
    #[serde(rename = "121221")]
    N121221,
    /// Crop Farmers nec
    #[serde(rename = "121299")]
    N121299,
    /// Livestock Farmers nfd
    #[serde(rename = "121300")]
    N121300,
    /// Apiarist
    #[serde(rename = "121311")]
    N121311,
    /// Beef Cattle Farmer
    #[serde(rename = "121312")]
    N121312,
    /// Dairy Cattle Farmer
    #[serde(rename = "121313")]
    N121313,
    /// Deer Farmer
    #[serde(rename = "121314")]
    N121314,
    /// Goat Farmer
    #[serde(rename = "121315")]
    N121315,
    /// Horse Breeder
    #[serde(rename = "121316")]
    N121316,
    /// Mixed Livestock Farmer
    #[serde(rename = "121317")]
    N121317,
    /// Pig Farmer
    #[serde(rename = "121318")]
    N121318,
    /// Poultry Farmer
    #[serde(rename = "121321")]
    N121321,
    /// Sheep Farmer
    #[serde(rename = "121322")]
    N121322,
    /// Livestock Farmers nec
    #[serde(rename = "121399")]
    N121399,
    /// Mixed Crop and Livestock Farmer
    #[serde(rename = "121411")]
    N121411,
    /// Specialist Managers nfd
    #[serde(rename = "130000")]
    N130000,
    /// Advertising and Sales Managers nfd
    #[serde(rename = "131100")]
    N131100,
    /// Advertising and Public Relations Manager
    #[serde(rename = "131111")]
    N131111,
    /// Sales and Marketing Manager
    #[serde(rename = "131112")]
    N131112,
    /// Business Administration Managers nfd
    #[serde(rename = "132000")]
    N132000,
    /// Corporate Services Manager
    #[serde(rename = "132111")]
    N132111,
    /// Finance Manager
    #[serde(rename = "132211")]
    N132211,
    /// Human Resource Manager
    #[serde(rename = "132311")]
    N132311,
    /// Policy and Planning Manager
    #[serde(rename = "132411")]
    N132411,
    /// Research and Development Manager
    #[serde(rename = "132511")]
    N132511,
    /// Construction, Distribution and Production Managers nfd
    #[serde(rename = "133000")]
    N133000,
    /// Construction Managers nfd
    #[serde(rename = "133100")]
    N133100,
    /// Construction Project Manager
    #[serde(rename = "133111")]
    N133111,
    /// Project Builder
    #[serde(rename = "133112")]
    N133112,
    /// Engineering Manager
    #[serde(rename = "133211")]
    N133211,
    /// Importers, Exporters and Wholesalers nfd
    #[serde(rename = "133300")]
    N133300,
    /// Importer or Exporter
    #[serde(rename = "133311")]
    N133311,
    /// Wholesaler
    #[serde(rename = "133312")]
    N133312,
    /// Manufacturer
    #[serde(rename = "133411")]
    N133411,
    /// Production Managers nfd
    #[serde(rename = "133500")]
    N133500,
    /// Production Manager (Forestry)
    #[serde(rename = "133511")]
    N133511,
    /// Production Manager (Manufacturing)
    #[serde(rename = "133512")]
    N133512,
    /// Production Manager (Mining)
    #[serde(rename = "133513")]
    N133513,
    /// Supply and Distribution Manager
    #[serde(rename = "133611")]
    N133611,
    /// Education, Health and Welfare Services Managers nfd
    #[serde(rename = "134000")]
    N134000,
    /// Child Care Centre Manager
    #[serde(rename = "134111")]
    N134111,
    /// Health and Welfare Services Managers nfd
    #[serde(rename = "134200")]
    N134200,
    /// Medical Administrator
    #[serde(rename = "134211")]
    N134211,
    /// Nursing Clinical Director
    #[serde(rename = "134212")]
    N134212,
    /// Primary Health Organization Manager
    #[serde(rename = "134213")]
    N134213,
    /// Welfare Centre Manager
    #[serde(rename = "134214")]
    N134214,
    /// Health and Welfare Services Managers nec
    #[serde(rename = "134299")]
    N134299,
    /// School Principal
    #[serde(rename = "134311")]
    N134311,
    /// Other Education Managers nfd
    #[serde(rename = "134400")]
    N134400,
    /// Faculty Head
    #[serde(rename = "134411")]
    N134411,
    /// Regional Education Manager
    #[serde(rename = "134412")]
    N134412,
    /// Education Managers nec
    #[serde(rename = "134499")]
    N134499,
    /// ICT Managers nfd
    #[serde(rename = "135100")]
    N135100,
    /// Chief Information Officer
    #[serde(rename = "135111")]
    N135111,
    /// ICT Project Manager
    #[serde(rename = "135112")]
    N135112,
    /// ICT Managers nec
    #[serde(rename = "135199")]
    N135199,
    /// Miscellaneous Specialist Managers nfd
    #[serde(rename = "139000")]
    N139000,
    /// Commissioned Officers (Management) nfd
    #[serde(rename = "139100")]
    N139100,
    /// Commissioned Defence Force Officer
    #[serde(rename = "139111")]
    N139111,
    /// Commissioned Fire Officer
    #[serde(rename = "139112")]
    N139112,
    /// Commissioned Police Officer
    #[serde(rename = "139113")]
    N139113,
    /// Senior Non-commissioned Defence Force Member
    #[serde(rename = "139211")]
    N139211,
    /// Other Specialist Managers nfd
    #[serde(rename = "139900")]
    N139900,
    /// Arts Administrator or Manager
    #[serde(rename = "139911")]
    N139911,
    /// Environmental Manager
    #[serde(rename = "139912")]
    N139912,
    /// Laboratory Manager
    #[serde(rename = "139913")]
    N139913,
    /// Quality Assurance Manager
    #[serde(rename = "139914")]
    N139914,
    /// Sports Administrator
    #[serde(rename = "139915")]
    N139915,
    /// Specialist Managers nec
    #[serde(rename = "139999")]
    N139999,
    /// Hospitality, Retail and Service Managers nfd
    #[serde(rename = "140000")]
    N140000,
    /// Accommodation and Hospitality Managers nfd
    #[serde(rename = "141000")]
    N141000,
    /// Cafe or Restaurant Manager
    #[serde(rename = "141111")]
    N141111,
    /// Caravan Park and Camping Ground Manager
    #[serde(rename = "141211")]
    N141211,
    /// Hotel or Motel Manager
    #[serde(rename = "141311")]
    N141311,
    /// Licensed Club Manager
    #[serde(rename = "141411")]
    N141411,
    /// Other Accommodation and Hospitality Managers nfd
    #[serde(rename = "141900")]
    N141900,
    /// Bed and Breakfast Operator
    #[serde(rename = "141911")]
    N141911,
    /// Accommodation and Hospitality Managers nec
    #[serde(rename = "141999")]
    N141999,
    /// Retail Managers nfd
    #[serde(rename = "142100")]
    N142100,
    /// Retail Manager (General)
    #[serde(rename = "142111")]
    N142111,
    /// Antique Dealer
    #[serde(rename = "142112")]
    N142112,
    /// Betting Agency Manager
    #[serde(rename = "142113")]
    N142113,
    /// Hair or Beauty Salon Manager
    #[serde(rename = "142114")]
    N142114,
    /// Post Office Manager
    #[serde(rename = "142115")]
    N142115,
    /// Travel Agency Manager
    #[serde(rename = "142116")]
    N142116,
    /// Miscellaneous Hospitality, Retail and Service Managers nfd
    #[serde(rename = "149000")]
    N149000,
    /// Amusement, Fitness and Sports Centre Managers nfd
    #[serde(rename = "149100")]
    N149100,
    /// Amusement Centre Manager
    #[serde(rename = "149111")]
    N149111,
    /// Fitness Centre Manager
    #[serde(rename = "149112")]
    N149112,
    /// Sports Centre Manager
    #[serde(rename = "149113")]
    N149113,
    /// Call or Contact Centre and Customer Service Managers nfd
    #[serde(rename = "149200")]
    N149200,
    /// Call or Contact Centre Manager
    #[serde(rename = "149211")]
    N149211,
    /// Customer Service Manager
    #[serde(rename = "149212")]
    N149212,
    /// Conference and Event Organizer
    #[serde(rename = "149311")]
    N149311,
    /// Transport Services Managers nfd
    #[serde(rename = "149400")]
    N149400,
    /// Fleet Manager
    #[serde(rename = "149411")]
    N149411,
    /// Railway Station Manager
    #[serde(rename = "149412")]
    N149412,
    /// Transport Company Manager
    #[serde(rename = "149413")]
    N149413,
    /// Other Hospitality, Retail and Service Managers nfd
    #[serde(rename = "149900")]
    N149900,
    /// Boarding Kennel or Cattery Operator
    #[serde(rename = "149911")]
    N149911,
    /// Cinema or Theatre Manager
    #[serde(rename = "149912")]
    N149912,
    /// Facilities Manager
    #[serde(rename = "149913")]
    N149913,
    /// Financial Institution Branch Manager
    #[serde(rename = "149914")]
    N149914,
    /// Hospitality, Retail and Service Managers nec
    #[serde(rename = "149999")]
    N149999,
    /// Professionals nfd
    #[serde(rename = "200000")]
    N200000,
    /// Arts and Media Professionals nfd
    #[serde(rename = "210000")]
    N210000,
    /// Arts Professionals nfd
    #[serde(rename = "211000")]
    N211000,
    /// Actors, Dancers and Other Entertainers nfd
    #[serde(rename = "211100")]
    N211100,
    /// Actor
    #[serde(rename = "211111")]
    N211111,
    /// Dancer or Choreographer
    #[serde(rename = "211112")]
    N211112,
    /// Entertainer or Variety Artist
    #[serde(rename = "211113")]
    N211113,
    /// Actors, Dancers and Other Entertainers nec
    #[serde(rename = "211199")]
    N211199,
    /// Music Professionals nfd
    #[serde(rename = "211200")]
    N211200,
    /// Composer
    #[serde(rename = "211211")]
    N211211,
    /// Music Director
    #[serde(rename = "211212")]
    N211212,
    /// Musician (Instrumental)
    #[serde(rename = "211213")]
    N211213,
    /// Singer
    #[serde(rename = "211214")]
    N211214,
    /// Music Professionals nec
    #[serde(rename = "211299")]
    N211299,
    /// Photographer
    #[serde(rename = "211311")]
    N211311,
    /// Visual Arts and Crafts Professionals nfd
    #[serde(rename = "211400")]
    N211400,
    /// Painter (Visual Arts)
    #[serde(rename = "211411")]
    N211411,
    /// Potter or Ceramic Artist
    #[serde(rename = "211412")]
    N211412,
    /// Sculptor
    #[serde(rename = "211413")]
    N211413,
    /// Visual Arts and Crafts Professionals nec
    #[serde(rename = "211499")]
    N211499,
    /// Media Professionals nfd
    #[serde(rename = "212000")]
    N212000,
    /// Artistic Directors, and Media Producers and Presenters nfd
    #[serde(rename = "212100")]
    N212100,
    /// Artistic Director
    #[serde(rename = "212111")]
    N212111,
    /// Media Producer (excluding Video)
    #[serde(rename = "212112")]
    N212112,
    /// Radio Presenter
    #[serde(rename = "212113")]
    N212113,
    /// Television Presenter
    #[serde(rename = "212114")]
    N212114,
    /// Authors, and Book and Script Editors nfd
    #[serde(rename = "212200")]
    N212200,
    /// Author
    #[serde(rename = "212211")]
    N212211,
    /// Book or Script Editor
    #[serde(rename = "212212")]
    N212212,
    /// Film, Television, Radio and Stage Directors nfd
    #[serde(rename = "212300")]
    N212300,
    /// Art Director (Film, Television or Stage)
    #[serde(rename = "212311")]
    N212311,
    /// Director (Film, Television, Radio or Stage)
    #[serde(rename = "212312")]
    N212312,
    /// Director of Photography
    #[serde(rename = "212313")]
    N212313,
    /// Film and Video Editor
    #[serde(rename = "212314")]
    N212314,
    /// Program Director (Television or Radio)
    #[serde(rename = "212315")]
    N212315,
    /// Stage Manager
    #[serde(rename = "212316")]
    N212316,
    /// Technical Director
    #[serde(rename = "212317")]
    N212317,
    /// Video Producer
    #[serde(rename = "212318")]
    N212318,
    /// Film, Television, Radio and Stage Directors nec
    #[serde(rename = "212399")]
    N212399,
    /// Journalists and Other Writers nfd
    #[serde(rename = "212400")]
    N212400,
    /// Copywriter
    #[serde(rename = "212411")]
    N212411,
    /// Newspaper or Periodical Editor
    #[serde(rename = "212412")]
    N212412,
    /// Print Journalist
    #[serde(rename = "212413")]
    N212413,
    /// Radio Journalist
    #[serde(rename = "212414")]
    N212414,
    /// Technical Writer
    #[serde(rename = "212415")]
    N212415,
    /// Television Journalist
    #[serde(rename = "212416")]
    N212416,
    /// Journalists and Other Writers nec
    #[serde(rename = "212499")]
    N212499,
    /// Business, Human Resource and Marketing Professionals nfd
    #[serde(rename = "220000")]
    N220000,
    /// Accountants, Auditors and Company Secretaries nfd
    #[serde(rename = "221000")]
    N221000,
    /// Accountants nfd
    #[serde(rename = "221100")]
    N221100,
    /// Accountant (General)
    #[serde(rename = "221111")]
    N221111,
    /// Management Accountant
    #[serde(rename = "221112")]
    N221112,
    /// Taxation Accountant
    #[serde(rename = "221113")]
    N221113,
    /// Auditors, Company Secretaries and Corporate Treasurers nfd
    #[serde(rename = "221200")]
    N221200,
    /// Company Secretary
    #[serde(rename = "221211")]
    N221211,
    /// Corporate Treasurer
    #[serde(rename = "221212")]
    N221212,
    /// External Auditor
    #[serde(rename = "221213")]
    N221213,
    /// Internal Auditor
    #[serde(rename = "221214")]
    N221214,
    /// Financial Brokers and Dealers, and Investment Advisers nfd
    #[serde(rename = "222000")]
    N222000,
    /// Financial Brokers nfd
    #[serde(rename = "222100")]
    N222100,
    /// Commodities Trader
    #[serde(rename = "222111")]
    N222111,
    /// Finance Broker
    #[serde(rename = "222112")]
    N222112,
    /// Insurance Broker
    #[serde(rename = "222113")]
    N222113,
    /// Financial Brokers nec
    #[serde(rename = "222199")]
    N222199,
    /// Financial Dealers nfd
    #[serde(rename = "222200")]
    N222200,
    /// Financial Market Dealer
    #[serde(rename = "222211")]
    N222211,
    /// Futures Trader
    #[serde(rename = "222212")]
    N222212,
    /// Stockbroking Dealer
    #[serde(rename = "222213")]
    N222213,
    /// Financial Dealers nec
    #[serde(rename = "222299")]
    N222299,
    /// Financial Investment Advisers and Managers nfd
    #[serde(rename = "222300")]
    N222300,
    /// Financial Investment Adviser
    #[serde(rename = "222311")]
    N222311,
    /// Financial Investment Manager
    #[serde(rename = "222312")]
    N222312,
    /// Human Resource and Training Professionals nfd
    #[serde(rename = "223000")]
    N223000,
    /// Human Resource Professionals nfd
    #[serde(rename = "223100")]
    N223100,
    /// Human Resource Adviser
    #[serde(rename = "223111")]
    N223111,
    /// Recruitment Consultant
    #[serde(rename = "223112")]
    N223112,
    /// Workplace Relations Adviser
    #[serde(rename = "223113")]
    N223113,
    /// ICT Trainer
    #[serde(rename = "223211")]
    N223211,
    /// Training and Development Professional
    #[serde(rename = "223311")]
    N223311,
    /// Information and Organization Professionals nfd
    #[serde(rename = "224000")]
    N224000,
    /// Actuaries, Mathematicians and Statisticians nfd
    #[serde(rename = "224100")]
    N224100,
    /// Actuary
    #[serde(rename = "224111")]
    N224111,
    /// Mathematician
    #[serde(rename = "224112")]
    N224112,
    /// Statistician
    #[serde(rename = "224113")]
    N224113,
    /// Archivists, Curators and Records Managers nfd
    #[serde(rename = "224200")]
    N224200,
    /// Archivist
    #[serde(rename = "224211")]
    N224211,
    /// Gallery or Museum Curator
    #[serde(rename = "224212")]
    N224212,
    /// Health Information Manager
    #[serde(rename = "224213")]
    N224213,
    /// Records Manager
    #[serde(rename = "224214")]
    N224214,
    /// Economist
    #[serde(rename = "224311")]
    N224311,
    /// Intelligence and Policy Analysts nfd
    #[serde(rename = "224400")]
    N224400,
    /// Intelligence Officer
    #[serde(rename = "224411")]
    N224411,
    /// Policy Analyst
    #[serde(rename = "224412")]
    N224412,
    /// Land Economists and Valuers nfd
    #[serde(rename = "224500")]
    N224500,
    /// Land Economist
    #[serde(rename = "224511")]
    N224511,
    /// Valuer
    #[serde(rename = "224512")]
    N224512,
    /// Librarian
    #[serde(rename = "224611")]
    N224611,
    /// Management and Organization Analysts nfd
    #[serde(rename = "224700")]
    N224700,
    /// Management Consultant
    #[serde(rename = "224711")]
    N224711,
    /// Organization and Methods Analyst
    #[serde(rename = "224712")]
    N224712,
    /// Other Information and Organization Professionals nfd
    #[serde(rename = "224900")]
    N224900,
    /// Electorate Officer
    #[serde(rename = "224911")]
    N224911,
    /// Liaison Officer
    #[serde(rename = "224912")]
    N224912,
    /// Migration Agent
    #[serde(rename = "224913")]
    N224913,
    /// Patents Examiner
    #[serde(rename = "224914")]
    N224914,
    /// Information and Organization Professionals nec
    #[serde(rename = "224999")]
    N224999,
    /// Sales, Marketing and Public Relations Professionals nfd
    #[serde(rename = "225000")]
    N225000,
    /// Advertising and Marketing Professionals nfd
    #[serde(rename = "225100")]
    N225100,
    /// Advertising Specialist
    #[serde(rename = "225111")]
    N225111,
    /// Market Research Analyst
    #[serde(rename = "225112")]
    N225112,
    /// Marketing Specialist
    #[serde(rename = "225113")]
    N225113,
    /// ICT Sales Professionals nfd
    #[serde(rename = "225200")]
    N225200,
    /// ICT Account Manager
    #[serde(rename = "225211")]
    N225211,
    /// ICT Business Development Manager
    #[serde(rename = "225212")]
    N225212,
    /// ICT Sales Representative
    #[serde(rename = "225213")]
    N225213,
    /// Public Relations Professional
    #[serde(rename = "225311")]
    N225311,
    /// Technical Sales Representatives nfd
    #[serde(rename = "225400")]
    N225400,
    /// Sales Representative (Industrial Products)
    #[serde(rename = "225411")]
    N225411,
    /// Sales Representative (Medical and Pharmaceutical Products)
    #[serde(rename = "225412")]
    N225412,
    /// Technical Sales Representatives nec
    #[serde(rename = "225499")]
    N225499,
    /// Design, Engineering, Science and Transport Professionals nfd
    #[serde(rename = "230000")]
    N230000,
    /// Air and Marine Transport Professionals nfd
    #[serde(rename = "231000")]
    N231000,
    /// Air Transport Professionals nfd
    #[serde(rename = "231100")]
    N231100,
    /// Aeroplane Pilot
    #[serde(rename = "231111")]
    N231111,
    /// Air Traffic Controller
    #[serde(rename = "231112")]
    N231112,
    /// Flying Instructor
    #[serde(rename = "231113")]
    N231113,
    /// Helicopter Pilot
    #[serde(rename = "231114")]
    N231114,
    /// Air Transport Professionals nec
    #[serde(rename = "231199")]
    N231199,
    /// Marine Transport Professionals nfd
    #[serde(rename = "231200")]
    N231200,
    /// Master Fisher
    #[serde(rename = "231211")]
    N231211,
    /// Ship's Engineer
    #[serde(rename = "231212")]
    N231212,
    /// Ship's Master
    #[serde(rename = "231213")]
    N231213,
    /// Ship's Officer
    #[serde(rename = "231214")]
    N231214,
    /// Ship's Surveyor
    #[serde(rename = "231215")]
    N231215,
    /// Marine Transport Professionals nec
    #[serde(rename = "231299")]
    N231299,
    /// Architects, Designers, Planners and Surveyors nfd
    #[serde(rename = "232000")]
    N232000,
    /// Architects and Landscape Architects nfd
    #[serde(rename = "232100")]
    N232100,
    /// Architect
    #[serde(rename = "232111")]
    N232111,
    /// Landscape Architect
    #[serde(rename = "232112")]
    N232112,
    /// Cartographers and Surveyors nfd
    #[serde(rename = "232200")]
    N232200,
    /// Cartographer
    #[serde(rename = "232211")]
    N232211,
    /// Surveyor
    #[serde(rename = "232212")]
    N232212,
    /// Fashion, Industrial and Jewellery Designers nfd
    #[serde(rename = "232300")]
    N232300,
    /// Fashion Designer
    #[serde(rename = "232311")]
    N232311,
    /// Industrial Designer
    #[serde(rename = "232312")]
    N232312,
    /// Jewellery Designer
    #[serde(rename = "232313")]
    N232313,
    /// Graphic and Web Designers, and Illustrators nfd
    #[serde(rename = "232400")]
    N232400,
    /// Graphic Designer
    #[serde(rename = "232411")]
    N232411,
    /// Illustrator
    #[serde(rename = "232412")]
    N232412,
    /// Multimedia Designer
    #[serde(rename = "232413")]
    N232413,
    /// Web Designer
    #[serde(rename = "232414")]
    N232414,
    /// Interior Designer
    #[serde(rename = "232511")]
    N232511,
    /// Urban and Regional Planner
    #[serde(rename = "232611")]
    N232611,
    /// Engineering Professionals nfd
    #[serde(rename = "233000")]
    N233000,
    /// Chemical and Materials Engineers nfd
    #[serde(rename = "233100")]
    N233100,
    /// Chemical Engineer
    #[serde(rename = "233111")]
    N233111,
    /// Materials Engineer
    #[serde(rename = "233112")]
    N233112,
    /// Civil Engineering Professionals nfd
    #[serde(rename = "233200")]
    N233200,
    /// Civil Engineer
    #[serde(rename = "233211")]
    N233211,
    /// Geotechnical Engineer
    #[serde(rename = "233212")]
    N233212,
    /// Quantity Surveyor
    #[serde(rename = "233213")]
    N233213,
    /// Structural Engineer
    #[serde(rename = "233214")]
    N233214,
    /// Transport Engineer
    #[serde(rename = "233215")]
    N233215,
    /// Electrical Engineer
    #[serde(rename = "233311")]
    N233311,
    /// Electronics Engineer
    #[serde(rename = "233411")]
    N233411,
    /// Industrial, Mechanical and Production Engineers nfd
    #[serde(rename = "233500")]
    N233500,
    /// Industrial Engineer
    #[serde(rename = "233511")]
    N233511,
    /// Mechanical Engineer
    #[serde(rename = "233512")]
    N233512,
    /// Production or Plant Engineer
    #[serde(rename = "233513")]
    N233513,
    /// Mining Engineers nfd
    #[serde(rename = "233600")]
    N233600,
    /// Mining Engineer (excluding Petroleum)
    #[serde(rename = "233611")]
    N233611,
    /// Petroleum Engineer
    #[serde(rename = "233612")]
    N233612,
    /// Other Engineering Professionals nfd
    #[serde(rename = "233900")]
    N233900,
    /// Aeronautical Engineer
    #[serde(rename = "233911")]
    N233911,
    /// Agricultural Engineer
    #[serde(rename = "233912")]
    N233912,
    /// Biomedical Engineer
    #[serde(rename = "233913")]
    N233913,
    /// Engineering Technologist
    #[serde(rename = "233914")]
    N233914,
    /// Environmental Engineer
    #[serde(rename = "233915")]
    N233915,
    /// Naval Architect
    #[serde(rename = "233916")]
    N233916,
    /// Engineering Professionals nec
    #[serde(rename = "233999")]
    N233999,
    /// Natural and Physical Science Professionals nfd
    #[serde(rename = "234000")]
    N234000,
    /// Agricultural and Forestry Scientists nfd
    #[serde(rename = "234100")]
    N234100,
    /// Agricultural Consultant
    #[serde(rename = "234111")]
    N234111,
    /// Agricultural Scientist
    #[serde(rename = "234112")]
    N234112,
    /// Forester
    #[serde(rename = "234113")]
    N234113,
    /// Chemists, and Food and Wine Scientists nfd
    #[serde(rename = "234200")]
    N234200,
    /// Chemist
    #[serde(rename = "234211")]
    N234211,
    /// Food Technologist
    #[serde(rename = "234212")]
    N234212,
    /// Wine Maker
    #[serde(rename = "234213")]
    N234213,
    /// Environmental Scientists nfd
    #[serde(rename = "234300")]
    N234300,
    /// Conservation Officer
    #[serde(rename = "234311")]
    N234311,
    /// Environmental Consultant
    #[serde(rename = "234312")]
    N234312,
    /// Environmental Research Scientist
    #[serde(rename = "234313")]
    N234313,
    /// Park Ranger
    #[serde(rename = "234314")]
    N234314,
    /// Environmental Scientists nec
    #[serde(rename = "234399")]
    N234399,
    /// Geologists and Geophysicists nfd
    #[serde(rename = "234400")]
    N234400,
    /// Geologist
    #[serde(rename = "234411")]
    N234411,
    /// Geophysicist
    #[serde(rename = "234412")]
    N234412,
    /// Life Scientists nfd
    #[serde(rename = "234500")]
    N234500,
    /// Life Scientist (General)
    #[serde(rename = "234511")]
    N234511,
    /// Anatomist or Physiologist
    #[serde(rename = "234512")]
    N234512,
    /// Biochemist
    #[serde(rename = "234513")]
    N234513,
    /// Biotechnologist
    #[serde(rename = "234514")]
    N234514,
    /// Botanist
    #[serde(rename = "234515")]
    N234515,
    /// Marine Biologist
    #[serde(rename = "234516")]
    N234516,
    /// Microbiologist
    #[serde(rename = "234517")]
    N234517,
    /// Zoologist
    #[serde(rename = "234518")]
    N234518,
    /// Life Scientists nec
    #[serde(rename = "234599")]
    N234599,
    /// Medical Laboratory Scientist
    #[serde(rename = "234611")]
    N234611,
    /// Veterinarian
    #[serde(rename = "234711")]
    N234711,
    /// Other Natural and Physical Science Professionals nfd
    #[serde(rename = "234900")]
    N234900,
    /// Conservator
    #[serde(rename = "234911")]
    N234911,
    /// Metallurgist
    #[serde(rename = "234912")]
    N234912,
    /// Meteorologist
    #[serde(rename = "234913")]
    N234913,
    /// Physicist
    #[serde(rename = "234914")]
    N234914,
    /// Natural and Physical Science Professionals nec
    #[serde(rename = "234999")]
    N234999,
    /// Education Professionals nfd
    #[serde(rename = "240000")]
    N240000,
    /// School Teachers nfd
    #[serde(rename = "241000")]
    N241000,
    /// Early Childhood (Pre-primary School) Teacher
    #[serde(rename = "241111")]
    N241111,
    /// Primary School Teacher
    #[serde(rename = "241213")]
    N241213,
    /// Middle School Teacher
    #[serde(rename = "241311")]
    N241311,
    /// Secondary School Teacher
    #[serde(rename = "241411")]
    N241411,
    /// Special Education Teachers nfd
    #[serde(rename = "241500")]
    N241500,
    /// Special Needs Teacher
    #[serde(rename = "241511")]
    N241511,
    /// Teacher of the Hearing Impaired
    #[serde(rename = "241512")]
    N241512,
    /// Teacher of the Sight Impaired
    #[serde(rename = "241513")]
    N241513,
    /// Special Education Teachers nec
    #[serde(rename = "241599")]
    N241599,
    /// Tertiary Education Teachers nfd
    #[serde(rename = "242000")]
    N242000,
    /// University Lecturers and Tutors nfd
    #[serde(rename = "242100")]
    N242100,
    /// University Lecturer
    #[serde(rename = "242111")]
    N242111,
    /// University Tutor
    #[serde(rename = "242112")]
    N242112,
    /// Vocational Education Teacher
    #[serde(rename = "242211")]
    N242211,
    /// Miscellaneous Education Professionals nfd
    #[serde(rename = "249000")]
    N249000,
    /// Education Advisers and Reviewers nfd
    #[serde(rename = "249100")]
    N249100,
    /// Education Adviser
    #[serde(rename = "249111")]
    N249111,
    /// Education Reviewer
    #[serde(rename = "249112")]
    N249112,
    /// Private Tutors and Teachers nfd
    #[serde(rename = "249200")]
    N249200,
    /// Art Teacher (Private Tuition)
    #[serde(rename = "249211")]
    N249211,
    /// Dance Teacher (Private Tuition)
    #[serde(rename = "249212")]
    N249212,
    /// Drama Teacher (Private Tuition)
    #[serde(rename = "249213")]
    N249213,
    /// Music Teacher (Private Tuition)
    #[serde(rename = "249214")]
    N249214,
    /// Private Tutors and Teachers nec
    #[serde(rename = "249299")]
    N249299,
    /// Teacher of English to Speakers of Other Languages
    #[serde(rename = "249311")]
    N249311,
    /// Health Professionals nfd
    #[serde(rename = "250000")]
    N250000,
    /// Health Diagnostic and Promotion Professionals nfd
    #[serde(rename = "251000")]
    N251000,
    /// Dietitian
    #[serde(rename = "251111")]
    N251111,
    /// Medical Imaging Professionals nfd
    #[serde(rename = "251200")]
    N251200,
    /// Medical Diagnostic Radiographer
    #[serde(rename = "251211")]
    N251211,
    /// Medical Radiation Therapist
    #[serde(rename = "251212")]
    N251212,
    /// Nuclear Medicine Technologist
    #[serde(rename = "251213")]
    N251213,
    /// Sonographer
    #[serde(rename = "251214")]
    N251214,
    /// Occupational and Environmental Health Professionals nfd
    #[serde(rename = "251300")]
    N251300,
    /// Environmental Health Officer
    #[serde(rename = "251311")]
    N251311,
    /// Occupational Health and Safety Adviser
    #[serde(rename = "251312")]
    N251312,
    /// Optometrists and Orthoptists nfd
    #[serde(rename = "251400")]
    N251400,
    /// Optometrist
    #[serde(rename = "251411")]
    N251411,
    /// Orthoptist
    #[serde(rename = "251412")]
    N251412,
    /// Pharmacists nfd
    #[serde(rename = "251500")]
    N251500,
    /// Hospital Pharmacist
    #[serde(rename = "251511")]
    N251511,
    /// Industrial Pharmacist
    #[serde(rename = "251512")]
    N251512,
    /// Retail Pharmacist
    #[serde(rename = "251513")]
    N251513,
    /// Other Health Diagnostic and Promotion Professionals nfd
    #[serde(rename = "251900")]
    N251900,
    /// Health Promotion Officer
    #[serde(rename = "251911")]
    N251911,
    /// Orthotist or Prosthetist
    #[serde(rename = "251912")]
    N251912,
    /// Health Diagnostic and Promotion Professionals nec
    #[serde(rename = "251999")]
    N251999,
    /// Health Therapy Professionals nfd
    #[serde(rename = "252000")]
    N252000,
    /// Chiropractors and Osteopaths nfd
    #[serde(rename = "252100")]
    N252100,
    /// Chiropractor
    #[serde(rename = "252111")]
    N252111,
    /// Osteopath
    #[serde(rename = "252112")]
    N252112,
    /// Complementary Health Therapists nfd
    #[serde(rename = "252200")]
    N252200,
    /// Acupuncturist
    #[serde(rename = "252211")]
    N252211,
    /// Homoeopath
    #[serde(rename = "252212")]
    N252212,
    /// Naturopath
    #[serde(rename = "252213")]
    N252213,
    /// Traditional Chinese Medicine Practitioner
    #[serde(rename = "252214")]
    N252214,
    /// Complementary Health Therapists nec
    #[serde(rename = "252299")]
    N252299,
    /// Dental Practitioners nfd
    #[serde(rename = "252300")]
    N252300,
    /// Dental Specialist
    #[serde(rename = "252311")]
    N252311,
    /// Dentist
    #[serde(rename = "252312")]
    N252312,
    /// Occupational Therapist
    #[serde(rename = "252411")]
    N252411,
    /// Physiotherapist
    #[serde(rename = "252511")]
    N252511,
    /// Podiatrist
    #[serde(rename = "252611")]
    N252611,
    /// Speech Professionals and Audiologists nfd
    #[serde(rename = "252700")]
    N252700,
    /// Audiologist
    #[serde(rename = "252711")]
    N252711,
    /// Speech Pathologist
    #[serde(rename = "252712")]
    N252712,
    /// Medical Practitioners nfd
    #[serde(rename = "253000")]
    N253000,
    /// Generalist Medical Practitioners nfd
    #[serde(rename = "253100")]
    N253100,
    /// General Medical Practitioner
    #[serde(rename = "253111")]
    N253111,
    /// Resident Medical Officer
    #[serde(rename = "253112")]
    N253112,
    /// Anaesthetist
    #[serde(rename = "253211")]
    N253211,
    /// Internal Medicine Specialists nfd
    #[serde(rename = "253300")]
    N253300,
    /// Specialist Physician (General Medicine)
    #[serde(rename = "253311")]
    N253311,
    /// Cardiologist
    #[serde(rename = "253312")]
    N253312,
    /// Clinical Haematologist
    #[serde(rename = "253313")]
    N253313,
    /// Clinical Oncologist
    #[serde(rename = "253314")]
    N253314,
    /// Endocrinologist
    #[serde(rename = "253315")]
    N253315,
    /// Gastroenterologist
    #[serde(rename = "253316")]
    N253316,
    /// Intensive Care Specialist
    #[serde(rename = "253317")]
    N253317,
    /// Neurologist
    #[serde(rename = "253318")]
    N253318,
    /// Paediatrician
    #[serde(rename = "253321")]
    N253321,
    /// Renal Medicine Specialist
    #[serde(rename = "253322")]
    N253322,
    /// Rheumatologist
    #[serde(rename = "253323")]
    N253323,
    /// Thoracic Medicine Specialist
    #[serde(rename = "253324")]
    N253324,
    /// Internal Medicine Specialists nec
    #[serde(rename = "253399")]
    N253399,
    /// Psychiatrist
    #[serde(rename = "253411")]
    N253411,
    /// Surgeons nfd
    #[serde(rename = "253500")]
    N253500,
    /// Surgeon (General)
    #[serde(rename = "253511")]
    N253511,
    /// Cardiothoracic Surgeon
    #[serde(rename = "253512")]
    N253512,
    /// Neurosurgeon
    #[serde(rename = "253513")]
    N253513,
    /// Orthopaedic Surgeon
    #[serde(rename = "253514")]
    N253514,
    /// Otorhinolaryngologist
    #[serde(rename = "253515")]
    N253515,
    /// Paediatric Surgeon
    #[serde(rename = "253516")]
    N253516,
    /// Plastic and Reconstructive Surgeon
    #[serde(rename = "253517")]
    N253517,
    /// Urologist
    #[serde(rename = "253518")]
    N253518,
    /// Vascular Surgeon
    #[serde(rename = "253521")]
    N253521,
    /// Other Medical Practitioners nfd
    #[serde(rename = "253900")]
    N253900,
    /// Dermatologist
    #[serde(rename = "253911")]
    N253911,
    /// Emergency Medicine Specialist
    #[serde(rename = "253912")]
    N253912,
    /// Obstetrician and Gynaecologist
    #[serde(rename = "253913")]
    N253913,
    /// Ophthalmologist
    #[serde(rename = "253914")]
    N253914,
    /// Pathologist
    #[serde(rename = "253915")]
    N253915,
    /// Radiologist
    #[serde(rename = "253916")]
    N253916,
    /// Medical Practitioners nec
    #[serde(rename = "253999")]
    N253999,
    /// Midwifery and Nursing Professionals nfd
    #[serde(rename = "254000")]
    N254000,
    /// Midwife
    #[serde(rename = "254111")]
    N254111,
    /// Nurse Educators and Researchers nfd
    #[serde(rename = "254200")]
    N254200,
    /// Nurse Educator
    #[serde(rename = "254211")]
    N254211,
    /// Nurse Researcher
    #[serde(rename = "254212")]
    N254212,
    /// Nurse Manager
    #[serde(rename = "254311")]
    N254311,
    /// Registered Nurses nfd
    #[serde(rename = "254400")]
    N254400,
    /// Nurse Practitioner
    #[serde(rename = "254411")]
    N254411,
    /// Registered Nurse (Aged Care)
    #[serde(rename = "254412")]
    N254412,
    /// Registered Nurse (Child and Family Health)
    #[serde(rename = "254413")]
    N254413,
    /// Registered Nurse (Community Health)
    #[serde(rename = "254414")]
    N254414,
    /// Registered Nurse (Critical Care and Emergency)
    #[serde(rename = "254415")]
    N254415,
    /// Registered Nurse (Developmental Disability)
    #[serde(rename = "254416")]
    N254416,
    /// Registered Nurse (Disability and Rehabilitation)
    #[serde(rename = "254417")]
    N254417,
    /// Registered Nurse (Medical)
    #[serde(rename = "254418")]
    N254418,
    /// Registered Nurse (Medical Practice)
    #[serde(rename = "254421")]
    N254421,
    /// Registered Nurse (Mental Health)
    #[serde(rename = "254422")]
    N254422,
    /// Registered Nurse (Perioperative)
    #[serde(rename = "254423")]
    N254423,
    /// Registered Nurse (Surgical)
    #[serde(rename = "254424")]
    N254424,
    /// Registered Nurses nec
    #[serde(rename = "254499")]
    N254499,
    /// ICT Professionals nfd
    #[serde(rename = "260000")]
    N260000,
    /// Business and Systems Analysts, and Programmers nfd
    #[serde(rename = "261000")]
    N261000,
    /// ICT Business and Systems Analysts nfd
    #[serde(rename = "261100")]
    N261100,
    /// ICT Business Analyst
    #[serde(rename = "261111")]
    N261111,
    /// Systems Analyst
    #[serde(rename = "261112")]
    N261112,
    /// Multimedia Specialists and Web Developers nfd
    #[serde(rename = "261200")]
    N261200,
    /// Multimedia Specialist
    #[serde(rename = "261211")]
    N261211,
    /// Web Developer
    #[serde(rename = "261212")]
    N261212,
    /// Software and Applications Programmers nfd
    #[serde(rename = "261300")]
    N261300,
    /// Analyst Programmer
    #[serde(rename = "261311")]
    N261311,
    /// Developer Programmer
    #[serde(rename = "261312")]
    N261312,
    /// Software Engineer
    #[serde(rename = "261313")]
    N261313,
    /// Software and Applications Programmers nec
    #[serde(rename = "261399")]
    N261399,
    /// Database and Systems Administrators, and ICT Security Specialists nfd
    #[serde(rename = "262100")]
    N262100,
    /// Database Administrator
    #[serde(rename = "262111")]
    N262111,
    /// ICT Security Specialist
    #[serde(rename = "262112")]
    N262112,
    /// Systems Administrator
    #[serde(rename = "262113")]
    N262113,
    /// ICT Network and Support Professionals nfd
    #[serde(rename = "263000")]
    N263000,
    /// Computer Network Professionals nfd
    #[serde(rename = "263100")]
    N263100,
    /// Computer Network and Systems Engineer
    #[serde(rename = "263111")]
    N263111,
    /// Network Administrator
    #[serde(rename = "263112")]
    N263112,
    /// Network Analyst
    #[serde(rename = "263113")]
    N263113,
    /// ICT Support and Test Engineers nfd
    #[serde(rename = "263200")]
    N263200,
    /// ICT Quality Assurance Engineer
    #[serde(rename = "263211")]
    N263211,
    /// ICT Support Engineer
    #[serde(rename = "263212")]
    N263212,
    /// ICT Systems Test Engineer
    #[serde(rename = "263213")]
    N263213,
    /// ICT Support and Test Engineers nec
    #[serde(rename = "263299")]
    N263299,
    /// Telecommunications Engineering Professionals nfd
    #[serde(rename = "263300")]
    N263300,
    /// Telecommunications Engineer
    #[serde(rename = "263311")]
    N263311,
    /// Telecommunications Network Engineer
    #[serde(rename = "263312")]
    N263312,
    /// Legal, Social and Welfare Professionals nfd
    #[serde(rename = "270000")]
    N270000,
    /// Legal Professionals nfd
    #[serde(rename = "271000")]
    N271000,
    /// Barrister
    #[serde(rename = "271111")]
    N271111,
    /// Judicial and Other Legal Professionals nfd
    #[serde(rename = "271200")]
    N271200,
    /// Judge
    #[serde(rename = "271211")]
    N271211,
    /// Magistrate
    #[serde(rename = "271212")]
    N271212,
    /// Tribunal Member
    #[serde(rename = "271213")]
    N271213,
    /// Judicial and Other Legal Professionals nec
    #[serde(rename = "271299")]
    N271299,
    /// Solicitor
    #[serde(rename = "271311")]
    N271311,
    /// Social and Welfare Professionals nfd
    #[serde(rename = "272000")]
    N272000,
    /// Counsellors nfd
    #[serde(rename = "272100")]
    N272100,
    /// Careers Counsellor
    #[serde(rename = "272111")]
    N272111,
    /// Drug and Alcohol Counsellor
    #[serde(rename = "272112")]
    N272112,
    /// Family and Marriage Counsellor
    #[serde(rename = "272113")]
    N272113,
    /// Rehabilitation Counsellor
    #[serde(rename = "272114")]
    N272114,
    /// Student Counsellor
    #[serde(rename = "272115")]
    N272115,
    /// Counsellors nec
    #[serde(rename = "272199")]
    N272199,
    /// Minister of Religion
    #[serde(rename = "272211")]
    N272211,
    /// Psychologists nfd
    #[serde(rename = "272300")]
    N272300,
    /// Clinical Psychologist
    #[serde(rename = "272311")]
    N272311,
    /// Educational Psychologist
    #[serde(rename = "272312")]
    N272312,
    /// Organizational Psychologist
    #[serde(rename = "272313")]
    N272313,
    /// Psychotherapist
    #[serde(rename = "272314")]
    N272314,
    /// Psychologists nec
    #[serde(rename = "272399")]
    N272399,
    /// Social Professionals nfd
    #[serde(rename = "272400")]
    N272400,
    /// Historian
    #[serde(rename = "272411")]
    N272411,
    /// Interpreter
    #[serde(rename = "272412")]
    N272412,
    /// Translator
    #[serde(rename = "272413")]
    N272413,
    /// Social Professionals nec
    #[serde(rename = "272499")]
    N272499,
    /// Social Worker
    #[serde(rename = "272511")]
    N272511,
    /// Welfare, Recreation and Community Arts Workers nfd
    #[serde(rename = "272600")]
    N272600,
    /// Community Arts Worker
    #[serde(rename = "272611")]
    N272611,
    /// Recreation Officer
    #[serde(rename = "272612")]
    N272612,
    /// Welfare Worker
    #[serde(rename = "272613")]
    N272613,
    /// Technicians and Trades Workers nfd
    #[serde(rename = "300000")]
    N300000,
    /// Engineering, ICT and Science Technicians nfd
    #[serde(rename = "310000")]
    N310000,
    /// Agricultural, Medical and Science Technicians nfd
    #[serde(rename = "311000")]
    N311000,
    /// Agricultural Technician
    #[serde(rename = "311111")]
    N311111,
    /// Medical Technicians nfd
    #[serde(rename = "311200")]
    N311200,
    /// Anaesthetic Technician
    #[serde(rename = "311211")]
    N311211,
    /// Cardiac Technician
    #[serde(rename = "311212")]
    N311212,
    /// Medical Laboratory Technician
    #[serde(rename = "311213")]
    N311213,
    /// Operating Theatre Technician
    #[serde(rename = "311214")]
    N311214,
    /// Pharmacy Technician
    #[serde(rename = "311215")]
    N311215,
    /// Medical Technicians nec
    #[serde(rename = "311299")]
    N311299,
    /// Primary Products Inspectors nfd
    #[serde(rename = "311300")]
    N311300,
    /// Fisheries Officer
    #[serde(rename = "311311")]
    N311311,
    /// Meat Inspector
    #[serde(rename = "311312")]
    N311312,
    /// Quarantine Officer
    #[serde(rename = "311313")]
    N311313,
    /// Primary Products Inspectors nec
    #[serde(rename = "311399")]
    N311399,
    /// Science Technicians nfd
    #[serde(rename = "311400")]
    N311400,
    /// Chemistry Technician
    #[serde(rename = "311411")]
    N311411,
    /// Earth Science Technician
    #[serde(rename = "311412")]
    N311412,
    /// Life Science Technician
    #[serde(rename = "311413")]
    N311413,
    /// School Laboratory Technician
    #[serde(rename = "311414")]
    N311414,
    /// Science Technicians nec
    #[serde(rename = "311499")]
    N311499,
    /// Building and Engineering Technicians nfd
    #[serde(rename = "312000")]
    N312000,
    /// Architectural, Building and Surveying Technicians nfd
    #[serde(rename = "312100")]
    N312100,
    /// Architectural Draftsperson
    #[serde(rename = "312111")]
    N312111,
    /// Building Associate
    #[serde(rename = "312112")]
    N312112,
    /// Building Inspector
    #[serde(rename = "312113")]
    N312113,
    /// Construction Estimator
    #[serde(rename = "312114")]
    N312114,
    /// Plumbing Inspector
    #[serde(rename = "312115")]
    N312115,
    /// Surveying or Cartographic Technician
    #[serde(rename = "312116")]
    N312116,
    /// Civil Engineering Draftspersons and Technicians nfd
    #[serde(rename = "312200")]
    N312200,
    /// Civil Engineering Draftsperson
    #[serde(rename = "312211")]
    N312211,
    /// Civil Engineering Technician
    #[serde(rename = "312212")]
    N312212,
    /// Electrical Engineering Draftspersons and Technicians nfd
    #[serde(rename = "312300")]
    N312300,
    /// Electrical Engineering Draftsperson
    #[serde(rename = "312311")]
    N312311,
    /// Electrical Engineering Technician
    #[serde(rename = "312312")]
    N312312,
    /// Electronic Engineering Draftspersons and Technicians nfd
    #[serde(rename = "312400")]
    N312400,
    /// Electronic Engineering Draftsperson
    #[serde(rename = "312411")]
    N312411,
    /// Electronic Engineering Technician
    #[serde(rename = "312412")]
    N312412,
    /// Mechanical Engineering Draftspersons and Technicians nfd
    #[serde(rename = "312500")]
    N312500,
    /// Mechanical Engineering Draftsperson
    #[serde(rename = "312511")]
    N312511,
    /// Mechanical Engineering Technician
    #[serde(rename = "312512")]
    N312512,
    /// Safety Inspector
    #[serde(rename = "312611")]
    N312611,
    /// Other Building and Engineering Technicians nfd
    #[serde(rename = "312900")]
    N312900,
    /// Maintenance Planner
    #[serde(rename = "312911")]
    N312911,
    /// Metallurgical or Materials Technician
    #[serde(rename = "312912")]
    N312912,
    /// Mine Deputy
    #[serde(rename = "312913")]
    N312913,
    /// Building and Engineering Technicians nec
    #[serde(rename = "312999")]
    N312999,
    /// ICT and Telecommunications Technicians nfd
    #[serde(rename = "313000")]
    N313000,
    /// ICT Support Technicians nfd
    #[serde(rename = "313100")]
    N313100,
    /// Hardware Technician
    #[serde(rename = "313111")]
    N313111,
    /// ICT Customer Support Officer
    #[serde(rename = "313112")]
    N313112,
    /// Web Administrator
    #[serde(rename = "313113")]
    N313113,
    /// ICT Support Technicians nec
    #[serde(rename = "313199")]
    N313199,
    /// Telecommunications Technical Specialists nfd
    #[serde(rename = "313200")]
    N313200,
    /// Radiocommunications Technician
    #[serde(rename = "313211")]
    N313211,
    /// Telecommunications Field Engineer
    #[serde(rename = "313212")]
    N313212,
    /// Telecommunications Network Planner
    #[serde(rename = "313213")]
    N313213,
    /// Telecommunications Technical Officer or Technologist
    #[serde(rename = "313214")]
    N313214,
    /// Automotive and Engineering Trades Workers nfd
    #[serde(rename = "320000")]
    N320000,
    /// Automotive Electricians and Mechanics nfd
    #[serde(rename = "321000")]
    N321000,
    /// Automotive Electrician
    #[serde(rename = "321111")]
    N321111,
    /// Motor Mechanics nfd
    #[serde(rename = "321200")]
    N321200,
    /// Motor Mechanic (General)
    #[serde(rename = "321211")]
    N321211,
    /// Diesel Motor Mechanic
    #[serde(rename = "321212")]
    N321212,
    /// Motorcycle Mechanic
    #[serde(rename = "321213")]
    N321213,
    /// Small Engine Mechanic
    #[serde(rename = "321214")]
    N321214,
    /// Fabrication Engineering Trades Workers nfd
    #[serde(rename = "322000")]
    N322000,
    /// Metal Casting, Forging and Finishing Trades Workers nfd
    #[serde(rename = "322100")]
    N322100,
    /// Blacksmith
    #[serde(rename = "322111")]
    N322111,
    /// Electroplater
    #[serde(rename = "322112")]
    N322112,
    /// Farrier
    #[serde(rename = "322113")]
    N322113,
    /// Metal Casting Trades Worker
    #[serde(rename = "322114")]
    N322114,
    /// Metal Polisher
    #[serde(rename = "322115")]
    N322115,
    /// Sheetmetal Trades Worker
    #[serde(rename = "322211")]
    N322211,
    /// Structural Steel and Welding Trades Workers nfd
    #[serde(rename = "322300")]
    N322300,
    /// Metal Fabricator
    #[serde(rename = "322311")]
    N322311,
    /// Pressure Welder
    #[serde(rename = "322312")]
    N322312,
    /// Welder (First Class)
    #[serde(rename = "322313")]
    N322313,
    /// Mechanical Engineering Trades Workers nfd
    #[serde(rename = "323000")]
    N323000,
    /// Aircraft Maintenance Engineers nfd
    #[serde(rename = "323100")]
    N323100,
    /// Aircraft Maintenance Engineer (Avionics)
    #[serde(rename = "323111")]
    N323111,
    /// Aircraft Maintenance Engineer (Mechanical)
    #[serde(rename = "323112")]
    N323112,
    /// Aircraft Maintenance Engineer (Structures)
    #[serde(rename = "323113")]
    N323113,
    /// Metal Fitters and Machinists nfd
    #[serde(rename = "323200")]
    N323200,
    /// Fitter (General)
    #[serde(rename = "323211")]
    N323211,
    /// Fitter and Turner
    #[serde(rename = "323212")]
    N323212,
    /// Fitter-Welder
    #[serde(rename = "323213")]
    N323213,
    /// Metal Machinist (First Class)
    #[serde(rename = "323214")]
    N323214,
    /// Textile, Clothing and Footwear Mechanic
    #[serde(rename = "323215")]
    N323215,
    /// Metal Fitters and Machinists nec
    #[serde(rename = "323299")]
    N323299,
    /// Precision Metal Trades Workers nfd
    #[serde(rename = "323300")]
    N323300,
    /// Engraver
    #[serde(rename = "323311")]
    N323311,
    /// Gunsmith
    #[serde(rename = "323312")]
    N323312,
    /// Locksmith
    #[serde(rename = "323313")]
    N323313,
    /// Precision Instrument Maker and Repairer
    #[serde(rename = "323314")]
    N323314,
    /// Saw Maker and Repairer
    #[serde(rename = "323315")]
    N323315,
    /// Watch and Clock Maker and Repairer
    #[serde(rename = "323316")]
    N323316,
    /// Toolmakers and Engineering Patternmakers nfd
    #[serde(rename = "323400")]
    N323400,
    /// Engineering Patternmaker
    #[serde(rename = "323411")]
    N323411,
    /// Toolmaker
    #[serde(rename = "323412")]
    N323412,
    /// Panelbeaters, and Vehicle Body Builders, Trimmers and Painters nfd
    #[serde(rename = "324000")]
    N324000,
    /// Panelbeater
    #[serde(rename = "324111")]
    N324111,
    /// Vehicle Body Builders and Trimmers nfd
    #[serde(rename = "324200")]
    N324200,
    /// Vehicle Body Builder
    #[serde(rename = "324211")]
    N324211,
    /// Vehicle Trimmer
    #[serde(rename = "324212")]
    N324212,
    /// Vehicle Painter
    #[serde(rename = "324311")]
    N324311,
    /// Construction Trades Workers nfd
    #[serde(rename = "330000")]
    N330000,
    /// Bricklayers, and Carpenters and Joiners nfd
    #[serde(rename = "331000")]
    N331000,
    /// Bricklayers and Stonemasons nfd
    #[serde(rename = "331100")]
    N331100,
    /// Bricklayer
    #[serde(rename = "331111")]
    N331111,
    /// Stonemason
    #[serde(rename = "331112")]
    N331112,
    /// Carpenter and Joiner
    #[serde(rename = "331211")]
    N331211,
    /// Carpenter
    #[serde(rename = "331212")]
    N331212,
    /// Joiner
    #[serde(rename = "331213")]
    N331213,
    /// Floor Finishers and Painting Trades Workers nfd
    #[serde(rename = "332000")]
    N332000,
    /// Floor Finisher
    #[serde(rename = "332111")]
    N332111,
    /// Painting Trades Worker
    #[serde(rename = "332211")]
    N332211,
    /// Glaziers, Plasterers and Tilers nfd
    #[serde(rename = "333000")]
    N333000,
    /// Glazier
    #[serde(rename = "333111")]
    N333111,
    /// Plasterers nfd
    #[serde(rename = "333200")]
    N333200,
    /// Fibrous Plasterer
    #[serde(rename = "333211")]
    N333211,
    /// Solid Plasterer
    #[serde(rename = "333212")]
    N333212,
    /// Roof Tiler
    #[serde(rename = "333311")]
    N333311,
    /// Wall and Floor Tiler
    #[serde(rename = "333411")]
    N333411,
    /// Plumbers nfd
    #[serde(rename = "334100")]
    N334100,
    /// Plumber (General)
    #[serde(rename = "334111")]
    N334111,
    /// Airconditioning and Mechanical Services Plumber
    #[serde(rename = "334112")]
    N334112,
    /// Drainer
    #[serde(rename = "334113")]
    N334113,
    /// Gasfitter
    #[serde(rename = "334114")]
    N334114,
    /// Roof Plumber
    #[serde(rename = "334115")]
    N334115,
    /// Electrotechnology and Telecommunications Trades Workers nfd
    #[serde(rename = "340000")]
    N340000,
    /// Electricians nfd
    #[serde(rename = "341100")]
    N341100,
    /// Electrician (General)
    #[serde(rename = "341111")]
    N341111,
    /// Electrician (Special Class)
    #[serde(rename = "341112")]
    N341112,
    /// Lift Mechanic
    #[serde(rename = "341113")]
    N341113,
    /// Electronics and Telecommunications Trades Workers nfd
    #[serde(rename = "342000")]
    N342000,
    /// Airconditioning and Refrigeration Mechanic
    #[serde(rename = "342111")]
    N342111,
    /// Electrical Distribution Trades Workers nfd
    #[serde(rename = "342200")]
    N342200,
    /// Electrical Linesworker
    #[serde(rename = "342211")]
    N342211,
    /// Technical Cable Jointer
    #[serde(rename = "342212")]
    N342212,
    /// Electronics Trades Workers nfd
    #[serde(rename = "342300")]
    N342300,
    /// Business Machine Mechanic
    #[serde(rename = "342311")]
    N342311,
    /// Communications Operator
    #[serde(rename = "342312")]
    N342312,
    /// Electronic Equipment Trades Worker
    #[serde(rename = "342313")]
    N342313,
    /// Electronic Instrument Trades Worker (General)
    #[serde(rename = "342314")]
    N342314,
    /// Electronic Instrument Trades Worker (Special Class)
    #[serde(rename = "342315")]
    N342315,
    /// Telecommunications Trades Workers nfd
    #[serde(rename = "342400")]
    N342400,
    /// Cabler (Data and Telecommunications)
    #[serde(rename = "342411")]
    N342411,
    /// Telecommunications Cable Jointer
    #[serde(rename = "342412")]
    N342412,
    /// Telecommunications Linesworker
    #[serde(rename = "342413")]
    N342413,
    /// Telecommunications Technician
    #[serde(rename = "342414")]
    N342414,
    /// Food Trades Workers nfd
    #[serde(rename = "351000")]
    N351000,
    /// Bakers and Pastrycooks nfd
    #[serde(rename = "351100")]
    N351100,
    /// Baker
    #[serde(rename = "351111")]
    N351111,
    /// Pastrycook
    #[serde(rename = "351112")]
    N351112,
    /// Butcher or Smallgoods Maker
    #[serde(rename = "351211")]
    N351211,
    /// Chef
    #[serde(rename = "351311")]
    N351311,
    /// Cook
    #[serde(rename = "351411")]
    N351411,
    /// Skilled Animal and Horticultural Workers nfd
    #[serde(rename = "360000")]
    N360000,
    /// Animal Attendants and Trainers, and Shearers nfd
    #[serde(rename = "361000")]
    N361000,
    /// Animal Attendants and Trainers nfd
    #[serde(rename = "361100")]
    N361100,
    /// Dog Handler or Trainer
    #[serde(rename = "361111")]
    N361111,
    /// Horse Trainer
    #[serde(rename = "361112")]
    N361112,
    /// Pet Groomer
    #[serde(rename = "361113")]
    N361113,
    /// Zookeeper
    #[serde(rename = "361114")]
    N361114,
    /// Animal Attendants and Trainers nec
    #[serde(rename = "361199")]
    N361199,
    /// Shearer
    #[serde(rename = "361211")]
    N361211,
    /// Veterinary Nurse
    #[serde(rename = "361311")]
    N361311,
    /// Horticultural Trades Workers nfd
    #[serde(rename = "362000")]
    N362000,
    /// Florist
    #[serde(rename = "362111")]
    N362111,
    /// Gardeners nfd
    #[serde(rename = "362200")]
    N362200,
    /// Gardener (General)
    #[serde(rename = "362211")]
    N362211,
    /// Arborist
    #[serde(rename = "362212")]
    N362212,
    /// Landscape Gardener
    #[serde(rename = "362213")]
    N362213,
    /// Greenkeeper
    #[serde(rename = "362311")]
    N362311,
    /// Nurseryperson
    #[serde(rename = "362411")]
    N362411,
    /// Other Technicians and Trades Workers nfd
    #[serde(rename = "390000")]
    N390000,
    /// Hairdresser
    #[serde(rename = "391111")]
    N391111,
    /// Printing Trades Workers nfd
    #[serde(rename = "392000")]
    N392000,
    /// Binders, Finishers and Screen Printers nfd
    #[serde(rename = "392100")]
    N392100,
    /// Binder and Finisher
    #[serde(rename = "392111")]
    N392111,
    /// Screen Printer
    #[serde(rename = "392112")]
    N392112,
    /// Graphic Pre-press Trades Worker
    #[serde(rename = "392211")]
    N392211,
    /// Printers nfd
    #[serde(rename = "392300")]
    N392300,
    /// Printing Machinist
    #[serde(rename = "392311")]
    N392311,
    /// Small Offset Printer
    #[serde(rename = "392312")]
    N392312,
    /// Textile, Clothing and Footwear Trades Workers nfd
    #[serde(rename = "393000")]
    N393000,
    /// Canvas and Leather Goods Makers nfd
    #[serde(rename = "393100")]
    N393100,
    /// Canvas Goods Maker
    #[serde(rename = "393111")]
    N393111,
    /// Leather Goods Maker
    #[serde(rename = "393112")]
    N393112,
    /// Sail Maker
    #[serde(rename = "393113")]
    N393113,
    /// Shoemaker
    #[serde(rename = "393114")]
    N393114,
    /// Clothing Trades Workers nfd
    #[serde(rename = "393200")]
    N393200,
    /// Apparel Cutter
    #[serde(rename = "393211")]
    N393211,
    /// Clothing Patternmaker
    #[serde(rename = "393212")]
    N393212,
    /// Dressmaker or Tailor
    #[serde(rename = "393213")]
    N393213,
    /// Clothing Trades Workers nec
    #[serde(rename = "393299")]
    N393299,
    /// Upholsterer
    #[serde(rename = "393311")]
    N393311,
    /// Wood Trades Workers nfd
    #[serde(rename = "394000")]
    N394000,
    /// Cabinetmaker
    #[serde(rename = "394111")]
    N394111,
    /// Wood Machinists and Other Wood Trades Workers nfd
    #[serde(rename = "394200")]
    N394200,
    /// Furniture Finisher
    #[serde(rename = "394211")]
    N394211,
    /// Picture Framer
    #[serde(rename = "394212")]
    N394212,
    /// Wood Machinist
    #[serde(rename = "394213")]
    N394213,
    /// Wood Turner
    #[serde(rename = "394214")]
    N394214,
    /// Wood Machinists and Other Wood Trades Workers nec
    #[serde(rename = "394299")]
    N394299,
    /// Miscellaneous Technicians and Trades Workers nfd
    #[serde(rename = "399000")]
    N399000,
    /// Boat Builders and Shipwrights nfd
    #[serde(rename = "399100")]
    N399100,
    /// Boat Builder and Repairer
    #[serde(rename = "399111")]
    N399111,
    /// Shipwright
    #[serde(rename = "399112")]
    N399112,
    /// Chemical, Gas, Petroleum and Power Generation Plant Operators nfd
    #[serde(rename = "399200")]
    N399200,
    /// Chemical Plant Operator
    #[serde(rename = "399211")]
    N399211,
    /// Gas or Petroleum Operator
    #[serde(rename = "399212")]
    N399212,
    /// Power Generation Plant Operator
    #[serde(rename = "399213")]
    N399213,
    /// Gallery, Library and Museum Technicians nfd
    #[serde(rename = "399300")]
    N399300,
    /// Gallery or Museum Technician
    #[serde(rename = "399311")]
    N399311,
    /// Library Technician
    #[serde(rename = "399312")]
    N399312,
    /// Jeweller
    #[serde(rename = "399411")]
    N399411,
    /// Performing Arts Technicians nfd
    #[serde(rename = "399500")]
    N399500,
    /// Broadcast Transmitter Operator
    #[serde(rename = "399511")]
    N399511,
    /// Camera Operator (Film, Television or Video)
    #[serde(rename = "399512")]
    N399512,
    /// Light Technician
    #[serde(rename = "399513")]
    N399513,
    /// Make Up Artist
    #[serde(rename = "399514")]
    N399514,
    /// Musical Instrument Maker or Repairer
    #[serde(rename = "399515")]
    N399515,
    /// Sound Technician
    #[serde(rename = "399516")]
    N399516,
    /// Television Equipment Operator
    #[serde(rename = "399517")]
    N399517,
    /// Performing Arts Technicians nec
    #[serde(rename = "399599")]
    N399599,
    /// Signwriter
    #[serde(rename = "399611")]
    N399611,
    /// Other Miscellaneous Technicians and Trades Workers nfd
    #[serde(rename = "399900")]
    N399900,
    /// Diver
    #[serde(rename = "399911")]
    N399911,
    /// Interior Decorator
    #[serde(rename = "399912")]
    N399912,
    /// Optical Dispenser
    #[serde(rename = "399913")]
    N399913,
    /// Optical Mechanic
    #[serde(rename = "399914")]
    N399914,
    /// Photographer's Assistant
    #[serde(rename = "399915")]
    N399915,
    /// Plastics Technician
    #[serde(rename = "399916")]
    N399916,
    /// Wool Classer
    #[serde(rename = "399917")]
    N399917,
    /// Technicians and Trades Workers nec
    #[serde(rename = "399999")]
    N399999,
    /// Community and Personal Service Workers nfd
    #[serde(rename = "400000")]
    N400000,
    /// Health and Welfare Support Workers nfd
    #[serde(rename = "411000")]
    N411000,
    /// Ambulance Officers and Paramedics nfd
    #[serde(rename = "411100")]
    N411100,
    /// Ambulance Officer
    #[serde(rename = "411111")]
    N411111,
    /// Intensive Care Ambulance Paramedic
    #[serde(rename = "411112")]
    N411112,
    /// Dental Hygienists, Technicians and Therapists nfd
    #[serde(rename = "411200")]
    N411200,
    /// Dental Hygienist
    #[serde(rename = "411211")]
    N411211,
    /// Dental Prosthetist
    #[serde(rename = "411212")]
    N411212,
    /// Dental Technician
    #[serde(rename = "411213")]
    N411213,
    /// Dental Therapist
    #[serde(rename = "411214")]
    N411214,
    /// Diversional Therapist
    #[serde(rename = "411311")]
    N411311,
    /// Enrolled and Mothercraft Nurses nfd
    #[serde(rename = "411400")]
    N411400,
    /// Enrolled Nurse
    #[serde(rename = "411411")]
    N411411,
    /// Mothercraft Nurse
    #[serde(rename = "411412")]
    N411412,
    /// Aboriginal and Torres Strait Islander Health Worker
    #[serde(rename = "411511")]
    N411511,
    /// Massage Therapist
    #[serde(rename = "411611")]
    N411611,
    /// Welfare Support Workers nfd
    #[serde(rename = "411700")]
    N411700,
    /// Community Worker
    #[serde(rename = "411711")]
    N411711,
    /// Disabilities Services Officer
    #[serde(rename = "411712")]
    N411712,
    /// Family Support Worker
    #[serde(rename = "411713")]
    N411713,
    /// Parole or Probation Officer
    #[serde(rename = "411714")]
    N411714,
    /// Residential Care Officer
    #[serde(rename = "411715")]
    N411715,
    /// Youth Worker
    #[serde(rename = "411716")]
    N411716,
    /// Carers and Aides nfd
    #[serde(rename = "420000")]
    N420000,
    /// Child Carers nfd
    #[serde(rename = "421100")]
    N421100,
    /// Child Care Worker
    #[serde(rename = "421111")]
    N421111,
    /// Family Day Care Worker
    #[serde(rename = "421112")]
    N421112,
    /// Nanny
    #[serde(rename = "421113")]
    N421113,
    /// Out of School Hours Care Worker
    #[serde(rename = "421114")]
    N421114,
    /// Education Aides nfd
    #[serde(rename = "422100")]
    N422100,
    /// Aboriginal and Torres Strait Islander Education Worker
    #[serde(rename = "422111")]
    N422111,
    /// Integration Aide
    #[serde(rename = "422112")]
    N422112,
    /// Preschool Aide
    #[serde(rename = "422115")]
    N422115,
    /// Teachers' Aide
    #[serde(rename = "422116")]
    N422116,
    /// Personal Carers and Assistants nfd
    #[serde(rename = "423000")]
    N423000,
    /// Aged or Disabled Carer
    #[serde(rename = "423111")]
    N423111,
    /// Dental Assistant
    #[serde(rename = "423211")]
    N423211,
    /// Nursing Support and Personal Care Workers nfd
    #[serde(rename = "423300")]
    N423300,
    /// Hospital Orderly
    #[serde(rename = "423311")]
    N423311,
    /// Nursing Support Worker
    #[serde(rename = "423312")]
    N423312,
    /// Personal Care Assistant
    #[serde(rename = "423313")]
    N423313,
    /// Therapy Aide
    #[serde(rename = "423314")]
    N423314,
    /// Special Care Workers nfd
    #[serde(rename = "423400")]
    N423400,
    /// Child or Youth Residential Care Assistant
    #[serde(rename = "423411")]
    N423411,
    /// Hostel Parent
    #[serde(rename = "423412")]
    N423412,
    /// Refuge Worker
    #[serde(rename = "423413")]
    N423413,
    /// Hospitality Workers nfd
    #[serde(rename = "431000")]
    N431000,
    /// Bar Attendants and Baristas nfd
    #[serde(rename = "431100")]
    N431100,
    /// Bar Attendant
    #[serde(rename = "431111")]
    N431111,
    /// Barista
    #[serde(rename = "431112")]
    N431112,
    /// Cafe Worker
    #[serde(rename = "431211")]
    N431211,
    /// Gaming Worker
    #[serde(rename = "431311")]
    N431311,
    /// Hotel Service Manager
    #[serde(rename = "431411")]
    N431411,
    /// Waiter
    #[serde(rename = "431511")]
    N431511,
    /// Other Hospitality Workers nfd
    #[serde(rename = "431900")]
    N431900,
    /// Bar Useful or Busser
    #[serde(rename = "431911")]
    N431911,
    /// Doorperson or Luggage Porter
    #[serde(rename = "431912")]
    N431912,
    /// Hospitality Workers nec
    #[serde(rename = "431999")]
    N431999,
    /// Protective Service Workers nfd
    #[serde(rename = "440000")]
    N440000,
    /// Defence Force Members, Fire Fighters and Police nfd
    #[serde(rename = "441000")]
    N441000,
    /// Defence Force Member - Other Ranks
    #[serde(rename = "441111")]
    N441111,
    /// Fire and Emergency Workers nfd
    #[serde(rename = "441200")]
    N441200,
    /// Emergency Service Worker
    #[serde(rename = "441211")]
    N441211,
    /// Fire Fighter
    #[serde(rename = "441212")]
    N441212,
    /// Police nfd
    #[serde(rename = "441300")]
    N441300,
    /// Detective
    #[serde(rename = "441311")]
    N441311,
    /// Police Officer
    #[serde(rename = "441312")]
    N441312,
    /// Prison and Security Officers nfd
    #[serde(rename = "442000")]
    N442000,
    /// Prison Officer
    #[serde(rename = "442111")]
    N442111,
    /// Security Officers and Guards nfd
    #[serde(rename = "442200")]
    N442200,
    /// Alarm, Security or Surveillance Monitor
    #[serde(rename = "442211")]
    N442211,
    /// Armoured Car Escort
    #[serde(rename = "442212")]
    N442212,
    /// Crowd Controller
    #[serde(rename = "442213")]
    N442213,
    /// Private Investigator
    #[serde(rename = "442214")]
    N442214,
    /// Retail Loss Prevention Officer
    #[serde(rename = "442215")]
    N442215,
    /// Security Consultant
    #[serde(rename = "442216")]
    N442216,
    /// Security Officer
    #[serde(rename = "442217")]
    N442217,
    /// Security Officers and Guards nec
    #[serde(rename = "442299")]
    N442299,
    /// Sports and Personal Service Workers nfd
    #[serde(rename = "450000")]
    N450000,
    /// Personal Service and Travel Workers nfd
    #[serde(rename = "451000")]
    N451000,
    /// Beauty Therapist
    #[serde(rename = "451111")]
    N451111,
    /// Driving Instructor
    #[serde(rename = "451211")]
    N451211,
    /// Funeral Workers nfd
    #[serde(rename = "451300")]
    N451300,
    /// Funeral Director
    #[serde(rename = "451311")]
    N451311,
    /// Funeral Workers nec
    #[serde(rename = "451399")]
    N451399,
    /// Gallery, Museum and Tour Guides nfd
    #[serde(rename = "451400")]
    N451400,
    /// Gallery or Museum Guide
    #[serde(rename = "451411")]
    N451411,
    /// Tour Guide
    #[serde(rename = "451412")]
    N451412,
    /// Personal Care Consultants nfd
    #[serde(rename = "451500")]
    N451500,
    /// Natural Remedy Consultant
    #[serde(rename = "451511")]
    N451511,
    /// Weight Loss Consultant
    #[serde(rename = "451512")]
    N451512,
    /// Tourism and Travel Advisers nfd
    #[serde(rename = "451600")]
    N451600,
    /// Tourist Information Officer
    #[serde(rename = "451611")]
    N451611,
    /// Travel Consultant
    #[serde(rename = "451612")]
    N451612,
    /// Travel Attendants nfd
    #[serde(rename = "451700")]
    N451700,
    /// Flight Attendant
    #[serde(rename = "451711")]
    N451711,
    /// Travel Attendants nec
    #[serde(rename = "451799")]
    N451799,
    /// Other Personal Service Workers nfd
    #[serde(rename = "451800")]
    N451800,
    /// Civil Celebrant
    #[serde(rename = "451811")]
    N451811,
    /// Hair or Beauty Salon Assistant
    #[serde(rename = "451812")]
    N451812,
    /// Sex Worker or Escort
    #[serde(rename = "451813")]
    N451813,
    /// Personal Service Workers nec
    #[serde(rename = "451899")]
    N451899,
    /// Sports and Fitness Workers nfd
    #[serde(rename = "452000")]
    N452000,
    /// Fitness Instructor
    #[serde(rename = "452111")]
    N452111,
    /// Outdoor Adventure Guides nfd
    #[serde(rename = "452200")]
    N452200,
    /// Bungy Jump Master
    #[serde(rename = "452211")]
    N452211,
    /// Fishing Guide
    #[serde(rename = "452212")]
    N452212,
    /// Hunting Guide
    #[serde(rename = "452213")]
    N452213,
    /// Mountain or Glacier Guide
    #[serde(rename = "452214")]
    N452214,
    /// Outdoor Adventure Instructor
    #[serde(rename = "452215")]
    N452215,
    /// Trekking Guide
    #[serde(rename = "452216")]
    N452216,
    /// Whitewater Rafting Guide
    #[serde(rename = "452217")]
    N452217,
    /// Outdoor Adventure Guides nec
    #[serde(rename = "452299")]
    N452299,
    /// Sports Coaches, Instructors and Officials nfd
    #[serde(rename = "452300")]
    N452300,
    /// Diving Instructor (Open Water)
    #[serde(rename = "452311")]
    N452311,
    /// Gymnastics Coach or Instructor
    #[serde(rename = "452312")]
    N452312,
    /// Horse Riding Coach or Instructor
    #[serde(rename = "452313")]
    N452313,
    /// Snowsport Instructor
    #[serde(rename = "452314")]
    N452314,
    /// Swimming Coach or Instructor
    #[serde(rename = "452315")]
    N452315,
    /// Tennis Coach
    #[serde(rename = "452316")]
    N452316,
    /// Other Sports Coach or Instructor
    #[serde(rename = "452317")]
    N452317,
    /// Dog or Horse Racing Official
    #[serde(rename = "452318")]
    N452318,
    /// Sports Development Officer
    #[serde(rename = "452321")]
    N452321,
    /// Sports Umpire
    #[serde(rename = "452322")]
    N452322,
    /// Other Sports Official
    #[serde(rename = "452323")]
    N452323,
    /// Sportspersons nfd
    #[serde(rename = "452400")]
    N452400,
    /// Footballer
    #[serde(rename = "452411")]
    N452411,
    /// Golfer
    #[serde(rename = "452412")]
    N452412,
    /// Jockey
    #[serde(rename = "452413")]
    N452413,
    /// Lifeguard
    #[serde(rename = "452414")]
    N452414,
    /// Sportspersons nec
    #[serde(rename = "452499")]
    N452499,
    /// Clerical and Administrative Workers nfd
    #[serde(rename = "500000")]
    N500000,
    /// Office Managers and Program Administrators nfd
    #[serde(rename = "510000")]
    N510000,
    /// Contract, Program and Project Administrators nfd
    #[serde(rename = "511100")]
    N511100,
    /// Contract Administrator
    #[serde(rename = "511111")]
    N511111,
    /// Program or Project Administrator
    #[serde(rename = "511112")]
    N511112,
    /// Office and Practice Managers nfd
    #[serde(rename = "512000")]
    N512000,
    /// Office Manager
    #[serde(rename = "512111")]
    N512111,
    /// Practice Managers nfd
    #[serde(rename = "512200")]
    N512200,
    /// Health Practice Manager
    #[serde(rename = "512211")]
    N512211,
    /// Practice Managers nec
    #[serde(rename = "512299")]
    N512299,
    /// Personal Assistants and Secretaries nfd
    #[serde(rename = "521000")]
    N521000,
    /// Personal Assistant
    #[serde(rename = "521111")]
    N521111,
    /// Secretaries nfd
    #[serde(rename = "521200")]
    N521200,
    /// Secretary (General)
    #[serde(rename = "521211")]
    N521211,
    /// Legal Secretary
    #[serde(rename = "521212")]
    N521212,
    /// General Clerical Workers nfd
    #[serde(rename = "530000")]
    N530000,
    /// General Clerk
    #[serde(rename = "531111")]
    N531111,
    /// Keyboard Operators nfd
    #[serde(rename = "532100")]
    N532100,
    /// Data Entry Operator
    #[serde(rename = "532111")]
    N532111,
    /// Machine Shorthand Reporter
    #[serde(rename = "532112")]
    N532112,
    /// Word Processing Operator
    #[serde(rename = "532113")]
    N532113,
    /// Inquiry Clerks and Receptionists nfd
    #[serde(rename = "540000")]
    N540000,
    /// Call or Contact Centre Information Clerks nfd
    #[serde(rename = "541000")]
    N541000,
    /// Call or Contact Centre Workers nfd
    #[serde(rename = "541100")]
    N541100,
    /// Call or Contact Centre Team Leader
    #[serde(rename = "541111")]
    N541111,
    /// Call or Contact Centre Operator
    #[serde(rename = "541112")]
    N541112,
    /// Inquiry Clerk
    #[serde(rename = "541211")]
    N541211,
    /// Receptionists nfd
    #[serde(rename = "542100")]
    N542100,
    /// Receptionist (General)
    #[serde(rename = "542111")]
    N542111,
    /// Admissions Clerk
    #[serde(rename = "542112")]
    N542112,
    /// Hotel or Motel Receptionist
    #[serde(rename = "542113")]
    N542113,
    /// Medical Receptionist
    #[serde(rename = "542114")]
    N542114,
    /// Numerical Clerks nfd
    #[serde(rename = "550000")]
    N550000,
    /// Accounting Clerks and Bookkeepers nfd
    #[serde(rename = "551000")]
    N551000,
    /// Accounting Clerks nfd
    #[serde(rename = "551100")]
    N551100,
    /// Accounts Clerk
    #[serde(rename = "551111")]
    N551111,
    /// Cost Clerk
    #[serde(rename = "551112")]
    N551112,
    /// Bookkeeper
    #[serde(rename = "551211")]
    N551211,
    /// Payroll Clerk
    #[serde(rename = "551311")]
    N551311,
    /// Financial and Insurance Clerks nfd
    #[serde(rename = "552000")]
    N552000,
    /// Bank Worker
    #[serde(rename = "552111")]
    N552111,
    /// Credit or Loans Officer
    #[serde(rename = "552211")]
    N552211,
    /// Insurance, Money Market and Statistical Clerks nfd
    #[serde(rename = "552300")]
    N552300,
    /// Bookmaker
    #[serde(rename = "552311")]
    N552311,
    /// Insurance Consultant
    #[serde(rename = "552312")]
    N552312,
    /// Money Market Clerk
    #[serde(rename = "552313")]
    N552313,
    /// Statistical Clerk
    #[serde(rename = "552314")]
    N552314,
    /// Clerical and Office Support Workers nfd
    #[serde(rename = "561000")]
    N561000,
    /// Betting Clerks nfd
    #[serde(rename = "561100")]
    N561100,
    /// Betting Agency Counter Clerk
    #[serde(rename = "561111")]
    N561111,
    /// Bookmaker's Clerk
    #[serde(rename = "561112")]
    N561112,
    /// Telephone Betting Clerk
    #[serde(rename = "561113")]
    N561113,
    /// Betting Clerks nec
    #[serde(rename = "561199")]
    N561199,
    /// Couriers and Postal Deliverers nfd
    #[serde(rename = "561200")]
    N561200,
    /// Courier
    #[serde(rename = "561211")]
    N561211,
    /// Postal Delivery Officer
    #[serde(rename = "561212")]
    N561212,
    /// Filing or Registry Clerk
    #[serde(rename = "561311")]
    N561311,
    /// Mail Sorters nfd
    #[serde(rename = "561400")]
    N561400,
    /// Mail Clerk
    #[serde(rename = "561411")]
    N561411,
    /// Postal Sorting Officer
    #[serde(rename = "561412")]
    N561412,
    /// Survey Interviewer
    #[serde(rename = "561511")]
    N561511,
    /// Switchboard Operator
    #[serde(rename = "561611")]
    N561611,
    /// Other Clerical and Office Support Workers nfd
    #[serde(rename = "561900")]
    N561900,
    /// Classified Advertising Clerk
    #[serde(rename = "561911")]
    N561911,
    /// Meter Reader
    #[serde(rename = "561912")]
    N561912,
    /// Parking Inspector
    #[serde(rename = "561913")]
    N561913,
    /// Clerical and Office Support Workers nec
    #[serde(rename = "561999")]
    N561999,
    /// Other Clerical and Administrative Workers nfd
    #[serde(rename = "590000")]
    N590000,
    /// Logistics Clerks nfd
    #[serde(rename = "591000")]
    N591000,
    /// Purchasing and Supply Logistics Clerks nfd
    #[serde(rename = "591100")]
    N591100,
    /// Order Clerk
    #[serde(rename = "591111")]
    N591111,
    /// Production Clerk
    #[serde(rename = "591112")]
    N591112,
    /// Purchasing Officer
    #[serde(rename = "591113")]
    N591113,
    /// Sales Clerk
    #[serde(rename = "591114")]
    N591114,
    /// Stock Clerk
    #[serde(rename = "591115")]
    N591115,
    /// Warehouse Administrator
    #[serde(rename = "591116")]
    N591116,
    /// Transport and Despatch Clerks nfd
    #[serde(rename = "591200")]
    N591200,
    /// Despatching and Receiving Clerk
    #[serde(rename = "591211")]
    N591211,
    /// Import-Export Clerk
    #[serde(rename = "591212")]
    N591212,
    /// Miscellaneous Clerical and Administrative Workers nfd
    #[serde(rename = "599000")]
    N599000,
    /// Conveyancers and Legal Executives nfd
    #[serde(rename = "599100")]
    N599100,
    /// Conveyancer
    #[serde(rename = "599111")]
    N599111,
    /// Legal Executive
    #[serde(rename = "599112")]
    N599112,
    /// Court and Legal Clerks nfd
    #[serde(rename = "599200")]
    N599200,
    /// Clerk of Court
    #[serde(rename = "599211")]
    N599211,
    /// Court Bailiff or Sheriff
    #[serde(rename = "599212")]
    N599212,
    /// Court Orderly
    #[serde(rename = "599213")]
    N599213,
    /// Law Clerk
    #[serde(rename = "599214")]
    N599214,
    /// Trust Officer
    #[serde(rename = "599215")]
    N599215,
    /// Debt Collector
    #[serde(rename = "599311")]
    N599311,
    /// Human Resource Clerk
    #[serde(rename = "599411")]
    N599411,
    /// Inspectors and Regulatory Officers nfd
    #[serde(rename = "599500")]
    N599500,
    /// Customs Officer
    #[serde(rename = "599511")]
    N599511,
    /// Immigration Officer
    #[serde(rename = "599512")]
    N599512,
    /// Motor Vehicle Licence Examiner
    #[serde(rename = "599513")]
    N599513,
    /// Noxious Weeds and Pest Inspector
    #[serde(rename = "599514")]
    N599514,
    /// Social Security Assessor
    #[serde(rename = "599515")]
    N599515,
    /// Taxation Inspector
    #[serde(rename = "599516")]
    N599516,
    /// Train Examiner
    #[serde(rename = "599517")]
    N599517,
    /// Transport Operations Inspector
    #[serde(rename = "599518")]
    N599518,
    /// Water Inspector
    #[serde(rename = "599521")]
    N599521,
    /// Inspectors and Regulatory Officers nec
    #[serde(rename = "599599")]
    N599599,
    /// Insurance Investigators, Loss Adjusters and Risk Surveyors nfd
    #[serde(rename = "599600")]
    N599600,
    /// Insurance Investigator
    #[serde(rename = "599611")]
    N599611,
    /// Insurance Loss Adjuster
    #[serde(rename = "599612")]
    N599612,
    /// Insurance Risk Surveyor
    #[serde(rename = "599613")]
    N599613,
    /// Library Assistant
    #[serde(rename = "599711")]
    N599711,
    /// Other Miscellaneous Clerical and Administrative Workers nfd
    #[serde(rename = "599900")]
    N599900,
    /// Coding Clerk
    #[serde(rename = "599911")]
    N599911,
    /// Production Assistant (Film, Television, Radio or Stage)
    #[serde(rename = "599912")]
    N599912,
    /// Proof Reader
    #[serde(rename = "599913")]
    N599913,
    /// Radio Despatcher
    #[serde(rename = "599914")]
    N599914,
    /// Clerical and Administrative Workers nec
    #[serde(rename = "599999")]
    N599999,
    /// Sales Workers nfd
    #[serde(rename = "600000")]
    N600000,
    /// Sales Representatives and Agents nfd
    #[serde(rename = "610000")]
    N610000,
    /// Insurance Agents and Sales Representatives nfd
    #[serde(rename = "611000")]
    N611000,
    /// Auctioneers, and Stock and Station Agents nfd
    #[serde(rename = "611100")]
    N611100,
    /// Auctioneer
    #[serde(rename = "611111")]
    N611111,
    /// Stock and Station Agent
    #[serde(rename = "611112")]
    N611112,
    /// Insurance Agent
    #[serde(rename = "611211")]
    N611211,
    /// Sales Representatives nfd
    #[serde(rename = "611300")]
    N611300,
    /// Sales Representative (Building and Plumbing Supplies)
    #[serde(rename = "611311")]
    N611311,
    /// Sales Representative (Business Services)
    #[serde(rename = "611312")]
    N611312,
    /// Sales Representative (Motor Vehicle Parts and Accessories)
    #[serde(rename = "611313")]
    N611313,
    /// Sales Representative (Personal and Household Goods)
    #[serde(rename = "611314")]
    N611314,
    /// Sales Representatives nec
    #[serde(rename = "611399")]
    N611399,
    /// Real Estate Sales Agents nfd
    #[serde(rename = "612100")]
    N612100,
    /// Business Broker
    #[serde(rename = "612111")]
    N612111,
    /// Property Manager
    #[serde(rename = "612112")]
    N612112,
    /// Real Estate Agency Principal
    #[serde(rename = "612113")]
    N612113,
    /// Real Estate Agent
    #[serde(rename = "612114")]
    N612114,
    /// Real Estate Representative
    #[serde(rename = "612115")]
    N612115,
    /// Sales Assistants and Salespersons nfd
    #[serde(rename = "621000")]
    N621000,
    /// Sales Assistant (General)
    #[serde(rename = "621111")]
    N621111,
    /// ICT Sales Assistant
    #[serde(rename = "621211")]
    N621211,
    /// Motor Vehicle and Vehicle Parts Salespersons nfd
    #[serde(rename = "621300")]
    N621300,
    /// Motor Vehicle or Caravan Salesperson
    #[serde(rename = "621311")]
    N621311,
    /// Motor Vehicle Parts Interpreter
    #[serde(rename = "621312")]
    N621312,
    /// Pharmacy Sales Assistant
    #[serde(rename = "621411")]
    N621411,
    /// Retail Supervisor
    #[serde(rename = "621511")]
    N621511,
    /// Service Station Attendant
    #[serde(rename = "621611")]
    N621611,
    /// Street Vendors and Related Salespersons nfd
    #[serde(rename = "621700")]
    N621700,
    /// Cash Van Salesperson
    #[serde(rename = "621711")]
    N621711,
    /// Door-to-door Salesperson
    #[serde(rename = "621712")]
    N621712,
    /// Street Vendor
    #[serde(rename = "621713")]
    N621713,
    /// Other Sales Assistants and Salespersons nfd
    #[serde(rename = "621900")]
    N621900,
    /// Materials Recycler
    #[serde(rename = "621911")]
    N621911,
    /// Rental Salesperson
    #[serde(rename = "621912")]
    N621912,
    /// Sales Assistants and Salespersons nec
    #[serde(rename = "621999")]
    N621999,
    /// Sales Support Workers nfd
    #[serde(rename = "630000")]
    N630000,
    /// Checkout Operators and Office Cashiers nfd
    #[serde(rename = "631100")]
    N631100,
    /// Checkout Operator
    #[serde(rename = "631111")]
    N631111,
    /// Office Cashier
    #[serde(rename = "631112")]
    N631112,
    /// Miscellaneous Sales Support Workers nfd
    #[serde(rename = "639000")]
    N639000,
    /// Models and Sales Demonstrators nfd
    #[serde(rename = "639100")]
    N639100,
    /// Model
    #[serde(rename = "639111")]
    N639111,
    /// Sales Demonstrator
    #[serde(rename = "639112")]
    N639112,
    /// Retail and Wool Buyers nfd
    #[serde(rename = "639200")]
    N639200,
    /// Retail Buyer
    #[serde(rename = "639211")]
    N639211,
    /// Wool Buyer
    #[serde(rename = "639212")]
    N639212,
    /// Telemarketer
    #[serde(rename = "639311")]
    N639311,
    /// Ticket Salespersons nfd
    #[serde(rename = "639400")]
    N639400,
    /// Ticket Seller
    #[serde(rename = "639411")]
    N639411,
    /// Transport Conductor
    #[serde(rename = "639412")]
    N639412,
    /// Visual Merchandiser
    #[serde(rename = "639511")]
    N639511,
    /// Other Sales Support Worker
    #[serde(rename = "639911")]
    N639911,
    /// Machinery Operators and Drivers nfd
    #[serde(rename = "700000")]
    N700000,
    /// Machine and Stationary Plant Operators nfd
    #[serde(rename = "710000")]
    N710000,
    /// Machine Operators nfd
    #[serde(rename = "711000")]
    N711000,
    /// Clay, Concrete, Glass and Stone Processing Machine Operators nfd
    #[serde(rename = "711100")]
    N711100,
    /// Clay Products Machine Operator
    #[serde(rename = "711111")]
    N711111,
    /// Concrete Products Machine Operator
    #[serde(rename = "711112")]
    N711112,
    /// Glass Production Machine Operator
    #[serde(rename = "711113")]
    N711113,
    /// Stone Processing Machine Operator
    #[serde(rename = "711114")]
    N711114,
    /// Clay, Concrete, Glass and Stone Processing Machine Operators nec
    #[serde(rename = "711199")]
    N711199,
    /// Industrial Spraypainter
    #[serde(rename = "711211")]
    N711211,
    /// Paper and Wood Processing Machine Operators nfd
    #[serde(rename = "711300")]
    N711300,
    /// Paper Products Machine Operator
    #[serde(rename = "711311")]
    N711311,
    /// Wood Processing Machine Operator
    #[serde(rename = "711312")]
    N711312,
    /// Photographic Developer and Printer
    #[serde(rename = "711411")]
    N711411,
    /// Plastics and Rubber Production Machine Operators nfd
    #[serde(rename = "711500")]
    N711500,
    /// Plastic Cablemaking Machine Operator
    #[serde(rename = "711511")]
    N711511,
    /// Plastic Compounding and Reclamation Machine Operator
    #[serde(rename = "711512")]
    N711512,
    /// Plastics Fabricator or Welder
    #[serde(rename = "711513")]
    N711513,
    /// Plastics Production Machine Operator (General)
    #[serde(rename = "711514")]
    N711514,
    /// Reinforced Plastic and Composite Production Worker
    #[serde(rename = "711515")]
    N711515,
    /// Rubber Production Machine Operator
    #[serde(rename = "711516")]
    N711516,
    /// Plastics and Rubber Production Machine Operators nec
    #[serde(rename = "711599")]
    N711599,
    /// Sewing Machinist
    #[serde(rename = "711611")]
    N711611,
    /// Textile and Footwear Production Machine Operators nfd
    #[serde(rename = "711700")]
    N711700,
    /// Footwear Production Machine Operator
    #[serde(rename = "711711")]
    N711711,
    /// Hide and Skin Processing Machine Operator
    #[serde(rename = "711712")]
    N711712,
    /// Knitting Machine Operator
    #[serde(rename = "711713")]
    N711713,
    /// Textile Dyeing and Finishing Machine Operator
    #[serde(rename = "711714")]
    N711714,
    /// Weaving Machine Operator
    #[serde(rename = "711715")]
    N711715,
    /// Yarn Carding and Spinning Machine Operator
    #[serde(rename = "711716")]
    N711716,
    /// Textile and Footwear Production Machine Operators nec
    #[serde(rename = "711799")]
    N711799,
    /// Other Machine Operators nfd
    #[serde(rename = "711900")]
    N711900,
    /// Chemical Production Machine Operator
    #[serde(rename = "711911")]
    N711911,
    /// Motion Picture Projectionist
    #[serde(rename = "711912")]
    N711912,
    /// Sand Blaster
    #[serde(rename = "711913")]
    N711913,
    /// Sterilisation Technician
    #[serde(rename = "711914")]
    N711914,
    /// Machine Operators nec
    #[serde(rename = "711999")]
    N711999,
    /// Stationary Plant Operators nfd
    #[serde(rename = "712000")]
    N712000,
    /// Crane, Hoist or Lift Operator
    #[serde(rename = "712111")]
    N712111,
    /// Drillers, Miners and Shot Firers nfd
    #[serde(rename = "712200")]
    N712200,
    /// Driller
    #[serde(rename = "712211")]
    N712211,
    /// Miner
    #[serde(rename = "712212")]
    N712212,
    /// Shot Firer
    #[serde(rename = "712213")]
    N712213,
    /// Engineering Production Systems Worker
    #[serde(rename = "712311")]
    N712311,
    /// Other Stationary Plant Operators nfd
    #[serde(rename = "712900")]
    N712900,
    /// Boiler or Engine Operator
    #[serde(rename = "712911")]
    N712911,
    /// Bulk Materials Handling Plant Operator
    #[serde(rename = "712912")]
    N712912,
    /// Cement Production Plant Operator
    #[serde(rename = "712913")]
    N712913,
    /// Concrete Batching Plant Operator
    #[serde(rename = "712914")]
    N712914,
    /// Concrete Pump Operator
    #[serde(rename = "712915")]
    N712915,
    /// Paper and Pulp Mill Operator
    #[serde(rename = "712916")]
    N712916,
    /// Railway Signal Operator
    #[serde(rename = "712917")]
    N712917,
    /// Train Controller
    #[serde(rename = "712918")]
    N712918,
    /// Waste Water or Water Plant Operator
    #[serde(rename = "712921")]
    N712921,
    /// Weighbridge Operator
    #[serde(rename = "712922")]
    N712922,
    /// Stationary Plant Operators nec
    #[serde(rename = "712999")]
    N712999,
    /// Mobile Plant Operators nfd
    #[serde(rename = "721000")]
    N721000,
    /// Agricultural, Forestry and Horticultural Plant Operators nfd
    #[serde(rename = "721100")]
    N721100,
    /// Agricultural and Horticultural Mobile Plant Operator
    #[serde(rename = "721111")]
    N721111,
    /// Logging Plant Operator
    #[serde(rename = "721112")]
    N721112,
    /// Earthmoving Plant Operators nfd
    #[serde(rename = "721200")]
    N721200,
    /// Earthmoving Plant Operator (General)
    #[serde(rename = "721211")]
    N721211,
    /// Backhoe Operator
    #[serde(rename = "721212")]
    N721212,
    /// Bulldozer Operator
    #[serde(rename = "721213")]
    N721213,
    /// Excavator Operator
    #[serde(rename = "721214")]
    N721214,
    /// Grader Operator
    #[serde(rename = "721215")]
    N721215,
    /// Loader Operator
    #[serde(rename = "721216")]
    N721216,
    /// Forklift Driver
    #[serde(rename = "721311")]
    N721311,
    /// Other Mobile Plant Operators nfd
    #[serde(rename = "721900")]
    N721900,
    /// Aircraft Baggage Handler and Airline Ground Crew
    #[serde(rename = "721911")]
    N721911,
    /// Linemarker
    #[serde(rename = "721912")]
    N721912,
    /// Paving Plant Operator
    #[serde(rename = "721913")]
    N721913,
    /// Railway Track Plant Operator
    #[serde(rename = "721914")]
    N721914,
    /// Road Roller Operator
    #[serde(rename = "721915")]
    N721915,
    /// Streetsweeper Operator
    #[serde(rename = "721916")]
    N721916,
    /// Mobile Plant Operators nec
    #[serde(rename = "721999")]
    N721999,
    /// Road and Rail Drivers nfd
    #[serde(rename = "730000")]
    N730000,
    /// Automobile, Bus and Rail Drivers nfd
    #[serde(rename = "731000")]
    N731000,
    /// Automobile Drivers nfd
    #[serde(rename = "731100")]
    N731100,
    /// Chauffeur
    #[serde(rename = "731111")]
    N731111,
    /// Taxi Driver
    #[serde(rename = "731112")]
    N731112,
    /// Automobile Drivers nec
    #[serde(rename = "731199")]
    N731199,
    /// Bus and Coach Drivers nfd
    #[serde(rename = "731200")]
    N731200,
    /// Bus Driver
    #[serde(rename = "731211")]
    N731211,
    /// Charter and Tour Bus Driver
    #[serde(rename = "731212")]
    N731212,
    /// Passenger Coach Driver
    #[serde(rename = "731213")]
    N731213,
    /// Train and Tram Drivers nfd
    #[serde(rename = "731300")]
    N731300,
    /// Train Driver
    #[serde(rename = "731311")]
    N731311,
    /// Tram Driver
    #[serde(rename = "731312")]
    N731312,
    /// Delivery Driver
    #[serde(rename = "732111")]
    N732111,
    /// Truck Drivers nfd
    #[serde(rename = "733100")]
    N733100,
    /// Truck Driver (General)
    #[serde(rename = "733111")]
    N733111,
    /// Aircraft Refueller
    #[serde(rename = "733112")]
    N733112,
    /// Furniture Removalist
    #[serde(rename = "733113")]
    N733113,
    /// Tanker Driver
    #[serde(rename = "733114")]
    N733114,
    /// Tow Truck Driver
    #[serde(rename = "733115")]
    N733115,
    /// Storeperson
    #[serde(rename = "741111")]
    N741111,
    /// Labourers nfd
    #[serde(rename = "800000")]
    N800000,
    /// Cleaners and Laundry Workers nfd
    #[serde(rename = "811000")]
    N811000,
    /// Car Detailer
    #[serde(rename = "811111")]
    N811111,
    /// Commercial Cleaner
    #[serde(rename = "811211")]
    N811211,
    /// Domestic Cleaner
    #[serde(rename = "811311")]
    N811311,
    /// Housekeepers nfd
    #[serde(rename = "811400")]
    N811400,
    /// Commercial Housekeeper
    #[serde(rename = "811411")]
    N811411,
    /// Domestic Housekeeper
    #[serde(rename = "811412")]
    N811412,
    /// Laundry Workers nfd
    #[serde(rename = "811500")]
    N811500,
    /// Laundry Worker (General)
    #[serde(rename = "811511")]
    N811511,
    /// Drycleaner
    #[serde(rename = "811512")]
    N811512,
    /// Ironer or Presser
    #[serde(rename = "811513")]
    N811513,
    /// Other Cleaners nfd
    #[serde(rename = "811600")]
    N811600,
    /// Carpet Cleaner
    #[serde(rename = "811611")]
    N811611,
    /// Window Cleaner
    #[serde(rename = "811612")]
    N811612,
    /// Cleaners nec
    #[serde(rename = "811699")]
    N811699,
    /// Construction and Mining Labourers nfd
    #[serde(rename = "821000")]
    N821000,
    /// Building and Plumbing Labourers nfd
    #[serde(rename = "821100")]
    N821100,
    /// Builder's Labourer
    #[serde(rename = "821111")]
    N821111,
    /// Drainage, Sewerage and Stormwater Labourer
    #[serde(rename = "821112")]
    N821112,
    /// Earthmoving Labourer
    #[serde(rename = "821113")]
    N821113,
    /// Plumber's Assistant
    #[serde(rename = "821114")]
    N821114,
    /// Concreter
    #[serde(rename = "821211")]
    N821211,
    /// Fencer
    #[serde(rename = "821311")]
    N821311,
    /// Insulation and Home Improvement Installers nfd
    #[serde(rename = "821400")]
    N821400,
    /// Building Insulation Installer
    #[serde(rename = "821411")]
    N821411,
    /// Home Improvement Installer
    #[serde(rename = "821412")]
    N821412,
    /// Paving and Surfacing Labourer
    #[serde(rename = "821511")]
    N821511,
    /// Railway Track Worker
    #[serde(rename = "821611")]
    N821611,
    /// Structural Steel Construction Workers nfd
    #[serde(rename = "821700")]
    N821700,
    /// Construction Rigger
    #[serde(rename = "821711")]
    N821711,
    /// Scaffolder
    #[serde(rename = "821712")]
    N821712,
    /// Steel Fixer
    #[serde(rename = "821713")]
    N821713,
    /// Structural Steel Erector
    #[serde(rename = "821714")]
    N821714,
    /// Other Construction and Mining Labourers nfd
    #[serde(rename = "821900")]
    N821900,
    /// Crane Chaser
    #[serde(rename = "821911")]
    N821911,
    /// Driller's Assistant
    #[serde(rename = "821912")]
    N821912,
    /// Lagger
    #[serde(rename = "821913")]
    N821913,
    /// Mining Support Worker
    #[serde(rename = "821914")]
    N821914,
    /// Surveyor's Assistant
    #[serde(rename = "821915")]
    N821915,
    /// Factory Process Workers nfd
    #[serde(rename = "830000")]
    N830000,
    /// Food Process Workers nfd
    #[serde(rename = "831000")]
    N831000,
    /// Food and Drink Factory Workers nfd
    #[serde(rename = "831100")]
    N831100,
    /// Baking Factory Worker
    #[serde(rename = "831111")]
    N831111,
    /// Brewery Worker
    #[serde(rename = "831112")]
    N831112,
    /// Confectionery Maker
    #[serde(rename = "831113")]
    N831113,
    /// Dairy Products Maker
    #[serde(rename = "831114")]
    N831114,
    /// Fruit and Vegetable Factory Worker
    #[serde(rename = "831115")]
    N831115,
    /// Grain Mill Worker
    #[serde(rename = "831116")]
    N831116,
    /// Sugar Mill Worker
    #[serde(rename = "831117")]
    N831117,
    /// Winery Cellar Hand
    #[serde(rename = "831118")]
    N831118,
    /// Food and Drink Factory Workers nec
    #[serde(rename = "831199")]
    N831199,
    /// Meat Boners and Slicers, and Slaughterers nfd
    #[serde(rename = "831200")]
    N831200,
    /// Meat Boner and Slicer
    #[serde(rename = "831211")]
    N831211,
    /// Slaughterer
    #[serde(rename = "831212")]
    N831212,
    /// Meat, Poultry and Seafood Process Workers nfd
    #[serde(rename = "831300")]
    N831300,
    /// Meat Process Worker
    #[serde(rename = "831311")]
    N831311,
    /// Poultry Process Worker
    #[serde(rename = "831312")]
    N831312,
    /// Seafood Process Worker
    #[serde(rename = "831313")]
    N831313,
    /// Packers and Product Assemblers nfd
    #[serde(rename = "832000")]
    N832000,
    /// Packers nfd
    #[serde(rename = "832100")]
    N832100,
    /// Chocolate Packer
    #[serde(rename = "832111")]
    N832111,
    /// Container Filler
    #[serde(rename = "832112")]
    N832112,
    /// Fruit and Vegetable Packer
    #[serde(rename = "832113")]
    N832113,
    /// Meat Packer
    #[serde(rename = "832114")]
    N832114,
    /// Seafood Packer
    #[serde(rename = "832115")]
    N832115,
    /// Packers nec
    #[serde(rename = "832199")]
    N832199,
    /// Product Assembler
    #[serde(rename = "832211")]
    N832211,
    /// Miscellaneous Factory Process Workers nfd
    #[serde(rename = "839000")]
    N839000,
    /// Metal Engineering Process Worker
    #[serde(rename = "839111")]
    N839111,
    /// Plastics and Rubber Factory Workers nfd
    #[serde(rename = "839200")]
    N839200,
    /// Plastics Factory Worker
    #[serde(rename = "839211")]
    N839211,
    /// Rubber Factory Worker
    #[serde(rename = "839212")]
    N839212,
    /// Product Quality Controllers nfd
    #[serde(rename = "839300")]
    N839300,
    /// Product Examiner
    #[serde(rename = "839311")]
    N839311,
    /// Product Grader
    #[serde(rename = "839312")]
    N839312,
    /// Product Tester
    #[serde(rename = "839313")]
    N839313,
    /// Timber and Wood Process Workers nfd
    #[serde(rename = "839400")]
    N839400,
    /// Paper and Pulp Mill Worker
    #[serde(rename = "839411")]
    N839411,
    /// Sawmill or Timber Yard Worker
    #[serde(rename = "839412")]
    N839412,
    /// Wood and Wood Products Factory Worker
    #[serde(rename = "839413")]
    N839413,
    /// Other Factory Process Workers nfd
    #[serde(rename = "839900")]
    N839900,
    /// Cement and Concrete Plant Worker
    #[serde(rename = "839911")]
    N839911,
    /// Chemical Plant Worker
    #[serde(rename = "839912")]
    N839912,
    /// Clay Processing Factory Worker
    #[serde(rename = "839913")]
    N839913,
    /// Fabric and Textile Factory Worker
    #[serde(rename = "839914")]
    N839914,
    /// Footwear Factory Worker
    #[serde(rename = "839915")]
    N839915,
    /// Glass Processing Worker
    #[serde(rename = "839916")]
    N839916,
    /// Hide and Skin Processing Worker
    #[serde(rename = "839917")]
    N839917,
    /// Factory Process Workers nec
    #[serde(rename = "839999")]
    N839999,
    /// Farm, Forestry and Garden Workers nfd
    #[serde(rename = "841000")]
    N841000,
    /// Aquaculture Worker
    #[serde(rename = "841111")]
    N841111,
    /// Crop Farm Workers nfd
    #[serde(rename = "841200")]
    N841200,
    /// Fruit or Nut Farm Worker
    #[serde(rename = "841211")]
    N841211,
    /// Fruit or Nut Picker
    #[serde(rename = "841212")]
    N841212,
    /// Grain, Oilseed or Pasture Farm Worker
    #[serde(rename = "841213")]
    N841213,
    /// Vegetable Farm Worker
    #[serde(rename = "841214")]
    N841214,
    /// Vegetable Picker
    #[serde(rename = "841215")]
    N841215,
    /// Vineyard Worker
    #[serde(rename = "841216")]
    N841216,
    /// Crop Farm Workers nec
    #[serde(rename = "841299")]
    N841299,
    /// Forestry and Logging Workers nfd
    #[serde(rename = "841300")]
    N841300,
    /// Forestry Worker
    #[serde(rename = "841311")]
    N841311,
    /// Logging Assistant
    #[serde(rename = "841312")]
    N841312,
    /// Tree Faller
    #[serde(rename = "841313")]
    N841313,
    /// Garden and Nursery Labourers nfd
    #[serde(rename = "841400")]
    N841400,
    /// Garden Labourer
    #[serde(rename = "841411")]
    N841411,
    /// Horticultural Nursery Assistant
    #[serde(rename = "841412")]
    N841412,
    /// Livestock Farm Workers nfd
    #[serde(rename = "841500")]
    N841500,
    /// Beef Cattle Farm Worker
    #[serde(rename = "841511")]
    N841511,
    /// Dairy Cattle Farm Worker
    #[serde(rename = "841512")]
    N841512,
    /// Mixed Livestock Farm Worker
    #[serde(rename = "841513")]
    N841513,
    /// Poultry Farm Worker
    #[serde(rename = "841514")]
    N841514,
    /// Sheep Farm Worker
    #[serde(rename = "841515")]
    N841515,
    /// Stablehand
    #[serde(rename = "841516")]
    N841516,
    /// Wool Handler
    #[serde(rename = "841517")]
    N841517,
    /// Livestock Farm Workers nec
    #[serde(rename = "841599")]
    N841599,
    /// Mixed Crop and Livestock Farm Worker
    #[serde(rename = "841611")]
    N841611,
    /// Other Farm, Forestry and Garden Workers nfd
    #[serde(rename = "841900")]
    N841900,
    /// Hunter-Trapper
    #[serde(rename = "841911")]
    N841911,
    /// Pest or Weed Controller
    #[serde(rename = "841912")]
    N841912,
    /// Farm, Forestry and Garden Workers nec
    #[serde(rename = "841999")]
    N841999,
    /// Food Preparation Assistants nfd
    #[serde(rename = "851000")]
    N851000,
    /// Fast Food Cook
    #[serde(rename = "851111")]
    N851111,
    /// Food Trades Assistants nfd
    #[serde(rename = "851200")]
    N851200,
    /// Pastrycook's Assistant
    #[serde(rename = "851211")]
    N851211,
    /// Food Trades Assistants nec
    #[serde(rename = "851299")]
    N851299,
    /// Kitchenhand
    #[serde(rename = "851311")]
    N851311,
    /// Other Labourers nfd
    #[serde(rename = "890000")]
    N890000,
    /// Freight Handlers and Shelf Fillers nfd
    #[serde(rename = "891000")]
    N891000,
    /// Freight and Furniture Handlers nfd
    #[serde(rename = "891100")]
    N891100,
    /// Freight Handler (Rail or Road)
    #[serde(rename = "891111")]
    N891111,
    /// Truck Driver's Offsider
    #[serde(rename = "891112")]
    N891112,
    /// Waterside Worker
    #[serde(rename = "891113")]
    N891113,
    /// Shelf Filler
    #[serde(rename = "891211")]
    N891211,
    /// Miscellaneous Labourers nfd
    #[serde(rename = "899000")]
    N899000,
    /// Caretaker
    #[serde(rename = "899111")]
    N899111,
    /// Deck and Fishing Hands nfd
    #[serde(rename = "899200")]
    N899200,
    /// Deck Hand
    #[serde(rename = "899211")]
    N899211,
    /// Fishing Hand
    #[serde(rename = "899212")]
    N899212,
    /// Handyperson
    #[serde(rename = "899311")]
    N899311,
    /// Motor Vehicle Parts and Accessories Fitters nfd
    #[serde(rename = "899400")]
    N899400,
    /// Motor Vehicle Parts and Accessories Fitter (General)
    #[serde(rename = "899411")]
    N899411,
    /// Autoglazier
    #[serde(rename = "899412")]
    N899412,
    /// Exhaust and Muffler Fitter
    #[serde(rename = "899413")]
    N899413,
    /// Radiator Fitter
    #[serde(rename = "899414")]
    N899414,
    /// Tyre Fitter
    #[serde(rename = "899415")]
    N899415,
    /// Printing Assistants and Table Workers nfd
    #[serde(rename = "899500")]
    N899500,
    /// Printer's Assistant
    #[serde(rename = "899511")]
    N899511,
    /// Printing Table Worker
    #[serde(rename = "899512")]
    N899512,
    /// Recycling or Rubbish Collector
    #[serde(rename = "899611")]
    N899611,
    /// Vending Machine Attendant
    #[serde(rename = "899711")]
    N899711,
    /// Other Miscellaneous Labourers nfd
    #[serde(rename = "899900")]
    N899900,
    /// Bicycle Mechanic
    #[serde(rename = "899911")]
    N899911,
    /// Car Park Attendant
    #[serde(rename = "899912")]
    N899912,
    /// Crossing Supervisor
    #[serde(rename = "899913")]
    N899913,
    /// Electrical or Telecommunications Trades Assistant
    #[serde(rename = "899914")]
    N899914,
    /// Leaflet or Newspaper Deliverer
    #[serde(rename = "899915")]
    N899915,
    /// Mechanic's Assistant
    #[serde(rename = "899916")]
    N899916,
    /// Railways Assistant
    #[serde(rename = "899917")]
    N899917,
    /// Sign Erector
    #[serde(rename = "899918")]
    N899918,
    /// Ticket Collector or Usher
    #[serde(rename = "899921")]
    N899921,
    /// Trolley Collector
    #[serde(rename = "899922")]
    N899922,
    /// Labourers nec
    #[serde(rename = "899999")]
    N899999,
}

/// The use of a human name
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
    /// Maiden
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

/// The status of a resource narrative
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

/// The type of network access point of this participant in the audit event
///
/// System: <http://hl7.org/fhir/network-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NetworkType {
    /// Machine Name
    #[default]
    #[serde(rename = "1")]
    N1,
    /// IP Address
    #[serde(rename = "2")]
    N2,
    /// Telephone Number
    #[serde(rename = "3")]
    N3,
    /// Email address
    #[serde(rename = "4")]
    N4,
    /// URI
    #[serde(rename = "5")]
    N5,
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

/// Codes specifying the state of the request. Describes the lifecycle of the
/// nutrition order.
///
/// System: <http://hl7.org/fhir/nutrition-order-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NutritionOrderStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On-Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Identifier for the data life-cycle stage for the object.
///
/// System: <http://hl7.org/fhir/object-lifecycle>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObjectLifecycle {
    /// Origination / Creation
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Import / Copy from original
    #[serde(rename = "2")]
    N2,
    /// Amendment
    #[serde(rename = "3")]
    N3,
    /// Verification
    #[serde(rename = "4")]
    N4,
    /// Translation
    #[serde(rename = "5")]
    N5,
    /// Access / Use
    #[serde(rename = "6")]
    N6,
    /// De-identification
    #[serde(rename = "7")]
    N7,
    /// Aggregation, summarization, derivation
    #[serde(rename = "8")]
    N8,
    /// Report
    #[serde(rename = "9")]
    N9,
    /// Export / Copy to target
    #[serde(rename = "10")]
    N10,
    /// Disclosure
    #[serde(rename = "11")]
    N11,
    /// Receipt of disclosure
    #[serde(rename = "12")]
    N12,
    /// Archiving
    #[serde(rename = "13")]
    N13,
    /// Logical deletion
    #[serde(rename = "14")]
    N14,
    /// Permanent erasure / Physical destruction
    #[serde(rename = "15")]
    N15,
}

/// Code representing the role the Object played in the event.
///
/// System: <http://hl7.org/fhir/object-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObjectRole {
    /// Patient
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Location
    #[serde(rename = "2")]
    N2,
    /// Report
    #[serde(rename = "3")]
    N3,
    /// Domain Resource
    #[serde(rename = "4")]
    N4,
    /// Master file
    #[serde(rename = "5")]
    N5,
    /// User
    #[serde(rename = "6")]
    N6,
    /// List
    #[serde(rename = "7")]
    N7,
    /// Doctor
    #[serde(rename = "8")]
    N8,
    /// Subscriber
    #[serde(rename = "9")]
    N9,
    /// Guarantor
    #[serde(rename = "10")]
    N10,
    /// Security User Entity
    #[serde(rename = "11")]
    N11,
    /// Security User Group
    #[serde(rename = "12")]
    N12,
    /// Security Resource
    #[serde(rename = "13")]
    N13,
    /// Security Granularity Definition
    #[serde(rename = "14")]
    N14,
    /// Practitioner
    #[serde(rename = "15")]
    N15,
    /// Data Destination
    #[serde(rename = "16")]
    N16,
    /// Data Repository
    #[serde(rename = "17")]
    N17,
    /// Schedule
    #[serde(rename = "18")]
    N18,
    /// Customer
    #[serde(rename = "19")]
    N19,
    /// Job
    #[serde(rename = "20")]
    N20,
    /// Job Stream
    #[serde(rename = "21")]
    N21,
    /// Table
    #[serde(rename = "22")]
    N22,
    /// Routing Criteria
    #[serde(rename = "23")]
    N23,
    /// Query
    #[serde(rename = "24")]
    N24,
}

/// Code for the object type involved audited.
///
/// System: <http://hl7.org/fhir/object-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObjectType {
    /// Person
    #[default]
    #[serde(rename = "1")]
    N1,
    /// System Object
    #[serde(rename = "2")]
    N2,
    /// Organization
    #[serde(rename = "3")]
    N3,
    /// Other
    #[serde(rename = "4")]
    N4,
}

/// Code to classify the kind of observation in Observation.value[x]
///
/// System: <http://hl7.org/fhir/obs-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObsKind {
    /// result
    #[default]
    #[serde(rename = "result")]
    Result,
    /// response
    #[serde(rename = "response")]
    Response,
    /// information
    #[serde(rename = "information")]
    Information,
}

/// Observation Category codes.
///
/// System: <http://hl7.org/fhir/observation-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationCategory {
    /// Social History
    #[default]
    #[serde(rename = "social-history")]
    SocialHistory,
    /// Vital Signs
    #[serde(rename = "vital-signs")]
    VitalSigns,
    /// Imaging
    #[serde(rename = "imaging")]
    Imaging,
    /// Laboratory
    #[serde(rename = "laboratory")]
    Laboratory,
    /// Procedure
    #[serde(rename = "procedure")]
    Procedure,
    /// Survey
    #[serde(rename = "survey")]
    Survey,
    /// Exam
    #[serde(rename = "exam")]
    Exam,
    /// Therapy
    #[serde(rename = "therapy")]
    Therapy,
}

/// Codes specifying how two observations are related.
///
/// System: <http://hl7.org/fhir/observation-relationshiptypes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ObservationRelationshiptypes {
    /// Has Member
    #[default]
    #[serde(rename = "has-member")]
    HasMember,
    /// Derived From
    #[serde(rename = "derived-from")]
    DerivedFrom,
    /// Sequel To
    #[serde(rename = "sequel-to")]
    SequelTo,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Qualified By
    #[serde(rename = "qualified-by")]
    QualifiedBy,
    /// Interfered By
    #[serde(rename = "interfered-by")]
    InterferedBy,
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
    /// cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown Status
    #[serde(rename = "unknown")]
    Unknown,
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

/// Operation Outcome codes used by FHIR test servers (see Implementation file
/// translations.xml)
///
/// System: <http://hl7.org/fhir/operation-outcome>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationOutcome {
    /// You must authenticate before you can use this service
    #[default]
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
    /// No Resource found matching the query "%s"
    #[serde(rename = "MSG_NO_MATCH")]
    MsgNoMatch,
    /// Resource Id "%s" does not exist
    #[serde(rename = "MSG_NO_EXIST")]
    MsgNoExist,
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
    /// Parameter "%s" is not allowed to repeat
    #[serde(rename = "MSG_PARAM_NO_REPEAT")]
    MsgParamNoRepeat,
    /// Parameter "%s" not understood
    #[serde(rename = "MSG_PARAM_UNKNOWN")]
    MsgParamUnknown,
    /// Parameter "%s" content is invalid
    #[serde(rename = "MSG_PARAM_INVALID")]
    MsgParamInvalid,
    /// Parameter "%s" modifier is invalid
    #[serde(rename = "MSG_PARAM_MODIFIER_INVALID")]
    MsgParamModifierInvalid,
    /// Resources with identity "example" cannot be deleted (for
    /// testing/training purposes)
    #[serde(rename = "MSG_RESOURCE_EXAMPLE_PROTECTED")]
    MsgResourceExampleProtected,
    /// unable to allocate resource id
    #[serde(rename = "MSG_RESOURCE_ID_FAIL")]
    MsgResourceIdFail,
    /// Not allowed to submit a resource for this operation
    #[serde(rename = "MSG_RESOURCE_NOT_ALLOWED")]
    MsgResourceNotAllowed,
    /// A resource is required
    #[serde(rename = "MSG_RESOURCE_REQUIRED")]
    MsgResourceRequired,
    /// Resource Id Mismatch
    #[serde(rename = "MSG_RESOURCE_ID_MISMATCH")]
    MsgResourceIdMismatch,
    /// Resource Id Missing
    #[serde(rename = "MSG_RESOURCE_ID_MISSING")]
    MsgResourceIdMissing,
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
    /// Error: Multiple matches exist for the conditional update
    #[serde(rename = "UPDATE_MULTIPLE_MATCHES")]
    UpdateMultipleMatches,
    /// Error: no processable search found for %s search parameters "%s"
    #[serde(rename = "SEARCH_NONE")]
    SearchNone,
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

/// The status of the response to an order.
///
/// System: <http://hl7.org/fhir/order-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OrderStatus {
    /// Pending
    #[default]
    #[serde(rename = "pending")]
    Pending,
    /// Review
    #[serde(rename = "review")]
    Review,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Replaced
    #[serde(rename = "replaced")]
    Replaced,
    /// Aborted
    #[serde(rename = "aborted")]
    Aborted,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
}

/// This example value set defines a set of codes that can be used to indicate
/// a type of organization.
///
/// System: <http://hl7.org/fhir/organization-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OrganizationType {
    /// Healthcare Provider
    #[default]
    #[serde(rename = "prov")]
    Prov,
    /// Hospital Department
    #[serde(rename = "dept")]
    Dept,
    /// Organizational team
    #[serde(rename = "team")]
    Team,
    /// Government
    #[serde(rename = "govt")]
    Govt,
    /// Insurance Company
    #[serde(rename = "ins")]
    Ins,
    /// Educational Institute
    #[serde(rename = "edu")]
    Edu,
    /// Religious Institution
    #[serde(rename = "reli")]
    Reli,
    /// Clinical Research Sponsor
    #[serde(rename = "crs")]
    Crs,
    /// Community Group
    #[serde(rename = "cg")]
    Cg,
    /// Non-Healthcare Business or Corporation
    #[serde(rename = "bus")]
    Bus,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// This value set defines a set of codes that can be used to indicate how an
/// individual participates in an encounter.
///
/// System: <http://hl7.org/fhir/participant-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ParticipantType {
    /// Translator
    #[default]
    #[serde(rename = "translator")]
    Translator,
    /// Emergency
    #[serde(rename = "emergency")]
    Emergency,
}

/// Is the Participant required to attend the appointment.
///
/// System: <http://hl7.org/fhir/participantrequired>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Participantrequired {
    /// Required
    #[default]
    #[serde(rename = "required")]
    Required,
    /// Optional
    #[serde(rename = "optional")]
    Optional,
    /// Information Only
    #[serde(rename = "information-only")]
    InformationOnly,
}

/// The Participation status of an appointment.
///
/// System: <http://hl7.org/fhir/participantstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Participantstatus {
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
    /// In Process
    #[serde(rename = "in-process")]
    InProcess,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Needs Action
    #[serde(rename = "needs-action")]
    NeedsAction,
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

/// This value set defines a set of codes that are used to indicate the nature
/// of the relationship between a patient and a contactperson for that patient.
///
/// System: <http://hl7.org/fhir/patient-contact-relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PatientContactRelationship {
    /// Emergency
    #[default]
    #[serde(rename = "emergency")]
    Emergency,
    /// Family
    #[serde(rename = "family")]
    Family,
    /// Guardian
    #[serde(rename = "guardian")]
    Guardian,
    /// Friend
    #[serde(rename = "friend")]
    Friend,
    /// Partner
    #[serde(rename = "partner")]
    Partner,
    /// Work
    #[serde(rename = "work")]
    Work,
    /// Caregiver
    #[serde(rename = "caregiver")]
    Caregiver,
    /// Agent
    #[serde(rename = "agent")]
    Agent,
    /// Guarantor
    #[serde(rename = "guarantor")]
    Guarantor,
    /// Owner of animal
    #[serde(rename = "owner")]
    Owner,
    /// Parent
    #[serde(rename = "parent")]
    Parent,
}

/// A Master Patient Index (MPI) assessment of whether a candidate patient
/// record is a match or not.
///
/// System: <http://hl7.org/fhir/patient-mpi-match>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PatientMpiMatch {
    /// Certain Match
    #[default]
    #[serde(rename = "certain")]
    Certain,
    /// Probable Match
    #[serde(rename = "probable")]
    Probable,
    /// Possible Match
    #[serde(rename = "possible")]
    Possible,
    /// Certainly Not a Match
    #[serde(rename = "certainly-not")]
    CertainlyNot,
}

/// This value set includes sample Payee Type codes.
///
/// System: <http://hl7.org/fhir/payeetype>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Payeetype {
    /// subscriber
    #[default]
    #[serde(rename = "subscriber")]
    Subscriber,
    /// provider
    #[serde(rename = "provider")]
    Provider,
    /// other
    #[serde(rename = "other")]
    Other,
}

/// This value set includes sample Payment Type codes.
///
/// System: <http://hl7.org/fhir/payment-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PaymentType {
    /// payment
    #[default]
    #[serde(rename = "payment")]
    Payment,
    /// adjustment
    #[serde(rename = "adjustment")]
    Adjustment,
    /// advance
    #[serde(rename = "advance")]
    Advance,
}

/// This value set includes a sample set of Payment Status codes.
///
/// System: <http://hl7.org/fhir/paymentstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Paymentstatus {
    /// paid
    #[default]
    #[serde(rename = "paid")]
    Paid,
    /// cleared
    #[serde(rename = "cleared")]
    Cleared,
}

/// This example value set defines a set of codes that can be used to indicate
/// the role of a Practitioner.
///
/// System: <http://hl7.org/fhir/practitioner-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PractitionerRole {
    /// Doctor
    #[default]
    #[serde(rename = "doctor")]
    Doctor,
    /// Nurse
    #[serde(rename = "nurse")]
    Nurse,
    /// Pharmacist
    #[serde(rename = "pharmacist")]
    Pharmacist,
    /// Researcher
    #[serde(rename = "researcher")]
    Researcher,
    /// Teacher/educator
    #[serde(rename = "teacher")]
    Teacher,
    /// ICT professional
    #[serde(rename = "ict")]
    Ict,
}

/// This example value set defines a set of codes that can be used to indicate
/// the specialty of a Practitioner.
///
/// System: <http://hl7.org/fhir/practitioner-specialty>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PractitionerSpecialty {
    /// Cardiologist
    #[default]
    #[serde(rename = "cardio")]
    Cardio,
    /// Dentist
    #[serde(rename = "dent")]
    Dent,
    /// Dietary consultant
    #[serde(rename = "dietary")]
    Dietary,
    /// Midwife
    #[serde(rename = "midw")]
    Midw,
    /// Systems architect
    #[serde(rename = "sysarch")]
    Sysarch,
}

/// This value set is provided as an exemplar. The value set to instantiate
/// this attribute should be drawn from a terminologically robust code system
/// that consists of or contains concepts to support the procedure performance
/// process.
///
/// System: <http://hl7.org/fhir/procedure-progress-status-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProcedureProgressStatusCode {
    /// In Operating Room
    #[default]
    #[serde(rename = "a")]
    A,
    /// Prepared
    #[serde(rename = "b")]
    B,
    /// Anesthesia induced
    #[serde(rename = "c")]
    C,
    /// Opened (skin)
    #[serde(rename = "d")]
    D,
    /// Closed (skin)
    #[serde(rename = "e")]
    E,
    /// In Recovery Room
    #[serde(rename = "f")]
    F,
}

/// The nature of the relationship with this procedure.
///
/// System: <http://hl7.org/fhir/procedure-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProcedureRelationshipType {
    /// Caused By
    #[default]
    #[serde(rename = "caused-by")]
    CausedBy,
    /// Because Of
    #[serde(rename = "because-of")]
    BecauseOf,
}

/// The priority of the request.
///
/// System: <http://hl7.org/fhir/procedure-request-priority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProcedureRequestPriority {
    /// Routine
    #[default]
    #[serde(rename = "routine")]
    Routine,
    /// Urgent
    #[serde(rename = "urgent")]
    Urgent,
    /// Stat
    #[serde(rename = "stat")]
    Stat,
    /// ASAP
    #[serde(rename = "asap")]
    Asap,
}

/// The status of the request.
///
/// System: <http://hl7.org/fhir/procedure-request-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProcedureRequestStatus {
    /// Proposed
    #[default]
    #[serde(rename = "proposed")]
    Proposed,
    /// Draft
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
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Aborted
    #[serde(rename = "aborted")]
    Aborted,
}

/// A code specifying the state of the procedure.
///
/// System: <http://hl7.org/fhir/procedure-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProcedureStatus {
    /// In Progress
    #[default]
    #[serde(rename = "in-progress")]
    InProgress,
    /// Aboted
    #[serde(rename = "aborted")]
    Aborted,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// This value set includes sample Process Outcome codes.
///
/// System: <http://hl7.org/fhir/processoutcomecodes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Processoutcomecodes {
    /// complete
    #[default]
    #[serde(rename = "complete")]
    Complete,
    /// pended
    #[serde(rename = "pended")]
    Pended,
    /// error
    #[serde(rename = "error")]
    Error,
}

/// This value set includes the financial processing priority codes.
///
/// System: <http://hl7.org/fhir/processpriority>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Processpriority {
    /// Immediate
    #[default]
    #[serde(rename = "stat")]
    Stat,
    /// Normal
    #[serde(rename = "normal")]
    Normal,
    /// Deferred
    #[serde(rename = "deferred")]
    Deferred,
}

/// How a property is represented on the wire.
///
/// System: <http://hl7.org/fhir/property-representation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PropertyRepresentation {
    /// XML Attribute
    #[default]
    #[serde(rename = "xmlAttr")]
    XmlAttr,
}

/// How an entity was used in an activity.
///
/// System: <http://hl7.org/fhir/provenance-entity-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProvenanceEntityRole {
    /// Derivation
    #[default]
    #[serde(rename = "derivation")]
    Derivation,
    /// Revision
    #[serde(rename = "revision")]
    Revision,
    /// Quotation
    #[serde(rename = "quotation")]
    Quotation,
    /// Source
    #[serde(rename = "source")]
    Source,
}

/// The role that a provenance participant played
///
/// System: <http://hl7.org/fhir/provenance-participant-role>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProvenanceParticipantRole {
    /// Enterer
    #[default]
    #[serde(rename = "enterer")]
    Enterer,
    /// Performer
    #[serde(rename = "performer")]
    Performer,
    /// Author
    #[serde(rename = "author")]
    Author,
    /// Verifier
    #[serde(rename = "verifier")]
    Verifier,
    /// Legal Authenticator
    #[serde(rename = "legal")]
    Legal,
    /// Attester
    #[serde(rename = "attester")]
    Attester,
    /// Informant
    #[serde(rename = "informant")]
    Informant,
    /// Custodian
    #[serde(rename = "custodian")]
    Custodian,
    /// Assembler
    #[serde(rename = "assembler")]
    Assembler,
    /// Composer
    #[serde(rename = "composer")]
    Composer,
}

/// The type of a provenance participant. Use either a defined FHIR resource
/// type, or, if the reference is not to a resource, one of the defined codes,
/// or some other code if none of the defined codes are appropriate.
///
/// System: <http://hl7.org/fhir/provenance-participant-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ProvenanceParticipantType {
    /// Person
    #[default]
    #[serde(rename = "person")]
    Person,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Organization
    #[serde(rename = "organization")]
    Organization,
    /// Software
    #[serde(rename = "software")]
    Software,
    /// Patient
    #[serde(rename = "patient")]
    Patient,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Related Person
    #[serde(rename = "related-person")]
    RelatedPerson,
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

/// Value Set for QICore Adverse Event Category (Preferred)
///
/// System: <http://hl7.org/fhir/qicore-adverseevent-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreAdverseeventCategory {
    /// Blood or Blood Product
    #[default]
    #[serde(rename = "blood")]
    Blood,
    /// Device, Materials, or Information Technology
    #[serde(rename = "device")]
    Device,
    /// Fall
    #[serde(rename = "fall")]
    Fall,
    /// Healthcare-Related Infection
    #[serde(rename = "infection")]
    Infection,
    /// Medication or Other Substance
    #[serde(rename = "substance")]
    Substance,
    /// Perinatal
    #[serde(rename = "perinatal")]
    Perinatal,
    /// Pressure Ulcer
    #[serde(rename = "ulcer")]
    Ulcer,
    /// Surgery or Anesthesia
    #[serde(rename = "surgery")]
    Surgery,
    /// Venous Thromboembolism
    #[serde(rename = "embolism")]
    Embolism,
    /// Other (please specify)
    #[serde(rename = "other")]
    Other,
}

/// Value Set for QICore Adverse Event Type Codes (Preferred)
///
/// System: <http://hl7.org/fhir/qicore-adverseevent-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreAdverseeventType {
    /// Incident
    #[default]
    #[serde(rename = "incident")]
    Incident,
    /// Near Miss
    #[serde(rename = "near-miss")]
    NearMiss,
    /// Unsafe Condition
    #[serde(rename = "unsafe")]
    Unsafe,
}

/// Value Set for QICore Communication Medium (Example)
///
/// System: <http://hl7.org/fhir/qicore-communication-medium>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreCommunicationMedium {
    /// Not Specified
    #[default]
    #[serde(rename = "unspecified")]
    Unspecified,
    /// Telephone
    #[serde(rename = "telephone")]
    Telephone,
    /// Facsimile (fax)
    #[serde(rename = "fax")]
    Fax,
    /// Via medical device
    #[serde(rename = "device")]
    Device,
    /// Video call
    #[serde(rename = "video")]
    Video,
    /// Voice message (voicemail)
    #[serde(rename = "voicemail")]
    Voicemail,
    /// Text message (SMS)
    #[serde(rename = "text")]
    Text,
    /// Social media
    #[serde(rename = "social-media")]
    SocialMedia,
    /// In person
    #[serde(rename = "in-person")]
    InPerson,
    /// Mail (conventional)
    #[serde(rename = "mail")]
    Mail,
    /// Electronic mail (email)
    #[serde(rename = "email")]
    Email,
    /// Patient portal
    #[serde(rename = "portal")]
    Portal,
}

/// Value Set for QICore Condition Criticality (Example)
///
/// System: <http://hl7.org/fhir/qicore-condition-criticality>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreConditionCriticality {
    /// Not Specified
    #[default]
    #[serde(rename = "unspecified")]
    Unspecified,
    /// Expected to Self-Resolve
    #[serde(rename = "self-resolving")]
    SelfResolving,
    /// Controllable
    #[serde(rename = "controllable")]
    Controllable,
    /// Potential loss of function or capacity
    #[serde(rename = "functional-loss")]
    FunctionalLoss,
    /// Life Threatening
    #[serde(rename = "life-threatening")]
    LifeThreatening,
    /// Requires Hospitalization
    #[serde(rename = "requires-hospitalization")]
    RequiresHospitalization,
}

/// The condition, state, or problem that the patient is in or has prior to a
/// therapy or procedure. This captures temporal (temporary circumstances) that
/// have bearing on the data that it qualifies but will not necessarily modify
/// its meaning. Things like 'after activity', 'at rest', or 'post-op'.
///
/// System: <http://hl7.org/fhir/qicore-diagnosticorder-precondition>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreDiagnosticorderPrecondition {
    /// After activity
    #[default]
    #[serde(rename = "1")]
    N1,
    /// At rest
    #[serde(rename = "2")]
    N2,
    /// Post-op
    #[serde(rename = "3")]
    N3,
}

/// Value Set for Military Service (Example)
///
/// System: <http://hl7.org/fhir/qicore-military-service>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreMilitaryService {
    /// Not Indicated
    #[default]
    #[serde(rename = "not-indicated")]
    NotIndicated,
    /// No Military Service
    #[serde(rename = "no-military-service")]
    NoMilitaryService,
    /// Veteran
    #[serde(rename = "veteran")]
    Veteran,
    /// Active Duty
    #[serde(rename = "active-duty")]
    ActiveDuty,
    /// Active Reserve
    #[serde(rename = "active-reserve")]
    ActiveReserve,
    /// Inactive Reserve
    #[serde(rename = "inactive-reserve")]
    InactiveReserve,
}

/// Code for method by which the observation result was validated, e.g., human
/// review, sliding average.
///
/// System: <http://hl7.org/fhir/qicore-observation-verification-method>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QicoreObservationVerificationMethod {
    /// Human review
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Sliding average
    #[serde(rename = "2")]
    N2,
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
}

/// Flags an element as having unlimited repetitions
///
/// System: <http://hl7.org/fhir/question-max-occurs>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionMaxOccurs {
    /// Repeating
    #[default]
    #[serde(rename = "*")]
    Unnamed,
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
}

/// Starter set of user interface controls that might be used when capturing
/// data from Questionnaire questions.
///
/// System: <http://hl7.org/fhir/questionnaire-question-control>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireQuestionControl {
    /// Auto-complete
    #[default]
    #[serde(rename = "autocomplete")]
    Autocomplete,
    /// Drop down
    #[serde(rename = "drop-down")]
    DropDown,
    /// Check-box
    #[serde(rename = "check-box")]
    CheckBox,
    /// Lookup
    #[serde(rename = "lookup")]
    Lookup,
    /// Radio Button
    #[serde(rename = "radio-button")]
    RadioButton,
    /// Slider
    #[serde(rename = "slider")]
    Slider,
    /// Spinner
    #[serde(rename = "spinner")]
    Spinner,
    /// Text Box
    #[serde(rename = "text-box")]
    TextBox,
}

/// Lifecycle status of the questionnaire.
///
/// System: <http://hl7.org/fhir/questionnaire-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum QuestionnaireStatus {
    /// Draft
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// Published
    #[serde(rename = "published")]
    Published,
    /// Retired
    #[serde(rename = "retired")]
    Retired,
}

/// Statement about the degree of clinical certainty that a Specific Substance
/// was the cause of the Manifestation in an reaction event.
///
/// System: <http://hl7.org/fhir/reaction-event-certainty>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReactionEventCertainty {
    /// Unlikely
    #[default]
    #[serde(rename = "unlikely")]
    Unlikely,
    /// Likely
    #[serde(rename = "likely")]
    Likely,
    /// Confirmed
    #[serde(rename = "confirmed")]
    Confirmed,
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

/// This value set is provided as an exemplar. The value set to instantiate
/// this attribute should be drawn from a terminologically robust code system
/// that consists of or contains concepts to support the medication process.
///
/// System: <http://hl7.org/fhir/reason-medication-given>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReasonMedicationGiven {
    /// None
    #[default]
    #[serde(rename = "a")]
    A,
    /// Given as Ordered
    #[serde(rename = "b")]
    B,
    /// Emergency
    #[serde(rename = "c")]
    C,
}

/// This value set is provided as an exemplar. The value set to instantiate
/// this attribute should be drawn from a terminologically robust code system
/// that consists of or contains concepts to support the medication process.
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

/// This value set defines a set of codes that can be used to indicate the
/// meaning/use of a reference range.
///
/// System: <http://hl7.org/fhir/referencerange-meaning>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ReferencerangeMeaning {
    /// Type
    #[default]
    #[serde(rename = "type")]
    Type,
    /// Normal Range
    #[serde(rename = "normal")]
    Normal,
    /// Recommended Range
    #[serde(rename = "recommended")]
    Recommended,
    /// Treatment Range
    #[serde(rename = "treatment")]
    Treatment,
    /// Therapeutic Desired Level
    #[serde(rename = "therapeutic")]
    Therapeutic,
    /// Pre Therapeutic Desired Level
    #[serde(rename = "pre")]
    Pre,
    /// Post Therapeutic Desired Level
    #[serde(rename = "post")]
    Post,
    /// Endocrine
    #[serde(rename = "endocrine")]
    Endocrine,
    /// Pre-Puberty
    #[serde(rename = "pre-puberty")]
    PrePuberty,
    /// Follicular Stage
    #[serde(rename = "follicular")]
    Follicular,
    /// MidCycle
    #[serde(rename = "midcycle")]
    Midcycle,
    /// Luteal
    #[serde(rename = "luteal")]
    Luteal,
    /// Post-Menopause
    #[serde(rename = "postmeopausal")]
    Postmeopausal,
}

/// The status of the referral.
///
/// System: <http://hl7.org/fhir/referralstatus>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Referralstatus {
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
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
}

/// This value set includes the Patient to subscriber relationship codes.
///
/// System: <http://hl7.org/fhir/relationship>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Relationship {
    /// 1
    #[default]
    #[serde(rename = "1")]
    N1,
    /// 2
    #[serde(rename = "2")]
    N2,
    /// 3
    #[serde(rename = "3")]
    N3,
    /// 4
    #[serde(rename = "4")]
    N4,
    /// 5
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

/// One of the resource types defined as part of FHIR.
///
/// System: <http://hl7.org/fhir/resource-types>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceTypes {
    /// Account
    #[default]
    #[serde(rename = "Account")]
    Account,
    /// AllergyIntolerance
    #[serde(rename = "AllergyIntolerance")]
    AllergyIntolerance,
    /// Appointment
    #[serde(rename = "Appointment")]
    Appointment,
    /// AppointmentResponse
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    /// AuditEvent
    #[serde(rename = "AuditEvent")]
    AuditEvent,
    /// Basic
    #[serde(rename = "Basic")]
    Basic,
    /// Binary
    #[serde(rename = "Binary")]
    Binary,
    /// BodySite
    #[serde(rename = "BodySite")]
    BodySite,
    /// Bundle
    #[serde(rename = "Bundle")]
    Bundle,
    /// CarePlan
    #[serde(rename = "CarePlan")]
    CarePlan,
    /// Claim
    #[serde(rename = "Claim")]
    Claim,
    /// ClaimResponse
    #[serde(rename = "ClaimResponse")]
    ClaimResponse,
    /// ClinicalImpression
    #[serde(rename = "ClinicalImpression")]
    ClinicalImpression,
    /// Communication
    #[serde(rename = "Communication")]
    Communication,
    /// CommunicationRequest
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    /// Composition
    #[serde(rename = "Composition")]
    Composition,
    /// ConceptMap
    #[serde(rename = "ConceptMap")]
    ConceptMap,
    /// Condition
    #[serde(rename = "Condition")]
    Condition,
    /// Conformance
    #[serde(rename = "Conformance")]
    Conformance,
    /// Contract
    #[serde(rename = "Contract")]
    Contract,
    /// Coverage
    #[serde(rename = "Coverage")]
    Coverage,
    /// DataElement
    #[serde(rename = "DataElement")]
    DataElement,
    /// DetectedIssue
    #[serde(rename = "DetectedIssue")]
    DetectedIssue,
    /// Device
    #[serde(rename = "Device")]
    Device,
    /// DeviceComponent
    #[serde(rename = "DeviceComponent")]
    DeviceComponent,
    /// DeviceMetric
    #[serde(rename = "DeviceMetric")]
    DeviceMetric,
    /// DeviceUseRequest
    #[serde(rename = "DeviceUseRequest")]
    DeviceUseRequest,
    /// DeviceUseStatement
    #[serde(rename = "DeviceUseStatement")]
    DeviceUseStatement,
    /// DiagnosticOrder
    #[serde(rename = "DiagnosticOrder")]
    DiagnosticOrder,
    /// DiagnosticReport
    #[serde(rename = "DiagnosticReport")]
    DiagnosticReport,
    /// DocumentManifest
    #[serde(rename = "DocumentManifest")]
    DocumentManifest,
    /// DocumentReference
    #[serde(rename = "DocumentReference")]
    DocumentReference,
    /// DomainResource
    #[serde(rename = "DomainResource")]
    DomainResource,
    /// EligibilityRequest
    #[serde(rename = "EligibilityRequest")]
    EligibilityRequest,
    /// EligibilityResponse
    #[serde(rename = "EligibilityResponse")]
    EligibilityResponse,
    /// Encounter
    #[serde(rename = "Encounter")]
    Encounter,
    /// EnrollmentRequest
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    /// EnrollmentResponse
    #[serde(rename = "EnrollmentResponse")]
    EnrollmentResponse,
    /// EpisodeOfCare
    #[serde(rename = "EpisodeOfCare")]
    EpisodeOfCare,
    /// ExplanationOfBenefit
    #[serde(rename = "ExplanationOfBenefit")]
    ExplanationOfBenefit,
    /// FamilyMemberHistory
    #[serde(rename = "FamilyMemberHistory")]
    FamilyMemberHistory,
    /// Flag
    #[serde(rename = "Flag")]
    Flag,
    /// Goal
    #[serde(rename = "Goal")]
    Goal,
    /// Group
    #[serde(rename = "Group")]
    Group,
    /// HealthcareService
    #[serde(rename = "HealthcareService")]
    HealthcareService,
    /// ImagingObjectSelection
    #[serde(rename = "ImagingObjectSelection")]
    ImagingObjectSelection,
    /// ImagingStudy
    #[serde(rename = "ImagingStudy")]
    ImagingStudy,
    /// Immunization
    #[serde(rename = "Immunization")]
    Immunization,
    /// ImmunizationRecommendation
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    /// ImplementationGuide
    #[serde(rename = "ImplementationGuide")]
    ImplementationGuide,
    /// List
    #[serde(rename = "List")]
    List,
    /// Location
    #[serde(rename = "Location")]
    Location,
    /// Media
    #[serde(rename = "Media")]
    Media,
    /// Medication
    #[serde(rename = "Medication")]
    Medication,
    /// MedicationAdministration
    #[serde(rename = "MedicationAdministration")]
    MedicationAdministration,
    /// MedicationDispense
    #[serde(rename = "MedicationDispense")]
    MedicationDispense,
    /// MedicationOrder
    #[serde(rename = "MedicationOrder")]
    MedicationOrder,
    /// MedicationStatement
    #[serde(rename = "MedicationStatement")]
    MedicationStatement,
    /// MessageHeader
    #[serde(rename = "MessageHeader")]
    MessageHeader,
    /// NamingSystem
    #[serde(rename = "NamingSystem")]
    NamingSystem,
    /// NutritionOrder
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    /// Observation
    #[serde(rename = "Observation")]
    Observation,
    /// OperationDefinition
    #[serde(rename = "OperationDefinition")]
    OperationDefinition,
    /// OperationOutcome
    #[serde(rename = "OperationOutcome")]
    OperationOutcome,
    /// Order
    #[serde(rename = "Order")]
    Order,
    /// OrderResponse
    #[serde(rename = "OrderResponse")]
    OrderResponse,
    /// Organization
    #[serde(rename = "Organization")]
    Organization,
    /// Parameters
    #[serde(rename = "Parameters")]
    Parameters,
    /// Patient
    #[serde(rename = "Patient")]
    Patient,
    /// PaymentNotice
    #[serde(rename = "PaymentNotice")]
    PaymentNotice,
    /// PaymentReconciliation
    #[serde(rename = "PaymentReconciliation")]
    PaymentReconciliation,
    /// Person
    #[serde(rename = "Person")]
    Person,
    /// Practitioner
    #[serde(rename = "Practitioner")]
    Practitioner,
    /// Procedure
    #[serde(rename = "Procedure")]
    Procedure,
    /// ProcedureRequest
    #[serde(rename = "ProcedureRequest")]
    ProcedureRequest,
    /// ProcessRequest
    #[serde(rename = "ProcessRequest")]
    ProcessRequest,
    /// ProcessResponse
    #[serde(rename = "ProcessResponse")]
    ProcessResponse,
    /// Provenance
    #[serde(rename = "Provenance")]
    Provenance,
    /// Questionnaire
    #[serde(rename = "Questionnaire")]
    Questionnaire,
    /// QuestionnaireResponse
    #[serde(rename = "QuestionnaireResponse")]
    QuestionnaireResponse,
    /// ReferralRequest
    #[serde(rename = "ReferralRequest")]
    ReferralRequest,
    /// RelatedPerson
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    /// Resource
    #[serde(rename = "Resource")]
    Resource,
    /// RiskAssessment
    #[serde(rename = "RiskAssessment")]
    RiskAssessment,
    /// Schedule
    #[serde(rename = "Schedule")]
    Schedule,
    /// SearchParameter
    #[serde(rename = "SearchParameter")]
    SearchParameter,
    /// Slot
    #[serde(rename = "Slot")]
    Slot,
    /// Specimen
    #[serde(rename = "Specimen")]
    Specimen,
    /// StructureDefinition
    #[serde(rename = "StructureDefinition")]
    StructureDefinition,
    /// Subscription
    #[serde(rename = "Subscription")]
    Subscription,
    /// Substance
    #[serde(rename = "Substance")]
    Substance,
    /// SupplyDelivery
    #[serde(rename = "SupplyDelivery")]
    SupplyDelivery,
    /// SupplyRequest
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    /// TestScript
    #[serde(rename = "TestScript")]
    TestScript,
    /// ValueSet
    #[serde(rename = "ValueSet")]
    ValueSet,
    /// VisionPrescription
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}

/// Codes indicating the type of validation to perform
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
}

/// The kind of response to a message
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

/// The mode of a RESTful conformance statement.
///
/// System: <http://hl7.org/fhir/restful-conformance-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RestfulConformanceMode {
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
    /// validate
    #[serde(rename = "validate")]
    Validate,
    /// conformance
    #[serde(rename = "conformance")]
    Conformance,
    /// transaction
    #[serde(rename = "transaction")]
    Transaction,
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

/// Codes representing the likelihood of a particular outcome in a risk
/// assessment.
///
/// System: <http://hl7.org/fhir/risk-probability>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RiskProbability {
    /// Negligible likelihood
    #[default]
    #[serde(rename = "negligible")]
    Negligible,
    /// Low likelihood
    #[serde(rename = "low")]
    Low,
    /// Moderate likelihood
    #[serde(rename = "moderate")]
    Moderate,
    /// High likelihood
    #[serde(rename = "high")]
    High,
    /// Certain
    #[serde(rename = "certain")]
    Certain,
}

/// This value set includes sample ruleset codes.
///
/// System: <http://hl7.org/fhir/ruleset>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Ruleset {
    /// x12-4010
    #[default]
    #[serde(rename = "x12-4010")]
    X124010,
    /// x12-5010
    #[serde(rename = "x12-5010")]
    X125010,
    /// x12-7010
    #[serde(rename = "x12-7010")]
    X127010,
    /// cdanet-v2
    #[serde(rename = "cdanet-v2")]
    CdanetV2,
    /// cdanet-v4
    #[serde(rename = "cdanet-v4")]
    CdanetV4,
    /// cpha-3
    #[serde(rename = "cpha-3")]
    Cpha3,
}

/// Why an entry is in the result set - whether it's included as a match or
/// because of an _include requirement.
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
}

/// How a search parameter relates to the set of elements returned by
/// evaluating its xpath query.
///
/// System: <http://hl7.org/fhir/search-xpath-usage>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SearchXpathUsage {
    /// Normal
    #[default]
    #[serde(rename = "normal")]
    Normal,
    /// Phonetic
    #[serde(rename = "phonetic")]
    Phonetic,
    /// Nearby
    #[serde(rename = "nearby")]
    Nearby,
    /// Distance
    #[serde(rename = "distance")]
    Distance,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// The type of process where the audit event originated from.
///
/// System: <http://hl7.org/fhir/security-source-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SecuritySourceType {
    /// User Device
    #[default]
    #[serde(rename = "1")]
    N1,
    /// Data Interface
    #[serde(rename = "2")]
    N2,
    /// Web Server
    #[serde(rename = "3")]
    N3,
    /// Application Server
    #[serde(rename = "4")]
    N4,
    /// Database Server
    #[serde(rename = "5")]
    N5,
    /// Security Server
    #[serde(rename = "6")]
    N6,
    /// Network Device
    #[serde(rename = "7")]
    N7,
    /// Network Router
    #[serde(rename = "8")]
    N8,
    /// Other
    #[serde(rename = "9")]
    N9,
}

/// The code(s) that detail the conditions under which the healthcare service
/// is available/offered.
///
/// System: <http://hl7.org/fhir/service-provision-conditions>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ServiceProvisionConditions {
    /// Free
    #[default]
    #[serde(rename = "free")]
    Free,
    /// Discounts Available
    #[serde(rename = "disc")]
    Disc,
    /// Fees apply
    #[serde(rename = "cost")]
    Cost,
}

/// The methods of referral can be used when referring to a specific
/// HealthCareService resource.
///
/// System: <http://hl7.org/fhir/service-referral-method>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ServiceReferralMethod {
    /// Fax
    #[default]
    #[serde(rename = "fax")]
    Fax,
    /// Phone
    #[serde(rename = "phone")]
    Phone,
    /// Secure Messaging
    #[serde(rename = "elec")]
    Elec,
    /// Secure Email
    #[serde(rename = "semail")]
    Semail,
    /// Mail
    #[serde(rename = "mail")]
    Mail,
}

/// The free/busy status of a slot.
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
}

/// A set of generally useful codes defined so they can be included in value
/// sets.
///
/// System: <http://hl7.org/fhir/special-values>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SpecialValues {
    /// true
    #[default]
    #[serde(rename = "true")]
    True,
    /// false
    #[serde(rename = "false")]
    False,
    /// Trace Amount Detected
    #[serde(rename = "trace")]
    Trace,
    /// Sufficient Quantity
    #[serde(rename = "sufficient")]
    Sufficient,
    /// Value Withdrawn
    #[serde(rename = "withdrawn")]
    Withdrawn,
    /// Nil Known
    #[serde(rename = "nil-known")]
    NilKnown,
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
    /// Entered-in-error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

/// Defines the type of structure that a definition is describing.
///
/// System: <http://hl7.org/fhir/structure-definition-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StructureDefinitionKind {
    /// Data Type
    #[default]
    #[serde(rename = "datatype")]
    Datatype,
    /// Resource
    #[serde(rename = "resource")]
    Resource,
    /// Logical Model
    #[serde(rename = "logical")]
    Logical,
}

/// The type of method used to execute a subscription.
///
/// System: <http://hl7.org/fhir/subscription-channel-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptionChannelType {
    /// Rest Hook
    #[default]
    #[serde(rename = "rest-hook")]
    RestHook,
    /// Websocket
    #[serde(rename = "websocket")]
    Websocket,
    /// Email
    #[serde(rename = "email")]
    Email,
    /// SMS
    #[serde(rename = "sms")]
    Sms,
    /// Message
    #[serde(rename = "message")]
    Message,
}

/// The status of a subscription.
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
}

/// Tags to put on a resource after subscriptions have been sent.
///
/// System: <http://hl7.org/fhir/subscription-tag>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubscriptionTag {
    /// Queued
    #[default]
    #[serde(rename = "queued")]
    Queued,
    /// Delivered
    #[serde(rename = "delivered")]
    Delivered,
}

/// Substance category codes
///
/// System: <http://hl7.org/fhir/substance-category>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SubstanceCategory {
    /// Allergen
    #[default]
    #[serde(rename = "allergen")]
    Allergen,
    /// Biological Substance
    #[serde(rename = "biological")]
    Biological,
    /// Body Substance
    #[serde(rename = "body")]
    Body,
    /// Chemical
    #[serde(rename = "chemical")]
    Chemical,
    /// Dietary Substance
    #[serde(rename = "food")]
    Food,
    /// Drug or Medicament
    #[serde(rename = "drug")]
    Drug,
    /// Material
    #[serde(rename = "material")]
    Material,
}

/// This value sets refers to a specific supply item.
///
/// System: <http://hl7.org/fhir/supply-item-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplyItemType {
    /// Medication
    #[default]
    #[serde(rename = "medication")]
    Medication,
    /// Device
    #[serde(rename = "device")]
    Device,
}

/// This value sets refers to a Category of supply.
///
/// System: <http://hl7.org/fhir/supply-kind>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplyKind {
    /// Central Supply
    #[default]
    #[serde(rename = "central")]
    Central,
    /// Non-Stock
    #[serde(rename = "nonstock")]
    Nonstock,
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
}

/// Why the supply item was requested
///
/// System: <http://hl7.org/fhir/supplyrequest-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplyrequestReason {
    /// Patient Care
    #[default]
    #[serde(rename = "patient-care")]
    PatientCare,
    /// Ward Stock
    #[serde(rename = "ward-stock")]
    WardStock,
}

/// Status of the supply request
///
/// System: <http://hl7.org/fhir/supplyrequest-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SupplyrequestStatus {
    /// Requested
    #[default]
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "completed")]
    Completed,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// This value set defines a set of codes that are used to indicate the
/// supported operations of a testing engine or tool.
///
/// System: <http://hl7.org/fhir/testscript-operation-codes>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TestscriptOperationCodes {
    /// Read
    #[default]
    #[serde(rename = "read")]
    Read,
    /// Version Read
    #[serde(rename = "vread")]
    Vread,
    /// Update
    #[serde(rename = "update")]
    Update,
    /// Delete
    #[serde(rename = "delete")]
    Delete,
    /// History
    #[serde(rename = "history")]
    History,
    /// Create
    #[serde(rename = "create")]
    Create,
    /// Search
    #[serde(rename = "search")]
    Search,
    /// Transaction
    #[serde(rename = "transaction")]
    Transaction,
    /// Conformance
    #[serde(rename = "conformance")]
    Conformance,
    /// $closure
    #[serde(rename = "closure")]
    Closure,
    /// $document
    #[serde(rename = "document")]
    Document,
    /// $everything
    #[serde(rename = "everything")]
    Everything,
    /// $expand
    #[serde(rename = "expand")]
    Expand,
    /// $find
    #[serde(rename = "find")]
    Find,
    /// $lookup
    #[serde(rename = "lookup")]
    Lookup,
    /// $meta
    #[serde(rename = "meta")]
    Meta,
    /// $meta-add
    #[serde(rename = "meta-add")]
    MetaAdd,
    /// $meta-delete
    #[serde(rename = "meta-delete")]
    MetaDelete,
    /// $populate
    #[serde(rename = "populate")]
    Populate,
    /// $process-message
    #[serde(rename = "process-message")]
    ProcessMessage,
    /// $questionnaire
    #[serde(rename = "questionnaire")]
    Questionnaire,
    /// $translate
    #[serde(rename = "translate")]
    Translate,
    /// $validate
    #[serde(rename = "validate")]
    Validate,
    /// $validate-code
    #[serde(rename = "validate-code")]
    ValidateCode,
}

/// Code for a known / defined timing pattern.
///
/// System: <http://hl7.org/fhir/timing-abbreviation>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TimingAbbreviation {
    /// BID
    #[default]
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
}

/// A code that indicates how transactions are supported.
///
/// System: <http://hl7.org/fhir/transaction-mode>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TransactionMode {
    /// None
    #[default]
    #[serde(rename = "not-supported")]
    NotSupported,
    /// Batches supported
    #[serde(rename = "batch")]
    Batch,
    /// Transactions Supported
    #[serde(rename = "transaction")]
    Transaction,
    /// Batches & Transactions
    #[serde(rename = "both")]
    Both,
}

/// A code that indicates whether an application accepts unknown elements or
/// extensions when reading resources.
///
/// System: <http://hl7.org/fhir/unknown-content-code>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UnknownContentCode {
    /// Neither Elements or Extensions
    #[default]
    #[serde(rename = "no")]
    No,
    /// Unknown Extensions
    #[serde(rename = "extensions")]
    Extensions,
    /// Unknown Elements
    #[serde(rename = "elements")]
    Elements,
    /// Unknown Elements and Extensions
    #[serde(rename = "both")]
    Both,
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

/// US Realm ambulatory care event codes that correspond the Use cases outlined
/// in the USLabOrder Implementation Guide.
///
/// System: <http://hl7.org/fhir/uslab-event>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum UslabEvent {
    /// new request
    #[default]
    #[serde(rename = "new-request")]
    NewRequest,
    /// add-on request
    #[serde(rename = "add-on-request")]
    AddOnRequest,
    /// orderer cancel
    #[serde(rename = "orderer-cancel")]
    OrdererCancel,
    /// laboratory cancel
    #[serde(rename = "lab-cancel")]
    LabCancel,
}

/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts
/// to support the medication process, in particular the process and reasons
/// for dispensing. This value set is provided as a suggestive example.
///
/// System: <http://hl7.org/fhir/vaccination-protocol-dose-status>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VaccinationProtocolDoseStatus {
    /// Counts
    #[default]
    #[serde(rename = "count")]
    Count,
    /// Does not Count
    #[serde(rename = "nocount")]
    Nocount,
}

/// The value set to instantiate this attribute should be drawn from a
/// terminologically robust code system that consists of or contains concepts
/// to support the medication process, in particular the process and reasons
/// for dispensing. This value set is provided as a suggestive example.
///
/// System: <http://hl7.org/fhir/vaccination-protocol-dose-status-reason>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VaccinationProtocolDoseStatusReason {
    /// Adverse storage condition
    #[default]
    #[serde(rename = "advstorage")]
    Advstorage,
    /// Cold chain break
    #[serde(rename = "coldchbrk")]
    Coldchbrk,
    /// Expired lot
    #[serde(rename = "explot")]
    Explot,
    /// Administered outside recommended schedule
    #[serde(rename = "outsidesched")]
    Outsidesched,
    /// Product recall
    #[serde(rename = "prodrecall")]
    Prodrecall,
}

/// The Digital Signature Purposes, an indication of the reason an entity signs
/// a document. This is included in the signed information and can be used when
/// determining accountability for various actions concerning the document.
/// Examples include: author, transcriptionist/recorder, and witness.
///
/// System: <http://hl7.org/fhir/valueset-signature-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ValuesetSignatureType {
    /// AuthorID
    #[default]
    #[serde(rename = "1.2.840.10065.1.12.1.1")]
    N128401006511211,
    /// Co-AuthorID
    #[serde(rename = "1.2.840.10065.1.12.1.2")]
    N128401006511212,
    /// Co-Participated
    #[serde(rename = "1.2.840.10065.1.12.1.3")]
    N128401006511213,
    /// Transcriptionist
    #[serde(rename = "1.2.840.10065.1.12.1.4")]
    N128401006511214,
    /// Verification
    #[serde(rename = "1.2.840.10065.1.12.1.5")]
    N128401006511215,
    /// Validation
    #[serde(rename = "1.2.840.10065.1.12.1.6")]
    N128401006511216,
    /// Consent
    #[serde(rename = "1.2.840.10065.1.12.1.7")]
    N128401006511217,
    /// Witness
    #[serde(rename = "1.2.840.10065.1.12.1.8")]
    N128401006511218,
    /// Event-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.9")]
    N128401006511219,
    /// Identity-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.10")]
    N1284010065112110,
    /// Consent-Witness
    #[serde(rename = "1.2.840.10065.1.12.1.11")]
    N1284010065112111,
    /// Interpreter
    #[serde(rename = "1.2.840.10065.1.12.1.12")]
    N1284010065112112,
    /// Review
    #[serde(rename = "1.2.840.10065.1.12.1.13")]
    N1284010065112113,
    /// Source
    #[serde(rename = "1.2.840.10065.1.12.1.14")]
    N1284010065112114,
    /// Addendum
    #[serde(rename = "1.2.840.10065.1.12.1.15")]
    N1284010065112115,
    /// Administrative
    #[serde(rename = "1.2.840.10065.1.12.1.16")]
    N1284010065112116,
    /// Timestamp
    #[serde(rename = "1.2.840.10065.1.12.1.17")]
    N1284010065112117,
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

/// The kind of relationship between two XDS documents.
///
/// System: <http://hl7.org/fhir/xds-relationship-type>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum XdsRelationshipType {
    /// APND
    #[default]
    #[serde(rename = "APND")]
    Apnd,
    /// RPLC
    #[serde(rename = "RPLC")]
    Rplc,
    /// XFRM
    #[serde(rename = "XFRM")]
    Xfrm,
    /// XFRM_RPLC
    #[serde(rename = "XFRM_RPLC")]
    XfrmRplc,
    /// Signs
    #[serde(rename = "signs")]
    Signs,
}
