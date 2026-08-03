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
  | `412` | `If-Match` did not match the stored version | `409` |
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

## Not implemented

- **SV2.14** `If-None-Exist` conditional create is **not served**, and the
  capability exists: the store implements `conditional_create_audited`. This is
  a route away from working. Restates `A7.10` (`C0.16`); tracked as **F-58**.

- **SV2.15** `$export` (Bulk Data) is **not served**. Restates `M8` (`C0.16`),
  and it is one of the three §13 compliance rows that depends on this crate.

---

Part of the [fhir-loco specification](index.md).
