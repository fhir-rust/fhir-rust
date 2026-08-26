# CLAUDE.md

Guidance for Claude Code and other agents working in this repository.

**Start with [`AGENTS.md`](AGENTS.md).** It is the single source of operational
guidance, shared by every contributor and every agent; this file adds only what
is specific to working here through an agent harness. Everything in `AGENTS.md`
applies — in particular its five rules, which are the ones that get broken.

## The one-paragraph orientation

Three families in one monorepo, plus two supporting crates. [`fhir/`](fhir/)
is the FHIR® **model** — every resource and datatype as Rust types, generated
from the specification packages, with its own spec and its own `AGENTS.md`.
Six `fhir-<engine>/` directories are the **database** ports: they store FHIR
R3/R4/R5 resources as real relational tables and give them back losslessly,
governed by one normative core in `/spec/databases`, each adding only a
dialect annex. The pure-Rust core of the ports (shred, reconstruct, fold,
canon, gen) is **identical across all six** and must be changed in all six
at once. [`fhir-store/`](fhir-store/) is **not** the HTTP surface, despite
the name and despite what an earlier revision of this file said — it is a
small shared **library** (the tamper-evident audit chain, `Audit`,
`AccessRecord`, and the other engine-agnostic value types every port
returns), extracted from ~860 duplicated lines across the six ports
(**F-45**) and has no spec of its own. The actual HTTP surface is
[`fhir-loco/`](fhir-loco/), a separate crate that mounts a FHIR REST API
over `fhir-sqlite` — it inherited the name `fhir-store` briefly before being
renamed (**F-37**, fixed 2026-08-02), and the name was then reused for the
unrelated library crate a few days later (**F-45**), which is exactly the
kind of history a name collision leaves behind for the next reader to trip
on.

## Read before editing

| Change | Read first |
| --- | --- |
| Anything, if unsure which spec applies | [`spec/index.md`](spec/index.md) — the root of all three |
| Anything normative in a port | [`spec/databases/index.md`](spec/databases/index.md), then the section |
| Anything in `fhir/` | [`fhir/spec/index.md`](fhir/spec/index.md) and [`fhir/AGENTS.md`](fhir/AGENTS.md) |
| Shared Rust core | [`spec/databases/15-portability-and-dialects.md`](spec/databases/15-portability-and-dialects.md) `X15.1` |
| A `ddl.rs` or a store | that port's `spec/14-*-dialect.md`, plus `X15.6` |
| Any documentation | [`agents/documentation.md`](agents/documentation.md) |
| Anything at all | [`spec/databases/audit.md`](spec/databases/audit.md) — the change may already be a tracked finding |

## Traps specific to this repository

**The READMEs were wrong, and are now right — the books still are not.** Every
port's `README.md` used to describe a CLI (`fhir-<engine> serve`) and claim
7,399 FHIR example resources round-tripped, in ports where none of it was true;
three were even titled "FHIR in PostgreSQL" while targeting another engine. All
six were rewritten (**F-01** fixed).

Their `book/` directories were **not**, until 2026-08-03. The engine
substitution is now fixed throughout (**F-56**): each book names its own engine,
its own backup tooling, and its own namespace mapping, and opens with a banner.
The REST half is settled too: the server is [`fhir-loco`](fhir-loco/), a
separate crate, and every book now says so in its banner — any `serve`, endpoint
or status code in those chapters is `fhir-loco`'s behaviour, not the library's
(`C0.17`, `C0.18`). Rewriting the eight chapters to describe it properly is
documentation work, not a correctness defect. The
[conformance matrix](spec/databases/conformance-matrix.md) remains the status document to
trust, and there is still no CLI crate in any workspace.

**Neither were the `tasks.md` files, and they make the strongest claim of the
three** — a `[x]` says the work is finished (**F-27**). Two of the three classes
are now fixed:

- The two scaffolds' files were **replaced**. They were `fhir-mysql`'s with the
  name substituted, ticking off a store and citing acceptance runs against
  MySQL 8.4. Each is now ~68 lines and true.
- The three non-PostgreSQL store ports no longer describe their stores as
  `tokio-postgres` with staged-schema installs. Note `FOR UPDATE` was *not*
  contamination in mysql/mariadb — both really use it; only sqlite does not.
- `T32 Encrypted database transport` was the worst of them: a **security**
  claim, `[x]` in all four store ports, describing TLS machinery that exists
  only in `fhir-postgresql`. mysql and mariadb are now unticked; sqlite says
  plainly that a local file has no link to encrypt.

**The class-1 REST/CLI entries are gone as of 2026-08-06** (**F-80**): the
owner had already resolved that the server is `fhir-loco` (F-27's class-1
disposition, 2026-08-03), and this pass executed it — each misattributed entry
in the four store ports' files is now a one-line tombstone keeping its task id
and pointing at `fhir-loco`. Two ticked store-layer *guarantees* that never
existed in mysql/mariadb (conditional-op atomicity, the `*_audited` envelope)
were also found and unticked (**F-78**).

