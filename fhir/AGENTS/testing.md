# Testing

The crate must stay **green**: `cargo build`, `cargo test`, and
`cargo clippy --all-targets` all clean, with `clippy::pedantic` enabled. This
document describes the test patterns so new code matches them.

## The green gate

Run all three before finishing any task:

```sh
cargo build
cargo test                 # unit tests AND doctests
cargo clippy --all-targets # must print zero warnings
```

**Only `r5` is on by default, so those commands do not see R3 or R4.** If you
touched anything they use — the generator, the derive macros, `fhir-core`,
modules, `fhir-release-3/src`, `fhir-release-4/src` — run the gate with those releases enabled too:

```sh
cargo build --all-targets --features "r3 r4 xml client"
cargo test --features "r3 r4 xml client"
cargo clippy --all-targets --features "r3 r4 xml client" -- -D warnings
```

Current baseline with `--features "r3 r4 xml client"`: 1443 unit tests + 1354
doctests pass, 0 clippy warnings. A change that reduces this is a regression.
CI also enforces `cargo test --doc`, `doc -D warnings`, the MSRV (1.88), the
`client`/`xml`/`precise-decimal` feature builds, each release on its own
(`--no-default-features --features r3`), and the mdBook build.

## Unit test pattern

Every model module carries a `#[cfg(test)] mod tests` with, at minimum, a
default construction test and a serde round-trip test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    type T = Example;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
```

### Round-trip, don't hardcode JSON

Prefer the **round-trip-of-default** pattern above over asserting an exact
`json!({...})` shape. Hardcoded shapes break whenever a field's representation
changes (for example, when a required primitive field is present). Round-tripping
the default value is stable regardless of which fields are required.

**Structs with a `1..*` (`Vec1`) field have no `Default`.** For those, construct
an explicit value (with the required non-empty field populated) instead of
`T::default()`, and mark the struct's `# Examples` doctest `ignore`.

## Doctests

Doctests run **only because the crate has a library target**. Keep them:

- Real and compiling. Pseudo-code examples must use a ```` ```text ```` fence,
  not ```` ```no_run ````, or they will fail to compile.
- Import via the crate name `fhir`, e.g.
  `use fhir::r5::resources::Patient;`.
- **A doctest that names a release only runs when that release is enabled.** A
  doctest inside `fhir-release-4/src/…` is compiled out with the feature, which is why a
  release's examples belong in its own modules. A doctest on a `fhir-core` item
  must not name a release at all — write it against the generic item, defining a stand-in enum
  or struct inline if it needs one (see `src/coded.rs`).

Typical struct doctest:

```rust
/// # Examples
///
/// ```
/// use fhir::r5::types::period::Period;
///
/// let value = Period::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Period = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
```

## Validation tests

`#[derive(Validate)]` recurses through fields; nested issues get a dotted
`path`. Assert on that:

```rust
let coding = types::Coding {
    code: Some(types::Code("bad  code".to_string())), // double space is invalid
    ..Default::default()
};
assert_eq!(coding.validate()[0].path, "code.code");
```

See [`../spec/07-validation.md`](../spec/07-validation.md).

## Per-release tests

The two release models are tested the same way, from the same harness:

| | R5 | R4 | R3 |
| --- | --- | --- | --- |
| Per-module unit tests | generated into every module | ditto | ditto |
| Curated round-trip | `tests/data/roundtrip_examples_r5/` | `…_r4/` | `…_r3/` |
| Full official round-trip | `tests/roundtrip_r5_examples.rs` | `…_r4_…` | `…_r3_…` |

All three round-trip tests share `tests/common/mod.rs` and differ only in which
`Resource` enum they parse into. The curated subsets are committed and always
run; the full official sets are large, are not committed, and are `#[ignore]`:

```sh
bin/fetch-examples r4
cargo test --features r4 --test roundtrip_r4_examples -- --ignored --nocapture
```

Note that R3's archive holds only the definitional resources
(StructureDefinition, ValueSet, CodeSystem, …); STU3 publishes the clinical
examples as individual files instead.

**Not every official example round-trips, and that is not always our bug.**

| Release | Files | Round-trip | Mismatch | Reject |
| --- | ---: | ---: | ---: | ---: |
| R4 | 2911 | 2713 | 0 | 198 |
| R3 | 1693 | 1519 | 0 | 174 |

Every rejection is an example that omits an element its own release makes
mandatory: questionnaires without `Questionnaire.item.linkId`, SearchParameters
without `SearchParameter.base`, and — in R3 — 18 of R3's own primitive
`StructureDefinition`s that omit `ElementDefinition.type.code`, which R3 itself
declares `1..1`. Rejecting them is correct. **Before weakening a type to accept
a failing example, check the specification's cardinality first.**

## Generator / parse tests

Each `r5::parse::<bundle>` module has a `test_serde_json_from_reader` test that
deserializes the real spec JSON bundle from
`doc/fhir-specifications/r5/fhir-definitions-json/`. If a spec field is missing
from a parse struct, this test fails with `unknown field …` — add the field.

## Clippy notes

- `clippy::pedantic` is enabled crate-wide in `src/lib.rs`, with a small set of
  documented `#![allow(...)]` (e.g. `doc_markdown`, `wildcard_imports`,
  `module_inception`). Do not add new blanket allows without justification.
- `cargo clippy` alone does not lint `#[cfg(test)]` code; always use
  `--all-targets` so test modules are checked too.
