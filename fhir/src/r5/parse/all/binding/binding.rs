//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Binding {
    /// TODO. Example: "required"
    pub strength: String,

    /// # valueSet
    ///
    /// ## Description
    ///
    /// The `valueSet` attribute references another ValueSet resource to include
    /// or exclude its contents in the current ValueSet composition. It enables
    /// modular value set construction by allowing one value set to incorporate
    /// concepts from other value sets, supporting reusable terminology building
    /// blocks and hierarchical value set organization. This promotes
    /// consistency and reduces duplication across related value sets.
    ///
    /// ## Purpose
    ///
    /// The `valueSet` exists to enable modular value set composition that
    /// supports:
    ///
    /// - Reusable terminology building blocks and components
    /// - Hierarchical organization of related value sets
    /// - Consistent concept grouping across multiple value sets
    /// - Reduced maintenance overhead through shared components
    /// - Flexible composition patterns for complex terminology requirements
    /// - Support for organizational and domain-specific value set libraries
    ///
    /// ## Usage
    ///
    /// Use the `valueSet` attribute when:
    ///
    /// - Including concepts from other value sets in your composition
    /// - Building hierarchical value set structures
    /// - Creating modular terminology components for reuse
    /// - Excluding concepts that are defined in other value sets
    /// - Implementing organizational value set inheritance patterns
    /// - Supporting complex terminology requirements through composition
    ///
    /// ValueSet references are resolved during expansion to incorporate the
    /// referenced concepts.
    ///
    /// ## Data Type
    ///
    /// **canonical** - A canonical URL reference to another ValueSet resource:
    ///
    /// - Must be a valid canonical URL format
    /// - Should resolve to an accessible ValueSet resource
    /// - May include version information using the |version syntax
    /// - Can reference value sets in the same system or external systems
    ///
    /// ## Constraints
    ///
    /// - **Required**: URI is required when valueSet is present
    /// - **Cardinality**: 0..* (zero to many occurrences within
    ///   include/exclude)
    /// - **Resolution**: Referenced value sets must be resolvable during
    ///   expansion
    /// - **Circular References**: Must not create circular reference patterns
    /// - **Version Consistency**: Version references should align with
    ///   available versions
    /// - **Access**: Referenced value sets must be accessible to the
    ///   terminology server
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete ValueSet
    /// resource demonstrating the `valueSet` attribute with multiple
    /// references, versioned references, and modular composition patterns.
    ///
    /// ## Related Keys
    ///
    /// - `include` - Container element that can reference value sets to include
    /// - `exclude` - Container element that can reference value sets to exclude
    /// - `url` - Canonical URL of the current or referenced value set
    /// - `version` - Version specification for referenced value sets
    /// - `compose` - Parent element containing value set references
    /// - `expansion` - Result that incorporates concepts from referenced value
    ///   sets
    /// - `identifier` - Alternative identifier for value set references
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ValueSet resource and composition
    /// principles.
    ///
    pub value_set: Option<String>,

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

    /// # additional
    ///
    /// ## Description
    ///
    /// The `additional` property defines additional search parameters that can
    /// be used when searching within a specific resource compartment. It
    /// extends the basic search capabilities with resource-specific parameters
    /// that are meaningful for that compartment context.
    ///
    /// ## Purpose
    ///
    /// - Specify compartment-specific search parameters beyond standard ones
    /// - Enable more refined searches within resource compartments
    /// - Provide resource-type-aware search capabilities
    /// - Support specialized search patterns for different FHIR resources
    ///
    /// ## Usage
    ///
    /// The `additional` property is used within CompartmentDefinition resources
    /// to specify extra search parameters that are available when searching for
    /// resources within that compartment. These parameters supplement the
    /// standard search parameters.
    ///
    /// ## Data Type
    ///
    /// **array** of **string** - Each string represents a search parameter name
    /// that is additionally supported for the compartment
    ///
    /// ## Constraints
    ///
    /// - Must be an array of valid search parameter names
    /// - Search parameters must be defined and supported by the server
    /// - Parameters should be relevant to the compartment context
    /// - Empty array is valid if no additional parameters are needed
    ///
    /// ## Examples
    ///
    /// ### Patient Compartment with Additional Search Parameters
    ///
    /// ```json
    /// {
    ///   "resourceType": "CompartmentDefinition",
    ///   "resource": [
    ///     {
    ///       "code": "Observation",
    ///       "param": ["subject", "patient"],
    ///       "additional": ["category", "code", "value-concept"]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Encounter Compartment with Additional Parameters
    ///
    /// ```json
    /// {
    ///   "resource": [
    ///     {
    ///       "code": "DiagnosticReport",
    ///       "param": ["encounter"],
    ///       "additional": ["status", "category", "date"]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `param` - Basic search parameters for the compartment resource
    /// - `code` - Resource type code for which the parameters apply
    /// - `resource` - Array of resource definitions within the compartment
    /// - `documentation` - Human-readable description of the compartment
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 CompartmentDefinition: [Resource - Additional
    /// Parameters](http://hl7.org/fhir/R5/compartmentdefinition.html#CompartmentDefinition.resource.additional)
    ///
    pub additional: Option<Vec<Additional>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Binding;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("binding")
            .join("binding.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.strength, "my strength");
    }
}
