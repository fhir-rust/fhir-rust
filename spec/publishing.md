# Publishing to crates.io — readiness

**Non-normative.** This records what stands between the repository's current
state and the stated goal of publishing every crate to crates.io. It is a status
document like the [conformance matrix](databases/conformance-matrix.md), not a
requirement: the requirements it measures against are `O10.10`, `O10.11`,
`W16.11`–`W16.15`, and `C0.11`, and the working procedure is
[`AGENTS/release.md`](../AGENTS/release.md).

It is **cross-family** — the one document in `spec/` that is, because publishing
is the one activity that treats the whole repository as a single release
surface. Family-specific findings still belong in that family's own register.

**Assessed:** 2026-08-01, against the tree as it then stood; **P-1 restated
2026-08-06** after both former scaffolds reached Store level (**F-65**,
**F-68** — the four days between assessments invalidated P-1's whole
premise, **F-76**). **Method:**
`cargo package --list` on representative crates; `cargo metadata` on every
workspace; the crates.io API for all 32 package names; anonymous HTTP for every
declared `repository` URL.

## The 33 crates

Thirty-three `[package]` manifests, in four families. Fuzz crates are
`publish = false` by design and are excluded.

The count changed on 2026-08-02: `fhir-store` was split in two. The name now
belongs to the **engine-agnostic persistence core** (`chain`, `Audit`,
`AccessRecord`, the result types — **F-45**), and the HTTP server it used to
name is **`fhir-loco`**. Both names were checked and are unregistered.

| Family | Crates | Names on crates.io |
| --- | --- | --- |
| Model (`fhir/`) | 13 | **all 13 already registered by this author** |
| Databases (`fhir-<engine>/`) | 18 | all 18 available |
| Persistence core (`fhir-store/`) | 1 | available |
| HTTP surface (`fhir-loco/`) | 1 | available |

The model family is therefore a **version bump**, not a first registration:
`fhir` 2.1.0, `fhir-core` 2.2.0, `fhir-derive-macros` 1.1.0, `fhir-release-2`
through `-6` at 2.1.0, and `fhir-release-1`, `-7`, `-8`, `-9`, `-10` reserved at
0.0.0. The database family, `fhir-store` and `fhir-loco` are unregistered, so
their names are free but also unclaimed by anyone else.

Local versions against those, after the bumps this pass made: `fhir` and
`fhir-core` at `3.0.0`, `fhir-release-2`…`-6` at `3.0.0`, `fhir-derive-macros`
at `1.2.0` (**P-4**), and the five reservations at `0.0.1` (**P-4a**). Every one
now clears its published number, which
[`scripts/check-published-match.sh`](../scripts/check-published-match.sh)
enforces from here on.

## Blockers

Ranked by what they would cost if published as-is.

### P-1 — The scaffold ports ship a store that does not exist, and a MySQL DDL emitter — **OVERTAKEN BY EVENTS, 2026-08-06**

*The section below recorded a real 2026-08-01 state and a real owner decision;
neither premise survived the week. It is retained in summary because the
decision ("publish scaffolds, mitigate in metadata") remains the precedent for
any future scaffold — but every factual claim it rested on is now false:*

- `fhir-oracle-map/src/ddl.rs` is a real Oracle emitter, executed live
  against 26ai with 0 invalid objects (**F-08** fixed 2026-08-03; the MySQL
  tokens survive only in doc comments and the test that *forbids* them).
- `fhir-oracle-store` (2,598 lines) and `fhir-mssql-store` (3,296 lines) are
  real stores, live-verified — **Store** level, not Scaffold (**F-68**,
  **F-65**).
- The "SCAFFOLD" `description` strings this section prescribed became the
  defect: four of the six manifests still carried them after the stores
  landed, which is exactly the above-the-level/below-the-level mismatch
  `C0.11` exists to prevent, inverted. Corrected 2026-08-06 (**F-76**):
  `fhir-oracle-store`'s description now records Store level with its evidence;
  the `-map`/`-gen` descriptions no longer claim a MySQL emitter or a missing
  store; `fhir-mssql-map`'s "never run against a green CI gate" is gone
  (`tests/upgrade.rs` runs the DDL live).

