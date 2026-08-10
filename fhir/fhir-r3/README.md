# fhir-r3

The FHIR **Release 3 (STU3, 3.0.2)** data model: every resource and datatype as typed
Rust, with validation, builders, and serde.

117 resources, 36 datatypes, 18 primitives, 386 code enums.

Part of the [`fhir`](https://crates.io/crates/fhir) workspace — one crate per
FHIR release, all siblings of
[`fhir-core`](https://crates.io/crates/fhir-core). Use it through the facade
rather than depending on it directly:

```toml
fhir = { version = "2", default-features = false, features = ["r3"] }
```

The facade re-exports this crate as `fhir::r3`, which is the path to
write in your code. Which crate provides it is an implementation detail.

## Status

STU3 was published in 2017 and superseded by R4 in 2019. It remains in
service across national programmes that certified against it, which is why it
is modelled here.

## How this release differs

| | STU3 | R4 onwards |
| --- | --- | --- |
| A resource's `id` | typed `id` | typed `string` |
| `Extension.url` | `uri` | `string` |
| `canonical`, `url` primitives | absent | present |
| `Observation.value[x]` | 11 types | 11 in R4, 13 in R5 |

The `Observation.value[x]` counts match, but the *sets* do not: STU3 admits
`Attachment` and not `integer`; R4 reversed both. A count is not a
compatibility check.

## Example

```rust
use fhir::r3::resources::Patient;
use fhir::r3::types;

let patient = Patient {
    // STU3 types a resource id as `id`, not `string`.
    id: Some(types::Id("pat-1".to_string())),
    active: Some(types::Boolean(true)),
    ..Default::default()
};
assert_eq!(patient.id.as_ref().unwrap().0, "pat-1");
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
use fhir::r3::coded::Coded;
use fhir::r3::codes::ObservationStatus;
use fhir::r3::resources::{Observation, Resource};
use fhir::r3::types;
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
