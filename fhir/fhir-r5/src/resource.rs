//! Resource
//!
//! <https://build.fhir.org/json.html>
//!
//! JSON Representation of Resources
//!
//! ```text
//! {
//!   "resourceType" : "[Resource Type]",
//!   "resourceDefinition" : "(see below)",
//!   // from Source: property0
//!   "property1" : "<[primitive]>", // short description
//!   "property2" : { [Datatype] }, // short description
//!   "property3" : { // Short Description
//!     "propertyA" : { CodeableConcept }, // Short Description (Example)
//!   },
//!   "property4" : [{ // Short Description
//!     "propertyB" : { Reference(ResourceType) } // R!  Short Description
//!   }]
//! }
//! ```
//!

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// # resourceType
    ///
    /// ## Description
    ///
    /// The `resourceType` attribute specifies the type of FHIR resource being
    /// represented. It is a mandatory element that identifies which resource
    /// schema and constraints apply to the JSON document in FHIR R5.
    ///
    /// ## Purpose
    ///
    /// The `resourceType` serves several critical functions:
    ///
    /// - Identifies the specific FHIR resource type for parsers and processors
    /// - Determines which validation rules and constraints apply
    /// - Enables proper routing and processing in FHIR systems
    /// - Provides context for interpreting the resource's data elements
    /// - Supports polymorphism in FHIR resource handling
    ///
    /// ## Usage
    ///
    /// The `resourceType` must be included in every FHIR resource as the first
    /// element. It should be used:
    ///
    /// - At the root level of every FHIR resource JSON document
    /// - When validating resources against their appropriate
    ///   StructureDefinitions
    /// - In API endpoints to determine resource-specific processing logic
    /// - For content negotiation and resource type filtering
    ///
    /// ## Data Type
    ///
    /// **code** - A string that must exactly match one of the defined FHIR
    /// resource types. The value is:
    ///
    /// - Case-sensitive
    /// - Must be an exact match to a valid FHIR R5 resource type name
    /// - Follows PascalCase naming convention (e.g., "Patient", "Observation",
    ///   "DiagnosticReport")
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - Must be present in every FHIR resource
    /// - **Cardinality**: 1..1 (exactly one occurrence)
    /// - **Fixed Position**: Must be the first element in the JSON object
    /// - **Valid Values**: Must be one of the 150+ defined FHIR R5 resource
    ///   types
    /// - **Case Sensitivity**: Exact case match required
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating the use of the `resourceType` attribute.
    ///
    /// ## Related Keys
    ///
    /// - `meta.profile` - Specifies which profile(s) the resource conforms to
    /// - `id` - Unique identifier for the resource instance
    /// - `meta` - Metadata about the resource
    /// - All resource-specific elements depend on the `resourceType` for their
    ///   validity
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details and the full list
    /// of valid resource types, refer to the official FHIR R5 documentation for
    /// resource definitions.
    ///
    pub resource_type: String,

    pub resource_definition: String,
    pub properties: Vec<crate::r5::todo::property::Property>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            resource_type: String::default(),
            resource_definition: String::default(),
            properties: vec![],
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
                    "resourceType": "",
                    "resourceDefinition": "",
                    "properties": []
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
            let expect: ::serde_json::Value = json!(
                {
                    "resourceType": "",
                    "resourceDefinition": "",
                    "properties": []
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
