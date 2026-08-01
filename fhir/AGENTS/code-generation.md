# Code generation

This crate is spec-driven: the official FHIR specification JSON is the input,
Rust is the output. This document explains the generators so agents can extend
them rather than hand-writing model shapes. The normative contract is
[`../spec/08-code-generation.md`](../spec/08-code-generation.md).

There are **two** generators, for historical reasons, and it matters which one
you are working with:

| | `crate::codegen` | `crate::r5::parse` |
| --- | --- | --- |
| Releases | any (`Version::R3`, `R4`, `R5`) | R5 only |
| Emits | the **finished** module tree | a rough **starting point** |
| Output | `src/<release>/…` | `tmp/out/*.rs` |
| Used for | all of `fhir-release-3/src` and `fhir-release-4/src` | the original authoring of `fhir-release-5/src` |

New work belongs in `codegen`. `r5::parse` is kept because the shipped R5 model
was authored through it and its splicing generators are still how R5 is edited
in bulk.

## Inputs

`doc/fhir-specifications/<release>/fhir-definitions-json/`:

| File | Produces |
| --- | --- |
| `profiles-types.json` | primitive + complex datatypes |
| `profiles-resources.json` | resources |
| `valuesets.json` | code-system enums (`<release>::codes`) |
| `conceptmaps.json`, `search-parameters.json`, `dataelements.json`, `profiles-others.json` | supporting bundles |

R3 and R4 also ship `v2-tables.json` and `v3-codesystems.json`. These are
deliberately not read: they are external HL7 v2/v3 terminologies, not
FHIR-defined ones, and no FHIR element has a `required` binding into them.

The R4 and R5 bundles have **identical structure** — same `StructureDefinition`,
same `ElementDefinition`, same FHIRPath system type URLs. **R3 does not**, and
`codegen::spec` normalizes the differences at the input boundary so they never
leak downstream:

| Fact | R3 | R4 / R5 |
| --- | --- | --- |
| `type.targetProfile` | one string | a list |
| binding's value set | `valueSetReference` (a `Reference`) or `valueSetUri` | `valueSet` (canonical) |
| a primitive's own `value` element | states no `type.code` | states one |
| `<Type>.id`, `Extension.url` | ordinary `string`/`id`/`uri` | a FHIRPath system type |

The last of those is why "is this element FHIR infrastructure?" is decided
**structurally** (`ElementDefinition::is_system_element`) rather than from the
type code — otherwise every R3 struct would sprout a spurious `_id` sibling.

Missing the value-set difference would have been silent: R3 would simply have
produced no `Coded<E>` fields at all.

## Running it

```sh
cargo run -- r3                    # rewrite fhir-release-3/src from the R3 definitions
cargo run -- r4                    # rewrite fhir-release-4/src from the R4 definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 somewhere safe, to compare
```

`cargo run -- r5` with no `--out` **refuses to run**. `fhir-release-5/src` carries
hand-written prose that regeneration would destroy; emitting elsewhere and
diffing is how you check the generator against the shipped model.

## `codegen`: the release-parameterized generator

```text
codegen::version   Version: the only thing that knows one release from another
codegen::spec      permissive serde views of the definition JSON
codegen::naming    FHIR names -> Rust identifiers (the rules in conventions.md)
codegen::plan      StructureDefinition -> TypePlan (all the real decisions)
codegen::render    TypePlan -> Rust source (text assembly only)
codegen::primitives   the primitive newtype table
codegen::codes_gen    CodeSystem -> enum
codegen::meta_gen     ElementDefinition -> the meta table
codegen::extension_ext_gen   the per-release ExtensionExt impl list
```

Planning is separated from rendering so the decisions with consequences can be
tested on their own. The interesting ones:

1. **Backbone detection is structural, not nominal.** An element becomes a
   nested struct if other elements have it as a path prefix — never because its
   type code says `BackboneElement`. The releases spell datatype backbones
   (`Element`) and resource backbones (`BackboneElement`) differently, and no
   spelling is reliable across all three.
2. **`contentReference` resolves to the referenced struct.** R3 and R4 write
   `#Observation.referenceRange`, R5 a full URL with the same fragment.
