//! Resource
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// # id
    /// 
    /// ## Description
    /// 
    /// The `id` attribute is the logical identifier for a FHIR resource within
    /// a given context. It uniquely identifies the resource and is used for
    /// resource addressing and referencing within FHIR R5.
    /// 
    /// ## Purpose
    /// 
    /// The `id` exists to provide a unique identifier for each FHIR resource
    /// instance. This identifier is essential for:
    /// 
    /// - Resource addressing via RESTful URLs
    /// - Creating references between resources
    /// - Version control and resource tracking
    /// - Enabling resource updates and deletions
    /// 
    /// ## Usage
    /// 
    /// Use the `id` attribute when:
    /// 
    /// - Creating a new resource that needs to be uniquely identifiable
    /// - Referencing a resource from another resource
    /// - Performing CRUD operations on existing resources
    /// - Building RESTful FHIR APIs
    /// 
    /// The `id` is typically assigned by the server when a resource is created,
    /// but can be provided by the client in some scenarios.
    /// 
    /// ## Data Type
    /// 
    /// **string** - A sequence of Unicode characters with the following
    /// constraints:
    /// 
    /// - Must be between 1 and 64 characters in length
    /// - Can contain letters (A-Z, a-z), digits (0-9), hyphens (-), and periods
    ///   (.)
    /// - Must start and end with an alphanumeric character
    /// - Case sensitive
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: No - The `id` is optional for resource creation but
    ///   typically assigned by servers
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Length**: 1-64 characters
    /// - **Pattern**: Must match the regex `[A-Za-z0-9\-\.]{1,64}`
    /// - **Uniqueness**: Must be unique within the context of the resource type
    ///   on a given server
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating the use of the `id` attribute.
    /// 
    /// ## Related Keys
    /// 
    /// - `meta.versionId` - Version identifier for the resource instance
    /// - `identifier` - Business identifiers for the resource
    /// - `fullUrl` - Absolute URL when used in bundles
    /// - `reference` - Used to reference this resource from other resources
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for resource identity and addressing.
    /// 
    pub id: Option<types::Id>,

    /// # meta
    /// 
    /// ## Description
    /// 
    /// The `meta` attribute contains metadata about a FHIR resource that is
    /// maintained by the infrastructure. It provides information about the
    /// resource's versioning, last modification, security labels, profiles, and
    /// tags in FHIR R5.
    /// 
    /// ## Purpose
    /// 
    /// The `meta` element serves to:
    /// - Track resource versioning and modification history
    /// - Specify which profiles the resource claims to conform to
    /// - Apply security labels and access control information
    /// - Provide tags for categorization and workflow management
    /// - Enable optimistic locking through version control
    /// - Support provenance and audit requirements
    /// 
    /// ## Usage
    /// 
    /// Use the `meta` attribute to:
    /// - Track when resources were last updated
    /// - Specify profile conformance for validation
    /// - Apply security classifications to resources
    /// - Tag resources for workflow or categorization purposes
    /// - Enable version-aware updates and conflict detection
    /// - Support system-level metadata requirements
    /// 
    /// The `meta` element is typically managed by the server infrastructure,
    /// though clients may provide some elements.
    /// 
    /// ## Data Type
    /// 
    /// **Meta** - A complex data type containing the following optional
    /// sub-elements:
    /// - `versionId`: string - Version identifier for the resource
    /// - `lastUpdated`: instant - When the resource was last updated  
    /// - `source`: uri - Identifies where the resource came from
    /// - `profile`: array of canonical URIs - Profiles this resource claims to
    ///   conform to
    /// - `security`: array of Coding - Security labels applied to the resource
    /// - `tag`: array of Coding - Tags applied to the resource for
    ///   categorization
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: No - The entire `meta` element is optional
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Server Managed**: Most sub-elements are controlled by the server
    /// - **versionId**: Must change when resource content changes
    /// - **lastUpdated**: Must be updated when resource content changes
    /// - **profile**: Must reference valid StructureDefinition resources
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for a complete Practitioner
    /// resource demonstrating comprehensive use of the `meta` attribute.
    /// 
    /// ## Related Keys
    /// 
    /// - `id` - Resource identifier that the meta information describes
    /// - `resourceType` - Resource type that determines applicable profiles
    /// - `extension` - May contain additional metadata not covered by meta
    /// - Bundle entries use `meta` for version control during transactions
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details on metadata
    /// management, versioning, and security labeling, refer to the official
    /// FHIR R5 documentation.    
    ///     
    pub meta: Option<types::Meta>,

    pub implicit_rules: Option<types::Uri>,

    pub language: Option<types::Code>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            id: None,
            meta: Meta::default(),
            implicit_rules: None,
            language: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "id": null,
                    "meta": null,
                    "implicitRules": null,
                    "language": null
                }
            );
            let actual: Resource = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
