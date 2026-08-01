//! Common imports for working with FHIR R5.
//!
//! ```
//! use fhir::prelude::*;
//!
//! let patient = Patient::default();
//! let outcome = patient.validate(); // `Validate` is in scope
//! assert!(outcome.is_empty());
//! ```
//!
//! This is the R5 prelude, kept at the crate root because R5 is the default
//! release. It re-exports [`r5::prelude`](crate::r5::prelude); for R4, use
//! [`r4::prelude`](crate::r4::prelude).

pub use crate::r5::prelude::*;
