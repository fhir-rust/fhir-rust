# CLAUDE.md

This file exists so Claude Code (and other coding agents) pick up the project's
guidance automatically. To keep a **single source of truth**, it does not repeat
that guidance — it points at it.

## Read these, in order

1. [`AGENTS.md`](AGENTS.md) — what the project is, the commands, the green gate,
   and the house rules. **Start here.**
2. [`AGENTS/`](AGENTS/architecture.md) — operational detail:
   [architecture](AGENTS/architecture.md),
   [conventions](AGENTS/conventions.md),
   [code generation](AGENTS/code-generation.md),
   [testing](AGENTS/testing.md), and the [glossary](AGENTS/glossary.md).
3. [`spec/index.md`](spec/index.md) — the **living specifications**, which are
   the source of truth for behaviour. Code and spec must not drift; when they
   disagree, reconcile them.

## The one rule that matters most

Keep the workspace **green** before considering any task done:

```sh
cargo build --all-targets
cargo test                                    # unit tests + doctests
cargo clippy --all-targets -- -D warnings     # zero warnings (pedantic is on)
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## One crate per FHIR release

This is a workspace, not a single crate:

| Crate | What it holds |
| --- | --- |
| `fhir-core` | Everything release-independent: `Decimal`, `Validate`, `Coded<E>`, builders, temporal parsing, XML, the REST client |
| `fhir-release-2` … `fhir-release-6` | One FHIR release each, ~90k–240k generated lines |
| `fhir` | The facade: re-exports the above behind features `r2`…`r6` |
| `fhir-release-7`, `fhir-release-8`, `fhir-release-9` | Name reservations. No such specifications exist; they contain no model |
| `fhir-derive-macros` | `#[derive(Validate)]`, `Builder`, `FhirChoice` |

The models are separate crates so that compiling all of them is several
`rustc` processes rather than one: peak memory is the largest crate, not the
sum. Before the split, `--features "r3 r4"` needed **12.9 GB**; after, **5.0 GB**.

Only `r5` is on by default, so the commands above see R5 alone. If you touched
the generator, the derive macros, `fhir-core`, or any release crate, run the
gate with the others enabled too:

```sh
cargo test --features "r2 r3 r4 r6 xml client"
cargo clippy --all-targets --features "r2 r3 r4 r6 xml client" -- -D warnings
```

## Which tree you are editing

- **`fhir-release-2/src` … `fhir-release-4/src` are generated.** Change `src/codegen/`,
  then `cargo run -- r2` … `cargo run -- r4`.
- **`fhir-release-5/src` is hand-documented.** Never regenerate over it; `cargo run
  -- r5` refuses without an explicit `--out`.
- **`fhir-release-6/src` is generated from a ballot draft** (6.0.0-ballot3). It is
  `publish = false` and outside the semver promise until R6 is final.
- **Twelve modules per release are hand-maintained**, not generated: `builder`,
  `bundle_util`, `choice`, `client`, `coded`, `lib`, `meta`, `prelude`,
  `summary`, `temporal`, `validate`, `xml`.

Adding a release is a documented procedure:
[`doc/adding-a-release.md`](doc/adding-a-release.md).

Everything else — the conventions, the cardinality mapping, the generator, the
release checklist — lives in the documents linked above.
