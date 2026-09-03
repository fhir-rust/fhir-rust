# Releasing

Governing requirements: `O10.10`, `O10.11`, `W16.11`–`W16.15`.

## Versioning

Each port versions **independently** (`W16.11`) — a fix to one port must not
require a bump in the other five.

All 34 publishable crates across the four families have already been published
to crates.io, more than once: the first full publication landed 2026-08-22
(21 crates going out for the first time, seven more re-published after a
`serde_json/float_roundtrip` bump), then the whole set went out again twice
more on 2026-08-26 — once for the routine bump that followed hosted CI turning
green, once for the trademark-disclaimer text required in every `description`.
`scripts/check-published-match.sh` reported `34 matched, 0 mismatched` after
each pass. See [`spec/publishing.md`](../spec/publishing.md) for the full
history, and "Before any release" below for the laptop step that does the
actual upload.

Versions keep moving independently per port (`W16.11`), so "current" is a
moving target — read it from source rather than this file:
`grep '^version' fhir-<engine>/Cargo.toml` (as of 2026-09-03:
`fhir-postgresql` `0.6.2`; `fhir-sqlite`/`fhir-mysql`/`fhir-mariadb`/`fhir-mssql`
`0.6.1`; `fhir-oracle` `0.6.0`; `fhir-store` `0.3.1`, `fhir-loco`
`0.3.3`). The question for a release is no
longer "has this ever been published" — it has — but whether the tree's
version has moved past what crates.io already holds for that crate, which is
exactly what `check-published-match.sh` (below) answers.

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
  docs.rs — read by exactly the person who has not looked at the code yet.
  `fhir-mssql` and `fhir-oracle`, the two ports that were Scaffold at the time,
  were made to say so in the description itself.
- That description now needs updating again: `fhir-mssql` and `fhir-oracle` are
  no longer Scaffold. Both reached **Store** (`fhir-mssql`: **F-65**;
  `fhir-oracle`: **F-68**) per the [conformance
  matrix](../spec/databases/conformance-matrix.md), and no port sits below
  Store today. A crate's `description` must track its level as it moves —
  publishing a Scaffold-era description after the port has earned Store is the
  same understatement as the reverse claim, just less dangerous.
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
| crates.io | a documented laptop step (`spec/publishing.md`) — decided 2026-08-26; no publish workflow | — |

The unit-test job passes with no database and no FHIR® packages, because those
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

**An agent working in this repository may work through §§1–4 below, decide
the release meets them, and carry out §5 itself** — the maintainer no longer
has to tick every box personally before `cargo publish` runs (delegated
2026-09-02; see [`GOVERNANCE.md`](../GOVERNANCE.md#what-is-decided-where) and
[`AI_STATEMENT.md`](../AI_STATEMENT.md) §5). Delegating *who* runs the
checklist does not loosen it: §§1–4 are the same gates that bound the
maintainer's own judgment, unchanged by who is checking them. The judgment
still has to be recorded, exactly as any other decision in this project is
(`GOVERNANCE.md`'s "Where decisions are recorded") — a `CHANGELOG.md` entry
and a commit or release note citing which of §§1–4 passed, not a conclusion
that exists only inside a session.

### §1. Source and artifact integrity

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

### §2. Build, lint, and tests

1. `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
2. Live suite green **against the port's own engine**

### §3. Supply chain

3. `cargo deny check`, `cargo audit`

### §4. Claims match reality

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

### §5. Publish

Once §§1–4 pass: `cargo publish` per crate, in the dependency order
[`spec/publishing.md`](../spec/publishing.md#order-of-publication) fixes
(`fhir-store` before any port; a port's `map` before its `gen` before its
`store`; `fhir-loco` last) — then tag and, per
[`MAINTAINERS.md`](../MAINTAINERS.md), sign both, from the machine holding
`~/.cargo/credentials.toml`. This is still the laptop step
[`spec/publishing.md`](../spec/publishing.md) documents, not a CI job; §5
changes who may run it, not where it runs.

Step 6 no longer blocks every port. **F-49** — no workflow in this repository
ran, because they all sat under `<family>/.github/workflows/` and GitHub reads
only the root — was fixed 2026-08-06 by the root-level CI consolidation, and
`C0.9`'s "justified by tests that run in that port's CI" is now satisfiable:
every port's live gate has since run hosted against its own engine —
PostgreSQL re-run 2026-08-03, MySQL and MariaDB measured 2026-08-03 (their
full-schema install gap, **F-90**, closed 2026-08-12), `fhir-mssql` measured
2026-08-10, and `fhir-oracle`'s live job, restored by **F-06**'s fix, ran
green on its first hosted execution 2026-08-12. See the [conformance
matrix](../spec/databases/conformance-matrix.md) for the current state of
each.

No port has an open High finding of its own, and no port remains **Scaffold**:
`fhir-mssql` and `fhir-oracle`, the last two, reached **Store** (**F-65**,
**F-68**). `fhir-oracle` also cleared **F-08** (its DDL is Oracle and installs
on 26ai); `fhir-postgresql` cleared **F-07** (chain portability). With no CI
or conformance blocker left, the pacing item for a release today is the
mechanical "Before any release" list above, run per port before each bump —
not a finding to fix first. What separates **Store** from **Reference** is
depth, not a blocker: `fhir-sqlite`, `fhir-mysql`, and `fhir-mariadb` have
concurrency, redaction, and upgrade suites but no `audit.rs` of PostgreSQL's
depth, and `T11.15` determinism is unmeasured there — that bears on a
Reference-level claim in step 7, not on publishing at the level each port has
actually earned.

## Pushing

Ask first — but not for the reason this section used to give. It said "do not:
all six ports still carry the ancestor project's `origin` (**F-11**)". That is
resolved: none of the six ports, nor `fhir/`, has a `.git` of its own any more.
They are directories in one repository with one remote,
`git@github.com:fhir-rust/fhir-rust.git`.

One thing remains, and it is not the original problem: that URL does not
resolve anonymously, which a private repository also does — unverified rather
than known-absent ([`spec/publishing.md`](../spec/publishing.md) **P-5**).

The old `fhir-store/`-nested-repository warning is obsolete, and for a more
confusing reason than the six-remotes one above. **F-37** was real, but it was
about a *different* directory that used to be called `fhir-store/` — the HTTP
surface, which had its own untracked `.git` with no remote. It was fixed
2026-08-02 by removing the nested repo and committing the source directly;
that directory was then renamed `fhir-loco` the same day (**F-45**). The name
`fhir-store` was reused a few days later for an unrelated extraction — the
small shared audit/value-type library, not a server — and **that** crate has
never had a nested `.git`: `git ls-files -s fhir-store/ | grep 160000` returns
nothing (no gitlink-mode entries), verified 2026-08-04 and re-verified since.
Do not carry F-37's caution forward onto the crate that now holds this name.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
