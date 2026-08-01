# Releasing

Governing requirements: `O10.10`, `O10.11`, `W16.11`–`W16.15`.

## Versioning

Each port versions **independently** (`W16.11`). All six currently sit at
`0.4.0`, which is a fact about a shared ancestor rather than a promise — a fix
to one port must not require a bump in the other five.

The workspace `[workspace.package] version` governs all three crates in a port,
and `[workspace.dependencies]` pins the sibling path dependencies to the same
number.

## The gate that matters most

**A published version must match the source that claims it** (`O10.11`).

A crates.io version is immutable, so a tree carrying an already-published
version number must be byte-identical to what was published, and CI must fail
otherwise.

Without the check the divergence is invisible. Every local build resolves the
path dependency and never fetches the registry copy, so the tree stays green
while the artifact someone downloads is different code. It surfaces only when a
third party packages a dependent — as an error about code they did not write.

For a component handling clinical data, "the released artifact is the reviewed
source" is the claim the whole audit trail rests on. `O10.10`'s SBOM describes
the artifact, and it is worth nothing if the artifact is not the source.

## Supply-chain evidence

`O10.10`, per release:

- `cargo deny` — advisories, licenses, bans
- `cargo audit`
- a CycloneDX SBOM per release artifact
- checksums for every published binary

This is the IEC 62304 / FDA cybersecurity expectation for a component handling
clinical data, and it is cheap to keep green from the start and expensive to
retrofit.

## Do not publish above the level

`W16.14`. A port publishes only at the conformance level it has earned
(`C0.8`), and its crate metadata must match:

- Every store crate described itself as **"PostgreSQL storage layer"**
  (**F-02**, fixed). `description` is published to crates.io and rendered on
  docs.rs — read by exactly the person who has not looked at the code yet. The
  two Scaffold ports now say so in the description itself.
- `fhir-mssql` and `fhir-oracle` are **Scaffold**. Publishing either under a
  name that implies a working FHIR store is a claim about clinical software made
  to people who cannot check it.
- All six READMEs claimed the reference port's results (**F-01**, fixed). The
  `book/` directories still do — fix those before a release, not after.

## CI gates

Both forges run the same gates deliberately: a change that passes on one and
fails on the other is a bug in the pipelines, not a property of the forge.

| Gate | GitHub Actions | Woodpecker (Codeberg) |
| --- | --- | --- |
| fmt, clippy, unit tests, book | `ci.yml` (`test`) | `.woodpecker/ci.yaml` |
| MSRV | `ci.yml` (`msrv`) | `.woodpecker/ci.yaml` |
| Live database suite | `ci.yml` (`database`) | `.woodpecker/database.yaml` |
| Advisories, licenses, SBOM | `ci.yml` (`supply-chain`) | `.woodpecker/supply-chain.yaml` |
| TLS-only database | `ci.yml` (`tls-database`) | — |
| Tag → artifacts | `release.yml` | `.woodpecker/release.yaml` |
| crates.io | `publish.yml` (manual) | — |

The unit-test job passes with no database and no FHIR packages, because those
tests self-skip. The **live pipeline is the required gate**, not an optional
extra — and it must provision the port's own engine (`O10.12`). `fhir-oracle`
has no such pipeline, deliberately: it has nothing to point one at, and a gate
against a substitute is worse than none (**F-06**).

The TLS gate is GitHub-only: Woodpecker starts services before workspace steps
run, so a certificate generated in a step does not exist when the database
container boots, and the workarounds (committing a test key, docker-in-docker)
are each worse than the gap. Codeberg pushes are covered by every gate except
that one.

## MSRV

`rust-version` is a promise to downstream users. Both forges build on exactly
that toolchain, because an unverified MSRV is a guess. Raising it is a
minor-version event, not a patch.

## Before any release

1. `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
2. Live suite green **against the port's own engine**
3. `cargo deny check`, `cargo audit`
4. `CHANGELOG.md` describes changes to **this port** (`W16.12`) — an entry
   inherited from another port's history describes work that was not done here
5. [`spec/conformance-matrix.md`](../spec/conformance-matrix.md) reflects
   reality
6. [`spec/audit.md`](../spec/audit.md) — no open **High** finding against this
   port
7. README claims match the level (`C0.11`)

Step 6 now blocks one port on a High finding: `fhir-oracle` has **F-08** (its
DDL is not Oracle). `fhir-postgresql` cleared **F-07** (chain portability).
`fhir-sqlite`, `fhir-mysql`, and `fhir-mariadb` are clear of High findings and
blocked instead by step 5 — their concurrency, redaction, and audit rows are
`?`, not `•`.

## Pushing

**Do not.** All six ports still carry the ancestor project's `origin`
(**F-11**), so pushing any branch would send that port to the wrong repository.
The related question — whether six products should keep a shared ancestor
history, be squashed, or be re-rooted — is an owner decision recorded in every
port's `tasks.md`.
