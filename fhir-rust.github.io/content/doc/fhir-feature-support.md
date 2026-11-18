# FHIR® feature support

Non-normative. This page answers one question: **which parts of FHIR does this
repository serve today, and which does it refuse?** It is derived by hand from
two sources, and where it disagrees with either, the source is right and this
page is the defect:

- the HTTP surface's requirements, [`fhir-loco/spec/02-endpoints.md`](../fhir-loco/spec/02-endpoints.md)
  (`SV2.x`), and
- the database ports' [conformance matrix](../spec/databases/conformance-matrix.md)
  and [search section](../spec/databases/06-search.md) (`P6.x`).

Compiled 2026-11-17. It is a statement about **what the specifications say is
served**, not a fresh test run — see [What this page is not](#what-this-page-is-not).
For a running server the authoritative, machine-readable answer is
`GET /{version}/metadata`, which `SV2.8`–`SV2.10` require to declare every
interaction the router serves and no others, and a test
(`metadata_declares_every_interaction_the_router_serves`) holds it to that.

Legend: `•` supported · `~` supported with a stated limit · `—` not supported
(refused, with the reason where one is recorded) · `?` specified but not
verified per feature.

## FHIR versions

| Release | Model (`fhir/`) | Schema in each port | Served by `fhir-loco` |
| --- | :-: | :-: | :-: |
| R3 (`r3`) | • | • (`S1.1`) | • |
| R4 (`r4`) | • | • (`S1.1`) | • |
| R5 (`r5`) | • | • (`S1.1`) | • |

`fhir-loco` mounts the versions the process was configured with and answers a
request for an unmounted version by saying which are mounted (`SV2.2`). It
serves one backend per process: `fhir-sqlite` (default) or `fhir-postgresql`
(`fhir-loco/spec/01-scope-and-conformance.md`). The other four ports are
libraries without a mounted HTTP surface.

## RESTful interactions (`fhir-loco`, route table `SV2.1`)

| Interaction | Status | Notes |
| --- | :-: | --- |
| `read` | • | `404` never existed, `410` deleted — kept distinct (`SV2.4`) |
| `vread` | • | `/{rtype}/{id}/_history/{vid}` |
| `create` | • | `201` with `Location` and `ETag` |
| `update` (`PUT`) | • | weak ETag `If-Match` honoured; unparseable header is an error (`SV2.5`) |
| `delete` | • | `204`; idempotent |
| `history-instance` | • | |
| `history-type`, `history-system` | ~ | newest first, deletions included, `_since` and `_count` only; **no continuation link**; other parameters refused (`SV2.17`) |
| `search-type` | ~ | see [Search](#search) |
| `search-system` | — | not served; listed as low priority in `tasks.md` |
| `conditional create` | • | `If-None-Exist`; `201`/`200`/`412` (`SV2.14`) |
| `conditional delete` | • | `204`/`412`; criteria-less request refused `400` (`SV2.19`) |
| `conditional read` (`If-None-Match`, `If-Modified-Since` → `304`) | — | not in `SV2`; proposed in `tasks.md` |
| `conditional update` | — | not in `SV2` |
| `patch` | — | no `PATCH` route (`SV2.1`) |
| `batch` Bundle | — | `501`, named "unbuilt" (`SV2.18`) |
| `transaction` Bundle | — | `501`; atomicity by compensation was rejected on purpose (`SV2.18`) |
| `capabilities` (`/metadata`) | • | generated from the relational map (`SV2.8`) |

Errors are `OperationOutcome`s that never echo a submitted value (`SV2.6`,
`SV2.7`). `403` is not issued: the crate authenticates but does not authorize
(`SV2.4`, `SV1.4`).

## Search

Search is specified in `P6.1`–`P6.9` and compiled by the shared generator, so
the *shape* is common to all six ports; the per-engine operation exists in all
six (`search` / `search_full` are `•` in the matrix; `search_page` is `~` on
Oracle).

| Feature | Status | Basis |
| --- | :-: | --- |
| token, string, date, number, quantity, reference, uri parameters | • specified | `P6.1` |
| composite and special parameters | — | `P6.1` permits deferral, "documented per parameter in generated docs" |
| string: case-, accent- and composition-insensitive prefix | • | `P6.2`, `P6.6`; `Ærø` is found by `aero` |
| `:exact`, `:contains` | • | `P6.2` |
| date prefixes `eq ne lt gt ge le sa eb` | • specified | `P6.2` |
| token `system\|code` | • | `P6.2` |
| `_count` (default 50, max 1000), `_offset`, `_total` | • | `P6.3`, `SV2.12` |
| `_sort`, `_id`, `_lastUpdated` | • specified | `P6.3`; not independently verified per port |
| `_include` / `_revinclude`, one hop | • | `SV2.16`; `:iterate` refused; relative references only; more than 1,000 included resources refused `400` |
| chained reference search | ~ | works on mssql and was found broken once and fixed (**F-65**); the matrix lists deeper chained coverage as open |
| other modifiers (`:missing`, `:not`, `:above`, `:below`, `:text`, `:in`) | ? | not named in `P6`; no inventory exists |
| `_elements`, `_summary` | — | not in `SV2`; proposed in `tasks.md` |
| `_has`, `_filter`, `_text`, `_content`, `_list` | — | not in `P6` |
| unsupported parameter | refused by name | `SV2.13` (the HTTP layer); `P6.5` allows the library to warn instead |

Every search value is bound as a parameter, never concatenated (`P6.8`), and
include expansion is capped with the truncation reported (`P6.7`).

## Operations

| Operation | Status | Notes |
| --- | :-: | --- |
| `$export` (system, Bulk Data async) | ~ | `_type` only; `_since` and everything else refused by name; not one transaction-time snapshot; every fetch disclosure-logged (`SV2.15`) |
| `Patient/$export`, `Group/$export` | — | no compartment machinery |
| `$everything` | — | no compartment machinery |
| `$validate` | — | out of scope by decision (`SV1.4`) |
| `$expand`, `$lookup`, `$translate`, other terminology | — | not specified |
| Subscriptions | — | absent from the spec; needs a scope decision |

## Storage semantics beneath the API (database ports)

Lossless round-trip, versioning and history, the tamper-evident audit chain and
access log, and erasure are properties of the ports, not of FHIR's REST
contract. They are summarised here because they are what the FHIR interactions
above are served *with*; the per-port detail belongs to the matrix.

| Capability | pg | sqlite | mysql | mariadb | mssql | oracle |
| --- | :-: | :-: | :-: | :-: | :-: | :-: |
| `put` / `get` / `delete` / `history` / `vread` / `search` | • | • | • | • | • | • |
| `conditional_create`, `conditional_delete` (store) | • | • | — | — | — | — |
| `put_audited`, `delete_audited` | • | • | — | — | — | — |
| `history_page` (type / system) | • | • | — | — | — | — |
| `transact_audited` | • | ~ | — | — | — | — |
| `verify_audit`, `purge`, `log_access` | • | • | • | • | • | • |
| `export` store operation | — | — | — | — | — | — |

**Consequence:** the conditional, audited and type/system-history endpoints
above can be exercised only against the two backends `fhir-loco` mounts
(`fhir-postgresql`, `fhir-sqlite`). The other four ports have no HTTP surface
and lack the store operations those endpoints need. Conformance level per
port (Reference / Store) is in the matrix and is not repeated here; it is a
claim about what has been verified, not about what the code contains (`C0.8`).

## What this page is not

- **Not a test result.** Nothing on this page was re-run when it was written.
  `•` means a requirement says it is served and the matrix or a named test
  backs it; `?` means nobody has inventoried it.
- **Not a clinical or regulatory compliance statement.** Mapping to regulatory
  controls is [`13-compliance-mapping.md`](../spec/databases/13-compliance-mapping.md).
  Nothing here is endorsed by HL7®.
- **Not generated yet.** The honest long-term form is a `CapabilityStatement`
  plus a per-parameter search inventory generated from the relational map, so
  it cannot drift (tracked in `tasks.md`, Tier 4). Until then, update this
  page in the same commit as any change to `SV2`, `P6` or the matrix.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
