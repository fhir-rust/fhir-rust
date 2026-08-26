# Rust MSRV — current minus three

**Normative, cross-family.** This governs every crate in the repository — model,
persistence core, databases, and HTTP surface alike — because a Minimum
Supported Rust Version is a property of the release surface, not of one family.
It is the second cross-family document in `spec/`, alongside the non-normative
[publishing readiness](publishing.md) note.

Requirement ids use the prefix `RV1`. Normative language is `C0.1`; ids are
permanent under `C0.5`.

## The rule

- **RV1.1** The Minimum Supported Rust Version is **current minus three**: with
  `N` the latest Rust stable release, a crate MUST NOT require a toolchain newer
  than `N-3`. Declaring an *older* MSRV is permitted; declaring a newer one is a
  defect, whatever the tree happens to compile with.
- **RV1.2** `N-3` counts stable releases, not months. Rust ships every six
  weeks, so `N-3` is roughly eighteen weeks — about four months — of headroom
  for downstream users, distributions, and pinned CI images.
- **RV1.3** `N` is evaluated at the moment a declaration is written or changed,
  not continuously. A crate does not fall out of conformance because upstream
  released; it falls out only if someone raises its `rust-version` past the
  `N-3` of that day.

  The asymmetry is deliberate. A floor that moved on its own schedule would
  break downstream builds on a calendar the repository does not control, which
  is the exact harm the policy exists to prevent.

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
- **RV1.6** Raising a declared MSRV is a **minor-version event**, not a patch,
  and MUST be recorded in that crate's `CHANGELOG.md`. Lowering one is not a
  breaking change and MAY be a patch.

## Where the tree stands

Non-normative, measured 2026-08-21 across all 47 manifests. `N` is **1.98.0**
(released 2026-08-18), so `N-3` is **1.95**.

| Crates | Declared | Under `RV1.1` |
| --- | --- | --- |
| model family (`fhir/`) — 14 crates | `1.88` | conformant — older than `N-3`, which is permitted |
| the six database ports — 6 roots, 18 members inheriting | `1.90` | conformant, same reason |
| [`fhir-store/`](../fhir-store/) | none | `RV1.4` unmet |
| [`fhir-loco/`](../fhir-loco/) | none | `RV1.4` unmet |
| the seven `fuzz/` crates | none | out of scope — `publish = false` |

Nothing in the tree requires a toolchain newer than `N-3`, and nothing is close
to it: the two declared floors sit ten and eight releases back respectively.
`RV1.1` therefore binds nobody today. It is written down so that the first crate
to reach for a newly stabilized feature has a number to be measured against
rather than an argument to have.

Both `fhir-store` and `fhir-loco` say in their manifests that the omission is
deliberate — the ports promise 1.90 and CI builds on exactly that, but neither
of those two crates has measured its own floor, and
[`agents/release.md`](../agents/release.md) is explicit that an unverified MSRV
is a guess. `RV1.4` says the number must exist; `RV1.5` says measure it before
writing it down. Those are not in tension: measure, then declare.
