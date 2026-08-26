# 12. Trust, principal, and audit

These libraries do not authenticate users — but "authentication is the
perimeter's job" cannot mean "the record of who did what is nobody's job". This
section defines the seam: how an authenticated identity reaches the store, and
what the store guarantees about recording it.

Requirements marked **[service]** describe how a service establishes the
principal (`C0.17`). The store-side obligations — `PR12.4`, `PR12.5`, `PR12.6`,
and the `Audit` value that carries the identity — are in force today, and are
the reason this section survives the retirement of §7 intact: identity
*verification* is outside, identity *recording* is not.

- **PR12.1** **[service]** A service accepts a **principal** from a configured
  trusted header, and optionally a purpose of use and an on-behalf-of patient.
  Values are length-capped and character-validated before use.
- **PR12.2** **[service]** A principal header is trusted **only** when the
  request arrives from a configured trusted proxy. From anywhere else the header
  is ignored, not honored — otherwise any client could assert any identity.
- **PR12.3** **[service]** A require-principal mode makes an unattributable
  request a 401. Deployments handling PHI are expected to set it. Without it,
  writes record `actor = 'unauthenticated'` and the service logs a startup
  warning.
- **PR12.9** **[service]** A service MAY establish the principal by
  **verifying a signed token** instead of by trusting a header, and where it
  does, `PR12.1`–`PR12.3` are satisfied by construction rather than by
  configuration: there is no header to length-cap, no proxy allowlist to get
  right, and no unattributable request to decide about, because a request
  without a valid token never reaches a handler.

  `actor_source` MUST name the mechanism and its version — `paseto:v4.public`,
  not `token` — so an auditor reading a history row years later can tell which
  scheme produced the attribution and judge what it was worth.

  A service that takes this option SHOULD NOT also accept the header. Two
  mechanisms mean one is a fallback, and a fallback that requires no key is the
  one a misconfigured deployment gets: it runs unauthenticated while appearing
  configured. `fhir-loco` removed its header for exactly this reason, and
  because it had never implemented `PR12.2`'s trusted-proxy check — it could
  not tell a perimeter-set header from a client-set one, which is the condition
  under which `PR12.2` says the header must be ignored rather than honoured.

  The asymmetric case is preferred where the choice exists. A symmetric token
  key verifies *and* mints, so every instance that checks credentials could
  issue them; an asymmetric one lets a service hold only the public half and be
  incapable of forging an identity it would then record as fact.

- **PR12.3a** The store MUST make the unattributed case **explicit and
  visible**, never a default that looks like an identity. A store API that
  accepts a write with no audit argument, and silently records something
  plausible, converts a deployment mistake into a permanent false record. The
  attribution a caller supplies MUST be a value it constructs deliberately —
  `Audit::unattributed()`, `Audit::cli()`, `Audit::principal(actor, source)` —
  and `actor_source` MUST record which, so an auditor can distinguish "the
  perimeter asserted this identity" from "nobody did".
- **PR12.4** Every state change records its principal in the history audit
  envelope (`M3.15`), in the same transaction as the change — never
  best-effort, never asynchronous.
- **PR12.5** Every **read** — read, vread, history, search, export — appends an
  access record to `<namespace>.fhir_access_log(ts, request_id, actor, client,
  interaction, rtype, id, version_id, outcome, result_count, reason)`.

  Disclosure logging is the requirement regulators actually audit first, and a
  store that records only mutations cannot answer "who looked at this patient".

- **PR12.6** Access logging has three modes: `sync` (the record commits before
  the result is returned — slowest, strongest, **the default**), `async`
  (batched through a bounded queue with a flush interval), and `off` (permitted
  only when explicitly allowed, and logged loudly at startup).

  `sync` is the default because the failure it prevents is the one that cannot
  be repaired afterwards: a disclosure with no record is indistinguishable,
  later, from a disclosure that never happened. A deployment that needs the
  throughput can opt into `async` knowingly; the reverse default would make
  every deployment silently accept a loss window it never chose. `async` MUST
  say at startup that records queued when the process dies are lost, and MUST
  drain its queue on graceful shutdown.

  In **every** mode a disclosure that cannot be recorded MUST fail closed — the
  read is refused, never served unlogged. A saturated queue is therefore an
  error returned to the caller, not a dropped record.

  Four counters are exported per version, and the distinction between them is
  the point: `enqueued` and `written` describe a healthy path; `refused` counts
  reads turned away to keep the log honest; `lost` counts records the writer
  could not commit *after* the data was served. Non-zero `lost` is an incident —
  disclosures happened that the log does not show — while non-zero `refused` is
  the system working as designed under strain. Queue depth is derived from these
  rather than tracked separately, so it can never report a value the counters
  contradict.

- **PR12.7** **[service]** A service accepts the standard `X-Provenance` header
  on writes and stores the supplied `Provenance` resource, linking it to the
  version it describes. It MAY additionally synthesize `AuditEvent` resources
  from the access log on demand, so the audit trail is queryable as FHIR® rather
  than only as SQL. The store-side half — that a `Provenance` resource can be
  stored and linked to a version — is in force.
- **PR12.8** The trust boundary is stated in one place, in the documentation, as
  a table: what the port guarantees, what the perimeter must provide
  (authentication, authorization, scope and compartment enforcement, consent,
  rate limiting per identity, TLS termination), and what neither provides yet. A
  boundary nobody can point at is not a boundary.

  See [`doc/trust-boundary.md`](../../doc/trust-boundary.md) for the monorepo-wide
  table; a port's book chapter MUST agree with it or amend it by number.

---

Part of the [fhir-databases specification](index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
