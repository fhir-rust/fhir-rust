# The fhir-rust monorepo specification

This directory is the **root of every normative specification in the
repository**. It says which specification governs which code, how their
requirement numbers relate, and where a citation resolves — so that a number
written in a commit message, a test name, or an auditor's workpaper has exactly
one meaning. It states requirements of its own only where they bind every
family at once: today that is the [agents directory
name](agents-directory-name-is-lowercase/index.md) (`AG1.x`), the [MSRV
rule](rust-msrv-n-minus-3/index.md) (`RV1.x`), and [what a git tag
names](git-tags-name-published-versions/index.md) (`TG1.x`).

Operational guidance for contributors lives in [`AGENTS.md`](../AGENTS.md) and
[`agents/`](../agents/index.md). Those say **how to work**; the specifications
below say **what must be true**. When they conflict, the specification wins.

## The four families

The repository holds four bodies of code. Three were specified independently
and still are; the persistence core is governed by the database specification
rather than one of its own, because it *is* the ports' shared half. They share a domain, a language, and a
discipline; they do not share requirement numbers, release cadence, or a
conformance model.

| Family | Code | Specification | Ids | Status |
| --- | --- | --- | --- | --- |
| **Model** | [`fhir/`](../fhir/) | [`fhir/spec/index.md`](../fhir/spec/index.md) — 14 sections | `R1.x`–`R14.x` | specified, versioned `3.0.0` |
| **Persistence core** | [`fhir-store/`](../fhir-store/) | governed by [`spec/databases/`](databases/index.md) — `M3.16`, `PR12.x` | shares the database ids | specified |
| **Databases** | [`fhir-postgresql/`](../fhir-postgresql/) and five sibling ports | [`spec/databases/index.md`](databases/index.md) — sections 0–16 | see table below | specified, pre-release |
| **HTTP surface** | [`fhir-loco/`](../fhir-loco/) | [`fhir-loco/spec/index.md`](../fhir-loco/spec/index.md) — 4 sections | `SV1.x`–`SV4.x` | specified 2026-08-03 |

They compose in one direction only:

```
fhir/                 FHIR R2–R6 as Rust types, generated from the spec packages
   │                  (no database, no I/O)
   │
fhir-store/           the engine-agnostic half of persistence: audit chain,
   │                  attribution, result types (no driver, no socket)
   ▼
fhir-<engine>/        those resources shredded into real relational tables
   │                  (no HTTP, no CLI — C0.17, C0.18)
   ▼
fhir-loco/            an HTTP surface over one of those stores
```

Nothing lower in that stack may depend on something higher. A requirement about
JSON fidelity belongs to the model family even though the database family also
depends on it; the database family cites it rather than restating it.

## Precedence

1. Within a family, that family's specification is normative for its code.
2. A **port dialect annex** (`M14.x`, one per database port) is normative only
   where it explicitly amends a core requirement **by number**. `M14.6 amends
   M3.4` is a departure; prose that merely differs is not.
3. Where a port has no annex text on a subject, the database core governs
   unmodified.
4. **No family is normative for another.** A `fhir/spec` requirement does not
   bind a database port, and a database requirement does not bind the model
   crate. Where one family relies on another's guarantee, it cites it as an
   assumption, not as a rule it enforces.
5. Nothing in a `README.md`, `book/` chapter, `plan.md`, `tasks.md`, or code
   comment is normative. Those describe; these decide.

An undeclared departure is a defect in the code, not an amendment to the
specification.

## Requirement-id namespaces

Every id is `<prefix><section>.<n>`, optionally with a letter suffix for a
requirement split after the fact. **Ids are permanent** (`C0.5`): never
renumbered, never reused, including across a file move.

### Cross-family — `spec/`

| Prefix | Document | Subject |
| --- | --- | --- |
| `AG1` | [The AI agents directory is named `agents`](agents-directory-name-is-lowercase/index.md) | the lowercase directory name, and what it does not bind |
| `RV1` | [Rust MSRV — current minus three](rust-msrv-n-minus-3/index.md) | the MSRV rule, where it is declared, how it is verified |
| `TG1` | [Git tags name published versions](git-tags-name-published-versions/index.md) | what a tag names, what it points at, and why six crates have none |

