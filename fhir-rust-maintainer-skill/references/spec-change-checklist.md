# Changing the spec: new requirement, amendment, or dialect departure

Full process lives in [`agents/spec-workflow.md`](../../agents/spec-workflow.md)
— this is the condensed action sequence. Applies to the database family
(`spec/databases/`); the model family (`fhir/spec/`) has its own ids
(`R1.x`–`R14.x`) but the same discipline.

## The loop, as steps

1. **Decide what must be true, in the spec, before writing Rust.** If you
   discover a requirement while implementing, that's normal — write it into
   `/spec` before the commit lands, not after the fact.
2. **Requirement ids are permanent** (`C0.5`):
   - Adding one: next unused ordinal in the section. Never insert
     mid-sequence by shifting existing numbers.
   - Amending one: keep the number.
   - Withdrawing one: keep the number, mark it withdrawn — do not delete.
   - Splitting one: letter suffixes (`M3.16` → `M3.16a`, `M3.16b`); the
     parent id survives.
   - Section gaps at 7, 8, and 14 are deliberate (`C0.6`) — do not close
     them by renumbering.
3. **Edit the core in one place.** `/spec/databases` is the only copy
   (`W16.5`). Never paste a section into a port's own directory.

## If this amends an existing requirement

4. State the reason in the commit. If the amendment exists to match what a
   port already does, **name the port** (`C0.22`) — a considered
   generalization and a rubber stamp read identically after the fact unless
   you say which.
5. Check the [conformance matrix](../../spec/databases/conformance-matrix.md):
   does this change any port's status?
6. Check [`spec/databases/audit.md`](../../spec/databases/audit.md): does the
   amendment close a finding, or create one?

## If a port can't meet a core requirement (dialect departure)

7. Write an `M14.x` entry in that port's `spec/14-<engine>-dialect.md`. It
   must, per `X15.7`:
   - **cite the requirement it amends, by number** (`M14.7 amends M3.4`
     style),
   - **state what holds instead**, not just "this engine is different,"
   - **show which invariants survive** (`C0.13`) — the departure changes the
     mechanism, not the guarantee.
8. What is *not* a valid departure: describing the engine without a
   requirement citation, restating a core requirement unchanged (`X15.8`),
   or implementing something different and only mentioning it in a code
   comment — that last one is a defect (`C0.14`), not documentation.
9. Every dialect annex must cover the full `X15.6` checklist (see
   `spec/databases/15-portability-and-dialects.md`). "Not applicable" is a
   fine answer to an item; silence on it is not — silence and
   not-having-considered-it look the same on the page.

## After the spec change

10. Break it into work in `tasks.md`, per port, with an acceptance criterion
    — but see [`pre-commit-checklist.md`](pre-commit-checklist.md) and
    `CLAUDE.md`'s tasks.md trap before trusting or writing a `[x]`.
11. Implement, test (and verify the test can fail — `T11.10`), and update the
    [conformance matrix](../../spec/databases/conformance-matrix.md) for any
    port whose status the change affects.
12. If this closes an audit finding, say so in the commit (`closes F-NN`);
    findings close when the underlying thing is fixed, not when the text
    describing them is rewritten.
