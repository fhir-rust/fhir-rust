//! Converted `Coded<E>` fields resolve to `Known`, not `Unknown`
//! (tasks.md T35).
//!
//! `Coded<E>` deliberately falls back to `Unknown(String)` so that a server
//! sending a code outside the value set does not break deserialization. That
//! is the right design and it is also a blind spot: pair a field with the
//! *wrong* code system and every real value lands in `Unknown`, round-trips
//! perfectly, and looks correct. The example corpus cannot tell the
//! difference, because `Unknown` preserves the string faithfully.
//!
//! So each field converted from `types::Code` to `Coded<E>` gets a real code
//! from its bound value set here, and must come back `Known`.
//!
//! This test earned its place immediately: it rejected three of the thirteen
//! conversions. `TaskIntent`, `TransportIntent` and `DetectedissueStatus` are
//! generated with a *single* `Unknown` variant, because the generator cannot
//! resolve those value sets from the bundled packages. Typing a field as
//! `Coded<E>` over such an enum is worse than leaving it a plain `Code`: every
//! real value becomes `Unknown(..)` while the signature claims type safety.
//! Those three stayed as `types::Code`; see tasks.md T35.

#![cfg(feature = "r5")]

use fhir::r5::coded::Coded;

/// Assert that a genuine code from the element's value set is recognised.
///
/// The serialized form is checked for the code rather than compared whole: a
/// resource struct does not emit `resourceType` (the polymorphic `Resource`
/// enum adds it), so an exact comparison would test that quirk instead of the
/// binding.
macro_rules! known {
    ($name:ident, $ty:ty, $json:expr, $code:expr, $get:expr) => {
        #[test]
        fn $name() {
            let parsed: $ty = ::serde_json::from_str($json).expect("deserialize");
            let coded = $get(&parsed);
            assert!(
                matches!(coded, Coded::Known(_)),
                "fell back to Unknown — the field is bound to the wrong code \
                 system, which the corpus cannot detect: {coded:?}"
            );
            let out = ::serde_json::to_string(&parsed).expect("serialize");
            assert!(
                out.contains($code),
                "the code did not survive serialization: {out}"
            );
        }
    };
}