The only normative ids that bind **every** family. They live here rather than in
a family's specification because the toolchain floor is a property of the
release surface as a whole, as is the layout it is released from; precedence
rule 4 above is about family-to-family bindings and does not exempt a family
from `AG1.x` or `RV1.x`.

Three further documents at this level bind the repository as a process rather
than any family's code, and carry prose rules instead of requirement ids:
[`professionalization/`](professionalization/index.md) (what "professional"
means here — plans, special files, CI-enforced claims, trademark discipline,
PHI candour, conduct, family harmonization, gated outreach),
[`hl7-trademarks-fair-use/`](hl7-trademarks-fair-use/index.md) (the HL7®
word-mark terms and the check that enforces them), and
[`special-files-for-public-repos/`](special-files-for-public-repos/index.md)
(the canonical root-document list).

### Databases — `spec/databases/`

| Prefix | Section | Subject |
| --- | --- | --- |
| `C0` | [0](databases/00-conformance.md) | normative language, id grammar, conformance levels |
| `S1` | [1](databases/01-scope.md) | FHIR® versions, resource coverage, engine floors |
| `G2` | [2](databases/02-schema-generation.md) | determinism, identifiers, install |
| `M3` | [3](databases/03-storage-model.md) | tables, types, extensions, audit, hash chain |
| `R4` | [4](databases/04-shredding-and-reconstruction.md) | lossless round-trip, snapshot reads |
| `H5` | [5](databases/05-versioning-and-history.md) | versions, soft delete, vread |
| `P6` | [6](databases/06-search.md) | parameters, folding, indexes, bounded cost |
| `V9` | [9](databases/09-validation.md) | structural, strict, terminology gap |
| `O10` | [10](databases/10-operations.md) | logging, migrations, TLS, supply chain |
| `T11` | [11](databases/11-conformance-testing.md) | what must be tested, and how honestly |
| `PR12` | [12](databases/12-trust-principal-and-audit.md) | attribution, disclosure logging |
| `M14` | 14 — *per port* | that engine's declared departures |
| `X15` | [15](databases/15-portability-and-dialects.md) | what is shared, what an annex must say |
| `W16` | [16](databases/16-repository-and-release.md) | layout, SSOT, versioning, release |
| `L1`–`L16` | [locale and accent folding](databases/locale-accent-folding.md) | the fold, normatively |
| `U1`–`U13` (incl. `U1a`, `U2a`, `U2b`, `U4a`, `U11a`) | [search adjuncts](databases/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) | bounded and checksum adjuncts for any search-reachable column a dialect cannot index or compare — string, CLOB, BLOB, fixed-shape |

Sections **7** (REST API) and **8** (CLI) are retired — the ports are embeddable
libraries. The numbering keeps the gap rather than renumbering, so `V9.2` still
means what it meant. Section **13** is a compliance mapping table and defines no
requirements of its own.

### HTTP surface — `fhir-loco/spec/`

| Prefix | Section | Subject |
| --- | --- | --- |
| `SV1` | [1](../fhir-loco/spec/01-scope-and-conformance.md) | what the crate is, three conformance levels, honesty |
| `SV2` | [2](../fhir-loco/spec/02-endpoints.md) | routes, status codes, `OperationOutcome`, CapabilityStatement |
| `SV3` | [3](../fhir-loco/spec/03-trust-and-attribution.md) | PASETO, the principal, what the store is told |
| `SV4` | [4](../fhir-loco/spec/04-operations.md) | limits, configuration, binding, logging |

`SV` collides with neither other family, deliberately — see the `R4` note below
for what a shared prefix costs once ids are permanent.

It **restates** rather than moves the requirements that already described this
crate: §7 and §8's retired ids (`A7.8`, `A7.10`–`A7.12`, `M8`, registered in
`C0.16`) and §10/§12's `[service]`-marked ones. Moving them would renumber
across families, which `C0.5` forbids and which is how the `R4` collision
happened.

### Model — `fhir/spec/`

Numbered `R<section>.<n>` throughout, one prefix per section: `R2` primitive
types, `R3` complex datatypes, `R4` resources, `R5` code systems, `R6`
serialization, `R7` validation, `R8` code generation, `R9` primitive
extensions, `R10` invariant coverage, `R11` choice types, `R12` FHIR releases,
`R13` assurance, `R14` cross-release conversion. Section 1 is an overview and
defines no requirements.

