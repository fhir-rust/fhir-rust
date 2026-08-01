# fhir-sqlite specification

**Engine:** SQLite 3 · **Conformance level:** Store (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures.

- **The core** — [`../../spec/index.md`](../../spec/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** — [`14-sqlite-dialect.md`](14-sqlite-dialect.md),
  numbered `M14.x`. **Status: proposed** (`X15.9`), so it MUST NOT be cited as
  evidence for a conformance level until ratified.

A departure is normative only where it amends a core requirement **by number**
(precedence rule 2). Nothing in this port's `README.md`, `book/`, `plan.md`, or
`tasks.md` is normative. Contributor guidance: [`../../AGENTS.md`](../../AGENTS.md).

> **Note on the annex's preamble.** It opens "Sections 1–13 were inherited
> verbatim from the PostgreSQL original and still describe PostgreSQL". That is
> no longer the case: the core has been consolidated and rewritten in
> engine-neutral terms, so sections 1–13 no longer describe PostgreSQL and no
> longer need amending on that account. The annex's `T67` task is thereby
> largely discharged. Its `M14.x` departures stand.

## What this port departs on

Summarized from the annex; the annex governs.

| `X15.6` item | SQLite |
| --- | --- |
| Engine floor (`S1.4`) | bundled SQLite via `rusqlite` `bundled` — pinned, not the host's |
| Namespace (`S1.2`) | one database file per version, or `ATTACH` |
| `ColTy` binding (`M3.6`) | `INTEGER`, `TEXT`, `TEXT COLLATE BINARY`; `Numeric`, `Date`, `Timestamptz`, `Jsonb` all `TEXT` (`M14.10`–`M14.12`) |
| `ords` binding (`M3.4a`) | `TEXT` holding the shared array literal (`M14.6`–`M14.8`) |
| Index limits (`P6.4a`) | no key-length cap; not applicable |
| Transport (`O10.7`) | **not applicable** — embedded, no connection. The obligation is at-rest: file permissions and storage encryption |
| Unmet core requirements | `O10.4a` (no `upgrade`), `M3.16c`/`M3.16d` (no `chain_witness`/`resign`) |

The annex's own account of why the port is easier than it looks is worth keeping
in view: `gen`, `shred`, `reconstruct`, `value`, `fold`, and `model` do not
change at all (`M14.1`, `M14.2`), because they operate on Rust types and never
emit SQL. That is `X15.1` stated from the other direction.

## Known limitations

From `tasks.md`, and reflected in the
[conformance matrix](../../spec/conformance-matrix.md):

- `transact_audited` returns `Unsupported`. This is the correct answer, not a
  stub: a FHIR transaction Bundle is atomic by definition and a compensating
  unwind is not — readers between ops observe a half-applied bundle, and a
  process dying mid-unwind leaves partial state permanently. Doing it properly
  needs `put` and `delete` split so their bodies run inside a caller-supplied
  `BEGIN IMMEDIATE`.
- `resign_history`, `chain_witness`, `export`, and `init --upgrade` are
  unimplemented; each fails saying so rather than pretending.
- `ColTy::Numeric` has no derived `_sort` companion yet, so numeric range search
  works via `CAST(… AS REAL)` — correct, but it gives up the index.
- The book teaches PostgreSQL's `ords[1]` subscript idiom, which a `TEXT` column
  cannot support (`M14.9`).

## Open findings against this port

- **F-15** — no `upgrade`, so the corrected fold (`L12`–`L14`, `O10.4a`) is a
  full reload rather than a migration here.
- **F-01** — the README carries the PostgreSQL reference's measured results.
- **F-02**, **F-11** — shared; see the [register](../../spec/audit.md).

## Contents of the core

| | Section | Prefix |
| --- | --- | --- |
| 0 | [Conformance](../../spec/00-conformance.md) | `C0.x` |
| 1 | [Scope](../../spec/01-scope.md) | `S1.x` |
| 2 | [Schema generation](../../spec/02-schema-generation.md) | `G2.x` |
| 3 | [Storage model](../../spec/03-storage-model.md) | `M3.x` |
| 4 | [Shredding and reconstruction](../../spec/04-shredding-and-reconstruction.md) | `R4.x` |
| 5 | [Versioning and history](../../spec/05-versioning-and-history.md) | `H5.x` |
| 6 | [Search](../../spec/06-search.md) | `P6.x` |
| 9 | [Validation](../../spec/09-validation.md) | `V9.x` |
| 10 | [Operations](../../spec/10-operations.md) | `O10.x` |
| 11 | [Conformance testing](../../spec/11-conformance-testing.md) | `T11.x` |
| 12 | [Trust, principal, and audit](../../spec/12-trust-principal-and-audit.md) | `PR12.x` |
| 13 | [Compliance mapping](../../spec/13-compliance-mapping.md) | — |
| **14** | [**SQLite dialect**](14-sqlite-dialect.md) | `M14.x` |
| 15 | [Portability and dialects](../../spec/15-portability-and-dialects.md) | `X15.x` |
| 16 | [Repository and release](../../spec/16-repository-and-release.md) | `W16.x` |
| — | [Locale and accent folding](../../spec/locale-accent-folding.md) | `Lx` |

Sections 7 and 8 are retired (`C0.15`).
