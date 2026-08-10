//! Element metadata extracted from the FHIR R4B specification.
//!
//! This module is a queryable, compile-time table of per-element facts drawn
//! from the official `ElementDefinition`s: cardinality, coded-value bindings,
//! the allowed types of `value[x]` choice elements, reference target profiles,
//! and summary membership. It is the foundation the type-safety and validation
//! layers build on (choice enums, required-binding checks, typed references,
//! summary serialization).
//!
//! The table's types are shared with every other release and live in
//! [`::fhir_core::meta`]; the data is **generated** from the specification JSON by
//! [`crate::codegen::meta_gen`] into [`mod@self::generated`]. Do not hand-edit
//! it — regenerate with `cargo run -- r4b`.
//!
//! Elements are keyed by their full FHIR path, e.g. `"Patient.gender"`,
//! `"Observation.value[x]"`, `"Patient.contact.name"`.
//!
//! ```
//! use fhir::r4b::meta;
//!
//! // Patient.gender is bound, with `required` strength, to administrative-gender.
//! let gender = meta::element("Patient.gender").unwrap();
//! let binding = gender.binding.unwrap();
//! assert_eq!(binding.strength, meta::BindingStrength::Required);
//! assert!(binding.value_set.unwrap().contains("administrative-gender"));
//!
//! // Observation.value[x] is a choice element with several allowed types.
//! let value = meta::element("Observation.value[x]").unwrap();
//! assert!(value.is_choice());
//! assert!(value.type_codes().any(|c| c == "Quantity"));
//! ```

mod generated;

pub use ::fhir_core::meta::{BindingMeta, BindingStrength, ElementMeta, TypeRef};

/// Look up the metadata for an R4B element by its full FHIR path.
///
/// Returns `None` if the path is not a known element.
#[must_use]
pub fn element(path: &str) -> Option<&'static ElementMeta> {
    ::fhir_core::meta::find(generated::ELEMENTS, path)
}

/// Every extracted R4B element, sorted by path.
#[must_use]
pub fn elements() -> &'static [ElementMeta] {
    generated::ELEMENTS
}

/// Every element belonging to a given resource or datatype, i.e. whose path is
/// `"<type_name>.…"` (including nested backbone paths).
pub fn elements_of(type_name: &str) -> impl Iterator<Item = &'static ElementMeta> {
    let prefix = format!("{type_name}.");
    generated::ELEMENTS
        .iter()
        .filter(move |e| e.path.starts_with(&prefix))
}

/// The FHIR path prefix a generated Rust struct corresponds to, e.g.
/// `"AppointmentParticipant"` → `"Appointment.participant"`, `"Patient"` →
/// `"Patient"`. Returns `None` for a name that is not a FHIR type/backbone.
#[must_use]
pub fn struct_prefix(struct_name: &str) -> Option<&'static str> {
    use std::collections::HashMap;
    use std::sync::LazyLock;

    static PREFIXES: LazyLock<HashMap<String, &'static str>> =
        LazyLock::new(|| ::fhir_core::meta::struct_prefixes(generated::ELEMENTS));

    PREFIXES.get(struct_name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_non_empty_and_sorted() {
        let all = elements();
        assert!(
            all.len() > 5000,
            "expected thousands of elements, got {}",
            all.len()
        );
        assert!(
            all.windows(2).all(|w| w[0].path <= w[1].path),
            "ELEMENTS must be sorted by path for binary search"
        );
    }

    #[test]
    fn patient_gender_binding() {
        let gender = element("Patient.gender").expect("Patient.gender");
        let binding = gender.binding.expect("gender has a binding");
        assert_eq!(binding.strength, BindingStrength::Required);
        assert!(
            binding.value_set.unwrap().contains("administrative-gender"),
            "value_set was {:?}",
            binding.value_set
        );
    }

    // R4B's Observation.value[x] allows 11 types; R5 later added Attachment and
    // Reference, which is exactly the kind of difference the two models exist
    // to keep apart.
    #[test]
    fn observation_value_choice_types() {
        let value = element("Observation.value[x]").expect("Observation.value[x]");
        assert!(value.is_choice());
        assert_eq!(value.types.len(), 11, "R4B Observation.value[x] type count");
        for code in ["Quantity", "CodeableConcept", "string", "Range", "Period"] {
            assert!(value.type_codes().any(|c| c == code), "missing type {code}");
        }
        assert!(
            !value.type_codes().any(|c| c == "Attachment"),
            "Attachment is R5-only"
        );
    }

    #[test]
    fn observation_subject_targets() {
        let subject = element("Observation.subject").expect("Observation.subject");
        let targets: Vec<&str> = subject
            .types
            .iter()
            .flat_map(TypeRef::target_names)
            .collect();
        for want in ["Patient", "Group", "Device", "Location"] {
            assert!(
                targets.contains(&want),
                "subject should target {want}: {targets:?}"
            );
        }
    }

    #[test]
    fn cardinality_helpers() {
        let status = element("Observation.status").expect("Observation.status");
        assert!(status.is_required());
        assert!(!status.is_multiple());
        let identifier = element("Observation.identifier").expect("Observation.identifier");
        assert!(!identifier.is_required());
        assert!(identifier.is_multiple());
    }

    #[test]
    fn elements_of_a_type() {
        let count = elements_of("Patient").count();
        assert!(count > 20, "Patient should have many elements, got {count}");
        assert!(elements_of("Patient").all(|e| e.path.starts_with("Patient.")));
    }
}
