//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AdditionalAttribute {
    /// # code
    ///
    /// ## Description
    ///
    /// The `code` attribute represents a coded value that identifies a specific
    /// concept from a terminology system. It is a fundamental building block in
    /// FHIR R5 for representing standardized concepts and is commonly used
    /// within CodeableConcept and Coding data types to provide machine-readable
    /// identifiers for clinical and administrative concepts.
    ///
    /// ## Purpose
    ///
    /// The `code` exists to provide standardized, machine-readable identifiers
    /// for concepts across healthcare systems. This enables:
    ///
    /// - Interoperability between different healthcare systems
    /// - Consistent representation of clinical concepts
    /// - Support for clinical decision support and analytics
    /// - Mapping between different terminology systems
    /// - Precise semantic meaning in healthcare data exchange
    ///
    /// ## Usage
    ///
    /// Use the `code` attribute when:
    ///
    /// - Representing a specific concept from a code system (diagnoses,
    ///   procedures, medications)
    /// - Creating CodeableConcept or Coding structures
    /// - Implementing standardized vocabularies (ICD-10, SNOMED CT, LOINC)
    /// - Ensuring semantic interoperability between systems
    /// - Supporting automated processing and clinical decision support
    ///
    /// The `code` should always be paired with a `system` that identifies the
    /// terminology from which the code is drawn.
    ///
    /// ## Data Type
    ///
    /// **string** - A sequence of characters representing the identifier within
    /// a code system:
    ///
    /// - Case sensitivity depends on the code system definition
    /// - Length and format constraints are defined by the specific terminology
    ///   system
    /// - May contain alphanumeric characters, hyphens, periods, and other
    ///   characters as allowed by the system
    /// - Should not contain leading or trailing whitespace
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - Required when used within a Coding,
    ///   optional in CodeableConcept when only text is provided
    /// - **Cardinality**: 0..1 (zero to one occurrence within a Coding)
    /// - **Format**: Must conform to the code format rules of the specified
    ///   system
    /// - **Validation**: Should be a valid code in the specified system
    /// - **Case Sensitivity**: Follows the case sensitivity rules of the code
    ///   system
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating various uses of the `code` attribute in different
    /// contexts.
    ///
    /// ## Related Keys
    ///
    /// - `system` - URI that identifies the code system containing this code
    /// - `display` - Human-readable representation of the code
    /// - `coding` - Array containing code/system/display triplets
    /// - `text` - Free text representation when no suitable code exists
    /// - `version` - Version of the code system when relevant
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Coding and CodeableConcept data
    /// types, and terminology binding principles.
    ///
    pub code: String,

    /// # uri
    ///
    /// ## Description
    ///
    /// The `uri` property contains Uniform Resource Identifier (URI) values
    /// that reference external resources, systems, namespaces, or identifiers.
    /// URIs provide globally unique identification and location information.
    ///
    /// ## Purpose
    ///
    /// - Reference external systems, APIs, or resources
    /// - Provide unique identifiers for code systems and namespaces
    /// - Enable linking to web-based resources and documentation
    /// - Support federated identity and resource management
    /// - Facilitate interoperability through standardized references
    ///
    /// ## Usage
    ///
    /// The `uri` property is used throughout FHIR resources to reference
    /// external entities, particularly for system identifiers, code system
    /// URIs, and resource references that extend beyond local FHIR servers.
    ///
    /// ## Data Type
    ///
    /// **uri** - A Uniform Resource Identifier conforming to RFC 3986
    ///
    /// ## Constraints
    ///
    /// - Must be a valid URI according to RFC 3986
    /// - Should be absolute URIs for interoperability
    /// - May include schemes like http, https, urn, oid, etc.
    /// - Should resolve to meaningful resources when applicable
    /// - Case-sensitive and must match exactly
    ///
    /// ## Examples
    ///
    /// ### System Identifier URI
    /// ```json
    /// {
    ///   "system": "http://terminology.hl7.org/CodeSystem/v3-AdministrativeGender",
    ///   "code": "F"
    /// }
    /// ```
    ///
    /// ### Extension URI
    /// ```json
    /// {
    ///   "url": "http://hl7.org/fhir/StructureDefinition/patient-birthPlace",
    ///   "valueAddress": {
    ///     "city": "Springfield",
    ///     "state": "IL"
    ///   }
    /// }
    /// ```
    ///
    /// ### Profile URI
    /// ```json
    /// {
    ///   "meta": {
    ///     "profile": [
    ///       "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `url` - Canonical URLs for resources
    /// - `system` - Code system URIs
    /// - `valueUri` - URI values in elements
    /// - `reference` - Resource references
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [uri](http://hl7.org/fhir/R5/datatypes.html#uri)
    ///
    pub uri: String,

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
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdditionalAttribute;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("additional_attribute")
            .join("additional_attribute.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.code, "spec-src");
    }
}
