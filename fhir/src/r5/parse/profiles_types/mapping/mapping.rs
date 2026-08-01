//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    /// Example: "identity" : "v2"
    pub identity: String,

    /// Example: "uri" : "http://hl7.org/v2"
    pub uri: String,

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
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Mapping;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_types::DIR
            .join("mapping")
            .join("mapping.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
