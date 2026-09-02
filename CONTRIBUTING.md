# Contributing

Thanks for considering it. This repository has some rules that will surprise
you, and the surprising ones are load-bearing — read this before your first
change rather than after your first rejected pull request.

The authoritative guidance is [`AGENTS.md`](AGENTS.md), shared by every
contributor and every AI agent, with topic guides in
[`agents/`](agents/index.md). This file is the short version and the entry
point. The model crate has its own, narrower guide:
[`fhir/CONTRIBUTING.md`](fhir/CONTRIBUTING.md).

## Ways to contribute

Code is the least of it. In rough order of how much each would help this project
today:

### Give it your data — the highest-value thing anyone can do

Round-trip fidelity is this project's central claim, and it has been tested
against **the official specification's example resources and nothing else**.
Your bundles have extensions we have never seen, profiles we have not read,
decimal precisions we have not hit, and identifier schemes we did not imagine.

```rust
let back = store.get("Patient", id).await?.unwrap();
assert_eq!(back, original);      // tell us when this fails
```

A single reproducible round-trip failure against real-world data is worth more
to this project than a hundred stars, because it produces *evidence*, and
evidence is what the [conformance matrix](spec/databases/conformance-matrix.md)
is starved of. You do not need to fix it. You do not need to share the data —
a redacted resource, or just the shape of the element that broke, is enough.

### Tell us where a claim is wrong

This repository's documented failure mode is confident text that nothing
substantiates ([`AI_STATEMENT.md`](AI_STATEMENT.md) §8). If a README, a book
chapter, a benchmark, or a specification says something that is not true, that
is a defect and we want the issue. Include what you checked and how.

### Bring engine expertise

Six SQL dialects, one specification, and nobody who is an expert in all six. If
you know what a real DBA would say about a port's DDL, its index strategy, its
transaction handling, or its migration story, the dialect annex
(`fhir-<engine>/spec/14-<engine>-dialect.md`) is where that knowledge belongs.
Oracle and SQL Server especially.

### Review the specification

[`spec/`](spec/index.md) decides behaviour before Rust does, so a flaw there is
cheaper to fix than a flaw in six implementations. Requirement-level review — is
this requirement testable, does it contradict another, does it match how FHIR®
actually behaves — is welcome and needs no Rust at all.

### Improve the documentation

Tutorials, the per-port books, examples, and the `doc/` reference. See
[`agents/documentation.md`](agents/documentation.md) for conventions and for the
substitution trap that produced **F-01**.

### Answer someone's question

Issues and, when they are enabled, Discussions. Explaining the storage model to
a newcomer is real work and it is visible.

### Write code

The rest of this file is about that.

### Donate money

If this is useful to you and you would rather send money than time:

| | |
| --- | --- |
| GitHub Sponsors | [github.com/sponsors/joelparkerhenderson](https://github.com/sponsors/joelparkerhenderson) — verified live |
| Patreon | [patreon.com/joelparkerhenderson](https://www.patreon.com/joelparkerhenderson) |
| Ko-fi | [ko-fi.com/joelparkerhenderson](https://ko-fi.com/joelparkerhenderson) |
| PayPal | [paypal.me/joelparkerhenderson](https://paypal.me/joelparkerhenderson) |

All four are declared in [`.github/FUNDING.yml`](.github/FUNDING.yml), which is
what puts the "Sponsor" button on the repository page.

**No Open Collective yet.** It was on the list; it is not set up. Checked
against Open Collective's own API rather than assumed: no collective exists at
either `joelparkerhenderson` or `fhir-rust`. Creating one needs the
maintainer's own sign-in and a fiscal-host choice (Open Source Collective is
the usual one for projects like this), which is not something to do on someone
else's behalf. If that changes, it gets a row here and in
[`.github/FUNDING.yml`](.github/FUNDING.yml) the same day — not before.

**What money buys here, honestly:** maintainer time, and the cost of running six
real database engines in CI. It does not buy a support contract, a service-level
agreement, priority on your issue, or influence over what ships — none of which
this project can deliver, and saying otherwise would be selling something that
does not exist. If you need any of those, you need a commercial FHIR platform;
[`COMPARISONS.md`](COMPARISONS.md) names several.

Sponsorship is never a condition of having a bug fixed, and no contribution is
weighted by whether its author donated.

### Something else

Cite it in a paper ([`CITATION.cff`](CITATION.cff)), tell the HL7® community it
exists, or tell us what you decided *not* to use it for and why — that last one
is genuinely useful and nobody ever sends it. [`RFC.md`](RFC.md) lists the open
questions this project most wants answered.

## Before anything: the rule that catches everyone

**Six files that look like six files are one file.** `shred.rs`,
`reconstruct.rs`, `fold.rs`, `canon.rs`, `model.rs`, `value.rs`, `error.rs`, and
everything under `gen/` **including its tests** — 100 files in total — are
identical across all six database ports, modulo the crate name. Editing one is a
**divergence, not a fix**, and it will be rejected.

Change shared code in **every port, in one commit** (`W16.7`, `X15.1`), and run
the gate before and after:

```sh
./scripts/check-shared-core.sh          # --diff to see what moved
```

It compares tokens rather than lines (`X15.1a`), because rustfmt wraps by
column and a longer crate name splits a line that fits in a shorter one. Its
exemption list is empty and should stay that way.

A corollary worth internalising: **`grep` will find the same string six times.**
A finding that appears in all six ports is usually one finding.

## The five rules

From [`AGENTS.md`](AGENTS.md), which has the full text:

1. **The spec is one copy, at the root.** `spec/databases/` holds every
   normative requirement that is not about a specific SQL dialect. Do not copy a
   section into a port (`W16.5`).
2. **Change shared code in every port, in one commit** (`W16.7`). See above.
3. **A dialect difference goes in the annex, by number** (`C0.12`). If a port
   cannot do what the core requires, write an `M14.x` departure naming the
   requirement it amends. An undeclared departure is a defect, not an amendment.
4. **Do not claim above the port's level** (`C0.11`). All six READMEs did until
   2026-07-31 (**F-01**), and it is the mistake this project is most prone to.
5. **Say what you did not verify.** A skipped test, an unset DSN, an untried
   engine — in the commit message, and in the [audit
   register](spec/databases/audit.md) if it persists. `T11.12` exists because a
   silent skip reads as a pass.

## Specification first

Behaviour is decided in [`spec/`](spec/index.md) before it is written in Rust.
If your change alters what the software promises, the specification changes
first, in the same pull request — see
[`agents/spec-workflow.md`](agents/spec-workflow.md).

**Requirement ids are permanent** (`C0.5`): never renumbered, never reused, not
even across a file move. If you split a requirement, use a letter suffix and
keep the parent.

Watch for the one prefix collision: `R4.x` means different things in
`fhir/spec/` and in `spec/databases/`. Write new citations qualified —
`db:R4.2`, `model:R4.2` — and see
[`spec/index.md`](spec/index.md#the-r4-collision--read-this-before-citing-r4x).

## The green gate

Every workspace is independent; there is no root workspace. From inside the
family you changed:

| Task | Command |
| --- | --- |
| Build | `cargo build --all-targets` |
| Test | `cargo test` |
| Doctests | `cargo test --doc` |
| Lint (pedantic, **zero** warnings) | `cargo clippy --all-targets -- -D warnings` |
| Docs (**zero** warnings) | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` |
| Shared core | `./scripts/check-shared-core.sh` |
| Doc examples | `./scripts/check-doc-examples.sh` |

Most of this project's guarantees are *database* guarantees, so the tests that
matter need a real engine. Each port ships a container script pinned to the same
version CI uses:

```sh
cd fhir-postgresql
scripts/db.sh up && scripts/db.sh corpus && scripts/db.sh test
```

MSRV is **current minus two** (`RV1.1`) — 1.96 across every crate in the
repository, model and ports alike — and CI verifies it on exactly that
toolchain.

## Scope discipline

The shared core makes small changes large. A one-line fix in `fold.rs` is:

1. six identical edits,
2. a specification check — does it change `L4`/`L6`, and is that a data
   migration (`L12`, `O10.4a`)?
3. a backfill story per port, in that port's dialect terms,
4. a mutation-verified test (`T11.10`, `L16`) — a test that cannot fail is a
   defect.

Say so in the pull request before starting, rather than discovering it at
step 3.

## Verifying a claim before you make one

This repository is about clinical software, and its main failure mode has been
confident text that nothing substantiates. Before writing that something works:

```sh
grep -rn "pub async fn <op>" fhir-<engine>/crates/*-store/src/   # does it exist?
ls fhir-<engine>/crates/*-store/tests/                           # is it tested?
grep -n "image:" .github/workflows/fhir-<engine>-ci.yml          # right engine?
```

If the answer is "the code is shared from a port where it works", that is `?` in
the [conformance matrix](spec/databases/conformance-matrix.md), not `•`.

**Do not use a `tasks.md` to decide what is done.** Read the conformance matrix
and the port's dialect annex.

## Pull requests

- One logical change. A shared-core change is one logical change across six
  ports, in one commit.
- Reference requirement ids and audit findings in the message — `M3.16b`,
  `F-67`. That is how a change stays traceable years later.
- State what you did not verify (rule 5).
- If you fixed something the [audit register](spec/databases/audit.md) tracks,
  update that finding's status with evidence.

## Using AI tools

You may. This repository is largely built with them and says so in
[`AI_STATEMENT.md`](AI_STATEMENT.md).

If a contribution contains **ai-generated** content (that document's §3
vocabulary), say so in the pull-request description — which tool, and what it
did. An agentic Claude Code session's own commits already carry that
disclosure as a `Co-Authored-By`/`Claude-Session` trailer, per
`AI_STATEMENT.md` §4 — that trailer is fine as it stands; for any other tool,
put the disclosure in the description rather than inventing a one-off trailer
format for it. You remain fully responsible for the submission: understood,
explained on request, tested, and honest, exactly as if you had typed every
character.

The prohibited uses in `AI_STATEMENT.md` §11 apply to contributors too. The one
that matters most: **never weaken a test, an expectation, or a gate to make
something pass.**

## Reporting problems

- A security defect, or PHI reaching a log: [`SECURITY.md`](SECURITY.md) —
  do not open a public issue.
- A wrong claim in the documentation: an issue, and it is a welcome one.
- Conduct: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## Licence of contributions

By contributing you agree that your contribution is licensed under the same
terms as the project: `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR
GPL-3.0-only` ([`LICENSE.md`](LICENSE.md)). There is no CLA.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