What survives of the original finding: the six ports remain **unpublished**,
and if published, the `0.4.0` line's changelogs must still say what each
version could and could not do — that obligation is unchanged.

### P-2 — The port version number contradicted its own changelog — **RESOLVED**

**Severity: High.** `O10.11` — a published version must match the source that
claims it.

Four sources said `0.4.0` and one said `0.1.0`:

| Source | Said |
| --- | --- |
| all six `Cargo.toml` | `version = "0.1.0"` |
| all six `Cargo.lock` | `0.4.0`, for all three crates of each port |
| all six `CHANGELOG.md`, top entry | `## 0.4.0 — tamper evidence that survives the database (2026-07-27)` |
| [`AGENTS/release.md`](../AGENTS/release.md#versioning) | "All six currently sit at `0.4.0`" |

**Owner chose `0.4.0`** (2026-08-01). Applied to all six
`[workspace.package]` blocks and to the eighteen `[workspace.dependencies]`
sibling pins, which carried the same stale `0.1.0`.

**The lock files then confirmed it mechanically.** Re-resolving all six
workspaces after the change produced **zero** `Cargo.lock` modifications: the
locks already said `0.4.0`, so the manifests were the stale side. Had `0.1.0`
been correct, all six locks would have been rewritten.

**One thing to carry into the first release.** None of the four changelog
releases ever reached crates.io — all eighteen port crate names are still
unregistered. The first publication will be `0.4.0` with no `0.1.x`–`0.3.x`
beneath it on the registry. That is legal and not unusual, but a reader
comparing the changelog against the registry sees three entries with no
artifact, so each changelog should say where those releases lived. Left for the
release itself rather than guessed at here.

### P-3a — `fhir-loco`'s advisories — **RESOLVED**

`loco-rs 0.16.4 -> 1.0.1`, which needed **no source changes**: a clean rebuild
of all targets compiled with zero errors. One advisory cleared outright
(`fxhash`, dropped by `scraper 0.25`); the remaining `quick-xml` DoS pair is
blocked on a loco release admitting `opendal 0.58`, and is documented in
`deny.toml` with its exposure argument and that argument's limit. All four
`cargo deny` categories are green.

**A regression this uncovered, which was mine.** Seven of the eight tests failed
after the upgrade — not because of loco, but because the **P-6** asset move had
changed `assets/`'s location and `fhir-loco/tests/requests/fhir.rs` still read
the old path. It was invisible because that crate's tests were never run after
that move. The same gap this register keeps finding elsewhere, committed here.
Fixed, along with the matching path in its README; all 8 pass.

### P-3 — `fhir-loco` could not be published as written — **RESOLVED**

**Severity: Medium.** It carried `publish = false`, had no `description`,
`license`, `keywords`, `categories`, or `readme`, and its two dependencies on
the sibling port were path-only, where Cargo requires a `version` on every
non-dev dependency at publish time.

All fixed: `publish = false` removed, the metadata added, and both dependencies
(and both dev-dependencies) now carry `version = "0.4.0"` alongside `path` —
path wins locally, the registry version is what ships.

`cargo publish --dry-run` now fails on **exactly one** thing, which is the
correct remaining state:

```text
error: failed to prepare local package for uploading
Caused by:
  no matching package named `fhir-sqlite-map` found
  location searched: crates.io index
```

That is publication *order*, not a manifest defect: `fhir-sqlite-map` and
`fhir-sqlite-store` have to reach the registry first. The manifest itself is
clean.

No `rust-version` was added. The six ports promise `1.90` and CI builds on
exactly that toolchain; this crate's floor against loco-rs and axum has never
been measured, and [`AGENTS/release.md`](../AGENTS/release.md#msrv) is explicit
that an unverified MSRV is a guess. Measure it before promising one.

### P-4 — `fhir-derive-macros` has diverged from its published 1.1.0

**Severity: High** (raised from Medium once measured). Violates `O10.11`
**now**, not at some future release.

Local version is `1.1.0`; crates.io already has `1.1.0`, and versions are
immutable. The question was whether the source had changed. It has. Measured by
packaging the local crate and diffing it against the registry copy fetched
through cargo:

| | `src/lib.rs` |
| --- | --- |
| published `1.1.0` | 554 lines |
| local `1.1.0` | 758 lines |
| difference | **206 added, 2 removed** |

The addition is the `qty-3` invariant support — a `QUANTITY_TYPES` list and the
field-matching helpers around it. `Cargo.toml.orig` is byte-identical between
the two, so nothing signals the divergence in the metadata.

This is precisely the failure
[`AGENTS/release.md`](../AGENTS/release.md#the-gate-that-matters-most) describes,
already happening: every local build resolves the **path** dependency and never
fetches the registry copy, so this workspace is green against 758 lines while
anyone writing `fhir-derive-macros = "1.1.0"` gets 554. The tree and the artifact
of the same name are different code, and nothing in CI checks it.

**Fixed: bumped to `1.2.0`**, along with its six dependency pins in `fhir` and
`fhir-release-2` … `-6`. `1.2.0` is the honest number — the change adds
validation behaviour rather than altering existing behaviour.

And the gate that would have caught it now exists:
[`scripts/check-published-match.sh`](../scripts/check-published-match.sh)
(**F-35**).

### P-4a — The reservation crates had diverged too

**This entry previously said the opposite, and was wrong.** It claimed the five
reservation crates were byte-identical to their published copies. That was based
on comparing `src/` trees and manifests-minus-the-license-line — a narrower
check than the claim implied. Running the new `O10.11` gate, which compares
*every* packaged file, found two things that comparison had missed:

- **`fhir-release-1`'s `README.md`** had gained a "What is actually available"
  section describing which releases are modelled. Not in the published `0.0.0`.
  Pre-existing, unrelated to any change made during this audit.
- **All five manifests** carry a `license` line changed by the P-7
  harmonization — which means that harmonization *created* four new `O10.11`
  violations, on top of the one it exposed.

Both are the same defect as P-4: a changed tree on an immutable published
number. **Fixed by bumping all five to `0.0.1`.** They are workspace members
only — nothing depends on them — so the bump is inert.

The general point stands and is worth keeping: the quintuple governs the source
from now on and does **not** reach artifacts already on crates.io.
`fhir-core 2.2.0` and `fhir 2.1.0` keep the terms they shipped with. The
difference is that for the reservations, a `0.0.1` now exists to carry the new
licence, where before this entry assumed none was needed.

### P-5 — Declared `repository` URLs did not resolve — **RESOLVED**

**Severity: Medium.** Every crate pointed at a repository that returned 404
anonymously, in three different flavours:

| Declared | Crates | Status |
| --- | ---: | --- |
| `fhir-rust-crate/fhir-rust-crate` | 11 | 404 — and the **org itself** does not exist |
| `joelparkerhenderson/fhir-rust-crate` | 1 | 404 — user exists, repo does not |
| `fhir-rust/fhir`, `…/fhir-<engine>` ×6, `…/fhir-store`, `…/fhir-loco` | 21 | 404 each |

The diagnosis that mattered was not "they 404" but **why**. Checking the org
rather than only the repos showed:

```text
200  github.com/fhir-rust            <- the org is public
200  github.com/fhir-rust/fhir-rust  <- the monorepo is public
404  everything else
```

So these were not private repositories an anonymous check cannot see. They were
**per-crate URLs from the layout that preceded the monorepo** — one repository
per port — describing a structure that no longer exists. The same fossil as
**F-11** and **F-39**, in a third place.

**Disposition: FIXED.** All 33 crates now declare
`https://github.com/fhir-rust/fhir-rust/`, which returns **200** and matches
`git remote get-url origin` exactly. That is not merely a URL that resolves: it
is the honest one, because this *is* a monorepo and one URL describes it.

Verified by resolving `cargo metadata` across all nine workspaces — 33 crates,
**one** distinct repository value, no exceptions. The `homepage` field on `fhir`
carried the same stale URL and was corrected with it.

**Why it was worth fixing for something `cargo publish` never checks.**
crates.io renders this as the "Repository" link on every crate page and docs.rs
resolves "Source" through it — a 404 there is the first thing a cautious reader
clicks, on crates that handle PHI. `O10.10` requires an SBOM per release whose
whole purpose is tracing an artifact to its source, and a dead repository link
undercuts precisely that. It was also already live: the published `fhir` 2.1.0,
`fhir-core` 2.2.0 and `fhir-derive-macros` 1.1.0 point at an org that does not
exist, and will keep doing so until each is republished.

**The same fossil in the documentation, also fixed.** All six
`doc/containers.md` files told a reader to find the FHIR packages at
`../fhir-rust-crate/…` or under a specific developer's home directory. Both are
the paths that made every spec-dependent test skip while reporting success
(**F-39**, **F-42**). They now name `../fhir/doc/fhir-specifications`, which is
what the code resolves, and say what the old entries cost.

### P-6 — The published crates could not obtain a map at all — **RESOLVED**

**Severity: High.** `RelMap::from_gz_bytes` was the only constructor and the
maps lived outside every package root, so `cargo add fhir-sqlite-map
fhir-sqlite-store` produced two crates that could not shred, reconstruct,
`init`, `put`, or `get` anything. `cargo package --list` reported zero asset
files in both.

**Fixed on the owner's decision: `include_bytes!` behind per-version features**,
matching the shape `fhir/`'s release crates already use.

- `assets/` moved into the **map crate**, since `include` cannot escape a
  package root.
- `RelMap::bundled("r5")` returns a ready map; features `r3`, `r4`, `r5`, with
  `r5` default. `bundled_versions()` reports what a build carries.
- The `store` crates forward the same features.

Verified: `fhir-sqlite-map` packages **19 files, 2.6 MiB** including all three
maps; `bundled()` yields 117 / 146 / 158 resources for R3 / R4 / R5; clippy is
clean at four feature combinations including `--no-default-features`; and the
map and store README examples were compiled against the real crates.

**Stated limit:** the features gate compilation, not download. A `.crate` is
static, so all three maps ship (~2.5 MB) however few are enabled. Making the
download opt-in would take separate data crates. The caveat lives in
`bundled`'s doc comment.

This also removes the constraint this entry previously placed on publishing:
the store crates are no longer unusable-as-published, so the only remaining
naming question is **P-1**'s, which is decided.

### P-7 — Split licensing inside the model family — **RESOLVED**

**Severity: Medium.** Was: `fhir` and `fhir-derive-macros` declared the
five-license form while `fhir-core` and `fhir-release-1` … `-10` declared bare
`MIT`, and the HTTP crate declared none at all. The facade's declaration did not
describe the set it pulls in.

**Harmonized to the quintuple on the owner's instruction.** All 32 packages now
declare, identically:

```
MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only
```

Twelve manifests changed — `fhir-core`, the ten `fhir-release-*`, and
`fhir-store`. Verified by resolving `cargo metadata` across every
workspace: 32 packages, one distinct licence string. `cargo deny check
licenses` re-run on `fhir/` and `fhir-sqlite/` — **ok** in both; the `OR`
expression satisfies a permissive-only allowlist because at least one option
(MIT, Apache-2.0, BSD-3-Clause) is allowed, and `OR` lets the recipient choose.

A root [`LICENSE.md`](../LICENSE.md) now states the grant once for the whole
repository — there was none before, though the README declared the five.

Two things it deliberately does **not** do:

- **It does not relicense what is already published.** A crates.io version is
  immutable, metadata included, so the releases of `fhir`, `fhir-core`,
  `fhir-derive-macros`, and `fhir-release-1` … `-10` that predate this stay
  under the terms they shipped with. The quintuple applies from each crate's
  next version.
- **It does not add licence texts to the crate packages.** See P-9.

### P-9 — No crate shipped a licence text — **RESOLVED**

**Severity: Low.** `cargo package` includes only what is inside the crate
directory. The six ports kept `LICENSE-APACHE` and `LICENSE-MIT` at the **port
root**, one level above `crates/<name>/`, and `fhir/LICENSE.md` sat at its
workspace root — so all 32 packages shipped the `license` *field* and none of
the five licences it names.

**Fixed.** `LICENSE.md` — the repository's canonical statement of the quintuple
— now sits in every crate directory. The eleven crates in `fhir/` that declare
an explicit `include` list (`fhir-core`, `fhir-release-1` … `-10`) had it added
there too, since an `include` list means nothing is packaged by default.

Verified across **all 32**, not sampled: each `cargo package --list` contains
`LICENSE.md`. 32 ship it, 0 missing.

Nothing about this blocked publication — crates.io requires the field, not a
file, and the SPDX expression is the operative grant either way. It is a
completeness fix: the licence now travels with the artifact instead of only
with the repository.

### P-10 — The map assets were half-moved and uncommitted, and `cargo publish` refused — **RESOLVED**

**Status: RESOLVED 2026-08-02.** Committed on branch `commit-map-assets` —
48 files changed: 24 added at the new path, 24 removed from the old.

`cargo package` would not run on any of the six ports:

```
error: 12 files in the working directory contain changes that were not yet
committed into git:
crates/fhir-sqlite-map/assets/CHECKSUMS.txt
crates/fhir-sqlite-map/assets/fhir-sqlite-relmap-r3.json.gz
crates/fhir-sqlite-map/assets/fhir-sqlite-relmap-r4.json.gz
crates/fhir-sqlite-map/assets/fhir-sqlite-relmap-r5.json.gz
```

**This was a half-committed move, not simply a missing `git add`** — and the
first write-up of this entry said the latter, which was wrong in a way that
mattered. **P-6** relocated the assets from `fhir-<engine>/assets/` to
`fhir-<engine>/crates/fhir-<engine>-map/assets/`, so that `RelMap::bundled()`
could reach them from inside the crate that actually ships. Neither side of that
move was committed:

| Path | On disk | In git |
| --- | --- | --- |
| `fhir-<engine>/assets/` (old) | **absent** | **tracked** |
| `crates/fhir-<engine>-map/assets/` (new) | present | untracked |

Nothing in `.gitignore` covered either. The asymmetry is what made the third
consequence below worse than "no assets": a clone got 15 MB of assets at the
path nothing reads, and none at the path `assets_root()` points to. Files
present, in the wrong place, is a harder failure to diagnose than files
absent.

Three consequences, in increasing order of how quietly they failed:

1. `cargo publish` stopped with the error above. Loud, and the easiest to
   notice.
2. `--allow-dirty` made it proceed, and the assets *were* in the package list —
   so the escape hatch worked and hid the problem rather than surfacing it.
3. **A fresh clone had no usable assets.** `RelMap::bundled()` (**P-6**) reads
   them and `assets_current.rs` gates on them, both at the *new* path — where a
   clone had nothing. On a clean checkout the drift gate had nothing to compare
   against — the `T11.12` failure shape: a check that cannot run looks exactly
   like a check that passed.

That third one is why this was a blocker rather than a nuisance. **P-6**
resolved "the published crates could not obtain a map at all" by bundling the
maps; bundling files that are not in the repository resolved it only on the
machine that generated them.

**Resolved by committing the move** — 24 files added at the new path, 24
removed from the old — the owner having chosen that over the alternative — generating at build time — which **P-6** had already rejected
because it needs the FHIR specification packages present at build time and a
published crate has no way to obtain them.

All 18 checksums were verified against their files before the commit, and again
in the clean checkout afterwards.

**Verified in a clean checkout, not in the working tree.** Running
`cargo package` where the assets had just been written would prove nothing about
a clone. A detached worktree at `HEAD` was used instead, and there all six ports
package **without** `--allow-dirty`, each carrying four asset files:

```
postgresql  ok, 4 asset file(s) in package
sqlite      ok, 4 asset file(s) in package
mysql       ok, 4 asset file(s) in package
mariadb     ok, 4 asset file(s) in package
mssql       ok, 4 asset file(s) in package
oracle      ok, 4 asset file(s) in package
```

**The maintenance cost this accepts:** gzip is not diffable, so every
regeneration replaces the blob wholesale — ~2.5 MB of churn per port, and a
review that cannot see what changed. `G2.2a` compares decompressed content
rather than the container (**F-41**), so the *gate* still tells the truth about
drift; it is the human diff that is opaque.

**The standing rule this leaves behind:** `cargo package` must succeed
**without** `--allow-dirty` before anything is published. Nothing should be
published on the strength of a dry run that had to be told to ignore the working
tree — that flag is what kept this defect invisible.

## Order of publication

Dependencies first. The split and the rewiring (**F-45**) added a step and made
one crate a hard prerequisite for eighteen others.

1. **`fhir-store`** — the persistence core. Nothing in the database family can
   publish until it is on the registry: all eighteen port crates now depend on
   it by version, so this is no longer convenient ordering but a gate.
2. **Model** — `fhir-core` and `fhir-derive-macros`, then `fhir-release-2` … `-6`
   (each depends on both), then `fhir` (depends on all). The five reserved
   `0.0.0` crates need nothing. Independent of step 1.
3. **Databases**, per port and independently of the others (`W16.11`) —
   `fhir-<engine>-map`, then `fhir-<engine>-gen` (depends on map), then
   `fhir-<engine>-store` (depends on map and on `fhir-store`, and dev-depends on
   gen, which must therefore already be on the registry for the verification
   build).
4. **`fhir-loco`** — last. It depends on `fhir-sqlite-map` and
   `fhir-sqlite-store`, so it follows the whole of step 3 for that port.

All six ports are in step 3 — **P-1** is overtaken (both former scaffolds are
Store level now), and each port's honesty rests on `description` and README
staying current with the matrix (**F-76** is what happens when they do not).

**A consequence of step 1 worth naming.** A change to the engine-agnostic half
now needs a `fhir-store` release before any port can take it. That version
coupling is what bought the removal of six copies, and it is the reason the
previous design preferred duplication plus a gate. The trade is recorded in
**F-45**.

## Dry runs

`cargo publish --dry-run --allow-dirty` on every crate that can be dry-run —
those whose dependencies already exist on the registry, or which have none.

| Crate | Result | Packaged |
| --- | --- | --- |
| `fhir-store` | **clean** | 9 files, 17.3 KiB compressed |
| `fhir-postgresql-map` | **clean** | 19 files, 2.5 MiB compressed |
| `fhir-sqlite-map` | **clean** | 20 files, 2.5 MiB |
| `fhir-mysql-map` | **clean** | 20 files, 2.5 MiB |
| `fhir-mariadb-map` | **clean** | 20 files, 2.5 MiB |
| `fhir-mssql-map` | **clean** | 20 files, 2.6 MiB |
| `fhir-oracle-map` | **clean** | 19 files, 2.5 MiB |
| `fhir-derive-macros` | **clean** | 7 files, 10.8 KiB |
| `fhir-core` | **clean** | 19 files, 46.3 KiB |
| `fhir-loco` | fails on order only — see P-3 | — |
| `fhir-<engine>-store` ×6 | fails on order only — they now need `fhir-store` and their own `map`/`gen` on the registry first | — |

The `map` crates grew from ~30 KiB to ~2.5 MiB when the bundled maps moved
inside them (**P-6**). That is the cost of `cargo add` producing a working
engine, and it is the same 2.5 MiB whichever version features are enabled.

Everything else — the `-gen` and `-store` crates, and the rest of the model
family — cannot be dry-run until its dependencies exist on the registry. Cargo
resolves `version` from crates.io during the verification build, not from
`path`.

**A clean dry run means the package is well-formed. It means nothing else.**
`fhir-oracle-map` passes, and `src/ddl.rs` — verified present in its file list —
is the MySQL emitter (**P-1**, **F-08**). The mechanical gate cannot see that,
which is why `C0.11` and the conformance matrix exist. Read this table as the
absence of one class of packaging defect, not as readiness.

## What is not assessed here

- Whether the crates build, test, and clippy clean — that is the green gate, run
  per family, not a publishing question.
- Release **checksums** for published artifacts — these only exist once
  something is published.

`O10.10`'s other three are now done and recorded in **F-43**: `cargo deny`
(advisories, licenses, bans, sources) across all eight workspaces, `cargo
audit`, and a CycloneDX SBOM for every crate. All workspaces are green on all
four categories, `fhir-loco` included since the loco-rs 1.0.1 upgrade (**P-3a**).