### The `R4` collision — read this before citing `R4.x`

`R4.1`–`R4.7` exist in **both** families and mean unrelated things:

| Id | `fhir/spec/04-resources.md` | `spec/databases/04-shredding-and-reconstruction.md` |
| --- | --- | --- |
| `R4.1` | each resource MUST be a Rust struct with the canonical derives | shred and reconstruct MUST be driven by the generated map through one generic engine |
| `R4.2` | fields MUST use the cardinality mapping of spec 06 | round-trip MUST be lossless, decimal precision and partial dates included |

This is the only overlap between the two families — every other database
section carries a distinct letter (`S1`, `G2`, `M3`, `H5`, `P6`, `V9`, `O10`,
`T11`, `PR12`, `X15`, `W16`), so `R4` is the single ambiguous prefix.

**A bare `R4.x` is therefore not a citation.** Qualify it:

- `db:R4.2` or `spec/databases R4.2` — the round-trip requirement
- `model:R4.2` or `fhir/spec R4.2` — the cardinality requirement

Existing bare citations are resolved by the file they appear in: a citation
inside `fhir/` means the model spec, one inside `fhir-<engine>/` or
`spec/databases/` means the database spec. Neither family may renumber to
remove the clash (`C0.5`), so the qualifier is the fix.

## Status, not requirements

These record reality rather than intent, and are non-normative:

| | |
| --- | --- |
| [databases conformance matrix](databases/conformance-matrix.md) | which port satisfies which core requirement **today** |
| [databases audit](databases/audit.md) | every known divergence between spec, docs, and code, with evidence |
| [`fhir/spec/13-assurance.md`](../fhir/spec/13-assurance.md) | what must hold before the model crate is depended on clinically |
| [publishing readiness](publishing.md) | what stands between all 33 crates and crates.io — **cross-family** |

The conformance matrix is the document to trust for port status. A README, a
`book/` chapter, and a `tasks.md` checkbox are all weaker evidence than it, and
in this repository all three have been wrong.

## Gaps

Recorded here so that "no specification" is a stated fact rather than an
oversight a reader has to infer:

- ~~**`fhir-loco/` has no specification.**~~ **Closed 2026-08-03.** It now has
  one: [`fhir-loco/spec/`](../fhir-loco/spec/index.md), ids `SV1.x`–`SV4.x`.
  Every promise this gap listed — `410` versus `404`, `If-Match`,
  `_count`/`_offset`/`_total`, the CapabilityStatement — is now a numbered
  requirement that can be cited and shown to have regressed.

  Sections 7 and 8 of the database spec stay retired: HTTP and CLI are out of
  scope *for the ports*, which was always the right reading. The new
  specification restates their obligations under `SV` ids rather than moving
  them, because `C0.5` makes ids permanent.

  **Resolved the same day.** The owner chose the second of the two ways out:
  `fhir-loco` has its own specification, and it restates the `[service]`
  obligations under `SV` ids while citing the originals. The `[service]` markers
  in §10 and §12 stay and now mean "binds `fhir-loco`, restated as `SV4.x`".

  What it does **not** meet is recorded at the `SV` id rather than in a list
  that drifts: `SV4.2` (no concurrency or in-flight limit — Loco 1.0.1 exposes
  neither), `SV4.3` (no admin plane, no `/metrics`), `SV2.14` (no conditional
  create), `SV2.15` (no `$export`), and `SV3.11` (no requirement anywhere states
  an obligation for the listener's own TLS).
- **All six dialect annexes are still marked *proposed*** (`X15.9`), so none may
  be cited as evidence for a conformance level.

## Where to start

| You are | Read |
| --- | --- |
| evaluating the databases | [`README.md`](../README.md), then the [conformance matrix](databases/conformance-matrix.md) |
| implementing a database port | [`spec/databases/index.md`](databases/index.md), then [§15](databases/15-portability-and-dialects.md) |
| changing the model crate | [`fhir/spec/index.md`](../fhir/spec/index.md) |
| auditing compliance | [§13 compliance mapping](databases/13-compliance-mapping.md) |
| releasing to crates.io | [publishing readiness](publishing.md) first, then [§16](databases/16-repository-and-release.md) `W16.x` |
| contributing at all | [`AGENTS.md`](../AGENTS.md) |

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
