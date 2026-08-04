# FHIR releases

This crate models **five** FHIR releases, each a complete, independent crate
(`fhir-release-2` … `fhir-release-6`) exposed as a facade feature:

| Release | Module | Cargo feature | Resources | Datatypes | Primitives | Code enums |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| R2 (1.0.2, DSTU2) | `fhir::r2` | `r2` | 94 | 28 | 18 | 265 |
| R3 (3.0.2, STU3) | `fhir::r3` | `r3` | 117 | 36 | 18 | 386 |
| R4 (4.0.1) | `fhir::r4` | `r4` | 146 | 43 | 20 | 486 |
| R5 (5.0.0) | `fhir::r5` | `r5` (default) | 158 | 50 | 21 | 442 |
| R6 (6.0.0-ballot3) | `fhir::r6` | `r6` | 161 | 51 | 21 | 459 |

R6 is a **ballot draft**: unpublished as a specification, off by default, and
outside this crate's semver promise until HL7 finalizes it (it *is* published
to crates.io, because the facade's optional dependency needs a registry
version — see `spec/12-fhir-releases.md`, R12.14a). This chapter, like the rest
of the guide, otherwise focuses on R5, R4, and R3; everything applies to R2 and
R6 identically once their features are enabled.

## Enabling a release

Each release is a complete model of roughly 90,000–240,000 lines of generated
Rust, so you compile only what you use. `r5` is on by default:

```toml
[dependencies]
# R5 only (the default)
fhir = "3"

# R5 plus older releases
# fhir = { version = "3", features = ["r4"] }
# fhir = { version = "3", features = ["r3", "r4"] }

# One older release on its own
# fhir = { version = "3", default-features = false, features = ["r3"] }

# The ballot draft and the oldest modelled release
# fhir = { version = "3", features = ["r6"] }
# fhir = { version = "3", default-features = false, features = ["r2"] }
```

Each release you enable adds its own compile time, which is why none is implied
by another.

## Every release's model is the same shape

Every module one release exposes has a counterpart in every other, so moving
code between releases means changing one path segment:

```rust
use fhir::r5::resources::Patient;
use fhir::r5::codes::AdministrativeGender;
```

```rust
use fhir::r4::resources::Patient;
use fhir::r4::codes::AdministrativeGender;
```

```rust
use fhir::r3::resources::Patient;
use fhir::r3::codes::AdministrativeGender;
```

Everything in this guide — builders, validation, choice enums, `Coded<E>`,
extensions, bundles, summary serialization — works identically across
releases.

## …but they are not the same types

`fhir::r3::resources::Patient`, `fhir::r4::resources::Patient` and
`fhir::r5::resources::Patient` are three distinct Rust types, and there is
deliberately no conversion between them.

The releases genuinely disagree, and not merely by growing:

- `Observation.value[x]` admits **eleven** types in R3 and **eleven** in R4 —
  but not the same eleven. R3 allows `Attachment` and not `integer`; R4
  reversed both; R5 allows all of them plus `Reference`.
- A resource's `id` is typed `id` in R3, `string` in R4 and R5.
  `Extension.url` is a `uri` in R3, a `string` afterwards.
- `MedicationRequest.medication[x]` is a choice element in R4 and a
  `CodeableReference` in R5.
- R3 has no `canonical` or `url` primitive. R4 has no `integer64`,
  `CodeableReference`, or `RatioRange` datatype. R5 has no `Contributor`,
  `Population`, or `SubstanceAmount`.
- `Bundle.link.relation` is a free `string` in R4 and a bound code in R5.

A single Rust type standing for all of them would have to be either their union
(accepting data that is invalid in *every* release, and letting an R5-only
element be written to an R3 server) or their intersection (silently dropping
data that is valid in the release it came from). Both failures are silent, and
both corrupt health records. Distinct types turn the mismatch into a compile
error.

## Converting between releases

