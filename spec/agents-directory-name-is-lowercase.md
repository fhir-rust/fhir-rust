# The AI agents directory is named `agents`, lowercase

**Normative, cross-family.** This governs every directory in the repository that
holds instructions for AI agents, at any depth and in any family. Requirement ids
use the prefix `AG1`. Normative language is `C0.1`; ids are permanent under
`C0.5`.

## The rule

- **AG1.1** A directory holding instructions for AI agents MUST be named
  **`agents`**, in lowercase. `AGENTS`, `Agents`, and `.agents` are
  non-conforming names for the same thing.
- **AG1.2** This binds **directories only**. The `AGENTS.md` *file* keeps its
  uppercase name at every level it appears, because that spelling is the
  cross-tool convention agents actually look for; renaming it would make the
  file invisible to the tools it exists for. A tree that holds both
  `AGENTS.md` and `agents/` is correct, not inconsistent.
- **AG1.3** A tool that fixes the name itself governs its own path: Claude
  Code's `.claude/agents/` is already lowercase by that tool's requirement, and
  `AG1.1` MUST NOT be read as licence to rename a directory whose name a tool
  defines.
- **AG1.4** Every path that names such a directory MUST use the lowercase
  spelling wherever it is written — links in `.md` files, paths in CI
  configuration, prose paths in shell scripts and in `Cargo.toml` comments. A
  link that resolves only because the local filesystem is case-insensitive is a
  defect under this requirement, even though nothing is observed to break
  locally.

  This is the whole reason the rule is worth writing down. macOS ships a
  case-insensitive filesystem by default, so `AGENTS/release.md` and
  `agents/release.md` are the same file on the machine most of this repository
  is edited on, and are different files on the Linux runners that build it and
  on the forge that renders it. A mixed tree stays green locally and 404s in CI
  and on the web.

## Renaming, when it happens

- **AG1.5** A rename under `AG1.1` MUST move the directory and update every
  reference to it in the same commit. A rename that leaves references behind
  satisfies the letter of `AG1.1` and breaks `AG1.4` in the same stroke.
- **AG1.6** On a case-insensitive filesystem, `git mv AGENTS agents` does not
  record a rename. It MUST be done through an intermediate name — `git mv AGENTS
  agents-tmp && git mv agents-tmp agents` — and the result MUST be confirmed
  with `git ls-files` rather than `ls`, which reports the name the filesystem
  chooses to show rather than the name that is committed.

## Where the tree stands

Non-normative, measured 2026-08-21. **The tree conforms**, as of the rename
recorded below.

| Path | State |
| --- | --- |
| [`agents/`](../agents/index.md) — 8 files | conforming |
| [`fhir/agents/`](../fhir/agents/) — 5 files | conforming |
| `fhir/.claude/agents/` | conforming — the tool defines it (`AG1.3`) |
| `AGENTS.md` at the root, in `fhir/`, and in all six ports | conforming — a file, not a directory (`AG1.2`) |

The rename was done on 2026-08-21 under `AG1.5` and `AG1.6`: both directories
moved through an intermediate name so that git recorded the move at all, and the
**111 references across 30 files** were rewritten in the same change — most
heavily `agents/release.md` (27), `agents/conventions.md` (15), and
`agents/testing.md` (14), plus manifest comments in `fhir-store/Cargo.toml` and
`fhir-loco/Cargo.toml`, doc comments in `fhir/src/codegen/naming.rs` and
`render.rs`, their copies in the generated `fhir/llms.txt`, and one comment in
`scripts/check-published-match.sh`. No CI configuration referenced the
directories. Every `.md` link that names them was checked to resolve
**case-sensitively**, not merely on the case-insensitive filesystem the rename
was performed on — which is the check `AG1.4` exists to require.

One uppercase mention survives deliberately: `fhir/plan.md` records "the AGENTS
sweep" as the name of a task completed on 2026-08-07. It is a historical record
of what something was called, not a path, so `AG1.4` does not reach it.
