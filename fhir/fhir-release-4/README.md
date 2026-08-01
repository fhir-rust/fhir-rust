# fhir-release-4

The FHIR **Release 4 (4.0.1)** data model: every resource and datatype as typed
Rust, with validation, builders, and serde.

146 resources, 43 datatypes, 20 primitives, 486 code enums.

Part of the [`fhir`](https://crates.io/crates/fhir) workspace — one crate per
FHIR release, all siblings of
[`fhir-core`](https://crates.io/crates/fhir-core). Use it through the facade
rather than depending on it directly:

```toml
fhir = { version = "2", default-features = false, features = ["r4"] }
```

The facade re-exports this crate as `fhir::r4`, which is the path to
write in your code. Which crate provides it is an implementation detail.

## Status

R4 is the most widely deployed FHIR release. It is the version most national
programmes, EHR vendors, and public APIs implement today, and the one to
target unless you have a reason not to.

## How this release differs

| | R4 | R5 |
| --- | --- | --- |
| `Observation.value[x]` | 11 types | 13 |
| Medication references | `medication[x]` choice | `CodeableReference` |
| `integer64` | absent | present |
| `RatioRange`, `CodeableReference` | absent | present |

R4 introduced `canonical` and `url` primitives, and changed a resource's `id`
from STU3's `id` type to `string`.

## Example

```rust
use fhir::r4::resources::Patient;
use fhir::r4::types;

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
use fhir::r4::coded::Coded;
use fhir::r4::codes::ObservationStatus;
use fhir::r4::resources::{Observation, Resource};
use fhir::r4::types;
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
