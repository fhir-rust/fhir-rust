# Code generator internals

The model under `src/<release>/{types,resources,codes.rs}` is **generated** from
that release's official FHIR specification JSON in
`doc/fhir-specifications/<release>/`. This chapter sketches how, for
contributors.

## The pipeline

The generator lives under `src/codegen/` and is driven by the thin binary in
`src/main.rs`:

```sh
cargo run -- r3                    # rewrite src/r3 from the R3 definitions
cargo run -- r4                    # rewrite src/r4 from the R4 definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 elsewhere, to compare
```

It deserializes the `StructureDefinition` bundles with serde structs mirroring
the FHIR JSON — one set for both releases, since their bundles have the same
structure — then plans each type and renders it, applying the uniform
conventions (`rename_all = "camelCase"`, `skip_serializing_none`, the
cardinality mapping).

`src/r3` and `src/r4` are entirely generated and safe to rewrite. `src/r5` is not: it carries
hand-written prose on top of generated shapes, so `cargo run -- r5` refuses to
write there without an explicit `--out`. Everything that varies by release is
reachable from `codegen::Version`.

## The metadata table

`src/codegen/meta_gen.rs` extracts a compile-time table (`fhir::r5::meta`,
`fhir::r4::meta`) of per-element facts — cardinality, coded-value bindings,
`value[x]` type lists, reference target profiles, and summary membership —
keyed by FHIR path. This
table is the foundation the later layers build on: choice enums, coded fields,
validation, and summary serialization all consult it.

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

R3 and R4 need none of this: one `cargo run -- <release>` emits the finished
shape directly.

## The green gate

Before any change is considered done, all of these must pass:

```sh
cargo build --all-targets
cargo test          # unit + integration
cargo test --doc    # doctests
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Only `r5` is on by default, so those commands do not exercise R3 or R4. If you
touched the generator, the derive macros, the crate-root modules, or a generated
tree, run the gate with those releases enabled too:

```sh
cargo test --features "r3 r4 xml client"
cargo clippy --all-targets --features "r3 r4 xml client" -- -D warnings
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
[`spec/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/spec)
are the source of truth; behaviour is defined there first, then implemented. See
[`AGENTS.md`](https://github.com/joelparkerhenderson/fhir-rust-crate/blob/main/AGENTS.md)
for the full contributor workflow.
