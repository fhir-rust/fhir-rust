# Maintainers and access continuity

This file is the roster and the honest answer to the question an enterprise
procurement review asks about any software that touches patient data: *what
happens if the person who can ship a fix is unavailable?*

It is deliberately not aspirational. Everything below describes the project as
it is on the day you read it in git history, not a structure the project hopes
to grow into.

## Roster

| Person | Contact | Role | Since |
| --- | --- | --- | --- |
| Joel Parker Henderson | [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com) · [ORCID 0009-0000-4681-282X](https://orcid.org/0009-0000-4681-282X) | Maintainer (sole) | 2026-08-01 |

**The bus factor of this project is one.** Every commit in the repository —
104 of them, from 2026-08-01 to 2026-08-22 — carries the same author and
committer. One person can accept a pull request, one person can publish a
crate, and one person decides what a specification silence means. No second
maintainer exists and no legal entity is a party to the project.

Everything else in this file follows from that sentence, and no wording
elsewhere in the repository should be read as softening it.

## Publishing identities and where they live

These are the credentials and configured identities that can put bytes in front
of a user. Naming them is the point: an inventory nobody has written down is an
inventory nobody can hand over.

| Identity | What it publishes | Held by | Recovery if the holder is unavailable |
| --- | --- | --- | --- |
| The GitHub organisation `fhir-rust` and the repository in it | the source, issues, settings, and any future release | the maintainer's account, as the organisation's owner | GitHub's organisation-recovery process, which is between GitHub and the owner. Being org-owned rather than user-owned means a second owner *could* be added without moving the repository — that is the cheapest continuity improvement available to this project, and it has not been taken |
| The crates.io account owning all 34 published crates | every published version of every crate | the maintainer | the crates.io owner list is the recovery surface, and it holds one account. Crate ownership must move before anyone else can publish, whatever repository access they have |
| `CARGO_REGISTRY_TOKEN`, a repository secret in the `crates-io` environment | nothing today — see the note below | the repository | a long-lived registry token, not Trusted Publishing. It is the one stored credential in the project and therefore the one that can leak |
| The `GITHUB_TOKEN` minted per workflow run | CI results only; no artefact is published from CI | GitHub, per run; nothing stored | not applicable — there is no credential to lose |
| The Codeberg and GitLab push mirrors | copies of the source | the maintainer's accounts on each | not applicable to continuity: they are mirrors, and GitHub is canonical |

**Two honest notes on that table.**

*The publish workflow does not run.* Each port carries
`fhir-<engine>/.github/workflows/publish.yml` — manual `workflow_dispatch`, a
typed `confirm` input, a dry-run default, gates before upload. It is carefully
written and it is **inert**: GitHub reads workflows only from
`.github/workflows/` at the repository root, and the root directory contains no
publish workflow. The 2026-08-22 publication of all 34 crates was therefore run
from a developer machine, as [`spec/publishing.md`](spec/publishing.md) records.
Either promote those workflows to the root or delete them; a workflow that looks
like a control but is not one is worse than no workflow.

*Nothing is signed.* Commits and tags carry no OpenPGP or SSH signature
(`git log -1 --format=%G?` returns `N`), there are no tags at all, and there are
no GitHub releases. A consumer cannot currently verify that a given commit came
from the maintainer, and there is no signed artefact to check a download
against.

## If the maintainer is unavailable

There is no succession plan that a document can create. What exists instead:

- **Nothing already published disappears.** The 34 crates on crates.io are
  immutable: a version can be yanked by its owner but never replaced or
  removed, and `docs.rs` builds persist. A deployment already pinned to a
  published version is unaffected by maintainer availability.
- **Nothing new ships.** No fix, no security patch, no publish. Because the
  project has no release cadence and no support commitment, this is a smaller
  change from the status quo than it would be for a mature project — which is
  itself a statement about the status quo.
- **The work is not lost.** The licence is a five-way permissive/copyleft
  choice ([`LICENSE.md`](LICENSE.md)), the history is public, every gate is a
  committed script (`scripts/check-shared-core.sh`,
  `scripts/check-published-match.sh`, `scripts/check-doc-examples.sh`), and
  every design decision is in [`spec/`](spec/index.md) with its reasoning. A
  fork is a complete and legitimate continuation, and the project's position is
  that it should be taken rather than waited on.
- **A vulnerability report has a fallback.** [`SECURITY.md`](SECURITY.md)
  commits to acknowledgement within 7 days and an assessment within 30. If a
  private report receives no acknowledgement inside that window, the policy
  already tells you to publish, and disclosure becomes your call. That path does
  not depend on the maintainer being available.

If you depend on this software in a clinical setting and that position is not
acceptable to you — it reasonably may not be, and this project is pre-release —
the mitigation is on your side of the boundary: pin a version, keep a fork you
can build, and budget for maintaining it. That is a truthful answer, and it is
more useful than a continuity plan with nobody behind it.

## What the maintainer is accountable for

Under [`AI_STATEMENT.md`](AI_STATEMENT.md), agentic AI tooling writes a large
share of the bytes in this repository. That changes nothing here: the
maintainer is the author of and accountable for every merged change, whatever
produced it. A tool is not named as an author, a reviewer, or a signer.

## Adding a maintainer

The route is in [`GOVERNANCE.md`](GOVERNANCE.md): sustained contribution, then
an invitation. It is open, and it is deliberately unbureaucratic. When someone
takes it,
this file gains a row, [`CODEOWNERS`](CODEOWNERS) gains their handle on the
areas they own, and the table above gains a second holder wherever the identity
permits one — which, per the first row, includes the GitHub organisation.
Those three edits are the whole mechanism.
