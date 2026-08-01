# fhir-release-2

The FHIR **DSTU2 (1.0.2)** data model: every resource and datatype as typed
Rust, with validation, builders, and serde.

94 resources, 28 datatypes, 18 primitives, 265 code enums.

Part of the [`fhir`](https://crates.io/crates/fhir) workspace — one crate per
FHIR release, all siblings of
[`fhir-core`](https://crates.io/crates/fhir-core). Use it through the facade
rather than depending on it directly:

```toml
fhir = { version = "2", default-features = false, features = ["r2"] }
```

The facade re-exports this crate as `fhir::r2`, which is the path to write in
your code. Which crate provides it is an implementation detail.

## Status

DSTU2 was published in 2015 and superseded by STU3 in 2017. It is modelled
here for one reason: deployed systems still speak it, and a library that
cannot read what a running server emits is of no use to the person holding
the data.

## How this release differs

| | DSTU2 | STU3 onwards |
| --- | --- | --- |
| `HumanName.family` | repeats (`0..*`) | a single value |
| Medication ordering | `MedicationOrder` | `MedicationRequest` |
| Server capabilities | `Conformance` | `CapabilityStatement` |
| Recursive elements | `nameReference` | `contentReference` |
| `Observation.value[x]` | 10 types | 11 in STU3, 13 in R5 |

`HumanName.family` is the difference most likely to break a port: it is a
`Vec` here and a single value afterwards, so code written against STU3 will
not compile until every family name becomes a list.

`OperationOutcome.issue` is a smaller trap. STU3 added `expression` and kept
`location`, so both exist there; DSTU2 has only `location`. Code that reads
`expression` compiles against STU3 and finds nothing in DSTU2 data.

The `nameReference` row is why this crate exists in the shape it does. DSTU2
expresses a recursive element by naming the element it repeats, where STU3
onwards give a path. The generator understood only the modern spelling, and
92 DSTU2 elements — `Bundle.entry.link`,
`ValueSet.codeSystem.concept.concept` and `ValueSet.expansion.contains
.contains` among them — were silently dropped from the model. Nothing
errored; parsing a bundle simply discarded its entry links and nested
concepts. It is fixed, and a round-trip over the 1,572 resources embedded in
the published DSTU2 definitions runs on every change so it cannot come back.

## Example

```rust
use fhir::r2::resources::Patient;
use fhir::r2::types;

let patient = Patient {
    id: Some(types::Id("pat-1".to_string())),
    // DSTU2 lets a family name repeat; STU3 onwards do not.
    name: vec![types::HumanName {
        family: vec![types::String("Doe".to_string())],
        given: vec![types::String("Jane".to_string())],
        ..Default::default()
    }],
    ..Default::default()
};
assert_eq!(patient.name[0].family[0].0, "Doe");
```

Every release exposes the same module shape — `types`, `resources`, `codes`,
`validate`, `meta`, `choice`, `coded`, `builder`, `temporal`, `summary`,
`extension_ext`, `bundle_util`, `prelude`, plus `client` and `xml` under
their features — so porting between releases is a matter of changing one path
segment. What it is *not* is a guarantee that the code still means the same
thing; see the table above.

## A fuller walkthrough

Parse, validate, serialize, build, and handle an unknown code — the whole
loop a server integration needs. This runs as a doctest, so it is compiled
and executed on every change.

```rust
use fhir::r2::coded::Coded;
use fhir::r2::codes::ObservationStatus;
use fhir::r2::resources::{Observation, Resource};
use fhir::r2::types;
use fhir::validate::Validate;

// `family` is a JSON array here. DSTU2 lets it repeat, so the wire format
// differs from STU3 onwards, not just the Rust type.
let wire = r#"{"resourceType":"Patient","id":"pat-1","active":true,
               "name":[{"family":["Doe"],"given":["Jane"]}]}"#;

// 1. Parse into the polymorphic enum when the type is not known ahead of
//    time — `resourceType` selects the variant.
let resource: Resource = serde_json::from_str(wire).expect("valid resource");
let Resource::Patient(patient) = resource else { panic!("expected a Patient") };
assert_eq!(patient.name[0].family[0].0, "Doe");

// 2. Validate: primitive formats, required bindings, cardinality.
assert!(patient.validate().is_empty());

// 3. Serialize. `resourceType` belongs to the enum, not the struct — a bare
//    `Patient` omits it, which matters when writing back to a server.
let bare = serde_json::to_string(&patient).expect("serializes");
assert!(!bare.contains("resourceType"));
let tagged = serde_json::to_string(&Resource::Patient(patient)).expect("serializes");
assert!(tagged.contains(r#""resourceType":"Patient""#));
// Absent fields stay absent rather than becoming null.
assert!(!tagged.contains("null"));

// 4. Required fields are checked when the builder is built, not at use.
let obs = Observation::builder()
    .status(Coded::Known(ObservationStatus::Final))
    .code(types::CodeableConcept::default())
    .build()
    .expect("both required fields set");
assert!(matches!(obs.status, Coded::Known(ObservationStatus::Final)));
assert!(Observation::builder().build().is_err());

// 5. An unrecognized code is preserved rather than failing the parse — real
//    servers emit codes outside the published value set.
let unknown: Coded<ObservationStatus> =
    serde_json::from_value(serde_json::json!("not-a-status")).unwrap();
assert_eq!(unknown, Coded::Unknown("not-a-status".to_string()));
```

Note line 1: `family` is indexed rather than unwrapped, because DSTU2 lets a
family name repeat. Every later release makes it a single value, so this line
is the one that changes when porting.

## Why the releases are separate crates

A shared `Patient` across releases would have to be a union of every
release's elements — accepting data valid in none of them — or an
intersection, silently dropping data that was valid where it came from. Both
failure modes are silent, and both corrupt health records. Distinct Rust
types make the mismatch a compile error instead.

Convert between releases explicitly, through JSON, and decide what to do with
whatever does not carry over. Serde reports what the target release will not
accept rather than discarding it.

Compiling one release costs one `rustc` process, not several: peak memory is
the largest crate, not the sum. That is why this is a crate rather than a
module.

## License

MIT. FHIR® is a registered trademark of HL7, used with permission.
