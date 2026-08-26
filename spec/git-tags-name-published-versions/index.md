# Git tags name published versions

**Normative, cross-family.** This governs every crate in the repository —
model, persistence core, databases, and HTTP surface alike — because what a
tag means is a property of the release surface, not of one family. It is the
third cross-family document in `spec/`, alongside the [agents directory
name](../agents-directory-name-is-lowercase/index.md) (`AG1`) and the [MSRV
rule](../rust-msrv-n-minus-3/index.md) (`RV1`).

Requirement ids use the prefix `TG1`. Normative language is `C0.1`; ids are
permanent under `C0.5`.

## Why this exists

Thirty-four crates were published to crates.io on 2026-08-22, and for four days
the repository contained **no tag and no release** pointing at what was
published. A published version is immutable and a git history is not, so
without a tag there was no way — for a maintainer, an auditor, or a downstream
consumer — to answer "which source produced `fhir-sqlite-store 0.5.0`?" except
by reading commit messages and guessing.

A second consequence, less obvious and more damaging in practice: with no tags
there are no GitHub Releases, and with no releases there is no feed. Someone
who reads about this project has no way to hear about it again.

## The unit a tag names

- **TG1.1** Every version published to crates.io MUST have a git tag naming it.
- **TG1.2** A tag names an **independently-versioned unit**, not necessarily a
  single crate. A unit is:
  - a **workspace that pins its members to one version** — the six database
    ports each declare `version` once at the workspace root and every member
    crate carries `version.workspace = true`, so `fhir-sqlite-map`, `-gen` and
    `-store` cannot diverge and one tag names all three;
  - otherwise, a **single crate**. The model family's crates version
    independently and demonstrably diverge — `fhir` 4.1.0, `fhir-core` 3.2.0,
    `fhir-derive-macros` 1.5.0 — so each takes its own tag.

  The distinction is mechanical, not editorial: a unit is a workspace if its
  members inherit `version.workspace`, and a crate otherwise. Nothing here
  requires a judgement about what "belongs together".
- **TG1.3** Tag names MUST be `<unit>-v<version>`, where `<unit>` is the
  workspace directory name or the crate name and `<version>` is the exact
  published version. `fhir-sqlite-v0.5.0`, `fhir-core-v3.2.0`. The `-v` form is
  what `cargo-release` and `release-plz` assume, so choosing it keeps the door
  open to automating this later.

## What a tag points at

- **TG1.4** A tag MUST point at a commit whose source, for that unit, is what
  was published — that is, a commit at which the unit's declared version equals
  the version on crates.io. `scripts/check-published-match.sh` is what
  establishes that equality.
- **TG1.5** Tags MUST NOT be moved or deleted once pushed. A published version
  is immutable, so the pointer to its source must be too. This is `C0.5`'s
  reasoning applied to tags rather than to requirement ids: a tag that moves
  makes every prior citation of it a lie.
- **TG1.6** A tag SHOULD be annotated (`git tag -a`) rather than lightweight,
  so that it carries a date, an author and a message. Signing is not required
  and is not currently possible: this repository signs nothing
  ([`MAINTAINERS.md`](../../MAINTAINERS.md)).

## Releases

- **TG1.7** Each tag SHOULD have a GitHub Release. The release note is where a
  human-readable summary lives; the tag is only a pointer.

  This is not decoration. `releases.atom` is the project's only subscribable
  feed, and creating releases is the whole of what it takes to have one.
- **TG1.8** A release note MUST NOT claim more than the [conformance
  matrix](../databases/conformance-matrix.md) supports (`C0.11`). Publication
  is not a conformance claim, and a release is the most likely place for that
  distinction to get lost.

## The retroactive tags

- **TG1.9** The 2026-08-22 publication was tagged retroactively on 2026-08-26.
  Sixteen tags — six ports, `fhir-store`, `fhir-loco`, `fhir`, `fhir-core`, and
  the six release crates published at 4.1.0 — all point at **`e28964e`**, the
  commit at which the repository recorded the publication as complete and at
  which `scripts/check-published-match.sh` reports `34 matched`.

  **What this does not establish**, stated because a retroactive tag invites the
  assumption: `e28964e` is the commit at which source and registry are known to
  agree, not necessarily the working tree each individual `cargo publish` ran
  from. Those uploads happened from a developer machine over 2026-08-21/22
  ([publishing readiness](../publishing.md)) and the exact per-crate commit is
  no longer recoverable. `TG1.4` is met — the versions do agree at `e28964e` —
  but a reader should not read more precision into these sixteen tags than that.

- **TG1.10** Six published crates originally had **no tag**:
  `fhir-derive-macros` 1.5.0, and `fhir-r1`, `fhir-r7`, `fhir-r8`, `fhir-r9`,
  `fhir-r10` at 0.0.1. They were published before this repository existed —
  its history begins 2026-08-01 — so there was no commit to point at, and
  inventing one would have been worse than the gap. The rule was "they
  acquire tags at their next published version", and it held: the 2026-08-26
  republication of all 34 crates gave every unit, these six included, a tag
  at the release commit. The untagged pre-repository versions remain
  untagged, permanently and correctly.

## Assurance

- **TG1.11** `scripts/check-tags.sh` verifies what can be verified offline:
  every tag is well-formed under `TG1.3`, names a unit that exists, and does not
  collide; and it reports any unit whose current source version has no tag.

  It **reports** rather than fails on that last case, deliberately: a version
  bump legitimately precedes its publication, so failing there would block
  ordinary work to catch a mistake that a release checklist catches better.
  Whether a tag exists for every version *actually on crates.io* cannot be
  checked without querying the registry, which `scripts/check-published-match.sh`
  already does for versions.
