# News

Project news, in reverse order. [`CHANGELOG.md`](CHANGELOG.md) records what
changed in the software; this page records what happened to the project, and is
the page to read if you are deciding whether to look again in six months.

Press and analyst enquiries: see [Press contacts](#press-contacts) below.

---

## 2026-08-22 — every crate is published

All 34 packages are on crates.io at the versions their source claims. The
project is installable with `cargo add` for the first time:
[`INSTALL.md`](INSTALL.md).

That is a distribution milestone, not a maturity one. The project remains
**pre-release**, and the
[conformance matrix](spec/databases/conformance-matrix.md) is the document that
says what each of the six database ports has actually been shown to do.

## 2026-08-12 — all six ports install their full schema on their own engine

The last engine-specific blocker closed. MySQL and MariaDB could not install the
full R3/R4/R5 schemas at all until a byte-aware split landed in the shared
generator; `fhir-oracle`'s live CI job ran green on its first hosted execution
the same week. Six ports, six real engines, six live jobs.

## 2026-08-06 — the two scaffold ports reached Store level

`fhir-mssql` and `fhir-oracle` had no store implementation. Both now have one —
`put`, `get`, `delete`, `history`, `vread`, `search`, `verify_audit`, `purge`,
`log_access` — live-verified against `azure-sql-edge` and `gvenzl/oracle-free`
respectively.

## 2026-08-03 — the documentation honesty pass

A large share of this project's early documentation described software that did
not exist: READMEs claiming round-trip results for ports with no store,
benchmark pages carrying another port's measured numbers, task lists ticking
work never done. Those findings are fixed, and every one of them is written
down with evidence in [`spec/databases/audit.md`](spec/databases/audit.md).

We record this as news rather than hiding it, because it is the single most
useful thing an evaluator can know about how this project handles claims. See
also [`AI_STATEMENT.md`](AI_STATEMENT.md) §8.

## 2026-08-01 — one repository

Seven separate projects became one monorepo with one specification root: the
FHIR® model crate, six relational database ports, and — added the next day — the
persistence core and the HTTP surface.

---

## How to follow this project

Honestly: **there is not yet a good way**, and fixing that is tracked as `PM-70`
in [`help/outreach/index.md`](help/outreach/index.md). Today the options are:

| | |
| --- | --- |
| Watch the repository | [github.com/fhir-rust/fhir-rust](https://github.com/fhir-rust/fhir-rust) — the canonical remote; Codeberg and GitLab carry mirrors |
| Watch a crate | any of the 34 on [crates.io](https://crates.io) — new versions appear there first |
| Read this page | updated when something happens that is not a code change |

There are no git tags, no GitHub releases, no release feed, and no mailing list.
When releases exist, `https://github.com/fhir-rust/fhir-rust/releases.atom` will
be the feed, and this section will say so.

## Press contacts

| | |
| --- | --- |
| **Contact** | Joel Parker Henderson — [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com) |
| **Role** | Sole maintainer ([`MAINTAINERS.md`](MAINTAINERS.md)) |
| **ORCID** | [0009-0000-4681-282X](https://orcid.org/0009-0000-4681-282X) |
| **Citation** | [`CITATION.cff`](CITATION.cff) |

**What this project is, in one paragraph, if you need to quote it:**

> fhir-rust stores HL7® FHIR® standard R3/R4/R5 resources as real relational
> tables — typed columns, child tables, foreign keys, check constraints — and
> gives them back byte-identical, including decimal precision and partial
> dates. Six SQL engines (PostgreSQL, SQLite, MySQL, MariaDB, SQL Server,
> Oracle) from one normative specification and one shared engine. Pure Rust,
> no server, no CLI: it is a library you embed. Pre-release; the conformance
> matrix says exactly what each port has been shown to do.

**Three things we will not say, and would rather you did not print:** that it is
production-ready (no known deployment), that it is FHIR-conformant (validation
is partial and no Inferno run has been performed), or that any port is
"certified" (nothing here is certified by anyone). The reasoning behind that
list, and what *can* be substantiated, is public in
[`help/outreach/index.md`](help/outreach/index.md) §2.

**If you are writing about this project**, [`RFC.md`](RFC.md) says what it is
asking the world for — chiefly: run your own FHIR bundles through the round-trip
test and tell us what breaks.

**For comparisons with other FHIR implementations**, please read
[`COMPARISONS.md`](COMPARISONS.md) first — it states what other projects do
better, which is usually the part a story needs.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
