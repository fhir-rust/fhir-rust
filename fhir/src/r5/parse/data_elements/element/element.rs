//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use crate::r5::parse::data_elements::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Element {
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

    /// Example: "string.value"
    pub path: String,

    /// Example: ["xmlAttr"]
    pub representation: Option<Vec<String>>,

    /// Example: "Primitive value for string"
    pub short: Option<String>,

    /// Example: "The actual value"
    pub definition: Option<String>,

    /// Example: ["Municpality"]
    pub alias: Option<Vec<String>>,

    /// Example: TODO
    pub comment: Option<String>,

    /// Example: "Allows an appropriate address to be chosen from a list of many."
    pub requirements: Option<String>,

    /// Example: "The order in which lines should appear in an address label"
    pub order_meaning: Option<String>,

    /// Example: "Patient"
    pub meaning_when_missing: Option<String>,

    /// Example: 0
    pub min: Option<u32>,

    /// Example: "1"
    pub max: Option<String>,

    /// Example: { "path": "Element.id", "min": 0, "max": "1" }
    pub base: Option<Base>,

    /// Example: [{ "extension": … }]
    pub r#type: Option<Vec<ElementType>>,

    /// Example: 1048576
    pub max_length: Option<i32>,

    /// Example:  -2147483648,
    pub min_value_integer: Option<i64>,

    /// Example: 2147483647
    pub max_value_integer: Option<i64>,

    /// Example: "-9223372036854775808"
    pub min_value_integer_64: Option<String>,

    /// Example: "9223372036854775807"
    pub max_value_integer_64: Option<String>,

    // /// TODO
    pub constraint: Option<Vec<Constraint>>,

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

    // /// TODO
    pub slicing: Option<Slicing>,

    // /// TODO common properties
    // pub alias: Option<Vec<String>>,
    // pub fixed_value: Option<serde_json::Value>,
    // pub pattern_value: Option<serde_json::Value>,
    /// TODO
    pub example: Option<Vec<Example>>,

    // // Additional properties
    // #[serde(flatten)]
    // pub additional: std::collections::HashMap<String, serde_json::Value>,
    /// Example: ["ele-1"]
    pub condition: Option<Vec<String>>,

    /// Example: false
    pub is_modifier: Option<bool>,

    /// Example: "This is labeled as \"Is Modifier\" because applications should not mistake a temporary or old address etc.for a current/permanent one",
    pub is_modifier_reason: Option<String>,

    /// Example: false
    pub is_summary: Option<bool>,

    /// TODO
    pub binding: Option<Binding>,

    /// Example: TODO
    pub mapping: Option<::serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Element;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::data_elements::DIR
            .join("element")
            .join("element.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
