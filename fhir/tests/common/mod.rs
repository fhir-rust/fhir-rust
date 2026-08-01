//! Shared machinery for the per-release official-example round-trip tests.
//!
//! Round-tripping is the same check in every FHIR release — parse the official
//! JSON, deserialize into that release's polymorphic `Resource`, re-serialize,
//! and compare — so the harness lives here and each release's test file only
//! supplies its own `Resource` type and example directory.
//!
//! `serde_json::Value` equality is order-independent for objects, so a passing
//! round-trip means the model preserves every field and value in semantic
//! terms: nothing is silently dropped or altered.

#![allow(dead_code)] // Each test binary uses a subset of this module.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

/// Deserialize a resource into a release's `Resource` enum and serialize it
/// back, or describe why that failed.
pub type RoundTrip = fn(Value) -> Result<Value, String>;

/// Run every file in `dir` through `parse`, asserting all of them round-trip.
///
/// Used for the curated subsets, which are chosen to pass today and therefore
/// guard against regressions.
pub fn assert_all_roundtrip(dir: &Path, parse: RoundTrip, label: &str) {
    let files = json_files(dir);
    assert!(
        !files.is_empty(),
        "no curated {label} example files found in {}",
        dir.display()
    );

    let mut failures = Vec::new();
    let mut checked = 0usize;
    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        match roundtrip_file(file, parse) {
            Outcome::Ok => checked += 1,
            Outcome::NotAResource => {}
            Outcome::Deserialize(msg) => {
                failures.push(format!("{name}: deserialize failed: {msg}"));
            }
            Outcome::Mismatch(m) => {
                failures.push(format!("{name}: mismatch:\n    {}", m.diffs.join("\n    ")));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} curated {label} examples failed round-trip:\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
    assert!(
        checked > 0,
        "no {label} resources were actually round-tripped"
    );
}

/// A *class* of known failure: many examples failing for one understood
/// reason (spec R13.2).
///
/// HL7 publishes some examples that do not satisfy their own specification —
/// hundreds of generated `*-questionnaire.json` files omit
/// `Questionnaire.item.linkId`, which every release makes `1..1`. Listing
/// them one by one would be noise, and *relaxing the model to accept them*
/// would be worse than noise: a clinical data model that accepts resources
/// missing required elements is not validating anything.
///
/// So the class is recorded with its exact count. The count is asserted, so
/// the number cannot grow silently, and cannot shrink without someone
/// updating it and saying why.
pub struct KnownFailureClass {
    /// Substring identifying the failure, e.g. "missing field `linkId`".
    pub message: &'static str,
    /// Exactly how many examples fail this way today.
    pub count: usize,
    /// Why this is the examples' fault rather than the model's.
    pub reason: &'static str,
}

/// One example the model is known not to round-trip, and why (spec R13.2).
///
/// An allowlist rather than a silently lower bar: each entry names a file and
/// states the reason, the suite fails if an allowed file starts passing, and
/// the length is asserted — so this list can only shrink by intention.
pub struct KnownFailure {
    /// The example's file name.
    pub file: &'static str,
    /// Why it does not round-trip. If the reason is "the model is wrong",
    /// this is a bug with a note attached, not an exemption.
    pub reason: &'static str,
}

/// Run the complete official example set as a **gate**: every file must
/// round-trip except those on the allowlist, and no allowlisted file may
/// quietly start passing.
///
/// The previous behaviour — `#[ignore]` plus an uncommitted corpus — meant CI
/// exercised only a curated subset chosen to pass, which cannot detect a
/// regression in the examples it does not run (spec R13.1).
pub fn gate_all_roundtrip(dir: &Path, parse: RoundTrip, label: &str, allowed: &[KnownFailure]) {
    gate_all_roundtrip_with(dir, parse, label, allowed, &[]);
}

/// [`gate_all_roundtrip`], also tolerating whole classes of known failure.
pub fn gate_all_roundtrip_with(
    dir: &Path,
    parse: RoundTrip,
    label: &str,
    allowed: &[KnownFailure],
    classes: &[KnownFailureClass],
) {
    let files = json_files(dir);
    assert!(
        !files.is_empty(),
        "no {label} example files found in {} — run bin/fetch-examples first",
        dir.display()
    );

    let mut ok = 0usize;
    let mut skipped = 0usize;
    let mut unexpected: Vec<String> = Vec::new();
    let mut allowed_and_failing: Vec<&str> = Vec::new();
    let mut allowed_but_passing: Vec<&str> = Vec::new();
    let mut class_counts: Vec<usize> = vec![0; classes.len()];

    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        let known = allowed.iter().find(|k| k.file == name);
        match (roundtrip_file(file, parse), known) {
            (Outcome::Ok, None) => ok += 1,
            (Outcome::Ok, Some(k)) => {
                ok += 1;
                allowed_but_passing.push(k.file);
            }
            (Outcome::NotAResource, _) => skipped += 1,
            (_, Some(k)) => allowed_and_failing.push(k.file),
            (Outcome::Deserialize(msg), None) => {
                match classes.iter().position(|c| msg.contains(c.message)) {
                    Some(i) => class_counts[i] += 1,
                    None => unexpected.push(format!("{name}\n    deserialize: {msg}")),
                }
            }
            (Outcome::Mismatch(m), None) => {
                unexpected.push(format!("{name}\n    {}", m.diffs.join("\n    ")));
            }
        }
    }

    println!("\n=== Full official {label} examples ===");
    println!("scanned {}, passed {ok}, skipped {skipped}", files.len());
    println!(
        "known failures still failing: {}",
        allowed_and_failing.len()
    );

    for (c, observed) in classes.iter().zip(&class_counts) {
        println!("class \"{}\": {observed} (expected {})", c.message, c.count);
    }

    // Unexpected failures first: a regression is more urgent than a stale
    // exemption, and asserting the other way round masks it.
    assert!(
        unexpected.is_empty(),
        "{label}: {} example(s) do not round-trip and are not on the \
         allowlist:\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
    for (c, observed) in classes.iter().zip(&class_counts) {
        assert_eq!(
            *observed, c.count,
            "{label}: the known-failure class {:?} now matches {observed} \
             examples, not {}. If it grew, something regressed. If it shrank, \
             update the count and say why — a class whose number nobody \
             maintains stops being evidence. Reason on record: {}",
            c.message, c.count, c.reason
        );
    }
    assert!(
        allowed_but_passing.is_empty(),
        "{label}: these are on the known-failure allowlist but now round-trip. \
         Remove them from the list — an allowlist that outlives the bug it \
         describes hides the next one: {allowed_but_passing:?}"
    );
}

/// Run the complete official example set, printing a per-file report rather
/// than stopping at the first failure.
pub fn report_all_roundtrip(dir: &Path, parse: RoundTrip, label: &str) {
    let files = json_files(dir);
    assert!(
        !files.is_empty(),
        "no {label} example files found in {} — run bin/fetch-examples first \
         (or set the directory environment variable)",
        dir.display()
    );

    let mut ok = 0usize;
    let mut skipped = 0usize;
    let mut deserialize_failures: Vec<String> = Vec::new();
    let mut mismatches: Vec<String> = Vec::new();

    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        match roundtrip_file(file, parse) {
            Outcome::Ok => ok += 1,
            Outcome::NotAResource => skipped += 1,
            Outcome::Deserialize(msg) => deserialize_failures.push(format!("{name}\n    {msg}")),
            Outcome::Mismatch(m) => {
                mismatches.push(format!("{name}\n    {}", m.diffs.join("\n    ")));
            }
        }
    }

    let total = files.len();
    println!("\n=== Full official {label} examples round-trip ===");
    println!("files scanned:          {total}");
    println!("skipped (non-resource): {skipped}");
    println!("passed:                 {ok}");
    println!("deserialize failures:   {}", deserialize_failures.len());
    println!("mismatches:             {}", mismatches.len());

    if !deserialize_failures.is_empty() {
        println!("\n--- Deserialize failures ---");
        for f in &deserialize_failures {
            println!("{f}");
        }
    }
    if !mismatches.is_empty() {
        println!("\n--- Round-trip mismatches ---");
        for f in &mismatches {
            println!("{f}");
        }
    }

    assert!(
        deserialize_failures.is_empty() && mismatches.is_empty(),
        "{} deserialize failures + {} mismatches out of {total} files (see report above)",
        deserialize_failures.len(),
        mismatches.len(),
    );
}

