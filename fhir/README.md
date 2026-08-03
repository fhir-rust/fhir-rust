# FHIR for Rust

[![CI](https://github.com/fhir-rust/fhir-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/fhir-rust/fhir-rust/actions/workflows/ci.yml)

A Rust implementation of the **HL7 FHIR®** data model, plus a code generator
that produces it from the official FHIR specification JSON files. Three releases
are modelled: **R5 (5.0.0)**, **R4 (4.0.1)**, and **R3 (3.0.2, STU3)**.

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you build,
parse, validate, and round-trip FHIR resources in idiomatic Rust with `serde`.

> **Status:** stable (1.0). All three data models (resources, datatypes,
> primitives, code systems, validation) are implemented and green, and the API
> follows semantic versioning.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.

## Features

Per release, under `fhir::r5`, `fhir::r4` and `fhir::r3`:

- **Every resource** (Patient, Observation, Encounter, …) as a Rust struct,
  round-tripping to and from canonical FHIR JSON via `serde` — 158 in R5,
  146 in R4, 117 in R3.
- **Every complex datatype** (Period, HumanName, CodeableConcept, …) and every
  **primitive newtype** (`Code`, `Id`, `DateTime`, …), serializing transparently.
- **400+ code systems** as type-safe enums that serialize to their canonical
  FHIR code strings.
- **A polymorphic `Resource` enum**, tagged by `resourceType`, for reading a
  resource whose type you do not know ahead of time.
- **Lightweight validation** via a `Validate` trait and `#[derive(Validate)]`
  that walks every field recursively.
- **Builders, a prelude, extension helpers, `Bundle` utilities**, and summary
  serialization.
- Optional **async REST client** (`client`) and **FHIR XML** (`xml`).

And a **code generator** that reads the bundled specification JSON for a release
and emits that release's Rust model.

## Installation

Each release is a complete model of ~135,000 lines of Rust, so they are cargo
features: you compile only what you use. `r5` is on by default.

```toml
[dependencies]
# R5 only (the default)
fhir = "2"

# R5 plus older releases
# fhir = { version = "2", features = ["r4"] }
# fhir = { version = "2", features = ["r3", "r4"] }

# One older release on its own — R3, R4 and R5 never both compile unless asked
# fhir = { version = "2", default-features = false, features = ["r4"] }
# fhir = { version = "2", default-features = false, features = ["r3"] }

# Optional capabilities
# fhir = { version = "2", features = ["xml", "client"] }

serde_json = "1" # or any other serde data format
```

| Release | Module | Feature | Resources | Datatypes | Primitives | Code enums |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| R5 (5.0.0) | `fhir::r5` | `r5` (default) | 158 | 50 | 21 | 442 |
| R4 (4.0.1) | `fhir::r4` | `r4` | 146 | 43 | 20 | 486 |
| R3 (3.0.2) | `fhir::r3` | `r3` | 117 | 36 | 18 | 386 |
| R6 (6.0.0-ballot3) | `fhir::r6` | `r6` | 161 | 51 | 21 | 459 |

R6 is **in ballot, not final**. Its model is generated from a draft that can
still change, so it is unpublished, off by default, and outside this crate's
semver promise. `Release::VERSION` reports `6.0.0-ballot3` — the identifier
the specification gives itself — because that string reaches
`CapabilityStatement.fhirVersion`.

## Choosing a release

An R3 `Patient`, an R4 `Patient` and an R5 `Patient` are **different Rust
types**, on purpose. The releases genuinely disagree, and not merely by growing:

- `Observation.value[x]` admits eleven types in R3 and eleven in R4 — but not
  the same eleven. R3 allows `Attachment` and not `integer`; R4 reversed both;
  R5 allows all of them plus `Reference`.
- A resource's `id` is typed `id` in R3 and `string` in R4/R5.
  `Extension.url` is a `uri` in R3 and a `string` afterwards.
- `MedicationRequest.medication[x]` is a choice element in R4 but a
  `CodeableReference` in R5.
- R3 has no `canonical` or `url` primitive; R4 has no `integer64`,
  `CodeableReference`, or `RatioRange`.

A single type standing for all of them would either accept data that is invalid
in every release or silently drop data that is valid in one.

The two modules are otherwise identical in shape, so porting code between
releases is a matter of changing one path segment:

```rust
use fhir::r4::resources::Patient;   // instead of fhir::r5::resources::Patient
use fhir::r4::codes::AdministrativeGender;
```

```rust
use fhir::r3::resources::Patient;   // …or fhir::r3
use fhir::r3::codes::AdministrativeGender;
```

To move data between releases, use `fhir::convert`, which converts the JSON wire
form and hands back a report of everything it changed or discarded:

```rust
use fhir::convert;
use fhir::{r4::R4, r5::R5};

let out = convert::between::<R4, R5>(&r4_json);

if !out.report.is_lossless() {
    for loss in out.report.iter() {
        eprintln!("{loss}");   // e.g. "Patient.animal: element not in target (…)"
    }
}
let r5_patient: fhir::r5::resources::Patient = serde_json::from_value(out.value)?;
```

Do **not** reach for plain serde here, which is what this README used to
recommend. Serde reports the first mismatch and stops, and — the case that
matters — an element the target release does not have is not a mismatch at all:
unknown keys are ignored, so the field disappears and nothing is said. The
commonest difference between releases is exactly the one serde is silent about.

There is no `From` between releases and there will not be one; the report is the
whole point, and a conversion the compiler performs for you cannot give you one.
See the `r4_and_r5_side_by_side` example for a worked version, and
[spec 14](spec/14-cross-release-conversion.md) for the rules.

What the releases *share* lives at the crate root and is re-exported by each, so
`fhir::r3::validate::Validate`, `fhir::r4::validate::Validate` and
`fhir::r5::validate::Validate` are all the same trait: validation, `Coded<E>`,
builders, the element metadata table, date/time parsing, and the REST client.

## Quick start

Build a `Patient`, serialize to canonical FHIR JSON, and parse it back:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::coded::Coded;
use fhir::r5::codes::AdministrativeGender;
use fhir::r5::types::{Boolean, HumanName, String as FhirString};

let patient = Patient {
    id: Some(FhirString("pat-1".to_string())),
    active: Some(Boolean(true)),
    gender: Some(Coded::Known(AdministrativeGender::Male)),
    name: vec![HumanName {
        family: Some(FhirString("Chalmers".to_string())),
        given: vec![FhirString("Peter".to_string())],
        ..Default::default()
    }],
    ..Default::default()
};

let json = serde_json::to_string_pretty(&patient).unwrap();
let parsed: Patient = serde_json::from_str(&json).unwrap();
assert_eq!(parsed, patient);
```

## How the model maps to Rust

Everything derives `serde::Serialize` and `serde::Deserialize`, so you work
through `serde_json` (or any serde format).

- **Primitives are transparent newtypes.** `Code("final")` serializes to the
  JSON string `"final"` — no wrapper object. (`integer64` is the FHIR-mandated
  exception: it serializes as a JSON *string*.)
- **Element cardinality maps directly:**

  | FHIR cardinality | Rust type        |
  |------------------|------------------|
  | `0..1`           | `Option<T>`      |
  | `1..1`           | `T`              |
  | `0..*`           | `Vec<T>`         |
  | `1..*`           | `Vec1<T>`        |

- **`value[x]` choice elements** are one generated enum per element (e.g.
  `Observation.value` is `Option<ObservationValue>` with a variant per allowed
  type), so exactly one type is set at compile time.
- **Required-binding coded fields** are their `codes::` enum wrapped in
  `Coded<E>` (a `Known(E)` | `Unknown(String)` fallback for wire compatibility).
- **Builders**: `Type::builder()…build()` enforces required `1..1` fields; a
  `fhir::prelude` re-exports the common items.
- **Nested backbone elements** become nested structs named `<Parent><Field>`
  (e.g. `PatientContact`, `BundleEntry`).
- **Unset optional fields are omitted** from the JSON (`skip_serializing_none`).

## Validation

`Validate` reports every problem as a `ValidationIssue { path, message }`.
Primitive types check their FHIR regex constraints; `#[derive(Validate)]` makes
complex types and resources validate recursively, prefixing each nested issue's
`path` with the field name.

```rust
use fhir::r5::types::Id;
use fhir::r5::validate::Validate;

assert!(Id("patient-1".to_string()).is_valid());
assert!(!Id("has spaces".to_string()).is_valid());
```

### What validation does *not* cover

"Validation" here means structural validation — element existence, types,
cardinality, primitive lexical rules, choice exclusivity, required bindings —
plus three FHIR invariants (`ext-1`, `dom-2`, `dom-4`) of the 314 that R5
states. It does **not** check:

- **FHIRPath invariants** — the remaining 311 keys, including `ele-1`, need a
  FHIRPath evaluator (see [`spec/10-invariants-coverage.md`](spec/10-invariants-coverage.md),
  which enumerates every unenforced rule rather than leaving it implicit).
- **Profile conformance** — US Core, IPS, or any implementation guide.
- **Terminology** — that a code is a member of a value set, beyond the
  required-binding enums the model generates.
- **Reference resolution** — that a `Reference` points at anything.

A resource this crate calls valid may still be rejected by a conformant FHIR
server. Use a validating server or a terminology service where those checks
matter.

## Code systems

```rust
use fhir::r5::codes::AdministrativeGender;

let gender = AdministrativeGender::Female;
assert_eq!(serde_json::to_value(&gender).unwrap(), "female");
```

## Reading a resource of unknown type

```rust
use fhir::r5::resources::Resource;

let json = serde_json::json!({ "resourceType": "Patient", "id": "pat-1" });
match serde_json::from_value(json).unwrap() {
    Resource::Patient(patient) => assert_eq!(patient.id.unwrap().0, "pat-1"),
    _ => unreachable!(),
}
```

## Runnable examples

Programs in the [`examples/`](examples/) directory demonstrate common tasks:

```sh
cargo run --example tutorial           # the guide's end-to-end walkthrough
cargo run --example build_patient      # build a resource and print its JSON
cargo run --example validate_resource  # recursive validation and issue paths
cargo run --example read_bundle        # dispatch on each entry's resourceType
cargo run --example code_systems       # code-system enums
cargo run --example primitive_extensions  # _field primitive extensions
cargo run --example operation_outcome     # validation → OperationOutcome
cargo run --example extensions            # ExtensionExt: get/set extensions
cargo run --example transaction_bundle    # build/read a transaction Bundle
cargo run --example client_crud --features client  # REST CRUD vs HAPI

cargo run --example r4_patient --features r4              # the same, in R4
cargo run --example r3_patient --features r3              # the same, in R3
cargo run --example r4_and_r5_side_by_side --features "r4 r5"  # two at once
```

The R5 examples all work for R4 or R3 by changing `r5` in the imports.

## Workspace layout

One crate per FHIR release, so a project that needs R4 does not download,
compile, or vendor R3 and R5.

| Crate | Contents |
| --- | --- |
| [`fhir`](https://crates.io/crates/fhir) | The facade you depend on. Re-exports the models behind features. |
| [`fhir-core`](https://crates.io/crates/fhir-core) | Everything release-independent: `Decimal`, `Validate`, `Coded<E>`, `BuilderError`, temporal parsing, `_summary` pruning, XML, the REST client, the `Release` trait. |
| [`fhir-release-5`](https://crates.io/crates/fhir-release-5) | 158 resources, 50 datatypes, 21 primitives, 442 code enums |
| [`fhir-release-4`](https://crates.io/crates/fhir-release-4) | 146 resources, 43 datatypes, 20 primitives, 486 code enums |
| [`fhir-release-3`](https://crates.io/crates/fhir-release-3) | 117 resources, 36 datatypes, 18 primitives, 386 code enums |
| [`fhir-release-2`](https://crates.io/crates/fhir-release-2) | 94 resources, 28 datatypes, 18 primitives, 265 code enums — DSTU2 |
| [`fhir-release-6`](https://crates.io/crates/fhir-release-6) | 161 resources, 51 datatypes, 21 primitives, 459 code enums — **ballot draft**, outside the semver promise |
| [`fhir-release-1`](https://crates.io/crates/fhir-release-1) | Name reservation. DSTU1 (0.0.82) is a real specification but is not modelled here; contains no types. |
| [`fhir-release-7`](https://crates.io/crates/fhir-release-7), `-8`, `-9`, `-10` | Name reservations. No such specifications exist; these contain no model. |
| [`fhir-derive-macros`](https://crates.io/crates/fhir-derive-macros) | `#[derive(Validate)]`, `Builder`, `FhirChoice` |

Every release crate has the same shape, so porting between releases is a
matter of changing one path segment:

```txt
fhir-release-5/
  src/
    resources/      Resource structs + the polymorphic `Resource` enum
    types/          Complex datatypes + primitive newtypes
    codes.rs        FHIR CodeSystems as enums
    validate.rs     That release's primitive constraints
    meta/           That release's generated element metadata
    coded.rs choice.rs builder.rs summary.rs temporal.rs
    extension_ext.rs bundle_util.rs prelude.rs client.rs xml.rs

src/                The facade: lib.rs, prelude, codegen/, r5/parse/
doc/                Bundled FHIR specification JSON, one directory per release
examples/           Runnable example programs
```

Why separate crates: type-checking is single-threaded and scales with crate
size, so as modules of one crate the releases summed. Building every release
needed **12.9 GB** of memory; as separate crates the peak is the largest crate
rather than the sum, and it needs **5.0 GB**.

## Documentation

- **The guide** — a task-oriented mdBook in [`book/`](book/) (getting started,
  model mapping, JSON serialization, validation, terminology, extensions,
  bundles, and generator internals). Build it with `mdbook build book`.
- **API reference** — build and open the full API docs, including the crate
  guide and every resource/datatype:

  ```sh
  cargo doc --open
  ```

## The code generator

Each release's `types`, `resources`, `codes`, and `meta` modules are derived
from that release's official specification JSON in
`doc/fhir-specifications/<release>/fhir-definitions-json/`. The generator lives
under `src/codegen`; the binary in `src/main.rs` drives it:

```sh
cargo run -- r3                    # rewrite fhir-release-3/src from the R3 definitions
cargo run -- r4                    # rewrite fhir-release-4/src from the R4 definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 elsewhere, to compare
```

`fhir-release-3/src` and `fhir-release-4/src` are entirely generated and safe to rewrite. `fhir-release-5/src` is not: it carries
hand-written prose on top of generated shapes, so `cargo run -- r5` refuses to
write there without an explicit `--out`. See [`AGENTS.md`](AGENTS.md) and
[`spec/`](spec/) for the generator's design and conventions.

---

## FHIR specification reference

The remainder of this document is background reference on the FHIR
specification files the generator consumes (described for R5; R4 publishes the
same bundles). It is useful when working on the generator itself.

### Datatype categories

FHIR R5 datatypes live in `profiles-types.json`, which distinguishes primitive
types (lowercase names) from complex types (uppercase names).

**Primitive types:** `base64Binary`, `boolean`, `canonical`, `code`, `date`,
`dateTime`, `decimal`, `id`, `instant`, `integer`, `integer64`, `markdown`,
`oid`, `positiveInt`, `string`, `time`, `unsignedInt`, `uri`, `url`, `uuid`.

**General-purpose complex types:** Address, Age, Annotation, Attachment,
CodeableConcept, Coding, ContactPoint, Count, Distance, Duration, HumanName,
Identifier, Money, MoneyQuantity, Period, Quantity, Range, Ratio, RatioRange,
SampledData, Signature, SimpleQuantity, Timing.

**Metadata complex types:** Availability, ContactDetail, Contributor,
DataRequirement, Expression, ExtendedContactDetail, MonetaryComponent,
ParameterDefinition, RelatedArtifact, TriggerDefinition, UsageContext,
VirtualServiceDetail.

**Special-purpose complex types:** BackboneType, CodeableReference, Dosage,
ElementDefinition, Extension, Meta, Narrative, Reference, xhtml.

You can list the ids straight from the spec with `jq`:

```sh
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "primitive-type")) | map(.resource.id)[]'
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "complex-type"))   | map(.resource.id)[]'
```

### Element extension URLs

Any element defined in any version of FHIR is automatically assigned an
extension URL that uniquely identifies it:

```txt
http://hl7.org/fhir/[version]/StructureDefinition/extension-[Path]
```

### Snapshot view versus differential view

A FHIR profile offers two views of a profiled resource:

- **Snapshot** — the complete, final structure after applying all changes from
  the differential to the base resource. Self-contained; useful when you do not
  have the base resource at hand.
- **Differential** — only the differences (added, modified, or removed elements)
  the profile introduces relative to its base. Useful for understanding what a
  profile customizes.

### FHIR documentation links

- Datatypes: <https://build.fhir.org/datatypes.html>
- JSON representation: <https://build.fhir.org/json.html>
- UML: <https://build.fhir.org/uml.html>
- References: <https://build.fhir.org/references.html>
- Extensibility: <https://build.fhir.org/extensibility.html>
- Narrative: <https://build.fhir.org/narrative.html>
- Resource: <https://build.fhir.org/resource.html>
- Versions / standards process: <https://build.fhir.org/versions.html#std-process>

## FHIR®

FHIR® is the registered trademark of HL7 and is used with the permission of HL7. Use of the FHIR trademark does not constitute endorsement of this library by HL7.

## License

Licensed under any of:

- MIT
- Apache License 2.0
- BSD 3-Clause
- GPL 2.0 only
- GPL 3.0 only

at your option.
