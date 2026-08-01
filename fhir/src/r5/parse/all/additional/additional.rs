//! # additional
//!
//! ## Description
//!
//! The `additional` property defines additional search parameters that can be
//! used when searching within a specific resource compartment. It extends the
//! basic search capabilities with resource-specific parameters that are
//! meaningful for that compartment context.
//!
//! ## Purpose
//!
//! - Specify compartment-specific search parameters beyond standard ones
//! - Enable more refined searches within resource compartments
//! - Provide resource-type-aware search capabilities
//! - Support specialized search patterns for different FHIR resources
//!
//! ## Usage
//!
//! The `additional` property is used within CompartmentDefinition resources to
//! specify extra search parameters that are available when searching for
//! resources within that compartment. These parameters supplement the standard
//! search parameters.
//!
//! ## Data Type
//!
//! **array** of **string** - Each string represents a search parameter name
//! that is additionally supported for the compartment
//!
//! ## Constraints
//!
//! - Must be an array of valid search parameter names
//! - Search parameters must be defined and supported by the server
//! - Parameters should be relevant to the compartment context
//! - Empty array is valid if no additional parameters are needed
//!
//! ## Examples
//!
//! ### Patient Compartment with Additional Search Parameters
//!
//! ```json
//! {
//!   "resourceType": "CompartmentDefinition",
//!   "resource": [
//!     {
//!       "code": "Observation",
//!       "param": ["subject", "patient"],
//!       "additional": ["category", "code", "value-concept"]
//!     }
//!   ]
//! }
//! ```
//!
//! ### Encounter Compartment with Additional Parameters
//!
//! ```json
//! {
//!   "resource": [
//!     {
//!       "code": "DiagnosticReport",
//!       "param": ["encounter"],
//!       "additional": ["status", "category", "date"]
//!     }
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `param` - Basic search parameters for the compartment resource
//! - `code` - Resource type code for which the parameters apply
//! - `resource` - Array of resource definitions within the compartment
//! - `documentation` - Human-readable description of the compartment
//!
//! ## Specification Reference
//!
//! FHIR R5 CompartmentDefinition: [Resource - Additional
//! Parameters](http://hl7.org/fhir/R5/compartmentdefinition.html#CompartmentDefinition.resource.additional)

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Additional {
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
    pub purpose: String,

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
    pub value_set: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Additional;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("additional")
            .join("additional.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.purpose, "starter");
    }
}
