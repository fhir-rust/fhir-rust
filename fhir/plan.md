# Improvement plan — `fhir` (FHIR® in Rust)

Status: **rewritten 2026-08-06**. The previous revision was the original
2026-07-11 plan, kept unchanged while the work it planned shipped; by the time
it was retired, every present-tense statement in it was false — it described a
single unpublished 0.1 crate at `src/r5/` with 419 code enums, no CI, no XML,
no client, and R5 only. Companion file: [`tasks.md`](tasks.md) — T1–T36 record
how each planned task actually resolved, and Phase B (T37–T48) is the current
work list. The specification ([`spec/index.md`](spec/index.md)) is the source
of truth for what the crate is; this file only says where it is going.

## Where the plan landed

The 2026-07-11 plan ran to completion or was overtaken, phase by phase:

| Phase | Planned | Outcome |
| --- | --- | --- |
| 0 — infrastructure | CI, examples oracle, publish hygiene, proptest | Done, exceeded — 16-job CI incl. fuzzing, SBOM, package-size, corpus gate (7,400 examples, hard gate); published. Proptest, the last unfinished item, closed 2026-08-06 (T48). |
| 1 — primitive extensions | `_field` siblings | Done — shipped as `_ext` fields serde-renamed to `_<name>`, generator-emitted. |
| 2 — type safety | choice enums, coded enums, typed refs, value APIs | **Complete** as of 2026-08-09: choice enums, `Coded<E>`, temporal accessors (T12), and the full typed-`Reference<T>` rollout — machinery in every release and typed fields wherever `targetProfile` names one resource (T11). |
| 3 — validation | cardinality, invariants, OperationOutcome | Done, exceeded — 8 invariant classes vs the planned 3, `vec1::Vec1` for `1..*`, corpus-tested. |
| 4 — ergonomics | builders, prelude, bundle utils, typed `contained` | **Complete** — typed `contained` landed 2026-08-06 (T47, breaking → 4.0). |
| 5 — interop | client, XML, summary | Done, hardened past the plan (timeouts, percent-encoding, size caps, fuzzing — T29/T31 found and fixed a remote DoS in the XML reader). |
| 6 — multi-version | "R4B under `src/r4b/`" | Exceeded in substance, obsolete in form: **five releases (R2–R6) shipped as separate crates** — the workspace split is what made compiling them tractable (peak memory 12.9 GB → 5.0 GB). R4B itself remains future work (spec 12). |
| A — assurance | (added later) | T26–T36 all done: lexical `Decimal`, full-corpus CI gate, client hardening, fuzzing, supply chain, R5 drift audit, cross-release conversion (`fhir::convert`, the 3.0.0 headline). |

The current tree: `fhir` 3.0.0 (facade, features `r2`–`r6`/`client`/`xml`),
`fhir-core` 3.0.0, `fhir-r2`…`-6` 3.0.0, `fhir-derive-macros` 1.2.0,
and five reservation crates at 0.0.1 — 13 crates, all published.

## What remains

Tracked as discrete tasks in [`tasks.md`](tasks.md); the plan-level view:

1. ~~Drift found by the 2026-08-06 audit (Phase B, T37–T49)~~ — **all
   executed or closed by decision by 2026-08-07**: MSRV reconciled, the llms
   gate repaired and its artifacts regenerated, `forbid(unsafe_code)` in all
   13 crates, the AGENTS sweep, the changelog and identity strings, the
   profdata removal, the fhir-core doctest gate. The two owner decisions are
   recorded in place (llms.txt duplication is deliberate, T38; no book
   deploy, T46).
2. ~~The unfinished halves of old phases~~ — all closed: typed `contained`
   2026-08-06 (T47), the temporal accessors were done all along (T12), and
   the typed `Reference<T>` rollout — machinery and field emission —
   completed 2026-08-09 (T11). What remains from the old plan is only R4B.
