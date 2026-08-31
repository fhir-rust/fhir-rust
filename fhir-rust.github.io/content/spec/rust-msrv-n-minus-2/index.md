# Rust MSRV — current N-2

**Normative, cross-family.** This governs every crate in the repository —
model, persistence core, databases, and HTTP surface alike — because a
Minimum Supported Rust Version is a property of the release surface, not of
one family. It is the third cross-family document in `spec/`, alongside the
[agents directory name](../agents-directory-name-is-lowercase/index.md)
(`AG1`) and [what a git tag names](../git-tags-name-published-versions/index.md)
(`TG1`).

Requirement ids use the prefix `RV1`. Normative language is `C0.1`; ids are
permanent under `C0.5`, **including across this file's move** — this
directory replaces `spec/rust-msrv-n-minus-3/`, and `RV1.1`–`RV1.6` below are
the same requirements that lived there, amended in place for the new target
rather than retired and renumbered. `RV1.1 was N-3; is now N-2` is the
amendment; everything else the old rule established about *how* the number is
declared, verified, and raised carries forward unchanged.

## The rule

- **RV1.1** The Minimum Supported Rust Version is **current minus two**: with
  `N` the latest Rust stable release, a crate MUST NOT require a toolchain
  newer than `N-2`. Declaring an *older* MSRV is permitted; declaring a newer
  one is a defect, whatever the tree happens to compile with. *(Amended
  2026-08-29: was `N-3`. The owner's call, not a defect being corrected — a
  narrower floor trades headroom for currency, and `RV1.2`'s math is what
  the trade actually costs.)*
- **RV1.2** `N-2` counts stable releases, not months. Rust ships every six
  weeks, so `N-2` is roughly twelve weeks — about three months — of headroom
  for downstream users, distributions, and pinned CI images. *(One release of
  headroom narrower than `N-3`'s four months.)*
- **RV1.3** `N` is evaluated at the moment a declaration is written or
  changed, not continuously. A crate does not fall out of conformance because
  upstream released; it falls out only if someone raises its `rust-version`
  past the `N-2` of that day.

  The asymmetry is deliberate. A floor that moved on its own schedule would
  break downstream builds on a calendar the repository does not control,
  which is the exact harm the policy exists to prevent.

## Declaring it

- **RV1.4** Every publishable crate MUST declare `rust-version` in its
  `Cargo.toml`. A workspace member SHOULD inherit it
  (`rust-version.workspace = true`) so that the number exists in exactly one
  place per workspace.
- **RV1.5** A declared `rust-version` MUST be verified by that crate's CI on
  **exactly** that toolchain — `cargo +$msrv check --workspace --all-targets
  --locked`, as the ports' `msrv` job already does. An unverified MSRV is a
  guess, and it breaks silently the first time anyone uses a newer language
  feature.
- **RV1.6** Raising a declared MSRV is a **minor-version event**, not a
  patch, and MUST be recorded in that crate's `CHANGELOG.md`. Lowering one is
  not a breaking change and MAY be a patch.
- Only the minor version is pinned. Patch releases of the MSRV minor version
  (`1.(N-2).x`) are all acceptable; the recorded value uses `.0`, dropped when
  citing the bare minor (`1.96`, not `1.96.0`, matches this repository's
  existing convention).
- Pre-release channels (beta, nightly) are never the MSRV and MUST NOT be
  required by any workspace target, including the fuzz targets — the
  `publish = false` fuzz crates stay outside every workspace precisely so
  this holds regardless of what they need.

## Maintenance

When a new stable Rust release `1.N` appears, the MSRV becomes `1.(N-2)`
**in the same change** that observes the release:

1. Set `rust-version` in the root `Cargo.toml` to `1.(N-2)`.
2. Set the pinned toolchain in the CI `msrv` job to the same value — or
   confirm the job reads `rust-version` from `Cargo.toml` dynamically, as
   every port's already does, in which case step 1 is the only edit.
3. Run `cargo +1.(N-2) check --all-targets --workspace --locked` and fix
   anything the older toolchain rejects — the MSRV is a floor the code must
   meet, not a ceiling on what the code may need.

Raising the MSRV is therefore routine and expected, not a breaking change to
be avoided. Lowering it below `N-2` (to support an older consumer) is a
design decision for `plan.md`, not a convenience.

## CI enforcement

CI MUST verify the MSRV, not merely declare it. The `msrv` job installs the
exact pinned toolchain and runs `cargo check --all-targets --workspace` with
it. `cargo check` (not `cargo build`) is sufficient and fast: the MSRV
question is "does this compile", and the `test` job already answers "does
this work" on stable.

The `msrv` job is separate from the `test` job so a failure names the cause
directly: `test` red means a behavior regression, `msrv` red means the code
started requiring a newer toolchain than the policy allows.

## Where the tree stands

Non-normative, measured 2026-08-29 (`rustc 1.98.0`, so `N-2` is **1.96**)
across all 41 `[package]` manifests, all nine workspaces:

| Crates | Declared | Under `RV1.1` |
| --- | --- | --- |
| model family (`fhir/`) — 14 crates | `1.96` | conformant — at the new floor, not above it |
| the six database ports — 6 roots, 18 members inheriting | `1.96` | conformant, same reason |
| [`fhir-store/`](../../fhir-store/) | `1.96` | conformant — `RV1.4` was unmet until this change; see below |
| [`fhir-loco/`](../../fhir-loco/) | `1.96` | conformant — `RV1.4` was unmet until this change; see below |
| the seven `fuzz/` crates | none | out of scope — `publish = false` |

**Two things this measurement closes, not just records.** The previous
version of this table (under the old `N-3` rule) found `fhir-store` and
`fhir-loco` declaring no `rust-version` at all — `RV1.4` unmet in both, a gap
this document itself said was worth catching and nobody had. Both now
declare `1.96`, verified by `cargo +1.96 check --all-targets --workspace
--locked` before being written down (`RV1.5`), and both gained an `msrv` CI
job for the first time — declaring the number without a job that builds on
it would repeat exactly the failure mode `RV1.5` exists to prevent.

Every crate in the tree now sits at the same declared value, `1.96` — the new
`N-2` exactly, not comfortably below it the way the old `N-3`-era values
(`1.88`, `1.90`) sat eight and ten releases back. That is a deliberate
consequence of narrowing the policy, not an oversight: a floor set exactly at
the ceiling has zero slack before the next stable release makes it stale
again, which is the cost `RV1.2`'s three-months-of-headroom figure already
names. The next `1.N` release is what starts the clock.
