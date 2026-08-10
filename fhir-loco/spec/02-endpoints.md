# 2. Endpoints

## Routes

- **SV2.1** The following routes MUST be served. `{version}` is a FHIR version
  namespace (`r3`, `r4`, `r5`); `{rtype}` is a resource type name.

  | Route | Methods |
  | --- | --- |
  | `/{version}/metadata` | `GET` |
  | `/{version}/{rtype}` | `GET` (search), `POST` (create) |
  | `/{version}/{rtype}/{id}` | `GET`, `PUT`, `DELETE` |
  | `/{version}/{rtype}/{id}/_history` | `GET` |
  | `/{version}/{rtype}/{id}/_history/{vid}` | `GET` (vread) |
  | `/{version}/$export` | `GET` (Bulk Data kick-off, `SV2.15`) |
  | `/{version}/$export-status/{job}` | `GET` (poll), `DELETE` (cancel) |
  | `/{version}/$export-file/{job}/{rtype}` | `GET` (NDJSON output) |

- **SV2.2** A request naming a version this process has not mounted MUST say
  **which versions are mounted**. "Not found" for an unmounted version is
  indistinguishable from a typo in the resource type, and the two have different
  fixes.

## Content type

- **SV2.3** Responses carrying FHIR content MUST use
  `application/fhir+json`. Plain `application/json` is wrong enough that
  conformance tooling rejects it.

## Status codes

- **SV2.4** These distinctions MUST each be preserved. They are the reason this
  crate exists as a translation layer rather than a thin proxy: the store draws
  every one of them, and losing one at the HTTP boundary discards information
  the store went to trouble to keep.

  | Code | Means | Must not be confused with |
  | --- | --- | --- |
  | `200` | read, search, history, vread succeeded | |
  | `201` | created, with the server-assigned id | `200` — a client cannot tell whether it created |
  | `204` | deleted | `200` with an empty body |
  | `400` | the request is malformed | `500` — this is the client's fault and is safe to describe |
  | `401` | no acceptable credential (`SV3`) | `403`, which this crate does not issue: it does not authorize |
  | `404` | never existed | `410` |
  | `410` | existed and was deleted | `404` — **the sharpest of these**. A client resolving a dangling reference needs to know the resource was deleted rather than mistyped, and a `404` invites a retry that will never succeed |
  | `412` | `If-Match` did not match the stored version, or `If-None-Exist` criteria matched more than one resource (`SV2.14`) | `409` |
  | `500` | this server's fault | `400` |
  | `503` | the store is unreachable | `500` — one is retryable, the other is not |

- **SV2.5** `If-Match` MUST be honoured on `PUT`, and MUST take a **weak** ETag
  (`W/"3"`), which is what FHIR specifies. A header that is present but
  unparseable MUST be an error rather than ignored: silently dropping a
  precondition turns an optimistic-concurrency check into an unconditional
  write, which is the failure the header exists to prevent.

## Errors

- **SV2.6** An error response MUST carry an `OperationOutcome`.

- **SV2.7** An `OperationOutcome` MUST NOT echo a submitted value, and MUST NOT
  disclose stored data or internal structure. It describes **the request**.

  Restates `A7.11`, retired with §7 and registered in `C0.16`. The store follows
  the same rule for its `Unsupported` error, and the reason is identical: an
  error is the one response an unauthenticated caller can reliably provoke, so
  it is the cheapest oracle in the system.

## CapabilityStatement

- **SV2.8** `GET /{version}/metadata` MUST return a `CapabilityStatement`
  generated from the mounted store's relational map, naming the FHIR version it
  actually serves.

  Restates `A7.12` (`C0.16`).

- **SV2.9** It MUST declare **every interaction the router serves, and no
  others**. Both directions are defects:

  - Declaring more than is served makes a client attempt what will fail.
  - Declaring less makes a conformance-driven client never attempt what would
    have worked.

  The second is what happened: this crate advertised `read`, `vread` and
  `search-type` while the router had carried `POST`, `PUT` and `DELETE` since it
  was written (**F-57**). `A7.12` is normally read as "do not declare what you
  cannot do", and every check written for it looked only for over-claiming.

