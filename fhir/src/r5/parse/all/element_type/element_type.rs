//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ElementType {
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

    /// # profile
    ///
    /// ## Description
    ///
    /// The `profile` property specifies which profiles a resource or element
    /// must conform to. It references StructureDefinitions that define
    /// additional constraints, extensions, and requirements beyond the base
    /// FHIR specification.
    ///
    /// ## Purpose
    ///
    /// - Specify conformance requirements for resources or elements
    /// - Reference implementation-specific profiles and constraints
    /// - Enable validation against custom business rules
    /// - Support interoperability through standardized profiles
    /// - Define specialized use cases and requirements
    ///
    /// ## Usage
    ///
    /// The `profile` property is used in Meta elements, CapabilityStatements,
    /// and other contexts where profile conformance needs to be declared. It
    /// contains canonical URLs referencing StructureDefinition resources.
    ///
    /// ## Data Type
    ///
    /// **canonical** - A canonical URL referencing a StructureDefinition **OR**
    /// **Array of canonical** - Multiple profile references
    ///
    /// ## Constraints
    ///
    /// - Must be a valid canonical URL referencing a StructureDefinition
    /// - Profile must be compatible with the resource type being profiled
    /// - Multiple profiles can be specified if they don't conflict
    /// - Profiles should be accessible and resolvable in the implementation
    ///   context
    ///
    /// ## Examples
    ///
    /// ### Profile in Resource Meta
    ///
    /// ```json
    /// {
    ///   "meta": {
    ///     "profile": [
    ///       "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ### Multiple Profiles
    ///
    /// ```json
    /// {
    ///   "meta": {
    ///     "profile": [
    ///       "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
    ///       "http://example.org/fhir/StructureDefinition/hospital-patient"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `meta` - Contains resource metadata including profiles
    /// - `url` - Canonical URL of the profile itself
    /// - `baseDefinition` - Base profile that is being specialized
    /// - `derivation` - How the profile relates to its base
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Meta:
    /// [Profile](http://hl7.org/fhir/R5/resource.html#Meta.profile)
    ///
    pub profile: Option<Vec<String>>,

    /// TODO
    pub target_profile: Option<Vec<String>>,

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
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ElementType;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("element_type")
            .join("element_type.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.code, "http://hl7.org/fhirpath/System.String");
    }
}
