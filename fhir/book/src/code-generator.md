# Code generator internals

Each release lives in its own crate, `fhir-r2` … `fhir-r6`. The
model under `fhir-r<n>/src/{types,resources,codes.rs}` is **generated**
from that release's official FHIR® specification JSON in
`doc/fhir-specifications/<release>/`. This chapter sketches how, for
contributors.

## The pipeline

The generator lives under `src/codegen/` (in the `fhir` facade crate) and is
driven by the thin binary in `src/main.rs`:

```sh
cargo run -- r2                    # rewrite fhir-r2/src from the R2 (DSTU2) definitions
cargo run -- r3                    # rewrite fhir-r3/src from the R3 definitions
cargo run -- r4                    # rewrite fhir-r4/src from the R4 definitions
cargo run -- r6                    # rewrite fhir-r6/src from the R6 ballot definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 elsewhere, to compare
```

It deserializes the `StructureDefinition` bundles with serde structs mirroring
the FHIR JSON — one set for every release, since their bundles have the same
structure — then plans each type and renders it, applying the uniform
conventions (`rename_all = "camelCase"`, `skip_serializing_none`, the
cardinality mapping).

`fhir-r2/src`, `fhir-r3/src`, `fhir-r4/src`, and
`fhir-r6/src` are entirely generated and safe to rewrite.
`fhir-r5/src` is not: it carries hand-written prose on top of generated
shapes, so `cargo run -- r5` refuses to write there without an explicit
`--out`. Everything that varies by release is reachable from
`codegen::Version`.

## The metadata table

`src/codegen/meta_gen.rs` extracts a compile-time table (`fhir::r5::meta`,
`fhir::r4::meta`, and so on per release) of per-element facts — cardinality,
coded-value bindings, `value[x]` type lists, reference target profiles, and
summary membership — keyed by FHIR path. This table is the foundation the
later layers build on: choice enums, coded fields, validation, and summary
serialization all consult it.

## Splicing generators (R5 only)

R5 predates the one-pass generator: its modules were authored by refining rough
output and then documenting it by hand. Editing them in bulk therefore uses a
second family of generators under `src/r5/parse/` that **splice into** the
already-documented files rather than regenerating them. Each is driven by an
ignored test and the metadata table:

- `siblings.rs` — the `_field` primitive-extension siblings, and the `Element`
  base (`id`/`extension`) on complex datatypes.
- `choice_gen.rs` — the `value[x]` choice enums (`#[derive(FhirChoice)]`).
- `coded_gen.rs` — `required`-binding fields retyped to `Coded<Enum>`.

R2, R3, R4, and R6 need none of this: one `cargo run -- <release>` emits the
finished shape directly.

## The green gate

Before any change is considered done, all of these must pass:

```sh
cargo build --all-targets
cargo test                                    # unit tests + doctests (root package)
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Only `r5` is on by default, so those commands do not exercise the other
releases. Each release crate also carries its own several-hundred doctests,
which run only when that crate is named directly:

```sh
for r in 2 3 4 6; do cargo test -p fhir-r$r --doc; done
```

If you touched the generator, the derive macros, `fhir-core`, or a generated
tree, run the gate with the other releases enabled too:

```sh
cargo test --features "r2 r3 r4 r6 xml client"
cargo clippy --all-targets --features "r2 r3 r4 r6 xml client" -- -D warnings
```

## Reading a release the specification spells differently

R3 predates several conventions R4 and R5 rely on, so the input layer
(`codegen::spec`) normalizes them rather than letting the difference leak
downstream: `targetProfile` is a single string in R3 and a list afterwards; a
binding's value set is `valueSetReference`/`valueSetUri` in R3 and a canonical
`valueSet` afterwards; and R3 does not mark infrastructure elements
(`<Type>.id`, `Extension.url`) with a FHIRPath system type, so which elements
those are is decided structurally.

The living specifications in
[`spec/`](https://github.com/fhir-rust/fhir-rust/tree/main/spec)
are the source of truth; behaviour is defined there first, then implemented. See
[`AGENTS.md`](https://github.com/fhir-rust/fhir-rust/blob/main/AGENTS.md)
for the full contributor workflow.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
