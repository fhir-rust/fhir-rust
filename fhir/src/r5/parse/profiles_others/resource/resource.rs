//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use crate::r5::parse::profiles_others::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Resource {
    /// # resourceType
    ///
    /// ## Description
    ///
    /// The `resourceType` attribute specifies the type of FHIR resource being
    /// represented. It is a mandatory element that identifies which resource
    /// schema and constraints apply to the JSON document in FHIR R5.
    ///
    /// ## Purpose
    ///
    /// The `resourceType` serves several critical functions:
    ///
    /// - Identifies the specific FHIR resource type for parsers and processors
    /// - Determines which validation rules and constraints apply
    /// - Enables proper routing and processing in FHIR systems
    /// - Provides context for interpreting the resource's data elements
    /// - Supports polymorphism in FHIR resource handling
    ///
    /// ## Usage
    ///
    /// The `resourceType` must be included in every FHIR resource as the first
    /// element. It should be used:
    ///
    /// - At the root level of every FHIR resource JSON document
    /// - When validating resources against their appropriate
    ///   StructureDefinitions
    /// - In API endpoints to determine resource-specific processing logic
    /// - For content negotiation and resource type filtering
    ///
    /// ## Data Type
    ///
    /// **code** - A string that must exactly match one of the defined FHIR
    /// resource types. The value is:
    ///
    /// - Case-sensitive
    /// - Must be an exact match to a valid FHIR R5 resource type name
    /// - Follows PascalCase naming convention (e.g., "Patient", "Observation",
    ///   "DiagnosticReport")
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - Must be present in every FHIR resource
    /// - **Cardinality**: 1..1 (exactly one occurrence)
    /// - **Fixed Position**: Must be the first element in the JSON object
    /// - **Valid Values**: Must be one of the 150+ defined FHIR R5 resource
    ///   types
    /// - **Case Sensitivity**: Exact case match required
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating the use of the `resourceType` attribute.
    ///
    /// ## Related Keys
    ///
    /// - `meta.profile` - Specifies which profile(s) the resource conforms to
    /// - `id` - Unique identifier for the resource instance
    /// - `meta` - Metadata about the resource
    /// - All resource-specific elements depend on the `resourceType` for their
    ///   validity
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details and the full list
    /// of valid resource types, refer to the official FHIR R5 documentation for
    /// resource definitions.
    ///
    pub resource_type: String,

    /// # id
    ///
    /// ## Description
    ///
    /// The `id` attribute is the logical identifier for a FHIR resource within
    /// a given context. It uniquely identifies the resource and is used for
    /// resource addressing and referencing within FHIR R5.
    ///
    /// ## Purpose
    ///
    /// The `id` exists to provide a unique identifier for each FHIR resource
    /// instance. This identifier is essential for:
    ///
    /// - Resource addressing via RESTful URLs
    /// - Creating references between resources
    /// - Version control and resource tracking
    /// - Enabling resource updates and deletions
    ///
    /// ## Usage
    ///
    /// Use the `id` attribute when:
    ///
    /// - Creating a new resource that needs to be uniquely identifiable
    /// - Referencing a resource from another resource
    /// - Performing CRUD operations on existing resources
    /// - Building RESTful FHIR APIs
    ///
    /// The `id` is typically assigned by the server when a resource is created,
    /// but can be provided by the client in some scenarios.
    ///
    /// ## Data Type
    ///
    /// **string** - A sequence of Unicode characters with the following
    /// constraints:
    ///
    /// - Must be between 1 and 64 characters in length
    /// - Can contain letters (A-Z, a-z), digits (0-9), hyphens (-), and periods
    ///   (.)
    /// - Must start and end with an alphanumeric character
    /// - Case sensitive
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - The `id` is optional for resource creation but
    ///   typically assigned by servers
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Length**: 1-64 characters
    /// - **Pattern**: Must match the regex `[A-Za-z0-9\-\.]{1,64}`
    /// - **Uniqueness**: Must be unique within the context of the resource type
    ///   on a given server
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating the use of the `id` attribute.
    ///
    /// ## Related Keys
    ///
    /// - `meta.versionId` - Version identifier for the resource instance
    /// - `identifier` - Business identifiers for the resource
    /// - `fullUrl` - Absolute URL when used in bundles
    /// - `reference` - Used to reference this resource from other resources
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for resource identity and addressing.
    ///
    pub id: String,

    /// # meta
    ///
    /// ## Description
    ///
    /// The `meta` attribute contains metadata about a FHIR resource that is
    /// maintained by the infrastructure. It provides information about the
    /// resource's versioning, last modification, security labels, profiles, and
    /// tags in FHIR R5.
    ///
    /// ## Purpose
    ///
    /// The `meta` element serves to:
    /// - Track resource versioning and modification history
    /// - Specify which profiles the resource claims to conform to
    /// - Apply security labels and access control information
    /// - Provide tags for categorization and workflow management
    /// - Enable optimistic locking through version control
    /// - Support provenance and audit requirements
    ///
    /// ## Usage
    ///
    /// Use the `meta` attribute to:
    /// - Track when resources were last updated
    /// - Specify profile conformance for validation
    /// - Apply security classifications to resources
    /// - Tag resources for workflow or categorization purposes
    /// - Enable version-aware updates and conflict detection
    /// - Support system-level metadata requirements
    ///
    /// The `meta` element is typically managed by the server infrastructure,
    /// though clients may provide some elements.
    ///
    /// ## Data Type
    ///
    /// **Meta** - A complex data type containing the following optional
    /// sub-elements:
    /// - `versionId`: string - Version identifier for the resource
    /// - `lastUpdated`: instant - When the resource was last updated  
    /// - `source`: uri - Identifies where the resource came from
    /// - `profile`: array of canonical URIs - Profiles this resource claims to
    ///   conform to
    /// - `security`: array of Coding - Security labels applied to the resource
    /// - `tag`: array of Coding - Tags applied to the resource for
    ///   categorization
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - The entire `meta` element is optional
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Server Managed**: Most sub-elements are controlled by the server
    /// - **versionId**: Must change when resource content changes
    /// - **lastUpdated**: Must be updated when resource content changes
    /// - **profile**: Must reference valid StructureDefinition resources
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Practitioner
    /// resource demonstrating comprehensive use of the `meta` attribute.
    ///
    /// ## Related Keys
    ///
    /// - `id` - Resource identifier that the meta information describes
    /// - `resourceType` - Resource type that determines applicable profiles
    /// - `extension` - May contain additional metadata not covered by meta
    /// - Bundle entries use `meta` for version control during transactions
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details on metadata
    /// management, versioning, and security labeling, refer to the official
    /// FHIR R5 documentation.    
    ///     
    pub meta: Option<Meta>,

    /// # text
    ///
    /// ## Description
    ///
    /// The `text` attribute provides a human-readable narrative summary of a
    /// FHIR resource's content in XHTML format. This narrative serves as a
    /// fallback representation that ensures the essential information remains
    /// accessible even when systems cannot process all the structured data
    /// elements. The text element is particularly important for clinical
    /// safety, regulatory compliance, and systems interoperability where human
    /// readability is required.
    ///
    /// ## Purpose
    ///
    /// The `text` exists to:
    ///
    /// - Provide human-readable summaries of structured resource content
    /// - Ensure clinical information remains accessible when structured data
    ///   cannot be processed
    /// - Support regulatory requirements for human-readable clinical documents
    /// - Enable fallback display when rendering systems have limited
    ///   capabilities
    /// - Provide narrative context that complements structured data
    /// - Support clinical safety by ensuring critical information is always
    ///   readable
    /// - Enable content review and validation by healthcare professionals
    ///
    /// ## Usage
    ///
    /// Use the `text` attribute when:
    ///
    /// - Creating clinical resources that require human-readable summaries
    /// - Supporting regulatory compliance for clinical documentation
    /// - Ensuring accessibility across diverse healthcare systems
    /// - Providing narrative context for complex structured data
    /// - Creating resources for patient-facing applications
    /// - Supporting clinical review workflows that need readable content
    /// - Implementing systems that require both structured and narrative
    ///   representations
    ///
    /// The narrative should accurately summarize the key information from the
    /// structured elements.
    ///
    /// ## Data Type
    ///
    /// **Narrative** - A complex structure containing:
    ///
    /// - `status` (code): The generation status of the narrative
    ///   (generated|extensions|additional|empty)
    /// - `div` (xhtml): The XHTML content of the narrative
    ///
    /// **Status Values:**
    ///
    /// - `generated`: Generated from structured data, no additional information
    /// - `extensions`: Generated from structured data with additional extension
    ///   content
    /// - `additional`: Contains additional information not in structured data
    /// - `empty`: No narrative content provided
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional but strongly recommended for most clinical
    ///   resources
    /// - **Cardinality**: 0..1 (at most one narrative per resource)
    /// - **XHTML Format**: The div element must contain valid XHTML content
    /// - **Safety**: Should include all critical information from structured
    ///   data
    /// - **Consistency**: Should accurately reflect the structured data content
    /// - **Language**: Should match the language specified in the resource
    /// - **Security**: XHTML content must be safe and not contain executable
    ///   scripts
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating text narratives for different resource types including
    /// clinical observations, medications, and patient information.
    ///
    /// ## Related Keys
    ///
    /// - `div` - The XHTML content portion of the narrative
    /// - `status` - Indicates how the narrative was generated and its
    ///   relationship to structured data
    /// - `language` - Language code that may affect narrative content
    /// - `meta` - Resource metadata that may influence narrative generation
    /// - `contained` - Inline resources that may be referenced in the narrative
    /// - `extension` - Extensions that may be included in "extensions" status
    ///   narratives
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Narrative data type and narrative
    /// generation requirements.
    ///
    pub text: Option<::serde_json::Value>,

    /// # extension
    ///
    /// ## Description
    ///
    /// The `extension` attribute provides a mechanism for extending FHIR
    /// resources with additional data elements that are not part of the base
    /// resource definition. Extensions allow for local customizations and the
    /// addition of new data elements while maintaining interoperability in FHIR
    /// R5.
    ///
    /// ## Purpose
    ///
    /// Extensions exist to:
    ///
    /// - Add data elements not covered by the base FHIR specification
    /// - Support local, regional, or national requirements
    /// - Enable gradual evolution of FHIR without breaking existing
    ///   implementations
    /// - Maintain semantic interoperability through standardized extension
    ///   definitions
    /// - Allow for experimental or emerging data requirements
    /// - Support backwards compatibility when new elements are added to FHIR
    ///
    /// ## Usage
    ///
    /// Use extensions when you need to:
    ///
    /// - Include additional data not supported by standard FHIR elements
    /// - Implement local business requirements
    /// - Support regulatory or compliance requirements
    /// - Add experimental data elements before they become part of core FHIR
    /// - Extend resources with organization-specific information
    ///
    /// Extensions should always reference a StructureDefinition that defines
    /// their meaning and constraints.
    ///
    /// ## Data Type
    ///
    /// **Extension** - A complex data type containing:
    ///
    /// - `url` (required): canonical URI identifying the extension definition
    /// - `value[x]` (optional): the actual extension value using one of the
    ///   allowed FHIR data types
    /// - `extension` (optional): nested extensions for complex extension
    ///   structures
    ///
    /// Extensions can be simple (single value) or complex (containing nested
    /// extensions).
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - Extensions are always optional
    /// - **Cardinality**: 0..* (zero to many occurrences)
    /// - **URL Required**: Every extension must have a `url` that references
    ///   its definition
    /// - **Value or Nested**: Extensions must have either a value or nested
    ///   extensions, not both
    /// - **Definition**: The URL must reference a valid StructureDefinition of
    ///   type Extension
    /// - **Placement**: Can appear on any element that allows extensions
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating various types of extensions including simple value
    /// extensions and complex nested extensions.
    ///
    /// ## Related Keys
    ///
    /// - `modifierExtension` - Extensions that modify the meaning of the
    ///   element
    /// - `url` - Required sub-element identifying the extension
    /// - `value[x]` - The extension's value using FHIR data types
    /// - Any FHIR element can contain extensions
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details on extension
    /// definitions, complex extensions, and extension registries, refer to the
    /// official FHIR R5 documentation on extensibility.
    ///
    pub extension: Option<::serde_json::Value>,

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
    pub url: String,

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
    pub version: String,

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
    pub name: Option<String>,

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
    pub title: Option<String>,

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
    pub status: String,

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
    pub experimental: bool,

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
    pub date: Option<String>,

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
    pub publisher: Option<String>,

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
    pub contact: Option<::serde_json::Value>,

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
    pub description: Option<String>,

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
    pub jurisdiction: Option<Vec<Jurisdiction>>,

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
    pub identifier: Option<Vec<Identifier>>,

    /// # caseSensitive
    ///
    /// ## Description
    ///
    /// The `caseSensitive` property indicates whether codes in a CodeSystem are
    /// case-sensitive. When true, codes must match exactly including case; when
    /// false, codes can be matched regardless of case differences.
    ///
    /// ## Purpose
    ///
    /// - Define case sensitivity rules for code matching and validation
    /// - Support different coding system conventions and requirements
    /// - Enable proper code comparison and lookup operations
    /// - Prevent errors from case mismatches in clinical data
    /// - Support international and legacy system variations
    ///
    /// ## Usage
    ///
    /// The `caseSensitive` property is used in CodeSystem resources to specify
    /// how codes should be compared. This affects validation, terminology
    /// services, and code lookup operations throughout the FHIR ecosystem.
    ///
    /// ## Data Type
    ///
    /// **boolean** - true if codes are case-sensitive, false if
    /// case-insensitive
    ///
    /// ## Constraints
    ///
    /// - Must be a boolean value (true or false)
    /// - If not specified, defaults to true (case-sensitive)
    /// - Should be consistent across all codes in the CodeSystem
    /// - Affects all code matching operations for the system
    ///
    /// ## Examples
    ///
    /// ### Case-Sensitive Code System
    ///
    /// ```json
    /// {
    ///   "caseSensitive": true,
    ///   "concept": [
    ///     {
    ///       "code": "Male",
    ///       "display": "Male"
    ///     },
    ///     {
    ///       "code": "MALE",
    ///       "display": "Male (uppercase variant)"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Case-Insensitive Code System
    ///
    /// ```json
    /// {
    ///   "caseSensitive": false,
    ///   "concept": [
    ///     {
    ///       "code": "active",
    ///       "display": "Active"
    ///     },
    ///     {
    ///       "code": "inactive",
    ///       "display": "Inactive"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Laboratory Code System (Case-Sensitive)
    ///
    /// ```json
    /// {
    ///   "caseSensitive": true,
    ///   "concept": [
    ///     {
    ///       "code": "GLU",
    ///       "display": "Glucose"
    ///     },
    ///     {
    ///       "code": "glu",
    ///       "display": "Glutamine (different from GLU)"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `code` - Individual codes within the CodeSystem
    /// - `concept` - Concepts that contain the codes
    /// - `property` - Additional properties for concepts
    /// - `content` - Indicates how complete the CodeSystem is
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 CodeSystem: [Case
    /// Sensitive](http://hl7.org/fhir/R5/codesystem.html#CodeSystem.caseSensitive)
    ///
    pub case_sensitive: Option<bool>,

    /// # content
    ///
    /// ## Description
    ///
    /// The `content` field represents the actual content or data within a FHIR
    /// resource or element. It is commonly used to hold the primary information
    /// that the resource is meant to convey, such as the binary data in a
    /// Binary resource or the narrative text in a DocumentReference.
    ///
    /// ## Purpose
    ///
    /// - Store the main content or data of a resource
    /// - Provide the actual information being represented
    /// - Support various content types and formats
    /// - Enable attachment of binary or textual data to resources
    ///
    /// ## Usage
    ///
    /// The `content` field is used in multiple contexts:
    ///
    /// - **Binary resources**: Contains base64-encoded binary data
    /// - **DocumentReference**: References to document content
    /// - **Media resources**: Multimedia content
    /// - **Attachment elements**: File content within other resources
    ///
    /// ## Data Type
    ///
    /// - **Type**: Various (base64Binary, string, Attachment, etc.)
    /// - **Cardinality**: Depends on context (0..1 or 0..*)
    /// - **Format**: Context-dependent (binary, text, structured data)
    ///
    /// ## Constraints
    ///
    /// - Content must be appropriately encoded (base64 for binary)
    /// - Size limitations may apply depending on implementation
    /// - Content type should match declared media type
    /// - Must conform to declared encoding
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` for practical usage examples.
    ///
    /// ## Related Keys
    ///
    /// - `contentType`: Specifies the media type of content
    /// - `data`: Alternative field for binary content
    /// - `url`: Reference to external content location
    /// - `attachment`: Structured content with metadata
    /// - `text`: Narrative text content
    ///
    /// ## Specification Reference
    ///
    /// - [FHIR R5 Binary Resource](https://hl7.org/fhir/R5/binary.html)
    /// - [FHIR R5 Attachment
    ///   Datatype](https://hl7.org/fhir/R5/datatypes.html#Attachment)
    /// - [FHIR R5
    ///   DocumentReference](https://hl7.org/fhir/R5/documentreference.html)
    ///
    pub content: Option<String>,

    /// # filter
    ///
    /// ## Description
    ///
    /// The `filter` attribute defines criteria for selecting concepts from a
    /// code system during value set composition. It provides a mechanism to
    /// include or exclude concepts based on their properties, relationships, or
    /// other characteristics rather than listing individual concepts
    /// explicitly. Filters enable dynamic value set construction based on
    /// logical rules and support efficient management of large terminology
    /// subsets.
    ///
    /// ## Purpose
    ///
    /// The `filter` exists to enable rule-based concept selection that
    /// supports:
    ///
    /// - Dynamic inclusion of concepts based on properties or relationships
    /// - Efficient management of large terminology subsets
    /// - Automated value set maintenance as terminologies evolve
    /// - Flexible concept selection using various criteria
    /// - Support for hierarchical and property-based filtering
    /// - Reduced maintenance overhead for evolving value sets
    ///
    /// ## Usage
    ///
    /// Use the `filter` attribute when:
    ///
    /// - Including/excluding concepts based on hierarchical relationships
    ///   (is-a, child-of)
    /// - Selecting concepts with specific property values
    /// - Creating subsets of large terminologies dynamically
    /// - Implementing concept filters based on status or other attributes
    /// - Building value sets that automatically adapt to terminology changes
    /// - Supporting complex inclusion/exclusion logic
    ///
    /// Filters are used within include and exclude elements of ValueSet
    /// composition to define selection criteria.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - A complex structure containing:
    ///
    /// - `property` (code) - The property to filter on
    /// - `op` (code) - The operation to perform (is-a, descendent-of, is-not-a,
    ///   regex, in, not-in, generalizes, exists)
    /// - `value` (string) - The value to compare against
    ///
    /// ## Constraints
    ///
    /// - **Required**: Property, op, and value are required when filter is
    ///   present
    /// - **Cardinality**: 0..* (zero to many occurrences within
    ///   include/exclude)
    /// - **Property**: Must be a valid property code for the referenced code
    ///   system
    /// - **Operation**: Must be supported by the code system for the specified
    ///   property
    /// - **Value**: Must be appropriate for the property type and operation
    /// - **Compatibility**: Filter operations must be supported by terminology
    ///   servers
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete ValueSet
    /// resource demonstrating the `filter` attribute with various operations
    /// including hierarchical filtering, property-based selection, and regex
    /// patterns.
    ///
    /// ## Related Keys
    ///
    /// - `property` - The concept property to filter on
    /// - `op` - The filter operation to apply
    /// - `value` - The value for comparison
    /// - `include` - Container element for inclusion filters
    /// - `exclude` - Container element for exclusion filters
    /// - `system` - Code system to which the filter applies
    /// - `concept` - Alternative to filter for explicit concept listing
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ValueSet resource and filtering
    /// operations.
    ///
    pub filter: Option<::serde_json::Value>,

    /// # concept
    ///
    /// ## Description
    ///
    /// The `concept` attribute represents an individual concept definition
    /// within a CodeSystem resource or specifies particular concepts to
    /// include/exclude within a ValueSet composition. It contains the
    /// fundamental information about a coded concept including its identifier,
    /// display text, definition, and relationships. Concepts are the atomic
    /// units of meaning in FHIR terminology and form the building blocks for
    /// clinical and administrative coding.
    ///
    /// ## Purpose
    ///
    /// The `concept` exists to provide structured representation of terminology
    /// concepts that enables:
    ///
    /// - Detailed definition and documentation of coded concepts
    /// - Hierarchical organization of related concepts
    /// - Support for multiple display names and translations
    /// - Concept properties and relationships
    /// - Precise semantic meaning for healthcare data
    /// - Interoperability across different terminology systems
    ///
    /// ## Usage
    ///
    /// Use the `concept` attribute when:
    ///
    /// - Defining concepts within a CodeSystem resource
    /// - Specifying particular concepts to include/exclude in ValueSet
    ///   composition
    /// - Building hierarchical terminology structures
    /// - Providing detailed concept definitions and properties
    /// - Supporting multilingual displays and translations
    /// - Implementing concept-based filtering and expansion
    ///
    /// Concepts can be nested to represent hierarchical relationships and
    /// contain properties for additional metadata.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - A complex structure containing:
    ///
    /// - `code` (code) - The identifier for the concept
    /// - `display` (string) - Human-readable display name
    /// - `definition` (string) - Formal definition of the concept
    /// - `designation` (array) - Additional display names and translations
    /// - `property` (array) - Properties associated with the concept
    /// - `concept` (array) - Child concepts for hierarchical organization
    ///
    /// ## Constraints
    ///
    /// - **Required**: Code is required when concept is present
    /// - **Cardinality**: 0..* (zero to many occurrences)
    /// - **Uniqueness**: Code must be unique within the same level of hierarchy
    /// - **Hierarchy**: Child concepts inherit context from parent concepts
    /// - **Properties**: Property codes must be defined in the CodeSystem
    /// - **Display**: Display text should be appropriate for the target
    ///   language
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete CodeSystem
    /// resource demonstrating the `concept` attribute with hierarchical
    /// organization, properties, designations, and multilingual support.
    ///
    /// ## Related Keys
    ///
    /// - `code` - The unique identifier for the concept
    /// - `display` - Human-readable name for the concept
    /// - `definition` - Formal definition text
    /// - `designation` - Alternative display names and translations
    /// - `property` - Additional concept properties and metadata
    /// - `contains` - Related concept used in ValueSet expansions
    /// - `system` - The code system containing the concept
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for CodeSystem resource and concept
    /// definition principles.
    ///
    pub concept: Option<::serde_json::Value>,

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
    pub purpose: Option<String>,

    /// Example: "logical"
    pub kind: Option<String>,

    /// Example: false,
    pub r#abstract: Option<bool>,

    /// Example: "date.id"
    pub r#type: Option<String>,

    /// Example: "specialization"
    pub derivation: Option<String>,

    /// Example: "5.0.0"
    pub fhir_version: Option<String>,

    /// Example: { "element": [...] }
    pub snapshot: Option<Snapshot>,

    /// Example: "baseDefinition" : "http://hl7.org/fhir/StructureDefinition/DataType"
    pub base_definition: Option<String>,

    /// Example: [{ "identity" : "rim", … ]}
    pub mapping: Option<::serde_json::Value>,

    /// Example: { "element": [...] }
    pub differential: Option<Differential>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_others::DIR
            .join("resource")
            .join("resource.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "Narrative");
    }
}
