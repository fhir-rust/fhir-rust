//! Round-trip real DSTU2 resources through the R2 data model.
//!
//! Like [`roundtrip_r1_spec`](../roundtrip_r1_spec/index.html), this uses the
//! resources embedded in the committed definition bundles rather than a
//! fetched example set, so it runs everywhere with no network.
//!
//! It exists because of a specific defect it now guards against. DSTU2
//! expresses a recursive element with `nameReference` — naming the element it
//! repeats — where R3 onwards use `contentReference` with a path. The
//! generator understood only the modern form, so 92 DSTU2 elements had no
//! type and were dropped, among them `Bundle.entry.link`,
//! `ValueSet.codeSystem.concept.concept` and `ValueSet.expansion.contains
//! .contains`. Nothing failed: the model simply discarded every nested
//! concept and every entry link on the way through. Only a round-trip against
//! real published data makes that visible.

#![cfg(feature = "r2")]

mod common;

use fhir::r2::resources::Resource;
use serde_json::Value;

/// The DSTU2 definition bundles, which double as this release's corpus.
const BUNDLES: &[&str] = &[
    "profiles-types.json",
    "profiles-resources.json",
    "profiles-others.json",
    "valuesets.json",
    "search-parameters.json",
];

/// Remove DSTU2's `fhir_comments` everywhere.
///
/// DSTU2 JSON could carry the authoring comments from the XML rendering in a
/// `fhir_comments` array beside any element; R3 dropped the mechanism. The
/// model does not represent them, so they are stripped from the oracle rather
/// than reported as loss: they are commentary about the specification's own
/// examples and carry no clinical content. 91 of the published DSTU2
/// ValueSets have them, and comparing without stripping hides every real
/// difference behind that noise.
fn strip_fhir_comments(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|k, _| k != "fhir_comments");
            // A `_field` sibling that held nothing but comments is now empty
            // and would differ from an absent one.
            for v in map.values_mut() {
                strip_fhir_comments(v);
            }
            map.retain(|k, v| {
                !(k.starts_with('_') && matches!(v, Value::Object(m) if m.is_empty()))
            });
        }
        Value::Array(items) => {
            for v in items.iter_mut() {
                strip_fhir_comments(v);
            }
        }
        _ => {}
    }
}

/// Does this resource nest an element inside another of the same name?
///
/// That is what `nameReference` expresses — `concept` inside `concept`,
/// `contains` inside `contains`, `part` inside `part`. Detecting it
/// structurally rather than by field name keeps the check honest if the
/// corpus changes.
fn has_recursive_element(value: &Value) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, child)| {
            let repeats = child.as_array().is_some_and(|items| {
                items
                    .iter()
                    .any(|item| item.get(key).is_some_and(|v| !v.is_null()))
            });
            repeats || has_recursive_element(child)
        }),
        Value::Array(items) => items.iter().any(has_recursive_element),
        _ => false,
    }
}

fn roundtrip(original: Value) -> Result<Value, String> {
    let resource: Resource = serde_json::from_value(original).map_err(|e| e.to_string())?;
    serde_json::to_value(&resource).map_err(|e| format!("re-serialize error: {e}"))
}

#[test]
fn roundtrip_dstu2_specification_resources() {
    common::with_large_stack(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("doc/fhir-specifications/r2/fhir-definitions-json");

        let mut total = 0usize;
        let mut nested = 0usize;
        let mut unparsable = 0usize;
        let mut failures: Vec<String> = Vec::new();

        for name in BUNDLES {
            let path = root.join(name);
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let bundle: Value = serde_json::from_str(&text)
                .unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));

            let entries = bundle
                .get("entry")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or_default();

            for entry in entries {
                let Some(original) = entry.get("resource") else {
                    continue;
                };
                total += 1;
                // Count the resources that actually exercise the recursion, so
                // this cannot quietly pass on a corpus that lost them.
                if has_recursive_element(original) {
                    nested += 1;
                }
                let label = format!(
                    "{name}: {} {}",
                    original
                        .get("resourceType")
                        .and_then(Value::as_str)
                        .unwrap_or("?"),
                    original.get("id").and_then(Value::as_str).unwrap_or("-"),
                );
                let mut expected = original.clone();
                strip_fhir_comments(&mut expected);
                match roundtrip(expected.clone()) {
                    Err(e) if e.starts_with("missing field `code`") => unparsable += 1,
                    Err(e) => failures.push(format!("{label}: {e}")),
                    Ok(back) if back != expected => {
                        failures.push(format!("{label}: round-trip changed the value"));
                    }
                    Ok(_) => {}
                }
            }
        }

        assert!(
            total > 500,
            "expected the DSTU2 corpus, found {total} resources"
        );
        // Without this the test would still pass if the corpus stopped
        // containing any recursive element — the exact thing it guards.
        // Exact, for the same reason as the total: ">0" would still pass
        // with a single recursive resource left, and the whole point of this
        // test is the 92 elements that recursion reaches.
        assert_eq!(
            nested, 14,
            "expected 14 DSTU2 resources exercising nameReference recursion, \
             found {nested}. Fewer means the corpus lost the very resources \
             this test exists to check."
        );
        // HL7's own DSTU2 primitive definitions omit `ElementDefinition
        // .type.code`, which DSTU2 declares 1..1 — the model is right and the
        // published file is not. Pinned so the number cannot grow unnoticed.
        assert_eq!(
            unparsable, 17,
            "expected exactly the 17 known non-conformant DSTU2 primitive \
             definitions to fail parsing"
        );
        assert!(
            failures.is_empty(),
            "{} of {total} DSTU2 resources failed to round-trip:\n{}",
            failures.len(),
            failures
                .iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    });
}
