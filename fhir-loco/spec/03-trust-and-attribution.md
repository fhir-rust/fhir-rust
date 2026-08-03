# 3. Trust and attribution

The database ports deliberately do not authenticate. This crate is the
*elsewhere* that boundary points at: it is the only component here that
terminates a request, so it is the only one that can turn a credential into an
[`Audit`].

## Establishing the principal

- **SV3.1** Every request MUST carry a verified credential. There MUST NOT be an
  unauthenticated mode, and there MUST NOT be a mode that checks a credential
  *if present* and proceeds when absent — that accepts an unauthenticated
  request whenever the caller omits the header, which is the same as not
  checking.

- **SV3.2** The credential is a **PASETO v4.public** token in
  `Authorization: Bearer`. A missing, malformed, tampered or expired token is
  `401`.

- **SV3.3** PASETO rather than JWT, and this MUST NOT be relaxed. A JWT names
  its own algorithm in a header the attacker also controls, which is the root of
  the `alg: none` and RS256→HS256 confusion families. A `v4.public` token is
  Ed25519 and cannot claim to be anything else; there is no negotiation to get
  wrong.

- **SV3.4** `v4.public`, not `v4.local`. `v4.local` is symmetric: verifying and
  minting use the same key, so any instance able to check a credential is able
  to issue one, and a single read of the configuration impersonates any
  principal. This service holds only the public half and **cannot mint a token
  at all**.

- **SV3.5** The verifying key is required at boot. Absent, empty, non-hex or
  wrong-length MUST all stop the process. There MUST be no configuration under
  which this service starts without a way to verify who is calling it.

  An earlier revision accepted a principal from a trusted header when no key was
  configured. Two modes meant the safe one had to be chosen and the unsafe one
  was the default: a deployment that forgot the variable ran unauthenticated
  while looking configured, and the only signal was a log line at boot.

- **SV3.6** A trusted header MUST NOT be reintroduced as an alternative. Such a
  header is only as trustworthy as the guarantee that nothing else can reach the
  port — a guarantee that lives in network configuration this service cannot
  see, cannot verify, and cannot fail loudly about. A signature is a property of
  the request; a header is a property of an assumption about the topology.

## What the store is told

- **SV3.7** Every mutating call MUST pass an [`Audit`] naming the principal, and
  MUST record **how** it was established (`actor_source`), so a reader can weigh
  what the actor is worth. Restates `PR12.1`–`PR12.4`, which are *not*
  `[service]`-marked: the store implements attribution, and this crate's
  obligation is to supply it truthfully.

- **SV3.8** The `sub` claim is the actor. A token without one, or with an empty
  one, MUST be refused rather than attributed to a placeholder. An audit trail
  naming `unknown` for a request that carried a valid signature is worse than a
  refusal, because it looks like a record.

- **SV3.9** `X-Request-Id`, `X-Forwarded-For` and a caller-supplied reason MAY
  be recorded alongside, and MUST NOT be treated as identity. They are
  unauthenticated strings; their value is correlation, not attribution.

## What this crate does not do

- **SV3.10** **No authorization.** This crate verifies *who* is calling and
  never decides *whether they may*. It issues `401`, never `403`.

  This MUST be stated wherever the trust boundary is documented, because the gap
  is invisible from outside: a deployment that authenticates every request and
  authorizes none looks secure and lets any valid token read any patient.

- **SV3.11** **No transport security of its own.** This service speaks plain
  HTTP and expects a TLS-terminating proxy. `O10.7` governs the *database* link,
  not this listener, and **no requirement currently states an obligation for the
  listener's own TLS** — a gap in §10 rather than in this crate, recorded as
  **F-58**.

  Until one exists, a deployment that exposes this port directly is carrying PHI
  in the clear and nothing in this repository will tell it so. `SV4.4`'s
  loopback default is the mitigation, and it is a weak one.

---

Part of the [fhir-loco specification](index.md).

[`Audit`]: ../../fhir-store/
