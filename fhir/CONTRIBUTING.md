# Contributing to `fhir`

Thanks for your interest in improving the `fhir` crate — a Rust implementation
of the HL7 FHIR® data model (releases R5, R4 and R3) plus the spec-driven generator
that produces it.

This file is the short version. The authoritative, in-depth guidance lives in
[`AGENTS.md`](AGENTS.md) and the [`AGENTS/`](AGENTS/) folder; the living
specifications are in [`spec/`](spec/).

## Prerequisites

- Rust **1.88** or newer (the crate uses edition 2024; MSRV is pinned via
  `rust-version` in `Cargo.toml`).
- Install the standard components: `rustup component add clippy rustfmt`.

## Build, test, lint — the green gate

Before you consider any change finished, all of these must pass:

| Task | Command |
| --- | --- |
| Build | `cargo build --all-targets` |
| Test (unit + integration) | `cargo test` |
| Doctests | `cargo test --doc` |
| Lint (pedantic; **zero** warnings) | `cargo clippy --all-targets -- -D warnings` |
| Docs (**zero** warnings) | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` |

The same checks run in CI (`.github/workflows/ci.yml`). `clippy::pedantic` is on
crate-wide, so keep it clean.

### Round-trip tests against the official examples

The always-on `roundtrip_curated_subset` test round-trips a committed set of
official examples (`tests/data/roundtrip_examples_<release>/`). To run the full
~2800-file official set:

```sh
bin/fetch-examples r5   # downloads + unpacks ~190 MB into a git-ignored dir
cargo test --test roundtrip_r5_examples -- --ignored --nocapture

bin/fetch-examples r4
cargo test --features r4 --test roundtrip_r4_examples -- --ignored --nocapture

bin/fetch-examples r3
cargo test --features r3 --test roundtrip_r3_examples -- --ignored --nocapture
```

Not every official example round-trips, and that is not always our bug: 198 of
2911 official R4 examples and 174 of 1693 R3 ones omit an element their own
release makes mandatory. Check the specification's cardinality before weakening
a type to accept one.

Known gaps are tracked in [`tasks-roundtrip-failures.md`](tasks-roundtrip-failures.md).

## The code generator

The model under `src/<release>/{types,resources,codes.rs}` is **generated** from
that release's official FHIR specification JSON in
`doc/fhir-specifications/<release>/`. The generator lives in `src/codegen/` and
is driven by the thin binary in `src/main.rs`. `src/r4` is entirely generated;
`src/r5` carries hand-written prose and must not be regenerated over:

```sh
cargo run -- r4                    # rewrite src/r4 from the R4 definitions
cargo run -- r5 --out tmp/out/r5   # emit R5 elsewhere, to compare
```

**Prefer changing the generator over hand-editing generated shapes.** When you
must make a mechanical edit across many datatype/resource modules, use
Read+Edit-only tooling or regenerate — see [`AGENTS/code-generation.md`](AGENTS/code-generation.md).
`tmp/out/` is tracked generator output; regenerate it, do not hand-edit.

## Conventions

- **The specs in `spec/` are the source of truth.** Behaviour is defined there
  first, then implemented. When code and spec disagree, fix the mismatch.
- **Struct/serde conventions are uniform:** `rename_all = "camelCase"`,
  `skip_serializing_none`, and the FHIR cardinality → Rust type mapping
  (`0..1`→`Option<T>`, `1..1`→`T`, `0..*`→`Option<Vec<T>>`, `1..*`→`Vec<T>`).
  See [`AGENTS/conventions.md`](AGENTS/conventions.md).
- **Documentation:** every datatype/resource module carries long-form FHIR prose
  doc comments; match the surrounding style. Add a doctest for anything with a
  runtime surface.
- **Dependencies are deliberately lean** (`serde`, `serde_json`, `serde_with`,
  `indoc`, `convert_case`, and the local `fhir-derive-macros`). Do not add
  dependencies without cause.

## Pull requests

- Keep changes small and verifiable; add a test or doctest for new behaviour.
- Ensure the full green gate passes locally.
- End agent-authored commit messages with a `Co-Authored-By` trailer.

## License

By contributing, you agree that your contributions are licensed under the same
terms as the project (see [`LICENSE.md`](LICENSE.md)).

FHIR® is a registered trademark of Health Level Seven International. This crate
is not affiliated with or endorsed by HL7.
