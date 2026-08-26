# CLAUDE.md

This file exists so Claude Code (and other coding agents) pick up the project's
guidance automatically. To keep a **single source of truth**, it does not repeat
that guidance — it points at it.

## Read these, in order

1. [`AGENTS.md`](AGENTS.md) — what the project is, the commands, the green gate,
   and the house rules. **Start here.**
2. [`agents/`](agents/architecture.md) — operational detail:
   [architecture](agents/architecture.md),
   [conventions](agents/conventions.md),
   [code generation](agents/code-generation.md),
   [testing](agents/testing.md), and the [glossary](agents/glossary.md).
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

## One crate per FHIR® release

This is a workspace, not a single crate:

| Crate | What it holds |
| --- | --- |
| `fhir-core` | Everything release-independent: `Decimal`, `Validate`, `Coded<E>`, builders, temporal parsing, XML, the REST client |
| `fhir-r2` … `fhir-r6` (incl. `fhir-r4b`) | One FHIR release each, ~90k–240k generated lines |
| `fhir` | The facade: re-exports the above behind features `r2`…`r6` |
| `fhir-r1`, `fhir-r7` … `fhir-r10` | Name reservations at `0.0.1`. No such specifications exist (or, for R1/DSTU1, none is modelled); they contain no model |
| `fhir-derive-macros` | `#[derive(Validate)]`, `Builder`, `FhirChoice` |

The models are separate crates so that compiling all of them is several
`rustc` processes rather than one: peak memory is the largest crate, not the
sum. Before the split, `--features "r3 r4"` needed **12.9 GB**; after, **5.0 GB**.

Only `r5` is on by default, so the commands above see R5 alone. If you touched
the generator, the derive macros, `fhir-core`, or any release crate, run the
gate with the others enabled too:

```sh
cargo test --features "r2 r3 r4 r4b r6 xml client"
cargo clippy --all-targets --features "r2 r3 r4 r4b r6 xml client" -- -D warnings
```

## Which tree you are editing

- **`fhir-r2/src` … `fhir-r4/src`, and `fhir-r4b/src`, are generated.**
  Change `src/codegen/`, then `cargo run -- r2` … `cargo run -- r4b`.
- **`fhir-r5/src` is hand-documented.** Never regenerate over it; `cargo run
  -- r5` refuses without an explicit `--out`.
- **`fhir-r6/src` is generated from a ballot draft** (6.0.0-ballot3). Its
  feature is off by default and it is outside the semver promise until R6 is
  final — but it **is** published, and has to be: the facade's optional
  dependency on it needs a registry version, so holding it back would make
  `fhir` unpublishable (`R12.14a`).
- **Twelve modules per release are hand-maintained**, not generated: `builder`,
  `bundle_util`, `choice`, `client`, `coded`, `lib`, `meta`, `prelude`,
  `summary`, `temporal`, `validate`, `xml`.

Adding a release is a documented procedure:
[`doc/adding-a-release.md`](doc/adding-a-release.md).

Everything else — the conventions, the cardinality mapping, the generator, the
release checklist — lives in the documents linked above.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
