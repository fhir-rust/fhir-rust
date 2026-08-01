//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use crate::r5::parse::value_sets::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Expansion {
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
    pub identifier: String,

    /// Example: "2015-06-22T13:56:07Z"
    pub timestamp: String,

    /// Example: 8
    pub total: Option<i64>,

    /// Example: 0
    pub offset: Option<i64>,

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
    pub extension: Option<Vec<Extension>>,

    /// TODO
    pub parameter: Option<Vec<Parameter>>,

    /// # property
    ///
    /// ## Description
    ///
    /// The `property` attribute defines additional properties and metadata
    /// associated with concepts in a CodeSystem. It provides structured
    /// information about concepts beyond the basic code, display, and
    /// definition, enabling rich semantic descriptions and supporting complex
    /// terminology operations. Properties can represent various aspects of
    /// concepts including relationships, classifications, and computational
    /// attributes.
    ///
    /// ## Purpose
    ///
    /// The `property` exists to provide extensible concept metadata that
    /// enables:
    ///
    /// - Rich semantic descriptions of terminology concepts
    /// - Support for complex terminology relationships and hierarchies
    /// - Computational attributes for terminology operations
    /// - Classification and categorization information
    /// - Version and lifecycle management of concepts
    /// - Integration with external terminology systems and standards
    ///
    /// ## Usage
    ///
    /// Use the `property` attribute when:
    ///
    /// - Defining concept metadata beyond basic identification
    /// - Implementing hierarchical relationships between concepts
    /// - Supporting advanced terminology operations and filtering
    /// - Providing classification and categorization information
    /// - Enabling computational processing of concepts
    /// - Supporting concept lifecycle and version management
    ///
    /// Properties are defined at the CodeSystem level and assigned values at
    /// the concept level.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - Property definition (at CodeSystem level):
    ///
    /// - `code` (code) - Identifies the property
    /// - `uri` (uri) - Formal identifier for the property
    /// - `description` (string) - Description of the property
    /// - `type` (code) - Data type (code, Coding, string, integer, boolean,
    ///   dateTime, decimal)
    ///
    /// **BackboneElement** - Property value (at concept level):
    ///
    /// - `code` (code) - Identifies which property
    /// - `value[x]` - The property value (type determined by property
    ///   definition)
    ///
    /// ## Constraints
    ///
    /// - **Required**: Code is required for both property definitions and
    ///   values
    /// - **Cardinality**: 0..* (zero to many occurrences)
    /// - **Type Consistency**: Property values must match the defined type
    /// - **Code Uniqueness**: Property codes should be unique within a
    ///   CodeSystem
    /// - **URI Uniqueness**: Property URIs should be globally unique when
    ///   present
    /// - **Value Validation**: Property values should conform to their defined
    ///   constraints
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete CodeSystem
    /// resource demonstrating the `property` attribute with various property
    /// types, relationships, and concept-level property values.
    ///
    /// ## Related Keys
    ///
    /// - `code` - Identifier for the property or concept
    /// - `uri` - Formal URI identifier for the property
    /// - `type` - Data type of property values
    /// - `value[x]` - Property value with type-specific suffix
    /// - `concept` - Parent concept containing property values
    /// - `description` - Human-readable property description
    /// - `filter` - Related element that can reference properties
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for CodeSystem resource and concept
    /// property definitions.
    ///
    pub property: Option<Vec<Property>>,

    /// TODO
    pub contains: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Expansion;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("expansion")
            .join("expansion.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
