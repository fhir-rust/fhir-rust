# 08 — Code generation

Defines the spec-JSON → Rust generator: the engine that makes this project
spec-driven. Operational how-to is in
[`../AGENTS/code-generation.md`](../AGENTS/code-generation.md).

## Background

Every FHIR release is published as StructureDefinition JSON bundles. The
generator reads those bundles and emits Rust source, so a model can be
(re)derived from the upstream truth rather than hand-maintained.

The bundles have the same structure in every release, which is what makes one
generator able to produce them all.

## Inputs

- **R8.1** The generator MUST read from
  `doc/fhir-specifications/<release>/fhir-definitions-json/`:
  `profiles-types.json` (datatypes), `profiles-resources.json` (resources),
  `valuesets.json` (code systems), and the supporting bundles
  (`conceptmaps.json`, `search-parameters.json`, `dataelements.json`,
  `profiles-others.json`).
- **R8.2** Bundles carrying terminologies FHIR does not itself define
  (R4's `v2-tables.json` and `v3-codesystems.json`) MUST NOT be read: no FHIR
  element has a `required` binding into them, so they would add enums nothing
  refers to.

## Release parameterization

- **R8.3** Every generator input and output that varies by release MUST be
  reachable from `codegen::Version`: the definition directory, the output
  directory, the module name, the release label, and the specification URL.
  Release-specific behaviour elsewhere is a defect.
- **R8.4** The generator MUST emit the *finished* module tree in one pass —
  nested backbone structs, choice enums, `Coded<E>`, `Vec1`, primitive-extension
  siblings, builders, and per-module tests — not an intermediate that needs
  hand-finishing.
- **R8.5** The generator MUST refuse to overwrite a hand-documented release
  tree. `fhir-r5/src` is such a tree; writing it requires an explicit output
  directory.

## Element → field mapping

- **R8.6** The generator MUST implement the serialization rules of spec 06:
  1. Skip the root element (a path with no `.`).
  2. Map the FHIR type code to a Rust type: a FHIRPath system primitive
     `http://hl7.org/fhirpath/System.X` becomes `types::X`; `Resource` and
     `DomainResource` become `::serde_json::Value`; anything else becomes
     `types::Pascal(code)`.
  3. Apply cardinality (spec 06, R6.6).
  4. Expand `[x]` choices into an enum (spec 06, R6.10; spec 11).
  5. snake_case names and escape Rust keywords (spec 06, R6.9), adding an
     explicit serde rename where camelCase cannot recover the FHIR name
     (spec 06, R6.3).
  6. Emit the `_field` primitive-extension siblings (spec 09).
  7. Retype `required`-binding codes to `Coded<Enum>` (spec 05, R5.7).

## Structural decisions the mapping cannot make element-by-element

- **R8.7** An element MUST become a nested struct if, and only if, other
  elements have it as a path prefix. Deciding this from the declared type code
  is not reliable: datatype backbones are typed `Element` and resource
  backbones `BackboneElement`, and neither spelling is used consistently across
  releases.
- **R8.8** An element with a `contentReference` MUST be typed as the struct it
  points at (spec 04, R4.4).
- **R8.9** A definition's Rust name MUST come from its `name`, not its `type`,
  so that profiles do not collide (spec 03, R3.7).
- **R8.10** Type cycles MUST be broken deterministically with `Box`
  (spec 03, R3.8), and which structs can derive `Default` MUST be settled across
  the whole model at once, since a mandatory field of a default-less struct
  inherits the problem (spec 06, R6.1).

## Determinism

- **R8.11** Generation MUST be deterministic: identical input yields
  byte-identical output. No dependence on hash-map iteration order, timestamps,
  or randomness. A second run MUST produce no diff.
- **R8.12** A file whose content is unchanged MUST NOT be rewritten, so that
  timestamps — and therefore rebuilds — stay stable.

## Code-system generation

- **R8.13** Each release's `codes` MUST be generated from its `valuesets.json`
  per spec 05 (complete CodeSystems, sanitized identifiers, serde renames,
  wrapped doc URLs).

## Legacy parse layer (R5 only)

`r5::parse` is the original R5-only generator, which produced a *starting point*
for hand-finishing in `tmp/out/` rather than a finished model. It is retained
because the shipped R5 modules were authored through it, and its splicing
generators are still how R5 is edited in bulk. New work goes in `codegen`.

- **R8.14** Each bundle MUST have an `r5::parse::<bundle>` module whose structs
  (`Bundle`, `Entry`, `Resource`, `Element`, `Snapshot`, …) deserialize the
  spec JSON faithfully. A `test_serde_json_from_reader` test MUST deserialize
  the real bundle; a missing field (serde `unknown field`) is a defect to fix.
- **R8.15** A splicing generator MUST edit the existing documented files in
  place, MUST be idempotent, and MUST NOT regenerate them wholesale.

## Future work

- Retire the legacy `r5::parse` layer once R5's prose can be carried across a
  regeneration — for example by holding the hand-written documentation outside
  the generated files.
- ~~Generate typed `Reference<T>` fields from each element's
  `targetProfile`~~ — done 2026-08-09 (T11; spec 04).

## Acceptance criteria

1. `cargo run -- r4` regenerates `fhir-r4/src` without error, and a second run
   produces no diff.
2. `cargo run -- r5` without `--out` exits non-zero and writes nothing.
3. Generated field types, cardinalities, choice expansions, keyword escaping and
   serde renames match spec 06.
4. Generated backbone elements are named nested structs, never flattened.
5. The generated model passes the green gate, and the official examples
   round-trip (spec 12, acceptance 7).
