//! CanonicalResource
//!
//! ## UML
//!
//! <https://build.fhir.org/uml.html>
//!
//! CanonicalResource «Interface»:
//!
//! - url : uri [0..1]
//! - identifier : Identifier [0..*]
//! - version: Option<types::string::String>,
//! - versionAlgorithm[x] : DataType [0..1] « string|Coding;
//! - VersionAlgorithm+ »
//! - name: Option<types::string::String>, « C »
//! - title: Option<types::string::String>,
//! - status : code [1..1] « PublicationStatus! »
//! - experimental : boolean [0..1]
//! - date : dateTime [0..1]
//! - publisher: Option<types::string::String>,
//! - contact : ContactDetail [0..*]
//! - description : markdown [0..1]
//! - useContext : UsageContext [0..*]
//! - jurisdiction : CodeableConcept [0..*] « JurisdictionValueSet+ »
//! - purpose : markdown [0..1]
//! - copyright : markdown [0..1]
//! - copyrightLabel: Option<types::string::String>,

#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CanonicalResource {
    /// # url
    ///
    /// ## Description
    ///
    /// The `url` attribute represents the canonical URL that uniquely
    /// identifies a FHIR resource such as a StructureDefinition, ValueSet,
    /// CodeSystem, or CapabilityStatement. This URL serves as a global
    /// identifier that remains constant across different versions of the
    /// resource and provides a stable reference for external systems to
    /// identify and reference the resource.
    ///
    /// ## Purpose
    ///
    /// The `url` exists to provide a globally unique, version-independent
    /// identifier for FHIR resources. This enables:
    ///
    /// - Stable referencing of resources across different FHIR implementations
    /// - Version management while maintaining resource identity
    /// - Canonical identification for resource dependencies and imports  
    /// - Support for resource discovery and resolution mechanisms
    /// - Consistent resource identification in distributed healthcare networks
    ///
    /// ## Usage
    ///
    /// Use the `url` attribute when:
    ///
    /// - Defining canonical resources like StructureDefinitions, ValueSets, or
    ///   CodeSystems
    /// - Creating stable references that persist across resource versions
    /// - Implementing resource registries or repositories
    /// - Supporting resource discovery and dependency resolution
    /// - Establishing canonical URLs for organizational FHIR artifacts
    ///
    /// The `url` should follow URI format conventions and be resolvable when
    /// possible to aid in resource discovery.
    ///
    /// ## Data Type
    ///
    /// **uri** - A Uniform Resource Identifier following RFC 3986:
    /// - Must be an absolute URI with scheme (typically http or https)
    /// - Should be unique globally to avoid conflicts
    /// - Recommended to use organization's domain for uniqueness
    /// - May include path components to organize related resources
    /// - Should remain stable even as resource content evolves
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes for canonical resources (StructureDefinition,
    ///   ValueSet, CodeSystem, etc.)
    /// - **Cardinality**: 1..1 (exactly one occurrence when present)
    /// - **Format**: Must be a valid absolute URI
    /// - **Uniqueness**: Should be globally unique within the resource type
    /// - **Stability**: Should remain constant across resource versions
    /// - **Resolvability**: Should ideally be resolvable to the actual resource
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete
    /// StructureDefinition resource demonstrating the canonical URL usage in
    /// various contexts.
    ///
    /// ## Related Keys
    ///
    /// - `version` - Business version that works with url to create
    ///   version-specific references
    /// - `name` - Machine-readable name often derived from the url path
    /// - `identifier` - Additional identifiers that may complement the
    ///   canonical url
    /// - `baseDefinition` - References other resources using their canonical
    ///   urls
    /// - `derivation` - Indicates relationship to base definitions via their
    ///   urls
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and the
    /// canonical URI data type definition.
    ///
    pub url: Option<types::Uri>,

    /// # identifier
    ///
    /// ## Description
    ///
    /// The `identifier` key is used throughout FHIR R5 resources to provide a
    /// unique identification for resources, elements, or entities. Identifiers
    /// are used to maintain consistent references across systems and enable
    /// interoperability by providing stable, unique identifiers that persist
    /// across systems.
    ///
    /// ## Purpose
    ///
    /// - Provides unique identification for resources and entities
    /// - Enables consistent referencing across different systems
    /// - Supports resource matching and deduplication
    /// - Facilitates interoperability between healthcare systems
    /// - Maintains stable identifiers independent of resource IDs
    ///
    /// ## Usage
    ///
    /// The `identifier` appears in:
    ///
    /// - **Most FHIR Resources**: As a primary identification mechanism
    /// - **Patient**: Medical record numbers, SSN, insurance IDs
    /// - **Practitioner**: License numbers, provider IDs
    /// - **Organization**: Tax ID, accreditation numbers
    /// - **Observation**: Lab order numbers, specimen IDs
    ///
    /// ## Data Type
    ///
    /// **Identifier** - A complex data type containing:
    ///
    /// - `use` - Purpose of the identifier (usual, official, temp, secondary)
    /// - `type` - Coded type of identifier
    /// - `system` - Namespace for the identifier value
    /// - `value` - The actual identifier value
    /// - `period` - Time period when identifier is valid
    /// - `assigner` - Organization that assigned the identifier
    ///
    /// ## Constraints
    ///
    /// - System and value combination should be unique within the namespace
    /// - System should be a valid URI identifying the namespace
    /// - Value must be provided if identifier is present
    /// - Type should align with the identifier's purpose
    /// - Multiple identifiers can be provided for a single resource
    ///
    /// ## Examples
    ///
    /// ### Basic Patient Medical Record Number
    ///
    /// ```json
    /// {
    ///   "identifier": [
    ///     {
    ///       "use": "official",
    ///       "type": {
    ///         "coding": [
    ///           {
    ///             "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
    ///             "code": "MR",
    ///             "display": "Medical Record Number"
    ///           }
    ///         ]
    ///       },
    ///       "system": "http://hospital.example.org/identifiers/mrn",
    ///       "value": "12345678"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Multiple Identifier Types
    ///
    /// ```json
    /// {
    ///   "identifier": [
    ///     {
    ///       "use": "official",
    ///       "type": {
    ///         "coding": [
    ///           {
    ///             "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
    ///             "code": "MR",
    ///             "display": "Medical Record Number"
    ///           }
    ///         ]
    ///       },
    ///       "system": "http://hospital.example.org/identifiers/mrn",
    ///       "value": "MRN123456",
    ///       "assigner": {
    ///         "display": "Example Hospital"
    ///       }
    ///     },
    ///     {
    ///       "use": "secondary",
    ///       "type": {
    ///         "coding": [
    ///           {
    ///             "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
    ///             "code": "SS",
    ///             "display": "Social Security Number"
    ///           }
    ///         ]
    ///       },
    ///       "system": "http://hl7.org/fhir/sid/us-ssn",
    ///       "value": "123-45-6789"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `id` - Logical resource identifier
    /// - `system` - Namespace for identifier values
    /// - `value` - The actual identifier string
    /// - `type` - Coded type of identifier
    /// - `use` - Purpose classification
    /// - `assigner` - Organization that issued identifier
    /// - `reference` - References using identifiers
    ///
    /// ## Specification Reference
    ///
    /// - **FHIR R5 Specification**: [Identifier Data
    ///   Type](http://hl7.org/fhir/R5/datatypes.html#Identifier)
    /// - **Identifier Types**: [Identifier Type
    ///   Codes](http://hl7.org/fhir/R5/valueset-identifier-type.html)
    /// - **Section**: Used across multiple resource types
    /// - **Context**: Primary identification mechanism in FHIR resources
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// # version
    ///
    /// ## Description
    ///
    /// The `version` attribute represents the business version identifier of a
    /// FHIR resource, particularly for canonical resources like
    /// StructureDefinitions, ValueSets, CodeSystems, and CapabilityStatements.
    /// This version works in conjunction with the canonical `url` to provide
    /// precise, version-specific identification of resources. Unlike technical
    /// versioning (like `meta.versionId`), the business version reflects
    /// meaningful changes in the resource's content, semantics, or clinical
    /// significance.
    ///
    /// ## Purpose
    ///
    /// The `version` exists to support:
    ///
    /// - Business-level versioning that reflects meaningful content changes
    /// - Version-specific resource references and dependencies
    /// - Change management and compatibility tracking across resource evolution
    /// - Support for multiple concurrent versions of the same conceptual
    ///   resource
    /// - Implementation guidance for version compatibility and migration
    /// - Regulatory and compliance requirements for versioned healthcare
    ///   standards
    ///
    /// ## Usage
    ///
    /// Use the `version` attribute when:
    ///
    /// - Publishing canonical resources that may evolve over time
    /// - Supporting multiple concurrent versions of clinical standards
    /// - Implementing version-aware resource resolution and validation
    /// - Managing dependencies between versioned FHIR artifacts
    /// - Providing clear change tracking for clinical decision support rules
    /// - Supporting regulatory requirements for versioned healthcare content
    ///
    /// Version values should follow semantic versioning principles where
    /// appropriate, using formats like "1.0.0" or "2024.1" depending on
    /// organizational conventions.
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable version identifier:
    ///
    /// - Commonly follows semantic versioning (e.g., "1.0.0", "2.1.3")
    /// - May use date-based versioning (e.g., "2024.08", "20240815")
    /// - Can include pre-release indicators (e.g., "1.0.0-beta", "2.0.0-rc1")
    /// - Should be consistently formatted within an organization
    /// - Must be comparable to determine version precedence
    /// - Should reflect the significance of changes between versions
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional for most resources, strongly recommended for
    ///   canonical resources
    /// - **Cardinality**: 0..1 (at most one occurrence)
    /// - **Format**: No strict format requirements, but should be consistent
    ///   and comparable
    /// - **Uniqueness**: Should be unique within the context of the same
    ///   canonical URL
    /// - **Ordering**: Should allow for logical ordering and comparison of
    ///   versions
    /// - **Stability**: Should not change once a version is published and in
    ///   use
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete examples
    /// demonstrating version usage in StructureDefinition, ValueSet, and
    /// CapabilityStatement resources with different versioning approaches.
    ///
    /// ## Related Keys
    ///
    /// - `url` - Canonical identifier that works with version to provide
    ///   precise resource identification
    /// - `name` - Machine-readable identifier that may reflect version in its
    ///   naming
    /// - `title` - Human-readable title that may include version information
    ///   for clarity
    /// - `status` - Indicates lifecycle status which relates to version
    ///   maturity
    /// - `date` - Publication date that often corresponds to version release
    ///   date
    /// - `publisher` - Entity responsible for version management and release
    /// - `experimental` - Flag indicating if this version is still experimental
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and
    /// versioning guidelines in the FHIR specification.
    ///
    pub version: Option<types::String>,

    pub version_algorithm: types::String, // « string | Coding; VersionAlgorithm+ »

    /// # name
    ///
    /// ## Description
    ///
    /// The `name` attribute represents a human-readable identifier or label
    /// used throughout FHIR R5 resources to provide meaningful, user-friendly
    /// text for various elements. It serves as the primary textual identifier
    /// that humans use to recognize, reference, and work with healthcare
    /// concepts, entities, and data elements.
    ///
    /// ## Purpose
    ///
    /// The `name` exists to provide human-readable identification across FHIR
    /// resources, enabling:
    ///
    /// - User-friendly display of resource information
    /// - Searchable and recognizable labels for healthcare entities
    /// - Support for multiple naming conventions and languages
    /// - Clear identification in user interfaces and documentation
    /// - Meaningful references in clinical workflows and communications
    ///
    /// ## Usage
    ///
    /// Use the `name` attribute when:
    ///
    /// - Defining patient names with proper structure (family, given names)
    /// - Naming healthcare providers, organizations, and facilities
    /// - Labeling medication and substance names
    /// - Creating human-readable identifiers for plans and protocols
    /// - Providing searchable names for locations and services
    /// - Establishing clear references for coded concepts
    ///
    /// Names should be accurate, culturally appropriate, and suitable for the
    /// intended use context.
    ///
    /// ## Data Type
    ///
    /// **varies by context** - Common patterns include:
    ///
    /// - **HumanName** - Structured representation for person names (family,
    ///   given, prefix, suffix)
    /// - **string** - Simple text name for organizations, medications, and
    ///   other entities
    /// - **Array of HumanName** - Multiple name representations with different
    ///   uses
    /// - **Complex structures** - May include use codes, periods of validity,
    ///   and preferred flags
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - often required for key identifying
    ///   elements
    /// - **Cardinality**: Varies by context (0..1, 0..*, or 1..1)
    /// - **Format**: Should follow cultural and linguistic conventions
    /// - **Validation**: May include format checking for structured names
    /// - **Uniqueness**: Not required to be unique across systems
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a comprehensive example
    /// showing various `name` attribute uses across different FHIR resources
    /// and contexts.
    ///
    /// ## Related Keys
    ///
    /// - `family` - Family name component in HumanName structures
    /// - `given` - Given name components in HumanName structures
    /// - `use` - Context or purpose of the name (official, usual, nickname)
    /// - `text` - Complete name as a single string
    /// - `period` - Time period when the name was/is in use
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for HumanName data types, naming
    /// conventions, and context-specific name requirements.
    ///
    pub name: Option<types::String>,

    /// # title
    ///
    /// ## Description
    ///
    /// The `title` attribute provides a human-readable, descriptive name for a
    /// FHIR resource that is intended for display to end users. Unlike the
    /// `name` attribute which is machine-readable and constrained to specific
    /// naming conventions, the `title` serves as a user-friendly label that can
    /// include spaces, punctuation, and formatting that makes it more
    /// accessible to healthcare professionals and patients.
    ///
    /// ## Purpose
    ///
    /// The `title` exists to provide a clear, descriptive display name that:
    ///
    /// - Offers immediate recognition and understanding for human users
    /// - Supports user interface display requirements with formatted text
    /// - Provides context and meaning beyond technical identifiers
    /// - Enables better user experience in clinical applications
    /// - Supports internationalization and localization needs
    /// - Complements machine-readable names with human-readable descriptions
    ///
    /// ## Usage
    ///
    /// Use the `title` attribute when:
    ///
    /// - Creating resources that will be displayed in user interfaces
    /// - Providing descriptive names for StructureDefinitions, ValueSets, or
    ///   CodeSystems
    /// - Supporting clinical decision support tools that need clear labels
    /// - Implementing patient-facing applications requiring readable names
    /// - Creating documentation or reports that need descriptive resource names
    /// - Building applications that require both technical and display names
    ///
    /// The `title` should be concise but descriptive, avoiding overly technical
    /// jargon when possible.
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable string value:
    ///
    /// - Can contain spaces, punctuation, and special characters
    /// - Should be reasonably concise while remaining descriptive
    /// - May include formatting for better readability
    /// - Can support multiple languages through internationalization
    /// - Should avoid excessive length that impacts display
    /// - May include version indicators or qualifiers for clarity
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional for most resources, recommended for canonical
    ///   resources
    /// - **Cardinality**: 0..1 (at most one occurrence)
    /// - **Length**: Should be practical for display purposes (typically under
    ///   200 characters)
    /// - **Format**: Free-text string without specific format restrictions
    /// - **Uniqueness**: Not required to be unique, but should be distinctive
    ///   within context
    /// - **Language**: Should match the language of the resource or be
    ///   appropriately localized
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition and ValueSet resources demonstrating the title usage
    /// in various clinical contexts.
    ///
    /// ## Related Keys
    ///
    /// - `name` - Machine-readable identifier that complements the
    ///   human-readable title
    /// - `description` - Longer narrative text that provides additional detail
    ///   beyond the title
    /// - `publisher` - Entity responsible for the resource, often reflected in
    ///   professional titles
    /// - `status` - Indicates whether the titled resource is active and
    ///   available for use
    /// - `version` - Business version that may be referenced in title for
    ///   version-specific resources
    /// - `url` - Canonical identifier that the title makes human-readable
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and string
    /// data type definitions.
    ///
    pub title: Option<types::string::String>,

    /// # status
    ///
    /// ## Description
    ///
    /// The `status` attribute indicates the current state of a resource within
    /// its workflow or lifecycle. It provides important information about
    /// whether the resource is active, completed, cancelled, or in some other
    /// defined state according to FHIR R5 specifications.
    ///
    /// ## Purpose
    ///
    /// The `status` element serves to:
    ///
    /// - Indicate the current workflow state of the resource
    /// - Support workflow management and business process automation
    /// - Enable filtering and querying based on resource state
    /// - Prevent inappropriate use of outdated or cancelled information
    /// - Support audit trails and state transition tracking
    /// - Ensure clinical safety by clearly indicating resource validity
    ///
    /// ## Usage
    ///
    /// Use the `status` attribute to:
    ///
    /// - Track the lifecycle state of clinical and administrative resources
    /// - Filter resources based on their current state
    /// - Implement workflow rules and business logic
    /// - Ensure clinical safety by checking resource status before use
    /// - Support reporting and analytics based on resource states
    ///
    /// The specific status values and their meanings vary by resource type, but
    /// common patterns include active/inactive, draft/final, and various
    /// workflow-specific states.
    ///
    /// ## Data Type
    ///
    /// **code** - A string value from a predefined set of status codes specific
    /// to each resource type. Common status patterns include:
    ///
    /// - **Workflow states**: draft, active, inactive, suspended, completed,
    ///   cancelled
    /// - **Publication states**: draft, published, retired
    /// - **Request states**: planned, requested, received, accepted,
    ///   in-progress, completed, suspended, rejected, failed
    /// - **Event states**: preparation, in-progress, completed,
    ///   entered-in-error
    ///
    /// ## Constraints
    ///
    /// - **Required**: Usually required - Most FHIR resources with workflow
    ///   implications require a status
    /// - **Cardinality**: 0..1 or 1..1 (depending on resource type)
    /// - **Fixed ValueSet**: Must be from the specific ValueSet defined for
    ///   each resource type
    /// - **Modifies Meaning**: Status often affects the interpretation of the
    ///   entire resource
    /// - **Immutability**: Some status transitions may be irreversible (e.g.,
    ///   completed to cancelled)
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating the use of the `status` attribute across different
    /// resource types including ServiceRequest, DiagnosticReport, and
    /// MedicationRequest.
    ///
    /// ## Related Keys
    ///
    /// - `meta.lastUpdated` - When the status was last changed
    /// - Various date/time fields that may be associated with status changes
    /// - `extension` - May contain additional status-related information
    /// - Resource-specific elements that depend on the current status
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details on status values
    /// for specific resource types, refer to the official FHIR R5 documentation
    /// and the respective ValueSets defined for each resource's status element.
    ///
    pub status: types::code::Code, // « PublicationStatus! »

    /// # experimental
    ///
    /// ## Description
    ///
    /// The `experimental` field indicates whether a FHIR resource is intended
    /// for testing, experimentation, or preliminary use rather than production
    /// deployment. It serves as a warning flag for implementers about the
    /// stability and maturity of the resource.
    ///
    /// ## Purpose
    ///
    /// - Indicate developmental or experimental status
    /// - Warn implementers about potential instability
    /// - Support graduated resource development processes
    /// - Enable safe testing and validation environments
    /// - Distinguish between production-ready and experimental content
    ///
    /// ## Usage
    ///
    /// The `experimental` field is commonly used in:
    /// - **StructureDefinition**: Experimental profiles and extensions
    /// - **ValueSet**: Draft or experimental value sets
    /// - **CodeSystem**: Experimental code systems
    /// - **ImplementationGuide**: Pilot or experimental implementation guides
    /// - **CapabilityStatement**: Experimental server capabilities
    ///
    /// ## Data Type
    ///
    /// - **Type**: boolean
    /// - **Cardinality**: 0..1
    /// - **Values**:
    ///   - `true`: Resource is experimental
    ///   - `false`: Resource is not experimental (production-ready)
    ///
    /// ## Constraints
    ///
    /// - Should accurately reflect the resource's development status
    /// - Must be consistent with resource lifecycle management
    /// - Should be updated as resource matures
    /// - Must consider impact on dependent resources
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` for practical usage examples.
    ///
    /// ## Related Keys
    ///
    /// - `status`: Resource lifecycle status
    /// - `version`: Resource version identifier
    /// - `date`: Resource modification date
    /// - `publisher`: Organization responsible for resource
    /// - `jurisdiction`: Applicable jurisdictions
    ///
    /// ## Specification Reference
    ///
    /// - [FHIR R5 Resource
    ///   Metadata](https://hl7.org/fhir/R5/resource.html#meta)
    /// - [FHIR R5 Conformance
    ///   Resources](https://hl7.org/fhir/R5/conformance-module.html)
    /// - [FHIR R5 Resource Lifecycle](https://hl7.org/fhir/R5/lifecycle.html)
    ///
    pub experimental: Option<types::Boolean>,

    /// # date
    ///
    /// ## Description
    ///
    /// The `date` attribute represents the publication, creation, revision, or
    /// last update date of a FHIR resource. This timestamp provides crucial
    /// information about when the resource was published or last modified,
    /// enabling users to assess the currency and relevance of the content,
    /// track version history, and make informed decisions about resource usage.
    ///
    /// ## Purpose
    ///
    /// The `date` exists to provide temporal context for FHIR resources. This
    /// enables:
    ///
    /// - Assessment of resource currency and relevance
    /// - Version control and change tracking
    /// - Implementation of data retention and refresh policies
    /// - Support for temporal queries and filtering
    /// - Compliance with regulatory requirements for data freshness
    /// - Trust assessment based on recency of updates
    ///
    /// ## Usage
    ///
    /// Use the `date` attribute when:
    ///
    /// - Publishing or updating canonical resources like StructureDefinitions,
    ///   ValueSets
    /// - Creating clinical resources that need temporal context
    /// - Implementing version control and change management systems
    /// - Supporting queries that filter resources by publication or update date
    /// - Meeting regulatory requirements for date documentation
    /// - Enabling cache invalidation and refresh mechanisms
    ///
    /// The date should represent the actual publication or last significant
    /// update of the resource content.
    ///
    /// ## Data Type
    ///
    /// **dateTime** - A date and optionally time following ISO 8601 format:
    ///
    /// - Format: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS+TZ
    /// - Time zone specification is recommended for precision
    /// - Can be partial (year, year-month, or full date)
    /// - Should use UTC or explicitly specify time zone offset
    /// - Precision should match the granularity needed for the use case
    ///
    /// ## Constraints
    ///
    /// - **Required**: Recommended for canonical resources, optional for others
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Format**: Must follow valid dateTime format per FHIR specification
    /// - **Precision**: Should match the appropriate level of granularity
    /// - **Consistency**: Should be updated when resource content changes
    ///   significantly
    /// - **Accuracy**: Should reflect actual publication or modification dates
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete CodeSystem
    /// resource demonstrating the date attribute in a terminology management
    /// context.
    ///
    /// ## Related Keys
    ///
    /// - `lastReviewDate` - Date when content was last reviewed for accuracy
    /// - `effectivePeriod` - Period when the resource is intended to be in use
    /// - `approvalDate` - Date when content was approved for publication
    /// - `meta.lastUpdated` - System-generated timestamp of last technical
    ///   update
    /// - `version` - Business version that may correlate with publication dates
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for dateTime data type and metadata
    /// requirements for canonical resources.
    ///
    pub date: Option<types::DateTime>,

    /// # publisher
    ///
    /// ## Description
    ///
    /// The `publisher` attribute identifies the organization, individual, or
    /// entity responsible for publishing and maintaining a FHIR resource. This
    /// field provides transparency about the source and authority behind the
    /// resource, enabling users to understand who has created, endorsed, or
    /// taken responsibility for the content and its quality.
    ///
    /// ## Purpose
    ///
    /// The `publisher` exists to establish accountability and authority for
    /// FHIR resources. This enables:
    ///
    /// - Clear identification of who is responsible for resource content and
    ///   maintenance
    /// - Trust assessment based on the publisher's reputation and authority
    /// - Contact point identification for questions or issues about the
    ///   resource
    /// - Support for governance and quality assurance processes
    /// - Attribution for intellectual property and licensing considerations
    ///
    /// ## Usage
    ///
    /// Use the `publisher` attribute when:
    ///
    /// - Publishing canonical resources like StructureDefinitions, ValueSets,
    ///   or Implementation Guides
    /// - Establishing organizational ownership and responsibility for resources
    /// - Supporting governance frameworks that require publisher identification
    /// - Creating resources that need clear attribution for trust and authority
    /// - Implementing resource catalogs that organize content by publisher
    ///
    /// The publisher should be clearly identifiable and ideally contactable for
    /// resource-related inquiries.
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable text string identifying the publisher:
    ///
    /// - Should be the official name of the organization or individual
    /// - May include department or division information for clarity
    /// - Should be consistent across related resources from the same publisher
    /// - Avoid abbreviations that might not be universally understood
    /// - Can include descriptive text to clarify the publisher's role
    ///
    /// ## Constraints
    ///
    /// - **Required**: Strongly recommended for canonical resources, optional
    ///   for others
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Format**: Free text, but should follow consistent naming conventions
    /// - **Length**: Should be concise but descriptive enough to identify the
    ///   publisher
    /// - **Consistency**: Should be consistent across resources from the same
    ///   publisher
    /// - **Authority**: Should represent the actual publishing authority, not
    ///   just implementers
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete ValueSet
    /// resource demonstrating the publisher attribute in a clinical terminology
    /// context.
    ///
    /// ## Related Keys
    ///
    /// - `contact` - Detailed contact information that complements the
    ///   publisher identification
    /// - `author` - Individual contributors who may be different from the
    ///   publisher
    /// - `editor` - Those responsible for editorial oversight, may work for the
    ///   publisher
    /// - `reviewer` - Those who have reviewed the content on behalf of the
    ///   publisher
    /// - `endorser` - Organizations that have endorsed the publisher's work
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and metadata
    /// requirements.
    ///
    pub publisher: Option<types::String>,

    /// # contact
    ///
    /// ## Description
    ///
    /// The `contact` attribute provides contact information for individuals or
    /// organizations associated with a FHIR resource. This includes names,
    /// roles, telecommunications details (phone, email, fax), and other means
    /// of communication. In canonical resources like StructureDefinitions and
    /// ValueSets, contact information typically identifies maintainers,
    /// authors, or support personnel who can provide assistance with the
    /// resource. In clinical resources, it may represent care team members,
    /// emergency contacts, or administrative contacts.
    ///
    /// ## Purpose
    ///
    /// The `contact` exists to:
    ///
    /// - Provide communication channels for resource maintainers and support
    ///   personnel
    /// - Enable stakeholder identification for canonical resources and
    ///   implementation guides  
    /// - Support collaboration and feedback mechanisms for FHIR artifacts
    /// - Facilitate clinical communication for patient care coordination
    /// - Enable emergency contact information for patients and care scenarios
    /// - Provide organizational contact points for administrative and business
    ///   processes
    /// - Support regulatory and compliance communication requirements
    ///
    /// ## Usage
    ///
    /// Use the `contact` attribute when:
    ///
    /// - Publishing canonical resources that require maintainer or author
    ///   identification
    /// - Creating implementation guides with support contact information
    /// - Managing patient emergency contacts and care team communication
    /// - Establishing organizational contact points for business relationships
    /// - Supporting regulatory submissions that require contact information
    /// - Enabling collaboration on FHIR artifacts and clinical content
    /// - Providing support channels for users of FHIR resources and systems
    ///
    /// Contact information should be current, accurate, and appropriate for the
    /// intended use.
    ///
    /// ## Data Type
    ///
    /// **ContactDetail** - A complex structure containing:
    ///
    /// - `name` (string): Name of the contact person or organization
    /// - `telecom` (ContactPoint[]): Telecommunications details (phone, email,
    ///   fax, etc.)
    ///
    /// **ContactPoint elements include:**
    ///
    /// - `system` (code): Communication system
    ///   (phone|fax|email|pager|url|sms|other)
    /// - `value` (string): The actual contact value (phone number, email
    ///   address, etc.)
    /// - `use` (code): Purpose of the contact (home|work|temp|old|mobile)
    /// - `rank` (positiveInt): Preference order for multiple contacts
    /// - `period` (Period): Time period when contact is valid
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional for most resources, recommended for canonical
    ///   resources
    /// - **Cardinality**: 0..* (zero or more contacts per resource)
    /// - **Telecom Systems**: Must use valid values from ContactPointSystem
    ///   value set
    /// - **Use Codes**: Must use valid values from ContactPointUse value set
    /// - **Completeness**: Should include sufficient information for effective
    ///   communication
    /// - **Privacy**: Should respect privacy requirements and data protection
    ///   regulations
    /// - **Currency**: Contact information should be kept current and accurate
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating contact usage in StructureDefinitions, Organizations, and
    /// Patient resources with various contact types and telecommunications
    /// details.
    ///
    /// ## Related Keys
    ///
    /// - `name` - Name of the contact person or organization
    /// - `telecom` - Telecommunications contact points including phone, email,
    ///   and other systems
    /// - `system` - Type of telecommunications system used for contact
    /// - `value` - Actual contact value such as phone number or email address
    /// - `use` - Purpose or context of the contact information
    /// - `publisher` - Entity responsible for the resource, often related to
    ///   primary contact
    /// - `author` - Resource authors who may also serve as contact points
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ContactDetail data type and
    /// ContactPoint structure definitions.
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// # description
    ///
    /// ## Description
    ///
    /// The `description` attribute provides detailed, comprehensive information
    /// about a FHIR resource, element, or concept. It serves as the primary
    /// field for conveying extended explanatory text that helps users
    /// understand the purpose, usage, constraints, and context of the described
    /// item beyond what a simple name or title can convey.
    ///
    /// ## Purpose
    ///
    /// The `description` exists to provide comprehensive documentation and
    /// context, enabling:
    ///
    /// - Detailed explanation of resource purpose and functionality
    /// - Clear guidance on proper usage and implementation
    /// - Documentation of constraints, limitations, and special considerations
    /// - Support for user understanding and decision-making
    /// - Enhanced searchability and discoverability of resources
    ///
    /// ## Usage
    ///
    /// Use the `description` attribute when:
    ///
    /// - Documenting the purpose and scope of StructureDefinitions and profiles
    /// - Explaining the clinical context and usage of value sets and code
    ///   systems
    /// - Providing implementation guidance for operation definitions
    /// - Describing the rationale behind business rules and constraints
    /// - Offering detailed explanations for complex clinical protocols
    /// - Supporting user interfaces with comprehensive help text
    ///
    /// Descriptions should be clear, accurate, and comprehensive while
    /// remaining concise enough to be useful.
    ///
    /// ## Data Type
    ///
    /// **markdown** or **string** - Rich text content that may include:
    ///
    /// - **markdown**: Supports basic formatting, links, lists, and structured
    ///   text
    /// - **string**: Plain text for simpler description needs
    /// - Multi-line text with proper formatting and structure
    /// - References to external documentation or standards
    /// - Technical details and implementation notes
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - often required for definitional resources
    /// - **Cardinality**: Typically 0..1 (zero to one occurrence)
    /// - **Length**: Should be comprehensive but not excessively long
    /// - **Format**: Should follow markdown conventions when applicable
    /// - **Content**: Should be technically accurate and clinically relevant
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete
    /// StructureDefinition demonstrating comprehensive use of the `description`
    /// attribute in various contexts.
    ///
    /// ## Related Keys
    ///
    /// - `title` - Brief, formal title that complements the description
    /// - `purpose` - Specific statement of why the resource exists
    /// - `comment` - Additional notes or implementation guidance
    /// - `usage` - Specific usage instructions and guidance
    /// - `copyright` - Legal information that may relate to usage
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for markdown usage, definitional resource
    /// requirements, and description best practices.
    ///
    pub description: Option<types::Markdown>,

    /// # useContext
    ///
    /// ## Description
    ///
    /// The `useContext` property defines the specific contexts, situations, or
    /// circumstances where a resource, profile, or artifact is intended to be
    /// used. It provides machine-readable context information for appropriate
    /// usage.
    ///
    /// ## Purpose
    ///
    /// - Define appropriate usage contexts for resources
    /// - Enable context-aware resource discovery and selection
    /// - Support automated filtering based on usage scenarios
    /// - Facilitate implementation-specific resource management
    /// - Provide semantic context for resource applicability
    ///
    /// ## Usage
    ///
    /// The `useContext` property is used in conformance and knowledge artifacts
    /// to specify when and where they should be applied, helping systems choose
    /// appropriate resources for specific situations.
    ///
    /// ## Data Type
    ///
    /// **Array of UsageContext** - Each UsageContext contains:
    ///
    /// - `code` - The type of context (age, gender, species, etc.)
    /// - `value[x]` - The specific context value (CodeableConcept, Quantity,
    ///   Range, Reference)
    ///
    /// ## Constraints
    ///
    /// - Must specify both code and value
    /// - Code must be from the usage-context-type value set
    /// - Value type must be appropriate for the context code
    /// - Should provide meaningful filtering criteria
    ///
    /// ## Examples
    ///
    /// ### Age and Gender Context
    ///
    /// ```json
    /// {
    ///   "useContext": [
    ///     {
    ///       "code": {
    ///         "system": "http://terminology.hl7.org/CodeSystem/usage-context-type",
    ///         "code": "age",
    ///         "display": "Age Range"
    ///       },
    ///       "valueRange": {
    ///         "low": {
    ///           "value": 18,
    ///           "unit": "years"
    ///         },
    ///         "high": {
    ///           "value": 65,
    ///           "unit": "years"
    ///         }
    ///       }
    ///     },
    ///     {
    ///       "code": {
    ///         "system": "http://terminology.hl7.org/CodeSystem/usage-context-type",
    ///         "code": "gender",
    ///         "display": "Gender"
    ///       },
    ///       "valueCodeableConcept": {
    ///         "coding": [
    ///           {
    ///             "system": "http://hl7.org/fhir/administrative-gender",
    ///             "code": "female",
    ///             "display": "Female"
    ///           }
    ///         ]
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Clinical Setting Context
    ///
    /// ```json
    /// {
    ///   "useContext": [
    ///     {
    ///       "code": {
    ///         "system": "http://terminology.hl7.org/CodeSystem/usage-context-type",
    ///         "code": "venue",
    ///         "display": "Clinical Venue"
    ///       },
    ///       "valueCodeableConcept": {
    ///         "coding": [
    ///           {
    ///             "system": "http://snomed.info/sct",
    ///             "code": "440655000",
    ///             "display": "Outpatient environment"
    ///           }
    ///         ]
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `jurisdiction` - Legal/geographic applicability
    /// - `context` - Additional context information
    /// - `topic` - Subject matter topics
    /// - `purpose` - Intended purpose description
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [UsageContext](http://hl7.org/fhir/R5/metadatatypes.html#UsageContext)
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// # jurisdiction
    ///
    /// ## Description
    ///
    /// The `jurisdiction` key is used in FHIR R5 conformance and terminology
    /// resources to specify the legal or political jurisdictions for which the
    /// resource is intended or applies. It helps identify the geographic or
    /// organizational scope of applicability.
    ///
    /// ## Purpose
    ///
    /// - Specifies geographic or political scope of resource applicability
    /// - Enables jurisdiction-specific filtering and discovery
    /// - Supports regulatory and legal compliance requirements
    /// - Facilitates international and multi-jurisdictional implementations
    /// - Provides context for resource interpretation and usage
    ///
    /// ## Usage
    ///
    /// The `jurisdiction` appears in:
    ///
    /// - **StructureDefinition**: To specify where profiles apply
    /// - **ValueSet/CodeSystem**: For terminology jurisdiction scope
    /// - **CapabilityStatement**: To indicate server/client jurisdiction
    /// - **Implementation guides**: For geographic applicability
    ///
    /// ## Data Type
    ///
    /// **CodeableConcept** - Array of coded jurisdictions containing:
    /// - `coding` - Coded jurisdiction (typically using ISO 3166 country codes)
    /// - `text` - Human-readable jurisdiction description
    ///
    /// ## Constraints
    ///
    /// - Should use standardized jurisdiction codes when available
    /// - ISO 3166 country codes are commonly used
    /// - Can specify multiple jurisdictions for multi-national resources
    /// - Should be consistent with the resource's intended use scope
    ///
    /// ## Examples
    ///
    /// ### Single Country Jurisdiction
    ///
    /// ```json
    /// {
    ///   "jurisdiction": [
    ///     {
    ///       "coding": [
    ///         {
    ///           "system": "urn:iso:std:iso:3166",
    ///           "code": "US",
    ///           "display": "United States of America"
    ///         }
    ///       ]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Multiple Jurisdictions
    ///
    /// ```json
    /// {
    ///   "jurisdiction": [
    ///     {
    ///       "coding": [
    ///         {
    ///           "system": "urn:iso:std:iso:3166",
    ///           "code": "US",
    ///           "display": "United States of America"
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "coding": [
    ///         {
    ///           "system": "urn:iso:std:iso:3166",
    ///           "code": "CA",
    ///           "display": "Canada"
    ///         }
    ///       ]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Regional Jurisdiction
    /// ```json
    /// {
    ///   "jurisdiction": [
    ///     {
    ///       "coding": [
    ///         {
    ///           "system": "http://unstats.un.org/unsd/methods/m49/m49.htm",
    ///           "code": "150",
    ///           "display": "Europe"
    ///         }
    ///       ]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `useContext` - Context of use for the resource
    /// - `publisher` - Organization publishing the resource
    /// - `contact` - Contact information for the resource
    /// - `copyright` - Copyright and legal notices
    /// - `status` - Publication status of the resource
    /// - `date` - Publication date
    ///
    /// ## Specification Reference
    ///
    /// - **FHIR R5 Specification**: Used across multiple conformance resources
    /// - **ISO 3166 Codes**: [Country
    ///   Codes](https://www.iso.org/iso-3166-country-codes.html)
    /// - **UN M49 Codes**: [Geographic
    ///   Regions](https://unstats.un.org/unsd/methodology/m49/)
    /// - **Context**: Used in conformance and terminology resources for scope
    ///   definition
    ///
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// # purpose
    ///
    /// ## Description
    ///
    /// The `purpose` attribute provides an explanation of why a FHIR resource
    /// exists and what it is intended to accomplish. This element goes beyond
    /// the technical description to articulate the clinical, business, or
    /// regulatory rationale for the resource's creation and use. The purpose
    /// helps implementers understand the intended context and appropriate
    /// applications for the resource, supporting better decision-making about
    /// adoption and implementation.
    ///
    /// ## Purpose
    ///
    /// The `purpose` exists to:
    ///
    /// - Explain the rationale and intended use cases for FHIR resources
    /// - Provide context for implementers to understand appropriate
    ///   applications
    /// - Support decision-making about resource adoption and implementation
    /// - Document regulatory or business requirements that drove resource
    ///   creation
    /// - Enable better resource discovery and selection for specific use cases
    /// - Facilitate understanding of resource scope and boundaries
    /// - Support governance and compliance requirements for resource usage
    ///
    /// ## Usage
    ///
    /// Use the `purpose` attribute when:
    ///
    /// - Publishing canonical resources like StructureDefinitions, ValueSets,
    ///   or CodeSystems
    /// - Creating implementation guides that need clear use case documentation
    /// - Supporting regulatory submissions that require rationale documentation
    /// - Enabling resource discovery and selection processes
    /// - Providing guidance for implementers about appropriate resource usage
    /// - Documenting business or clinical requirements that justify resource
    ///   creation
    /// - Supporting governance processes that require purpose documentation
    ///
    /// The purpose should be clear, concise, and focused on the "why" rather
    /// than the "what" or "how".
    ///
    /// ## Data Type
    ///
    /// **markdown** - Formatted text supporting Markdown syntax:
    ///
    /// - Supports rich text formatting including lists, emphasis, and links
    /// - Should be concise but comprehensive enough to explain the rationale
    /// - May include references to regulatory requirements or clinical
    ///   guidelines
    /// - Can use formatting to improve readability and organization
    /// - Should avoid overly technical jargon when possible
    /// - May include examples or scenarios to illustrate intended use
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional but strongly recommended for canonical
    ///   resources
    /// - **Cardinality**: 0..1 (at most one purpose statement per resource)
    /// - **Length**: Should be substantial enough to explain rationale but
    ///   concise for readability
    /// - **Format**: Markdown text that renders appropriately in documentation
    ///   systems
    /// - **Language**: Should match the language specified in the resource
    /// - **Clarity**: Should be understandable to the target audience of
    ///   implementers
    /// - **Accuracy**: Should accurately reflect the actual intended use and
    ///   rationale
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating purpose usage in various FHIR resources including
    /// StructureDefinitions, ValueSets, and ImplementationGuides with clear
    /// rationale statements.
    ///
    /// ## Related Keys
    ///
    /// - `description` - Technical description that complements the purpose
    ///   with "what" information
    /// - `title` - Human-readable name that should align with the stated
    ///   purpose
    /// - `useContext` - Specific contexts where the resource applies,
    ///   supporting the purpose
    /// - `jurisdiction` - Geographic or organizational scope related to the
    ///   purpose
    /// - `copyright` - Legal context that may relate to the purpose and
    ///   intended use
    /// - `publisher` - Organization responsible for the resource, often related
    ///   to the purpose
    /// - `status` - Current status that indicates readiness for the stated
    ///   purpose
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and purpose
    /// element usage guidelines.
    ///
    pub purpose: Option<types::Markdown>,

    /// # copyright
    ///
    /// ## Description
    ///
    /// The `copyright` field contains copyright and intellectual property
    /// rights information for FHIR resources such as ImplementationGuide,
    /// ValueSet, CodeSystem, and other definitional resources. It provides
    /// legal notice about the ownership and usage rights of the resource.
    ///
    /// ## Purpose
    ///
    /// - Specify copyright ownership and intellectual property rights
    /// - Provide legal notice for resource usage
    /// - Indicate licensing terms and restrictions
    /// - Support compliance with intellectual property requirements
    /// - Enable proper attribution of resource authorship
    ///
    /// ## Usage
    ///
    /// The `copyright` field is commonly used in:
    ///
    /// - **ImplementationGuide**: Copyright information for the entire guide
    /// - **ValueSet**: Copyright for value set definitions and content
    /// - **CodeSystem**: Copyright for code system definitions
    /// - **StructureDefinition**: Copyright for profile definitions
    /// - **Other definitional resources**: Any resource requiring copyright
    ///   notice
    ///
    /// ## Data Type
    ///
    /// - **Type**: markdown
    /// - **Cardinality**: 0..1
    /// - **Format**: Markdown-formatted text allowing rich formatting
    ///
    /// ## Constraints
    ///
    /// - Should include clear copyright ownership statement
    /// - Must comply with applicable copyright laws
    /// - Should specify usage permissions and restrictions
    /// - May reference external license terms
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` for practical usage examples.
    ///
    /// ## Related Keys
    ///
    /// - `copyrightLabel`: Short copyright label for display
    /// - `publisher`: Entity responsible for publication
    /// - `contact`: Contact information for rights holder
    /// - `useContext`: Context where copyright applies
    /// - `jurisdiction`: Legal jurisdiction for copyright
    ///
    /// ## Specification Reference
    ///
    /// - [FHIR R5
    ///   ImplementationGuide](https://hl7.org/fhir/R5/implementationguide.html)
    /// - [FHIR R5 ValueSet](https://hl7.org/fhir/R5/valueset.html)
    /// - [FHIR R5 CodeSystem](https://hl7.org/fhir/R5/codesystem.html)
    ///
    pub copyright: Option<types::Markdown>,

    pub copyright_label: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CanonicalResource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            url: None,
            identifier: vec![],
            version: None,
            version_algorithm: types::String::default(),
            name: None,
            title: None,
            status: types::Code::default(),
            experimental: None,
            date: None,
            publisher: None,
            contact: vec![],
            description: None,
            use_context: vec![],
            jurisdiction: vec![],
            purpose: None,
            copyright: None,
            copyright_label: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;

        #[test]
        fn test_serde_json_round_trip() {
            let value = T::default();
            let json = ::serde_json::to_value(&value).expect("to_value");
            let back: T = ::serde_json::from_value(json).expect("from_value");
            assert_eq!(value, back);
        }
    }
}