**Do not go through plain serde.** `serde_json::to_value` then `from_value`
into the target release's type used to be this guide's advice, and it is
wrong: serde reports only the *first* mismatch and stops, and — the case that
matters here — an element the target release does not have is not a mismatch
at all. Unknown keys are silently ignored, so the field disappears and nothing
is said. That is exactly the commonest difference between two FHIR releases.

Use [`fhir::convert`](https://docs.rs/fhir/latest/fhir/convert/index.html)
instead. It converts the JSON wire form using both releases' generated element
tables and hands back a `LossReport` naming everything it changed or
discarded — nothing is dropped silently:

```rust
use fhir::convert;
use fhir::r4::R4;
use fhir::r5::R5;

let r4_patient = serde_json::json!({
    "resourceType": "Patient",
    "id": "pat-1",
    "active": true
});

let out = convert::between::<R4, R5>(&r4_patient);
assert!(out.report.is_lossless(), "{}", out.report); // Patient's shape is stable here
let r5_patient: fhir::r5::resources::Patient = serde_json::from_value(out.value).unwrap();
```

When it is *not* lossless, `out.report` names the path and reason for every
loss (`LossKind::ElementRemoved`, `ChoiceVariantUnsupported`,
`CardinalityNarrowed`, `RequiredMissing`, …) instead of leaving you to
discover it the hard way. `convert::strict::<S, T>` is the same conversion but
refuses (returns `Err(LossReport)`) unless it is lossless — useful when a
dropped element should stop the exchange rather than proceed quietly.
`convert::from_typed::<S, T>` takes a typed `Resource` enum value instead of
raw JSON (needed because `resourceType` comes from that enum's serde tag, not
from a bare resource struct). There is no `From`/`Into` between release types,
and there will not be one — the report is the point, and a conversion the
compiler performed silently could not give you one. See
`cargo run --example r4_and_r5_side_by_side --features "r4 r5"` for a full
worked version, including a loss (an R5 `Observation.valueAttachment` has
nowhere to go in R4), and [spec 14](https://github.com/fhir-rust/fhir-rust/blob/main/spec/14-cross-release-conversion.md)
for the rules.

The `meta` tables tell you where the releases differ, without guessing:

```rust
let r3_value = fhir::r3::meta::element("Observation.value[x]").unwrap();
let r4_value = fhir::r4::meta::element("Observation.value[x]").unwrap();

// Same count, different contents.
assert_eq!(r3_value.types.len(), r4_value.types.len());

let dropped: Vec<&str> = r3_value
    .type_codes()
    .filter(|code| !r4_value.type_codes().any(|c| c == *code))
    .collect();
assert_eq!(dropped, ["Attachment"]);   // R4 removed it, R5 brought it back
```

## What the releases share

Anything that does not name a release's types is defined once at the crate root
and re-exported by each, so `fhir::r3::validate::Validate`,
`fhir::r4::validate::Validate` and `fhir::r5::validate::Validate` are the
**same trait**:

| Crate root | Purpose |
| --- | --- |
| `fhir::validate` | The `Validate` trait and `ValidationIssue` |
| `fhir::coded` | `Coded<E>`, the required-binding wrapper |
| `fhir::builder` | `BuilderError` |
| `fhir::meta` | The element-metadata table types and lookups |
| `fhir::temporal` | Date/time parsing and precision-aware comparison |
| `fhir::summary` | `_summary=true` pruning |
| `fhir::xml` | The FHIR XML bridge (feature `xml`) |
| `fhir::client` | The async REST client (feature `client`) |
| `fhir::release` | The `Release` trait |

Because the trait is shared, one function can validate values of any release:

```rust
use fhir::validate::Validate;

fn check<T: Validate>(value: &T) -> bool {
    value.validate().is_empty()
}
```

The REST client is generic over `Release` for the same reason.
`fhir::r3::client::Client`, `fhir::r4::client::Client` and
`fhir::r5::client::Client` are three aliases for one implementation, each
returning its own release's `Resource`, `Bundle`, and `OperationOutcome`.
