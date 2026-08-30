# Specification-driven development

How a change moves through this repository. The short version: **behaviour is
decided in `/spec` before it is written in Rust**, and the decision keeps a
number so that a test, a commit, and an auditor's workpaper can all point at the
same thing years later.

## The loop

```
spec/         decide what must be true, with a requirement id
  ↓
tasks.md      break it into work, per port, with an acceptance criterion
  ↓
code          implement
  ↓
test          prove it, and prove the test can fail
  ↓
matrix        record which ports now satisfy it
```

Skipping a step is allowed only backwards: discovering a requirement while
implementing is normal, and the fix is to write it into `/spec` before the
commit lands, not after.

## Requirement identifiers

Every normative statement has an id: `M3.16b`, `PR12.6`, `T11.12`, `L4`. The
prefix is fixed per section; the [core index](../spec/databases/index.md) lists
them, and [`spec/index.md`](../spec/index.md) lists every family's.

**Ids are permanent and never reused** (`C0.5`). This is the rule that costs
nothing to keep and cannot be repaired once broken:

- Withdrawing a requirement keeps its number, marked withdrawn.
- Amending a requirement keeps its number.
- Splitting one uses letter suffixes (`M3.16` → `M3.16a`, `M3.16b`); the parent
  survives.
- Adding one takes the **next unused ordinal** in the section. Never insert
  mid-sequence by shifting.

Section numbering has deliberate gaps at 7, 8, and 14 (`C0.6`). Do not close
them.

## Amending the core

1. Edit the file in [`/spec/databases`](../spec/databases/index.md). Once —
   there are no other copies (`W16.5`).
2. State the reason in the commit. If the amendment exists to match what a port
   already does, **say which port** (`C0.22`). A considered generalization and a
   rubber stamp are indistinguishable afterwards.
3. Check the [conformance matrix](../spec/databases/conformance-matrix.md): does the
   amendment change any port's status?
4. Check [`audit.md`](../spec/databases/audit.md): does it close a finding, or create one?

## Adding a dialect departure

A port that cannot satisfy a core requirement writes an `M14.x` in its annex.

```markdown
- **M14.7** `ords` MUST be stored as `TEXT` holding the array literal produced
  by `fmt_ords`, amending **M3.4**'s `smallint[]`. The three value-domain
  properties of `M3.4a` survive: negative ordinals appear verbatim, the empty
  path is the two-character string `{}`, and depth is unbounded because the
  encoding is variable-length.
```

What makes that a departure rather than prose:

- It **cites what it amends** by number (`X15.7`).
- It says what holds **instead**, not merely that the engine is different.
- It shows the invariants that must not move (`C0.13`) still hold.

What is not a departure: describing the engine, restating a core requirement
unchanged (`X15.8`), or implementing something different and mentioning it in a
comment. That last one is a defect (`C0.14`).

Every annex must cover the twelve-item `X15.6` checklist. "Not applicable" is an
acceptable answer; silence is not, because silence and not-having-considered-it
are the same on the page.

## Tasks

Each port has a `tasks.md` organized by milestone, with `[ ]`, `[~]`, `[x]`
markers and an acceptance criterion per task. A task should name the requirement
it satisfies, so the trace from regulation → requirement → task → test is
walkable in both directions. That trace is what §13 sells to an auditor and what
IEC 62304 asks for.

A cross-cutting task — one that applies to all six ports — is currently
duplicated into all six `tasks.md` files. That is the same duplication problem
`/spec` just solved, one level down; when it next causes trouble, hoist it.

## Recording what you did not verify

`T11.12`, `C0.9`, and rule 5 of [`AGENTS.md`](../AGENTS.md) all say the same
thing from different angles: **a gap that is not written down reads as a pass**.

- A test that self-skips without its inputs must say so, and must fail if it
  ends up checking nothing.
- A requirement implemented but untested is `?` in the matrix, not `•`.
- A requirement satisfied by shared code in a port with no test for it is also
  `?`. Sharing a correct implementation is not evidence that this port runs it.

The register of everything currently in that state is
[`spec/audit.md`](../spec/databases/audit.md). Add to it rather than carrying the
knowledge in your head or in a commit message nobody will re-read.

## Worked example: the accent fold

The change that widened the fold to reach `Ærø` shows the whole loop, including
the parts that are easy to skip.

1. **Spec.** The fold is normative in
   [`locale-accent-folding.md`](../spec/databases/locale-accent-folding.md). Widening it
   meant `L6` gained mappings — a change to `L4`/`L6` is by `L12` a **data
   migration**, not a code change.
2. **Consequence.** `L13`: a deployment that changes the fold must backfill
   `_norm` first. `O10.4a` generalizes it: a migration that changes stored
   derived values must ship a backfill.
3. **Reality, at the time this example was written (F-15 open).** Only
   `fhir-postgresql` had `backfill_norm`; four ports had no `upgrade` at all, so
   for them the migration was a full reload. **This is now historical**: F-15
   closed 2026-08-09 on the last of the six ports, and every port has both
   `upgrade` and `backfill_norm` today (`find . -name upgrade.rs -path
   '*/src/*'` and `grep -rl backfill_norm */crates/*/src/` each return all
   six). The step still matters as a worked example of checking reality rather
   than assuming it — just don't cite its numbers as current.
4. **Test.** `L16` requires mutation verification: disable the expansion, watch
   the test fail with `left: "ærø", right: "aero"`. It was, and two of the
   original test expectations turned out to be wrong before the code was.
5. **Record.** The gap is in `tasks.md` as T90a in all six ports, and in the
   audit register.

Step 3 is the one that would have been skipped, and it is the one where
deploying the fix without the backfill is **worse than not fixing the bug** —
stored values fold under the old rules, search terms under the new, and the
query matches neither.
