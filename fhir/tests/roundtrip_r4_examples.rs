//! Round-trip the official FHIR R4 example resources through the data model.
//!
//! For every example `*.json` file we:
//!
//! 1. Parse the raw JSON into a [`serde_json::Value`] (the oracle).
//! 2. Deserialize that value into the polymorphic
//!    [`Resource`](fhir::r4::resources::Resource) enum.
//! 3. Re-serialize the `Resource` back to a [`serde_json::Value`].
//! 4. Assert the re-serialized value equals the original.
//!
//! # Two entry points
//!
//! - [`roundtrip_curated_subset`] — always runs. It scans a small, curated set
//!   of diverse example files committed under
//!   `tests/data/roundtrip_examples_r4/`. These are chosen to pass today, so
//!   this test guards against regressions.
//!
//! - [`roundtrip_full_official_examples`] — skips itself when the corpus is
//!   absent. It scans the complete official example set, which is *not*
//!   committed. Populate it first
//!   with `bin/fetch-examples r4`, then run:
//!
//!   ```sh
//!   cargo test --features r4 --test roundtrip_r4_examples -- --nocapture
//!   ```
//!
//!   Point it at an alternate directory with `FHIR_ROUNDTRIP_DIR_R4`.
//!   The full run prints a per-file failure report rather than panicking on the
//!   first mismatch.

#![cfg(feature = "r4")]

mod common;

use fhir::r4::resources::Resource;
use serde_json::Value;

/// Deserialize into the R4 `Resource` enum and serialize straight back.
fn roundtrip(original: Value) -> Result<Value, String> {
    let resource: Resource = serde_json::from_value(original).map_err(|e| e.to_string())?;
    serde_json::to_value(&resource).map_err(|e| format!("re-serialize error: {e}"))
}

#[test]
fn roundtrip_curated_subset() {
    common::with_large_stack(|| {
        common::assert_all_roundtrip(&common::curated_dir("r4"), roundtrip, "R4");
    });
}

/// Examples the R4 model is known not to round-trip (spec R13.2). Empty
/// until the full corpus has been run against this release in CI; an entry
/// added here must state why.
const KNOWN_FAILURES: &[common::KnownFailure] = &[];

/// Classes of failure caused by HL7's own non-conformant examples (R13.2).
/// The specification is explicit that these elements are required; the model
/// is right and the published example is not.
const KNOWN_CLASSES: &[common::KnownFailureClass] = &[
    common::KnownFailureClass {
        message: "missing field `linkId`",
        count: 188,
        reason: "the generated *-questionnaire.json examples omit \
                 Questionnaire.item.linkId, which R4 defines as 1..1",
    },
    common::KnownFailureClass {
        message: "missing field `base`",
        count: 10,
        reason: "SearchParameter.base is 1..* in R4; these examples omit it",
    },
];

/// The corpus gate (spec R13.1). Skips itself when the corpus is absent, so a
/// developer without the ~190 MB download still gets a green local run — but
/// CI fetches it, which is where "the full corpus round-trips" stops being a
/// claim and starts being a check.
#[test]
fn roundtrip_full_official_examples() {
    let dir = common::full_dir("r4", "FHIR_ROUNDTRIP_DIR_R4");
    if !dir.exists() {
        eprintln!(
            "skipping: no corpus at {} — run bin/fetch-examples r4",
            dir.display()
        );
        return;
    }
    common::with_large_stack(move || {
        common::gate_all_roundtrip_with(&dir, roundtrip, "R4", KNOWN_FAILURES, KNOWN_CLASSES);
    });
}
