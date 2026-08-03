# fhir-loco specification

Normative requirements for the **HTTP surface**: a FHIR RESTful API over one of
the database ports, built on Loco.rs, Axum, Tokio and Hyper.

This is the fourth family's specification, and it is new — created 2026-08-03,
after the owner settled where it belongs. Until then `fhir-loco` was governed by
nothing, while §7, §8, §10 and §12 of the database specification described it
from another family's document. See [`audit.md`](../../spec/databases/audit.md)
**F-04** and **F-58**.

## Requirement ids: `SV`

Every id here is `SV<section>.<n>`. **Ids are permanent** — never renumbered,
never reused, including across a file move. That rule is `C0.5` in the database
specification and it applies here for the same reason: an id is cited from
commit messages, tests and audit findings, and a renumber silently changes what
those citations mean.

`SV` was chosen because it collides with neither existing family. The database
specification uses `C0`, `S1`, `G2`, `M3`, `R4`, `H5`, `P6`, `V9`, `O10`,
`T11`, `PR12`, `M14`, `X15`, `W16`, `L1`–`L16` and `U1`–`U13`; the model crate
uses `R1.x`–`R14.x`. The repository has already been bitten once by a prefix
shared between two families — see [the `R4`
collision](../../spec/index.md#the-r4-collision--read-this-before-citing-r4x),
which cannot be fixed by renumbering and now needs a qualifier on every
citation. `SV` is the avoidance of a repeat, not a preference.

## Sections

| | Section | Prefix | Subject |
| --- | --- | --- | --- |
| 1 | [Scope and conformance](01-scope-and-conformance.md) | `SV1.x` | what this crate is, what it is not, and what it may claim |
| 2 | [Endpoints](02-endpoints.md) | `SV2.x` | routes, status codes, `OperationOutcome`, CapabilityStatement |
| 3 | [Trust and attribution](03-trust-and-attribution.md) | `SV3.x` | who is calling, and what the store is told about them |
| 4 | [Operations](04-operations.md) | `SV4.x` | limits, configuration, logging, deployment |

## What this specification adopts, and from where

`fhir-loco` did not appear in a vacuum. Two bodies of requirement already
described it, in a specification that governs something else:

- **§7 (REST API) and §8 (CLI)** of the database specification, **retired** as
  out of scope. That was right, and it meant out of scope *for the ports*. The
  ids `A7.8`, `A7.10`, `A7.11`, `A7.12` and `M8` are registered as dangling in
  `C0.16` and are still cited — `A7.12` caught a live defect in this crate
  (**F-57**), where the CapabilityStatement declared a read-only server while
  the router served writes.
- **The `[service]`-marked requirements** of §10 and §12 — `O10.1`, `O10.3`,
  `O10.5`, `O10.7`–`O10.9`, `V9.2`, `V9.3`, and most of `PR12.1`–`PR12.8`.

**Those ids are not moved here.** They stay where they are, retired or marked,
because `C0.5` makes them permanent and because renumbering across families is
exactly what produced the `R4` problem. Instead each section below **restates**
the obligation under an `SV` id and cites the original. A reader tracing
`A7.12` finds it in `C0.16`, and `C0.16` points here.

Where an obligation is restated rather than adopted verbatim, the difference is
stated at the point of use.

## Precedence

1. This specification is normative for `fhir-loco`.
2. The [database specification](../../spec/databases/index.md) is normative for
   the **store** this crate calls. `fhir-loco` MUST NOT restate a storage
   guarantee as its own: if history is append-only, that is `M3.17` and the
   store's, and this crate's obligation is only to not misrepresent it.
3. Where the two disagree about an externally visible behaviour, the database
   specification wins on **what is stored** and this one on **what is served**.
4. Nothing in a README, `book/`, or code comment is normative.

## Status

**This specification describes what `fhir-loco` MUST do. It is not a claim that
it does.** Several requirements below are currently unmet, and each says so at
its own id rather than in a summary that can drift out of date.

The honest summary as of 2026-08-03: routes, status codes and attribution are
implemented and tested; a body limit and request timeout are configured in
production; concurrency limits, an admin plane, `/metrics`, `$export` and
conditional create are not.

---

Part of the [fhir-rust monorepo specification](../../spec/index.md).
