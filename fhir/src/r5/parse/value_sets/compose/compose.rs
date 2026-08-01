//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Compose {
    /// TODO
    pub include: Vec<::serde_json::Value>,

    /// TODO
    pub exclude: Option<Vec<::serde_json::Value>>,

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
    pub property: Option<Vec<String>>,

    /// Example: "2012-06-13",
    pub locked_date: Option<String>,

    /// TODO
    pub inactive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Compose;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("compose")
            .join("compose.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