/// The directory holding a release's curated, always-on example subset.
pub fn curated_dir(release: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(format!("roundtrip_examples_{release}"))
}

/// The directory holding a release's full official example set. Not committed;
/// populated by `bin/fetch-examples`. Overridable with `env_var`.
pub fn full_dir(release: &str, env_var: &str) -> PathBuf {
    if let Ok(dir) = std::env::var(env_var) {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("doc")
        .join("fhir-specifications")
        .join(release)
        .join("fhir-examples-json")
}

/// The outcome of round-tripping a single file.
pub enum Outcome {
    /// Round-tripped with an exact semantic match.
    Ok,
    /// Not a FHIR resource file (no string `resourceType`); skipped.
    NotAResource,
    /// Failed to deserialize into the `Resource` enum.
    Deserialize(String),
    /// Deserialized, but re-serialization did not match the original.
    Mismatch(Mismatch),
}

/// A structured description of the first differences between original and
/// re-serialized JSON, for reporting.
pub struct Mismatch {
    /// JSON pointer paths that differ, with a short reason. Capped for brevity.
    pub diffs: Vec<String>,
}

/// Round-trip one file through a release's `Resource` enum.
///
/// The parser is passed in because each FHIR release has its own `Resource`
/// type; everything else about the check is release-independent.
pub fn roundtrip_file(path: &Path, parse: RoundTrip) -> Outcome {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => return Outcome::Deserialize(format!("read error: {e}")),
    };
    let original: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => return Outcome::Deserialize(format!("not valid JSON: {e}")),
    };
    // Only top-level FHIR resources have a string `resourceType`.
    if original
        .get("resourceType")
        .and_then(Value::as_str)
        .is_none()
    {
        return Outcome::NotAResource;
    }
    let reserialized = match parse(original.clone()) {
        Ok(v) => v,
        Err(e) => return Outcome::Deserialize(e),
    };
    if reserialized == original {
        Outcome::Ok
    } else {
        let mut diffs = Vec::new();
        diff_json("", &original, &reserialized, &mut diffs);
        diffs.truncate(8);
        Outcome::Mismatch(Mismatch { diffs })
    }
}