- **SV2.10** A test MUST assert the correspondence between the declared
  interactions and the routed methods. Neither side's own tests can catch a
  disagreement: each is self-consistent while contradicting the other, which is
  the shape `U11a` names in the database specification.

  Met by `metadata_declares_every_interaction_the_router_serves`.

- **SV2.11** `software.name` MUST name **this** crate. It read `fhir-store`
  until 2026-08-03 — the name of the engine-agnostic persistence core after the
  split (**F-45**) — so every CapabilityStatement this service emitted
  identified itself as a different piece of software.

## Search

- **SV2.12** `_count`, `_offset` and `_total` are result parameters and MUST be
  treated as such; every other query parameter is a search parameter.

- **SV2.13** A search parameter the store does not support MUST be refused,
  naming the parameter, rather than silently ignored. A filter that is dropped
  returns **more** results than asked for, and the caller cannot tell.

## Conditional create

- **SV2.14** `POST /{version}/{rtype}` carrying an `If-None-Exist` header MUST
  be served as FHIR conditional create (restates `A7.10`, `C0.16`). The header
  value is search criteria in query-string form. The store's
  `conditional_create_audited` makes the search-then-create indivisible with
  respect to other writers — the HTTP layer MUST NOT search and create in two
  calls — and each of its three outcomes MUST be preserved:

  | Store outcome | Status | Body |
  | --- | --- | --- |
  | no match — created | `201`, with `Location` and `ETag` | the created resource |
  | exactly one match | `200`, with `ETag` | the existing resource, unchanged |
  | more than one match | `412` | `OperationOutcome`: the criteria are not selective enough |

  A header that is present but empty or unparseable MUST be a `400` rather
  than ignored, for `SV2.5`'s reason: silently dropping the precondition turns
  a conditional create into an unconditional one, which is the
  duplicate-writing failure the header exists to prevent. The
  CapabilityStatement declares `conditionalCreate` (`SV2.9`).

  *Served since 2026-08-07. Until then this requirement recorded the gap —
  the store capability existed with no route to it (**F-58**).*

## Bulk Data export

- **SV2.15** System-level `$export` MUST be served per the Bulk Data async
  contract (restates `M8`, `C0.16`): kick-off (`GET /{version}/$export`,
  authenticated, requiring `Prefer: respond-async`) answers `202` with a
  `Content-Location`; the status endpoint answers `202` + `X-Progress` while
  running, a JSON manifest (`transactionTime`, `request`,
  `requiresAccessToken: true`, per-type `output` entries with counts) on
  completion, and an `OperationOutcome` on failure; `DELETE` on the status
  URL cancels the job and removes its files; each output is
  `application/fhir+ndjson`, its line count MUST equal the manifest's count,
  and **every fetch is disclosure-logged** — an export is the largest
  disclosure this server can make. Jobs expire (`FHIR_LOCO_EXPORT_TTL_SECS`,
  default one hour) and their files go with them: exported PHI on disk has a
  lifetime. The CapabilityStatement declares the operation (`SV2.9`).

  The honest edges of the slice, stated rather than implied: `_type` is the
  one supported parameter — `_since` and every other parameter are **refused
  by name** (`SV2.13`'s principle; a silently dropped filter exports more
  than was asked); compartment-based `Patient/$export` and `Group/$export`
  are not served (the store has no compartment machinery); the export is a
  sequence of per-resource snapshot reads, **not one transaction-time
  snapshot** — `transactionTime` marks the kick-off, and a write racing the
  scan may or may not appear; file URLs are host-relative, resolved against
  whatever the fronting proxy serves.

  *Served since 2026-08-09; until then this requirement recorded the gap —
  the last of **F-58**'s feature gaps.*

