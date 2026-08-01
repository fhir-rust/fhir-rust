//! Cross-release conversion against the real generated element tables.
//!
//! `fhir-core`'s own unit tests exercise the conversion engine against small
//! hand-built tables, because that crate cannot see a release model. These are
//! the tests that matter: they run the engine over R3, R4 and R5 as generated
//! from the specifications, and over the committed official examples.
//!
//! The strongest property here is not "the report says the right thing" but
//! **the converted document parses in the target release**. A conversion that
//! produced JSON the target's model rejects would be useless however good its
//! bookkeeping was.
//!
//! Needs several releases at once, so it runs under
//! `cargo test --features "r3 r4"` (the `all-releases` CI job).

#![cfg(all(feature = "r3", feature = "r4"))]

mod common;

use std::path::Path;

use fhir::convert::{self, LossKind};
use fhir::r3::R3;
use fhir::r4::R4;
use fhir::r5::R5;
use serde_json::{json, Value};

/// Every JSON file in a committed curated example directory.
fn examples(dir: &str) -> Vec<Value> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(dir);
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&path).expect("example dir") {
        let file = entry.expect("dir entry").path();
        if file.extension().is_some_and(|e| e == "json") {
            let text = std::fs::read_to_string(&file).expect("read example");
            out.push(serde_json::from_str(&text).expect("parse example"));
        }
    }
    assert!(!out.is_empty(), "no examples found in {}", path.display());
    out
}

/// The `resourceType` of a document, for failure messages.
fn type_of(value: &Value) -> &str {
    value.get("resourceType").and_then(Value::as_str).unwrap_or("?")
}

#[test]
fn an_element_r4_removed_is_dropped_and_named() {
    // `Patient.animal` is in R3 and gone in R4.
    let r3_patient = json!({
        "resourceType": "Patient",
        "active": true,
        "animal": { "species": { "text": "canine" } },
    });

    let out = convert::between::<R3, R4>(&r3_patient);

    assert_eq!(out.value["active"], true, "the rest of the resource survives");
    assert!(out.value.get("animal").is_none(), "animal has no home in R4");
    let loss = out
        .report
        .of_kind(LossKind::ElementRemoved)
        .find(|l| l.path == "Patient.animal")
        .unwrap_or_else(|| panic!("animal not reported; got:\n{}", out.report));
    assert!(loss.kind.discards_data());
}

#[test]
fn a_choice_variant_the_target_does_not_allow_is_dropped_and_named() {
    // R3's Observation.value[x] admits Attachment; R4's does not.
    let r3_obs = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": { "text": "weight" },
        "valueAttachment": { "title": "a scan" },
    });

    let out = convert::between::<R3, R4>(&r3_obs);

    assert!(out.value.get("valueAttachment").is_none());
    assert_eq!(
        out.report
            .of_kind(LossKind::ChoiceVariantUnsupported)
            .filter(|l| l.path == "Observation.valueAttachment")
            .count(),
        1,
        "expected one unsupported-variant loss; got:\n{}",
        out.report
    );
    // A variant both releases share crosses untouched.
    let shared = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": { "text": "weight" },
        "valueQuantity": { "value": 70, "unit": "kg" },
    });
    let out = convert::between::<R3, R4>(&shared);
    assert_eq!(out.value["valueQuantity"]["unit"], "kg");
}

#[test]
fn converting_a_release_to_itself_never_discards_data() {
    // The identity conversion is the cleanest available oracle: whatever the
    // tables say, a release can always represent its own documents. Anything
    // dropped here is a bug in the walk, not a difference between releases.
    for example in examples("roundtrip_examples_r4") {
        let out = convert::between::<R4, R4>(&example);
        assert!(
            !out.report.discarded_data(),
            "R4 -> R4 discarded data from a {}:\n{}",
            type_of(&example),
            out.report
        );
        assert_eq!(
            out.value,
            example,
            "R4 -> R4 altered a {}",
            type_of(&example)
        );
    }
}

