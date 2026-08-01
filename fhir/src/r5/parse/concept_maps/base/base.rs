//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Base {
    /// # base
    ///
    /// ## Description
    ///
    /// The `base` property specifies the base definition or parent element that
    /// this element definition is derived from or builds upon. It establishes
    /// inheritance relationships in StructureDefinition profiles and
    /// extensions.
    ///
    /// ## Purpose
    ///
    /// - Define inheritance relationships between element definitions
    /// - Enable profile derivation and constraint application
    /// - Support element definition reuse and specialization
    /// - Provide traceability to base FHIR specifications
    /// - Enable proper validation and constraint checking
    ///
    /// ## Usage
    ///
    /// The `base` property is used within ElementDefinition objects in
    /// StructureDefinition resources to indicate the base element from which
    /// the current element derives. This is essential for profile validation
    /// and understanding element constraints.
    ///
    /// ## Data Type
    ///
    /// **ElementDefinition.Base** object containing:
    ///
    /// - `path` (string) - Path of the base element
    /// - `min` (unsignedInt) - Minimum cardinality in base definition
    /// - `max` (string) - Maximum cardinality in base definition
    ///
    /// ## Constraints
    ///
    /// - Path must reference a valid base element definition
    /// - Min/max values reflect the base definition's cardinality
    /// - Must be consistent with the inheritance hierarchy
    /// - Required for all elements in differential definitions
    ///
    /// ## Examples
    ///
    /// ### Patient Profile Element Base
    ///
    /// ```json
    /// {
    ///   "path": "Patient.name",
    ///   "base": {
    ///     "path": "Patient.name",
    ///     "min": 0,
    ///     "max": "*"
    ///   },
    ///   "min": 1,
    ///   "mustSupport": true
    /// }
    /// ```
    ///
    /// ### Extension Element Base
    ///
    /// ```json
    /// {
    ///   "path": "Extension.valueString",
    ///   "base": {
    ///     "path": "Extension.value[x]",
    ///     "min": 0,
    ///     "max": "1"
    ///   },
    ///   "type": [
    ///     {
    ///       "code": "string"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Complex Type Element Base
    /// ```json
    /// {
    ///   "path": "Observation.component.valueQuantity.value",
    ///   "base": {
    ///     "path": "Quantity.value",
    ///     "min": 0,
    ///     "max": "1"
    ///   },
    ///   "min": 1
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `path` - The element path being defined
    /// - `min`/`max` - Cardinality constraints for this element
    /// - `type` - Data type specifications
    /// - `mustSupport` - Whether the element must be supported
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 ElementDefinition: [Base
    /// Definition](http://hl7.org/fhir/R5/elementdefinition.html#ElementDefinition.base)
    ///
    pub path: String,

    /// # min
    ///
    /// ## Description
    ///
    /// The `min` attribute specifies the minimum number of times an element
    /// must occur within a FHIR resource or complex data type. It establishes
    /// the lower bound for element cardinality, determining whether elements
    /// are optional (min=0) or required (min=1 or higher), and supporting
    /// validation rules that ensure essential data is captured according to
    /// clinical, regulatory, or business requirements.
    ///
    /// ## Purpose
    ///
    /// The `min` exists to establish minimum data requirements by:
    ///
    /// - Defining which elements are mandatory versus optional for resource
    ///   validity
    /// - Supporting clinical safety requirements that mandate critical
    ///   information
    /// - Enabling regulatory compliance by enforcing required data elements
    /// - Facilitating quality assurance through minimum data completeness
    ///   standards
    /// - Supporting business rules that require specific information for
    ///   processing
    /// - Enabling flexible profiles that can strengthen requirements from base
    ///   definitions
    /// - Providing clear guidance to implementers about essential data elements
    ///
    /// ## Usage
    ///
    /// Use the `min` attribute when:
    ///
    /// - Making optional elements mandatory in derived profiles
    /// - Establishing clinical safety requirements for critical information
    /// - Implementing regulatory compliance mandates for specific data elements
    /// - Supporting quality assurance initiatives requiring minimum data sets
    /// - Creating business rules that require certain information for
    ///   processing
    /// - Defining implementation-specific requirements that strengthen base
    ///   standards
    /// - Establishing data governance policies that mandate information capture
    ///
    /// The min value cannot exceed the max value and must be compatible with
    /// the base definition.
    ///
    /// ## Data Type
    ///
    /// **unsignedInt** - A non-negative integer:
    ///
    /// - Must be 0 or a positive integer (0, 1, 2, 3, ...)
    /// - Cannot exceed the corresponding max value
    /// - Cannot be less restrictive than the base definition's min value
    /// - Should reflect actual business, clinical, or regulatory requirements
    /// - Must be realistic and achievable in the intended implementation
    ///   context
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - Uses base definition min value if not specified
    /// - **Cardinality**: 0..1 (zero to one occurrence per element)
    /// - **Range**: Must be 0 or positive integer
    /// - **Compatibility**: Cannot be less than base definition's min value
    /// - **Consistency**: Must not exceed the max value for the same element
    /// - **Business Alignment**: Should reflect actual requirements, not
    ///   theoretical maximums
    /// - **Inheritance**: Derived profiles can increase but not decrease min
    ///   values
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various min cardinality
    /// patterns including mandatory clinical elements, required business
    /// information, regulatory compliance requirements, and quality assurance
    /// mandates across different healthcare contexts.
    ///
    /// ## Related Keys
    ///
    /// - `max` - Maximum cardinality that works with min to define element
    ///   bounds
    /// - `constraint` - Additional validation rules that may complement
    ///   cardinality
    /// - `condition` - Conditional requirements that may affect when min
    ///   applies
    /// - `mustSupport` - Implementation requirements that often align with min
    ///   > 0
    /// - `path` - Element path that the min cardinality applies to
    /// - `type` - Data type that may influence appropriate cardinality values
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ElementDefinition cardinality and
    /// validation requirements.
    ///
    pub min: u32,

    /// # max
    ///
    /// ## Description
    ///
    /// The `max` attribute specifies the maximum number of times an element can
    /// occur within a FHIR resource or complex data type. It establishes the
    /// upper bound for element cardinality, determining whether elements can
    /// appear once (max=1), multiple times (max=* or higher integers), or are
    /// prohibited (max=0), and supporting validation rules that prevent
    /// excessive data repetition and maintain resource integrity.
    ///
    /// ## Purpose
    ///
    /// The `max` exists to establish upper limits for data elements by:
    ///
    /// - Controlling resource size and complexity by limiting element
    ///   repetition
    /// - Preventing data bloat and maintaining processing efficiency
    /// - Supporting business rules that restrict certain information types
    /// - Enabling profile constraints that are more restrictive than base
    ///   definitions
    /// - Facilitating system design with predictable resource structures
    /// - Supporting validation rules that ensure data quality and consistency
    /// - Providing clear guidance to implementers about acceptable repetition
    ///   limits
    ///
    /// ## Usage
    ///
    /// Use the `max` attribute when:
    ///
    /// - Restricting repeating elements to specific counts in derived profiles
    /// - Prohibiting optional elements by setting max=0
    /// - Limiting collection sizes for performance or business reasons
    /// - Implementing organizational policies that restrict certain data types
    /// - Creating profiles with more constrained cardinality than base
    ///   definitions
    /// - Supporting system limitations that cannot handle unlimited repetition
    /// - Establishing data governance rules that control information
    ///   proliferation
    ///
    /// The max value must be greater than or equal to the min value and
    /// compatible with the base definition.
    ///
    /// ## Data Type
    ///
    /// **string** - Either a non-negative integer or the asterisk (*)
    /// character:
    ///
    /// - **Integer values**: 0, 1, 2, 3, ... (specific maximum count)
    /// - **Asterisk (*)**: Unlimited repetition allowed
    /// - **Zero (0)**: Element is prohibited (must also have min=0)
    /// - Must be greater than or equal to the corresponding min value
    /// - Cannot be more restrictive than the base definition allows
    /// - Should reflect actual business, clinical, or technical requirements
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - Uses base definition max value if not specified
    /// - **Cardinality**: 0..1 (zero to one occurrence per element)
    /// - **Format**: Must be "0", positive integer, or "*"
    /// - **Compatibility**: Cannot be more restrictive than base definition's
    ///   max value
    /// - **Consistency**: Must be greater than or equal to the min value for
    ///   the same element
    /// - **Business Alignment**: Should reflect actual requirements and system
    ///   capabilities
    /// - **Inheritance**: Derived profiles can restrict but not expand max
    ///   values
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various max cardinality
    /// patterns including single-occurrence restrictions, collection size
    /// limits, element prohibition, unlimited repetition, and business-driven
    /// cardinality constraints across different healthcare scenarios.
    ///
    /// ## Related Keys
    ///
    /// - `min` - Minimum cardinality that works with max to define element
    ///   bounds
    /// - `constraint` - Additional validation rules that may complement
    ///   cardinality limits
    /// - `condition` - Conditional requirements that may affect when max
    ///   applies
    /// - `mustSupport` - Implementation requirements that may influence
    ///   repetition needs
    /// - `path` - Element path that the max cardinality applies to
    /// - `type` - Data type that may influence appropriate cardinality limits
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for ElementDefinition cardinality and
    /// validation requirements.
    ///
    pub max: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Base;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("base")
            .join("base.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
