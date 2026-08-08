//! Property-based round-trip tests (T4/T48).
//!
//! The corpus gate proves the model handles the documents HL7 publishes; these
//! prove it handles *combinations* nobody published. Each strategy assembles a
//! schema-valid JSON document for one of five representative types with a
//! randomized subset of populated fields and randomized values, and the
//! property is a lossless round-trip through the typed model:
//!
//! ```text
//! Value -> T -> Value   must be identity
//! T -> Value -> T       must be identity (via the same comparison)
//! ```
//!
//! `serde_json`'s `arbitrary_precision` is always on (spec 02, R2.2), so the
//! `Value` comparison sees decimal lexemes — `36.60` surviving as `36.60` is
//! part of the property, not an accident of float formatting.
//!
//! The generators only ever *add* populated fields — no nulls, no empty
//! arrays — because the model's `skip_serializing_if` conventions drop
//! empties on the way out, which would be a spurious mismatch rather than a
//! data loss.

#![cfg(feature = "r5")]

use proptest::prelude::*;
use serde_json::{Value, json};

/// Round-trip `json` through `T`, requiring exact `Value` equality.
fn assert_roundtrip<T>(v: &Value)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let typed: T = serde_json::from_value(v.clone())
        .unwrap_or_else(|e| panic!("deserialize failed: {e}\n{v:#}"));
    let back = serde_json::to_value(&typed).expect("serialize");
    assert_eq!(*v, back, "round-trip changed the document");
}

// --- small value strategies -------------------------------------------------

/// A FHIR `id`: 1..=64 of `[A-Za-z0-9.-]`.
fn fhir_id() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[A-Za-z0-9][A-Za-z0-9.-]{0,63}").unwrap()
}

/// A human-ish string with no control characters.
fn text() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[ -~]{1,24}").unwrap()
}

/// A FHIR `code`: no leading/trailing/double spaces.
fn code() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-z][a-z0-9-]{0,15}").unwrap()
}

/// A decimal rendered with trailing-zero variety, as a raw JSON number.
fn decimal() -> impl Strategy<Value = Value> {
    (0i64..10_000, 0u8..4u8).prop_map(|(mantissa, scale)| {
        let s = if scale == 0 {
            mantissa.to_string()
        } else {
            let d = 10i64.pow(u32::from(scale));
            format!(
                "{}.{:0width$}",
                mantissa / d,
                mantissa % d,
                width = scale as usize
            )
        };
        serde_json::from_str::<Value>(&s).unwrap()
    })
}

/// A date at one of FHIR's three precisions.
fn date() -> impl Strategy<Value = String> {
    (1900u32..2100, 1u32..13, 1u32..29, 0u8..3u8).prop_map(|(y, m, d, precision)| match precision {
        0 => format!("{y:04}"),
        1 => format!("{y:04}-{m:02}"),
        _ => format!("{y:04}-{m:02}-{d:02}"),
    })
}

/// A `Coding` object.
fn coding() -> impl Strategy<Value = Value> {
    (code(), proptest::option::of(text()), any::<bool>()).prop_map(|(c, display, user_selected)| {
        let mut o = json!({ "system": "http://example.org/cs", "code": c });
        if let Some(d) = display {
            o["display"] = json!(d);
        }
        if user_selected {
            o["userSelected"] = json!(true);
        }
        o
    })
}

/// A `CodeableConcept`: 1..=3 codings, optional text.
fn codeable_concept() -> impl Strategy<Value = Value> {
    (
        proptest::collection::vec(coding(), 1..=3),
        proptest::option::of(text()),
    )
        .prop_map(|(codings, t)| {
            let mut o = json!({ "coding": codings });
            if let Some(t) = t {
                o["text"] = json!(t);
            }
            o
        })
}

