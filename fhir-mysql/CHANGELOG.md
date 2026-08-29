# Changelog

> ## ⚠ Entries before 0.4.0 are inherited
>
> This changelog began as `fhir-postgresql`'s. Entries that name PostgreSQL,
> `tokio-postgres`, `NoTls`, `PGSSLMODE`/`PGSSLROOTCERT`, `jsonb`, or advisory
> locks describe **that** port's history; this port's driver is `mysql_async` and its
> equivalents differ or do not exist. Audit **F-62**, `W16.12`.
>
> They are kept rather than rewritten because the shared half — the generator,
> the shred/reconstruct engine, the fold — genuinely does change with these
> releases, and because a changelog is a historical record. What is specific to
> this port is stated where it differs.

## Unreleased

**MSRV raised 1.90 → 1.96** (spec `spec/rust-msrv-n-minus-2/`, `RV1.1`
amended from N-3 to N-2, 2026-08-29). Verified with `cargo +1.96 check
--all-targets --workspace --locked` before being declared, per `RV1.5`.

**Security — the database connection can now use TLS, and verifies by
default.** This port previously set no TLS options at all and was built with
`mysql_async`'s `minimal` feature, which excludes TLS entirely: every
connection was plaintext and nothing could change that. Both ports are
Store-level, so real patient data crossed that link (`O10.7`, audit **F-54**).

`FHIR_MYSQL_SSL_MODE` takes MySQL's own `--ssl-mode` values —
`DISABLED`, `REQUIRED`, `VERIFY_CA`, `VERIFY_IDENTITY` — and defaults to
`VERIFY_IDENTITY`. `FHIR_MYSQL_SSL_CA` names a root certificate for a
private CA. `Store::connect_with` takes a mode explicitly.

Note that MySQL's own `REQUIRED` encrypts without validating anything, so it
is not the default here. `PREFERRED` is rejected with an explanation rather
than approximated: the driver either requires TLS or does not use it, and
silently choosing either one would be wrong.

**This will break local development against a stock container**, whose
certificate is self-signed — that is the change working. `scripts/db.sh` now
prints `FHIR_MYSQL_SSL_MODE=DISABLED` next to the DSN for that case.

## 0.4.0 — tamper evidence that survives the database (2026-07-27)

**Breaking:** `ChainBreak` gained an `algorithm` field, so a break is
reported per algorithm rather than once. It is now `#[non_exhaustive]`, so
the next field is a patch rather than another break. Installed schemas need
`fhir-mysql init --upgrade` for the new digest, tag, and counter-signature
columns; existing rows keep verifying, with the new chains reported as
beginning where their first digest appears.


**Changed — hash chains are computed by fhir-mysql, not by PostgreSQL.** The
digests were computed in SQL. They are unkeyed over a published pre-image,
so a database that computes them holds everything needed to forge them — and,
decisively, a MAC can only be introduced where the database is not. A key
stored where an attacker already has write access protects nothing. Writing
and verifying both moved into the application; `pgcrypto` is no longer
required, because nothing hashes in SQL.

**Added — a second chain in a second design family.** History is chained
under SHA-256 *and* SHA3-256. The value is family diversity, not digest
length: SHA-256 is Merkle–Damgård and SHA-3 is a sponge, so the line of
cryptanalysis that took MD5 and SHA-1 — both Merkle–Damgård — cannot take
both. A clinical record may be retained for decades, longer than anyone can
promise a single hash function will stand. Both are FIPS-approved (180-4,
202), and `verify-audit` reports each separately so a reader relies on
whichever their regime recognises (M3.16a).

**Added — a keyed tag, which is what actually resists an attacker.**
`HMAC-SHA-256` (FIPS 198-1) over the same pre-image, keyed from
`FHIR_MYSQL_CHAIN_KEY` and never written to the database or logged. It sits
alongside the digests rather than replacing them, so an outside auditor
holding only the data can still check the chain while forgery still needs
the secret. The key id travels with the tag (`k1:9f86d0…`), so rotation is
additive: retired keys stay loadable via `FHIR_MYSQL_CHAIN_KEYS_RETIRED` and
turning a key over does not invalidate history all at once (M3.16b).

