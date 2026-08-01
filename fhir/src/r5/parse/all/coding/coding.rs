//! # coding
//!
//! ## Description
//!
//! The `coding` attribute is an array that contains one or more
//! code/system/display triplets representing the same concept from different
//! terminology systems. It is a core component of the CodeableConcept data type
//! in FHIR R5, enabling multiple representations of the same clinical concept
//! using different coding systems for maximum interoperability.
//!
//! ## Purpose
//!
//! The `coding` exists to provide multiple coded representations of a single
//! concept, supporting:
//!
//! - Cross-terminology mapping and translation
//! - System interoperability across different coding standards
//! - Fallback options when primary codes are not understood
//! - Support for local and international terminology systems
//! - Semantic equivalence across different healthcare contexts
//!
//! ## Usage
//!
//! Use the `coding` attribute when:
//!
//! - Creating CodeableConcept structures with multiple code representations
//! - Mapping between different terminology systems (e.g., ICD-10 to SNOMED CT)
//! - Providing both local and standard codes for the same concept
//! - Supporting systems that may understand different coding systems
//! - Ensuring maximum interoperability in data exchange
//!
//! Each coding entry should represent the same concept but from different
//! terminology systems or versions.
//!
//! ## Data Type
//!
//! **array of Coding** - An array containing Coding objects, where each Coding
//! contains:
//!
//! - `system` (string): URI identifying the code system
//! - `code` (string): Symbol in syntax defined by the system
//! - `display` (string): Representation defined by the system
//! - `version` (string): Version of the system (optional)
//! - `userSelected` (boolean): Whether this coding was chosen by the user
//!
//! ## Constraints
//!
//! - **Required**: Conditional - Required in CodeableConcept when no text is
//!   provided
//! - **Cardinality**: 0..* (zero to many occurrences)
//! - **Uniqueness**: Each coding should represent equivalent concepts from
//!   different systems
//! - **System Requirements**: Each coding should have a system URI
//! - **Order**: Generally ordered by preference or specificity
//!
//! ## Examples
//!
//! See the accompanying `example.json` file for a complete MedicationRequest
//! resource demonstrating the use of the `coding` attribute with multiple
//! terminology systems.
//!
//! ## Related Keys
//!
//! - `code` - The specific identifier within each coding
//! - `system` - The terminology system URI for each coding
//! - `display` - Human-readable text for each coding
//! - `text` - Free text representation of the overall concept
//! - `userSelected` - Indicates user preference among multiple codings
//!
//! ## Specification Reference
//!
//! Based on FHIR R5 specification. For complete details, refer to the official
//! FHIR R5 documentation for CodeableConcept and Coding data types, and
//! terminology binding guidance.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Coding {
    /// # system
    ///
    /// ## Description
    ///
    /// The `system` attribute is a URI that identifies the code system or
    /// terminology from which a code is drawn. It serves as the namespace that
    /// gives meaning and context to coded values in FHIR, ensuring that codes
    /// can be properly interpreted and validated within their appropriate
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
    ///
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
    pub system: Option<String>,

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
    pub code: Option<String>,

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
    ///
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
    pub display: Option<String>,

    /// # version
    ///
    /// ## Description
    ///
    /// The `version` attribute represents the business version identifier of a
    /// FHIR resource, particularly for canonical resources like
    /// StructureDefinitions, ValueSets, CodeSystems, and CapabilityStatements.
    /// This version works in conjunction with the canonical `url` to provide
    /// precise, version-specific identification of resources. Unlike technical
    /// versioning (like `meta.versionId`), the business version reflects
    /// meaningful changes in the resource's content, semantics, or clinical
    /// significance.
    ///
    /// ## Purpose
    ///
    /// The `version` exists to support:
    ///
    /// - Business-level versioning that reflects meaningful content changes
    /// - Version-specific resource references and dependencies
    /// - Change management and compatibility tracking across resource evolution
    /// - Support for multiple concurrent versions of the same conceptual
    ///   resource
    /// - Implementation guidance for version compatibility and migration
    /// - Regulatory and compliance requirements for versioned healthcare
    ///   standards
    ///
    /// ## Usage
    ///
    /// Use the `version` attribute when:
    ///
    /// - Publishing canonical resources that may evolve over time
    /// - Supporting multiple concurrent versions of clinical standards
    /// - Implementing version-aware resource resolution and validation
    /// - Managing dependencies between versioned FHIR artifacts
    /// - Providing clear change tracking for clinical decision support rules
    /// - Supporting regulatory requirements for versioned healthcare content
    ///
    /// Version values should follow semantic versioning principles where
    /// appropriate, using formats like "1.0.0" or "2024.1" depending on
    /// organizational conventions.
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable version identifier:
    ///
    /// - Commonly follows semantic versioning (e.g., "1.0.0", "2.1.3")
    /// - May use date-based versioning (e.g., "2024.08", "20240815")
    /// - Can include pre-release indicators (e.g., "1.0.0-beta", "2.0.0-rc1")
    /// - Should be consistently formatted within an organization
    /// - Must be comparable to determine version precedence
    /// - Should reflect the significance of changes between versions
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional for most resources, strongly recommended for
    ///   canonical resources
    /// - **Cardinality**: 0..1 (at most one occurrence)
    /// - **Format**: No strict format requirements, but should be consistent
    ///   and comparable
    /// - **Uniqueness**: Should be unique within the context of the same
    ///   canonical URL
    /// - **Ordering**: Should allow for logical ordering and comparison of
    ///   versions
    /// - **Stability**: Should not change once a version is published and in
    ///   use
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete examples
    /// demonstrating version usage in StructureDefinition, ValueSet, and
    /// CapabilityStatement resources with different versioning approaches.
    ///
    /// ## Related Keys
    ///
    /// - `url` - Canonical identifier that works with version to provide
    ///   precise resource identification
    /// - `name` - Machine-readable identifier that may reflect version in its
    ///   naming
    /// - `title` - Human-readable title that may include version information
    ///   for clarity
    /// - `status` - Indicates lifecycle status which relates to version
    ///   maturity
    /// - `date` - Publication date that often corresponds to version release
    ///   date
    /// - `publisher` - Entity responsible for version management and release
    /// - `experimental` - Flag indicating if this version is still experimental
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for canonical resource types and
    /// versioning guidelines in the FHIR specification.
    ///
    pub version: Option<String>,

    /// TODO
    ///
    /// userSelected` (boolean): Whether this coding was chosen by the user
    ///
    pub user_selected: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("coding")
            .join("coding.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.code, Some(String::from("my code")));
    }
}
