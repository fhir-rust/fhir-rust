//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Example {
    /// # label
    ///
    /// ## Description
    ///
    /// The `label` key is used in FHIR ElementDefinition within
    /// StructureDefinition resources to provide a concise, human-readable label
    /// for an element. It serves as a brief identifier that can be used in user
    /// interfaces, forms, and documentation.
    ///
    /// ## Purpose
    ///
    /// - Provides concise, human-readable labels for elements
    /// - Supports user interface development and form generation
    /// - Enables better user experience in clinical applications
    /// - Offers standardized labeling for consistent presentation
    /// - Facilitates internationalization and localization efforts
    ///
    /// ## Usage
    ///
    /// The `label` appears in:
    ///
    /// - **StructureDefinition**: Within ElementDefinition for UI labeling
    /// - **Profile definitions**: To specify display labels for elements
    /// - **Form generation**: For automated form creation from profiles
    /// - **User interfaces**: For consistent element labeling
    /// - **Documentation**: For element identification in specifications
    ///
    /// ## Data Type
    ///
    /// **string** - A human-readable label
    ///
    /// - Should be concise and clear
    /// - Typically shorter than the `short` description
    /// - Should be appropriate for UI display
    /// - May be used in form labels and field names
    ///
    /// ## Constraints
    ///
    /// - Should be brief and suitable for UI display
    /// - Should be meaningful to end users
    /// - May be used in contexts with space limitations
    /// - Should align with the element's purpose and meaning
    /// - Can be localized for different languages
    ///
    /// ## Examples
    ///
    /// ### Basic Element Label
    ///
    /// ```json
    ///
    /// {
    ///   "label": "Patient ID"
    /// }
    /// ```
    ///
    /// ### Form Field Label
    ///
    /// ```json
    /// {
    ///   "label": "Birth Date"
    /// }
    /// ```
    ///
    /// ### Clinical Data Label
    ///
    /// ```json
    /// {
    ///   "label": "Blood Pressure"
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `short` - Brief description of the element (usually longer than label)
    /// - `definition` - Complete definition of the element
    /// - `comment` - Additional implementation guidance
    /// - `requirements` - Why the element is needed
    /// - `alias` - Alternative names for the element
    /// - `path` - Element path being labeled
    ///
    /// ## Specification Reference
    ///
    /// - **FHIR R5 Specification**:
    ///   [ElementDefinition](http://hl7.org/fhir/R5/elementdefinition.html)
    /// - **Section**: ElementDefinition.label
    /// - **Context**: Used in structure definitions for UI and documentation
    ///   labeling
    /// - **Best Practices**: Should be concise and user-friendly for interface
    ///   display
    ///
    pub label: String,

    /// # value
    ///
    /// ## Description
    ///
    /// The `value` attribute serves as a generic field prefix used throughout
    /// FHIR to represent the actual data content of various data types and
    /// elements. It appears in numerous contexts as part of polymorphic value
    /// fields (like value[x] in extensions and observations) and as the core
    /// data element in primitive data types and complex structures.
    ///
    /// ## Purpose
    ///
    /// The `value` exists to provide a consistent naming pattern for data
    /// content across FHIR, enabling:
    ///
    /// - Polymorphic data representation through value[x] patterns
    /// - Consistent structure for primitive and complex data types
    /// - Clear separation between metadata and actual data content
    /// - Support for type-specific value representations
    /// - Uniform handling of data values in processing systems
    ///
    /// ## Usage
    ///
    /// Use the `value` attribute when:
    ///
    /// - Creating Extension elements with typed values (valueString,
    ///   valueInteger, etc.)
    /// - Representing measurement values in Quantity data types
    /// - Storing actual data content in Observation.value[x] fields
    /// - Implementing primitive data types with additional metadata
    /// - Creating parameters with typed value content
    ///
    /// The value field is often suffixed with a type indicator (e.g.,
    /// valueString, valueCodeableConcept) to specify the data type.
    ///
    /// ## Data Type
    ///
    /// **varies** - The data type depends on the specific context and type
    /// suffix:
    /// - **Primitive types**: string, integer, decimal, boolean, date,
    ///   dateTime, etc.
    /// - **Complex types**: CodeableConcept, Quantity, Reference, Period, etc.
    /// - **Special types**: base64Binary, code, uri, url, canonical
    /// - Type safety ensured through polymorphic naming (value[x])
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - depends on the specific context and
    ///   requirements
    /// - **Cardinality**: Typically 0..1 or 1..1 depending on usage context
    /// - **Type Constraints**: Must conform to the specified data type
    ///   requirements
    /// - **Validation**: Subject to type-specific validation rules
    /// - **Polymorphism**: Only one value[x] variant allowed per element
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a complete Observation
    /// resource demonstrating various uses of the `value` attribute in
    /// different typed contexts.
    ///
    /// ## Related Keys
    ///
    /// - `unit` - Unit of measure when value represents a quantity
    /// - `system` - System reference for coded values
    /// - `code` - Coded representation when value is part of coded concepts
    /// - `extension` - Contains value[x] fields for additional data
    /// - `component` - May contain value[x] fields in complex observations
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for data types, polymorphic elements, and
    /// the value[x] pattern implementation.
    ///
    pub value: Option<serde_json::Value>,

    /// # valueBoolean
    ///
    /// ## Description
    ///
    /// The `valueBoolean` property contains boolean (true/false) values in FHIR
    /// resources. It represents binary choices, yes/no answers, or toggle
    /// states.
    ///
    /// ## Purpose
    ///
    /// - Represent binary true/false values
    /// - Support yes/no questions and responses
    /// - Enable boolean flags and switches
    /// - Provide simple binary data representation
    /// - Support boolean parameters and extensions
    ///
    /// ## Usage
    ///
    /// The `valueBoolean` property is used throughout FHIR resources for
    /// boolean values, particularly in extensions, parameters, and elements
    /// requiring true/false answers.
    ///
    /// ## Data Type
    ///
    /// **boolean** - Either true or false
    ///
    /// ## Constraints
    ///
    /// - Must be exactly true or false (not null or undefined)
    /// - Cannot be empty or missing when specified
    /// - JSON boolean representation (lowercase true/false)
    ///
    /// ## Examples
    ///
    /// ### Extension with Boolean Value
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/is-emergency",
    ///       "valueBoolean": true
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter Boolean Value
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "includeDefinition",
    ///       "valueBoolean": false
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueInteger` - Integer values
    /// - `valueString` - String values
    /// - `valueCode` - Coded values
    /// - `active` - Boolean status flags
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [boolean](http://hl7.org/fhir/R5/datatypes.html#boolean)
    ///
    pub value_boolean: Option<bool>,

    /// # valueCanonical
    ///
    /// ## Description
    ///
    /// The `valueCanonical` property contains a canonical URL reference to a
    /// FHIR resource. It represents a URI that uniquely identifies a resource
    /// definition, profile, or other FHIR artifact using a canonical
    /// identifier.
    ///
    /// ## Purpose
    ///
    /// - Reference FHIR definitions and profiles using canonical URLs
    /// - Provide version-independent references to FHIR artifacts
    /// - Support profile validation and conformance checking
    /// - Enable cross-referencing between FHIR resources and definitions
    /// - Maintain stable references across resource versions
    ///
    /// ## Usage
    ///
    /// The `valueCanonical` property is used in extensions, parameters, and
    /// elements that need to reference FHIR resource definitions, profiles,
    /// value sets, code systems, or other canonically-identified artifacts.
    ///
    /// ## Data Type
    ///
    /// **canonical** - A URI that refers to a resource by its canonical URL
    ///
    /// ## Constraints
    ///
    /// - Must be a valid URI format
    /// - Should reference an existing FHIR resource with a canonical URL
    /// - May include version information using the format: `url|version`
    /// - Must conform to canonical URI patterns defined in FHIR
    /// - Should be resolvable to a FHIR resource definition
    ///
    /// ## Examples
    ///
    /// ### Extension with Profile Reference
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/conformsToProfile",
    ///       "valueCanonical": "http://hl7.org/fhir/StructureDefinition/Patient"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with ValueSet Reference
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "valueSet",
    ///       "valueCanonical": "http://hl7.org/fhir/ValueSet/administrative-gender|5.0.0"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueUri` - General URI values
    /// - `valueUrl` - URL references
    /// - `canonical` - Direct canonical references
    /// - `profile` - Profile references
    /// - `targetProfile` - Target profile references
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [canonical](http://hl7.org/fhir/R5/datatypes.html#canonical)
    ///
    pub value_canonical: Option<String>,

    /// # valueCode
    ///
    /// ## Description
    ///
    /// The `valueCode` property contains a coded value from a defined set of
    /// codes. It represents a single code token that is selected from a
    /// specific code system or value set, providing structured and standardized
    /// data representation.
    ///
    /// ## Purpose
    ///
    /// - Store coded values from predefined code systems
    /// - Ensure data consistency through controlled vocabularies
    /// - Support interoperability with standard coding schemes
    /// - Enable precise and unambiguous data representation
    /// - Facilitate automated processing and validation
    ///
    /// ## Usage
    ///
    /// The `valueCode` property is used in extensions, parameters, and data
    /// elements where a single code value is required from a specific code
    /// system or constrained value set.
    ///
    /// ## Data Type
    ///
    /// **code** - A string that represents a code from a code system
    ///
    /// ## Constraints
    ///
    /// - Must be a valid token (no whitespace, limited punctuation)
    /// - Should come from a defined code system or value set
    /// - Case-sensitive unless explicitly defined otherwise
    /// - Must match the pattern defined by the binding value set
    /// - Should be a meaningful code within its context
    ///
    /// ## Examples
    ///
    /// ### Extension with Status Code
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/approval-status",
    ///       "valueCode": "approved"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Gender Code
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "gender",
    ///       "valueCode": "female"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueCoding` - Coded values with system and display
    /// - `valueCodeableConcept` - Complex coded concepts
    /// - `code` - Direct code values
    /// - `system` - Code system identifier
    /// - `value` - Generic value field
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [code](http://hl7.org/fhir/R5/datatypes.html#code)
    ///
    pub value_code: Option<String>,

    /// # valueDateTime
    ///
    /// ## Description
    ///
    /// The `valueDateTime` property contains date and time values with optional
    /// time zone information. It represents precise temporal data points using
    /// ISO 8601 format, supporting various levels of precision from year to
    /// seconds with fractional seconds.
    ///
    /// ## Purpose
    ///
    /// - Store precise date and time information
    /// - Support temporal queries and calculations
    /// - Enable time-based ordering and filtering
    /// - Provide timezone-aware temporal data
    /// - Support various precision levels as clinically appropriate
    ///
    /// ## Usage
    ///
    /// The `valueDateTime` property is used in extensions, parameters, and data
    /// elements where precise temporal information is required, such as
    /// timestamps, event times, or scheduled dates.
    ///
    /// ## Data Type
    ///
    /// **dateTime** - A date, date-time or partial date (ISO 8601 format)
    ///
    /// ## Constraints
    ///
    /// - Must follow ISO 8601 format (YYYY-MM-DDTHH:MM:SS[.ffffff][±HH:MM|Z])
    /// - Precision can range from year (YYYY) to fractional seconds
    /// - Time zone is recommended for precise timestamps
    /// - Must be a valid date and time combination
    /// - Should use UTC time zone for interoperability when possible
    ///
    /// ## Examples
    ///
    /// ### Extension with Precise Timestamp
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/event-timestamp",
    ///       "valueDateTime": "2024-01-15T10:30:45.123Z"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Date and Time Zone
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "scheduledTime",
    ///       "valueDateTime": "2024-03-20T14:00:00-05:00"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueDate` - Date only values
    /// - `valueInstant` - Precise timestamps
    /// - `valuePeriod` - Date/time ranges
    /// - `date` - Simple date fields
    /// - `effectiveDateTime` - Effective date/time
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [dateTime](http://hl7.org/fhir/R5/datatypes.html#dateTime)
    ///
    pub value_date_time: Option<String>,

    /// # valueId
    ///
    /// ## Description
    ///
    /// The `valueId` property contains identifier values that conform to FHIR
    /// id constraints. It represents logical identifiers that are used to
    /// reference resources within a server or across FHIR systems, following
    /// specific formatting rules for maximum interoperability.
    ///
    /// ## Purpose
    ///
    /// - Store logical identifiers for FHIR resources
    /// - Provide unique reference mechanisms within systems
    /// - Support resource linking and cross-referencing
    /// - Enable consistent identifier formatting across implementations
    /// - Facilitate resource resolution and retrieval
    ///
    /// ## Usage
    ///
    /// The `valueId` property is used in extensions, parameters, and data
    /// elements where FHIR-compliant identifiers are required for referencing
    /// resources or creating unique logical identifiers.
    ///
    /// ## Data Type
    ///
    /// **id** - A string constrained to contain only valid id characters
    ///
    /// ## Constraints
    ///
    /// - Must be 1-64 characters in length
    /// - Can only contain letters (a-z, A-Z), numbers (0-9), hyphens (-), and
    ///   periods (.)
    /// - Must not start or end with a period
    /// - Case sensitive
    /// - Must be unique within the context where it's used
    ///
    /// ## Examples
    ///
    /// ### Extension with Resource Reference ID
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/linked-resource",
    ///       "valueId": "patient-12345"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Generated ID
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "resourceId",
    ///       "valueId": "obs-bp-2024-001"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueString` - General string values
    /// - `identifier` - Complex identifier objects
    /// - `id` - Direct resource identifiers
    /// - `reference` - Resource references
    /// - `fullUrl` - Complete resource URLs
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [id](http://hl7.org/fhir/R5/datatypes.html#id)
    ///
    pub value_id: Option<String>,

    /// # valueInteger
    ///
    /// ## Description
    ///
    /// The `valueInteger` property contains whole number values within the
    /// 32-bit signed integer range. It represents numeric data for counts,
    /// measurements, ages, durations, and other quantifiable values that don't
    /// require decimal precision.
    ///
    /// ## Purpose
    ///
    /// - Store whole number quantities and measurements
    /// - Represent counts, ages, and duration values
    /// - Support numeric calculations and comparisons
    /// - Provide precise integer-based data capture
    /// - Enable mathematical operations in clinical contexts
    ///
    /// ## Usage
    ///
    /// The `valueInteger` property is used in extensions, parameters, and data
    /// elements where whole number values are required, such as age in years,
    /// count of items, or sequence numbers.
    ///
    /// ## Data Type
    ///
    /// **integer** - A signed 32-bit integer (range: -2,147,483,648 to
    /// 2,147,483,647)
    ///
    /// ## Constraints
    ///
    /// - Must be within the 32-bit signed integer range
    /// - Cannot contain decimal points or fractional parts
    /// - Negative values are permitted
    /// - Must be a valid integer representation
    /// - Leading zeros are typically not significant
    ///
    /// ## Examples
    ///
    /// ### Extension with Age Value
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/age-at-diagnosis",
    ///       "valueInteger": 45
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Count Value
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "itemCount",
    ///       "valueInteger": 12
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valuePositiveInt` - Positive integers only
    /// - `valueUnsignedInt` - Unsigned integers
    /// - `valueInteger64` - 64-bit integers
    /// - `valueDecimal` - Decimal numbers
    /// - `value` - Generic value field
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [integer](http://hl7.org/fhir/R5/datatypes.html#integer)
    ///
    pub value_integer: Option<i32>,

    /// # valueInteger64
    ///
    /// ## Description
    ///
    /// The `valueInteger64` property contains whole number values within the
    /// 64-bit signed integer range. It represents large numeric values that
    /// exceed the 32-bit integer range, supporting high-precision counts,
    /// timestamps, and large-scale measurements.
    ///
    /// ## Purpose
    ///
    /// - Store large whole number quantities beyond 32-bit range
    /// - Represent high-precision timestamps and counters
    /// - Support large-scale measurements and calculations
    /// - Enable storage of very large identifiers or sequence numbers
    /// - Provide extended range for clinical and research data
    ///
    /// ## Usage
    ///
    /// The `valueInteger64` property is used in extensions, parameters, and
    /// data elements where 64-bit integer precision is required, such as
    /// microsecond timestamps, large patient identifiers, or high-volume
    /// counters.
    ///
    /// ## Data Type
    ///
    /// **integer64** - A signed 64-bit integer (range:
    /// -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    ///
    /// ## Constraints
    ///
    /// - Must be within the 64-bit signed integer range
    /// - Cannot contain decimal points or fractional parts
    /// - Negative values are permitted
    /// - Must be a valid integer representation
    /// - Should be used only when 32-bit integers are insufficient
    ///
    /// ## Examples
    ///
    /// ### Extension with Large Identifier
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/system-generated-id",
    ///       "valueInteger64": "9876543210123456789"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with High-Precision Timestamp
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "microsecondTimestamp",
    ///       "valueInteger64": "1705320645123456"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueInteger` - 32-bit integers
    /// - `valuePositiveInt` - Positive integers only
    /// - `valueUnsignedInt` - Unsigned integers
    /// - `valueDecimal` - Decimal numbers
    /// - `value` - Generic value field
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [integer64](http://hl7.org/fhir/R5/datatypes.html#integer64)
    ///
    pub value_integer_64: Option<String>,

    /// # valueMarkdown
    ///
    /// ## Description
    ///
    /// The `valueMarkdown` property contains text formatted using Markdown
    /// syntax. It supports rich text formatting including headers, lists,
    /// links, emphasis, and other Markdown elements while maintaining
    /// machine-readable structure.
    ///
    /// ## Purpose
    ///
    /// - Store formatted documentation and narrative text
    /// - Provide rich text capabilities for clinical notes
    /// - Support structured documentation with formatting
    /// - Enable consistent text presentation across systems
    /// - Maintain both human-readable and machine-processable content
    ///
    /// ## Usage
    ///
    /// The `valueMarkdown` property is used in extensions, parameters, and data
    /// elements where formatted text content is needed, such as clinical
    /// guidelines, patient instructions, or detailed descriptions.
    ///
    /// ## Data Type
    ///
    /// **markdown** - A string containing Markdown-formatted text
    ///
    /// ## Constraints
    ///
    /// - Must contain valid Markdown syntax
    /// - Should be properly escaped for JSON representation
    /// - Line breaks should use appropriate Markdown formatting
    /// - Embedded HTML should be minimal and safe
    /// - Should render consistently across different viewers
    ///
    /// ## Examples
    ///
    /// ### Extension with Clinical Instructions
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/patient-instructions",
    ///       "valueMarkdown": "## Post-Procedure Care\n\n**Immediately after procedure:**\n- Rest for 24 hours\n- Take prescribed medication every 6 hours\n\n**Follow-up:**\n- Schedule appointment in 2 weeks\n- Call if symptoms worsen\n\n*For emergencies, call 911 immediately.*"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Formatted Description
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "procedureDescription",
    ///       "valueMarkdown": "### Cardiac Catheterization\n\nA **minimally invasive** procedure involving:\n\n1. Local anesthesia\n2. Catheter insertion\n3. Contrast dye injection\n4. Real-time imaging\n\nExpected duration: *45-90 minutes*"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueString` - Plain text values
    /// - `text` - Narrative content
    /// - `div` - HTML-formatted content
    /// - `documentation` - Technical documentation
    /// - `description` - Simple descriptions
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [markdown](http://hl7.org/fhir/R5/datatypes.html#markdown)
    ///
    pub value_markdown: Option<String>,

    /// # valuePeriod
    ///
    /// ## Description
    ///
    /// The `valuePeriod` property contains a time period defined by start and
    /// end date/time values. It represents duration-based data such as
    /// treatment periods, enrollment spans, validity ranges, or any
    /// time-bounded clinical events.
    ///
    /// ## Purpose
    ///
    /// - Define time ranges for clinical events and treatments
    /// - Specify validity periods for data and authorizations
    /// - Represent duration-based measurements and observations
    /// - Support temporal queries and filtering
    /// - Enable time-based business rule evaluation
    ///
    /// ## Usage
    ///
    /// The `valuePeriod` property is used in extensions, parameters, and data
    /// elements where time ranges need to be specified, such as treatment
    /// periods, coverage spans, or observation intervals.
    ///
    /// ## Data Type
    ///
    /// **Period** - A complex type containing:
    ///
    /// - `start` - Start date/time
    /// - `end` - End date/time
    ///
    /// ## Constraints
    ///
    /// - Start must be before or equal to end if both are present
    /// - At least one of start or end should be present
    /// - Date/time values must be valid ISO 8601 format
    /// - Open-ended periods can omit start or end
    /// - Should include time zones for precision
    ///
    /// ## Examples
    ///
    /// ### Extension with Treatment Period
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/treatment-period",
    ///       "valuePeriod": {
    ///         "start": "2024-01-15T09:00:00Z",
    ///         "end": "2024-03-15T17:00:00Z"
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Coverage Period
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "coveragePeriod",
    ///       "valuePeriod": {
    ///         "start": "2024-01-01T00:00:00Z",
    ///         "end": "2024-12-31T23:59:59Z"
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueDateTime` - Single date/time values
    /// - `effectivePeriod` - Effective time ranges
    /// - `start` - Period start times
    /// - `end` - Period end times
    /// - `period` - Direct period objects
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [Period](http://hl7.org/fhir/R5/datatypes.html#Period)
    ///
    pub value_period: Option<Period>,

    /// # valuePositiveInt
    ///
    /// ## Description
    ///
    /// The `valuePositiveInt` property contains positive integer values
    /// starting from 1. It represents counts, quantities, and measurements
    /// where zero and negative values are not meaningful or allowed.
    ///
    /// ## Purpose
    ///
    /// - Store counts that must be greater than zero
    /// - Represent quantities where positive values are required
    /// - Ensure data validity for naturally positive measurements
    /// - Support constraints on numeric data entry
    /// - Enable validation of meaningful positive quantities
    ///
    /// ## Usage
    ///
    /// The `valuePositiveInt` property is used in extensions, parameters, and
    /// data elements where only positive integer values make sense, such as
    /// dosage counts, repetition numbers, or sequence positions.
    ///
    /// ## Data Type
    ///
    /// **positiveInt** - An integer greater than 0 (range: 1 to 2,147,483,647)
    ///
    /// ## Constraints
    ///
    /// - Must be greater than 0 (minimum value is 1)
    /// - Cannot be zero or negative
    /// - Must be within 32-bit signed integer positive range
    /// - Should be used when zero values are not meaningful
    /// - Validates that meaningful positive quantities are provided
    ///
    /// ## Examples
    ///
    /// ### Extension with Dosage Count
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/daily-dose-count",
    ///       "valuePositiveInt": 3
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Repetition Count
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "exerciseRepetitions",
    ///       "valuePositiveInt": 15
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueInteger` - All integers including negative and zero
    /// - `valueUnsignedInt` - Non-negative integers (including zero)
    /// - `valueInteger64` - 64-bit integers
    /// - `count` - Count values
    /// - `value` - Generic value field
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [positiveInt](http://hl7.org/fhir/R5/datatypes.html#positiveInt)
    ///
    pub value_positive_int: Option<i64>,

    /// TODO
    ///
    /// Example: "1" to "9223372036854775807"
    ///
    pub value_positive_int_64: Option<String>,

    /// # valueString
    ///
    /// ## Description
    ///
    /// The `valueString` property contains string values in FHIR resources. It
    /// represents textual data, free-text content, or coded string values that
    /// don't require complex structure.
    ///
    /// ## Purpose
    ///
    /// - Store textual data and free-form content
    /// - Represent simple string-based values
    /// - Support human-readable text content
    /// - Enable flexible text-based data capture
    /// - Provide string values for extensions and parameters
    ///
    /// ## Usage
    ///
    /// The `valueString` property is used extensively throughout FHIR resources
    /// for storing text values, particularly in extensions, parameters, and
    /// simple data elements.
    ///
    /// ## Data Type
    ///
    /// **string** - A sequence of Unicode characters
    ///
    /// ## Constraints
    ///
    /// - Must be valid Unicode text
    /// - Length limitations may apply based on context
    /// - Should not contain control characters unless specifically allowed
    /// - May have format constraints depending on usage context
    ///
    /// ## Examples
    ///
    /// ### Extension with String Value
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/patient-note",
    ///       "valueString": "Patient prefers morning appointments"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter String Value
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "display",
    ///       "valueString": "Blood Pressure Reading"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueCode` - Coded string values
    /// - `valueMarkdown` - Markdown formatted text
    /// - `valueUri` - URI string values
    /// - `text` - Narrative text content
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [string](http://hl7.org/fhir/R5/datatypes.html#string)
    ///
    pub value_string: Option<String>,

    /// # valueUnsignedInt
    ///
    /// ## Description
    ///
    /// The `valueUnsignedInt` property contains non-negative integer values
    /// starting from 0. It represents counts, indices, and measurements where
    /// negative values are not meaningful but zero is acceptable.
    ///
    /// ## Purpose
    ///
    /// - Store counts that can be zero or positive
    /// - Represent array indices and position values
    /// - Support measurements where zero is meaningful
    /// - Enable validation of non-negative quantities
    /// - Provide precise non-negative integer constraints
    ///
    /// ## Usage
    ///
    /// The `valueUnsignedInt` property is used in extensions, parameters, and
    /// data elements where non-negative integers are required, such as array
    /// positions, occurrence counts, or measurements that can be zero.
    ///
    /// ## Data Type
    ///
    /// **unsignedInt** - A non-negative integer (range: 0 to 2,147,483,647)
    ///
    /// ## Constraints
    ///
    /// - Must be greater than or equal to 0
    /// - Cannot be negative
    /// - Must be within 32-bit unsigned integer range
    /// - Should be used when zero values are meaningful
    /// - Validates that non-negative quantities are provided
    ///
    /// ## Examples
    ///
    /// ### Extension with Array Index
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/sequence-position",
    ///       "valueUnsignedInt": 0
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Count Value
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "errorCount",
    ///       "valueUnsignedInt": 3
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valuePositiveInt` - Positive integers only (minimum 1)
    /// - `valueInteger` - All integers including negative
    /// - `valueInteger64` - 64-bit integers
    /// - `count` - Count values
    /// - `value` - Generic value field
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [unsignedInt](http://hl7.org/fhir/R5/datatypes.html#unsignedInt)
    ///
    pub value_unsigned_int: Option<i64>,

    /// TODO
    ///
    /// Example: "0" to "18446744073709551615"
    ///
    pub value_unsigned_int_64: Option<String>,

    /// # valueUri
    ///
    /// ## Description
    ///
    /// The `valueUri` property contains Uniform Resource Identifier (URI)
    /// values that can reference external resources, namespaces, or
    /// identifiers. It supports both absolute and relative URIs following RFC
    /// 3986 standards.
    ///
    /// ## Purpose
    ///
    /// - Reference external resources and services
    /// - Identify namespaces and vocabularies
    /// - Provide links to documentation and specifications
    /// - Enable resource identification and resolution
    /// - Support distributed system integration
    ///
    /// ## Usage
    ///
    /// The `valueUri` property is used in extensions, parameters, and data
    /// elements where URI references are needed, such as namespace identifiers,
    /// external resource links, or system identifiers.
    ///
    /// ## Data Type
    ///
    /// **uri** - A Uniform Resource Identifier (RFC 3986)
    ///
    /// ## Constraints
    ///
    /// - Must be a valid URI according to RFC 3986
    /// - Can be absolute or relative
    /// - Should be properly encoded for special characters
    /// - Length should be reasonable for practical use
    /// - Should be resolvable when referencing external resources
    ///
    /// ## Examples
    ///
    /// ### Extension with Namespace URI
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/namespace-reference",
    ///       "valueUri": "http://example.org/fhir/namespace/clinical-data"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with Documentation Link
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "documentationUrl",
    ///       "valueUri": "https://example.org/docs/api-guide#section-3"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueUrl` - URLs specifically
    /// - `valueCanonical` - Canonical resource references
    /// - `uri` - Direct URI fields
    /// - `url` - URL fields
    /// - `system` - System identifiers
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [uri](http://hl7.org/fhir/R5/datatypes.html#uri)
    ///
    pub value_uri: Option<String>,

    /// # valueUrl
    ///
    /// ## Description
    ///
    /// The `valueUrl` property contains Uniform Resource Locator (URL) values
    /// that specify the location of web-accessible resources. It represents
    /// addresses for web services, documents, images, or other
    /// network-accessible content.
    ///
    /// ## Purpose
    ///
    /// - Reference web-accessible resources and services
    /// - Provide links to external documents and media
    /// - Enable access to remote endpoints and APIs
    /// - Support hyperlink functionality in FHIR resources
    /// - Facilitate integration with web-based systems
    ///
    /// ## Usage
    ///
    /// The `valueUrl` property is used in extensions, parameters, and data
    /// elements where web URLs are needed, such as links to external documents,
    /// web service endpoints, or multimedia content.
    ///
    /// ## Data Type
    ///
    /// **url** - A Uniform Resource Locator (absolute URL)
    ///
    /// ## Constraints
    ///
    /// - Must be an absolute URL (includes scheme and authority)
    /// - Should be accessible over standard web protocols (HTTP/HTTPS)
    /// - Must be properly encoded for special characters
    /// - Should be a valid, well-formed URL
    /// - Should resolve to accessible content when dereferenced
    ///
    /// ## Examples
    ///
    /// ### Extension with Document URL
    ///
    /// ```json
    /// {
    ///   "extension": [
    ///     {
    ///       "url": "http://example.org/fhir/StructureDefinition/patient-photo",
    ///       "valueUrl": "https://example.org/patient-photos/12345.jpg"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Parameter with API Endpoint
    ///
    /// ```json
    /// {
    ///   "parameter": [
    ///     {
    ///       "name": "callbackUrl",
    ///       "valueUrl": "https://client-system.example.org/fhir/callback"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `valueUri` - General URI references
    /// - `valueCanonical` - Canonical resource references
    /// - `url` - Direct URL fields
    /// - `fullUrl` - Complete resource URLs
    /// - `endpoint` - Service endpoints
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types: [url](http://hl7.org/fhir/R5/datatypes.html#url)
    ///
    pub value_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Example;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("example")
            .join("example.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.label, "my label");
    }
}
