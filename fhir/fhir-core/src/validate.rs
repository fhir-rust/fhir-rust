//! The version-independent core of FHIR validation.
//!
//! Validation is identical in shape across FHIR releases: a value reports a
//! list of [`ValidationIssue`]s, and container types (`Option`, `Vec`,
//! [`Vec1`](::vec1::Vec1), `Box`) delegate to what they hold. Only the
//! *primitive format constraints* and the `OperationOutcome` bridge are
//! release-specific, so those live in the per-release modules
//! ([`crate::r4::validate`] and [`crate::r5::validate`]) which re-export the
//! [`Validate`] trait and [`ValidationIssue`] type defined here.
//!
//! Because a single trait is shared, one `#[derive(Validate)]` implementation
//! serves every release and a generic helper can validate R4 and R5 values
//! alike.
//!
//! ```
//! use fhir::validate::{Validate, ValidationIssue};
//!
//! // Containers delegate: an empty `Option` has nothing to report.
//! let nothing: Option<String> = None;
//! assert!(nothing.validate().is_empty());
//!
//! // An issue names what failed and why.
//! let issue = ValidationIssue { path: "gender".to_string(), message: "bad".to_string() };
//! assert_eq!(issue.path, "gender");
//! ```

/// A single validation problem, with the location and a human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// A path or label identifying what failed (e.g. the datatype name).
    pub path: String,
    /// A human-readable description of the problem.
    pub message: String,
}

impl ValidationIssue {
    /// Build an issue at `path`.
    ///
    /// Public because the generated release crates construct issues, and they
    /// are no longer the same crate as this one.
    #[must_use]
    pub fn new(path: &str, message: impl Into<String>) -> Self {
        Self {
            path: path.to_string(),
            message: message.into(),
        }
    }
}

/// Types that can validate themselves against FHIR constraints.
pub trait Validate {
    /// Return all validation issues; an empty vector means the value is valid.
    fn validate(&self) -> Vec<ValidationIssue>;

    /// Convenience: `true` when [`Validate::validate`] finds no issues.
    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        self.as_ref().map(Validate::validate).unwrap_or_default()
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        self.iter().flat_map(Validate::validate).collect()
    }
}

/// A non-empty `Vec1` (used for FHIR `1..*` elements) validates each element.
impl<T: Validate> Validate for ::vec1::Vec1<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        self.iter().flat_map(Validate::validate).collect()
    }
}

impl<T: Validate> Validate for Box<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        (**self).validate()
    }
}

/// A zero-sized type marker (e.g. the phantom on `Reference<T>`) has nothing to
/// validate.
impl<T: ?Sized> Validate for ::std::marker::PhantomData<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

/// Arbitrary embedded JSON (e.g. a `contained` resource) is not structurally
/// validated here.
impl Validate for ::serde_json::Value {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

/// A bare `String` (used by a few fields) carries no FHIR-level constraint here.
impl Validate for ::std::string::String {
    fn validate(&self) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

/// FHIR `code`: non-empty, no leading/trailing whitespace, single internal
/// spaces (regex `[^\s]+(\s[^\s]+)*`).
///
/// The rule is unchanged between R4 and R5, so both releases share it.
#[must_use]
pub fn is_valid_code(s: &str) -> bool {
    !s.is_empty()
        && s == s.trim()
        && !s.split(' ').any(str::is_empty)
        && !s.chars().any(|c| c != ' ' && c.is_whitespace())
}

/// FHIR `id`: 1..=64 chars from `[A-Za-z0-9-.]`.
#[must_use]
pub fn is_valid_id(s: &str) -> bool {
    (1..=64).contains(&s.len())
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
}

/// FHIR `uri`/`url`/`canonical`: non-empty and not surrounded by whitespace.
#[must_use]
pub fn is_valid_uri_like(s: &str) -> bool {
    !s.is_empty() && s.trim() == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_rules() {
        assert!(is_valid_code("final"));
        assert!(is_valid_code("entered in error"));
        assert!(!is_valid_code(" leading"));
        assert!(!is_valid_code("double  space"));
        assert!(!is_valid_code(""));
    }

    #[test]
    fn id_rules() {
        assert!(is_valid_id("abc-123.4"));
        assert!(!is_valid_id("bad id!"));
        assert!(!is_valid_id(&"x".repeat(65)));
    }

    #[test]
    fn uri_rules() {
        assert!(is_valid_uri_like("http://example.org"));
        assert!(!is_valid_uri_like(" http://example.org "));
        assert!(!is_valid_uri_like(""));
    }

    #[test]
    fn containers_delegate() {
        // `String` is always valid, so containers of it report nothing.
        let some: Option<String> = Some("x".to_string());
        assert!(some.validate().is_empty());
        assert!(vec!["a".to_string(), "b".to_string()].validate().is_empty());
        assert!(Box::new("a".to_string()).validate().is_empty());
        assert!(::serde_json::Value::Null.validate().is_empty());
    }
}