3. ~~**R4B**~~ — **decided (a) and built, 2026-08-10**: `fhir-r4b`,
   feature `r4b`, module `fhir::r4b` (the crate family was renamed
   `fhir-release-N` → `fhir-rN` the same day, owner-directed, so the name
   landed as `fhir-r4b` rather than the drafted `fhir-release-4b`).
   1,035 crate tests, the 59-file curated subset, and the full-corpus gate
   green; the corpus surfaced audit **F-86**/**F-87**. The original
   decision draft, kept for the record: the only FHIR release published by
   HL7® and not modelled here. The generator and the adding-a-release procedure
   (`doc/adding-a-release.md`, budgeted at roughly an hour) are proven by
   five releases; the definitions bundle is one documented download
   (`hl7.org/fhir/R4B/definitions.json.zip` — not vendored yet). What blocks
   it is the **name decision**, drafted here for the owner:
   - *(a)* `fhir-r4b`, feature `r4b`, module `fhir::r4b` — breaks the
     numeric reservation scheme's pattern but says exactly what it is;
     crates.io accepts the name, and none of the reservation crates
     (`-1`, `-7`…`-10`) is disturbed. **Recommended**: the scheme exists to
     reserve future numbers, not to forbid HL7's own naming, and `4b` is
     what every FHIR implementer will search for.
   - *(b)* Skip R4B entirely, recorded: it is R4 plus a small delta
     (the `Medicinal*`→`…Definition` swap and a handful of resources), and
     `fhir::convert` already moves documents between the neighbours it
     shares. Costs nothing; leaves the one HL7-published gap standing.
   - Not an option: shoehorning it into a numeric slot (`fhir-r7` is
     a reservation for a future *R7*, and `C0.5`-style permanence applies to
     what the names promise).
4. ~~mdBook deploy~~ — closed by decision (T46, 2026-08-06): the website is
   the published documentation surface; the book stays a CI-checked,
   read-from-checkout artifact. (Proptest closed the same day, T48 —
   `tests/proptest_roundtrip.rs`.)

## Guiding constraints (unchanged where still true)

1. **Round-trip fidelity is sacred.** The corpus gate (7,400 examples, three
   releases, counted failure classes) is a hard CI gate; any representation
   change goes through it.
2. **Semver is real now.** The crate is 3.0.0 with published dependents;
   breaking changes are majors, batched, with migration notes. (The old
   "breaking changes are batched per minor since the crate is pre-1.0" is
   history.)
3. **Generator-first.** Mass model changes are made in `src/codegen/` and
   regenerated — `fhir-r2/-3/-4/-6/src` are generated trees;
   `fhir-r5/src` is hand-documented, protected by `tests/r5_drift.rs`
   (generate-and-compare with a sanctioned-differences table), and must never
   be blindly regenerated.
4. **Operational lesson (hard-won):** when fanning out parallel agents that
   edit files in the shared working tree, give them Read+Edit ONLY (no Bash);
   commit a protected baseline first. See
   `~/.claude/.../memory/parallel-file-edit-agents-no-bash.md`.

## Risks

| Risk | Mitigation |
| --- | --- |
| R5 hand-documented tree drifts from the generator again | `tests/r5_drift.rs` fails on any unsanctioned difference (T35/T36 closed the 18 that had accumulated) |
| A generator fix lands in some release crates and not others | regeneration is per-release and cheap; the drift test covers R5; corpus gates cover all |
| Metadata drift (the Phase B class) recurs | the audit pattern that caught it — verify every doc claim against the tree — is recorded in `tasks.md`; `bin/check-llms` (repaired, T38) and `scripts/check-published-match.sh` are the mechanical share |
| Compile time / memory regress with new releases | the crate-per-release split is the mitigation and is measured (12.9 → 5.0 GB); keep new releases in their own crates |
| `unsafe` enters via dependencies or macros unguarded | T39 rolls `forbid(unsafe_code)` to all crates; `cargo-deny` + SBOM in CI |

## Verification standard (every task)

Per [`CLAUDE.md`](CLAUDE.md): `cargo build --all-targets`, `cargo test`,
`cargo clippy --all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings"
cargo doc --no-deps` — and, when the generator, derive macros, `fhir-core`,
or any release crate is touched, the same with
`--features "r2 r3 r4 r6 xml client"`. The corpus gate must stay green.
Nontrivial features get a runnable example exercised end-to-end.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
