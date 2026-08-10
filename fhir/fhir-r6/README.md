# fhir-r6

The FHIR **Release 6 (6.0.0-ballot3)** data model: every resource and datatype as typed
Rust, with validation, builders, and serde.

161 resources, 51 datatypes, 21 primitives, 459 code enums.

Part of the [`fhir`](https://crates.io/crates/fhir) workspace — one crate per
FHIR release, all siblings of
[`fhir-core`](https://crates.io/crates/fhir-core). Use it through the facade
rather than depending on it directly:

```toml
fhir = { version = "2", default-features = false, features = ["r6"] }
```

The facade re-exports this crate as `fhir::r6`, which is the path to
write in your code. Which crate provides it is an implementation detail.

## Status

**R6 is a ballot draft, not a published release.** It is generated from
6.0.0-ballot3 and is outside this workspace's semantic-versioning promise: a
later ballot can rename or remove anything here, and this crate will follow it
without a major version bump. Do not build a production integration on it.

## How this release differs

| | R6 (ballot3) | R5 |
| --- | --- | --- |
| Resources | 161 | 158 |
| `Bundle.link.relation` | coded value | bare string |

The `Bundle.link.relation` change was the only per-release support code that
needed judgement when R6 was added — everything else was generated.

## Example

```rust
use fhir::r6::resources::Patient;
use fhir::r6::types;

let patient = Patient {
    id: Some(types::String("pat-1".to_string())),
    active: Some(types::Boolean(true)),
    ..Default::default()
};
assert!(patient.active.unwrap().0);
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
use fhir::r6::coded::Coded;
use fhir::r6::codes::ObservationStatus;
use fhir::r6::resources::{Observation, Resource};
use fhir::r6::types;
use fhir::validate::Validate;

let wire = r#"{"resourceType":"Patient","id":"pat-1","active":true,
               "name":[{"family":"Doe","given":["Jane"]}]}"#;

// 1. Parse into the polymorphic enum when the type is not known ahead of
//    time — `resourceType` selects the variant.
let resource: Resource = serde_json::from_str(wire).expect("valid resource");
let Resource::Patient(patient) = resource else { panic!("expected a Patient") };
assert_eq!(patient.name[0].family.as_ref().unwrap().0, "Doe");

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
