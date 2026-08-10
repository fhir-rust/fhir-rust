//! Summary serialization for FHIR R4B (the `_summary=true` view).
//!
//! FHIR lets a client request only the *summary* elements of a resource — those
//! marked `isSummary` in the specification, plus mandatory elements. This module
//! prunes a serialized resource down to that set using the
//! [`meta`](crate::r4b::meta) table.
//!
//! The pruning itself does not vary by release and lives in [`::fhir_core::summary`];
//! this module binds it to the R4B element table.
//!
//! ```
//! use fhir::r4b::resources::Patient;
//! use fhir::r4b::summary::to_summary_value;
//!
//! let patient: Patient = serde_json::from_value(serde_json::json!({
//!     "resourceType": "Patient",
//!     "id": "pat-1",
//!     "gender": "male",           // isSummary
//!     "photo": [{ "title": "x" }] // NOT isSummary
//! })).unwrap();
//!
//! let summary = to_summary_value(&patient, "Patient");
//! assert!(summary.get("gender").is_some());
//! assert!(summary.get("photo").is_none());
//! ```

use ::serde::Serialize;
use ::serde_json::Value;

use crate::r4b::meta;

/// Serialize `resource` (a value of FHIR type `resource_type`) and prune it to
/// the summary view: top-level elements that are `isSummary` or mandatory, plus
/// `resourceType`. Non-summary elements — and their `_field` extension siblings
/// — are removed.
#[must_use]
pub fn to_summary_value<T: Serialize>(resource: &T, resource_type: &str) -> Value {
    ::fhir_core::summary::to_summary_value(meta::elements(), resource, resource_type)
}

/// Prune a serialized resource object in place to its summary elements.
pub fn prune_to_summary(value: &mut Value, resource_type: &str) {
    ::fhir_core::summary::prune_to_summary(meta::elements(), value, resource_type);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4b::resources::Patient;

    #[test]
    fn patient_summary_keeps_summary_drops_the_rest() {
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient",
            "id": "p1",
            "gender": "male",
            "birthDate": "1970-01-01",
            "name": [{ "family": "Chalmers" }],
            "photo": [{ "title": "portrait" }],
            "maritalStatus": { "text": "married" },
            "communication": [{ "language": { "text": "en" } }]
        }))
        .unwrap();

        let s = to_summary_value(&patient, "Patient");
        // Summary elements are kept.
        for k in ["resourceType", "id", "gender", "birthDate", "name"] {
            assert!(s.get(k).is_some(), "expected {k} in summary");
        }
        // Non-summary elements are dropped.
        for k in ["photo", "maritalStatus", "communication"] {
            assert!(s.get(k).is_none(), "expected {k} absent from summary");
        }
    }

    #[test]
    fn choice_element_summary() {
        // Patient.deceased[x] is isSummary; multipleBirth[x] is not.
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient",
            "deceasedBoolean": true,
            "multipleBirthInteger": 2
        }))
        .unwrap();
        let s = to_summary_value(&patient, "Patient");
        assert!(s.get("deceasedBoolean").is_some());
        assert!(s.get("multipleBirthInteger").is_none());
    }
}
