# fhir-derive-macros

The procedural macros behind the [`fhir`](https://crates.io/crates/fhir) data
model.

## Install

You almost certainly want [`fhir`](https://crates.io/crates/fhir) rather than
this crate directly — it re-exports the derives already.

```toml
[dependencies]
fhir-derive-macros = "1.2"
```

## What it generates

| Derive | Generates |
| --- | --- |
| `Validate` | a validator that walks every field recursively, including FHIR® invariants such as `qty-3` |
| `Builder` | a builder for the generated resource and datatype structs |
| `FhirChoice` | the `value[x]` choice-element representation, as an enum |

## Why derives at all

The model is generated: roughly 135,000 lines of Rust per FHIR release, across
five releases. Hand-writing three impls per type is not an option at that
scale, and a hand-written impl that drifted from its struct would be a
validation gap nobody could see.

## Invariants

`Validate` enforces structural rules — cardinality, required elements, choice
exclusivity — plus the FHIR invariants that can be checked without a
terminology server. `qty-3` (a `Quantity` with a `code` must have a `system`)
is one of them, and it applies to `Quantity` and each of its specializations:
`SimpleQuantity`, `MoneyQuantity`, `Age`, `Count`, `Distance`, `Duration`.

Terminology validation — is this code actually in that value set — is
deliberately **out of scope**; it needs a server this crate does not talk to.

## Further reading

- [`fhir`](https://crates.io/crates/fhir) · [`fhir-core`](https://crates.io/crates/fhir-core)
- [Validation spec](https://github.com/fhir-rust/fhir-rust/blob/main/fhir/spec/07-validation.md) — `R7.x`
- [Invariant coverage](https://github.com/fhir-rust/fhir-rust/blob/main/fhir/spec/10-invariants-coverage.md) — `R10.x`, and why occurrence counts are the wrong column to read as coverage

> FHIR® is a registered trademark of HL7® International. This crate is not
> affiliated with or endorsed by HL7.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