Only a tag mismatch is a finding. A missing tag, a tag naming a key this
process does not hold, and a malformed tag are each reported as what they
are — reporting a key-distribution problem as forgery would burn an incident
response.

**Added — `fhir-mysql chain-witness` and automatic checkpoints.** A tag proves a
row was not rewritten; it says nothing about a row that is *gone*, and a
chain missing its last version verifies perfectly because nothing left
behind refers to what was removed. The witness digests every chain head for
recording off-box. Checkpoints are also emitted as INFO on an
`audit_checkpoint` log target at startup, after every erasure, and each
`--checkpoint-interval-mins`, so a deployment already shipping logs has a
witness for free. The line carries only counts and digests — no PHI — so it
can be retained longer than ordinary logs (M3.16c).

A checkpoint is a witness only if it lands where the database cannot reach.
Logs shipped off-host qualify; a log table in the same database does not.
fhir-mysql cannot enforce that and does not claim to.

**Fixed — the pre-image hashed a timestamp rendered in the session's
TimeZone.** A verifier connecting from a different zone would have
recomputed different bytes and reported every row broken. Both sides now
render UTC explicitly.

**Fixed — the erasure tombstone terminated only one chain**, leaving the
second null at the row an auditor examines hardest. The suite passed;
counting the columns did not.

**Changed — the specification is one file per section.** `spec/index.md` had
reached 39k against a 40k limit; it is now an index over thirteen files.
Requirement numbers are unchanged.

## 0.3.1 — supply chain (2026-07-26)

**Fixed — a published version depended on an unmaintained crate.**
`rustls-pemfile` is unmaintained (RUSTSEC-2025-0134) with no safe upgrade.
The direct use is gone, replaced by `rustls-pki-types`' `PemObject` — the
same code `rustls-pemfile` delegated to — and `axum-server` moves to 0.8,
which drops it as well. 0.3.0 carried it transitively behind the `tls`
feature, so a downstream `cargo deny check --all-features` failed on a
version of fhir-mysql that had already shipped.

The lesson is in the flag: `cargo deny check` does not see an optional
feature's dependency tree, so the supply-chain gate only means what it says
when it runs `--all-features`.

## 0.3.0 — M7, trustworthy under load and under audit (2026-07-25)

Correctness and security work from the hardening review. Specification:
[`spec/`](spec/index.md) §§ 3–13; work breakdown:
[`tasks.md`](tasks.md) M7.

**Fixed — reads could return a resource that never existed.** A read spans
one base table and every child table; those were separate implicit
transactions, so a write landing mid-read reconstructed a mix of two
versions. Every multi-statement read (`get`, `get_all`, `status`,
`search_page`) now runs in one `REPEATABLE READ READ ONLY` transaction
(R4.5), and a search materializes its whole page in a single snapshot. The
new `concurrency` suite reproduces the old behaviour when isolation is
lowered, so the fix is pinned by a test that fails without it.

**Fixed — conditional create could create twice.** `If-None-Exist` searched
and then wrote in two steps, so two concurrent requests with identical
criteria both found nothing and both created — a patient entered twice.
Match and write now share one transaction guarded by a
`pg_advisory_xact_lock` on a hash of the criteria (A7.10). Conditional
delete likewise.

**Fixed — PHI crossed to PostgreSQL in the clear.** The connector was
hard-coded `NoTls`, so `sslmode=require` could not be honored. Connections
now go through rustls, honoring `sslmode` and `PGSSLROOTCERT` (O10.7), and
`serve` refuses a non-loopback bind over an unencrypted database link
without `--allow-insecure-db`. fhir-mysql's `require` validates the server
certificate where libpq's does not — a deliberate deviation, in the safe
direction.

