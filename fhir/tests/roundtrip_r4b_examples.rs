//! Round-trip the official FHIR R4B example resources through the data model.
//!
//! For every example `*.json` file we:
//!
//! 1. Parse the raw JSON into a [`serde_json::Value`] (the oracle).
//! 2. Deserialize that value into the polymorphic
//!    [`Resource`](fhir::r4b::resources::Resource) enum.
//! 3. Re-serialize the `Resource` back to a [`serde_json::Value`].
//! 4. Assert the re-serialized value equals the original.
//!
//! # Two entry points
//!
//! - [`roundtrip_curated_subset`] — always runs. It scans a small, curated set
//!   of diverse example files committed under
//!   `tests/data/roundtrip_examples_r4b/`. These are chosen to pass today, so
//!   this test guards against regressions.
//!
//! - [`roundtrip_full_official_examples`] — skips itself when the corpus is
//!   absent. It scans the complete official example set, which is *not*
//!   committed. Populate it first
//!   with `bin/fetch-examples r4b`, then run:
//!
//!   ```sh
//!   cargo test --features r4b --test roundtrip_r4b_examples -- --nocapture
//!   ```
//!
//!   Point it at an alternate directory with `FHIR_ROUNDTRIP_DIR_R4B`.
//!   The full run prints a per-file failure report rather than panicking on the
//!   first mismatch.

#![cfg(feature = "r4b")]

mod common;

use fhir::r4b::resources::Resource;
use serde_json::Value;

/// Deserialize into the R4B `Resource` enum and serialize straight back.
fn roundtrip(original: Value) -> Result<Value, String> {
    let resource: Resource = serde_json::from_value(original).map_err(|e| e.to_string())?;
    serde_json::to_value(&resource).map_err(|e| format!("re-serialize error: {e}"))
}

#[test]
fn roundtrip_curated_subset() {
    common::with_large_stack(|| {
        common::assert_all_roundtrip(&common::curated_dir("r4b"), roundtrip, "R4B");
    });
}

/// Examples the R4B model is known not to round-trip (spec R13.2).
///
/// Empty — and it got here by shrinking, which is the direction an
/// allowlist is allowed to move. Nine null-padded `Timing.event` examples
/// sat here for audit **F-86**/**F-87** on 2026-08-10; both were fixed the
/// same day (F-87's loud refusal in the morning, F-86's `PrimVec` value
/// arrays in the afternoon) and the gate itself demanded their removal
/// once they round-tripped.
const KNOWN_FAILURES: &[common::KnownFailure] = &[];

/// Classes of failure caused by HL7's own non-conformant examples (R13.2).
/// The specification is explicit that these elements are required; the model
/// is right and the published example is not.
const KNOWN_CLASSES: &[common::KnownFailureClass] = &[
    // R4's corpus needs a `linkId` class here (188 generated questionnaire
    // examples omit it); measured on R4B, that count is **zero** — HL7 fixed
    // the generated questionnaires for 4.3.0. The class is dropped, not
    // copied: an allowlist can only shrink.
    common::KnownFailureClass {
        message: "missing field `base`",
        count: 10,
        reason: "SearchParameter.base is 1..* in R4B; these examples omit it",
    },
    common::KnownFailureClass {
        message: "missing field `status`",
        count: 2,
        reason: "CodeSystem.status and ValueSet.status are 1..1; HL7's \
                 codesystem-catalogType.json and valueset-catalogType.json \
                 omit it",
    },
];

/// The corpus gate (spec R13.1). Skips itself when the corpus is absent, so a
/// developer without the ~190 MB download still gets a green local run — but
/// CI fetches it, which is where "the full corpus round-trips" stops being a
/// claim and starts being a check.
#[test]
fn roundtrip_full_official_examples() {
    let dir = common::full_dir("r4b", "FHIR_ROUNDTRIP_DIR_R4B");
    if !dir.exists() {
        eprintln!(
            "skipping: no corpus at {} — run bin/fetch-examples r4b",
            dir.display()
        );
        return;
    }
    common::with_large_stack(move || {
        common::gate_all_roundtrip_with(&dir, roundtrip, "R4B", KNOWN_FAILURES, KNOWN_CLASSES);
    });
}