/// The property that makes the loss report worth having.
///
/// A conversion may legitimately produce a document the target rejects: if the
/// target requires an element the source never carried, there is nothing honest
/// to put there, and inventing a value is the one thing this layer must never
/// do. What must never happen is that the document fails to parse and the
/// report did not see it coming.
///
/// So for every example: either it parses, or the report names a
/// [`LossKind::RequiredMissing`] for the very field the target complained
/// about. Anything else is a silent loss, which is the failure mode the whole
/// module exists to prevent.
fn assert_report_predicts_parse_failures<Parse>(dir: &str, convert: impl Fn(&Value) -> fhir::convert::Converted, parse: Parse, label: &str)
where
    Parse: Fn(Value) -> Result<(), String>,
{
    let mut checked = 0usize;
    let mut predicted = 0usize;
    let mut silent = Vec::new();

    for example in examples(dir) {
        let out = convert(&example);
        if out.value.is_null() {
            continue; // A resource type the target lacks; reported already.
        }
        checked += 1;
        let Err(e) = parse(out.value.clone()) else {
            continue;
        };

        // serde names the field it wanted: "missing field `kind`".
        let field = e
            .split_once("missing field `")
            .and_then(|(_, rest)| rest.split_once('`'))
            .map(|(name, _)| name.to_string());

        let foreseen = match &field {
            Some(name) => out
                .report
                .of_kind(LossKind::RequiredMissing)
                .any(|l| l.path.rsplit('.').next() == Some(name.as_str())),
            // Not a missing-field error: a code the target's value set rejects
            // is the other way a converted document can fail, and the report
            // flags those as a changed binding.
            None => out.report.of_kind(LossKind::BindingChanged).next().is_some(),
        };

        if foreseen {
            predicted += 1;
        } else {
            silent.push(format!(
                "{} ({}): {e}\nreport was:\n{}",
                type_of(&example),
                dir,
                out.report
            ));
        }
    }

    assert!(checked > 0, "no {label} examples were converted");
    assert!(
        silent.is_empty(),
        "{} converted {label} documents failed to parse with nothing in the report to explain it:\n\n{}",
        silent.len(),
        silent.join("\n\n")
    );
    // Guard the guard: if nothing ever failed to parse, this test proves
    // nothing about the report and should not be trusted as coverage.
    assert!(
        predicted > 0,
        "no {label} document exercised the required-missing path; \
         the corpus or the models changed and this test is now vacuous"
    );
}

#[test]
fn the_report_predicts_every_r4_to_r5_parse_failure() {
    assert_report_predicts_parse_failures(
        "roundtrip_examples_r4",
        convert::between::<R4, R5>,
        |v| {
            serde_json::from_value::<fhir::r5::resources::Resource>(v)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        "R4 -> R5",
    );
}

#[test]
fn the_report_predicts_every_r3_to_r4_parse_failure() {
    assert_report_predicts_parse_failures(
        "roundtrip_examples_r3",
        convert::between::<R3, R4>,
        |v| {
            serde_json::from_value::<fhir::r4::resources::Resource>(v)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        "R3 -> R4",
    );
}

#[test]
fn strict_mode_refuses_exactly_what_the_report_objects_to() {
    // Every committed R4 example, through strict R4 -> R5. The two must agree
    // perfectly: strict succeeds precisely when the report is empty, so a
    // caller choosing to refuse never refuses something the report called
    // clean, and never accepts something it did not.
    let mut accepted = 0usize;
    let mut refused = 0usize;

    for example in examples("roundtrip_examples_r4") {
        let report_is_clean = convert::between::<R4, R5>(&example).report.is_lossless();
        match convert::strict::<R4, R5>(&example) {
            Ok(value) => {
                accepted += 1;
                assert!(report_is_clean, "strict accepted a {} the report faulted", type_of(&example));
                // Anything strict accepts must be usable in the target.
                serde_json::from_value::<fhir::r5::resources::Resource>(value)
                    .unwrap_or_else(|e| panic!("strict accepted an unparsable {}: {e}", type_of(&example)));
            }
            Err(report) => {
                refused += 1;
                assert!(!report_is_clean, "strict refused a clean {}", type_of(&example));
                assert!(!report.is_lossless());
            }
        }
    }

    assert!(accepted > 0 && refused > 0, "expected both outcomes in the corpus");
}

#[test]
fn a_resource_the_target_release_lacks_is_reported_not_silently_emptied() {
    // R5 renamed R3's `DeviceComponent`; nothing in R5 answers to that name.
    let r3 = json!({ "resourceType": "DeviceComponent", "type": { "text": "x" } });
    let out = convert::between::<R3, R5>(&r3);

    assert!(out.value.is_null(), "an unconvertible resource is null, not {{}}");
    assert_eq!(out.report.of_kind(LossKind::ResourceRemoved).count(), 1);
}
