# 09 — Primitive extensions (`_field`)

Defines how the crate represents FHIR® *primitive extensions* — the `_field`
siblings that carry `id` and `extension` for a primitive value.

Applies to every modelled release, identically.

## Background: how FHIR JSON represents primitive extensions

In FHIR, every element — including a *primitive* such as `birthDate` or
`gender` — may carry an `id` and `extension`s. JSON has no place to hang those
on a bare scalar, so FHIR splits them into a **sibling property** named with a
leading underscore ([FHIR §JSON representation of primitive
elements](https://hl7.org/fhir/R5/json.html#primitive)):

```json
{
  "birthDate": "1970-03-25",
  "_birthDate": {
    "extension": [{
      "url": "http://hl7.org/fhir/StructureDefinition/patient-birthTime",
      "valueDateTime": "1970-03-25T14:35:00-05:00"
    }]
  }
}
```

Rules any representation must honour:

1. The value (`birthDate`) and its extension (`_birthDate`) are **two sibling
   keys** on the *parent* object, not a nested object.
2. Either may appear **without** the other: a value with no extension, or an
   extension with no value (a data-absent-reason, say).
3. For a **repeating** primitive, the two arrays are **positionally aligned**
   and JSON `null` fills the gaps:

   ```json
   { "given": ["Peter", "James"], "_given": [null, {"id": "n2"}] }
   ```

## Requirements

- **R9.1** For every primitive-typed element `x` — including primitive `value[x]`
  choice variants such as `deceasedBoolean` — the owning struct MUST have a
  sibling field `x_ext` typed by that release's
  [`Element`](../fhir-r5/src/types/element.rs) (`{ id, extension }`):

  ```rust
  pub birth_date: Option<types::Date>,
  #[serde(rename = "_birthDate")]
  pub birth_date_ext: Option<types::Element>,          // scalar

  pub given: Vec<types::String>,
  #[serde(rename = "_given")]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub given_ext: Vec<Option<types::Element>>,          // repeating (null-aligned)
  ```

- **R9.1a** *(added 2026-08-10, audit **F-86**)* The **value array** of a
  repeating primitive is `::fhir_core::PrimVec<T>` (R6.7a): a `null`
  position is an extension-only placeholder whose `Element` lives at the
  same index of the `_x` sibling. Constructing a placeholder with no
  matching sibling entry is representable but meaningless; the serializer
  writes the `null` faithfully rather than guessing.

- **R9.2** Scalar primitives use `Option<Element>`; repeating primitives use
  `Vec<Option<Element>>` — an empty `Vec` means "no `_field` at all", and the
  inner `Option` is the per-position `null`.
- **R9.3** The sibling MUST carry an explicit
  `#[serde(rename = "_<fhirName>")]`. This is mandatory, not cosmetic: the
  struct-level `rename_all = "camelCase"` would otherwise map `birth_date_ext`
  to `birthDateExt`, and would map a naive `_birth_date` to `birthDate`,
  colliding with the value field.
- **R9.4** The sibling MUST NOT be emitted when absent —
  `skip_serializing_none` for the scalar form, `skip_serializing_if =
  "Vec::is_empty"` for the repeating one — so extension-free data serializes
  unchanged.
- **R9.5** FHIR infrastructure elements — every `<Type>.id`, and
  `Extension.url` — MUST NOT get a sibling: they are bare JSON attributes with
  no `_id`/`_url`. R4 and R5 say so by typing them as a FHIRPath system type
  (`http://hl7.org/fhirpath/System.String`); R3 predates that convention and
  types them as an ordinary `string`, `id` or `uri`, so the rule MUST be applied
  structurally rather than read off the type code.
- **R9.6** `#[derive(Validate)]` MUST recurse into `_field` `Element`s so their
  nested extensions are validated.
- **R9.7** Non-primitive elements are unaffected: complex datatypes model
  `id`/`extension` on the type itself.

## Design rationale

The alternative considered was a single field holding value, `id`, and
`extension` together:

```rust
pub struct FhirField<T> { pub value: Option<T>, pub id: Option<String>, pub extension: Vec<Extension> }
pub birth_date: FhirField<Date>,
```

This is more cohesive in Rust but **cannot** serialize with a plain derive:
serde maps one struct field to one JSON key, whereas FHIR requires `birth_date`
to become **two** sibling keys on the parent. Making it produce valid FHIR JSON
would need either a hand-written `Serialize`/`Deserialize` for every container —
abandoning the uniform derive recipe across every struct in the model — or a
bespoke proc-macro re-implementing serde's field handling for the `x` / `_x`
split. It would also bury the common case (a plain value) inside a wrapper.

The sibling representation works with the crate's existing
`#[derive(Serialize, Deserialize)]` + `skip_serializing_none` + `rename_all`
recipe with **zero** custom serde, and is purely additive. Its ergonomic
cost — a separate `<field>_ext` field — is small, and is smoothed by the
`ExtensionExt` accessors without changing the wire model.

## Acceptance criteria

1. Every primitive-typed element in every release has its `_field` sibling.
2. The scalar, extension-only, mandatory-primitive, and repeating (null-aligned)
   cases round-trip with exact `serde_json::Value` equality —
   `tests/primitive_extensions_prototype.rs` covers all four.
3. Official example resources carrying `_birthDate`, `_family`, `_given`,
   `_profile` and similar round-trip without loss.
4. An element typed `http://hl7.org/fhirpath/System.String` has no sibling.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
