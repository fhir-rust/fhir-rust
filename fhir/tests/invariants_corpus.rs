//! The enforced invariants must not fire on documents that are actually valid.
//!
//! The unit tests in each release's `validate` module prove the checks fire
//! when a rule is broken. They cannot prove the converse — that the checks stay
//! quiet on real data — and that is the half a too-eager check gets wrong.
//! `qty-3` keyed on field names alone would flag every `Coding` in the corpus;
//! `drq-1` with its exclusive-or inverted would flag every well-formed filter.
//!
//! So: validate the committed official examples and require silence. HL7's own
//! examples are the closest thing available to a corpus of documents that are
//! supposed to satisfy the specification's own constraints.
//!
//! This is deliberately *not* a test that the corpus is invariant-clean in
//! general — several unenforced rules it would fail are listed in spec 10. It
//! asserts only that the rules this crate *does* enforce are satisfied by it.

#![cfg(feature = "r5")]

use std::collections::BTreeMap;
use std::path::Path;

use fhir::r5::resources::Resource;
use fhir::r5::validate::Validate;

/// The invariant keys `invariant_stmts` in `fhir-derive-macros` enforces.
const ENFORCED: &[&str] = &[
    "ele-1", "ext-1", "dom-2", "dom-4", "qty-3", "inv-1", "att-1", "drq-1",
];

#[test]
fn the_enforced_invariants_are_silent_on_the_official_examples() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/roundtrip_examples_r5");
    let mut violations: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut checked = 0usize;

    for entry in std::fs::read_dir(&dir).expect("example directory") {
        let file = entry.expect("dir entry").path();
        if file.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("read example");
        // A file this release cannot parse is the round-trip suite's business,
        // not this one's.
        let Ok(resource) = serde_json::from_str::<Resource>(&text) else {
            continue;
        };
        checked += 1;

        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        for issue in resource.validate() {
            if let Some(key) = ENFORCED.iter().find(|k| issue.message.contains(*k)) {
                violations
                    .entry((*key).to_string())
                    .or_default()
                    .push(format!("{name}: {} — {}", issue.path, issue.message));
            }
        }
    }

    assert!(checked > 0, "no examples were validated");
    assert!(
        violations.is_empty(),
        "enforced invariants fired on documents HL7 publishes as valid, \
         which means the check is wrong or too broad:\n{}",
        violations
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
