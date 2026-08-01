//! Summary serialization (the FHIR `_summary=true` view).
//!
//! FHIR lets a client request only the *summary* elements of a resource — those
//! marked `isSummary` in the specification, plus mandatory elements. Pruning to
//! that set is the same operation in every release; only the element table
//! differs, so the logic lives here and each release passes its own table. Use
//! [`r4::summary`](crate::r4::summary) or [`r5::summary`](crate::r5::summary)
//! rather than calling this directly.

use ::serde::Serialize;
use ::serde_json::Value;

use crate::meta::ElementMeta;

/// Serialize `resource` (a value of FHIR type `resource_type`) and prune it to
/// the summary view: top-level elements that are `isSummary` or mandatory, plus
/// `resourceType`. Non-summary elements — and their `_field` extension siblings
/// — are removed.
#[must_use]
pub fn to_summary_value<T: Serialize>(
    table: &'static [ElementMeta],
    resource: &T,
    resource_type: &str,
) -> Value {
    let mut value = ::serde_json::to_value(resource).unwrap_or(Value::Null);
    prune_to_summary(table, &mut value, resource_type);
    // A bare resource struct does not serialize `resourceType` (the `Resource`
    // enum adds it); inject it so the summary is a valid resource.
    if let Value::Object(map) = &mut value {
        map.entry("resourceType".to_string())
            .or_insert_with(|| Value::String(resource_type.to_string()));
    }
    value
}

/// Prune a serialized resource object in place to its summary elements.
pub fn prune_to_summary(table: &'static [ElementMeta], value: &mut Value, resource_type: &str) {
    let Value::Object(map) = value else { return };
    let bases = summary_bases(table, resource_type);
    map.retain(|key, _| keep_in_summary(key, &bases));
}

/// A direct child element: (camelCase base name, is-summary-or-mandatory, is-choice).
fn summary_bases(table: &'static [ElementMeta], resource_type: &str) -> Vec<(String, bool, bool)> {
    let prefix = format!("{resource_type}.");
    let prefix_len = prefix.len();
    table
        .iter()
        .filter(|e| e.path.starts_with(&prefix))
        .filter(|e| !e.path[prefix_len..].contains('.')) // direct children only
        .map(|e| {
            let leaf = &e.path[prefix_len..];
            let is_choice = leaf.ends_with("[x]");
            let base = leaf.trim_end_matches("[x]").to_string();
            (base, e.is_summary || e.min >= 1, is_choice)
        })
        .collect()
}

/// Whether a serialized JSON key survives pruning.
fn keep_in_summary(key: &str, bases: &[(String, bool, bool)]) -> bool {
    if key == "resourceType" {
        return true;
    }
    // A `_field` sibling follows its base element.
    let bare = key.strip_prefix('_').unwrap_or(key);
    bases.iter().any(|(base, is_summary, is_choice)| {
        *is_summary
            && (bare == base
                || (*is_choice
                    && bare.starts_with(base.as_str())
                    && bare[base.len()..]
                        .chars()
                        .next()
                        .is_some_and(char::is_uppercase)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::TypeRef;

    static TABLE: &[ElementMeta] = &[
        ElementMeta {
            path: "Patient.deceased[x]",
            min: 0,
            max: "1",
            is_summary: true,
            binding: None,
            types: &[TypeRef {
                code: "boolean",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.gender",
            min: 0,
            max: "1",
            is_summary: true,
            binding: None,
            types: &[TypeRef {
                code: "code",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.multipleBirth[x]",
            min: 0,
            max: "1",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "integer",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.name.family",
            min: 0,
            max: "1",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "string",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.photo",
            min: 0,
            max: "*",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "Attachment",
                target_profiles: &[],
            }],
            content_reference: None,
        },
    ];

    #[test]
    fn keeps_summary_elements_and_drops_the_rest() {
        let mut value = ::serde_json::json!({
            "gender": "male",
            "photo": [{ "title": "portrait" }]
        });
        prune_to_summary(TABLE, &mut value, "Patient");
        assert!(value.get("gender").is_some());
        assert!(value.get("photo").is_none());
    }

    #[test]
    fn choice_elements_match_every_typed_key() {
        let mut value = ::serde_json::json!({
            "deceasedBoolean": true,
            "multipleBirthInteger": 2
        });
        prune_to_summary(TABLE, &mut value, "Patient");
        assert!(value.get("deceasedBoolean").is_some());
        assert!(value.get("multipleBirthInteger").is_none());
    }

    #[test]
    fn extension_siblings_follow_their_element() {
        let mut value = ::serde_json::json!({
            "gender": "male",
            "_gender": { "id": "g1" },
            "photo": [],
            "_photo": [null]
        });
        prune_to_summary(TABLE, &mut value, "Patient");
        assert!(value.get("_gender").is_some());
        assert!(value.get("_photo").is_none());
    }

    #[test]
    fn resource_type_is_always_kept_and_injected() {
        #[derive(::serde::Serialize)]
        struct Bare {
            gender: &'static str,
        }
        let value = to_summary_value(TABLE, &Bare { gender: "male" }, "Patient");
        assert_eq!(value.get("resourceType").unwrap(), "Patient");
    }
}
