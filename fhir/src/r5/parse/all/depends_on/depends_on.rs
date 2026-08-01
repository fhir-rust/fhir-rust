//! # dependsOn
//!
//! ## Description
//!
//! The `dependsOn` field specifies dependencies that a resource has on other
//! resources or external systems. This field indicates what other resources,
//! terminologies, or systems must be available for the current resource to
//! function properly.
//!
//! ## Purpose
//!
//! - Document resource dependencies for proper implementation
//! - Enable dependency resolution and management
//! - Support implementation planning and deployment
//! - Facilitate resource validation and integrity checking
//! - Guide system integration requirements
//!
//! ## Usage
//!
//! The `dependsOn` field is commonly used in:
//!
//! - **ImplementationGuide**: Dependencies on other implementation guides
//! - **ConceptMap**: Dependencies on source and target systems
//! - **ValueSet**: Dependencies on included code systems
//! - **StructureDefinition**: Dependencies on base profiles
//! - **Library**: Dependencies on other libraries or terminologies
//!
//! ## Data Type
//!
//! - **Type**: BackboneElement
//! - **Cardinality**: 0..*
//! - **Components**:
//!   - `uri`: URI of the dependent resource
//!   - `packageId`: Package identifier for dependency
//!   - `version`: Required version of dependency
//!
//! ## Constraints
//!
//! - Dependencies must be resolvable and available
//! - Version specifications should be precise when needed
//! - Circular dependencies should be avoided
//! - All dependencies must be properly documented
//!
//! ## Examples
//!
//! See the accompanying `example.json` for practical usage examples.
//!
//! ## Related Keys
//!
//! - `imports`: Resources imported by this resource
//! - `url`: Canonical URL of the resource
//! - `version`: Version of the current resource
//! - `fhirVersion`: FHIR version dependency
//! - `packageId`: Package containing the resource
//!
//! ## Specification Reference
//!
//! - [FHIR R5
//!   ImplementationGuide](https://hl7.org/fhir/R5/implementationguide.html)
//! - [FHIR R5 ConceptMap](https://hl7.org/fhir/R5/conceptmap.html)
//! - [FHIR R5
//!   Dependencies](https://hl7.org/fhir/R5/implementationguide-definitions.html#ImplementationGuide.dependsOn)

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DependsOn {
    /// # attribute
    ///
    /// ## Description
    ///
    /// The `attribute` property defines metadata properties that can be
    /// assigned to concepts or resources. Attributes provide additional
    /// descriptive information, classification data, or specialized properties
    /// that extend the base resource model.
    ///
    /// ## Purpose
    ///
    /// - Define custom properties for concepts and resources
    /// - Support extensible metadata frameworks
    /// - Enable domain-specific classification systems
    /// - Provide structured annotation capabilities
    /// - Support advanced search and filtering requirements
    ///
    /// ## Usage
    ///
    /// The `attribute` property appears in various contexts within FHIR,
    /// including ConceptMap for mapping properties, and in terminology
    /// resources for concept classification. It provides a flexible mechanism
    /// for adding structured metadata.
    ///
    /// ## Data Type
    ///
    /// **object** or **array** of **objects** - Structure varies by context,
    /// typically containing:
    ///
    /// - `code` (string) - Identifier for the attribute
    /// - `value` (various types) - Value of the attribute
    /// - Additional metadata fields as appropriate
    ///
    /// ## Constraints
    ///
    /// - Structure depends on the specific resource and context
    /// - Attribute codes should be unique within their scope
    /// - Values must conform to the expected data type
    /// - Should follow established patterns for the resource type
    ///
    /// ## Examples
    ///
    /// ### ConceptMap Element Attribute
    ///
    /// ```json
    /// {
    ///   "code": "equivalent",
    ///   "attribute": [
    ///     {
    ///       "code": "confidence",
    ///       "valueDecimal": 0.95
    ///     },
    ///     {
    ///       "code": "mapping-source",
    ///       "valueString": "Expert consensus"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Terminology Concept Attribute
    ///
    /// ```json
    /// {
    ///   "attribute": {
    ///     "classification": "high-priority",
    ///     "reviewStatus": "approved",
    ///     "lastModified": "2023-12-01"
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `code` - Identifier that the attribute applies to
    /// - `property` - Similar concept in different contexts
    /// - `extension` - Alternative mechanism for additional data
    /// - `meta` - Resource-level metadata
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5: [Context-specific attribute
    /// definitions](http://hl7.org/fhir/R5/) (varies by resource type)
    ///
    pub attribute: String,

    /// # valueCoding
    ///
    /// ## Description
    ///
    /// The `valueCoding` property contains a single coding from a specific code
    /// system. It represents a coded value with its associated system, code,
    /// display text, and optional version information, providing standardized
    /// terminology representation.
    ///
    /// ## Purpose
    ///
    /// - Reference specific codes from established terminologies
    /// - Provide system, code, and display information together
    /// - Support precise terminology binding and validation
    /// - Enable automated processing of coded data
    /// - Facilitate terminology server interactions
    ///
    /// ## Usage
    ///
    /// The `valueCoding` property is used in extensions, parameters, and data
    /// elements where a single, well-defined coding from a specific terminology
    /// system is required.
    ///
    /// ## Data Type
    ///
    /// **Coding** - A complex type containing:
    ///
    /// - `system` - Identity of the terminology system
    /// - `version` - Version of the terminology system
    /// - `code` - Symbol in syntax defined by the system
    /// - `display` - Representation defined by the system
    /// - `userSelected` - If this coding was chosen directly by the user
    ///
    /// ## Constraints
    ///
    /// - Must have a valid code from the specified system
    /// - System should be a valid URI identifying a code system
    /// - Display text should match the official display for the code
    /// - Version should correspond to the terminology system version
    /// - Code must be active in the specified system version
    ///
    /// ## Examples
    ///
    /// ### Extension with SNOMED CT Coding
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/primary-condition",
    ///       "valueCoding": {
    ///         "system": "http://snomed.info/sct",
    ///         "version": "http://snomed.info/sct/731000124108/version/20240301",
    ///         "code": "73211009",
    ///         "display": "Diabetes mellitus",
    ///         "userSelected": false
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with LOINC Coding
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "observationCode",
    ///       "valueCoding": {
    ///         "system": "http://loinc.org",
    ///         "code": "33747-0",
    ///         "display": "General appearance of patient"
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueCodeableConcept` - Complex concepts with multiple codings
    /// - `valueCode` - Simple code without system
    /// - `coding` - Direct coding arrays
    /// - `system` - Code system identifier
    /// - `code` - Code value
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [Coding](http://hl7.org/fhir/R5/datatypes.html#Coding)
    ///
    pub value_coding: Coding,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DependsOn;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("depends_on")
            .join("depends_on.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.attribute, "ex3");
    }
}