/// A `Timing`: events and/or a repeat with a few populated members.
fn timing() -> impl Strategy<Value = Value> {
    (
        proptest::collection::vec(date(), 0..=2),
        proptest::option::of((1u32..5, decimal(), code())),
    )
        .prop_map(|(events, repeat)| {
            let mut o = json!({});
            if !events.is_empty() {
                // Timing.event is dateTime; a date-precision value is valid.
                o["event"] = json!(events);
            }
            if let Some((freq, period, unit_ish)) = repeat {
                o["repeat"] = json!({
                    "frequency": freq,
                    "period": period,
                    // periodUnit is a required binding (UCUM units); use a
                    // fixed valid code and keep the random one as a comment
                    // field it cannot break: Timing.code is CodeableConcept.
                    "periodUnit": "h",
                });
                o["code"] = json!({ "text": unit_ish });
            }
            o
        })
}

/// A `HumanName`.
fn human_name() -> impl Strategy<Value = Value> {
    (text(), proptest::collection::vec(text(), 0..=2)).prop_map(|(family, given)| {
        let mut o = json!({ "family": family });
        if !given.is_empty() {
            o["given"] = json!(given);
        }
        o
    })
}

/// A `Patient` with a randomized subset of populated fields.
fn patient() -> impl Strategy<Value = Value> {
    (
        fhir_id(),
        proptest::option::of(any::<bool>()),
        proptest::option::of(proptest::sample::select(vec![
            "male", "female", "other", "unknown",
        ])),
        proptest::option::of(date()),
        proptest::collection::vec(human_name(), 0..=3),
    )
        .prop_map(|(id, active, gender, birth_date, names)| {
            let mut o = json!({ "resourceType": "Patient", "id": id });
            if let Some(a) = active {
                o["active"] = json!(a);
            }
            if let Some(g) = gender {
                o["gender"] = json!(g);
            }
            if let Some(b) = birth_date {
                o["birthDate"] = json!(b);
            }
            if !names.is_empty() {
                o["name"] = json!(names);
            }
            o
        })
}

/// An `Observation`, sometimes carrying a `valueQuantity` and a contained
/// Patient — `contained` is typed as of T47, so it belongs in the property.
fn observation() -> impl Strategy<Value = Value> {
    (
        fhir_id(),
        proptest::sample::select(vec!["registered", "preliminary", "final", "amended"]),
        codeable_concept(),
        proptest::option::of(decimal()),
        proptest::option::of(patient()),
    )
        .prop_map(|(id, status, concept, value, contained)| {
            let mut o = json!({
                "resourceType": "Observation",
                "id": id,
                "status": status,
                "code": concept
            });
            if let Some(v) = value {
                o["valueQuantity"] = json!({ "value": v, "unit": "1" });
            }
            if let Some(c) = contained {
                o["contained"] = json!([c]);
            }
            o
        })
}

/// A searchset `Bundle` of Patients and Observations.
fn bundle() -> impl Strategy<Value = Value> {
    (
        proptest::collection::vec(prop_oneof![patient(), observation()], 0..=4),
        proptest::option::of(0u32..1000),
    )
        .prop_map(|(resources, total)| {
            let mut o = json!({ "resourceType": "Bundle", "type": "searchset" });
            if let Some(t) = total {
                o["total"] = json!(t);
            }
            if !resources.is_empty() {
                o["entry"] = json!(
                    resources
                        .into_iter()
                        .map(|r| json!({ "resource": r }))
                        .collect::<Vec<_>>()
                );
            }
            o
        })
}

// --- the properties ----------------------------------------------------------

proptest! {
    #[test]
    fn proptest_codeable_concept_roundtrips(v in codeable_concept()) {
        assert_roundtrip::<fhir::r5::types::CodeableConcept>(&v);
    }

    #[test]
    fn proptest_timing_roundtrips(v in timing()) {
        assert_roundtrip::<fhir::r5::types::Timing>(&v);
    }

    // Resources round-trip through the `Resource` enum: the `resourceType`
    // tag lives on the enum, so a bare struct serializes without it — the
    // first thing this suite caught on its first run.
    #[test]
    fn proptest_patient_roundtrips(v in patient()) {
        assert_roundtrip::<fhir::r5::resources::Resource>(&v);
    }

    #[test]
    fn proptest_observation_roundtrips(v in observation()) {
        assert_roundtrip::<fhir::r5::resources::Resource>(&v);
    }

    #[test]
    fn proptest_bundle_roundtrips(v in bundle()) {
        assert_roundtrip::<fhir::r5::resources::Resource>(&v);
    }
}
