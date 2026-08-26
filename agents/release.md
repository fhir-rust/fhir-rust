# Releasing

Governing requirements: `O10.10`, `O10.11`, `W16.11`–`W16.15`.

## Versioning

Each port versions **independently** (`W16.11`) — a fix to one port must not
require a bump in the other five.

All six currently sit at **`0.4.0`** — manifest, lock file, and changelog in
agreement since 2026-08-01, when the manifests were found a release behind at
`0.1.0` and corrected (**F-34**).

None of those four releases reached crates.io: the eighteen port crate names are
unregistered, so the **first publication will be `0.4.0`** with nothing beneath
it on the registry. Say so in the changelog when it happens, rather than leaving
a reader to wonder where `0.1.0`–`0.3.1` went.

The workspace `[workspace.package] version` governs all three crates in a port,
and `[workspace.dependencies]` pins the sibling path dependencies to the same
number.

## The gate that matters most

**A published version must match the source that claims it** (`O10.11`).

A crates.io version is immutable, so a tree carrying an already-published
version number must be byte-identical to what was published, and CI must fail
otherwise.

```sh
scripts/check-published-match.sh          # every crate, against the registry
scripts/check-published-match.sh --diff   # show what moved
```

This gate exists because the tree had already diverged and nobody could see it:
`fhir-derive-macros` sat on `1.1.0`, the published number, with 206 lines the
published crate does not contain (**F-35**). Run it before any release, and read
its output rather than its exit code — it passes *vacuously* when no crate sits
on a published version, and says so.

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
  name that implies a working FHIR® store is a claim about clinical software made
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

0. `scripts/check-published-match.sh` — nothing claims a published number it no
   longer matches (`O10.11`)
0b. `cargo run -p fhir-<engine>-gen --bin regen-assets -- --check` — the
   committed maps are what this generator produces (`G2.2`)
0c. `cargo package --list` — **without** `--allow-dirty`. That flag is not a
   convenience: it is what let the map assets sit untracked in all six ports
   while every dry run looked fine (**P-10**). If packaging needs it, something
   the package depends on is not in the repository, and a clone will not have
   it. Confirm in a clean checkout — `git worktree add --detach <dir> HEAD` —
   not in the tree that just generated the files.
0d. `scripts/check-doc-examples.sh` — every ```` ```rust ```` block in `doc/` and
   `README.md` compiles. It found six real defects on its first run, including a
   block that could not parse (**F-60**). A block that cannot compile must be
   marked ```` ```rust,ignore ```` **with a reason**, not silently.
1. `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
2. Live suite green **against the port's own engine**
3. `cargo deny check`, `cargo audit`
4. `CHANGELOG.md` describes changes to **this port** (`W16.12`) — an entry
   inherited from another port's history describes work that was not done here.
   All five non-PostgreSQL changelogs are still `fhir-postgresql`'s and now
   carry a banner saying so (**F-62**); two of them announced a TLS security fix
   for a connector they do not have. Do not add a release entry above that
   banner without checking it is true of *this* crate
5. [`spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) reflects
   reality
6. [`spec/audit.md`](../spec/databases/audit.md) — no open **High** finding against this
   port
7. README claims match the level (`C0.11`)
8. [`spec/publishing.md`](../spec/publishing.md) — no open **P-** blocker
   against this crate. That register covers all **four** families and is the one
   place the crates.io view is assembled.

Step 6 blocks **every** port, on one finding that is not any port's fault:
**F-49** — no workflow in this repository runs, because they all sit under
`<family>/.github/workflows/` and GitHub reads only the root. Until that is
resolved, `C0.9`'s "justified by tests that run in that port's CI" cannot be
satisfied by anyone.

No port has an open High finding of its own. `fhir-oracle` cleared **F-08** (its
DDL is now Oracle and installs on 26ai); `fhir-postgresql` cleared **F-07**
(chain portability). `fhir-sqlite`, `fhir-mysql` and `fhir-mariadb` are blocked
instead by step 5 — they now have concurrency, redaction and upgrade suites, but
no audit suite of PostgreSQL's depth, and `T11.15` determinism is unmeasured.

## Pushing

Ask first — but not for the reason this section used to give. It said "do not:
all six ports still carry the ancestor project's `origin` (**F-11**)". That is
resolved: none of the six ports, nor `fhir/`, has a `.git` of its own any more.
They are directories in one repository with one remote,
`git@github.com:fhir-rust/fhir-rust.git`.

Two things remain, and neither is the original problem:

- that URL does not resolve anonymously, which a private repository also does —
  unverified rather than known-absent ([`spec/publishing.md`](../spec/publishing.md) **P-5**);
- `fhir-store/` is a **nested repository with no remote**, untracked by the
  parent (**F-37**), so `git add` on it records a gitlink rather than the files
  and a clone would get an empty directory with no error. Settle that before
  pushing anything meant to include it.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
