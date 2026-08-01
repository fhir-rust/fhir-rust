//! # contact
//!
//! ## Description
//!
//! The `contact` attribute provides contact information for individuals or
//! organizations associated with a FHIR resource. This includes names, roles,
//! telecommunications details (phone, email, fax), and other means of
//! communication. In canonical resources like StructureDefinitions and
//! ValueSets, contact information typically identifies maintainers, authors, or
//! support personnel who can provide assistance with the resource. In clinical
//! resources, it may represent care team members, emergency contacts, or
//! administrative contacts.
//!
//! ## Purpose
//!
//! The `contact` exists to:
//!
//! - Provide communication channels for resource maintainers and support
//!   personnel
//! - Enable stakeholder identification for canonical resources and
//!   implementation guides  
//! - Support collaboration and feedback mechanisms for FHIR artifacts
//! - Facilitate clinical communication for patient care coordination
//! - Enable emergency contact information for patients and care scenarios
//! - Provide organizational contact points for administrative and business
//!   processes
//! - Support regulatory and compliance communication requirements
//!
//! ## Usage
//!
//! Use the `contact` attribute when:
//!
//! - Publishing canonical resources that require maintainer or author
//!   identification
//! - Creating implementation guides with support contact information
//! - Managing patient emergency contacts and care team communication
//! - Establishing organizational contact points for business relationships
//! - Supporting regulatory submissions that require contact information
//! - Enabling collaboration on FHIR artifacts and clinical content
//! - Providing support channels for users of FHIR resources and systems
//!
//! Contact information should be current, accurate, and appropriate for the
//! intended use.
//!
//! ## Data Type
//!
//! **ContactDetail** - A complex structure containing:
//!
//! - `name` (string): Name of the contact person or organization
//! - `telecom` (ContactPoint[]): Telecommunications details (phone, email, fax,
//!   etc.)
//!
//! **ContactPoint elements include:**
//!
//! - `system` (code): Communication system
//!   (phone|fax|email|pager|url|sms|other)
//! - `value` (string): The actual contact value (phone number, email address,
//!   etc.)
//! - `use` (code): Purpose of the contact (home|work|temp|old|mobile)
//! - `rank` (positiveInt): Preference order for multiple contacts
//! - `period` (Period): Time period when contact is valid
//!
//! ## Constraints
//!
//! - **Required**: Optional for most resources, recommended for canonical
//!   resources
//! - **Cardinality**: 0..* (zero or more contacts per resource)
//! - **Telecom Systems**: Must use valid values from ContactPointSystem value
//!   set
//! - **Use Codes**: Must use valid values from ContactPointUse value set
//! - **Completeness**: Should include sufficient information for effective
//!   communication
//! - **Privacy**: Should respect privacy requirements and data protection
//!   regulations
//! - **Currency**: Contact information should be kept current and accurate
//!
//! ## Examples
//!
//! See the accompanying `example.json` file for complete resources
//! demonstrating contact usage in StructureDefinitions, Organizations, and
//! Patient resources with various contact types and telecommunications details.
//!
//! ## Related Keys
//!
//! - `name` - Name of the contact person or organization
//! - `telecom` - Telecommunications contact points including phone, email, and
//!   other systems
//! - `system` - Type of telecommunications system used for contact
//! - `value` - Actual contact value such as phone number or email address
//! - `use` - Purpose or context of the contact information
//! - `publisher` - Entity responsible for the resource, often related to
//!   primary contact
//! - `author` - Resource authors who may also serve as contact points
//!
//! ## Specification Reference
//!
//! Based on FHIR R5 specification. For complete details, refer to the official
//! FHIR R5 documentation for ContactDetail data type and ContactPoint structure
//! definitions.

