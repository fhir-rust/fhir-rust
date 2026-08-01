//! # relatedArtifact
//!
//! ## Description
//!
//! The `relatedArtifact` property references external documents, publications, websites, or other artifacts that are related to or support the current resource. It provides citations, links, and metadata about related materials.
//!
//! ## Purpose
//!
//! - Reference supporting literature and evidence
//! - Link to related guidelines, protocols, or standards
//! - Provide citations for clinical evidence
//! - Connect to external documentation and resources
//! - Support evidence-based practice and research
//!
//! ## Usage
//!
//! The `relatedArtifact` property is used across many FHIR resources to reference external artifacts like publications, guidelines, or supporting documentation that relate to the resource content.
//!
//! ## Data Type
//!
//! **RelatedArtifact** - A complex data type containing:
//! - `type` - Type of relationship (documentation, citation, etc.)
//! - `label` - Short label for the artifact
//! - `display` - Brief description
//! - `citation` - Bibliographic citation
//! - `url` - Link to the artifact
//! - `document` - Attached document
//! - `resource` - Reference to a FHIR resource
//! - `resourceReference` - Reference to a related resource
//!
//! ## Constraints
//!
//! - Must specify the type of relationship
//! - Should provide sufficient information to locate the artifact
//! - Either citation, url, document, or resource should be provided
//! - Citations should follow standard bibliographic formats
//!
//! ## Examples
//!
//! ### Citation to Published Study
//! ```json
//! {
//!   "relatedArtifact": [
//!     {
//!       "type": "citation",
//!       "label": "Primary Evidence",
//!       "display": "Randomized controlled trial on medication effectiveness",
//!       "citation": "Smith J, et al. Efficacy of Treatment X in Hypertension: A Randomized Controlled Trial. New England Journal of Medicine. 2024;380(1):23-31.",
//!       "url": "https://doi.org/10.1056/NEJMoa2024001"
//!     }
//!   ]
//! }
//! ```
//!
//! ### Link to Clinical Guideline
//! ```json
//! {
//!   "relatedArtifact": [
//!     {
//!       "type": "documentation",
//!       "label": "Clinical Guideline",
//!       "display": "AHA/ACC Hypertension Guidelines 2024",
//!       "url": "https://www.ahajournals.org/hypertension-guidelines"
//!     }
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `library` - References to logic libraries
//! - `extension` - Additional resource extensions
//! - `contained` - Contained resources
//! - `text` - Human-readable narrative
//!
//! ## Specification Reference
//!
//! FHIR R5 Data Types: [RelatedArtifact](http://hl7.org/fhir/R5/metadatatypes.html#RelatedArtifact)

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RelatedArtifact {
    /// # type
    ///
    /// ## Description
    ///
    /// The `type` attribute is used throughout FHIR R5 to specify the category,
    /// classification, or specific kind of an element or resource. It provides
    /// essential context that determines how data should be interpreted,
    /// processed, or displayed, and often drives business logic and workflow
    /// decisions within healthcare systems.
    ///
    /// ## Purpose
    ///
    /// The `type` exists to provide categorization and context for FHIR
    /// elements, enabling:
    ///
    /// - Classification of data elements into meaningful categories
    /// - Support for polymorphic data structures and processing
    /// - Workflow and business logic decision-making
    /// - Appropriate rendering and user interface behavior
    /// - Filtering and querying based on type classifications
    ///
    /// ## Usage
    ///
    /// Use the `type` attribute when:
    ///
    /// - Classifying identifiers (MRN, SSN, driver's license, etc.)
    /// - Specifying communication methods (phone, email, fax)
    /// - Categorizing addresses (home, work, temporary)
    /// - Defining contact relationships (emergency contact, next of kin)
    /// - Classifying observations, procedures, or other clinical data
    /// - Specifying reference types in resource relationships
    ///
    /// The type often uses standardized code systems to ensure consistency and
    /// interoperability.
    ///
    /// ## Data Type
    ///
    /// **CodeableConcept** - Typically a coded value that may include:
    ///
    /// - `coding` - Array of coded representations from standard terminologies
    /// - `text` - Human-readable description of the type
    /// - Support for multiple coding systems for the same concept
    /// - Fallback to text when no appropriate code exists
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - depends on the specific context and use
    ///   case
    /// - **Cardinality**: Usually 0..1, sometimes 0..* for multiple type
    ///   classifications
    /// - **Binding Strength**: Often bound to specific value sets with varying
    ///   strength
    /// - **Consistency**: Should align with established terminology standards
    /// - **Context Dependency**: Meaning may vary based on the containing
    ///   element
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating various uses of the `type` attribute across different
    /// elements.
    ///
    /// ## Related Keys
    ///
    /// - `system` - URI identifying the code system used in type coding
    /// - `code` - Specific identifier within the type's code system
    /// - `display` - Human-readable representation of the type
    /// - `use` - Usage context that may complement type information
    /// - `category` - Higher-level classification that may contain type
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for specific element type bindings,
    /// CodeableConcept usage, and terminology binding requirements.
    ///
    pub r#type: String,

    /// # resource
    ///
    /// ## Description
    ///
    /// The `resource` property defines capabilities and constraints for
    /// specific FHIR resource types in capability statements and other
    /// conformance resources.
    ///
    /// ## Purpose
    ///
    /// - Define resource-specific capabilities
    /// - Specify supported operations per resource
    /// - Document resource constraints and profiles
    /// - Enable resource capability discovery
    /// - Support conformance testing
    ///
    /// ## Usage
    ///
    /// The `resource` property is used in CapabilityStatement and other
    /// conformance resources to define capabilities for specific resource
    /// types.
    ///
    /// ## Data Type
    ///
    /// **BackboneElement** - Complex structure defining resource capabilities
    ///
    /// ## Constraints
    ///
    /// - Must specify valid resource type
    /// - Should define realistic capabilities
    /// - Must align with server implementation
    /// - Should include relevant interactions
    ///
    /// ## Examples
    ///
    /// ### Patient Resource Capabilities
    ///
    /// ```json
    /// {
    ///   "resource": [
    ///     {
    ///       "type": "Patient",
    ///       "interaction": [
    ///         {"code": "read"},
    ///         {"code": "search-type"}
    ///       ]
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `type` - Resource types
    /// - `interaction` - Supported interactions
    /// - `profile` - Resource profiles
    /// - `rest` - REST capabilities
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 CapabilityStatement:
    /// [resource](http://hl7.org/fhir/R5/capabilitystatement-definitions.html#CapabilityStatement.rest.resource)
    ///
    pub resource: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelatedArtifact;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("related_artifact")
            .join("related_artifact.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.r#type, "derived-from");
    }
}
