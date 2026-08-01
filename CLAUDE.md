# CLAUDE.md

Guidance for Claude Code and other agents working in this repository.

**Start with [`AGENTS.md`](AGENTS.md).** It is the single source of operational
guidance, shared by every contributor and every agent; this file adds only what
is specific to working here through an agent harness. Everything in `AGENTS.md`
applies — in particular its five rules, which are the ones that get broken.

## The one-paragraph orientation

Six FHIR-to-relational database libraries in one monorepo, one directory each.
They store FHIR R3/R4/R5 resources as real relational tables generated from the
FHIR specification, and give them back losslessly. `/spec` is the normative
single source of truth for all six; each port adds only a dialect annex. The
pure-Rust core (shred, reconstruct, fold, canon, gen) is **identical across all
six ports** and must be changed in all six at once.

## Read before editing

| Change | Read first |
| --- | --- |
| Anything normative | [`spec/index.md`](spec/index.md), then the section |
| Shared Rust core | [`spec/15-portability-and-dialects.md`](spec/15-portability-and-dialects.md) `X15.1` |
| A `ddl.rs` or a store | that port's `spec/14-*-dialect.md`, plus `X15.6` |
| Any documentation | [`AGENTS/documentation.md`](AGENTS/documentation.md) |
| Anything at all | [`spec/audit.md`](spec/audit.md) — the change may already be a tracked finding |

## Traps specific to this repository

**The READMEs were wrong, and are now right — the books still are not.** Every
port's `README.md` used to describe a CLI (`fhir-<engine> serve`) and claim
7,399 FHIR example resources round-tripped, in ports where none of it was true;
three were even titled "FHIR in PostgreSQL" while targeting another engine. All
six were rewritten (**F-01** fixed).

Their `book/` directories were **not**, and still carry PostgreSQL text and REST
chapters describing a service layer that does not exist (`C0.17`). The
[conformance matrix](spec/conformance-matrix.md) remains the status document to
trust, and there is still no CLI crate in any workspace.

**Neither were the `tasks.md` files, and they make the strongest claim of the
three** — a `[x]` says the work is finished (**F-27**). All six tick off a REST
server, a CLI, and a full HTTP suite that no port has ever contained; the two
scaffolds additionally tick off a store, citing acceptance runs against MySQL;
and the three non-PostgreSQL store ports describe their real stores in terms of
`tokio-postgres`, `FOR UPDATE`, and staged-schema installs they do not use.

Each file now opens with a header saying which parts of it are untrue. **Do not
use a `tasks.md` to decide what is done or what to work on** — read the
conformance matrix and the port's dialect annex instead.

**Six files that look like six files are one file.** `shred.rs`,
`reconstruct.rs`, `fold.rs`, `canon.rs`, `model.rs`, `value.rs`, `error.rs`, and
everything under `gen/src` are byte-identical across all six ports modulo the
crate name. Editing one is a divergence, not a fix.

This is now gated (**F-10** fixed) — run it yourself before and after touching
any of them:

```sh
./scripts/check-shared-core.sh          # --diff to see what moved
```

There are **no exemptions left**: `fhir-postgresql`'s missing `canon.rs` was
the only one, and closing **F-07** ported it in. The script's `EXEMPT` list is
empty and should stay that way — an exemption is a divergence that survived
review, so a new one must cite the finding or the `M14.x` departure allowing it.

**`grep` will find the same string six times.** When searching, either scope to
one port or expect sixfold results. A finding that appears in all six is usually
one finding.

**The two scaffold ports have no store.** `fhir-mssql` and `fhir-oracle` contain
`lib.rs` and `chain.rs` and no implementation. Until 2026-07-31 both also
provisioned **MySQL** in CI and in `scripts/db.sh` while invoking a test target
that did not exist, so their database jobs could not pass at all (**F-06**,
fixed): `fhir-mssql` now provisions SQL Server 2022 and fails rather than skips
without a database; `fhir-oracle`'s gate was removed, because there is nothing
yet to point it at.

**Requirement ids are permanent** (`C0.5`). Never renumber, never reuse. If you
split a requirement, use letter suffixes and keep the parent.

## Verifying a claim before you make one

This repository is about clinical software, and its main failure mode has been
confident text that nothing substantiates. Before writing that something works:

```sh
# Does the operation exist?
grep -rn "pub async fn <op>" fhir-<engine>/crates/*-store/src/

# Is there a test, and does it run without a database?
ls fhir-<engine>/crates/*-store/tests/
grep -rn "TEST_DSN\|return Ok(())" fhir-<engine>/crates/*/tests/

# Does CI provision the right engine?
grep -n "image:" fhir-<engine>/.github/workflows/ci.yml
```

If the answer is "the code is shared from a port where it works", that is `?` in
the [conformance matrix](spec/conformance-matrix.md), not `•`.

## Scope discipline

The shared core makes small changes large. A one-line fix in `fold.rs` is:

1. six identical edits (rule 2),
2. a spec check — does it change `L4`/`L6`, and is that a data migration
   (`L12`, `O10.4a`)?
3. a backfill story per port — four ports have no `upgrade` (**F-15**),
4. a mutation-verified test (`T11.10`, `L16`).

Say so before starting, rather than discovering it at step 3.

## Commit and push

Commit conventions are in [`AGENTS.md`](AGENTS.md#commit-conventions). Reference
requirement ids and audit findings.

**Do not push.** All six ports still carry the ancestor project's `origin`
(**F-11**), so a push would send a port to the wrong repository.
