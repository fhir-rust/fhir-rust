//! A coded value that is usually a known enum variant but tolerates any code.
//!
//! FHIR elements with a `required` binding draw their value from a value set, so
//! this crate types them as the matching release-specific `codes` enum
//! ([`r4::codes`](crate::r4::codes), [`r5::codes`](crate::r5::codes)). But real
//! data occasionally carries a code outside the set (a newer code, a local
//! extension, or simply invalid data), and a data-exchange library must not fail
//! to parse it. [`Coded<E>`] wraps the enum with an [`Unknown`](Coded::Unknown)
//! fallback so every wire value round-trips. See `spec/05-code-systems.md`.
//!
//! The wrapper itself is release-independent — it is generic over the enum — so
//! it is defined once here and re-exported as [`r4::coded`](crate::r4::coded)
//! and [`r5::coded`](crate::r5::coded).
//!
//! ```
//! use fhir::coded::Coded;
//!
//! // Stands in for a generated `codes` enum such as `AdministrativeGender`.
//! #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
//! enum Gender {
//!     #[serde(rename = "female")]
//!     Female,
//! }
//!
//! // A known code parses into the enum variant.
//! let known: Coded<Gender> =
//!     serde_json::from_value(serde_json::json!("female")).unwrap();
//! assert_eq!(known, Coded::Known(Gender::Female));
//!
//! // An unrecognized code is preserved as-is rather than rejected.
//! let unknown: Coded<Gender> =
//!     serde_json::from_value(serde_json::json!("robot")).unwrap();
//! assert_eq!(unknown, Coded::Unknown("robot".to_string()));
//!
//! // Both round-trip to their original code string.
//! assert_eq!(serde_json::to_value(&known).unwrap(), "female");
//! assert_eq!(serde_json::to_value(&unknown).unwrap(), "robot");
//! ```

use ::serde::{Deserialize, Serialize};

use crate::validate::{Validate, ValidationIssue};

/// A coded value: a known `codes` enum variant `E`, or any other code string
/// preserved verbatim.
///
/// Deserialization tries `E` first (untagged) and falls back to
/// [`Unknown`](Self::Unknown); serialization emits the underlying code string in
/// either case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coded<E> {
    /// A recognized code from the bound value set.
    Known(E),
    /// A code outside the value set, preserved for round-tripping.
    Unknown(String),
}

impl<E: Default> Default for Coded<E> {
    fn default() -> Self {
        Coded::Known(E::default())
    }
}

impl<E> Coded<E> {
    /// The known enum variant, if this value is recognized.
    pub fn known(&self) -> Option<&E> {
        match self {
            Coded::Known(e) => Some(e),
            Coded::Unknown(_) => None,
        }
    }

    /// Whether this value is an unrecognized code.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Coded::Unknown(_))
    }
}

impl<E: Serialize> Coded<E> {
    /// The underlying FHIR code string (the enum's canonical code, or the raw
    /// unknown code).
    #[must_use]
    pub fn code(&self) -> String {
        match self {
            Coded::Known(e) => ::serde_json::to_value(e)
                .ok()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_default(),
            Coded::Unknown(s) => s.clone(),
        }
    }
}

impl<E> Validate for Coded<E> {
    // Every `Coded` field has a `required` binding (that is why it was typed as
    // an enum), so an `Unknown` code is outside the value set (T13).
    fn validate(&self) -> Vec<ValidationIssue> {
        match self {
            Coded::Known(_) => Vec::new(),
            Coded::Unknown(code) => vec![ValidationIssue::new(
                "code",
                format!("code {code:?} is not in the required value set"),
            )],
        }
    }
}
