# 1. Scope and conformance

## What this crate is

- **SV1.1** `fhir-loco` is a **FHIR® RESTful API server** over one of the
  database ports. It translates HTTP to store calls and store outcomes to HTTP.
  It MUST NOT implement storage semantics of its own.

  The distinction is the whole design. A version conflict is the store's
  `Conflict { expected, found }`; this crate's job is to turn that into `412`
  and not to decide when it happens. Where this crate starts *deciding* storage
  behaviour, two implementations of the same rule exist and will diverge.

- **SV1.2** It MUST mount exactly one store implementation per process, and MAY
  mount several FHIR versions of it. Today that store is `fhir-sqlite` and the
  versions are `r3`, `r4` and `r5`, selected by the `{version}` path segment.

- **SV1.3** The database ports MUST NOT depend on this crate, and MUST NOT
  acquire a web framework by depending on anything that does (`C0.17`,
  `C0.18`). A program wanting FHIR storage should not also acquire an HTTP
  stack.

## What it is not

- **SV1.4** It is **not** a FHIR facade over an arbitrary backend, not a
  validation service, and not an authorization server. Authentication is
  verified here (`SV3`); *authorization* — whether this principal may read this
  patient — is out of scope and MUST be stated as such wherever the trust
  boundary is documented.

## Conformance levels

- **SV1.5** This crate conforms at exactly one of three levels. The level is a
  claim about what has been **verified**, not about what has been written — the
  same rule as `C0.8`, and for the same reason.

  | Level | Means |
  | --- | --- |
  | **Draft** | It builds and serves. Endpoint behaviour is untested or tested only against a mock. |
  | **Tested** | Every route in `SV2` is exercised against a real store, and the status-code distinctions in `SV2.4` are asserted individually. |
  | **Hardened** | Tested, plus `SV4`'s limits, and a live run under an adversarial client — oversized bodies, malformed tokens, aborted requests. |

- **SV1.6** The level MUST be justified by tests that **run**. A level claimed
  on the strength of code that exists is Draft.

  This mirrors `C0.9`, and it has already bitten once: `fhir-oracle` produced a
  full Oracle schema, installed it on a live engine, and stayed at Scaffold
  because the verification was a human at a terminal rather than a test
  (**F-51**). The same standard applies here.

- **SV1.7** **Current level: Tested.** `cargo test` runs the endpoint suite
  against a real SQLite database with a real installed schema — not a mock,
  because the thing worth testing is the join between the store's distinctions
  and HTTP's, and a mock asserts only that the test author understood it.

  Not Hardened: `SV4.2`'s concurrency limits and `SV4.3`'s admin plane are
  unmet, and no adversarial run has been performed.

## Honesty

- **SV1.8** Documentation MUST NOT describe a capability at a level above the
  crate's, and MUST NOT describe one it does not have at all (`C0.11`).

  This is not a hypothetical import from the database family. The port `book/`
  directories described a `fhir-<engine> serve` command for months; the
  CapabilityStatement this crate serves advertised a read-only server while the
  router accepted writes (**F-57**). Both were documentation-shaped defects with
  externally visible consequences.

- **SV1.9** Where a requirement here is unmet, the requirement MUST say so at
  its own id. A summary of gaps kept somewhere else goes stale; the id is what a
  reader cites and therefore where the truth has to live.

- **SV1.10** *(added 2026-08-10, with the second backend)* The storage
  backend is selected by **configuration at boot** — `FHIR_LOCO_BACKEND`,
  `sqlite` (default) or `postgresql` (with `FHIR_LOCO_PG_DSN`; TLS policy
  from `PGSSLMODE`, the store's own `O10.7` mechanism) — never by Cargo
  feature, so a deployment changes engine without a rebuild. One backend
  per process; every mounted FHIR version is served by that backend. The
  HTTP surface MUST be backend-agnostic: the same requirements in this
  spec, the same status codes, the same distinctions (410/404, conflict,
  refusal-by-name), whichever engine is configured — verified end to end
  against live PostgreSQL by `tests/pg_backend.rs`, a separate binary
  because the choice is per process.

---

Part of the [fhir-loco specification](index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