## Search includes

- **SV2.16** `_include` and `_revinclude` MUST be served on type-level
  search, in exactly this slice — each edge stated rather than implied:

  - `_include=<type>:<param>` resolves the references the **matched**
    resources hold in `<param>`. `<type>` MUST equal the searched type; a
    mismatch is refused by name (`SV2.13`'s principle). An optional third
    segment (`_include=Patient:general-practitioner:Practitioner`) filters
    the resolved references to that target type.
  - `_revinclude=<type>:<param>` adds the resources of `<type>` whose
    reference parameter `<param>` points at a matched resource. A third
    segment, if present, MUST equal the searched type — on `_revinclude`
    it names what the parameter points at, which here is always the
    searched type.
  - `<param>` MUST be a reference search parameter of `<type>`; anything
    else — an unknown parameter, a non-reference parameter, a malformed
    value — is refused by name, never ignored: a silently dropped include
    returns less than the client asked for while looking complete.
  - `:iterate` is refused by name. Iteration is transitive closure, and a
    server that cannot bound it should not pretend to.
  - Only **relative** references (`Type/id`) resolve. Absolute URLs, `urn:`
    values and `#fragment` references are stored in a separate column and
    are not followed.
  - Included entries carry `search.mode = "include"`; matches carry
    `"match"`. The included set is deduplicated, and a resource that is
    already a match is not repeated as an include. A reference to a
    resource that does not exist is skipped: dangling references are data,
    not a request error.
  - Includes are computed from the **current page's** matches (the standard
    reading of includes under paging).
  - More than **1,000** included resources refuses the request with `400`
    `too-costly`, naming the cap — a truncated include silently returns
    less than it claims (`C0.11`'s shape).
  - Included resources are reads and land in the search's disclosure
    record's count (`PR12.5`).
  - The CapabilityStatement declares `searchInclude` per resource, exactly
    the reference parameters the map compiled (`SV2.9`).
    `searchRevInclude` is deliberately undeclared: reference columns here
    are untyped — any reference parameter may point at any type — so the
    honest list is every reference parameter of every type crossed with
    every resource, which is thousands of entries serving nobody. The
    operation is served; this paragraph is its declaration.

  *Served since 2026-08-10; until then `_include`/`_revinclude` were
  refused as unknown search parameters (**F-58**).*

## Type- and system-level history

- **SV2.17** `GET /{version}/{type}/_history` and `GET /{version}/_history`
  MUST be served, in exactly this slice:

  - Entries are **newest first** (`last_updated`, then `version_id`, both
    descending), deletions included as entries with no resource — history
    that hid its deletions would not be an audit trail (`H5.1`, `H5.2`).
  - `_count` bounds the result (default 50, clamped to 1–1,000). **There
    is no continuation link**: the response is the newest `_count` entries
    and says nothing about older ones. An honest bounded slice beats an
    approximate pager; a real cursor is future work, not an implied
    promise.
  - `_since` keeps versions written **at or after** the given instant
    (FHIR's definition). It MUST parse as RFC 3339; anything else is
    refused — a malformed instant silently compared as text would return
    wrong slices while looking right.
  - Every other parameter is **refused by name** (`SV2.13`'s principle) —
    `_at`, `_list`, and anything unknown. A silently dropped filter
    returns more than was asked.
  - The scope is a disclosure-logged read (`PR12.5`): one record per
    request, with the entry count.
  - The CapabilityStatement declares `history-type` on every resource and
    `history-system` on the REST endpoint (`SV2.9`).

  The store half is `fhir-sqlite`'s `history_page`, which merges the
  per-type history tables newest-first; instance-level `_history` is
  unchanged (`SV2.4`'s neighbours).

  *Served since 2026-08-10; until then only instance-level history
  existed (a gap `tasks.md` tracked from F-58's account).*

---

Part of the [fhir-loco specification](index.md).
