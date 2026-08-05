//! # target
//!
//! ## Description
//!
//! The `target` property specifies the allowed resource types that can be
//! referenced by a reference-type element or search parameter. It constrains
//! which resource types are valid targets for references.
//!
//! ## Purpose
//!
//! - Define allowed target resource types for references
//! - Constrain reference relationships between resources
//! - Enable validation of reference integrity
//! - Support type-specific reference processing
//! - Guide client reference resolution behavior
//!
//! ## Usage
//!
//! The `target` property is used in SearchParameter definitions,
//! StructureDefinition elements, and other contexts where reference types need
//! to be constrained to specific resource types.
//!
//! ## Data Type
//!
//! **code** - Array of resource type codes
//!
//! ## Constraints
//!
//! - Must contain valid FHIR resource type names
//! - Should be consistent with the reference element's purpose
//! - Empty array means no specific target constraints
//! - Should be implementable by the server
//! - Must align with business rules and use cases
//!
//! ## Examples
//!
//! ### Search Parameter Target Types
//!
//! ```json
//! {
//!   "resourceType": "SearchParameter",
//!   "name": "patient",
//!   "type": "reference",
//!   "target": [
//!     "Patient"
//!   ]
//! }
//! ```
//!
//! ### Multiple Target Types
//!
//! ```json
//! {
//!   "name": "performer",
//!   "type": "reference",
//!   "target": [
//!     "Practitioner",
//!     "PractitionerRole",
//!     "Organization",
//!     "Patient",
//!     "RelatedPerson"
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `source` - Source resource types
//! - `reference` - Reference data types
//! - `type` - Element or parameter types
//! - `resourceType` - Resource type identifiers
//! - `targetProfile` - Specific profile targets
//!
//! ## Specification Reference
//!
//! FHIR R5 SearchParameter:
//! [target](http://hl7.org/fhir/R5/searchparameter-definitions.html#SearchParameter.target)

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Target {
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
    ///
    /// - Interoperability between different healthcare systems
    /// - Consistent representation of clinical concepts
    /// - Support for clinical decision support and analytics
    /// - Mapping between different terminology systems
    /// - Precise semantic meaning in healthcare data exchange
    ///
    /// ## Usage
    ///
    /// Use the `code` attribute when:
    ///
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
    ///
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

    /// # display
    ///
    /// ## Description
    ///
    /// The `display` attribute provides a human-readable representation of a
    /// coded concept as defined by the terminology system. It serves as the
    /// official textual description that corresponds to a specific code within
    /// its system context, helping users understand the meaning of coded values
    /// without requiring deep knowledge of the underlying terminology.
    ///
    /// ## Purpose
    ///
    /// The `display` exists to provide human-interpretable text for coded
    /// concepts, enabling:
    ///
    /// - User-friendly presentation of clinical data
    /// - Verification that the correct code was selected
    /// - Fallback text when terminology services are unavailable
    /// - Consistency with official terminology definitions
    /// - Support for user interfaces and clinical documentation
    ///
    /// ## Usage
    ///
    /// Use the `display` attribute when:
    ///
    /// - Creating Coding structures with human-readable labels
    /// - Providing official terminology descriptions for codes
    /// - Supporting user interfaces that show coded concepts
    /// - Ensuring consistency with terminology system definitions
    /// - Enabling verification of code selection accuracy
    ///
    /// The display text should match the official definition from the
    /// terminology system and should not be modified or localized arbitrarily.
    ///
    /// ## Data Type
    ///
    /// **string** - Human-readable text representing the code:
    /// - Should match the official display text from the terminology system
    /// - Case sensitivity follows the terminology system rules
    /// - Should be concise but descriptive
    /// - May include additional context as defined by the system
    /// - Should not contain markup or special formatting
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - Optional but strongly recommended for usability
    /// - **Cardinality**: 0..1 (zero to one occurrence within a Coding)
    /// - **Accuracy**: Should match the official terminology system definition
    /// - **Language**: Typically in the language specified by the terminology
    ///   system
    /// - **Consistency**: Should be stable for a given code/system combination
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Condition
    /// resource demonstrating the use of the `display` attribute across various
    /// coded fields.
    ///
    /// ## Related Keys
    ///
    /// - `code` - The machine-readable identifier that this display represents
    /// - `system` - The terminology system that defines both code and display
    /// - `text` - Free-text description that may differ from official display
    /// - `designation` - Alternative representations in different
    ///   languages/contexts
    /// - `coding` - Contains the display along with code and system
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Coding data types, terminology
    /// services, and display text requirements.
    ///
    pub display: String,

    /// # relationship
    ///
    /// ## Description
    ///
    /// The `relationship` property defines the type of relationship between
    /// entities, resources, or concepts. It specifies how one element relates
    /// to another, supporting various relationship semantics in FHIR resources.
    ///
    /// ## Purpose
    ///
    /// - Define semantic relationships between resources or entities
    /// - Specify the nature of connections and associations
    /// - Support relationship-based queries and navigation
    /// - Enable complex data modeling and linking
    /// - Facilitate understanding of entity connections
    ///
    /// ## Usage
    ///
    /// The `relationship` property is used in various FHIR resources to specify
    /// how entities relate to each other, such as patient relationships,
    /// organizational hierarchies, or clinical concept associations.
    ///
    /// ## Data Type
    ///
    /// One of:
    /// - **code** - A simple coded relationship value
    /// - **CodeableConcept** - A coded value representing the relationship type
    ///
    /// ## Constraints
    ///
    /// - Must be a valid relationship type from appropriate value sets
    /// - Should be semantically meaningful for the context
    /// - Relationship types should be standardized where possible
    /// - Bidirectional relationships may require inverse specification
    ///
    /// ## Examples
    ///
    /// ### Patient Relationship
    ///
    /// ```json
    /// {
    ///   "relationship": {
    ///     "coding": [
    ///       {
    ///         "system": "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
    ///         "code": "CHILD",
    ///         "display": "Child"
    ///       }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ### Organizational Relationship
    ///
    /// ```json
    /// {
    ///   "relationship": {
    ///     "coding": [
    ///       {
    ///         "system": "http://terminology.hl7.org/CodeSystem/organization-relationship",
    ///         "code": "part-of",
    ///         "display": "Part of"
    ///       }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `type` - Entity or resource type
    /// - `target` - Target of the relationship
    /// - `source` - Source of the relationship
    /// - `context` - Context in which relationship applies
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Various Resources: [Relationship
    /// Modeling](http://hl7.org/fhir/R5/references.html)
    ///
    pub relationship: String, //TODO this data type is one of String or CodeableConcept

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
    pub depends_on: Vec<DependsOn>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Target;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("target")
            .join("target.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.code, "my code");
    }
}
