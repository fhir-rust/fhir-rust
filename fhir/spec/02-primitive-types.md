# 02 — Primitive types

Defines how the FHIR primitive datatypes are represented in Rust.

Applies to every modelled release. Where a release differs, the difference is
stated here rather than in the code.

| Release | Primitives |
| --- | --- |
| R6 | 21 |
| R5 | 21 |
| R4 | 20 (no `integer64`) |
| R3 | 18 (no `integer64`, `canonical` or `url`) |
| R2 | 18 (no `integer64`, `canonical` or `url`) |

## Background

FHIR primitives are single scalar values with a lowercase initial letter. In
FHIR JSON they serialize as bare scalars — a JSON string, number, or boolean —
**not** as objects.

## Requirements

- **R2.1** Each primitive MUST be a Rust **newtype** wrapping the smallest
  faithful inner type, so it serializes transparently as a bare scalar:

  | FHIR primitive | Rust |
  | --- | --- |
  | `string`, `code`, `id`, `markdown`, `uri`, `url`, `canonical`, `oid`, `uuid`, `base64Binary`, `xhtml` | `struct X(pub String)` |
  | `date`, `dateTime`, `instant`, `time` | `struct X(pub String)` |
  | `boolean` | `struct Boolean(pub bool)` |
  | `integer` | `struct Integer(pub i32)` |
  | `positiveInt`, `unsignedInt` | `struct X(pub u32)` |
  | `integer64` (R5/R6 only) | `struct Integer64(pub i64)` — serialized as a JSON **string** |
  | `decimal` | `struct Decimal` — lexical form preserved verbatim (R2.2) |

  `canonical` and `url` arrived in R4; `integer64` in R5. A release simply does
  not generate a primitive it does not define.

- **R2.2** `decimal` MUST preserve the **lexical form** of the value it was
  given — every significant digit, including trailing zeros — and satisfy `Eq`.
  Its `Default` is zero, the one primitive whose `Default` cannot be derived.

  FHIR treats decimal precision as clinically meaningful: `0.50` mmol/L states
  two significant figures and `0.5` states one, and a dose of `1.000` mg is a
  different claim from `1.0` mg. A representation that normalizes them is
  discarding information the sender chose to send.

  `struct Decimal(pub serde_json::Number)` does **not** satisfy this in the
  crate's default configuration. Without `serde_json`'s `arbitrary_precision`,
  `Number` is backed by `f64`, and observed behaviour is:

  | Input | Re-serialized |
  | --- | --- |
  | `0.50` | `0.5` |
  | `1.000` | `1.0` |
  | `0.1234567890123456789012345` | `0.12345678901234568` |
  | `12345678901234567890.5` | `1.2345678901234567e+19` |

  The crate therefore enables `serde_json/arbitrary_precision` as a
  **non-optional dependency feature**, so a `Number` carries the lexeme it was
  parsed from. Cargo features are additive and a dependent cannot switch one
  off, which is what turns precision from a default into a guarantee — the
  former `precise-decimal` opt-in left correctness depending on whether some
  unrelated crate in the graph happened to enable the same feature, and
  correctness that arrives by luck is not correctness.

  Two alternatives were tried and rejected:

  - **`serde_json::value::RawValue`**, storing the lexeme as a string and
    emitting it as a raw number token. It preserves precision on `from_str`
    but **fails through `#[serde(flatten)]`**, which every `value[x]` choice
    element uses (spec 11): `flatten` buffers input through serde's `Content`,
    which has no representation for a raw token, so the whole choice variant
    is silently dropped. Losing `Observation.valueQuantity` entirely is far
    worse than rounding it.
  - **Leaving the opt-in and documenting it**, which keeps the failure mode
    silent and the default wrong.

  `Decimal` is consequently a hand-written wrapper (`crate::decimal`, shared by
  every release rather than generated once per release) presenting a lexical API —
  `new`, `as_str`, `as_f64`, lexical `Eq`, numeric `PartialOrd` — over a
  precision-preserving `Number`.

  The cost is stated rather than hidden: `arbitrary_precision` is global to the
  compiled binary, so every crate's `serde_json::Number` in that build becomes
  lexeme-preserving and `Number` arithmetic goes through `as_f64()`. For a
  library whose numbers are doses and lab results, that is the correct side to
  err on. See spec 13.
- **R2.2a** `Decimal` MUST offer explicit, lossy-by-request conversions
  (`as_f64`, `to_string`, and a `PartialOrd` that compares numerically rather
  than lexically), so that `1.0` and `1.00` compare equal in value while
  remaining distinguishable on the wire. Equality is lexical; ordering is
  numeric; both are documented at the type.
- **R2.3** `integer64` MUST serialize and deserialize as a JSON **string**
  (FHIR encodes 64-bit integers as strings so they survive consumers whose
  numbers are 64-bit floats). Implemented with `serde_with`'s `DisplayFromStr`.
- **R2.4** Every primitive MUST derive `Debug, Default, Clone, PartialEq, Eq`
  and be `serde` (de)serializable. No primitive may contain `f64`/`f32`.
- **R2.5** Each primitive lives in `fhir-release-N/src/types/<snake>.rs` and is
  re-exported from `fhir-release-N/src/types.rs` as `pub use <snake>::<Pascal>;`.
- **R2.6** Each primitive MUST implement `Validate` (spec 07) with its FHIR
  format constraint where one exists (`code`, `id`, `oid`, `uuid`, `uri`,
  `canonical`, `url`); the rest are structurally valid by construction.
- **R2.7** The Rust representation is a design decision the specification JSON
  does not state, so it MUST live in one table
  (`codegen::primitives::PRIMITIVES`) shared by every release. A release that
  defines a primitive absent from that table MUST fail generation loudly rather
  than be guessed at.

## Representation notes

- A single-field tuple struct is serialized by serde **as its inner value**, so
  no `#[serde(transparent)]` attribute is required.
- Inside `string.rs`, refer to the standard library type as
  `std::string::String` to avoid shadowing by the newtype.

## Rationale

Newtypes rather than type aliases give each primitive its own `Validate` impl
and prevent a `Code` being passed where an `Id` is meant, at no runtime cost —
the wire form is identical to the bare scalar.

## Future work

- Format validation covers a subset of the primitives; date/time and base64
  format checks MAY be added under spec 07.

## Acceptance criteria

1. Every primitive its release defines exists as a newtype per R2.1 and
   re-exports from `types.rs`.
2. `Decimal` round-trips `3.5` as JSON `3.5`; its `Default` is `0`.
2a. `Decimal` round-trips each of `0.50`, `1.000`, `1e-7`,
   `0.1234567890123456789012345`, and `12345678901234567890.5` **byte for
   byte**, in a default-feature build, with no `serde_json` feature enabled
   anywhere in the dependency graph.
2b. `Decimal("1.0") != Decimal("1.00")` (lexically distinct) while
   `Decimal("1.0").partial_cmp(&Decimal("1.00")) == Some(Ordering::Equal)`
   (numerically equal), per R2.2a.
3. `Integer64` (R5/R6) round-trips `9007199254740993` as the JSON string
   `"9007199254740993"`.
4. `Code("bad  code")` and `Id("bad id!")` are reported invalid by `Validate`.
5. Every primitive module passes its generated round-trip test.
