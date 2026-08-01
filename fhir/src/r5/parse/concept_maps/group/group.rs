//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Group {
    /// # source
    ///
    /// ## Description
    ///
    /// The `source` property identifies the source of data, mappings, or
    /// transformations in FHIR resources. It can reference systems, documents,
    /// or other sources that provided the information being processed or
    /// mapped.
    ///
    /// ## Purpose
    ///
    /// - Identify the origin of data or mappings
    /// - Support data lineage and provenance tracking
    /// - Enable source-specific processing rules
    /// - Facilitate data quality and validation
    /// - Support transformation and mapping operations
    ///
    /// ## Usage
    ///
    /// The `source` property is used in ConceptMap resources, data
    /// transformation contexts, and other scenarios where identifying the
    /// source of information is important for processing or validation.
    ///
    /// ## Data Type
    ///
    /// **uri** or **canonical** - Reference to the source system, document, or
    /// resource
    ///
    /// ## Constraints
    ///
    /// - Should be a valid URI or canonical reference
    /// - Must identify a retrievable or recognizable source
    /// - Should be consistent across related mappings or transformations
    /// - Should support the intended use case for source identification
    /// - May reference external systems or internal FHIR resources
    ///
    /// ## Examples
    ///
    /// ### ConceptMap Source
    ///
    /// ```json
    /// {
    ///   "resourceType": "ConceptMap",
    ///   "source": "http://hl7.org/fhir/ValueSet/administrative-gender",
    ///   "target": "http://example.org/fhir/ValueSet/local-gender-codes"
    /// }
    /// ```
    ///
    /// ### Mapping with Source Reference
    ///
    /// ```json
    /// {
    ///   "group": [
    ///     {
    ///       "source": "http://terminology.hl7.org/CodeSystem/v3-AdministrativeGender",
    ///       "target": "http://example.org/CodeSystem/gender"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `target` - Target resources or systems
    /// - `sourceUri` - Source URI references
    /// - `sourceScope` - Source scope definitions
    /// - `map` - Mapping definitions
    /// - `system` - Code system references
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 ConceptMap:
    /// [source](http://hl7.org/fhir/R5/conceptmap-definitions.html#ConceptMap.source_x_)
    ///
    pub source: String,

    /// # target
    ///
    /// ## Description
    ///
    /// The `target` property specifies the allowed resource types that can be
    /// referenced by a reference-type element or search parameter. It
    /// constrains which resource types are valid targets for references.
    ///
    /// ## Purpose
    ///
    /// - Define allowed target resource types for references
    /// - Constrain reference relationships between resources
    /// - Enable validation of reference integrity
    /// - Support type-specific reference processing
    /// - Guide client reference resolution behavior
    ///
    /// ## Usage
    ///
    /// The `target` property is used in SearchParameter definitions,
    /// StructureDefinition elements, and other contexts where reference types
    /// need to be constrained to specific resource types.
    ///
    /// ## Data Type
    ///
    /// **code** - Array of resource type codes
    ///
    /// ## Constraints
    ///
    /// - Must contain valid FHIR resource type names
    /// - Should be consistent with the reference element's purpose
    /// - Empty array means no specific target constraints
    /// - Should be implementable by the server
    /// - Must align with business rules and use cases
    ///
    /// ## Examples
    ///
    /// ### Search Parameter Target Types
    /// ```json
    /// {
    ///   "resourceType": "SearchParameter",
    ///   "name": "patient",
    ///   "type": "reference",
    ///   "target": [
    ///     "Patient"
    ///   ]
    /// }
    /// ```
    ///
    /// ### Multiple Target Types
    /// ```json
    /// {
    ///   "name": "performer",
    ///   "type": "reference",
    ///   "target": [
    ///     "Practitioner",
    ///     "PractitionerRole",
    ///     "Organization",
    ///     "Patient",
    ///     "RelatedPerson"
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `source` - Source resource types
    /// - `reference` - Reference data types
    /// - `type` - Element or parameter types
    /// - `resourceType` - Resource type identifiers
    /// - `targetProfile` - Specific profile targets
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 SearchParameter:
    /// [target](http://hl7.org/fhir/R5/searchparameter-definitions.html#SearchParameter.target)
    ///
    pub target: String,

    /// # element
    ///
    /// ## Description
    ///
    /// The `element` attribute contains an array of ElementDefinition objects
    /// that define the structure, constraints, and properties of individual
    /// elements within a FHIR resource or data type. Each ElementDefinition
    /// provides comprehensive metadata about a specific element including its
    /// data type, cardinality, constraints, bindings, and other properties
    /// essential for validation and implementation.
    ///
    /// ## Purpose
    ///
    /// The `element` exists to provide detailed specifications for resource
    /// structure by:
    ///
    /// - Defining the properties and constraints for each element in the
    ///   structure
    /// - Specifying data types, cardinality, and validation rules
    /// - Establishing bindings to terminology systems
    /// - Supporting profiling through constraints and extensions
    /// - Enabling automated validation and code generation
    /// - Providing complete metadata for implementation guidance
    ///
    /// ## Usage
    ///
    /// Use the `element` attribute when:
    ///
    /// - Defining the structure of resources in StructureDefinition
    /// - Specifying constraints and requirements in profiles
    /// - Adding new elements through extensions
    /// - Establishing validation rules and invariants
    /// - Binding elements to value sets and code systems
    /// - Creating implementation-specific requirements
    ///
    /// Each ElementDefinition in the array represents one element or
    /// sub-element in the resource structure.
    ///
    /// ## Data Type
    ///
    /// **ElementDefinition[]** - Array of ElementDefinition objects, each
    /// containing:
    ///
    /// - **path** - The path to the element within the structure
    /// - **min/max** - Cardinality constraints (minimum and maximum
    ///   occurrences)
    /// - **type** - Data type specifications
    /// - **binding** - Value set bindings for coded elements
    /// - **constraint** - Additional validation rules
    /// - **mustSupport** - Implementation requirements
    /// - And many other properties for comprehensive element definition
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - Required in differential when making
    ///   changes, optional in snapshot
    /// - **Cardinality**: 0..* (zero or more occurrences)
    /// - **Ordering**: Must follow canonical FHIR element ordering in snapshot
    /// - **Paths**: All paths must be valid within the base structure
    /// - **Inheritance**: Constraints must be compatible with base definition
    /// - **Consistency**: Element definitions must be internally consistent
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various ElementDefinition
    /// patterns including constraints, extensions, slicing, and bindings.
    ///
    /// ## Related Keys
    ///
    /// - `path` - The path identifier for each element
    /// - `differential` - Container for changed elements in profiles
    /// - `snapshot` - Container for complete element definitions
    /// - `binding` - Value set bindings within element definitions
    /// - `constraint` - Additional validation rules for elements
    /// - `type` - Data type specifications for elements
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ElementDefinition data type and
    /// StructureDefinition resource.
    ///
    pub element: Vec<Element>,

    /// # unmapped
    ///
    /// ## Description
    ///
    /// The `unmapped` property defines what should happen when a source concept
    /// cannot be mapped to any target concept in a ConceptMap. It provides
    /// fallback behavior for handling unmappable values during code
    /// translation.
    ///
    /// ## Purpose
    ///
    /// - Define fallback behavior for unmappable source concepts
    /// - Ensure graceful handling of missing mappings
    /// - Support different strategies for unmapped values
    /// - Enable controlled error handling in code translation
    /// - Provide consistent behavior across mapping operations
    ///
    /// ## Usage
    ///
    /// The `unmapped` property is used in ConceptMap group elements to specify
    /// how to handle source concepts that don't have explicit target mappings
    /// defined.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - A complex structure containing:
    /// - `mode` - Strategy for handling unmapped values
    /// - `code` - Fixed code to use (when mode is "fixed")
    /// - `display` - Display text for the unmapped handling
    /// - `otherMap` - Reference to another ConceptMap (when mode is
    ///   "other-map")
    ///
    /// ## Constraints
    ///
    /// - Mode must be a valid unmapped mode value
    /// - Code should be provided when mode is "fixed"
    /// - OtherMap should be provided when mode is "other-map"
    /// - Should provide meaningful fallback behavior
    /// - Should be consistent with business requirements
    ///
    /// ## Examples
    ///
    /// ### Fixed Code for Unmapped Values
    ///
    /// ```json
    /// {
    ///   "unmapped": {
    ///     "mode": "fixed",
    ///     "code": "unknown",
    ///     "display": "Unknown/Unmapped value"
    ///   }
    /// }
    /// ```
    ///
    /// ### Other ConceptMap Fallback
    ///
    /// ```json
    /// {
    ///   "unmapped": {
    ///     "mode": "other-map",
    ///     "otherMap": "http://example.org/fhir/ConceptMap/fallback-mapping"
    ///   }
    /// }
    /// ```
    ///
    /// ### Provided Mode
    ///
    /// ```json
    /// {
    ///   "unmapped": {
    ///     "mode": "provided",
    ///     "display": "Use the provided source code as-is"
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `source` - Source concept systems
    /// - `target` - Target concept systems
    /// - `mode` - Unmapped handling modes
    /// - `otherMap` - Fallback ConceptMaps
    /// - `element` - Mapping elements
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 ConceptMap:
    /// [unmapped](http://hl7.org/fhir/R5/conceptmap-definitions.html#ConceptMap.group.unmapped)
    ///
    pub unmapped: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Group;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("group")
            .join("group.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
