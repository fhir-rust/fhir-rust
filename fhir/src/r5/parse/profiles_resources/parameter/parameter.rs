//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
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
    pub name: String,

    /// Example: "string"
    pub r#type: Option<String>,

    /// Example: "reference"
    pub search_type: Option<String>,

    /// Example: "in"
    pub r#use: Option<String>,

    /// Example: ["type"],
    pub scope: Option<Vec<String>>,

    /// Example: 0
    pub min: Option<i64>,

    /// Example: "1"
    pub max: Option<String>,

    /// Example: "The activity definition to apply…",
    pub documentation: Option<String>,

    /// TODO
    pub binding: Option<Binding>,

    /// Example: TODO
    pub target_profile: Option<Vec<String>>,

    /// Recursive
    pub part: Option<Vec<Parameter>>,

    /// TODO
    pub value_string: Option<String>,

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
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Parameter;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("parameter")
            .join("parameter.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
