# Changing the shared core

Applies to `shred.rs`, `reconstruct.rs`, `fold.rs`, `canon.rs`, `model.rs`,
`value.rs`, `error.rs`, and everything under `gen/src` and `gen/tests` in any
`fhir-<engine>` port. These files are identical across all six ports modulo
crate name and rustfmt's column-width wrapping (`X15.1`, `X15.1a`). Editing
one without the other five is a divergence, not a fix — full background in
`AGENTS.md` rule 2 and `spec/databases/15-portability-and-dialects.md`.

## Before starting

1. Say out loud (in the PR/commit description, or to whoever asked) that this
   is a shared-core change: it will be six identical edits, not one.
2. Check whether the change touches `L4` or `L6` (the spec's data-format
   requirements). If it does, it's a **data migration**, not just a code
   change — see `L12`, `O10.4a`, and plan a backfill story per port, not just
   the code.
3. Confirm the baseline is clean before you touch anything:
   ```sh
   ./scripts/check-shared-core.sh
   ```
   A pre-existing divergence is not yours to silently absorb into this
   change — flag it separately.

## Making the change

4. Edit the file in **all six ports**:
   `fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`,
   `fhir-mssql`, `fhir-oracle`. Same logic, same structure — only the
   crate-name substitution and rustfmt wrapping may differ.
5. If a port's engine genuinely can't do what the new code assumes, that is
   an `M14.x` dialect departure in that port's `spec/14-<engine>-dialect.md`,
   not a silently different implementation of the "shared" file. See
   [`spec-change-checklist.md`](spec-change-checklist.md).
6. Re-run the gate, this time to confirm:
   ```sh
   ./scripts/check-shared-core.sh --diff
   ```
   It compares tokens, not lines, specifically so crate-name-driven rewrap
   doesn't false-positive. A remaining diff after `--diff` is a real
   divergence — resolve it, don't suppress it.

## Migration and tests

7. Every port already has `upgrade`/`backfill_norm` (closed **F-15**, all
   six as of 2026-08-09) — the question for a data-shape change is each
   port's *dialect story* for the migration, not whether one exists:
   - one transaction on `mssql`
   - resumable/rerunnable on `oracle`
   - reported-partial on `mysql`/`mariadb`

   Write or extend the migration path per port accordingly; don't assume the
   postgres story generalizes.
8. Add a test, and **verify it can fail** (`T11.10`, `L16`) — comment out or
   invert the fix locally, confirm the test goes red, then restore it. A
   test that can't fail is worse than no test: it reads as coverage that
   isn't there.
9. Run the exemption check: `./scripts/check-shared-core.sh`'s `EXEMPT` list
   must stay empty. If you believe a new exemption is warranted, it needs
   either a cited finding (like `F-07`, the postgres `canon.rs` gap that was
   closed rather than exempted) or an explicit `M14.x` departure — not a
   quiet addition to the list.

## Before committing

- One commit for all six ports (`AGENTS.md` rule 2) — not six commits, and
  not five now plus one later.
- Reference the requirement id(s) affected and, if this closes or creates an
  audit finding, say so (`closes F-NN`).
- If any port was left out deliberately (e.g. blocked on something), say
  that explicitly rather than letting the omission look like an oversight.
