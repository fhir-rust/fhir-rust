# Promoting fhir-rust to professionals

**Non-normative, cross-family.** This is a research and planning document. It
lives in `help/` rather than `spec/` because it is not a specification and never
was: nothing here is normative, nothing here binds any family, and no code
conforms to it. *(Moved from `spec/promote/index.md` on 2026-08-26. The `PM-n`
numbers survive the move unchanged, in the spirit of `C0.5` — "including across
a file move".)*

It records *where the professionals who could use this repository actually are*,
*how each of those channels works mechanically*, and *what has to be true before
we may address any of them* — because the failure mode this repository has
already lived through once (**F-01**: six READMEs claiming measured results that
did not exist) becomes materially worse the moment the audience is external.

A promotion claim is a conformance claim made to strangers. The
[conformance matrix](../../spec/databases/conformance-matrix.md) is still the document
that decides what is true; nothing here may outrun it.

Items are numbered `PM-n` so a commit or a follow-up can cite one. The numbers
are permanent in the spirit of `C0.5`, though nothing here is normative.

**Assessed:** 2026-08-25; **revised 2026-08-26** — §3A added (the seven
missing assets, verified against the working tree) and three rows of §1
corrected. **Method:** live HTTP against github.com, docs.rs and
crates.io; `git remote -v`; directory listing of the repository root; web search
across the HL7®/FHIR® community, the Rust community, health-IT trade press, and
cold-outreach compliance guidance. Sources are listed at the end. Anything
marked *(unverified)* was derived from search snippets and must be confirmed
against the primary source before it is acted on.

---

## 1. What is actually true today

The good news is larger than the last assessment assumed, and one long-standing
caution is now closed.

| Fact | Evidence, 2026-08-25 | Bearing on promotion |
| --- | --- | --- |
| **The repository is public.** | `GET api.github.com/repos/fhir-rust/fhir-rust` → `"private": false`, `"visibility": "public"`; anonymous `GET github.com/fhir-rust/fhir-rust` → `200` | Closes `P-5` in [publishing readiness](../../spec/publishing.md), which recorded the URL as 404-ing anonymously and therefore *unverified rather than known-absent*. It resolves: the repo is reachable. **Every link we publish will work.** |
| **All 34 crates are on crates.io** at the versions their source claims | [publishing readiness](../../spec/publishing.md); `scripts/check-published-match.sh` → `34 matched` | `cargo add` works for a stranger. This is the single biggest promotion asset the repository has. |
| **docs.rs built them** | `GET docs.rs/fhir-sqlite-store` → `302` → `/fhir-sqlite-store/latest/fhir_sqlite_store/` → `200` | There is a public API reference to link to. Do not build a docs site before using this one. |
| Three git remotes: GitHub (fetch+push), Codeberg, GitLab | `git remote -v` | Mirrors exist. Pick **one** canonical URL for all outreach — GitHub, because that is what the ecosystem's registries and lists link to — and let the others be mirrors. Publishing three URLs splits every star, issue and inbound link three ways. |
| GitHub description is `FHIR Rust`; **no topics, no homepage, no Discussions**; 3 stars, 1 watcher | GitHub API | This is the first thing every channel below sends people to. It is currently unsearchable on GitHub and says nothing. See **PM-1**. |
| **No `CONTRIBUTING.md`, `SECURITY.md`, or `CODE_OF_CONDUCT.md` at the repository root.** One `CONTRIBUTING.md` exists, in [`fhir/`](../../fhir/CONTRIBUTING.md) — 101 lines, model-crate only | `find . -iname CONTRIBUTING*` | Professionals in clinical software check for these. A security-relevant library with no disclosure address is a red flag to exactly the audience we want. See **PM-74**. |
| **Benchmarks exist for one port of six.** `fhir-postgresql` has a real gated harness (`crates/fhir-postgresql-store/tests/bench.rs`) and measured numbers; the other five have `doc/benchmarks.md` pages but no harness | `find . -name bench.rs`; `fhir-postgresql/doc/benchmarks.md` | Better than the last assessment claimed, and still short of the pitch: the *JSONB comparison* the whole argument rests on has never been run. See **PM-72** and [`BENCHMARKS.md`](../../BENCHMARKS.md). |
| **No news route.** Six port `CHANGELOG.md` files and `fhir/CHANGELOG.md` exist — but **0 git tags, 0 GitHub releases** | `git tag \| wc -l` → `0`; `GET /repos/.../releases` → `[]` | 34 crates published and nothing a stranger can subscribe to. See **PM-70**. |
| The licence **is** precise — `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`, declared identically in all 33 manifests — but GitHub reads it as `NOASSERTION` and the expression names GPL | [`LICENSE.md`](../../LICENSE.md); `grep -rn ^license --include=Cargo.toml`; GitHub API `license.spdx_id` | Not the problem yesterday's assessment assumed. See the correction below and **PM-75**. |
| Two audit findings remain open, one **High** | [audit](../../spec/databases/audit.md): **F-51** (oracle DDL executed by hand), **F-67** (**High** — four TLS advisories now reach the shipping `fhir-mssql-store`; `native-tls` fails the handshake) | **F-67 is a promotion blocker for `fhir-mssql`, not for the repository.** It is a published crate with known advisories in its dependency tree. Do not promote that port by name until it is resolved or documented; see **PM-4**. |
| All six dialect annexes are still **proposed** (`X15.9`) | [`spec/index.md`](../../spec/index.md) *Gaps* | No annex may be cited as evidence for a conformance level — including in a slide, a blog post, or a Zulip message. |
| The `O10.4c` re-shred was verified on one developer machine, not in CI | [publishing readiness](../../spec/publishing.md) | "Verified" in outreach must mean *hosted CI*, per `C0.9`/`T11.13`. A green laptop is not a public claim. |

**Corrected 2026-08-26.** Four rows of the table above were wrong when first
written, in the direction this repository is most prone to — asserting an
absence without looking. `LICENSE.md` exists and is more careful than most of
the ecosystem's; `fhir/CONTRIBUTING.md` exists; seven `CHANGELOG.md` files
exist, and so do benchmarks for one port. The gaps are real but they are
*different* gaps, and §3A restates them
against what is actually on disk. The original `PM-2`, `PM-3` and `PM-5` are
superseded there and kept below only so the ids resolve (`C0.5`).

### The one-paragraph pitch this supports

> **fhir-rust stores FHIR R3/R4/R5 resources as real relational tables — typed
> columns, child tables, foreign keys, check constraints — and gives them back
> byte-identical, including decimal precision and partial dates. Six SQL engines
> (PostgreSQL, SQLite, MySQL, MariaDB, SQL Server, Oracle) from one normative
> specification and one shared engine. Pure Rust, no server, no CLI: it is a
> library you embed. Pre-release; the conformance matrix says exactly what each
> port has been shown to do.**

Every clause of that is defensible today. Nothing longer is.

---

## 2. The claims register

The rule that makes the rest of this document safe to execute. Print it beside
anything written for an outside reader.

| We may say | We may not say | Why |
| --- | --- | --- |
| "PostgreSQL is the reference port: full store, full test suite" | "production-ready", "battle-tested", "used in production" | No known deployment. `C0.9`. |
| "lossless round-trip is a tested invariant, decimals and partial dates included" | "fully FHIR-conformant" | Validation is `V9.x`-partial; terminology is a declared gap. |
| "six ports share one engine; the shared core is gated by `scripts/check-shared-core.sh`" | "all six are equivalent" | Conformance levels differ per port. |
| "R2, R3, R4, R4B, R5 and R6 modelled in code, one cargo feature each" | "supports R6" without saying it is a draft | `fhir-r6` is generated from 6.0.0-ballot3, is off by default, and can change between ballots. `fhir-r1` and `fhir-r7`–`r10` are name reservations containing no types. |
| "34 crates published to crates.io" | "released 1.0" / "stable API" | Pre-release, `0.x`/`4.x` mixed; no stability promise stated. |
| "live-verified against `azure-sql-edge` / `gvenzl/oracle-free` in CI" | "certified", "validated against SQL Server" | The container is not the product. Name the container. |
| "the audit register lists every known divergence, with two open" | silence about F-67 | Volunteering an open High finding is the strongest credibility signal we own. |
| "~7,355 generated tables for R5"; the PostgreSQL numbers in [`BENCHMARKS.md`](../../BENCHMARKS.md), dated and attributed to one dev machine | "scales to N million resources"; any store number for a port that did not produce it | 100k is the largest run; five ports have no harness. `W16.10`, and **F-64** is what happens when it is ignored. |

**PM-0.** Any external artefact — post, email, slide, pitch — cites the
conformance matrix by link and states *pre-release* in its first three
sentences. If a claim is not in the left column above, it needs evidence in the
matrix before it is written, not after it is questioned.

---

## 3. Prerequisites before any outreach

**Read with §3A**, which supersedes and sharpens several of these against what
is actually on disk. Roughly two days of work. Skipping them wastes every channel below, because
each one delivers a stranger to a GitHub page that currently says `FHIR Rust`.

- **PM-1 — Make the landing surface answer the three questions.** GitHub
  description (one sentence, from §1's pitch), homepage field (docs.rs or a
  future site), and topics: `fhir`, `hl7`, `healthcare`, `interoperability`,
  `rust`, `postgresql`, `sqlite`, `mysql`, `mariadb`, `sql-server`, `oracle`,
  `health-informatics`, `emr`, `database`. GitHub topic search is how health-IT
  engineers browse; with zero topics we are invisible to it. Enable Discussions
  so questions have somewhere to go that is not an issue.
- **PM-2 — Add `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.**
  *Superseded by **PM-74** (2026-08-26): `fhir/CONTRIBUTING.md` already exists, so
  this is an extension and a promotion, not a creation.*
  `SECURITY.md` needs a real address and a response window. For clinical
  software this is table stakes, and its absence is a reason not to evaluate
  us. `CONTRIBUTING.md` should point at [`AGENTS.md`](../../AGENTS.md) and its
  five rules rather than restate them.
- **PM-3 — Fix the SPDX expression** *— superseded by **PM-75** (2026-08-26):
  the expression is already correct and already in all 33 manifests; the problem
  is detection and GPL-in-expression, not precision.* Original text: so GitHub and licence scanners resolve it
  (e.g. `MIT OR Apache-2.0`, with the "contact us for custom licensing" prose
  kept *below* the machine-readable line, not instead of it).
- **PM-4 — Decide F-67 before naming `fhir-mssql` externally.** Either resolve
  the advisories, or publish a short statement in the crate's README saying
  which advisories apply, why the port is still published, and what a user
  should do. Both are defensible; silence is not, and the first person to run
  `cargo audit` after reading our post will find it.
- **PM-5 — Write one status page a non-contributor can read in 90 seconds.**
  *Sharpened by **PM-76** (2026-08-26): what segment A means by "conformance
  statement" is a specific FHIR artefact, not a status page.*
  The conformance matrix is the right document and the wrong artefact for a
  first-time reader: give it a preamble that says what the six levels mean and
  which port to pick. This is the page every channel below links to.
- **PM-6 — Produce one honest benchmark.** *Restated as **PM-72**.* Not a marketing number: a
  reproducible script, one machine, stated hardware, one claim — e.g. shred +
  reconstruct throughput for R5 `Observation` and `Patient` on
  PostgreSQL 18 and SQLite, against a JSONB-column baseline in the same
  database. The relational-vs-JSON trade-off is our entire argument and we
  currently assert it without measurement. A modest, clearly-scoped number
  beats none by a wide margin, and *beats an impressive one that a reader
  cannot reproduce by more*.
- **PM-7 — Record a 3–5 minute screen capture**: `cargo add`, `store.init`,
  `put`, `get` asserting equality including `"1974-12"`, then the SQL join from
  the README. No slides, no voice-over polish. Every channel below can consume
  it; it is the single most reusable asset for the money.

---

## 3A. The seven assets, against what is on disk

The owner's list names **six**; there are seven headings below because
`LICENSE.md` turns out to exist and to be good, so it is a *fix* rather than a
creation. The other six are genuinely absent as artefacts, though four of them
have most of their raw material already written somewhere in the repository.
That distinction matters: none of these is a research project. They are
**extraction, promotion and precision** jobs on material that mostly exists,
which is why they are worth doing before any channel in §5 is opened.

Each is stated as: *what exists → what is missing → what it must contain →
where it lives → what it blocks.*

### PM-70 — A news route

**Decided and half-delivered 2026-08-26.** The convention is
[`spec/git-tags-name-published-versions/`](../../spec/git-tags-name-published-versions/index.md)
(`TG1.x`, cross-family): one tag per independently-versioned unit,
`<unit>-v<version>`. Sixteen annotated tags now exist locally at `e28964e`,
`scripts/check-tags.sh` gates them, and sixteen release notes are drafted.
**Outstanding: pushing the tags and creating the releases**, which is what
actually produces `releases.atom` — the feed this item is about. Until that
happens the gap analysis below still describes what a stranger experiences.

**Exists.** Seven `CHANGELOG.md` files: one per port, plus
[`fhir/CHANGELOG.md`](../../fhir/CHANGELOG.md).

**Missing.** Anything a stranger can *follow*. `git tag | wc -l` is `0` and the
GitHub releases endpoint returns `[]` — for a repository with **34 crates
already published to crates.io**. There is no tag pointing at what was
published, no release note, no feed, no announcement list. An interested
architect who reads a Zulip post today has no way to hear about us again except
by remembering to look.

**What it must contain.** The cheapest correct answer is the one already
half-built: **tag what was published and write GitHub Releases against the
tags.** That yields `github.com/fhir-rust/fhir-rust/releases.atom` for free — a
real feed, indexed, no infrastructure, no mailing list to run under CAN-SPAM.
Backfill at minimum the 2026-08-22 publication of all 34 crates, since that is
the largest piece of news the project has ever had and it currently exists only
as a paragraph inside [publishing readiness](../../spec/publishing.md).

Then decide the one policy question: **a monorepo with 34 independently
versioned crates needs a tag convention** (`fhir-sqlite-store-v0.5.0`, or dated
repository-wide releases, or both). `W16.x` governs versioning and should be the
place that decides it, not this document.

**Blocks.** PM-21 (This Week in Rust wants canonical, stable URLs), PM-50, and
every channel in §5 — all of them end with "follow along", and today that
sentence has no object.

### PM-71 — A comparison page

**Exists.** [`doc/choosing-an-engine.md`](../../doc/choosing-an-engine.md),
which compares **our six ports to each other**, well. Outside that, the entire
repository mentions the ecosystem exactly once: a `client_crud` example in
`fhir/README.md` described as "REST CRUD vs HAPI".

**Missing.** Any comparison to the outside world. This is the single largest
promotion gap on the list, because segments **A** and **D** (§4) do not evaluate
software in isolation — they evaluate it against HAPI FHIR, Firely, Medplum,
Aidbox, and whatever they already run. A reader who cannot place us relative to
those will not adopt, and worse, will place us *wrongly*: as another FHIR server
competing with Medplum, which we are not.

**What it must contain.** One page, honest to the point of discomfort, along two
axes:

1. **Against the FHIR platform ecosystem** — HAPI FHIR (Java), Firely (.NET),
   Medplum, Aidbox / Health Samurai, LinuxForHealth, Google Open Health Stack.
   The framing that is both true and differentiating: *they store FHIR as
   documents and query around it; we shred it into typed relational tables and
   query it as SQL.* State plainly what they have that we do not — maturity,
   deployments, terminology services, IG/profile validation, SMART, a
   community, support contracts. A comparison page that shows us winning every
   row is read as marketing and discarded.
2. **Against the Rust FHIR crates** — `octofhir`, `fhirbolt`, `helios-fhir`,
   `fhir-sdk`, `fhir-rs`. Here the honest finding from §5F is unusually
   favourable: **none of them does relational shredding.** They do models,
   parsing, FHIRPath, clients. So this half of the page is a
   *complementarity* map, not a competition table, and it doubles as the
   opening line of the peer outreach in PM-42.1.

**Where.** `doc/comparison.md`, linked from the root `README.md` and from
`doc/index.md` beside `choosing-an-engine.md` — the two are the same question
asked at two scopes ("which engine" / "why this project at all").

**Careful.** Every external claim here is a claim about *someone else's*
software, made by us, in public. The claims register (§2) applies with the
polarity reversed: understate their weaknesses, do not characterise their
roadmaps, link to their own docs for every assertion, and date the page — it
will go stale, and a stale comparison is worse than none.

**Blocks.** PM-25 (a Show HN thread's first question is always "how is this
different from X"), PM-30/31, and all of §5F.

### PM-72 — Benchmarks

*Restates PM-6, and corrects it: the earlier "no benchmarks anywhere" was
wrong.* **Exists:** a real gated harness in `fhir-postgresql`
(`tests/bench.rs`), six `doc/benchmarks.md` pages, and dated measured numbers —
7,355 R5 tables installed in 5.8–9.5 s, 7,396/7,396 lossless live round-trip at
~13 ms/resource, 100k resources loaded at 6,146/s, 1.18 ms average read, and an
`EXPLAIN` audit that *fails the build on a sequential scan*, which is the one
performance property here that is actually gated. All from one Apple Silicon
developer machine, 2026-07-24, and none from hosted CI (`C0.9`).

**Missing.** Two things, and the second is the one that matters.

*Five ports of six have no harness at all* — and this is the exact ground on
which **F-64** (High, fixed 2026-08-03) was found: every non-PostgreSQL
`doc/benchmarks.md` used to carry PostgreSQL's numbers with the engine name
substituted, including a live round-trip for two ports that had no store. Any
new measurement work must not recreate that.

*And evidence for the* entire *argument is still absent.* The README asserts that "JSON
storage makes writing FHIR easy and querying it painful" and that normalized
storage inverts the trade. That claim is the reason to choose this project over
every alternative in PM-71, and it is currently unmeasured.

**What it must contain.** One reproducible script, one machine, stated hardware
and engine versions, and a small number of claims:

- shred + reconstruct throughput, R5 `Patient` and `Observation`, PostgreSQL 18
  and SQLite;
- **the query claim, which is the important one** — a representative analytic
  query (the README's family-name/observation-count join) against the same data
  stored as a JSONB column in the same database, same hardware, same planner;
- install cost: wall-clock and disk for `store.init` at ~7,355 R5 tables, which
  is the objection every reviewer will raise first.

Publish the harness, not just the numbers. A modest reproducible result beats an
impressive unreproducible one by a wide margin with this audience — and an
unreproducible one in a clinical-software pitch will be read as the same species
of claim as **F-01**.

**Blocks.** PM-30/31/33 (an op-ed on relational-vs-JSON with no measurement is
an opinion), PM-24, PM-60/61 (JOSS and JAMIA reviewers will ask), and the
"~7,355 tables" line in §2's claims register.

### PM-73 — A PHI statement

**Delivered 2026-08-26: [`PHI.md`](../../PHI.md).** It cites the sources below
rather than restating them, covers all four families, and names the open limits
(F-67, no Inferno run, the four `?` ports, nothing signed). The gap analysis
that produced it follows.

**Exists, and it is better than the gap implies.**
[`doc/trust-boundary.md`](../../doc/trust-boundary.md) is required by `PR12.8`
and is exactly the right table: twenty concerns, each marked *provided here* or
*provided by your deployment*, plus the reasoning for where the seam falls.
[`spec/databases/13-compliance-mapping.md`](../../spec/databases/13-compliance-mapping.md)
maps HIPAA §164.312(b)/(c)/(e), §164.502, GDPR Arts. 17/30/32 and IEC 62304 to
numbered requirements and to tests. The relevant guarantees are real and
numbered: `O10.2` (no PHI in logs at default level, tested by `T11.7`), `O10.7`
(encrypted database transport), `M3.15`–`M3.18` (attribution, tamper-evident
chain, append-only history, erasure with tombstone), `PR12.4`–`PR12.6`
(attribution and disclosure logging).

**Missing.** A statement addressed to the person who actually asks. What exists
is written *for a specification auditor*, in requirement-id language, lives
three levels down in `doc/`, and covers **the six ports only** — not the model
crate, not `fhir-store`, not `fhir-loco`. The people who will ask are a
hospital's privacy officer, a security reviewer filling in a vendor
questionnaire, and a CTO deciding whether depending on this creates an
obligation. None of them will find `doc/trust-boundary.md`, and none of them
reads `PR12.5` as an answer.

**What it must contain**, in plain language, at the root:

- **What the software does with PHI: nothing on its own.** It is a library. It
  opens no socket except to the database you configure, phones nothing home,
  collects no telemetry, and transmits nothing anywhere. *(Grep found no
  telemetry or analytics code; state this only after a deliberate egress
  review, and then state it as a verified fact, because it is the first
  question and a very strong answer.)*
- **What it records about people who use it** — attribution on every write and
  disclosure logging on every read, which is a *feature* answering HIPAA
  §164.312(b), and which a reviewer must know about because it means the audit
  tables also hold identifiers.
- **What it does not do** — no authentication, no authorization, no consent
  evaluation, no `meta.security` enforcement, no terminology or profile
  validation. Lift this straight from the trust-boundary table's right-hand
  column; it is the honest core of the document.
- **The four families' differing posture** — the model crate touches no
  database and no I/O at all, which is a materially different answer from a
  store's, and `fhir-loco` is different again.
- **The explicit non-claim**: these are components, not certified systems.
  §13 already says it well — *"they cannot make a deployment compliant, but they
  must not be the reason a deployment cannot be"* — and that sentence should be
  the statement's thesis.
- **Known open items that bear on it**, named: **F-67** (TLS advisories in the
  shipping `fhir-mssql-store`), no Inferno run, terminology validation absent.

**Where.** `PHI.md` or `doc/phi-and-privacy.md` at a findable level, linked from
the root `README.md`, from `SECURITY.md` (PM-74), and from `doc/index.md`; it
should *cite* `doc/trust-boundary.md` rather than restate it, so the normative
table stays single-source (`W16.x` SSOT).

**Blocks.** Every conversation with segment **D**, and realistically any
evaluation by a covered entity at all. Of the seven, this is the one most likely
to be the actual reason a professional does not proceed — and the one we are
closest to already having.

### PM-74 — `CONTRIBUTING.md`

**Delivered 2026-08-26:** [`CONTRIBUTING.md`](../../CONTRIBUTING.md) (leading
with rule 2, the shared-core gate), [`SECURITY.md`](../../SECURITY.md) (a real
address, a 7/30-day window, publish-if-silent, and F-67 named), and
[`CODE_OF_CONDUCT.md`](../../CODE_OF_CONDUCT.md) (Contributor Covenant 2.1 plus
a claim-accuracy clause). The gap analysis follows.

*Supersedes PM-2.* **Exists:**
[`fhir/CONTRIBUTING.md`](../../fhir/CONTRIBUTING.md), 101 lines, good — MSRV,
the green gate as a command table, a pointer to `AGENTS.md` and `spec/`.

**Missing.** The same at the repository root, and anything at all in the six
ports, `fhir-store`, and `fhir-loco`. A root file also has to carry what the
model crate's does not: **the five rules from [`AGENTS.md`](../../AGENTS.md)**,
and above all **rule 2 — the shared core is one file appearing six times, and
`./scripts/check-shared-core.sh` gates it.** A first-time contributor who edits
`fold.rs` in one port and opens a PR has done a divergence, not a fix, and will
have their work rejected for a reason no other repository would have taught
them. That has to be on the first screen.

Alongside it, and equally missing: **`SECURITY.md`** — a real disclosure
address and a response window, which for a library in this domain is
table stakes and whose absence is itself a reason not to evaluate us — and
**`CODE_OF_CONDUCT.md`**, which HL7's own community expects as a matter of
course.

**Blocks.** PM-21's call-for-participation issue (TWiR requires a link to
contribution guidelines and an OSI-approved licence), PM-60 (JOSS reviews
community documents directly), and every "we welcome contributors" sentence in
§5.

### PM-75 — `LICENSE.md`

*Supersedes PM-3, which was wrong.* **Exists, and is unusually careful:** a
five-way `OR` — `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR
GPL-3.0-only` — declared **identically in all 33 `[package]` manifests**
(verified: `grep -rn '^license' --include=Cargo.toml`), with the scope, the
`publish = false` fuzz crates, the immutability of already-published versions,
and the HL7 trademark position all stated explicitly. Very little in this
ecosystem is this precise.

**Missing** — two distinct things, neither of which is precision:

1. **Machine detection.** GitHub reports `license.spdx_id: "NOASSERTION"`,
   because a prose `LICENSE.md` offering five alternatives is not a licence file
   its detector recognises. Consequence: we are absent from GitHub's licence
   facet, and any automated ingest — registries, the HL7 Confluence listing
   (PM-11), corporate inventory tooling — records us as unlicensed. The fix is
   mechanical: keep `LICENSE.md` as the explanation and add the conventional
   files a detector reads (`LICENSE-MIT`, `LICENSE-APACHE` — which six ports
   already carry — at the root too).
2. **The GPL problem, which is a real adoption decision.** `OR` means the
   recipient chooses, so nothing obliges anyone to take GPL. But **automated
   licence policy at hospitals, EHR vendors and payers does not evaluate `OR`
   expressions** — a great many scanners flag any expression containing
   `GPL-2.0-only`/`GPL-3.0-only` and route it to legal, which in practice ends
   the evaluation. This is exactly segment **D**. The generous intent of
   offering five licences may therefore cost adoption among the audience this
   whole document is about.

**Decided 2026-08-26: the five-way expression stays**, and the scanner friction
is accepted as a known cost rather than traded away — narrowing the grant to
suit tooling would take a real freedom from users to buy a cosmetic metadata
improvement. The mechanical half was done the same day: [`LICENSES/`](../../LICENSES)
now carries the full text of all five options (the REUSE convention), which the
repository root previously shipped none of. Two defects surfaced while doing it —
the root offered five licences and shipped no text at all, and the six ports'
`LICENSE-APACHE` files contain Apache's *source-header boilerplate* rather than
the License, which §4 of Apache-2.0 requires be delivered to recipients.
Replacing the per-port files is outstanding.

The original framing is kept for the record: Note that **already-published
versions are immutable**, so either way the change applies from the next
published version of each crate — `LICENSE.md` already says so.

**Blocks.** PM-11, PM-60 (JOSS requires an OSI-approved licence — all five
qualify, so this is about detection only), and quiet, unobservable losses at
segment **D**, which is the worst kind.

### PM-76 — A precise conformance statement

*Sharpens PM-5.* **Exists.** The
[conformance matrix](../../spec/databases/conformance-matrix.md) — per requirement, per
port, the document this repository rightly treats as the truth — plus
`doc/choosing-an-engine.md`'s status grid, plus the six conformance levels
defined in `C0.x`.

**Missing.** Two different things that the phrase means to two different
readers, and neither is a status page:

1. **To segment A, "conformance statement" is a FHIR artefact.** It means a
   machine-readable **`CapabilityStatement`**: which releases, which resource
   types, which interactions, which search parameters, which `_include`s.
   `fhir-loco` serves one (`SV2.8`–`SV2.11`, corrected under **F-57**) — but
   the *libraries*, which are what we are promoting, publish no statement of
   coverage in any form a tool can read. For a project whose pitch is
   completeness of coverage, that is a conspicuous absence, and generating one
   from the relational map is very close to free: **the map already knows every
   resource type and every search parameter it can compile.** A generated,
   per-version, per-port `CapabilityStatement` — or the honest subset of one
   for a non-server — would be a genuinely distinctive artefact, and it is
   testable, which a prose page is not.
2. **To everyone else, it means a plain-language statement of coverage and its
   limits**: R2/R3/R4/R4B/R5 in code and R6 from a draft ballot; structural
   validation yes, terminology no, profile/IG no; search parameters compiled
   from the official `SearchParameter` definitions with per-port exceptions;
   **and no Inferno run has ever been performed** — which §13 already records
   for `fhir-loco` and which must be said out loud rather than left for a
   reviewer to discover.

**What it must contain.** One page stating, for each family and each port:
versions, resource coverage, operations, search, validation, what is explicitly
out of scope, the conformance level with its `C0.x` definition, and the date and
CI run the statement was true as of. Then a link to the matrix for anyone who
wants it requirement by requirement.

**Careful — this is the asset most likely to become the next F-01.** It is a
summary of a document that is itself carefully hedged, and every act of
summarising drops a qualifier. Two protections: state a date and a commit, and
**generate whatever can be generated** from the map and the matrix rather than
writing it by hand, so it cannot drift from the thing it summarises.

**Blocks.** PM-5, PM-11, PM-12 (a connectathon track proposal states coverage),
PM-13, and any procurement conversation whatsoever.

### Sequencing these against §6

They are not equal, and they do not all gate the same things.

| Asset | Effort | Gates | Do it |
| --- | --- | --- | --- |
| **PM-70** news route | convention **decided** (`TG1`), tags created, notes drafted; push + releases outstanding | everything ends with "follow along" | ◐ |
| **PM-74** `CONTRIBUTING.md` + `SECURITY.md` + `CODE_OF_CONDUCT.md` | ~~hours~~ **done 2026-08-26** | PM-21, PM-60, credibility with segment D | ✔ |
| **PM-75** `LICENSE.md` detection | hours, *plus an owner decision on GPL* | PM-11, PM-60, silent segment-D losses | Phase 0; raise the decision now |
| **PM-73** PHI statement | ~~a day~~ **done 2026-08-26** | every segment-D evaluation | ✔ |
| **PM-76** conformance statement | a day for the prose page; longer if generated | PM-11, PM-12, PM-13, procurement | Phase 1; generate the machine-readable half later |
| **PM-71** comparison page | ~~a day~~ **done 2026-08-26** — [`COMPARISONS.md`](../../COMPARISONS.md) | PM-25, PM-30/31, all of §5F | ✔ |
| **PM-72** benchmarks | days; one port is done, five need a harness, and the JSONB comparison needs designing | PM-30/31/33, PM-24, PM-60/61 | Phase 2 — the one thing worth delaying the announcement for |

Phase 0 in §6 was budgeted at "≈2 days". With PM-70 through PM-76 it is
realistically **four to five days, plus one owner decision (GPL) and one
specification decision (the tag convention)** — and the announcement in Phase 2
should not happen without PM-71, PM-72, PM-73 and PM-76, because those four are
precisely what a professional asks for in the first ten minutes.

---

## 4. Who "professionals" means here

Five distinct audiences. They read different things, believe different
evidence, and are reached by different channels. Addressing them with one
message is the most common way this kind of launch fails.

| Segment | Who they are | What they want to know | What convinces them |
| --- | --- | --- | --- |
| **A. FHIR implementers** | integration engineers at EHR vendors, HIEs, payers, national programmes | "does it round-trip *my* resources, and what does it not support?" | the conformance matrix, a connectathon appearance, round-trip evidence on their own bundles |
| **B. Health-data / analytics engineers** | data platform teams, research informatics, OMOP/OHDSI adjacent | "can I query this with plain SQL instead of `->>` spelunking?" | the SQL example, the relational map, a benchmark |
| **C. Rust systems engineers** | crate consumers, infra teams considering Rust in health | "is the API sane, is the MSRV kind, is it maintained?" | docs.rs, `RV1.x` (current − 3), the shared-core gate, CI badges |
| **D. Technical decision-makers** | CTOs, principal architects, heads of interoperability | "what is the risk of depending on this?" | the audit register, the honesty of the READMEs, a named maintainer |
| **E. Academic / research informatics** | AMIA, JAMIA, JOSS constituency | "is it citable and reproducible?" | a DOI, a paper, deterministic generation from official spec packages |

Segments **A** and **D** are where "professionals" most naturally lands, and
they are the two least reachable by the usual open-source launch playbook.
Segment **C** is the easiest to reach and the least likely to adopt for clinical
use. Budget attention accordingly: the Rust channels are cheap and fast and
should be used, but they are not the goal.

---

## 5. Channel catalogue

### 5A. The FHIR community — highest value, slowest, least tolerant of marketing

**PM-10 — chat.fhir.org (Zulip).** The centre of gravity: ~23,000 members, and
HL7 retired `chat.hl7.org` into it, so there is no second venue. Relevant
streams: `#implementers`, `#Announcements`, `#social`, `#analytics on FHIR`,
`#research`. Participation is governed by the FHIR Code of Conduct and the FHIR
IP Rules, **and contributions to the forum are considered public domain under
those rules** — read that before posting anything we would rather licence.
Community expectations explicitly warn against `@all`/`@everyone` on large
streams, and posting to `#Announcements` may be restricted.

*How to use it:* not as a launch pad. Join, subscribe, and answer other
people's questions about relational storage, shredding and search for several
weeks before mentioning our work. Then a single, factual post in
`#implementers` — what it does, what it does not, the conformance matrix link,
and an explicit request for round-trip failures against real bundles. That last
ask is the highest-value thing we can do anywhere: it converts promotion into
testing. *Cost:* time only. *Risk:* a vendor-pitch-shaped post here is
remembered by exactly the 200 people whose opinion matters most.

**PM-11 — HL7 Confluence: Open Source Implementations + FHIR Tools Registry.**
Two curated pages
(`confluence.hl7.org/display/FHIR/Open+Source+Implementations` and
`.../FHIR+Tools+Registry`) listing implementations by language — HAPI, Firely
and the rest. There is no Rust relational-storage entry to compete with. Getting
listed is a permanent, high-authority inbound link that segment **A** and **D**
actually consult. Editing needs an HL7 Confluence account *(unverified — the
page returned 405 to automated fetch; check the account requirement and the
edit convention manually)*.

**PM-12 — Connectathons.** The one venue where the claim is *tested* rather
than asserted, which is worth more to us than any article.
- **September 2026 WGM + FHIR Connectathon, 19–25 Sept 2026, Bethesda North
  Marriott, Rockville MD.** Track proposals run from ~12 weeks to ~6 weeks
  before, so that window has almost certainly closed for a *new* track — but
  **participating in an existing track does not require a proposal**, and
  registration is the only gate. This is the nearest real deadline.
- **January 2027 virtual FHIR Connectathon, 12–15 Jan 2027.** Virtual, so
  travel is zero. Its track-proposal window is roughly late Oct – early Dec
  2026: **this is the realistic slot for proposing a track of our own** (e.g.
  "relational persistence and lossless round-trip"), and it should be the
  calendar item that anchors the plan in §6.
- **CMS HL7 FHIR Connectathon** — free, virtual, three days, annual (the 7th
  ran 14–16 July 2026). Free and virtual makes it the cheapest credible
  appearance available; watch for the 2027 dates.
- **HL7 January 2027 virtual WGM, 25–29 Jan 2027.**

**PM-13 — FHIR DevDays.** The implementer conference. 2026 ran 15–18 June in
Minneapolis; its call for presentations closed 31 Jan 2026 with acceptances by
15 Mar and a 25% speaker discount — so expect the 2027 CFP to open around
year-end *(inferred from the 2026 cycle, not announced)*. A DevDays talk is the
single best fit for what we have built, and the abstract writes itself from §1.
Put the CFP on the calendar now; it is the kind of deadline that is missed by
three weeks.

**PM-14 — HL7 Work Groups.** Free to attend on the relevant calls. For us:
whichever group owns persistence/implementation concerns. Slower than
everything else and the only route to influencing the standard rather than
following it. Optional; list it so the choice is deliberate.

### 5B. The Rust community — fast, cheap, and the wrong audience to over-invest in

**PM-20 — r/rust.** The default announcement venue, and now doubly important
because **This Week in Rust no longer accepts project/tooling updates by PR —
its editors monitor r/rust instead**. So the r/rust post is the *input* to TWiR,
not an alternative to it. Post the crate announcement plainly; the weekly
"What's everyone working on this week?" threads on r/rust and
`users.rust-lang.org` are the low-key alternative for incremental progress.

**PM-21 — This Week in Rust.** Three separate mechanisms, only one of which is
a PR:
- *Blog posts / deep dives:* open a PR against `drafts/` in
  `rust-lang/this-week-in-rust`. No paywalls, no email-gated content, and
  LLM-generated articles must be disclosed. Link titles must match the page
  title exactly, use canonical URLs, and strip tracking parameters.
- *Project updates:* **not** by PR — post to r/rust (see PM-20).
- *Call for participation:* create a **labelled GitHub issue** with a difficulty
  level, a clear task description, a link to contribution guidelines, and an
  OSI-approved licence. This is the cheapest contributor-recruitment channel in
  the Rust ecosystem and it depends on **PM-2** and **PM-3** being done first.
- *Crate of the Week:* nominated by the community in a long-running thread on
  `users.rust-lang.org`. Nominating one's own crate is poor form; being
  nominated follows from PM-20 going well.

**PM-22 — Registry hygiene.** crates.io categories and keywords, `lib.rs`
presentation, README badges, and an `Awesome Rust` PR. Costs an hour, compounds
indefinitely, and is how segment **C** finds anything. (Note: crates.io's API
refused automated queries during this assessment under its data-access policy,
so current download counts are **unmeasured** — read them from the web UI.)

**PM-23 — Podcasts.** *Rust in Production* (corrode.dev, Matthias Endler,
bi-weekly) interviews companies using Rust in production and is a close
thematic fit — though "in production" is precisely the claim we may not yet
make, which argues for pitching it *after* a first real deployment rather than
now. *Rustacean Station* is community-run and more accessible for a
project-focused episode.

**PM-24 — Conferences.** *RustConf 2026*: 8–11 Sept, Montreal + online — too
close to submit to, worth attending online. *EuroRust 2026*: 14–17 Oct,
Barcelona + online. *Rust Nation UK*: London, February; its 2026 CFP is closed,
so the 2027 CFP is the target. A "we generated 7,355 SQL tables from a
healthcare standard and made round-trips exact" talk is a genuinely good Rust
conference talk — but see §7 on why this is the second-priority stage.

**PM-25 — Hacker News (`Show HN`) and lobste.rs.** Show HN fits: it is a real
thing a reader can try without a signup. Title formula: `Show HN: <name> – <one
sentence technical description>`; no uppercase, no exclamation marks, no
editorialising. Link to the repository, not a landing page. Be present in the
thread for several hours to answer. **Never** solicit upvotes — vote-brigading
is the fastest route to a ban, and messaging people to upvote violates the
guidelines outright. Reported outcomes range widely (thousands of visitors,
roughly one GitHub star per upvote for OSS tools) — treat those as folklore,
not a forecast. *Prerequisite:* PM-1, PM-5 and PM-7 must be done; an HN
front-page slot spent on a page reading `FHIR Rust` is not recoverable.

### 5C. Press and trade media

Understand what we are before pitching: an unfunded, pre-release open-source
library with no customers. Health-IT trade press covers *news* — funding,
rules, deployments, outages — and we have none. What we do have access to is
the **bylined-contribution** and **opinion** surface, which is open to
practitioners and is read by segments **A** and **D**.

**PM-30 — HISTalk "Readers Write".** The clearest published route.
Up to 500 words, original and not published elsewhere, edited for brevity;
**commercial pitches are refused**; anonymous submission is permitted.
Submissions go to the editor *(`mr_histalk@histalk.com` — search-derived,
**verify on histalk2.com before sending**)*. A 500-word piece on *why FHIR's
JSON-first storage assumption costs analytics teams* — mentioning our work once,
at the end — fits the format exactly. A 500-word piece about our crates does
not and will be rejected.

**PM-31 — Healthcare IT Today** (John Lynn) — daily coverage plus an active
interview podcast; runs contributed articles. **Health IT Answers** — similar
posture, publishes contributed interoperability pieces. Both are realistic
first placements. *(Their contact pages blocked automated fetch; get the
current submission address from the site.)*

**PM-32 — The Standard, HL7's official blog** (`blog.hl7.org`, FHIR topic).
Publishing here reaches segment **A** with HL7's imprimatur. Route in is
through HL7 community relationships (PM-10/PM-12/PM-14), which is another
reason those come first.

**PM-33 — The larger outlets** — Fierce Healthcare, Healthcare IT News,
Healthcare Innovation, Modern Healthcare, STAT. Realistic only with a news
hook. Two hooks may plausibly arrive: (i) a named organisation deploying a
port, (ii) tying the relational-storage argument to the 2026 regulatory wave —
CMS's Interoperability Framework, TEFCA, the CMS Interoperability and Prior
Authorization rule, and the industry's shift from compliance to strategy, all
of which the trade press is actively covering this year. Hook (ii) is available
now and is what a contributed op-ed should hang on.

**PM-34 — Newsletters and communities with reach into segment D.**
*Health Tech Nerds* (Sunday newsletter fed by an active Slack), *Out-Of-Pocket*
(~31k subscribers), *Digital Health Wire*, and *HealthDevHub* — a private
community explicitly for CTOs, VPs, solution architects and senior engineers in
digital health, FHIR included, which is the closest thing to a purpose-built
room for segments **C**+**D** at once. Join as a participant, not as a
broadcaster; these communities filter marketing aggressively and their value is
that they do.

**PM-35 — What not to do.** No press release on a wire service (openpr-style
distribution reaches nobody who matters and is visibly promotional); no PR
agency at this stage; no "market size" framing. Health-IT readers have a very
high tolerance for technical depth and a very low one for vendor language.

### 5D. Email

Direct email is the highest-intent channel for segments **A** and **D** and the
easiest to do illegally or annoyingly.

**PM-40 — Compliance.** Cold B2B email is lawful in the US under CAN-SPAM with
accurate sender information, a physical postal address, a working unsubscribe,
and opt-outs honoured promptly — penalties are per-message and large. In the
EU/UK, GDPR permits it under **legitimate interest** when contacting a named
professional about something relevant to their role, with the assessment
documented and opt-out honoured immediately. Canada's CASL is stricter
(consent-based); treat `.ca` recipients conservatively. **Never buy a list.**

**PM-41 — Deliverability mechanics.** SPF, DKIM and DMARC on the sending domain
are now effectively mandatory at Google and Microsoft. Practical volume is
~20–50 messages per mailbox per day; scaling past that means multiple mailboxes,
which for our purposes is a signal we are doing the wrong thing. Plain text,
under ~150 words: both providers now use transformer-based models that
recognise templated outreach, and a short plain message from a real person is
both more effective and more honest. Personalised-on-a-real-signal messages
report several-fold better response rates than generic ones — and the "signal"
should be genuine (they wrote the FHIR bulk-export IG; they run the OMOP
conversion; they filed the issue about JSONB query pain).

**PM-42 — Who to write to, in order.**
1. Maintainers of adjacent open-source FHIR projects (§5F) — peer-to-peer, not
   sales.
2. Research-informatics leads at academic medical centres — they have the
   relational-analytics pain and no procurement gate.
3. Interoperability architects at HIEs and public-health agencies.
4. EHR-vendor integration engineers met at a connectathon — **after**, never
   before, meeting them.

**PM-43 — The template.** Six sentences: who I am; the one thing we built; the
one sentence of why it might matter *to them specifically*; the honest status
line ("pre-release, here is the conformance matrix"); a single ask that costs
them under ten minutes (*"would you run one of your own bundles through the
round-trip test and tell me what breaks?"*); an unsubscribe/"tell me to stop"
line. The ask is a favour, not a demo request, and it is the part that works.

### 5E. Social

**PM-50 — LinkedIn is the primary social channel for segments A/D**, and the
only one where health-IT professionals are reliably present under their real
names. Post the honest-status piece there — the "we published 34 crates and
here are the two findings still open" framing performs unusually well with this
audience because it is so rare. Personal profile, not a company page: pages get
throttled and are read as marketing.

**PM-51 — Mastodon/Fediverse and Bluesky** carry much of the Rust community
and a meaningful slice of the FHIR one. Cheap, and the natural home for the
PM-7 video. **X** is optional; note only that TWiR still accepts a tweet to
`@thisweekinrust` as a submission path.

**PM-52 — YouTube** as the host for PM-7, so every other channel can embed one
URL. No channel-building programme; just a durable link.

### 5F. Adjacent projects — the most underrated channel here

We are not competing with these, and saying so plainly is what makes the
outreach land.

- **Rust FHIR peers:** `octofhir` (FHIRPath engine, FHIRSchema, model crates),
  `fhirbolt` (serde for R4/R4B/R5), `helios-fhir`, `fhir-sdk`, `fhir-rs`. **None
  of them does relational shredding** — they do models, parsing, FHIRPath and
  clients. That is a complement, not an overlap: a FHIRPath engine plus our
  storage is a better story for both. `octofhir-fhirpath` in particular would
  pair with our search layer. Reach out maintainer-to-maintainer (PM-42.1),
  offer interop, and ask for cross-links.
- **The wider FHIR platform ecosystem:** HAPI FHIR (Java), Firely (.NET),
  Medplum, Aidbox / Health Samurai, Metriport, LinuxForHealth, Google's Open
  Health Stack. These define what "normal" looks like to segment A; being
  understood in relation to them is most of the positioning work.
- **OHDSI / OMOP.** The [OHDSI forums](https://forums.ohdsi.org/) have a
  Developers category for open-source tooling on the OMOP CDM, and there is a
  curated `omop-list` of open-source OMOP tools. FHIR→OMOP is a live, painful,
  much-discussed problem and our relational shredding sits directly next to it.
  This is the best-fit community outside HL7 itself and it is largely
  overlooked by FHIR tool authors.

### 5G. Academic and citable

**PM-60 — JOSS (Journal of Open Source Software).** Peer-reviewed, free,
open-access, developer-friendly; requires an OSI-approved licence and a
research application. Yields a DOI, which converts the repository into
something segment **E** can cite and segment **D** reads as durable. The review
is a genuine software review — this repository's specification discipline is an
asset in it. Depends on **PM-3**.

**PM-61 — JAMIA Open.** Gold open access, health informatics, explicitly
encourages citable source code in public repositories. A short applications
paper on deterministic generation of a relational schema from FHIR
StructureDefinitions is a real contribution.

**PM-62 — AMIA Annual Symposium**, Dallas, November 2026 — the audience for
segment **E**, and a surprising amount of segment **D** attends.

---

## 6. Sequence

Ordered by dependency, not by appeal. The calendar anchors are real; the
groupings are ours.

**Phase 0 — before anything external (≈4–5 days).** PM-0 through PM-7, and
the Phase-0 half of §3A: **PM-70** (tags and releases), **PM-73** (the PHI
statement), **PM-74** (`CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`)
and **PM-75** (licence detection). Two things need an owner rather than an
afternoon and should be raised on day one: the **GPL-in-the-expression**
decision (PM-75) and the **tag convention** for 34 independently versioned
crates (PM-70, a `W16.x` question). PM-4 must be settled before `fhir-mssql`
is named anywhere.

**Phase 1 — presence without announcement (weeks 1–4).** Join chat.fhir.org and
subscribe to `#implementers` (PM-10); answer questions, announce nothing. List
on the HL7 Confluence pages (PM-11). Registry hygiene (PM-22). Email the Rust
FHIR peers (PM-42.1 / §5F). Register for the **19–25 Sept 2026 WGM +
Connectathon** in Rockville, MD (PM-12) — the nearest real deadline, and
participation needs no track proposal.

**Phase 2 — the announcement (weeks 4–6, once PM-71, PM-72, PM-73, PM-76 and
PM-7 exist — see §3A; those four are what a professional asks for in the first
ten minutes).** r/rust
(PM-20), which feeds TWiR (PM-21); the TWiR call-for-participation issue; Show
HN (PM-25) on a Tuesday–Thursday morning US time; LinkedIn (PM-50); the video
(PM-52). Run these within one week of each other so inbound attention overlaps.

**Phase 3 — the professional channels (months 2–4).** The
HISTalk/Healthcare-IT-Today contributed piece hung on the 2026 interoperability
hook (PM-30/31/33). The Zulip `#implementers` post, now with a benchmark and a
video behind it (PM-10). The OHDSI forum post (§5F). The
**January 2027 virtual connectathon track proposal**, drafted in the
late-Oct-to-early-Dec window (PM-12). The **DevDays 2027 CFP** when it opens,
expected around year-end (PM-13).

**Phase 4 — durable (months 4–12).** JOSS submission (PM-60); JAMIA Open paper
(PM-61); Rust conference CFPs for 2027 (PM-24); podcast pitches once there is a
real deployment to talk about (PM-23).

---

## 7. What we are actually optimising for

Worth stating, because it changes which of the above matters.

A GitHub star from a Rust developer who will never touch healthcare is worth
approximately nothing to this repository. **One integration engineer who runs
their own bundles through the round-trip test and files an issue is worth more
than a Hacker News front page**, because it produces evidence, and evidence is
the thing the conformance matrix is starved of.

So the metrics to watch are: issues filed by people outside the project;
round-trip failures reported against real bundles; connectathon participation;
crates.io downloads *by version over time* rather than in total; and inbound
questions on Zulip. Stars, upvotes and impressions are diagnostics of reach,
not of progress, and should never appear in a status document here.

The corresponding trap: promotion generates pressure to soften the status
language, because "pre-release, two open findings, dialect annexes still
proposed" reads as weakness to a marketer. In this domain it reads as competence
to the only people whose adoption matters. **The honesty is the differentiator**
— HL7's own community has watched a decade of vendors claim conformance they
did not have. A repository that publishes its own audit register with a High
finding still open is making a claim no competitor is willing to copy.

---

## 8. Sources

FHIR community and events —
[Welcome to the global FHIR Community](https://confluence.hl7.org/spaces/FHIR/pages/175606999/Welcome+to+the+FHIR+Community),
[chat.fhir.org Community Expectations](https://confluence.hl7.org/spaces/FHIR/pages/76158463/Chat.fhir.org+Community+Expectations),
[HL7 Zulip has moved](https://chat.hl7.org/),
[FHIR Community & Resources (MITRE)](https://mitre.github.io/fhir-for-research/modules/fhir-community),
[Open Source Implementations](https://confluence.hl7.org/display/FHIR/Open+Source+Implementations),
[FHIR Tools Registry](https://confluence.hl7.org/spaces/FHIR/pages/66941491/FHIR+Tools+Registry),
[FHIR Connectathons](https://confluence.hl7.org/display/FHIR/Connectathons),
[HL7 events](https://www.hl7.org/events/),
[HL7 Work Group Meetings](https://www.hl7.org/events/workgroupmeetings.cfm),
[CMS HL7 FHIR Connectathon 2026](https://ecqi.healthit.gov/save-date-2026-cms-hl7-fhir-connectathon-july-14-16-2026),
[FHIR DevDays](https://www.devdays.com/),
[DevDays call for presentations 2026](https://www.devdays.com/call-for-presentations-2026/),
[The Standard — HL7 blog, FHIR](https://blog.hl7.org/topic/fhir).

Rust community —
[This Week in Rust](https://this-week-in-rust.org/),
[TWiR repository](https://github.com/rust-lang/this-week-in-rust),
[Crate of the Week thread](https://users.rust-lang.org/t/crate-of-the-week/2704),
["What's everyone working on this week"](https://users.rust-lang.org/c/community/8),
[RustConf 2026](https://rustconf.com/rustconf-2026/),
[EuroRust 2026](https://eurorust.eu/),
[Rust Nation UK CFP](https://www.rustnationuk.com/call-for-papers),
[Rust conferences 2026 (corrode)](https://corrode.dev/blog/rust-conferences-2026/),
[Rust in Production podcast](https://corrode.dev/podcast/),
[How to do a successful Hacker News launch](https://www.lucasfcosta.com/blog/hn-launch),
[Hacker News posting guide](https://syften.com/blog/hacker-news-marketing/).

Rust FHIR peers —
[OctoFHIR](https://octofhir.tech/),
[fhirpath-rs](https://github.com/octofhir/fhirpath-rs),
[fhirbolt](https://lib.rs/crates/fhirbolt),
[helios-fhir](https://crates.io/crates/helios-fhir),
[fhir-sdk](https://crates.io/crates/fhir-sdk),
[fhir-rs](https://crates.io/crates/fhir-rs).

Press, communities, and the 2026 policy backdrop —
[HISTalk Readers Write](https://histalk2.com/),
[Healthcare IT Today](https://www.healthcareittoday.com/),
[Health IT Answers](https://www.healthitanswers.net/),
[Healthcare interoperability — 2026 predictions](https://www.healthcareittoday.com/2026/01/08/healthcare-interoperability-2026-health-it-predictions/),
[Top communities in digital health (Health Samurai)](https://www.health-samurai.io/articles/top-communities-in-digital-health),
[Must-join digital health communities](https://www.tellescope.com/blog/must-join-digital-health-communities),
[Out-Of-Pocket](https://www.outofpocket.health/),
[OHDSI Developers forum](https://forums.ohdsi.org/c/developers/7),
[omop-list](https://github.com/AndyRae/omop-list).

Email compliance —
[Cold email compliance: GDPR & CAN-SPAM](https://www.inboxkit.com/learn/cold-email-compliance-gdpr-can-spam),
[GDPR legitimate interest and cold email](https://litemail.ai/blog/gdpr-legitimate-interest-cold-email-2026),
[Cold email outreach best practices 2026](https://pipeline.zoominfo.com/sales/cold-email-outreach).

Academic —
[Journal of Open Source Software](https://joss.theoj.org/),
[JOSS documentation](https://joss.readthedocs.io/),
[JAMIA Open](https://academic.oup.com/jamiaopen),
[AMIA journals](https://amia.org/news-publications/journals).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
