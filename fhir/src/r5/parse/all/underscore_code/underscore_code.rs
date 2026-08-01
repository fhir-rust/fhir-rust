//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UnderscoreCode {
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
    pub extension: Vec<Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = UnderscoreCode;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("underscore_code")
            .join("underscore_code.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.extension.first().unwrap().url,
            "http://hl7.org/fhir/StructureDefinition/rendered-value"
        );
    }
}
