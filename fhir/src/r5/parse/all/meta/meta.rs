//! # meta
//!
//! ## Description
//!
//! The `meta` attribute contains metadata about a FHIR resource that is
//! maintained by the infrastructure. It provides information about the
//! resource's versioning, last modification, security labels, profiles, and
//! tags in FHIR R5.
//!
//! ## Purpose
//!
//! The `meta` element serves to:
//!
//! - Track resource versioning and modification history
//! - Specify which profiles the resource claims to conform to
//! - Apply security labels and access control information
//! - Provide tags for categorization and workflow management
//! - Enable optimistic locking through version control
//! - Support provenance and audit requirements
//!
//! ## Usage
//!
//! Use the `meta` attribute to:
//!
//! - Track when resources were last updated
//! - Specify profile conformance for validation
//! - Apply security classifications to resources
//! - Tag resources for workflow or categorization purposes
//! - Enable version-aware updates and conflict detection
//! - Support system-level metadata requirements
//!
//! The `meta` element is typically managed by the server infrastructure, though
//! clients may provide some elements.
//!
//! ## Data Type
//!
//! **Meta** - A complex data type containing the following optional
//! sub-elements:
//!
//! - `versionId`: string - Version identifier for the resource
//! - `lastUpdated`: instant - When the resource was last updated  
//! - `source`: uri - Identifies where the resource came from
//! - `profile`: array of canonical URIs - Profiles this resource claims to
//!   conform to
//! - `security`: array of Coding - Security labels applied to the resource
//! - `tag`: array of Coding - Tags applied to the resource for categorization
//!
//! ## Constraints
//!
//! - **Required**: No - The entire `meta` element is optional
//! - **Cardinality**: 0..1 (zero to one occurrence)
//! - **Server Managed**: Most sub-elements are controlled by the server
//! - **versionId**: Must change when resource content changes
//! - **lastUpdated**: Must be updated when resource content changes
//! - **profile**: Must reference valid StructureDefinition resources
//!
//! ## Examples
//!
//! See the accompanying `example.json` file for a complete Practitioner
//! resource demonstrating comprehensive use of the `meta` attribute.
//!
//! ## Related Keys
//!
//! - `id` - Resource identifier that the meta information describes
//! - `resourceType` - Resource type that determines applicable profiles
//! - `extension` - May contain additional metadata not covered by meta
//! - Bundle entries use `meta` for version control during transactions
//!
//! ## Specification Reference
//!
//! Based on FHIR R5 specification. For complete details on metadata management,
//! versioning, and security labeling, refer to the official FHIR R5
//! documentation.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Meta {
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

    /// Example: "lastUpdated" : "2023-03-26T15:21:02.749+11:00"
    pub last_updated: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Meta;

    #[test]
    fn serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR.join("meta").join("meta.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).expect("from_reader");
        assert_eq!(
            actual.last_updated,
            Some(String::from("2023-03-26T15:21:02.749+11:00"))
        );
    }
}
