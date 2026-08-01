//! T9a prototype: choice-type (`value[x]`) as an enum with custom serde.
//!
//! FHIR choice elements appear on the parent object as exactly one of
//! `valueQuantity`, `valueString`, `valueBoolean`, … (the type name suffixed to
//! the base). Primitive variants may additionally carry a paired `_valueString`
//! extension sibling. The current model flattens these into one `Option` field
//! per type; this prototype replaces them with a single enum whose custom
//! `Serialize`/`Deserialize` maps to/from the `value<Type>` (and `_value<Type>`)
//! keys, and integrates into the parent struct with `#[serde(flatten)]`.
//!
//! This validates, before the T9 rollout: (1) complex + primitive variants
//! round-trip; (2) the paired `_value<Type>` extension round-trips; (3) the enum
//! flattens onto the parent; (4) "exactly one" is what serialization emits.

#![cfg(feature = "r5")]

use fhir::r5::types::{Boolean, CodeableConcept, Element, Quantity, String as FhirString};
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A primitive choice value together with its optional `_value<Type>` extension.
#[derive(Debug, Clone, PartialEq, Default)]
struct Primitive<T> {
    value: T,
    extension: Option<Element>,
}

/// Prototype of a generated `Observation.value[x]` choice enum (a representative
/// subset of the 13 R5 types: two complex, two primitive). The real generated
/// enums will `Box` large variants; the prototype keeps them inline.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum ObservationValue {
    Quantity(Quantity),
    CodeableConcept(CodeableConcept),
    String(Primitive<FhirString>),
    Boolean(Primitive<Boolean>),
}

impl Serialize for ObservationValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Serialize as a single-entry (or two-entry, for a primitive with an
        // extension) map so `#[serde(flatten)]` merges the keys onto the parent.
        let mut map = serializer.serialize_map(None)?;
        match self {
            ObservationValue::Quantity(v) => map.serialize_entry("valueQuantity", v)?,
            ObservationValue::CodeableConcept(v) => {
                map.serialize_entry("valueCodeableConcept", v)?;
            }
            ObservationValue::String(p) => {
                map.serialize_entry("valueString", &p.value)?;
                if let Some(ext) = &p.extension {
                    map.serialize_entry("_valueString", ext)?;
                }
            }
            ObservationValue::Boolean(p) => {
                map.serialize_entry("valueBoolean", &p.value)?;
                if let Some(ext) = &p.extension {
                    map.serialize_entry("_valueBoolean", ext)?;
                }
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for ObservationValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ObservationValueVisitor)
    }
}

struct ObservationValueVisitor;

impl<'de> Visitor<'de> for ObservationValueVisitor {
    type Value = ObservationValue;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a FHIR value[x] choice element")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut quantity = None;
        let mut codeable = None;
        let mut string = None;
        let mut string_ext = None;
        let mut boolean = None;
        let mut boolean_ext = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "valueQuantity" => quantity = Some(map.next_value::<Quantity>()?),
                "valueCodeableConcept" => codeable = Some(map.next_value::<CodeableConcept>()?),
                "valueString" => string = Some(map.next_value::<FhirString>()?),
                "_valueString" => string_ext = Some(map.next_value::<Element>()?),
                "valueBoolean" => boolean = Some(map.next_value::<Boolean>()?),
                "_valueBoolean" => boolean_ext = Some(map.next_value::<Element>()?),
                // Ignore any non-value key (matters under flatten, where sibling
                // parent fields also flow through here).
                _ => {
                    map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        // Exactly one value variant must be present.
        let count = [
            quantity.is_some(),
            codeable.is_some(),
            string.is_some(),
            boolean.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count();
        if count != 1 {
            return Err(de::Error::custom(format!(
                "expected exactly one value[x] variant, found {count}"
            )));
        }
        if let Some(v) = quantity {
            return Ok(ObservationValue::Quantity(v));
        }
        if let Some(v) = codeable {
            return Ok(ObservationValue::CodeableConcept(v));
        }
        if let Some(value) = string {
            return Ok(ObservationValue::String(Primitive {
                value,
                extension: string_ext,
            }));
        }
        let value = boolean.unwrap();
        Ok(ObservationValue::Boolean(Primitive {
            value,
            extension: boolean_ext,
        }))
    }
}

/// A tiny stand-in for `Observation`: a required primitive plus the flattened
/// choice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ObservationLite {
    status: FhirString,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    value: Option<ObservationValue>,
}

fn roundtrip(v: &serde_json::Value) -> serde_json::Value {
    let obs: ObservationLite = serde_json::from_value(v.clone()).expect("deserialize");
    serde_json::to_value(&obs).expect("serialize")
}

#[test]
fn complex_variant_quantity() {
    let json = serde_json::json!({
        "status": "final",
        "valueQuantity": { "value": 98.6, "unit": "F" }
    });
    assert_eq!(roundtrip(&json), json);
    let obs: ObservationLite = serde_json::from_value(json).unwrap();
    assert!(matches!(obs.value, Some(ObservationValue::Quantity(_))));
}

#[test]
fn complex_variant_codeable_concept() {
    let json = serde_json::json!({
        "status": "final",
        "valueCodeableConcept": { "text": "Present" }
    });
    assert_eq!(roundtrip(&json), json);
}

#[test]
fn primitive_variant_string() {
    let json = serde_json::json!({ "status": "final", "valueString": "positive" });
    assert_eq!(roundtrip(&json), json);
}

#[test]
fn primitive_variant_with_extension() {
    // The paired `_valueString` primitive extension must round-trip alongside
    // its value.
    let json = serde_json::json!({
        "status": "final",
        "valueString": "positive",
        "_valueString": {
            "extension": [{
                "url": "http://example.org/confidence",
                "valueDecimal": 0.9
            }]
        }
    });
    assert_eq!(roundtrip(&json), json);
}

#[test]
fn extension_only_is_treated_as_absent() {
    // FINDING: with `#[serde(flatten)]` on `Option<Choice>`, serde turns the
    // field's deserialize errors into `None`. So a malformed choice — here a
    // lone `_valueBoolean` with no `valueBoolean` — is leniently treated as
    // absent rather than rejected. Deserialize-time "exactly one" strictness is
    // therefore NOT enforceable through flatten; it holds at the type level (the
    // enum can hold only one variant) and on the serialize side. This is an
    // accepted tradeoff (be liberal in what you accept); strict rejection would
    // require a hand-written parent `Deserialize` instead of `flatten`.
    let json = serde_json::json!({ "status": "final", "_valueBoolean": { "id": "x" } });
    let obs: ObservationLite = serde_json::from_value(json).expect("deserialize (lenient)");
    assert_eq!(obs.value, None);
}

#[test]
fn absent_choice_is_none() {
    let json = serde_json::json!({ "status": "final" });
    let obs: ObservationLite = serde_json::from_value(json.clone()).expect("deserialize");
    assert_eq!(obs.value, None);
    assert_eq!(serde_json::to_value(&obs).unwrap(), json);
}

#[test]
fn build_and_serialize_emits_one_key() {
    let obs = ObservationLite {
        status: FhirString("final".to_string()),
        value: Some(ObservationValue::Boolean(Primitive {
            value: Boolean(true),
            extension: None,
        })),
    };
    let json = serde_json::to_value(&obs).unwrap();
    assert_eq!(
        json,
        serde_json::json!({ "status": "final", "valueBoolean": true })
    );
}
