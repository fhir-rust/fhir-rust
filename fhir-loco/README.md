# fhir-loco

A FHIR® RESTful API server: Rust, [Loco](https://loco.rs),
[Axum](https://github.com/tokio-rs/axum), Tokio and Hyper.

The HTTP half of this repository's FHIR persistence. The engine-agnostic
storage half is [`fhir-store`](https://crates.io/crates/fhir-store); the SQL
engines are the six `fhir-<engine>` ports.

This project is the HTTP surface for the FHIR database libraries in the same
monorepo — not sibling repositories, which is what this line said until
2026-08-03; all four families live in `fhir-rust/fhir-rust` (**F-11**). Those
libraries — `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`, `fhir-postgresql`,
`fhir-mssql`, and `fhir-oracle` (all six at Store level or above since
**F-65**/**F-68**; an earlier revision called the last two "the two
scaffolds") — are deliberately embeddable and carry no server of their own
(`C0.17`, `C0.18`), so that a program wanting FHIR storage does not also acquire
a web framework. The split is the point: storage guarantees live in the library,
HTTP lives here.

**Specification: [`spec/index.md`](spec/index.md)**, ids `SV1.x`–`SV4.x`, added
2026-08-03. Every externally visible promise below is a numbered requirement
there — `410` versus `404` is `SV2.4`, `If-Match` is `SV2.5`, the
CapabilityStatement is `SV2.8`–`SV2.11`, PASETO is `SV3.2`–`SV3.6`.

What this crate does **not** do is recorded at its own id rather than in a list
that drifts: no concurrency limit (`SV4.2`, a Loco 1.0.1 framework limit),
and within `$export` no `_since` and no compartment exports — the served
slice's edges are stated in `SV2.15` itself. The listener
speaks plain HTTP behind a TLS-terminating proxy, and `SV3.11` enforces
that posture: a non-loopback bind refuses to boot unless
`FHIR_LOCO_TLS_TERMINATED_UPSTREAM=true` acknowledges the proxy.
(Conditional create, the admin plane, and system-level async Bulk Data
`$export` — all formerly on this list — are served: `SV2.14` and `SV4.3`
since 2026-08-07, `SV2.15` since 2026-08-09. Set `FHIR_LOCO_ADMIN_BIND`
for `/health`, `/ready`, and `/metrics` on their own listener;
`FHIR_LOCO_EXPORT_DIR`/`FHIR_LOCO_EXPORT_TTL_SECS` govern where export
files live and how long.)

## What belongs where

Everything this service claims — versioned history, the tamper-evident audit
chain, search semantics, GDPR erasure, decimal fidelity — is the storage crate's
work. This layer translates HTTP to store calls and back, and its own job is
narrow: get the status codes right.

Where the store draws a distinction, the HTTP layer has to preserve it. A
resource that was deleted answers **410 Gone**; one that never existed answers
**404 Not Found**. Collapsing those would tell a caller that a record it once
held never was.

## Running it

The library owns schema installation, so a database is prepared with the
storage crate and served from here:

```sh
export FHIR_LOCO_DB=/path/to/fhir.sqlite
export FHIR_LOCO_ASSETS=../fhir-sqlite/crates/fhir-sqlite-map/assets
cargo run --bin fhir_loco-cli -- start
```

Both variables have defaults (`fhir.sqlite` and `assets`). A FHIR version is
mounted only if its schema is actually installed — an empty database would
otherwise advertise a CapabilityStatement for resources that cannot be read, and
a server lying about what it can do is worse than one admitting it serves
nothing.

Backends are selected by configuration rather than by Cargo feature, so a
deployment can change engine without a rebuild.

## Authentication

**PASETO v4.public, required.** There is no unauthenticated mode.

```sh
# 32-byte Ed25519 public key, hex. The issuer signs; this service only verifies.
export FHIR_LOCO_PASETO_PUBLIC_KEY=<64 hex chars>
```

The process **refuses to boot** without it — absent, empty, non-hex, or the
wrong length all stop startup. Every request must carry `Authorization: Bearer
v4.public.…`; missing, malformed, tampered or expired is `401` before any
handler runs. The `sub` claim becomes the audit actor, recorded as
`paseto:v4.public` (`PR12.9`).

### There is no header fallback

An earlier revision accepted `x-fhir-loco-principal` when no key was
configured. It is gone, for two reasons.

Two modes meant the unsafe one was the default: a deployment that forgot the
variable ran unauthenticated while looking configured, and the only signal was
a log line at boot.

And it never satisfied `PR12.2`, which says a principal header is trusted *only*
when the request arrives from a configured trusted proxy. This service had no
proxy allowlist — it could not tell a perimeter-set header from a client-set
one, so under `PR12.2` the header should have been ignored rather than honoured.
A signature it can verify itself is a property of the request rather than an
assumption about the network.

There is likewise no mode that checks a token when present and shrugs when
absent: that accepts an unauthenticated request whenever the caller omits the
header, which is the same as not checking.

### Why PASETO rather than JWT

A JWT names its own algorithm in a field the attacker also controls, which is
the root of the `alg: none` and RS256→HS256 confusion families. PASETO fixes the
algorithm per version: a `v4.public` token is Ed25519 and cannot claim to be
anything else.

### Why v4.public rather than v4.local

`v4.local` is symmetric — verifying and minting use the same key, so every
instance that checks credentials could also issue them, and one read of the
configuration would be enough to impersonate any principal. `v4.public` is
asymmetric: this service holds only the public half and **cannot mint a token
at all**.

### What this does not change

**Authorization** — scopes, compartments, consent — is still your deployment's.
This establishes *who*, not *what they may do*. The storage crates still do not
authenticate; that boundary is unmoved, and this is the layer §12 always
expected to hold it.

## Endpoints

| | |
|---|---|
| `GET /{version}/metadata` | CapabilityStatement |
| `GET /{version}/{type}` | search |
| `POST /{version}/{type}` | create (server assigns the id) |
| `GET /{version}/{type}/{id}` | read |
| `PUT /{version}/{type}/{id}` | update, honouring `If-Match` |
| `DELETE /{version}/{type}/{id}` | delete |
| `GET /{version}/{type}/{id}/_history` | history |
| `GET /{version}/{type}/{id}/_history/{vid}` | vread |
| `GET /{version}/{type}/_history` | type-level history (`SV2.17`) |
| `GET /{version}/_history` | system-level history (`SV2.17`) |

Search accepts `_count`, `_offset`, and `_total`; everything else is treated as
a search parameter. `If-Match` takes a weak ETag (`W/"3"`); a header that is
present but unparseable is an error rather than being ignored, because a client
asking for optimistic concurrency and not getting it is worse than one told no.

Reads are recorded in the store's disclosure log — "who looked at this patient"
is usually the first question an audit asks, and reads are where it is easiest
to forget. A logging failure is reported but does not fail the request.

`{version}` is `r3`, `r4`, or `r5`. Responses are `application/fhir+json` —
plain `application/json` is wrong enough that conformance tooling rejects it —
and carry a weak `ETag`, weak because two representations of one version may
differ in whitespace.

Errors are `OperationOutcome`. Their text describes the *request*, never
storage, so it can be returned verbatim without leaking schema names or stored
values; anything else is logged and answered with a generic 500.

## Status

SQLite by default; PostgreSQL by configuration (`SV1.10`). Read, vread,
create, update, delete, search (including
`_include`/`_revinclude`, `SV2.16`), history at instance, type and system
level (`SV2.17`), conditional create (`SV2.14`), conditional delete
(`SV2.19`), and system-level `$export` (`SV2.15`) work and have been
exercised end to end against a real database. (Until 2026-08-10 this
paragraph still listed conditional create as unimplemented — stale since
2026-08-07; until 2026-09-03 it listed conditional delete as
unimplemented too — stale the same day it was written, since the store
capability already existed and only the route was missing.) Not
implemented: transaction and batch Bundles — the store refuses those
explicitly rather than pretending.

`FHIR_LOCO_BACKEND=postgresql` (with `FHIR_LOCO_PG_DSN` and the postgres
relmap assets in `FHIR_LOCO_ASSETS`) mounts `fhir-postgresql` instead —
same HTTP surface, verified end to end by `tests/pg_backend.rs` against a
live server (`SV1.10`). The other four ports are not wired: none carries
the audited-write surface this crate calls. (Until 2026-08-10 this
paragraph said the MySQL and MariaDB stores were "still being written in
their own repositories" — stale twice over: all six ports have had stores
since F-65/F-68, and they live in this monorepo, not sibling repos.)

`serde_json` is built with `arbitrary_precision`, and that is not a preference:
without it a FHIR `decimal` of `9.60` is parsed into an `f64` and handed back as
`9.6`, destroying on the way out the precision the storage layer works hard to
preserve.

## Tests

`cargo test` runs the endpoint suite against a real SQLite database, installed
through the library the same way an operator would. There is no mock store: the
interesting behaviour is exactly the translation between the store's
distinctions and HTTP's, and a mock would assert that the test author
understands those rather than that the code does.

The suite needs `fhir-sqlite` checked out beside this repository, since it reads
the relmap assets from there.

## A note on Loco's hooks

Store initialisation lives in `Hooks::before_run`, not `Hooks::boot`. `boot` is
not on the path the `start` command takes, so initialising there leaves every
request answering 503 while `/_health` stays green — a load balancer sees a
healthy instance serving nothing. If you add startup work, put it in
`before_run` and verify that it actually runs.

## Getting help with Loco

[A quick tour](https://loco.rs/docs/getting-started/tour/) or
[the complete guide](https://loco.rs/docs/getting-started/guide/).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