/// Recursively collect the paths where `orig` and `got` differ.
fn diff_json(path: &str, orig: &Value, got: &Value, out: &mut Vec<String>) {
    if out.len() >= 8 {
        return;
    }
    match (orig, got) {
        (Value::Object(a), Value::Object(b)) => {
            let keys: BTreeMap<&String, ()> = a.keys().chain(b.keys()).map(|k| (k, ())).collect();
            for (k, ()) in keys {
                let child = format!("{path}/{k}");
                match (a.get(k), b.get(k)) {
                    (Some(av), Some(bv)) => diff_json(&child, av, bv, out),
                    (Some(_), None) => out.push(format!("{child}: dropped")),
                    (None, Some(_)) => out.push(format!("{child}: added")),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                out.push(format!("{path}: array len {} -> {}", a.len(), b.len()));
                return;
            }
            for (i, (av, bv)) in a.iter().zip(b.iter()).enumerate() {
                diff_json(&format!("{path}/{i}"), av, bv, out);
            }
        }
        _ => {
            if orig != got {
                out.push(format!("{path}: {orig} -> {got}"));
            }
        }
    }
}

/// Run `f` on a thread with a large stack.
///
/// `serde_json` deserializes by recursive descent, and the deepest official
/// examples are recursively-nested `Questionnaire`s (item within item, ~24
/// levels). As the model grows more fields per struct, each level's stack frame
/// grows too, so the default 2 MiB test-harness stack can overflow. A generous
/// stack keeps this test robust regardless of build profile or field count.
/// Panics from `f` propagate so failing assertions still fail the test.
pub fn with_large_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(f)
        .expect("spawn")
        .join()
        .expect("round-trip thread panicked");
}

/// List `*.json` files in a directory (non-recursive), sorted.
pub fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = match fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();
    files
}
