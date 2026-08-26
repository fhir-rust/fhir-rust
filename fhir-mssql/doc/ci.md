# Continuous integration and delivery

fhir-mssql is mirrored on GitHub and Codeberg, and both forges run the same
gates. The two configurations are kept deliberately parallel: a change that
passes on one and fails on the other is a bug in the pipelines, not a
property of the forge.

| Gate | GitHub Actions | Woodpecker (Codeberg) |
| --- | --- | --- |
| fmt, clippy, unit tests, book | `.github/workflows/ci.yml` (`test`) | `.woodpecker/ci.yaml` |
| MSRV | `ci.yml` (`msrv`) | `.woodpecker/ci.yaml` (`msrv` step) |
| Live MySQL suite | `ci.yml` (`database`) | `.woodpecker/database.yaml` |
| Advisories, licenses, SBOM | `ci.yml` (`supply-chain`) | `.woodpecker/supply-chain.yaml` |
| TLS-only server | removed — see below |  |
| Tag → artifacts | `.github/workflows/release.yml` | `.woodpecker/release.yaml` |
| crates.io | `.github/workflows/publish.yml` (manual) | — |

## What actually gates a merge

The unit-test job passes with no database and no FHIR® specification packages
present, because the corpus- and spec-driven tests skip themselves when their
inputs are absent. That is convenient locally and misleading in CI, so the
live-database pipeline is a separate required gate rather than an optional
extra: most of fhir-mssql's guarantees are database guarantees — snapshot
isolation, advisory locks, the append-only trigger, the hash chain, and
index-using search plans — and none of them are exercised without a server.

The live pipeline downloads the FHIR definitions and example corpora from
hl7.org on each run. That is a network dependency on a third party, and it
will occasionally be the reason a build is red.

## The TLS gate, and why it is temporarily absent

The PostgreSQL original runs a second live job against a server configured to
refuse plaintext, proving the store's `SslPolicy` really does refuse an
unencrypted link. MySQL is a network server, so that gate **does** apply here
and should come back — but it cannot be written honestly until the store speaks
MySQL (T64), because `SslPolicy` and the plaintext-refusal guard are store-layer
behaviour. Rather than leave a PostgreSQL job pretending to cover this port, it
is removed and tracked as an open task.
## MSRV

`rust-version` in `Cargo.toml` is a promise to downstream users. Both forges
read that value and build on exactly that toolchain, because an unverified
MSRV breaks silently the first time anyone uses a newer language feature.

The job reads the version from the manifest rather than hard-coding it, so
raising the MSRV is a one-line change in one place.

## Releasing

Pushing a `v*` tag builds binaries, generates a CycloneDX SBOM, and attaches
both — with SHA-256 checksums — to a release on that forge. GitHub builds
five targets (Linux gnu/musl on x86-64, Linux on arm64, macOS on both
architectures); Woodpecker builds the statically linked musl target, which is
the one that runs anywhere.

The SBOM ships with the release rather than only with the CI run that
produced it: a component handling clinical data is part of someone's IEC
62304 file (spec O10.10), and a CI log ages out while a release artifact does
not.

**Tagging does not publish to crates.io.** A crates.io version is immutable —
it can be yanked but never replaced — so publishing is a manual workflow that
requires typing `publish` into a confirmation field, and it re-runs fmt,
clippy, and the full test suite before uploading anything. A tag is easy to
create by accident; an immutable published version is impossible to withdraw.

## Secrets

| Secret | Used by | Purpose |
| --- | --- | --- |
| `CARGO_REGISTRY_TOKEN` | GitHub `publish.yml` (environment `crates-io`) | crates.io upload |
| `codeberg_token` | Woodpecker `release.yaml` | attach artifacts to a Codeberg release |

Neither pipeline needs access to any database containing real data, and
neither should ever be given one.

To run the live suite locally against the same pinned image CI uses, see
[containers.md](containers.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
