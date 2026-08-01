//! # quantity
//!
//! ## Description
//!
//! The `quantity` property contains measured quantities with numeric values,
//! units, and optional comparators. It represents clinical measurements,
//! dosages, and quantified observations using standardized unit systems.
//!
//! ## Purpose
//!
//! - Store measured values with appropriate units
//! - Support standardized unit representation (UCUM)
//! - Enable unit conversion and comparison
//! - Provide precise quantitative clinical data
//! - Support mathematical operations on measurements
//!
//! ## Usage
//!
//! The `quantity` property is used in extensions, parameters, and data elements
//! where quantified measurements are required, such as vital signs, medication
//! dosages, or laboratory results.
//!
//! ## Data Type
//!
//! **Quantity** - A complex type containing:
//!
//! - `value` - Numeric value
//! - `comparator` - <, <=, >=, > (if applicable)
//! - `unit` - Human-readable unit
//! - `system` - Unit system (typically UCUM)
//! - `code` - Coded unit representation
//!
//! ## Constraints
//!
//! - Value should be a valid decimal number
//! - Unit system should be a recognized standard (preferably UCUM)
//! - Unit code should be valid within the specified system
//! - Comparator should be used only when exact values are not available
//! - Precision should be appropriate for the measurement context
//!
//! ## Examples
//!
//! ### Extension with Weight Measurement
//!
//! ```json
//! {
//!   "extension": [
//!     {
//!       "url": "http://example.org/fhir/StructureDefinition/baseline-weight",
//!       "quantity": {
//!         "value": 70.5,
//!         "unit": "kg",
//!         "system": "http://unitsofmeasure.org",
//!         "code": "kg"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ### Parameter with Temperature Range
//!
//! ```json
//! {
//!   "parameter": [
//!     {
//!       "name": "storageTemperature",
//!       "quantity": {
//!         "value": 4,
//!         "comparator": "<=",
//!         "unit": "degrees Celsius",
//!         "system": "http://unitsofmeasure.org",
//!         "code": "Cel"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `valueDecimal` - Simple decimal values
//! - `valueInteger` - Integer values
//! - `quantity` - Direct quantity objects
//! - `value` - Numeric values
//! - `unit` - Unit specifications
//!
//! ## Specification Reference
//!
//! FHIR R5 Data Types:
//! [Quantity](http://hl7.org/fhir/R5/datatypes.html#Quantity)

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Quantity {
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
    pub value: i64,

    /// # comparator
    ///
    /// ## Description
    ///
    /// The `comparator` property defines comparison operators that can be used
    /// with a search parameter. It specifies which mathematical or logical
    /// comparisons are supported when searching with this parameter.
    ///
    /// ## Purpose
    ///
    /// - Define supported comparison operations for search parameters
    /// - Enable range and inequality searches
    /// - Support numeric and date comparisons
    /// - Provide clear API capabilities documentation
    /// - Optimize search query performance
    ///
    /// ## Usage
    ///
    /// The `comparator` property is used in SearchParameter definitions to
    /// specify which comparison operators (eq, ne, gt, lt, ge, le, sa, eb, ap)
    /// can be used when searching with this parameter.
    ///
    /// ## Data Type
    ///
    /// **array** of **code** - Each code represents a supported comparison
    /// operator
    ///
    /// **Valid comparator codes:**
    ///
    /// - `eq` - equal
    /// - `ne` - not equal
    /// - `gt` - greater than
    /// - `ge` - greater than or equal
    /// - `lt` - less than
    /// - `le` - less than or equal
    /// - `sa` - starts after
    /// - `eb` - ends before
    /// - `ap` - approximately
    ///
    /// ## Constraints
    ///
    /// - Must contain valid comparator codes
    /// - Should be appropriate for the parameter's data type
    /// - Not all comparators make sense for all data types
    /// - Empty array means only equality comparison is supported
    ///
    /// ## Examples
    ///
    /// ### Numeric Parameter with Full Range Support
    ///
    /// ```json
    /// {
    ///   "code": "value-quantity",
    ///   "type": "quantity",
    ///   "comparator": ["eq", "ne", "gt", "ge", "lt", "le", "ap"]
    /// }
    /// ```
    ///
    /// ### Date Parameter with Range Support
    ///
    /// ```json
    /// {
    ///   "code": "date",
    ///   "type": "date",
    ///   "comparator": ["eq", "ne", "gt", "ge", "lt", "le", "sa", "eb", "ap"]
    /// }
    /// ```
    ///
    /// ### String Parameter (No Comparators)
    ///
    /// ```json
    /// {
    ///   "code": "name",
    ///   "type": "string",
    ///   "comparator": []
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `code` - The search parameter name
    /// - `type` - The search parameter data type
    /// - `modifier` - Other search modifiers supported
    /// - `expression` - FHIRPath expression for the search
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 SearchParameter:
    /// [Comparator](http://hl7.org/fhir/R5/searchparameter.html#SearchParameter.comparator)
    ///
    pub comparator: String,

    /// # unit
    ///
    /// ## Description
    ///
    /// The `unit` property specifies the human-readable unit of measure for
    /// quantities and measurements in FHIR resources.
    ///
    /// ## Purpose
    ///
    /// - Provide human-readable unit labels
    /// - Support quantity interpretation and display
    /// - Enable unit-aware calculations
    /// - Facilitate clinical decision making
    /// - Support measurement validation
    ///
    /// ## Usage
    ///
    /// The `unit` property is used in Quantity data types to provide
    /// human-readable unit descriptions.
    ///
    /// ## Data Type
    ///
    /// **string** - Human-readable unit description
    ///
    /// ## Constraints
    ///
    /// - Should be meaningful to humans
    /// - Should align with coded unit (if present)
    /// - Must represent the actual unit
    /// - Should follow standard conventions
    ///
    /// ## Examples
    ///
    /// ### Weight Unit
    ///
    /// ```json
    /// {
    ///   "valueQuantity": {
    ///     "value": 70.5,
    ///     "unit": "kg",
    ///     "system": "http://unitsofmeasure.org",
    ///     "code": "kg"
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `code` - Coded units
    /// - `system` - Unit code systems
    /// - `value` - Quantity values
    /// - `comparator` - Value comparators
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Quantity:
    /// [unit](http://hl7.org/fhir/R5/datatypes-definitions.html#Quantity.unit)
    ///
    pub unit: String,

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
    ///
    /// - Proper interpretation of codes within their correct context
    /// - Validation of codes against their source terminology
    /// - Disambiguation when the same code exists in multiple systems
    /// - Support for terminology services and code system maintenance
    /// - Interoperability through standardized system identifiers
    ///
    /// ## Usage
    ///
    /// Use the `system` attribute when:
    ///
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
    /// ## Example
    ///
    /// Example: "http://unitsofmeasure.org"
    ///
    pub system: String,

    /// # code
    ///
    /// ## Description
    ///
    /// The `code` attribute represents a coded value that identifies a specific
    /// concept from a terminology system. It is a fundamental building block in
    /// FHIR R5 for representing standardized concepts and is commonly used
    /// within CodeableConcept and Coding data types to provide machine-readable
    /// identifiers for clinical and administrative concepts.
    ///
    /// ## Purpose
    ///
    /// The `code` exists to provide standardized, machine-readable identifiers
    /// for concepts across healthcare systems. This enables:
    /// - Interoperability between different healthcare systems
    /// - Consistent representation of clinical concepts
    /// - Support for clinical decision support and analytics
    /// - Mapping between different terminology systems
    /// - Precise semantic meaning in healthcare data exchange
    ///
    /// ## Usage
    ///
    /// Use the `code` attribute when:
    /// - Representing a specific concept from a code system (diagnoses,
    ///   procedures, medications)
    /// - Creating CodeableConcept or Coding structures
    /// - Implementing standardized vocabularies (ICD-10, SNOMED CT, LOINC)
    /// - Ensuring semantic interoperability between systems
    /// - Supporting automated processing and clinical decision support
    ///
    /// The `code` should always be paired with a `system` that identifies the
    /// terminology from which the code is drawn.
    ///
    /// ## Data Type
    ///
    /// **string** - A sequence of characters representing the identifier within
    /// a code system:
    /// - Case sensitivity depends on the code system definition
    /// - Length and format constraints are defined by the specific terminology
    ///   system
    /// - May contain alphanumeric characters, hyphens, periods, and other
    ///   characters as allowed by the system
    /// - Should not contain leading or trailing whitespace
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - Required when used within a Coding,
    ///   optional in CodeableConcept when only text is provided
    /// - **Cardinality**: 0..1 (zero to one occurrence within a Coding)
    /// - **Format**: Must conform to the code format rules of the specified
    ///   system
    /// - **Validation**: Should be a valid code in the specified system
    /// - **Case Sensitivity**: Follows the case sensitivity rules of the code
    ///   system
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating various uses of the `code` attribute in different
    /// contexts.
    ///
    /// ## Related Keys
    ///
    /// - `system` - URI that identifies the code system containing this code
    /// - `display` - Human-readable representation of the code
    /// - `coding` - Array containing code/system/display triplets
    /// - `text` - Free text representation when no suitable code exists
    /// - `version` - Version of the code system when relevant
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Coding and CodeableConcept data
    /// types, and terminology binding principles.
    ///
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Quantity;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("quantity")
            .join("quantity.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.system, "http://unitsofmeasure.org");
    }
}
