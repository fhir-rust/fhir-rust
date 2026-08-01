//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Bundle {
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

    /// # type
    ///
    /// ## Description
    ///
    /// The `type` attribute is used throughout FHIR R5 to specify the category,
    /// classification, or specific kind of an element or resource. It provides
    /// essential context that determines how data should be interpreted,
    /// processed, or displayed, and often drives business logic and workflow
    /// decisions within healthcare systems.
    ///
    /// ## Purpose
    ///
    /// The `type` exists to provide categorization and context for FHIR
    /// elements, enabling:
    ///
    /// - Classification of data elements into meaningful categories
    /// - Support for polymorphic data structures and processing
    /// - Workflow and business logic decision-making
    /// - Appropriate rendering and user interface behavior
    /// - Filtering and querying based on type classifications
    ///
    /// ## Usage
    ///
    /// Use the `type` attribute when:
    ///
    /// - Classifying identifiers (MRN, SSN, driver's license, etc.)
    /// - Specifying communication methods (phone, email, fax)
    /// - Categorizing addresses (home, work, temporary)
    /// - Defining contact relationships (emergency contact, next of kin)
    /// - Classifying observations, procedures, or other clinical data
    /// - Specifying reference types in resource relationships
    ///
    /// The type often uses standardized code systems to ensure consistency and
    /// interoperability.
    ///
    /// ## Data Type
    ///
    /// **CodeableConcept** - Typically a coded value that may include:
    ///
    /// - `coding` - Array of coded representations from standard terminologies
    /// - `text` - Human-readable description of the type
    /// - Support for multiple coding systems for the same concept
    /// - Fallback to text when no appropriate code exists
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - depends on the specific context and use
    ///   case
    /// - **Cardinality**: Usually 0..1, sometimes 0..* for multiple type
    ///   classifications
    /// - **Binding Strength**: Often bound to specific value sets with varying
    ///   strength
    /// - **Consistency**: Should align with established terminology standards
    /// - **Context Dependency**: Meaning may vary based on the containing
    ///   element
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating various uses of the `type` attribute across different
    /// elements.
    ///
    /// ## Related Keys
    ///
    /// - `system` - URI identifying the code system used in type coding
    /// - `code` - Specific identifier within the type's code system
    /// - `display` - Human-readable representation of the type
    /// - `use` - Usage context that may complement type information
    /// - `category` - Higher-level classification that may contain type
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for specific element type bindings,
    /// CodeableConcept usage, and terminology binding requirements.
    ///
    #[serde(rename = "type")]
    pub r#type: String,

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
    ///
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
    ///
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
    ///
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
    pub meta: Meta,

    /// # entry
    ///
    /// ## Description
    ///
    /// The `entry` attribute represents individual entries within a FHIR Bundle
    /// resource. Each entry contains a single FHIR resource along with metadata
    /// about that resource's inclusion in the bundle, such as search scores,
    /// request information, and response details. Entries are the fundamental
    /// building blocks of bundles, allowing multiple related resources to be
    /// grouped together for transmission, processing, or storage as a cohesive
    /// unit.
    ///
    /// ## Purpose
    ///
    /// The `entry` exists to:
    /// - Package individual FHIR resources within a bundle structure
    /// - Provide metadata context for each resource within the bundle
    /// - Support batch operations and transaction processing
    /// - Enable search result compilation with relevance scoring
    /// - Facilitate resource relationships and cross-references within bundles
    /// - Support messaging workflows with multiple related resources
    /// - Enable atomic operations across multiple resources
    ///
    /// ## Usage
    ///
    /// Use the `entry` attribute when:
    /// - Creating Bundle resources containing multiple FHIR resources
    /// - Implementing search operations that return multiple results
    /// - Supporting batch or transaction operations across multiple resources
    /// - Building messaging payloads with related clinical information
    /// - Implementing subscription notifications with multiple resources
    /// - Creating document bundles with structured clinical content
    /// - Supporting RESTful operations that affect multiple resources
    ///   simultaneously
    ///
    /// Each entry should contain a resource and may include search metadata,
    /// request details, or response information depending on the bundle type.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - A complex structure containing:
    ///
    /// - `fullUrl` (string): Absolute URL for the resource
    /// - `resource` (Resource): The actual FHIR resource
    /// - `search` (BackboneElement): Search-related information
    /// - `request` (BackboneElement): Request details for batch/transaction
    /// - `response` (BackboneElement): Response details for batch/transaction
    /// - Additional metadata elements specific to bundle type and context
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes for Bundle resources (must have at least one entry
    ///   or be empty)
    /// - **Cardinality**: 0..* (zero or more entries per bundle)
    /// - **Resource**: Each entry should contain exactly one FHIR resource
    /// - **URL**: fullUrl should be unique within the bundle when present
    /// - **Type Consistency**: All entries should be appropriate for the bundle
    ///   type
    /// - **Referential Integrity**: Internal references should resolve within
    ///   the bundle
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Bundle resource
    /// demonstrating entry usage in search results, transactions, and messaging
    /// contexts with various resource types.
    ///
    /// ## Related Keys
    ///
    /// - `fullUrl` - Absolute URL identifier for the resource within the entry
    /// - `resource` - The actual FHIR resource contained within the entry
    /// - `search` - Search-specific metadata when the bundle represents search
    ///   results
    /// - `request` - Request details for batch and transaction bundle entries
    /// - `response` - Response information for completed batch and transaction
    ///   operations
    /// - `total` - Total count of matching resources (bundle-level, not
    ///   entry-level)
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Bundle resource structure and entry
    /// element definitions.
    ///
    pub entry: Vec<Entry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("bundle")
            .join("bundle.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "conceptmaps");
    }
}
