//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// # fullUrl
    ///
    /// ## Description
    ///
    /// The `fullUrl` attribute provides an absolute URL that uniquely
    /// identifies a resource within a Bundle entry. This URL serves as a stable
    /// identifier that can be used for references within the bundle and
    /// provides a canonical location where the resource can be found. The
    /// fullUrl is particularly important for maintaining referential integrity
    /// within bundles and supporting resource resolution in distributed
    /// systems.
    ///
    /// ## Purpose
    ///
    /// The `fullUrl` exists to:
    ///
    /// - Provide unique identification for resources within bundle entries
    /// - Support referential integrity and link resolution within bundles
    /// - Enable absolute URL references for resource location and retrieval
    /// - Support bundle processing and resource cross-referencing
    /// - Facilitate resource caching and resolution mechanisms
    /// - Enable proper resource identification in messaging and transaction
    ///   bundles
    /// - Support canonical resource referencing across distributed systems
    ///
    /// ## Usage
    ///
    /// Use the `fullUrl` attribute when:
    ///
    /// - Creating Bundle entries that need unique resource identification
    /// - Supporting internal bundle references between resources
    /// - Implementing resource resolution and caching mechanisms
    /// - Creating transaction or batch bundles with resource interdependencies
    /// - Building messaging bundles with multiple related resources
    /// - Supporting search bundles where resources need canonical URLs
    /// - Implementing resource versioning and history tracking
    ///
    /// The fullUrl should be globally unique and ideally resolvable to the
    /// actual resource location.
    ///
    /// ## Data Type
    ///
    /// **uri** - An absolute Uniform Resource Identifier:
    ///
    /// - Must be an absolute URI (not relative)
    /// - Should be globally unique within the context
    /// - Commonly uses HTTP/HTTPS scheme for web-accessible resources
    /// - May use URN scheme for non-web identifiers
    /// - Should be stable and persistent when possible
    /// - Must be unique within the containing bundle
    /// - Should follow RFC 3986 URI syntax requirements
    ///
    /// ## Constraints
    ///
    /// - **Required**: Optional within bundle entries, but recommended for
    ///   referential integrity
    /// - **Cardinality**: 0..1 (at most one occurrence per entry)
    /// - **Format**: Must be a valid absolute URI
    /// - **Uniqueness**: Must be unique within the containing bundle
    /// - **Resolvability**: Should ideally resolve to the actual resource when
    ///   accessed
    /// - **Stability**: Should remain stable across bundle versions when
    ///   representing the same resource
    /// - **Scheme**: Typically HTTP/HTTPS, but other URI schemes are permitted
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Bundle resource
    /// demonstrating fullUrl usage in various contexts including search
    /// results, transactions, and messaging with proper cross-referencing.
    ///
    /// ## Related Keys
    ///
    /// - `entry` - The bundle entry container that includes the fullUrl
    /// - `resource` - The actual FHIR resource that the fullUrl identifies
    /// - `reference` - Resource references that may use the fullUrl for
    ///   resolution
    /// - `url` - Canonical URLs in resources that may match the fullUrl
    /// - `id` - Resource logical ID that may be derived from or related to the
    ///   fullUrl
    /// - `link` - Bundle-level links that provide navigation context for
    ///   entries
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for Bundle resource structure and fullUrl
    /// element requirements.
    ///
    pub full_url: String,

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
    pub resource: Resource,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Entry;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("entry")
            .join("entry.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.full_url,
            "http://hl7.org/fhir/ConceptMap/cm-administrative-gender-v2"
        );
    }
}
