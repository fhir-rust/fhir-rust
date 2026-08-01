# Continuous integration and delivery

fhir-postgresql is mirrored on GitHub and Codeberg, and both forges run the same
gates. The two configurations are kept deliberately parallel: a change that
passes on one and fails on the other is a bug in the pipelines, not a
property of the forge.

| Gate | GitHub Actions | Woodpecker (Codeberg) |
| --- | --- | --- |
| fmt, clippy, unit tests, book | `.github/workflows/ci.yml` (`test`) | `.woodpecker/ci.yaml` |
| MSRV | `ci.yml` (`msrv`) | `.woodpecker/ci.yaml` (`msrv` step) |
| Live PostgreSQL suite | `ci.yml` (`database`) | `.woodpecker/database.yaml` |
| Advisories, licenses, SBOM | `ci.yml` (`supply-chain`) | `.woodpecker/supply-chain.yaml` |
| TLS-only PostgreSQL | `ci.yml` (`tls-database`) | — (see below) |
| Tag → artifacts | `.github/workflows/release.yml` | `.woodpecker/release.yaml` |
| crates.io | `.github/workflows/publish.yml` (manual) | — |

## What actually gates a merge

The unit-test job passes with no database and no FHIR specification packages
present, because the corpus- and spec-driven tests skip themselves when their
inputs are absent. That is convenient locally and misleading in CI, so the
live-database pipeline is a separate required gate rather than an optional
extra: most of fhir-postgresql's guarantees are database guarantees — snapshot
isolation, advisory locks, the append-only trigger, the hash chain, and
index-using search plans — and none of them are exercised without a server.

The live pipeline downloads the FHIR definitions and example corpora from
hl7.org on each run. That is a network dependency on a third party, and it
will occasionally be the reason a build is red.

## The TLS gate, and why it is GitHub-only

fhir-postgresql refuses a networked bind over a plaintext database link, and its
`require` validates the server certificate where libpq's does not. Neither
claim is worth much against a server that happily accepts plaintext, so one
job runs the live suite against a PostgreSQL configured `hostssl`-only —
and first proves the server really does refuse a plaintext connection, since
a gate that silently permits downgrade tests nothing.

This has no Woodpecker counterpart yet. Woodpecker starts services before
workspace steps run, so a certificate generated in a step does not exist when
the database container boots, and the obvious workarounds (committing a test
key, or docker-in-docker) are each worse than the gap. Codeberg pushes are
therefore covered by every gate *except* this one.

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
