//! # fixedCodeableConcept
//!
//! ## Description
//!
//! The `fixedCodeableConcept` key is used in FHIR R5 StructureDefinition and
//! ElementDefinition resources to specify a fixed value of type
//! `CodeableConcept` that an element must have. When this constraint is
//! applied, the element cannot have any other value and must be exactly the
//! specified CodeableConcept structure.
//!
//! ## Purpose
//!
//! - Constrains element values to a specific CodeableConcept in profiles
//! - Enforces invariant coded values with multiple coding representations
//! - Ensures consistency and conformance for complex coded elements
//! - Used in profile derivation to restrict allowable coded concepts
//! - Provides semantic consistency across implementations
//!
//! ## Usage
//!
//! The `fixedCodeableConcept` appears in:
//!
//! - **StructureDefinition**: Within `differential.element` or
//!   `snapshot.element` arrays
//! - **ElementDefinition**: As a direct property to constrain element values
//! - **Profiles**: To specify mandatory CodeableConcept values with specific
//!   coding and text
//!
//! ## Data Type
//!
//! **CodeableConcept** - A complex FHIR data type containing:
//! - `coding` array with system, code, display values
//! - Optional `text` for human-readable representation
//!
//! ## Constraints
//!
//! - Must be a valid FHIR CodeableConcept structure
//! - All coding entries must be valid (system, code pairs)
//! - When present, the element cannot have any other value
//! - Mutually exclusive with other fixed* or pattern* constraints on the same
//!   element
//! - Must include at least one coding or text value
//!
//! ## Examples
//!
//! ### Basic Structure Definition Usage
//!
//! ```json
//! {
//!   "fixedCodeableConcept": {
//!     "coding": [
//!       {
//!         "system": "http://snomed.info/sct",
//!         "code": "408542003",
//!         "display": "Clinical observation"
//!       }
//!     ]
//!   }
//! }
//! ```
//!
//! ### Complex Multi-Coding Example
//!
//! ```json
//! {
//!   "fixedCodeableConcept": {
//!     "coding": [
//!       {
//!         "system": "http://snomed.info/sct",
//!         "code": "386053000",
//!         "display": "Evaluation procedure"
//!       },
//!       {
//!         "system": "http://loinc.org",
//!         "code": "33747-0",
//!         "display": "General assessment"
//!       }
//!     ],
//!     "text": "Clinical evaluation"
//!   }
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `patternCodeableConcept` - Allows additional properties beyond the
//!   specified pattern
//! - `fixedCode` - Fixed value for simple code type elements
//! - `fixedCoding` - Fixed value for single Coding type elements
//! - `binding` - Defines allowed value sets for coded elements
//! - `code` - Simple code values in various FHIR contexts
//! - `coding` - Individual coding entries within CodeableConcept
//!
//! ## Specification Reference
//!
//! - **FHIR R5 Specification**: [ElementDefinition - Fixed
//!   Values](http://hl7.org/fhir/R5/elementdefinition.html)
//! - **CodeableConcept Type**: [FHIR R5
//!   CodeableConcept](http://hl7.org/fhir/R5/datatypes.html#CodeableConcept)
//! - **Section**: ElementDefinition.fixed[x]
//! - **Context**: Used in profile definitions and structure definitions

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CodeableConcept {
    /// # coding
    ///
    /// ## Description
    ///
    /// The `coding` attribute is an array that contains one or more
    /// code/system/display triplets representing the same concept from
    /// different terminology systems. It is a core component of the
    /// CodeableConcept data type in FHIR, enabling multiple representations
    /// of the same clinical concept using different coding systems for maximum
    /// interoperability.
    ///
    /// ## Purpose
    ///
    /// The `coding` exists to provide multiple coded representations of a
    /// single concept, supporting:
    ///
    /// - Cross-terminology mapping and translation
    /// - System interoperability across different coding standards
    /// - Fallback options when primary codes are not understood
    /// - Support for local and international terminology systems
    /// - Semantic equivalence across different healthcare contexts
    ///
    /// ## Usage
    ///
    /// Use the `coding` attribute when:
    ///
    /// - Creating CodeableConcept structures with multiple code representations
    /// - Mapping between different terminology systems (e.g., ICD-10 to SNOMED
    ///   CT)
    /// - Providing both local and standard codes for the same concept
    /// - Supporting systems that may understand different coding systems
    /// - Ensuring maximum interoperability in data exchange
    ///
    /// Each coding entry should represent the same concept but from different
    /// terminology systems or versions.
    ///
    /// ## Data Type
    ///
    /// **array of Coding** - An array containing Coding objects, where each
    /// Coding contains:
    ///
    /// - `system` (string): URI identifying the code system
    /// - `code` (string): Symbol in syntax defined by the system
    /// - `display` (string): Representation defined by the system
    /// - `version` (string): Version of the system (optional)
    /// - `userSelected` (boolean): Whether this coding was chosen by the user
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - Required in CodeableConcept when no text
    ///   is provided
    /// - **Cardinality**: 0..* (zero to many occurrences)
    /// - **Uniqueness**: Each coding should represent equivalent concepts from
    ///   different systems
    /// - **System Requirements**: Each coding should have a system URI
    /// - **Order**: Generally ordered by preference or specificity
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete
    /// MedicationRequest resource demonstrating the use of the `coding`
    /// attribute with multiple terminology systems.
    ///
    /// ## Related Keys
    ///
    /// - `code` - The specific identifier within each coding
    /// - `system` - The terminology system URI for each coding
    /// - `display` - Human-readable text for each coding
    /// - `text` - Free text representation of the overall concept
    /// - `userSelected` - Indicates user preference among multiple codings
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for CodeableConcept and Coding data
    /// types, and terminology binding guidance.
    ///
    pub coding: Option<Vec<Coding>>,

    /// # text
    ///
    /// ## Description
    ///
    /// The `text` attribute provides a human-readable narrative summary of a
    /// FHIR resource's content in XHTML format. This narrative serves as a
    /// fallback representation that ensures the essential information remains
    /// accessible even when systems cannot process all the structured data
    /// elements. The text element is particularly important for clinical
    /// safety, regulatory compliance, and systems interoperability where human
    /// readability is required.
    ///
    /// ## Purpose
    ///
    /// The `text` exists to:
    ///
    /// - Provide human-readable summaries of structured resource content
    /// - Ensure clinical information remains accessible when structured data
    ///   cannot be processed
    /// - Support regulatory requirements for human-readable clinical documents
    /// - Enable fallback display when rendering systems have limited
    ///   capabilities
    /// - Provide narrative context that complements structured data
    /// - Support clinical safety by ensuring critical information is always
    ///   readable
    /// - Enable content review and validation by healthcare professionals
    ///
    /// ## Usage
    ///
    /// Use the `text` attribute when:
    ///
    /// - Creating clinical resources that require human-readable summaries
    /// - Supporting regulatory compliance for clinical documentation
    /// - Ensuring accessibility across diverse healthcare systems
    /// - Providing narrative context for complex structured data
    /// - Creating resources for patient-facing applications
    /// - Supporting clinical review workflows that need readable content
    /// - Implementing systems that require both structured and narrative
    ///   representations
    ///
    /// The narrative should accurately summarize the key information from the
    /// structured elements.
    ///
    /// ## Data Type
    ///
    /// **Narrative** - A complex structure containing:
    ///
    /// - `status` (code): The generation status of the narrative
    ///   (generated|extensions|additional|empty)
    /// - `div` (xhtml): The XHTML content of the narrative
    ///
    /// **Status Values:**
    ///
    /// - `generated`: Generated from structured data, no additional information
    /// - `extensions`: Generated from structured data with additional extension
    ///   content
    /// - `additional`: Contains additional information not in structured data
    /// - `empty`: No narrative content provided
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional but strongly recommended for most clinical
    ///   resources
    /// - **Cardinality**: 0..1 (at most one narrative per resource)
    /// - **XHTML Format**: The div element must contain valid XHTML content
    /// - **Safety**: Should include all critical information from structured
    ///   data
    /// - **Consistency**: Should accurately reflect the structured data content
    /// - **Language**: Should match the language specified in the resource
    /// - **Security**: XHTML content must be safe and not contain executable
    ///   scripts
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete resources
    /// demonstrating text narratives for different resource types including
    /// clinical observations, medications, and patient information.
    ///
    /// ## Related Keys
    ///
    /// - `div` - The XHTML content portion of the narrative
    /// - `status` - Indicates how the narrative was generated and its
    ///   relationship to structured data
    /// - `language` - Language code that may affect narrative content
    /// - `meta` - Resource metadata that may influence narrative generation
    /// - `contained` - Inline resources that may be referenced in the narrative
    /// - `extension` - Extensions that may be included in "extensions" status
    ///   narratives
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Narrative data type and narrative
    /// generation requirements.
    ///
    pub text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CodeableConcept;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("codeable_concept")
            .join("codeable_concept.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.coding.unwrap().first().unwrap().code,
            Some(String::from("AUT"))
        );
    }
}