**Fixed — response URLs came from the `Host` header.** `Bundle.entry.fullUrl`
and paging links were built from an attacker-controlled header, always as
`http://`. They now come from `--base-url`, or from the address actually
bound; forwarded headers are honored only under `--trust-proxy` with an
optional `--allowed-host` list (A7.7).

**Fixed — Bundle entries silently ignored their preconditions.**
`ifMatch` is now honored inside batch and transaction entries (412 on
mismatch, no longer flattened to 400); `ifNoneExist`, `ifModifiedSince`, and
`ifNoneMatch` fail the entry with 501 rather than being accepted and
discarded (A7.9).

**Fixed — `urn:uuid` rewriting corrupted non-reference data.** Transaction
processing replaced *any* string equal to a bundle `fullUrl`, including
narrative, `valueString`, and identifiers. Only `Reference.reference` values
are rewritten now.

**Fixed — internal error text reached clients.** `StoreError` now separates
`Unsupported` (client-safe: names the caller's own parameter or modifier)
from `Other` (internal: logged behind an incident id, never returned)
(A7.11). The honest "this search parameter is not supported" messages are
unchanged.

**Fixed — string search missed accented names, and scanned the table
looking for them.** FHIR® requires `string` search to ignore case *and*
accents; fhir-mysql shipped `ILIKE`, which ignores only case, so `family=muller`
did not find `Müller` and neither spelling of a decomposed `é` found the
other. Each string search target now has a folded companion column computed
in Rust at write time, so one implementation defines string equality rather
than one in Rust and one in SQL that must agree for every codepoint (P6.6).
No PostgreSQL extension is required.

The prefix predicate is emitted as a range (`col >= $1 AND col < $2`, upper
bound computed in Rust) rather than `LIKE $1 || '%'`. PostgreSQL extracts a
prefix from a *constant* pattern only, so the `LIKE` form uses the index in
a custom plan and silently falls back to a sequential scan in the generic
plan — fast in every hand-run `EXPLAIN` with a literal, and fast for the
first few executions in production, then not. A test pins this with
`plan_cache_mode = force_generic_plan`, the only setting under which the old
form visibly fails. `:exact` still compares the literal stored string.

`init --upgrade` adds the columns **and backfills them** before returning,
reporting how many values it folded; without that, an upgraded install would
have answered string searches from NULL columns and returned fewer results
with no error.

**Added — edge limits are configurable.** The request timeout, concurrency
cap, body size, `_count` ceiling, `_include` expansion cap, and pool size
were constants compiled into the binary; each is now a `serve` flag. The
right value depends on the deployment, and the previous answer to "this is
too low" was "rebuild it". `--pool-size` beats `FHIR_MYSQL_POOL_SIZE`, so a
typed flag is never silently overridden by an inherited environment.

**Fixed — a read whose audit record failed was served anyway.** Recording a
disclosure was best-effort: a failed insert was logged loudly and the
response returned regardless. That inverts the guarantee — a disclosure with
no record is indistinguishable, afterwards, from a disclosure that never
happened. All four read paths (read, vread, history, search) now propagate a
refusal as 503, so in every audit mode a read that cannot be recorded is not
served (PR12.6).

**Added — `--audit-mode sync|async|off`.** `sync` (the default) commits the
record before responding. `async` queues in memory and writes in batches
through one `INSERT ... SELECT unnest(...)`, trading a round trip per read
for a bounded loss window, which it announces at startup and drains on
graceful shutdown. `off` still requires `--allow-unaudited`. A saturated
queue refuses the read rather than dropping the record.

The spec draft had `async` as the default; it ships as `sync`, because a
fast default would make every deployment silently accept a loss window it
never chose.

Four counters per version on `/metrics` distinguish the two failures that
look alike from a distance: `refused` counts reads turned away to keep the
log honest — the system working as designed — while `lost` counts records
the writer could not commit after the data was served, which is an incident.
Queue depth is derived from the counters rather than tracked separately, so
it cannot report a value they contradict.

**Added — latency as a histogram.** `/metrics` served a cumulative
`fhir_mysql_request_latency_micros_total`, which with a request count gives only
the mean — and a mean cannot distinguish "every request took 40ms" from "99%
took 5ms and 1% took 4 seconds", which is the case an operator is paged for.
`fhir_mysql_request_latency_seconds` is now a Prometheus histogram over the
default 1ms–10s buckets, so `histogram_quantile` answers p99. The old
counter remains as the histogram's `_sum`.

**Added — CI/CD on both forges.** GitHub Actions and Codeberg Woodpecker run
the same gates: fmt, clippy, unit tests, the book, the live-PostgreSQL
suite, and the supply-chain checks. Tagging `v*` builds binaries, generates
a CycloneDX SBOM, and attaches both with SHA-256 checksums to a release —
the bill of materials now ships with the artifact rather than only with the
CI run that produced it. Publishing to crates.io stays manual and
confirmation-gated, because a published version is immutable. See
[`doc/ci.md`](doc/ci.md).

**Added — the declared MSRV is now verified.** `rust-version = "1.90"` was a
claim no job checked; both forges now build on exactly that toolchain, read
from the manifest rather than hard-coded.

**Added**

- Resource ids are validated against the FHIR `id` production on every path
  that accepts one (R4.6).
- Every response carries `Cache-Control: no-store`, `Pragma: no-cache`,
  `X-Content-Type-Options: nosniff`, and `Referrer-Policy: no-referrer`
  (A7.8); a client-supplied `X-Request-Id` is length- and
  character-validated before being echoed or logged.
- `_include`/`_revinclude` expansion is capped at 1,000 resources and adds
  an `OperationOutcome` warning to the bundle when the cap truncates, rather
  than silently returning less clinical context than asked for (P6.7).
- Pool size is configurable via `FHIR_MYSQL_POOL_SIZE` instead of compiled in.
- `crates/fhir-mysql-store/tests/concurrency.rs`: torn reads, racing conditional
  creates, and racing `If-Match` updates (T11.6).

**Added — the audit trail (M3.15–M3.17, §12)**

fhir-mysql still does not authenticate; the perimeter does. But "authentication
is elsewhere" cannot mean "the record of who did what is nowhere": the
perimeter knows the identity, and only the store knows which rows were
touched.

- Every history row carries an **audit envelope** — `actor`, `actor_source`,
  `client`, `request_id`, `reason` — written by the same statement, in the
  same transaction, as the change it describes. An audit record that can be
  lost independently of its change is not an audit record.
- Every **read** appends a disclosure record to `fhir_mysql_access_log`: read,
  vread, history, and search, the last carrying how many resources it
  returned. "Who looked at this patient" was previously unanswerable.
- History is **tamper-evident**: a per-resource SHA-256 chain
  (`prev_hash`/`row_hash`) computed in SQL, so it covers the database's own
  `now()` and cannot race the read of the previous hash. `fhir-mysql
  verify-audit` recomputes every chain and exits nonzero on a break.
- History is **append-only in the database**: a `BEFORE UPDATE OR DELETE`
  trigger on every history table refuses the operation. Rewriting history
  now requires deliberately disabling a trigger.
- The server accepts a principal from a configured header, honored **only**
  behind `--trust-proxy` — without it the header is ignored, not trusted,
  because otherwise any client could name itself anyone.
  `--require-principal` makes an unattributable request a 401. CLI writes
  are attributed to the operator at the keyboard.

**Fixed — `--validate` now covers R3 and R4, not just R5**

The restriction was never fhir-mysql's: the published `fhir` 1.2.0 could not
compile its own `r3`/`r4` features, because the `Validate` derive expanded to
`crate::r5::` paths that are feature-gated away. An R4-only build failed with
4,422 errors. The repository never showed it — Cargo resolves the derive macro
through its `path` dependency, where the fix already existed — so only
crates.io consumers were affected.

Fixed upstream by publishing `fhir-derive-macros` 1.0.1 and `fhir` 1.2.1;
fhir-mysql's floor moves to `fhir = "1.2.1"` with `features = ["r3", "r4"]`, and
`validate_typed` gains the two missing arms.

`validate_tests` in the CLI pins the behaviour for all three versions,
including one assertion about what `--validate` does *not* do: serde ignores
unknown fields, so the typed model tolerates an element the release never
defined. The shredder is what rejects those (plan D12); `--validate` adds type
and cardinality rigour on top.

**Added — resource limits, capability honesty, supply chain**

- A per-request timeout and an edge concurrency limit, shedding as 503 +
  `Retry-After` or 504, both as OperationOutcomes (O10.8).
- The CapabilityStatement now declares `versioning`, `readHistory`,
  `updateCreate`, `conditionalCreate`/`Update`/`Delete`, `referencePolicy`,
  `searchInclude`/`RevInclude`, system-level `transaction`/`batch`, and a
  `security` block stating plainly that fhir-mysql verifies no identity (A7.12).
- CI gains a supply-chain job (cargo-deny, CycloneDX SBOM) and `deny.toml`
  (O10.10).
- `book/src/trust-boundary.md`: what fhir-mysql guarantees, what the deployment
  must provide, and what neither provides yet — with a worked `serve`
  invocation and the `REVOKE` grants that complement the trigger.

**Fixed — `postgres: db error`.** `tokio_postgres::Error`'s `Display` is the
bare string `"db error"`; every SQLSTATE, message, and hint hangs off
`source()`. Operator logs were therefore recording nothing useful about any
database failure. `StoreError::Pg` now renders the full detail — to logs
only, never to a response body.

**Specification** gained the audit envelope and tamper-evident history
(M3.15–M3.18), snapshot reads (R4.5), id validation (R4.6),
accent-insensitive search (P6.6), bounded search cost (P6.7), A7.7–A7.12,
O10.7–O10.10, § 12 *Trust, principal, and audit*, and § 13 *Compliance
mapping* (HIPAA, GDPR, ONC/HTI, IEC 62304).

## Unreleased — 2.0.0-dev (ground-up rewrite, 2026-07)

A complete rewrite: FHIR resources now live in **fully normalized
relational tables** (typed columns, child tables per repeating element,
`ords smallint[]` ordinal paths) instead of jsonb bodies. The prior
fhirbase-style implementation remains in git history.

- Spec-driven generator: DDL + relational map assets for R5 5.0.0,
  R4 4.0.1, R3 3.0.2 (158/146/117 resource types; 7,355 tables for R5).
- Lossless round trip proven against all 7,399 official spec examples —
  in memory and through live PostgreSQL 18 — plus 10,000 map-driven
  property-test cases (which caught, and fixed, an ordinal ambiguity
  between QuestionnaireResponse's two recursive item paths).
- Extensions, primitive extensions, and element ids stored relationally
  as typed leaf rows; the Reference↔Identifier type cycle spills to a
  relational leaf table; no live data in JSONB.
- Store: transactional create/update/delete with append-only history,
  vread, 404-vs-410 status, optimistic concurrency (If-Match), and
  multi-op all-or-nothing transactions.
- Search: 94.8% of R5 SearchParameters compiled to indexed SQL
  (token/string/date/number/quantity/reference/uri), `_sort`, `_total`,
  paging; strict unsupported-parameter reporting.
- FHIR REST server (`fhir-mysql serve`): every installed version mounted,
  CRUD + history + vread, search with next-link paging, batch and
  transaction Bundles with urn:uuid resolution, generated
  CapabilityStatements, OperationOutcome errors, request ids, Prometheus
  metrics, graceful shutdown, pool wait-timeout → 503 + Retry-After,
  server-side statement timeouts.
- CLI: gen, init (staged-schema, idempotent, checksum-guarded), load
  (optional `--validate` through the typed fhir crate model, R5),
  get/delete/export/search/transform/drop/serve.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
