//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::value_sets::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Property {
    /// Example: "priority"
    pub code: String,

    /// Example: TODO
    #[serde(rename = "_code")]
    pub underscore_code: Option<UnderscoreCode>,

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

    /// Example: "string"
    pub r#type: Option<String>,

    /// Example: TODO
    pub uri: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Property;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("property")
            .join("property.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
