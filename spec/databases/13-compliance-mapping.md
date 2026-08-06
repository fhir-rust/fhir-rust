# 13. Compliance mapping

These are components, not certified systems: they cannot make a deployment
compliant, but they must not be the reason a deployment cannot be. This table
maps the obligations that shape the requirements above, so a reviewer can trace
a regulation to a numbered requirement to a test.

| Obligation | Requirements | Evidence |
| --- | --- | --- |
| HIPAA §164.312(b) audit controls | `M3.15`, `PR12.3a`, `PR12.4`, `PR12.5` | `T11.8` |
| HIPAA §164.312(c) integrity | `M3.16`, `M3.16a`, `M3.16b`, `M3.16c`, `M3.17`, `R4.4`, `R4.5`, `R4.7`, `H5.4` | `T11.6`, `T11.8` |
| HIPAA §164.312(e) transmission security | `O10.5`, `O10.7`, ~~`A7.8`~~ | live TLS smoke test |
| HIPAA §164.502 minimum necessary | `PR12.1`, `PR12.8` (perimeter) | boundary table |
| GDPR Art. 17 erasure | `M3.18` | purge test |
| GDPR Art. 30 records of processing | `PR12.5`, `PR12.7` | `T11.8` |
| GDPR Art. 32 security of processing | `O10.7`, `O10.8`, `O10.10` | CI gates |
| ONC/HTI FHIR conformance | ~~`A7.12`~~, `T11.4`, §9 validation | Inferno run |
| ONC/HTI Bulk Data | ~~`M8`~~ (`$export`) | Inferno run |
| IEC 62304 §5–8 lifecycle | spec ↔ tasks ↔ test traceability | this document, [conformance matrix](conformance-matrix.md) |
| IEC 62304 / FDA cybersecurity | `O10.10`, `O10.11` | release artifacts |
| Software identity / SBOM accuracy | `O10.11` | published-version check |

~~Struck~~ identifiers belong to retired sections (`C0.16`). Their rows are
**unsatisfied as written**, not satisfied by something else, and the strike is
there so a reviewer does not read a retired citation as a met obligation.

**Corrected 2026-08-03.** This paragraph said those three rows "depend on a
service layer this monorepo does not contain". It does contain one:
[`fhir-loco`](../../fhir-loco/), a Loco/Axum FHIR REST API. The evidence cells
therefore describe runs that *can* happen — against `fhir-loco`, not against a
port — and the honest status per row is:

| Row | Against `fhir-loco` |
| --- | --- |
| HIPAA §164.312(e) transmission security (~~`A7.8`~~) | **not met.** Plain HTTP; TLS is expected from a proxy the deployment supplies, and no requirement currently states that (**F-58**, **F-59**) |
| ONC/HTI FHIR conformance (~~`A7.12`~~) | **partly.** A CapabilityStatement is served and was corrected under **F-57**; no Inferno run has been performed |
| ONC/HTI Bulk Data (~~`M8`~~) | **not met.** No `$export` |

The strikes stay, because a retired id is retired however live the behaviour it
describes. Since 2026-08-03 each is **restated** in
[`fhir-loco/spec/`](../../fhir-loco/spec/index.md) under an `SV` id — `A7.8` as
`SV3.11`, `A7.12` as `SV2.8`–`SV2.11`, `M8` as `SV2.15` — so these three rows
now map to requirements that can be cited and audited rather than to struck
identifiers. `C0.16` carries the full correspondence.

What each row still needs to be *satisfied* is unchanged and stated above.

## Deliberate gaps

Stated rather than papered over:

- **Authorization** — scopes, compartments, consent, and `meta.security` label
  enforcement — lives at the perimeter (`PR12.8`). Nothing here evaluates
  whether a principal *may* see what it asked for; the store records that it
  did.
- **Terminology validation** is out of scope until a terminology service is
  integrated (`V9.4`). A `required` binding is checked against a literal code
  list, not an expanded value set.
- **No port is certified.** The compliance mapping shows which requirements
  exist to support an obligation. Whether a *deployment* meets that obligation
  depends on the perimeter, the operating procedures, and the evidence retained,
  none of which a library provides.

## How to use this table in an audit

1. Pick the obligation row.
2. Follow each requirement id into the core spec — it says what MUST hold and
   why.
3. Follow the evidence column into the test suite.
4. Check the [conformance matrix](conformance-matrix.md) for the port you are
   actually deploying. A requirement satisfied by the PostgreSQL reference is
   not thereby satisfied by another port — `fhir-oracle` is Store level now
   (**F-68**), and its matrix row still differs from the reference on `R4.5`,
   `O10.7`, and every concurrency cell — and the matrix is the only document
   in this tree that distinguishes them.

Step 4 is the one that gets skipped, and it is the one that matters: five of the
six ports currently sit below Reference level (`C0.8`).

---

Part of the [fhir-databases specification](index.md).