/// `Timing.repeat.when` is a **required** binding to `event-timing`, and the
/// T36 resolver recovered 14 of its 27 codes: `AC` (before meals), `PC`
/// (after meals), `HS` (at bedtime) and their meal-specific variants. Before
/// the fix a dosage saying "before meals" deserialized to `Unknown("AC")` —
/// preserved on the wire, absent from the type.
#[test]
fn event_timing_recovers_meal_relative_codes() {
    for code in ["AC", "PC", "HS", "WAKE", "ACM", "PCV"] {
        let json = format!(r#"{{"repeat":{{"when":["{code}"]}}}}"#);
        let parsed: fhir::r5::types::Timing = ::serde_json::from_str(&json).expect("deserialize");
        let when = &parsed.repeat.as_ref().expect("repeat").when;
        assert_eq!(when.len(), 1, "{code}");
        assert!(
            matches!(when[0], Coded::Known(_)),
            "{code} fell back to Unknown — the event-timing value set is not \
             fully resolved: {:?}",
            when[0]
        );
        assert_eq!(
            ::serde_json::to_string(&parsed).expect("serialize"),
            json,
            "{code} did not survive a round trip"
        );
    }
}

known!(
    detected_issue_status,
    fhir::r5::resources::DetectedIssue,
    r#"{"status":"final","code":{"text":"x"}}"#,
    "\"final\"",
    |p: &fhir::r5::resources::DetectedIssue| p.status.clone()
);

known!(
    task_intent,
    fhir::r5::resources::Task,
    r#"{"status":"requested","intent":"order"}"#,
    "\"order\"",
    |p: &fhir::r5::resources::Task| p.intent.clone()
);

known!(
    transport_intent,
    fhir::r5::resources::Transport,
    // Transport makes requestedLocation and currentLocation 1..1, so a
    // minimal instance still needs them.
    r#"{"intent":"order","requestedLocation":{"reference":"Location/a"},"currentLocation":{"reference":"Location/b"}}"#,
    "\"order\"",
    |p: &fhir::r5::resources::Transport| p.intent.clone()
);

known!(
    capability_statement_fhir_version,
    fhir::r5::resources::CapabilityStatement,
    r#"{"resourceType":"CapabilityStatement","status":"active","date":"2026-01-01","kind":"instance","fhirVersion":"5.0.0","format":["json"]}"#,
    "5.0.0",
    |p: &fhir::r5::resources::CapabilityStatement| p.fhir_version.clone()
);

known!(
    structure_definition_fhir_version,
    fhir::r5::resources::StructureDefinition,
    r#"{"resourceType":"StructureDefinition","url":"http://example.org/s","name":"S","status":"active","kind":"resource","abstract":false,"type":"Patient","fhirVersion":"5.0.0"}"#,
    "5.0.0",
    |p: &fhir::r5::resources::StructureDefinition| p
        .fhir_version
        .clone()
        .expect("fhirVersion present")
);

/// No `required` binding resolves to an enum that cannot represent it.
///
/// The generator refuses to offer a one-variant enum for binding (T36). That
/// guard should now catch nothing — the value-set resolver populates every
/// enum a required binding names — and this test says so out loud. If it
/// starts failing, a value set has appeared that the resolver cannot reach,
/// and the affected field silently became a `Code`.
#[test]
fn no_required_binding_resolves_to_a_degenerate_enum() {
    use std::path::Path;

    let defs = Path::new("doc/fhir-specifications/r5/fhir-definitions-json");
    if !defs.exists() {
        eprintln!("skipping: no bundled R5 definitions");
        return;
    }
    let codes = std::fs::read_to_string("fhir-r5/src/codes.rs").expect("codes.rs");

    // enum name -> variant count
    let mut sizes = std::collections::BTreeMap::new();
    let mut rest = codes.as_str();
    while let Some(at) = rest.find("pub enum ") {
        rest = &rest[at + "pub enum ".len()..];
        let Some(brace) = rest.find(" {") else { break };
        let name = rest[..brace].to_string();
        let Some(end) = rest.find("\n}") else { break };
        let count = rest[brace..end]
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.ends_with(',') && t.chars().next().is_some_and(char::is_uppercase)
            })
            .count();
        sizes.insert(name, count);
        rest = &rest[end..];
    }

    let mut offenders = Vec::new();
    for pkg in ["profiles-types.json", "profiles-resources.json"] {
        let Ok(text) = std::fs::read_to_string(defs.join(pkg)) else {
            continue;
        };
        let doc: serde_json::Value = serde_json::from_str(&text).expect("spec JSON");
        for entry in doc["entry"].as_array().into_iter().flatten() {
            let elements = entry["resource"]["snapshot"]["element"].as_array();
            for el in elements.into_iter().flatten() {
                if el["binding"]["strength"].as_str() != Some("required") {
                    continue;
                }
                let Some(vs) = el["binding"]["valueSet"].as_str() else {
                    continue;
                };
                let Some(seg) = vs.split('|').next().and_then(|u| u.rsplit('/').next()) else {
                    continue;
                };
                // Match the generator's naming: strip separators, title-case.
                let candidate: String = seg
                    .split(['-', '.'])
                    .map(|w| {
                        let mut c = w.chars();
                        c.next()
                            .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                            .unwrap_or_default()
                    })
                    .collect();
                if let Some(&n) = sizes.get(&candidate)
                    && n <= 1
                {
                    offenders.push(format!("{} -> {candidate} ({n} variant)", el["path"]));
                }
            }
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "these required bindings resolve to an enum that cannot represent \
         them, so the field silently fell back to `types::Code`:\n{}",
        offenders.join("\n")
    );
}
