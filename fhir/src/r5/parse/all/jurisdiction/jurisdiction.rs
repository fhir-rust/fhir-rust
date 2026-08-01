//! # jurisdiction
//!
//! ## Description
//!
//! The `jurisdiction` key is used in FHIR R5 conformance and terminology
//! resources to specify the legal or political jurisdictions for which the
//! resource is intended or applies. It helps identify the geographic or
//! organizational scope of applicability.
//!
//! ## Purpose
//!
//! - Specifies geographic or political scope of resource applicability
//! - Enables jurisdiction-specific filtering and discovery
//! - Supports regulatory and legal compliance requirements
//! - Facilitates international and multi-jurisdictional implementations
//! - Provides context for resource interpretation and usage
//!
//! ## Usage
//!
//! The `jurisdiction` appears in:
//!
//! - **StructureDefinition**: To specify where profiles apply
//! - **ValueSet/CodeSystem**: For terminology jurisdiction scope
//! - **CapabilityStatement**: To indicate server/client jurisdiction
//! - **Implementation guides**: For geographic applicability
//!
//! ## Data Type
//!
//! **CodeableConcept** - Array of coded jurisdictions containing:
//!
//! - `coding` - Coded jurisdiction (typically using ISO 3166 country codes)
//! - `text` - Human-readable jurisdiction description
//!
//! ## Constraints
//!
//! - Should use standardized jurisdiction codes when available
//! - ISO 3166 country codes are commonly used
//! - Can specify multiple jurisdictions for multi-national resources
//! - Should be consistent with the resource's intended use scope
//!
//! ## Examples
//!
//! ### Single Country Jurisdiction
//!
//! ```json
//! {
//!   "jurisdiction": [
//!     {
//!       "coding": [
//!         {
//!           "system": "urn:iso:std:iso:3166",
//!           "code": "US",
//!           "display": "United States of America"
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! ### Multiple Jurisdictions
//!
//! ```json
//! {
//!   "jurisdiction": [
//!     {
//!       "coding": [
//!         {
//!           "system": "urn:iso:std:iso:3166",
//!           "code": "US",
//!           "display": "United States of America"
//!         }
//!       ]
//!     },
//!     {
//!       "coding": [
//!         {
//!           "system": "urn:iso:std:iso:3166",
//!           "code": "CA",
//!           "display": "Canada"
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! ### Regional Jurisdiction
//!
//! ```json
//! {
//!   "jurisdiction": [
//!     {
//!       "coding": [
//!         {
//!           "system": "http://unstats.un.org/unsd/methods/m49/m49.htm",
//!           "code": "150",
//!           "display": "Europe"
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `useContext` - Context of use for the resource
//! - `publisher` - Organization publishing the resource
//! - `contact` - Contact information for the resource
//! - `copyright` - Copyright and legal notices
//! - `status` - Publication status of the resource
//! - `date` - Publication date
//!
//! ## Specification Reference
//!
//! - **FHIR R5 Specification**: Used across multiple conformance resources
//! - **ISO 3166 Codes**: [Country
//!   Codes](https://www.iso.org/iso-3166-country-codes.html)
//! - **UN M49 Codes**: [Geographic
//!   Regions](https://unstats.un.org/unsd/methodology/m49/)
//! - **Context**: Used in conformance and terminology resources for scope
//!   definition

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Jurisdiction {
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
    type T = Jurisdiction;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("jurisdiction")
            .join("jurisdiction.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.coding.first().unwrap().code,
            Some(String::from("my code"))
        );
    }
}
