//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    /// # system
    ///
    /// ## Description
    ///
    /// The `system` attribute is a URI that identifies the code system or
    /// terminology from which a code is drawn. It serves as the namespace that
    /// gives meaning and context to coded values in FHIR R5, ensuring that
    /// codes can be properly interpreted and validated within their appropriate
    /// terminology framework.
    ///
    /// ## Purpose
    ///
    /// The `system` exists to provide unambiguous identification of terminology
    /// systems, enabling:
    /// - Proper interpretation of codes within their correct context
    /// - Validation of codes against their source terminology
    /// - Disambiguation when the same code exists in multiple systems
    /// - Support for terminology services and code system maintenance
    /// - Interoperability through standardized system identifiers
    ///
    /// ## Usage
    ///
    /// Use the `system` attribute when:
    /// - Specifying codes within Coding data types
    /// - Creating references to standard terminologies (SNOMED CT, LOINC,
    ///   ICD-10)
    /// - Implementing local code systems with custom URIs
    /// - Ensuring codes can be validated against their source system
    /// - Supporting terminology binding in FHIR profiles
    ///
    /// The system URI should be persistent and resolvable when possible,
    /// following established conventions for terminology system identification.
    ///
    /// ## Data Type
    ///
    /// **uri** - A Uniform Resource Identifier that identifies the code system:
    /// - Must be an absolute URI
    /// - Should be persistent and stable over time
    /// - Often follows established patterns for well-known terminologies
    /// - May include version information in some cases
    /// - Should be resolvable to terminology services when possible
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes when used within a Coding
    /// - **Cardinality**: 1..1 (exactly one occurrence in a Coding)
    /// - **Format**: Must be a valid absolute URI
    /// - **Stability**: Should remain consistent for the same code system
    /// - **Registration**: Well-known systems should use registered URIs
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete DiagnosticReport
    /// resource demonstrating various `system` URIs for different terminology
    /// systems.
    ///
    /// ## Related Keys
    ///
    /// - `code` - The specific identifier within the system
    /// - `display` - Human-readable representation defined by the system
    /// - `version` - Specific version of the system when relevant
    /// - `coding` - Contains the system along with code and display
    /// - `valueSet` - References sets of codes from specific systems
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Coding data types, terminology
    /// binding, and the registry of known code system URIs.
    ///
    pub system: String,

    /// # value
    ///
    /// ## Description
    ///
    /// The `value` attribute serves as a generic field prefix used throughout
    /// FHIR R5 to represent the actual data content of various data types and
    /// elements. It appears in numerous contexts as part of polymorphic value
    /// fields (like value[x] in extensions and observations) and as the core
    /// data element in primitive data types and complex structures.
    ///
    /// ## Purpose
    ///
    /// The `value` exists to provide a consistent naming pattern for data
    /// content across FHIR, enabling:
    /// - Polymorphic data representation through value[x] patterns
    /// - Consistent structure for primitive and complex data types
    /// - Clear separation between metadata and actual data content
    /// - Support for type-specific value representations
    /// - Uniform handling of data values in processing systems
    ///
    /// ## Usage
    ///
    /// Use the `value` attribute when:
    /// - Creating Extension elements with typed values (valueString,
    ///   valueInteger, etc.)
    /// - Representing measurement values in Quantity data types
    /// - Storing actual data content in Observation.value[x] fields
    /// - Implementing primitive data types with additional metadata
    /// - Creating parameters with typed value content
    ///
    /// The value field is often suffixed with a type indicator (e.g.,
    /// valueString, valueCodeableConcept) to specify the data type.
    ///
    /// ## Data Type
    ///
    /// **varies** - The data type depends on the specific context and type
    /// suffix:
    /// - **Primitive types**: string, integer, decimal, boolean, date,
    ///   dateTime, etc.
    /// - **Complex types**: CodeableConcept, Quantity, Reference, Period, etc.
    /// - **Special types**: base64Binary, code, uri, url, canonical
    /// - Type safety ensured through polymorphic naming (value[x])
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - depends on the specific context and
    ///   requirements
    /// - **Cardinality**: Typically 0..1 or 1..1 depending on usage context
    /// - **Type Constraints**: Must conform to the specified data type
    ///   requirements
    /// - **Validation**: Subject to type-specific validation rules
    /// - **Polymorphism**: Only one value[x] variant allowed per element
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating various uses of the `value` attribute in
    /// different typed contexts.
    ///
    /// ## Related Keys
    ///
    /// - `unit` - Unit of measure when value represents a quantity
    /// - `system` - System reference for coded values
    /// - `code` - Coded representation when value is part of coded concepts
    /// - `extension` - Contains value[x] fields for additional data
    /// - `component` - May contain value[x] fields in complex observations
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for data types, polymorphic elements, and
    /// the value[x] pattern implementation.
    ///
    pub value: String,

    /// # use
    ///
    /// ## Description
    ///
    /// The `use` property specifies the purpose or context in which an element
    /// should be used, such as the use of a name, address, or contact point.
    ///
    /// ## Purpose
    ///
    /// - Identify the purpose of an element
    /// - Enable context-appropriate processing
    /// - Support element selection based on use
    /// - Facilitate appropriate display and handling
    /// - Enable use-based validation
    ///
    /// ## Usage
    ///
    /// The `use` property is commonly found in HumanName, Address,
    /// ContactPoint, and Identifier data types.
    ///
    /// ## Data Type
    ///
    /// **code** - A coded use value from appropriate value sets
    ///
    /// ## Constraints
    ///
    /// - Must be from defined value sets
    /// - Should align with element purpose
    /// - Must be supported by context
    /// - Should enable appropriate processing
    ///
    /// ## Examples
    ///
    /// ### Name Use
    ///
    /// ```json
    /// {
    ///   "name": [{
    ///     "use": "official",
    ///     "family": "Smith",
    ///     "given": ["John"]
    ///   }]
    /// }
    /// ```
    ///
    /// ### Address Use
    ///
    /// ```json
    /// {
    ///   "address": [{
    ///     "use": "home",
    ///     "line": ["123 Main St"],
    ///     "city": "Anytown"
    ///   }]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `type` - Element types
    /// - `period` - Usage periods
    /// - `system` - System identifiers
    /// - `value` - Element values
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [use](http://hl7.org/fhir/R5/datatypes.html)
    ///
    pub r#use: Option<String>,

    /// TODO
    pub rank: Option<u32>,

    /// TODO
    pub period: Option<Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Identifier;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("contact_point")
            .join("contact_point.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.system, "phone");
    }
}
