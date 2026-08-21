# AGENTS.md

Guidance for AI coding agents working in this repository, following the open
[AGENTS.md](https://agents.md) convention. Human contributors are welcome to
read it too.

## What this project is

`fhir` is a Rust crate that provides the **FHIR** data model as strongly-typed,
`serde`-serializable Rust, together with a **spec-driven code generator** that
derives that model from the official FHIR specification JSON files. FHIR (Fast
Healthcare Interoperability Resources) is the HL7 standard for exchanging
electronic health records.

**Five releases are modelled**, each complete and independent:

| Release | Module | Feature | Primitives | Datatypes | Resources | Code enums |
| --- | --- | --- | --- | --- | --- | --- |
| R5 (5.0.0) | `fhir::r5` | `r5` (default) | 21 | 50 | 158 | 442 |
| R4 (4.0.1) | `fhir::r4` | `r4` | 20 | 43 | 146 | 486 |
| R3 (3.0.2, STU3) | `fhir::r3` | `r3` | 18 | 36 | 117 | 386 |
| R2 (1.0.2, DSTU2) | `fhir::r2` | `r2` | 18 | 28 | 94 | 265 |
| R6 (6.0.0-ballot3) | `fhir::r6` | `r6` | 21 | 51 | 161 | 459 |

R6 is a **ballot draft**: off by default and outside the semver promise —
but **published**, necessarily: the facade's optional dependency on it needs
a registry version (`R12.14a`; an earlier revision of this paragraph said
"unpublished", which was never something a workspace member of a published
facade could be). `fhir-r1`, `fhir-r7` through
`fhir-r10` are name reservations only (0.0.1) — no such
specifications exist (or, for R1/DSTU1, none is modelled) and those crates
contain no model.

Each release adds `value[x]` **choice enums**, **`Coded<E>`** for
required-binding codes, generated **builders**, a **prelude**, extension
helpers, `Bundle` utilities, summary serialization, and optional **`client`**
(async REST) and **`xml`** support.

**The releases are separate types on purpose.** An R3 `Patient` is not an R4
`Patient` is not an R5 `Patient`; the releases disagree about which elements exist and what they mean,
and a type standing for both would let those differences pass silently. Code
that genuinely does not care about the release belongs in `fhir-core` (see
below).

## R5 is hand-tended; the other four are generated

This distinction governs how you edit each one:

- **`fhir-r5/src/` carries hand-written prose documentation** on top of generated
  shapes. Never blind-regenerate it. Change it with the metadata-driven splicing
  generators described in [`agents/code-generation.md`](agents/code-generation.md),
  or by hand. `cargo run -- r5` refuses to write there for this reason.
- **`fhir-r2/src/`, `fhir-r3/src/`, `fhir-r4/src/` and
  `fhir-r6/src/` are entirely generated** by `cargo run -- r2` … `-- r6`.
  Do not hand-edit them; change the generator and regenerate. (Their few
  hand-written support modules — `validate.rs`, `choice.rs`,
  `bundle_util.rs`, `prelude.rs`, … — are not generated and are edited
  normally.)

## Repository shape

A cargo **workspace**: one crate per FHIR release, plus a release-independent
core and a thin facade.

```text
Cargo.toml            # workspace root; also the package `fhir` (the facade)
src/
  main.rs             # thin binary: runs the code generator
  lib.rs              # facade: re-exports fhir-core and the release crates
  prelude.rs          # common imports (feature `r5`)
  codegen/            # the release-parameterized generator (spec JSON -> Rust)
  r5/parse/           # legacy R5-only generator + splicing generators

fhir-core/src/        # release-independent. One implementation, shared by all.
  decimal.rs          # FHIR `decimal`, preserving the precision it was given
  validate.rs         # `Validate` trait, `ValidationIssue`, primitive checks
  coded.rs            # `Coded<E>` (Known | Unknown) for required bindings
  builder.rs          # `BuilderError`
  meta.rs             # the shape of the per-element metadata table
  temporal.rs         # date/time parsing and precision-aware comparison
  summary.rs          # `_summary=true` pruning
  xml.rs              # the FHIR XML bridge (feature `xml`)
  client.rs           # async REST client, generic over `Release` (feature `client`)
  release.rs          # the `Release` trait: naming a release in generic code

# One crate per release, identical in shape.
fhir-r5/src/          # R5: hand-tended prose over generated shapes
  types/              # 21 primitives + 50 complex datatypes (one module each)
  resources/          # 158 resources (one module each) + `Resource` enum
  codes.rs            # 442 code-system enums
  meta/generated.rs   # the generated element table
fhir-r4/src/          # R4: fully generated. 146 resources, 486 code enums
fhir-r3/src/          # R3 (STU3): fully generated. 117 resources, 386 code enums
fhir-r2/src/          # R2 (DSTU2): fully generated. 94 resources, 265 code enums
fhir-r6/src/          # R6: generated from 6.0.0-ballot3. 161 resources, 459 enums
                      #     outside the semver promise (published — R12.14a)
fhir-r1/, fhir-r7/ … fhir-r10/   # name reservations (0.0.1); no model
fhir-derive-macros/   # proc-macro crate: #[derive(Validate, FhirChoice, Builder)]

doc/fhir-specifications/<release>/fhir-definitions-json/  # the official spec JSON
doc/adding-a-release.md  # the procedure for adding one, written from doing R6
tmp/out/              # legacy R5 generator scratch output (untracked)
spec/                 # the living specifications — the source of truth (read these)
agents/               # operational guidance for agents (this folder)
```

Each release crate self-aliases (`pub use crate as r5;`) so that generated
paths written `crate::r5::…` resolve inside the crate that *is* r5. That is
what let ~5,100 such paths, and the derive macros that emit them, survive the
split untouched.

## Commands you must be able to run

| Task | Command |
| --- | --- |
| Build | `cargo build` |
| Build every release | `cargo build --all-targets --features "r2 r3 r4 r4b r6"` |
| Test (unit + doctests) | `cargo test` |
| Test every release | `cargo test --features "r2 r3 r4 r4b r6"` |
| Doctests only | `cargo test --doc` |
| Doctests of one release crate | `cargo test -p fhir-r2 --doc` |
| Lint (pedantic; must be 0) | `cargo clippy --all-targets -- -D warnings` |
| Lint every release | `cargo clippy --all-targets --features "r3 r4 xml client" -- -D warnings` |
| Docs (deny warnings) | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features "r3 r4 xml client"` |
| Feature builds | `cargo test --features client` / `--features xml` / `cargo build --features precise-decimal` |
| Regenerate a release | `cargo run -- r2` … `cargo run -- r4`, `cargo run -- r6` |
| Regenerate R5 to compare | `cargo run -- r5 --out tmp/out/r5` (never over `fhir-r5/src`) |
| Build only the proc-macro | `cargo build -p fhir-derive-macros` |
| Publish dry-run | `cargo publish --dry-run -p fhir-derive-macros` |

The crate is imported as `fhir` (e.g. `use fhir::r5::resources::Patient;`).
**Only `r5` is on by default**, so anything exercising R3 or R4 needs
`--features "r3 r4"`.

## The prime directive: keep it green

Before you consider any task finished, **all three must pass**:

1. `cargo build` — clean.
2. `cargo test` — every unit test **and doctest** passes.
3. `cargo clippy --all-targets` — **zero** warnings (`clippy::pedantic` is on).

`cargo test --doc` covers the **root package only**. Each release crate holds
its own several-hundred doctests, and those run only when the crate is named:

```sh
for r in 2 3 4 6; do cargo test -p fhir-$r --doc; done
```

They are the main documentation for the generated model, and they are easy to
break when a release crate is seeded by copying a later one — the examples
compile against fields that release does not have. CI runs them per crate in
the `release-crate-doctests` job.

This crate is currently 100% green. Do not regress it. If you touch the model,
re-run the full gate **for every release you touched** — only `r5` is on by
default, so a plain `cargo test` will not notice an R3 or R4 regression.
CI additionally enforces `doc -D warnings`, the MSRV (1.88), the feature builds
(including each release on its own, `--no-default-features --features r3`), the
mdBook build, and the proc-macro publish dry-run.

## How to work here

- **The specs in `spec/` are the source of truth.** This is spec-driven
  development: behaviour is defined in `spec/*` first, then implemented. When
  code and spec disagree, fix the mismatch — do not silently diverge. Start at
  [`spec/index.md`](spec/index.md).
- **Follow the conventions exactly.** Every datatype/resource struct uses the
  same serde derives, `rename_all = "camelCase"`, `skip_serializing_none`, and
  cardinality mapping. See [`agents/conventions.md`](agents/conventions.md).
- **Prefer the generator over hand-editing generated shapes.** `src/codegen/`
  produces a whole release's Rust from the spec JSON. See
  [`agents/code-generation.md`](agents/code-generation.md).
- **Put release-independent code in `fhir-core`.** If it does not mention a
  release's types, it belongs in `src/*.rs`, re-exported from `r4` and `r5` —
  not copied into both.
- **Small, verifiable changes.** Add a test or doctest for anything with a
  runtime surface.

## Map of the guidance

| Document | Purpose |
| --- | --- |
| [`agents/architecture.md`](agents/architecture.md) | Module tree, layering, data flow |
| [`agents/conventions.md`](agents/conventions.md) | The exact struct/field/serde conventions |
| [`agents/testing.md`](agents/testing.md) | Test & doctest patterns, the green gate |
| [`agents/code-generation.md`](agents/code-generation.md) | The spec-JSON → Rust generator |
| [`agents/glossary.md`](agents/glossary.md) | FHIR and project terminology |
| [`spec/index.md`](spec/index.md) | Index of the living specifications |

## House rules

- Keep every file in `agents/` and `spec/` under **40 KB**; split if it grows.
- Do not add dependencies without cause; this crate is deliberately lean
  (`serde`, `serde_json`, `serde_with`, `indoc`, `convert_case`, `vec1` for
  non-empty `1..*` fields, and the local `fhir-derive-macros`). Feature-gated
  extras stay optional: `reqwest`/`tokio` (`client`), `quick-xml` (`xml`).
- Never commit to the default branch; branch first. End commit messages with
  the `Co-Authored-By` trailer if you are an agent.
- `tmp/out/` (untracked scratch) and the generated release trees
  (`fhir-r2/-3/-4/-6/src/`) are generator output — regenerate them, do not
  hand-edit them. `fhir-r5/src/` is hand-tended and must not be regenerated over.
- When adding a FHIR release, everything release-specific must be reachable from
  `codegen::Version`; if you find yourself adding a `match` on the release
  anywhere else, that is a sign the fact belongs on `Version`.