**Do not use a `tasks.md` to decide what is done** — read the conformance matrix
and the port's dialect annex instead.

**Six files that look like six files are one file.** `shred.rs`,
`reconstruct.rs`, `fold.rs`, `canon.rs`, `model.rs`, `value.rs`, `error.rs`, and
everything under `gen/src` **and `gen/tests`** are identical across all six
ports modulo the crate name. Editing one is a divergence, not a fix.

Identical *modulo whitespace*, not byte-identical: rustfmt wraps at a column, so
a longer crate name splits a line that fits in a shorter one. The gate compares
tokens for that reason (`X15.1a`, **F-48**).

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
one finding. Scope away from `fhir/` too unless you mean it: it is a different
family, ~135k generated lines per release, and `fhir/fhir.md` alone is a
generated transcript that will dominate any repo-wide result.

**`R4.x` means two different things.** `R4.1`–`R4.7` are defined in both
`fhir/spec/04-resources.md` (Rust struct conventions) and
`spec/databases/04-shredding-and-reconstruction.md` (lossless round-trip). It is
the only prefix the two families share — every other database section carries a
distinct letter. Resolve a bare citation by the file it appears in, and write
new ones qualified: `db:R4.2`, `model:R4.2`. Neither family may renumber to fix
it (`C0.5`); see [`spec/index.md`](spec/index.md#the-r4-collision--read-this-before-citing-r4x).

**`fhir-mssql` and `fhir-oracle` were the two scaffold ports with no store.
Neither is, any longer.** Both reached **Store** level this project
(`fhir-mssql`: **F-65**; `fhir-oracle`: **F-68**, superseding the earlier
"compiles but never connects" **F-66**) — each has a real store with `put`/
`get`/`delete`/`history`/`vread`/`search`/`verify_audit`/`purge`/
`log_access`, live-tested against a real server, `azure-sql-edge` and
`gvenzl/oracle-free` respectively. Do not describe either as "no
implementation" — check the [conformance
matrix](spec/databases/conformance-matrix.md) for what actually remains
(as of 2026-08-09 both have `upgrade`/`backfill_norm` — **F-15** closed on
all six ports; `fhir-oracle` still has no working `R4.5` mechanism and no
concurrency test). Until 2026-07-31 both
also provisioned **MySQL** in CI and in `scripts/db.sh` while invoking a
test target that did not exist, so their database jobs could not pass at
all (**F-06**, fixed): `fhir-mssql` now provisions SQL Server 2022 and fails
rather than skips without a database; `fhir-oracle`'s CI gate was removed
rather than faked, and restored 2026-08-12 with a real engine: its live job
provisions `gvenzl/oracle-free` and ran the store suite green on its first
hosted execution; `scripts/db.sh` covers local live testing.

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
the [conformance matrix](spec/databases/conformance-matrix.md), not `•`.

## Scope discipline

The shared core makes small changes large. A one-line fix in `fold.rs` is:

1. six identical edits (rule 2),
2. a spec check — does it change `L4`/`L6`, and is that a data migration
   (`L12`, `O10.4a`)?
3. a backfill story per port — all six ports have `upgrade`/`backfill_norm`
   as of 2026-08-09 (**F-15** closed), so the question is each port's
   dialect story (one transaction on mssql, resumable-rerunnable on oracle,
   reported-partial on mysql/mariadb), not whether one exists,
4. a mutation-verified test (`T11.10`, `L16`).

Say so before starting, rather than discovering it at step 3.

## Commit and push

Commit conventions are in [`AGENTS.md`](AGENTS.md#commit-conventions). Reference
requirement ids and audit findings.

**Pushing: the old warning is obsolete, a smaller one is not.** This file used
to say "do not push — all six ports carry the ancestor project's `origin`
(**F-11**)". That is no longer true and has been verified: none of the six
ports, nor `fhir/`, has a `.git` of its own. They are directories in one
repository with one remote, `git@github.com:fhir-rust/fhir-rust.git`.

What remains is narrower. That URL 404s anonymously — which a private repository
also does, so it is unverified rather than known-absent (`P-5`).

**The old `fhir-store/`-nested-repository warning (F-37) is obsolete too, and
for a more confusing reason than the six-remotes one.** F-37 was real and was
about a *different* directory that used to be called `fhir-store/` — the
HTTP surface, which had its own untracked `.git` with no remote. It was
fixed 2026-08-02 by removing the nested repo and committing the source
directly; that directory was then renamed `fhir-loco` (**F-45**, the same
day). The name `fhir-store` was reused a few days later for an unrelated
extraction — a small shared library, not a server — and **that** crate has
never had a nested `.git`: `git status` shows its files tracked normally
(`git ls-files fhir-store/` lists them; the index has no `160000` gitlink
mode entries), verified 2026-08-04. Do not carry F-37's caution forward onto
the crate that now holds this name.

Still ask before pushing.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
