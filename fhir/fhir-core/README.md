# fhir-core

Everything in the [`fhir`](https://crates.io/crates/fhir) data model that does
**not** depend on which FHIR® release you are using.

The releases — `fhir-r2` through `fhir-r6` — are ~135,000
generated lines each. This crate is what they share, so a fix to `Decimal` or
to validation is made once rather than five times.

## Install

You usually want the [`fhir`](https://crates.io/crates/fhir) facade instead; it
re-exports what you need behind the `r2`…`r6` features and picks the release
crates up for you.

```toml
[dependencies]
fhir-core = "3.0"
```

## What is in here

| | Purpose |
| --- | --- |
| `Decimal` | keeps the precision a value was written with — `9.60` stays `9.60`, which binary floating point cannot do |
| `Validate` | the recursive validation trait behind `#[derive(Validate)]` |
| `Coded<E>` | a code-system enum **plus** the raw string, so an unknown code survives a round trip instead of being dropped |
| builders, prelude | ergonomics over the generated types |
| temporal parsing | partial dates and times, kept verbatim (`1974-12` is not `1974-12-01`) |
| `Bundle` helpers, summary serialization | the operations that are the same in every release |
| `xml` *(feature)* | FHIR XML |
| `client` *(feature)* | an async REST client |

## Why precision is load-bearing

A FHIR `decimal` carries meaning in its trailing zeros: `9.60` mg is a
different statement of accuracy from `9.6` mg. Parsing into `f64` destroys
that, and no amount of care downstream recovers it. `Decimal` therefore keeps
the lexical form, and round-trip fidelity is a tested invariant rather than an
aspiration.

## Features

| Feature | Effect |
| --- | --- |
| `precise-decimal` | arbitrary-precision decimal arithmetic |
| `xml` | FHIR XML serialization |
| `client` | async REST client |

## Further reading

- [`fhir`](https://crates.io/crates/fhir) — the facade you probably want
- [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/fhir/spec/index.md) — 14 sections, ids `R1.x`–`R14.x`

> FHIR® is a registered trademark of HL7® International. This crate is not
> affiliated with or endorsed by HL7.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