3. **A definition's Rust name is its `name`, not its `type`.** A *profile*
   constrains an existing type and keeps that type's element paths:
   `MoneyQuantity`, `SimpleQuantity`, `Age`, `Distance`, `Count` and `Duration`
   are all `type: "Quantity"`. Without this they would collide on one module.
4. **Type cycles are broken with `Box`.** `Reference` holds an `Identifier`
   which holds a `Reference`; a depth-first walk in name order boxes the field
   that closes each cycle, so the choice is stable across runs. Only inline
   fields can close a cycle — a `Vec` already indirects.
5. **`Default` is settled across the whole model.** A struct with a `1..*`
   (`Vec1`) field has no default, and a `1..1` field of such a struct inherits
   the problem, so it is iterated to a fixed point rather than decided per type.
6. **Choice fields are always `Option`**, even when the specification makes them
   mandatory: a choice enum has no default, and every struct must derive one.
7. **A field spells its serde key out only when it must.** `rename_all =
   "camelCase"` covers nearly everything, but serde only uppercases after an
   underscore, so it cannot produce `truthTP` or `requestURL` from a snake_case
   identifier. `naming::needs_explicit_rename` decides.

## Determinism

Generation must be deterministic (same input → same output) so `git diff` on
a generated tree is meaningful. Do not introduce ordering that depends on hash-map
iteration, timestamps, or randomness — `codegen` sorts with `BTreeMap`/`BTreeSet`
and by name throughout, and `write_if_changed` leaves untouched files alone so
timestamps stay stable.

## Naming a release in generated code

Most generated code targets `fhir-core` (`::fhir_core::validate`,
`crate::coded`, `crate::builder`), which is release-independent. Three things
are not: the `meta` table, `types::Element`, and `choice::Primitive`. The derive
macros resolve those through a `#[fhir_version("r4")]` (or `"r3"`) attribute, which
`render::version_attribute` emits for every release except R5 (the macros'
default). If you add a release, add it to `KNOWN_VERSIONS` in
`fhir-derive-macros`.

## `r5::parse`: the legacy layer and its splicing generators

`r5::parse::<bundle>` mirrors each StructureDefinition bundle, with
`resource_into_rust(&Resource)` writing a rough Rust file to `tmp/out/`. Its
field mapping is only a starting point: **it flattens nested backbone elements**,
producing duplicate `id`/`extension` fields rather than named nested structs.

The shipped R5 model was refined from that starting point by a family of
**metadata-driven splicing generators** that edit the existing hand-documented
files in place (never blind regeneration), each driven by an `#[ignore]` test:

- `meta.rs` — emits the path-keyed table of cardinality, bindings, choice types,
  reference targets, and summary membership. It underpins the rest.
- `siblings.rs` — adds the `_field` primitive-extension siblings (spec 09).
- `choice_gen.rs` — folds `value[x]` fields into `FhirChoice` enums (spec 11).
- `coded_gen.rs` — retypes `required`-binding fields to `Coded<Enum>` (spec 05).
- `vec1_gen.rs` — retypes `1..*` fields to `vec1::Vec1<T>` (drops `Default`).
- `option_vec_gen.rs` — retypes `0..*` fields from `Option<Vec<T>>` to `Vec<T>`.

Because these generators must compile the library to run (they are lib tests), a
mass edit that breaks `#[cfg(test)]` code must be applied together with its test
fix-ups, or reverted to a compiling state first.

## Extending the generator

- Change `codegen::plan` (it improves every release at once) rather than editing
  individual output files.
- Anything that differs between releases must be reachable from
  `codegen::Version`. A `match` on the release anywhere else is a smell.
- After any generator change, regenerate **every** release it can affect:
  `cargo run -- r3 && cargo run -- r4`, run the green gate with
  `--features "r3 r4 xml client"`, then check `git diff --stat fhir-release-3/src fhir-release-4/src`
  looks like what you meant.
- Validate against reality with the official examples:
  `bin/fetch-examples r4` then
  `cargo test --features r4 --test roundtrip_r4_examples -- --ignored --nocapture`.
- A definition that fails to parse **stops** generation rather than being
  skipped. Keep it that way: silently skipping one drops a whole resource from
  the model, and this rule is what surfaced two of R3's differences immediately.