use crate::r5::parse::all::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Contact {
    /// # name
    ///
    /// ## Description
    ///
    /// The `name` attribute represents a human-readable identifier or label
    /// used throughout FHIR R5 resources to provide meaningful, user-friendly
    /// text for various elements. It serves as the primary textual identifier
    /// that humans use to recognize, reference, and work with healthcare
    /// concepts, entities, and data elements.
    ///
    /// ## Purpose
    ///
    /// The `name` exists to provide human-readable identification across FHIR
    /// resources, enabling:
    ///
    /// - User-friendly display of resource information
    /// - Searchable and recognizable labels for healthcare entities
    /// - Support for multiple naming conventions and languages
    /// - Clear identification in user interfaces and documentation
    /// - Meaningful references in clinical workflows and communications
    ///
    /// ## Usage
    ///
    /// Use the `name` attribute when:
    ///
    /// - Defining patient names with proper structure (family, given names)
    /// - Naming healthcare providers, organizations, and facilities
    /// - Labeling medication and substance names
    /// - Creating human-readable identifiers for plans and protocols
    /// - Providing searchable names for locations and services
    /// - Establishing clear references for coded concepts
    ///
    /// Names should be accurate, culturally appropriate, and suitable for the
    /// intended use context.
    ///
    /// ## Data Type
    ///
    /// **varies by context** - Common patterns include:
    ///
    /// - **HumanName** - Structured representation for person names (family,
    ///   given, prefix, suffix)
    /// - **string** - Simple text name for organizations, medications, and
    ///   other entities
    /// - **Array of HumanName** - Multiple name representations with different
    ///   uses
    /// - **Complex structures** - May include use codes, periods of validity,
    ///   and preferred flags
    ///
    /// ## Constraints
    ///
    /// - **Required**: Conditional - often required for key identifying
    ///   elements
    /// - **Cardinality**: Varies by context (0..1, 0..*, or 1..1)
    /// - **Format**: Should follow cultural and linguistic conventions
    /// - **Validation**: May include format checking for structured names
    /// - **Uniqueness**: Not required to be unique across systems
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` file for a comprehensive example
    /// showing various `name` attribute uses across different FHIR resources
    /// and contexts.
    ///
    /// ## Related Keys
    ///
    /// - `family` - Family name component in HumanName structures
    /// - `given` - Given name components in HumanName structures
    /// - `use` - Context or purpose of the name (official, usual, nickname)
    /// - `text` - Complete name as a single string
    /// - `period` - Time period when the name was/is in use
    ///
    /// ## Specification Reference
    ///
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for HumanName data types, naming
    /// conventions, and context-specific name requirements.
    ///
    name: Option<String>,

    /// # telecom
    ///
    /// ## Description
    ///
    /// The `telecom` property contains contact information such as phone
    /// numbers, email addresses, fax numbers, and other telecommunications
    /// details for individuals, organizations, or related persons.
    ///
    /// ## Purpose
    ///
    /// - Provide contact information for communication
    /// - Support multiple communication methods and preferences
    /// - Enable prioritized contact options
    /// - Facilitate electronic and voice communications
    /// - Support emergency and routine contact scenarios
    ///
    /// ## Usage
    ///
    /// The `telecom` property is used across many FHIR resources (Patient,
    /// Practitioner, Organization, RelatedPerson, etc.) to provide
    /// telecommunication contact details.
    ///
    /// ## Data Type
    ///
    /// **Array of ContactPoint** - Each ContactPoint contains:
    ///
    /// - `system` - Type of contact (phone, email, fax, etc.)
    /// - `value` - The actual contact value
    /// - `use` - Purpose of contact (home, work, mobile, etc.)
    /// - `rank` - Priority order for contact attempts
    /// - `period` - Time period when contact is valid
    ///
    /// ## Constraints
    ///
    /// - Must specify both system and value
    /// - System values must be from the ContactPointSystem value set
    /// - Use values must be from the ContactPointUse value set
    /// - Phone numbers should follow standard formatting
    /// - Email addresses must be valid email format
    ///
    /// ## Examples
    ///
    /// ### Basic Contact Information
    ///
    /// ```json
    /// {
    ///   "telecom": [
    ///     {
    ///       "system": "phone",
    ///       "value": "+1-555-123-4567",
    ///       "use": "home"
    ///     },
    ///     {
    ///       "system": "email",
    ///       "value": "patient@example.com",
    ///       "use": "home"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### Prioritized Contact with Period
    ///
    /// ```json
    /// {
    ///   "telecom": [
    ///     {
    ///       "system": "phone",
    ///       "value": "555-123-4567",
    ///       "use": "mobile",
    ///       "rank": 1
    ///     },
    ///     {
    ///       "system": "phone",
    ///       "value": "555-987-6543",
    ///       "use": "work",
    ///       "rank": 2,
    ///       "period": {
    ///         "start": "2024-01-01",
    ///         "end": "2024-12-31"
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `address` - Physical address information
    /// - `name` - Name information
    /// - `contact` - Organization contact details
    /// - `communication` - Communication preferences and languages
    ///
    /// ## Specification Reference
    ///
    /// FHIR R5 Data Types:
    /// [ContactPoint](http://hl7.org/fhir/R5/datatypes.html#ContactPoint)
    ///
    telecom: Vec<Identifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contact;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("contact")
            .join("contact.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.name.unwrap(), "FHIR project team");
    }
}
