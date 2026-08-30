---
name: fhir-rust-maintainer-skill
description: Technical implementation skill for maintainers and agents working on this repository's code, spec, or docs — checklists and exact commands for shared-core changes across the six database ports, spec-driven changes, verifying a claim before making it, and pre-commit/pre-push checks. Use when the task is contributing to, fixing, or reviewing this repo, as opposed to explaining FHIR® concepts to an end user (that's fhir-skill).
---

# Maintaining fhir-rust

This skill is the **task-oriented layer** on top of the canonical docs — it
does not restate them, because restating them is exactly how this repository
got README claims, `tasks.md` checkboxes, and dialect annexes that drifted
from the truth (see the traps section of [`CLAUDE.md`](../CLAUDE.md)). Read
the canonical files; use this skill for the checklists and commands that
turn their rules into steps.

**Read first, in this order:**

1. [`AGENTS.md`](../AGENTS.md) — the five rules, the four families, the
   commit conventions.
2. The one file in [`agents/`](../agents/index.md) that matches your change
   (`spec-workflow.md`, `rust.md`, `testing.md`, `databases.md`,
   `documentation.md`, `security.md`, `release.md`).
3. [`CLAUDE.md`](../CLAUDE.md) — repo-specific traps: the F-numbers, the
   `R4.x` collision, and why `tasks.md`/README claims are not trustworthy on
   their own.
4. If a database port is involved: [`spec/databases/index.md`](../spec/databases/index.md)
   and that port's own `spec/14-<engine>-dialect.md`.
5. If `fhir/` is involved: [`fhir/spec/index.md`](../fhir/spec/index.md) and
   [`fhir/AGENTS.md`](../fhir/AGENTS.md) instead of the database rules above.

## Which checklist applies

| Your change touches | Use |
| --- | --- |
| `shred.rs`, `reconstruct.rs`, `fold.rs`, `canon.rs`, `model.rs`, `value.rs`, `error.rs`, or anything under `gen/src`/`gen/tests` in a port | [`references/shared-core-checklist.md`](references/shared-core-checklist.md) |
| A spec requirement (new, amended, or a dialect departure) | [`references/spec-change-checklist.md`](references/spec-change-checklist.md) |
| A claim you're about to write down ("this works", "this port supports X") | [`references/verification-commands.md`](references/verification-commands.md) |
| Documentation (README, book, `doc/`, `tasks.md`) | [`agents/documentation.md`](../agents/documentation.md) directly — its substitution trap (F-01/F-08/F-16) is the whole point and is already concrete |
| You're about to commit or push | [`references/pre-commit-checklist.md`](references/pre-commit-checklist.md) |

## The rules this skill exists to enforce

From `AGENTS.md`, condensed to what to *do*:

1. Normative text lives once, at `/spec/databases`. Never copy a section into
   a port.
2. A change to the shared core is **one commit across all six ports**, not
   six commits and not a partial rollout.
3. A dialect difference is a numbered `M14.x` departure in that port's annex,
   not silent divergence.
4. Never claim a conformance level the port hasn't earned — check the
   [conformance matrix](../spec/databases/conformance-matrix.md), don't
   infer it from the code looking plausible.
5. State what you didn't verify — a skipped test, an unset DSN, an untried
   engine — in the commit message, and in `spec/databases/audit.md` if it
   persists.

## Two traps worth carrying into every session

- **`grep` finds the same string six times.** Scope a search to one port, or
  expect sixfold results and dedupe mentally before treating "found in N
  places" as N findings.
- **`R4.x` is two different requirement sets.** It exists in both
  `fhir/spec/04-resources.md` and
  `spec/databases/04-shredding-and-reconstruction.md`. Resolve a bare
  citation by which file it's in; write new citations qualified
  (`db:R4.2`, `model:R4.2`).

## Scope discipline

A one-line fix in a shared-core file is never just one line: it's six
identical edits, a check against whether it changes `L4`/`L6` (a data
migration), a backfill story per port (dialect differs, but all six ports
have `upgrade`/`backfill_norm` as of F-15), and a mutation-verified test.
Say this before starting work, not after discovering it partway through —
see [`references/shared-core-checklist.md`](references/shared-core-checklist.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
