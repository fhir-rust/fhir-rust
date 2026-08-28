# Request for comments

**This project is pre-release and is asking for help thinking, not just help
typing.** Below are the questions it most wants answered, the kind of feedback
that moves them, and what happens to what you send.

If you only have five minutes, skip to [the one question](#the-one-question).

## The one question

**Does the round trip hold on *your* data?**

Every round-trip result this project publishes comes from the example resources
shipped inside the official FHIR® specification packages — 7,399 of them, all
lossless. That corpus is a modelling artifact. It is clean, it is small, and it
was written by the same people who wrote the specification.

Your data is not like that. It has extensions nobody documented, profiles from a
national programme, decimal precisions that matter clinically, identifier
systems with surprising characters, and resources that have been migrated
through three EHRs. **We have never seen any of it.**

```rust
store.put(&resource, &Audit::cli()).await?;
let back = store.get(ty, id).await?.unwrap();
assert_eq!(back, resource);
```

If that assertion fails, that is the most valuable bug report this project can
receive. You do not need to diagnose it, and you must not send us PHI — a
redacted resource, a synthetic reproduction, or even a description of the
element shape is enough to act on.

## What else we want to learn

### 1. Is a 7,355-table schema actually usable?

The whole argument for this project is that a relational schema beats a JSON
blob for anything analytical. R5 generates 7,355 tables and 58,405 data columns.
Nobody outside this project has sat down with that schema and tried to answer a
real clinical question.

- Are the table and column names guessable, or do you need the relational map
  open beside you the whole time?
- Is the `rid`/`ords` addressing scheme for child tables comprehensible after
  five minutes, or is it a permanent tax?
- What did you have to look up that should have been obvious?

### 2. Is our benchmark design fair?

[`BENCHMARKS.md`](BENCHMARKS.md) says plainly that the measurement the argument
rests on — a normalized query versus the same data in a JSONB column, same
database, same hardware — **has never been run**. Before we run it, we would
rather have it critiqued than defend it afterwards:

- What query shapes actually matter in your workload? Cohort selection?
  Longitudinal patient timelines? Quality-measure denominators? Bulk export?
- What would make the comparison dishonest in a way we would not notice —
  indexing choices, cache warmth, the JSONB schema we pick as the baseline?
- What number would change your mind, in either direction?

### 3. Are these the right six engines?

PostgreSQL, SQLite, MySQL, MariaDB, SQL Server, Oracle. That set was chosen by
reasoning, not by demand.

- Is anyone actually going to run clinical FHIR storage on Oracle or SQL Server,
  or is that effort better spent deepening PostgreSQL?
- Is DuckDB, ClickHouse, or a columnar target more useful than a seventh OLTP
  engine, given the analytics framing?
- Does SQLite's embeddability matter to you, or is it a curiosity?

### 4. Which of the uncompiled search parameters matter?

1,823 of 1,972 R5 `SearchParameter`s compile — 92.4% (down from a claimed
94.8% after **F-38** found 51 of them silently dropping a `where()` value
restriction). The rest are composites, specials, and `exists()`-style
expressions, and each records its reason in the map asset. We do not know
which of them you would actually miss.

### 5. Is the trust boundary drawn in the right place?

[`doc/trust-boundary.md`](doc/trust-boundary.md) puts authentication and
authorization outside these libraries and keeps attribution and disclosure
logging inside. The reasoning is that the perimeter knows *who*, and only the
store knows *which rows*.

- Does that split survive contact with a real deployment, or does it push work
  onto you that the library should have done?
- Is `Audit::unattributed()` a pragmatic escape hatch or a hole?
- Should `meta.security` label enforcement be in scope after all?

### 6. Do the conformance levels communicate anything?

`C0.x` defines six levels, and the
[conformance matrix](spec/databases/conformance-matrix.md) is the document this
project asks you to trust above every README. Does it work?

- Could you tell, in under five minutes, whether a given port does what you
  need?
- Is `?` — "the code is shared from a port where this is tested, and nothing
  tests it here" — a useful distinction or a cop-out?

### 7. Is the Rust API idiomatic?

`put`, `get`, `delete`, `history`, `vread`, `search`, `verify_audit`, `purge`,
`log_access`, over `serde_json::Value` rather than typed resources. That last
choice is deliberate — the ports do not depend on the model crate — and it is
the one we are least sure about.

### 8. What did you decide *not* to use this for, and why?

Nobody sends this and it is the most useful signal there is. A paragraph on why
you closed the tab is worth more than a feature request.

### 9. Did we miss a driver alternative for F-67?

The oldest open decision in the repository has a decision now: four TLS
advisories reach the shipping `fhir-mssql-store` crate through its driver
stack, `native-tls` fails the handshake as an escape route, and — as of
2026-08-28 — the owner has accepted the risk formally rather than chase a
replacement, after investigating and pricing three alternatives (a
from-scratch driver, a fork of the one upstream fix that exists,
`prisma/tiberius#419`, and one newer crate disqualified on sight). Full
account: `M14.34` in `fhir-mssql/spec/14-mssql-dialect.md`.

That decision was made with the alternatives we could find. **If you know a
TDS driver, a fork, or a mitigation this search missed** — or if you run SQL
Server in production and have a view on whether encrypted transport with a
known-vulnerable certificate parser beats no port at all — that is still a
genuinely useful thing to send, and it is the one question here with a real
chance of reopening a closed decision rather than just informing an open one.

### 10. Should publishing move to CI?

**Decided 2026-08-26: it does not.** Documented laptop publishing is
permanent — the owner's judgment that GitHub is not reliable enough to hold
the publish path, made hours after an Actions major outage stalled every
hosted run. The inert `publish.yml` workflows are deleted, the process is
`spec/publishing.md`, and the residual (one machine, one person, a
long-lived local token) is recorded in MAINTAINERS.md. The question stays
here because the *evidence request* still stands: if you have run Trusted
Publishing at this scale — 34 crates, one repository — what broke, and
which failure mode would you rather explain to a security reviewer? A
strong answer could reopen the decision; that is what this file is for.

## What kind of feedback helps most

This project runs on evidence, and its house style is that a claim names what
demonstrated it. Feedback shaped the same way lands hardest:

**Especially useful**

- A reproducible case: input, expected, actual, engine, version.
- "Requirement `X` contradicts requirement `Y`" — with both ids.
- "Your documentation says `X`; I ran it and got `Y`."
- "I could not work out how to `X` from the docs, and here is where I got stuck."
- Domain correction from someone who works in health informatics: a FHIR
  behaviour we have misread, a regulation we have mischaracterised, a clinical
  assumption that does not hold.
- Dialect expertise: "no DBA would ship that index strategy on Oracle, because…"

**Less useful, honestly**

- Feature requests with no use case behind them. Tell us the problem, not the
  solution you have picked.
- "Rewrite it in `X`" / "use `Y` instead of `Z`" without a trade-off analysis.
- Style opinions on generated code. It is generated; the generator is the thing
  to argue with.
- Benchmarks run in debug mode. Shredding is allocation-heavy and debug numbers
  are a different shape, not merely slower.
- Anything containing real patient data. Please — redact or synthesise first.

**Disagreement is welcome and does not need to be softened.** Telling this
project it is wrong is a service, including when the thing that is wrong was
written by the maintainer. Rule 5 of [`AGENTS.md`](AGENTS.md) — say what you did
not verify — applies to everyone here, and it cuts both ways.

## How to send it

| | |
| --- | --- |
| A defect, a wrong claim, a round-trip failure | a GitHub [issue](https://github.com/fhir-rust/fhir-rust/issues) |
| Anything security-sensitive, or PHI reaching a log | [`SECURITY.md`](SECURITY.md) — **not** a public issue |
| A conversation rather than a report | email the maintainer ([`MAINTAINERS.md`](MAINTAINERS.md)). GitHub Discussions is not enabled yet |
| Something you would rather say privately | the same address; say so and it stays private |

## What happens to what you send

- **Every report gets a response.** With one maintainer that may not be fast;
  see [`SECURITY.md`](SECURITY.md) for the windows that *are* committed to.
- **A confirmed divergence between what we say and what the code does becomes a
  numbered finding** in the [audit register](spec/databases/audit.md), with the
  evidence and with your credit unless you ask otherwise. That register is
  public and includes the findings that are unflattering — there is no separate,
  quieter list.
- **A specification flaw becomes a requirement change**, with the id preserved
  (`C0.5`) and the reasoning recorded.
- **Nothing is silently absorbed.** If we decide not to act, you get told that
  and why, which is a worse answer than a fix but a better one than silence.

## What this project will not do with your feedback

It will not be used to claim adoption. "Someone opened an issue" is not "someone
deployed it", and [`help/outreach/index.md`](help/outreach/index.md) §2 lists
what may and may not be said publicly about this project. If you would like your
organisation named as a user, tell us explicitly; otherwise you will not be.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
