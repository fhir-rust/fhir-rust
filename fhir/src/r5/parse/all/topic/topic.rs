//! # topic
//!
//! ## Description
//!
//! The `topic` property identifies the clinical or administrative topics
//! covered by a FHIR resource, enabling categorization and discovery.
//!
//! ## Purpose
//!
//! - Categorize resources by clinical topics
//! - Enable topic-based search and filtering
//! - Support knowledge organization
//! - Facilitate content discovery
//! - Enable topic-specific workflows
//!
//! ## Usage
//!
//! The `topic` property is used in knowledge resources like PlanDefinition,
//! ActivityDefinition, and others to identify covered topics.
//!
//! ## Data Type
//!
//! **CodeableConcept** - Coded topic classifications
//!
//! ## Constraints
//!
//! - Should use recognized topic vocabularies
//! - Must accurately represent resource content
//! - Should support discovery and categorization
//! - Can include multiple topics
//!
//! ## Examples
//!
//! ### Clinical Topic
//!
//! ```json
//! {
//!   "topic": [{
//!     "coding": [{
//!       "system": "http://snomed.info/sct",
//!       "code": "73211009",
//!       "display": "Diabetes mellitus"
//!     }]
//!   }]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `category` - General categories
//! - `type` - Resource types
//! - `subject` - Subject references
//! - `useContext` - Usage contexts
//!
//! ## Specification Reference
//!
//! FHIR R5 Metadata:
//! [topic](http://hl7.org/fhir/R5/metadatatypes.html#UsageContext)

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Topic {
    /// # coding
    ///
    /// ## Description
    ///
    /// The `coding` attribute is an array that contains one or more
    /// code/system/display triplets representing the same concept from
    /// different terminology systems. It is a core component of the
    /// CodeableConcept data type in FHIR R5, enabling multiple representations
    /// of the same clinical concept using different coding systems for maximum
    /// interoperability.
    ///
    /// ## Purpose
    ///
    /// The `coding` exists to provide multiple coded representations of a
    /// single concept, supporting:
    /// - Cross-terminology mapping and translation
    /// - System interoperability across different coding standards
    /// - Fallback options when primary codes are not understood
    /// - Support for local and international terminology systems
    /// - Semantic equivalence across different healthcare contexts
    ///
    /// ## Usage
    ///
    /// Use the `coding` attribute when:
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
    pub coding: Vec<Coding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Topic;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR.join("topic").join("topic.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.coding.first().unwrap().code,
            Some(String::from("my code"))
        );
    }
}
