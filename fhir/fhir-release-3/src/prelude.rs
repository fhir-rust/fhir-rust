//! Common imports for working with FHIR R3.
//!
//! ```
//! use fhir::r3::prelude::*;
//!
//! let patient = Patient::default();
//! let outcome = patient.validate(); // `Validate` is in scope
//! assert!(outcome.is_empty());
//! ```
//!
//! The FHIR `String` primitive is re-exported as [`FhirString`] to avoid
//! shadowing [`std::string::String`].

// The polymorphic resource enum and the most-used resources.
pub use crate::r3::resources::{
    Bundle, Condition, Encounter, MedicationRequest, Observation, Organization, Patient,
    Practitioner, Procedure, Resource,
};

// The most-used datatypes. `String` is aliased to avoid shadowing std.
pub use crate::r3::types::String as FhirString;
pub use crate::r3::types::{
    Boolean, Code, CodeableConcept, Coding, Date, DateTime, Decimal, Element, Extension, HumanName,
    Identifier, Instant, Integer, Period, Quantity, Reference, Uri,
};

// Coded values, choice/extension support, and validation.
pub use crate::r3::choice::Primitive;
pub use crate::r3::coded::Coded;
pub use crate::r3::extension_ext::{
    ExtensionExt, HasExtension, HasModifierExtension, ModifierExtensionExt,
};
pub use crate::r3::validate::{Validate, ValidationIssue};

// A few frequently-used code enums.
pub use crate::r3::codes::{
    AdministrativeGender, BundleType, ObservationStatus, PublicationStatus,
};
