//! # constraint
//!
//! ## Description
//!
//! The `constraint` attribute defines validation rules that must be satisfied
//! for a FHIR resource or element to be considered valid. It specifies business
//! rules, invariants, and additional validation logic beyond what is captured
//! by the basic element definitions, ensuring data integrity and consistency
//! across FHIR implementations. Each constraint contains a unique key, severity
//! level, human-readable description, and an executable expression that defines
//! the validation logic.
//!
//! ## Purpose
//!
//! The `constraint` exists to ensure data quality and business rule compliance
//! by:
//!
//! - Defining custom validation rules beyond basic data type constraints
//! - Enforcing business logic and clinical safety requirements
//! - Supporting regulatory compliance and quality assurance
//! - Enabling consistent data validation across different systems
//! - Providing clear error messages when validation fails
//! - Supporting complex inter-element validation scenarios
//! - Facilitating automated quality checks and data governance
//!
//! ## Usage
//!
//! Use the `constraint` attribute when:
//!
//! - Defining business rules that must be enforced during validation
//! - Creating custom validation logic for profiles or extensions
//! - Enforcing clinical safety requirements and best practices
//! - Implementing regulatory or organizational compliance rules
//! - Validating complex relationships between multiple elements
//! - Supporting quality assurance and data governance initiatives
//! - Providing meaningful validation feedback to users
//!
//! Each constraint includes a unique key for identification, severity level,
//! human-readable description, and executable expression.
//!
//! ## Data Type
//!
//! **BackboneElement** - A complex structure containing:
//!
//! - **key** (id) - Unique identifier for the constraint
//! - **requirements** (markdown) - Why the constraint is needed
//! - **severity** (code) - error, warning, or guideline
//! - **human** (string) - Human-readable description
//! - **expression** (string) - FHIRPath expression defining the rule
//! - **xpath** (string) - XPath equivalent (deprecated)
//! - **source** (canonical) - Source of the constraint definition
//!
//! ## Constraints
//!
//! - **Required**: No - Only needed when custom validation rules are required
//! - **Cardinality**: 0..* (zero to many constraints per element)
//! - **Unique Keys**: Each constraint key must be unique within the structure
//! - **Expression Format**: Must be valid FHIRPath or equivalent expression
//!   language
//! - **Severity Values**: Must be one of: error, warning, guideline
//! - **Inheritance**: Derived profiles inherit constraints from base
//!   definitions
//! - **Source Specification**: Source should reference the defining
//!   specification
//!
//! ## Examples
//!
//! See the accompanying `example.json` file for complete StructureDefinition
//! resources demonstrating various constraint patterns including clinical
//! safety rules, business logic validation, regulatory compliance checks, and
//! quality assurance constraints.
//!
//! ## Related Keys
//!
//! - `key` - Unique identifier within the constraint definition
//! - `severity` - Level of constraint enforcement (error, warning, guideline)
//! - `human` - Human-readable description of the constraint
//! - `expression` - Executable validation logic using FHIRPath
//! - `condition` - Conditions that must be met for elements to be present
//! - `min` - Minimum cardinality constraints on elements
//! - `max` - Maximum cardinality constraints on elements
//!
//! ## Specification Reference
//!
//! Based on FHIR R5 specification. For complete details, refer to the official
//! FHIR R5 documentation for ElementDefinition constraints and validation
//! principles.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Constraint {
    /// # key
    ///
    /// ## Description
    ///
    /// The `key` attribute provides a unique identifier for validation
    /// constraints, discriminator criteria, and other rule-based elements
    /// within FHIR StructureDefinitions. It serves as a stable reference point
    /// for identifying specific validation rules, enabling consistent error
    /// reporting, constraint inheritance, and rule management across different
    /// profiles and implementations.
    ///
    /// ## Purpose
    ///
    /// The `key` exists to provide unique identification and management by:
    ///
    /// - Uniquely identifying validation constraints within a structure
    /// - Enabling consistent error reporting and debugging
    /// - Supporting constraint inheritance in derived profiles
    /// - Facilitating rule management and maintenance
    /// - Providing stable references for documentation and tooling
    /// - Supporting automated validation and quality assurance processes
    /// - Enabling cross-reference between related validation rules
    ///
    /// ## Usage
    ///
    /// Use the `key` attribute when:
    ///
    /// - Defining unique identifiers for validation constraints
    /// - Creating discriminator rules for slicing operations
    /// - Establishing references to specific validation logic
    /// - Supporting error reporting and debugging workflows
    /// - Managing constraint inheritance in profile derivation
    /// - Implementing automated validation and testing
    /// - Documenting and maintaining validation rule sets
    ///
    /// Keys must be unique within their scope and should follow consistent
    /// naming conventions.
    ///
    /// ## Data Type
    ///
    /// **id** - A string following FHIR id data type rules:
    /// - Must be 1-64 characters long
    /// - May contain letters (a-z, A-Z), digits (0-9), hyphens (-), and periods
    ///   (.)
    /// - Must start with a letter or digit
    /// - Case-sensitive identifier
    /// - Should follow consistent naming patterns within an organization
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - When used within constraint, discriminator, or
    ///   similar structures
    /// - **Cardinality**: 1..1 (exactly one occurrence per constraint)
    /// - **Uniqueness**: Must be unique within the containing structure
    /// - **Format**: Must conform to FHIR id data type requirements
    /// - **Consistency**: Should follow organizational naming conventions
    /// - **Stability**: Should remain stable across versions when possible
    /// - **Scope**: Uniqueness required within the immediate container
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various key patterns
    /// including constraint identifiers, discriminator keys, and validation
    /// rule references with consistent naming conventions.
    ///
    /// ## Related Keys
    ///
    /// - `constraint` - Container element that includes the key for
    ///   identification
    /// - `severity` - Validation severity level associated with the key
    /// - `human` - Human-readable description linked to the key
    /// - `expression` - Validation logic identified by the key
    /// - `discriminator` - Slicing discriminator that may include key
    ///   references
    /// - `path` - Element path that may be referenced by key-based rules
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for id data type and constraint key
    /// requirements.
    ///
    pub key: String,

    /// # severity
    ///
    /// ## Description
    ///
    /// The `severity` attribute defines the level of enforcement for validation
    /// constraints, indicating whether a rule violation should be treated as a
    /// blocking error, a warning that should be addressed, or a guideline for
    /// best practices. It determines how validation tools and systems should
    /// respond when a constraint is not satisfied, enabling flexible quality
    /// assurance approaches that balance data integrity with practical
    /// implementation needs.
    ///
    /// ## Purpose
    ///
    /// The `severity` exists to enable graduated validation responses by:
    ///
    /// - Categorizing constraint violations by their clinical and technical
    ///   impact
    /// - Supporting flexible validation workflows for different use cases
    /// - Enabling quality improvement processes with appropriate escalation
    /// - Balancing data quality requirements with implementation practicality
    /// - Supporting risk-based validation approaches for patient safety
    /// - Facilitating automated quality assurance with appropriate responses
    /// - Providing clear guidance for implementers on compliance priorities
    ///
    /// ## Usage
    ///
    /// Use the `severity` attribute when:
    ///
    /// - Defining the enforcement level for validation constraints
    /// - Establishing quality assurance priorities and escalation paths
    /// - Supporting graduated validation responses in different contexts
    /// - Balancing strict validation with implementation flexibility
    /// - Creating risk-based validation frameworks for patient safety
    /// - Supporting quality improvement initiatives with appropriate feedback
    /// - Implementing automated validation with contextual responses
    ///
    /// The severity level should match the clinical and technical impact of the
    /// constraint.
    ///
    /// ## Data Type
    ///
    /// **code** - A coded value from a predefined set:
    ///
    /// - **error** - Constraint violation prevents resource acceptance or
    ///   processing
    /// - **warning** - Constraint violation should be flagged but doesn't block
    ///   processing
    /// - **guideline** - Constraint represents best practice recommendations
    ///
    /// The severity code must be one of these three standardized values.
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - When used within constraint definitions
    /// - **Cardinality**: 1..1 (exactly one occurrence per constraint)
    /// - **Valid Values**: Must be one of: error, warning, guideline
    /// - **Consistency**: Should align with clinical and technical risk levels
    /// - **Context Sensitivity**: May vary based on implementation requirements
    /// - **Escalation Logic**: Should support appropriate quality assurance
    ///   workflows
    /// - **Risk Alignment**: Should match the actual impact of constraint
    ///   violations
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various severity patterns
    /// including error-level safety constraints, warning-level quality
    /// indicators, and guideline-level best practices across different clinical
    /// contexts.
    ///
    /// ## Related Keys
    ///
    /// - `constraint` - Container element that includes the severity
    ///   specification
    /// - `key` - Unique identifier for the constraint with specified severity
    /// - `human` - Human-readable description that should align with severity
    ///   level
    /// - `expression` - Validation logic whose failure triggers the specified
    ///   severity
    /// - `requirements` - Justification that should explain the chosen severity
    ///   level
    /// - `source` - Origin specification that may define severity standards
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for constraint severity levels and
    /// validation processing requirements.
    ///
    pub severity: String,

    /// # human
    ///
    /// ## Description
    ///
    /// The `human` attribute provides a human-readable description of
    /// validation constraints, offering clear, understandable explanations of
    /// what the constraint checks and why it might fail. It serves as the
    /// primary communication mechanism between validation systems and users,
    /// translating technical validation logic into meaningful guidance that
    /// helps implementers understand and resolve validation issues effectively.
    ///
    /// ## Purpose
    ///
    /// The `human` exists to provide accessible validation feedback by:
    ///
    /// - Translating technical validation logic into understandable language
    /// - Supporting user-friendly error reporting and validation feedback
    /// - Enabling effective troubleshooting and constraint resolution
    /// - Providing context for validation failures to guide corrective actions
    /// - Supporting training and education on data quality requirements
    /// - Facilitating clear communication between technical and clinical users
    /// - Enhancing user experience in validation and quality assurance
    ///   workflows
    ///
    /// ## Usage
    ///
    /// Use the `human` attribute when:
    ///
    /// - Creating user-friendly validation error messages
    /// - Providing clear explanations of constraint requirements
    /// - Supporting troubleshooting and resolution of validation failures
    /// - Documenting validation logic for implementers and users
    /// - Enabling effective communication about data quality requirements
    /// - Supporting training and education on FHIR validation
    /// - Creating validation reports that are accessible to clinical users
    ///
    /// The human description should be clear, actionable, and focused on
    /// helping users understand and resolve issues.
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable text description:
    ///
    /// - Should be clear and concise while being complete
    /// - Must explain what the constraint checks in understandable terms
    /// - Should provide actionable guidance when possible
    /// - May include examples or specific requirements
    /// - Should avoid technical jargon when addressing clinical users
    /// - Must be meaningful to the intended audience
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - When used within constraint definitions
    /// - **Cardinality**: 1..1 (exactly one occurrence per constraint)
    /// - **Clarity**: Must be understandable by the intended audience
    /// - **Completeness**: Should fully explain the constraint requirement
    /// - **Actionability**: Should provide guidance on how to resolve
    ///   violations
    /// - **Consistency**: Should align with the severity level and expression
    ///   logic
    /// - **Localization**: May need translation for international
    ///   implementations
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various human-readable
    /// constraint descriptions including clinical safety explanations,
    /// technical requirement clarifications, and user-friendly guidance for
    /// different types of validation scenarios.
    ///
    /// ## Related Keys
    ///
    /// - `constraint` - Container element that includes the human description
    /// - `key` - Unique identifier for the constraint being described
    /// - `severity` - Enforcement level that should align with description tone
    /// - `expression` - Technical logic that the human description explains
    /// - `requirements` - Underlying rationale that may inform the description
    /// - `source` - Origin specification that may define description standards
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for constraint human descriptions and
    /// validation user experience guidelines.
    ///
    pub human: String,

    /// # source
    ///
    /// ## Description
    ///
    /// The `source` property identifies the source of data, mappings, or
    /// transformations in FHIR resources. It can reference systems, documents,
    /// or other sources that provided the information being processed or
    /// mapped.
    ///
    /// ## Purpose
    ///
    /// - Identify the origin of data or mappings
    /// - Support data lineage and provenance tracking
    /// - Enable source-specific processing rules
    /// - Facilitate data quality and validation
    /// - Support transformation and mapping operations
    ///
    /// ## Usage
    ///
    /// The `source` property is used in ConceptMap resources, data
    /// transformation contexts, and other scenarios where identifying the
    /// source of information is important for processing or validation.
    ///
    /// ## Data Type
    ///
    /// **uri** or **canonical** - Reference to the source system, document, or
    /// resource
    ///
    /// ## Constraints
    ///
    /// - Should be a valid URI or canonical reference
    /// - Must identify a retrievable or recognizable source
    /// - Should be consistent across related mappings or transformations
    /// - Should support the intended use case for source identification
    /// - May reference external systems or internal FHIR resources
    ///
    /// ## Examples
    ///
    /// ### ConceptMap Source
    ///
    /// ```json
    /// {
    ///   "resourceType": "ConceptMap",
    ///   "source": "http://hl7.org/fhir/ValueSet/administrative-gender",
    ///   "target": "http://example.org/fhir/ValueSet/local-gender-codes"
    /// }
    /// ```
    ///
    /// ### Mapping with Source Reference
    ///
    /// ```json
    /// {
    ///   "group": [
    ///     {
    ///       "source": "http://terminology.hl7.org/CodeSystem/v3-AdministrativeGender",
    ///       "target": "http://example.org/CodeSystem/gender"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `target` - Target resources or systems
    /// - `sourceUri` - Source URI references
    /// - `sourceScope` - Source scope definitions
    /// - `map` - Mapping definitions
    /// - `system` - Code system references
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 ConceptMap:
    /// [source](http://hl7.org/fhir/R5/conceptmap-definitions.html#ConceptMap.source_x_)
    ///
    pub source: Option<String>,

    /// # expression
    ///
    /// ## Description
    ///
    /// The `expression` attribute contains executable validation logic written
    /// in FHIRPath or other supported expression languages that defines the
    /// specific rule or condition that must be satisfied for a constraint to
    /// pass. It serves as the computational heart of validation constraints,
    /// translating business rules and clinical requirements into precise,
    /// testable logic that can be automatically evaluated against FHIR
    /// resources during validation processes.
    ///
    /// ## Purpose
    ///
    /// The `expression` exists to provide executable validation logic by:
    ///
    /// - Defining precise, testable validation rules using standard expression
    ///   languages
    /// - Enabling automated validation and quality assurance processes
    /// - Supporting complex inter-element validation and business rule
    ///   enforcement
    /// - Facilitating consistent validation logic across different FHIR
    ///   implementations
    /// - Providing computational precision for clinical safety and regulatory
    ///   requirements
    /// - Supporting dynamic validation based on resource context and
    ///   relationships
    /// - Enabling sophisticated quality checks beyond basic data type
    ///   validation
    ///
    /// ## Usage
    ///
    /// Use the `expression` attribute when:
    ///
    /// - Defining computational validation logic for constraints
    /// - Creating complex business rules that span multiple elements
    /// - Implementing clinical safety checks and quality assurance requirements
    /// - Supporting automated validation workflows and quality monitoring
    /// - Establishing precise criteria for data acceptance and processing
    /// - Creating context-sensitive validation rules based on resource
    ///   relationships
    /// - Implementing regulatory compliance and organizational policy
    ///   validation
    ///
    /// Expressions should be written in FHIRPath and be testable, precise, and
    /// maintainable.
    ///
    /// ## Data Type
    ///
    /// **string** - An executable expression in FHIRPath or equivalent
    /// language:
    ///
    /// - Must be valid FHIRPath syntax or other supported expression language
    /// - Should be deterministic and produce consistent boolean results
    /// - Must reference valid element paths and functions
    /// - Should handle edge cases and null values appropriately
    /// - Must be testable and verifiable during implementation
    /// - Should be maintainable and understandable by implementers
    ///
    /// ## Constraints
    ///
    /// - **Required**: Yes - When used within constraint definitions
    /// - **Cardinality**: 1..1 (exactly one occurrence per constraint)
    /// - **Syntax**: Must be valid FHIRPath or other supported expression
    ///   syntax
    /// - **Boolean Result**: Must evaluate to true (constraint satisfied) or
    ///   false (violation)
    /// - **Performance**: Should be efficient enough for production validation
    ///   scenarios
    /// - **Determinism**: Must produce consistent results for the same input
    ///   data
    /// - **Scope**: Must reference valid elements within the resource context
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for complete
    /// StructureDefinition resources demonstrating various expression patterns
    /// including FHIRPath validation logic, complex inter-element checks,
    /// clinical safety rules, and sophisticated business logic validation
    /// across different healthcare scenarios.
    ///
    /// ## Related Keys
    ///
    /// - `constraint` - Container element that includes the expression
    /// - `key` - Unique identifier for the constraint containing the expression
    /// - `severity` - Enforcement level that determines expression violation
    ///   handling
    /// - `human` - Human-readable explanation of what the expression validates
    /// - `requirements` - Business rationale that informs the expression logic
    /// - `xpath` - Legacy XPath equivalent (deprecated in favor of FHIRPath)
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for FHIRPath expressions, constraint
    /// expressions, and validation processing requirements.
    ///
    pub expression: Option<String>,

    /// # requirements
    ///
    /// ## Description
    ///
    /// The `requirements` property provides detailed explanations of why an
    /// element exists and how it should be used, including business and
    /// technical requirements.
    ///
    /// ## Purpose
    ///
    /// - Document business requirements for elements
    /// - Provide implementation guidance
    /// - Explain element purpose and context
    /// - Support requirement traceability
    /// - Facilitate proper element usage
    ///
    /// ## Usage
    ///
    /// The `requirements` property is used in StructureDefinition element
    /// definitions to document detailed requirements and usage guidance.
    ///
    /// ## Data Type
    ///
    /// **markdown** - Formatted text describing requirements
    ///
    /// ## Constraints
    ///
    /// - Should provide clear, actionable guidance
    /// - Must be relevant to the element
    /// - Should support implementer understanding
    /// - Can include business rules and context
    ///
    /// ## Examples
    ///
    /// ### Business Requirements
    ///
    /// ```json
    /// {
    ///   "element": [
    ///     {
    ///       "path": "Patient.identifier",
    ///       "requirements": "Patient identifiers are required for matching and deduplication. At least one identifier from an authoritative source must be provided."
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `element` - Element definitions
    /// - `comment` - Implementation comments
    /// - `definition` - Element definitions
    /// - `short` - Short descriptions
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 StructureDefinition:
    /// [requirements](http://hl7.org/fhir/R5/elementdefinition-definitions.html#ElementDefinition.requirements)
    ///
    pub requirements: Option<String>,

    /// # extension
    ///
    /// ## Description
    ///
    /// The `extension` attribute provides a mechanism for extending FHIR
    /// resources with additional data elements that are not part of the base
    /// resource definition. Extensions allow for local customizations and the
    /// addition of new data elements while maintaining interoperability in
    /// FHIR.
    ///
    /// ## Purpose
    ///
    /// Extensions exist to:
    ///
    /// - Add data elements not covered by the base FHIR specification
    /// - Support local, regional, or national requirements
    /// - Enable gradual evolution of FHIR without breaking existing
    ///   implementations
    /// - Maintain semantic interoperability through standardized extension
    ///   definitions
    /// - Allow for experimental or emerging data requirements
    /// - Support backwards compatibility when new elements are added to FHIR
    ///
    /// ## Usage
    ///
    /// Use extensions when you need to:
    ///
    /// - Include additional data not supported by standard FHIR elements
    /// - Implement local business requirements
    /// - Support regulatory or compliance requirements
    /// - Add experimental data elements before they become part of core FHIR
    /// - Extend resources with organization-specific information
    ///
    /// Extensions should always reference a StructureDefinition that defines
    /// their meaning and constraints.
    ///
    /// ## Data Type
    ///
    /// **Extension** - A complex data type containing:
    ///
    /// - `url` (required): canonical URI identifying the extension definition
    /// - `value[x]` (optional): the actual extension value using one of the
    ///   allowed FHIR data types
    /// - `extension` (optional): nested extensions for complex extension
    ///   structures
    ///
    /// Extensions can be simple (single value) or complex (containing nested
    /// extensions).
    ///
    /// ## Constraints
    ///
    /// - **Required**: No - Extensions are always optional
    /// - **Cardinality**: 0..* (zero to many occurrences)
    /// - **URL Required**: Every extension must have a `url` that references
    ///   its definition
    /// - **Value or Nested**: Extensions must have either a value or nested
    ///   extensions, not both
    /// - **Definition**: The URL must reference a valid StructureDefinition of
    ///   type Extension
    /// - **Placement**: Can appear on any element that allows extensions
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating various types of extensions including simple value
    /// extensions and complex nested extensions.
    ///
    /// ## Related Keys
    ///
    /// - `modifierExtension` - Extensions that modify the meaning of the
    ///   element
    /// - `url` - Required sub-element identifying the extension
    /// - `value[x]` - The extension's value using FHIR data types
    /// - Any FHIR element can contain extensions
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details on extension
    /// definitions, complex extensions, and extension registries, refer to the
    /// official FHIR R5 documentation on extensibility.
    ///
    pub extension: Option<Vec<Extension>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Constraint;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("constraint")
            .join("constraint.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.key, "sdf-4");
    }
}
