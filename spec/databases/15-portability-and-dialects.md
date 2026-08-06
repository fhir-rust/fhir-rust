# 15. Portability and dialects

This section defines what makes six ports one product rather than six products
with a shared ancestor. It is the section that did not exist while the ports
were diverging, and its absence is why they did.

## The portable core

- **X15.1** The following MUST be identical across ports — not merely similar —
  after normalizing the crate-name substitution (`fhir_sqlite_map` ↔
  `fhir_postgresql_map`) and annex cross-reference numbers. They are pure Rust,
  operate on Rust types, and never emit SQL:

  | Module | What it is |
  | --- | --- |
  | `map/src/model.rs` | the relational map types |
  | `map/src/shred.rs` | JSON → rows |
  | `map/src/reconstruct.rs` | rows → JSON |
  | `map/src/value.rs` | leaf value representation |
  | `map/src/fold.rs` | the accent/case fold |
  | `map/src/canon.rs` | canonical JSON for the hash chain |
  | `map/src/error.rs` | shred/reconstruct errors |
  | `gen/src/**` | the generator |
  | `gen/tests/**` | the generator's tests |

  `gen/tests/**` is in scope for the same reason `gen/src/**` is — nothing in
  the generator is dialect-specific, so neither is anything that tests it. Six
  copies of a test that drift are six different claims about the same code. It
  was unwatched until **F-48**, and adding it found `fhir-postgresql` carrying a
  duplicated path candidate that a line-based comparison could not see.

  **X15.1a** The comparison MUST be made on a **token stream**, not on lines.
  rustfmt wraps at a column, so a longer crate name pushes a line over the limit
  and splits it; two ports then differ by whitespace alone, with no code
  difference and no way to reconcile them — the boundary was measured at exactly
  69/70 columns across all six. Normalizing the crate name cannot undo what the
  name's *length* already did to the layout. A gate that reports this is red for
  a reason nobody can fix, which is how a gate stops being read.

  Tokenizing MUST preserve punctuation, so that an operator change is still a
  divergence. It necessarily discards whitespace inside string literals; that is
  acceptable **only** for this file set, whose defining property is that it
  never emits SQL, and is a reason not to extend this comparison to `ddl.rs`.

  A divergence in any of them is a defect, and `W16.6` requires CI to detect it.
  The dialect surface is exactly two places: `map/src/ddl.rs` (which SQL the
  generator emits) and the `store` crate (which driver, which transaction, which
  placeholder syntax).

  As of this revision the requirement **holds** — every listed module is
  identical across all six ports under that normalization, and `canon.rs` is
  identical across the five that have it. It holds because the ports were forked
  recently, not because anything checks; that is what `W16.6` is for.

- **X15.2** The **canonical form** a history row's hash chain commits to MUST be
  computed in Rust, by one function shared between the writer and the verifier,
  and MUST NOT be delegated to the database.

  This is not only the security argument of `M3.16b`. It is a portability
  requirement with a hard consequence: a chain over whatever one engine's JSON
  type happened to produce — reordered keys, rewritten number spellings — is
  reproducible by that engine alone, so no other port could ever verify it and
  the format would not survive a port.

  The form:
  - Object keys sorted by UTF-8 byte order.
  - Numbers emitted as their **parsed lexeme**, not re-formatted.

  RFC 8785 (JCS) is the obvious standard and is wrong here: §3.2.2.3 serializes
  numbers as IEEE-754 doubles, turning `1.50` into `1.5` and losing everything
  past the seventeenth significant digit. That collides resources differing in a
  clinically meaningful decimal and violates the precision `M3.6` requires.

- **X15.3** Generated identifiers MUST be **the same names on every port**. The
  identifier budget (`G2.4`) and split width (`G2.6`) are therefore set to the
  tightest target, not to each engine's own limit. A schema that is name-for-name
  comparable across engines is what makes a cross-engine diff a meaningful
  test; per-engine name budgets would make every port's schema unique and every
  such comparison impossible.
- **X15.4** The fold (`P6.6`, [locale-accent-folding](locale-accent-folding.md))
  MUST be byte-identical across ports. It is pure Rust precisely so that no
  port depends on an engine extension, an engine's collation tables, or an
  engine's Unicode version. `fold("Ærø") == "aero"` on all six or the property
  is not a property.
- **X15.5** The stored image of `ords` MUST be the shared array literal
  (`M3.4b`) whatever type it is bound to, so a database written by one port can
  be compared value-for-value against another's.

## What a dialect annex must contain

- **X15.6** Every port MUST have a `spec/14-<engine>-dialect.md` annex, and it
  MUST address, explicitly and by name, each of the following. "Not applicable"
  is an acceptable answer; silence is not, because silence is indistinguishable
  from having not considered it.

  1. **Engine floor** (`S1.4`) — the minimum version, and the dialect fact that
     sets it.
  2. **Namespace mechanism** (`S1.2`) — how `r5`/`r4`/`r3` are isolated.
  3. **`ColTy` binding** (`M3.6`) — the full table, with the `Numeric`,
     `TextC`, and `Jsonb` choices justified against `M3.6a`–`M3.6c`.
  4. **`ords` binding** (`M3.4a`) — the type, and how the three value-domain
     properties survive it.
  5. **Install atomicity** (`G2.5`) — whether DDL is transactional, and what
     "effectively atomic" means here.
  6. **Snapshot isolation** (`R4.5`) — the level named, and any database-level
     setting it requires.
  7. **Write serialization** (`H5.4`) — the lock that orders `version_id`
     assignment and chain appends.
  8. **Append-only enforcement** (`M3.17`) — the trigger, or its absence.
  9. **Index limits** (`P6.4a`) — any column that cannot be indexed as bound,
     and what is done instead.
  10. **Paging and placeholders** — `LIMIT`/`OFFSET` versus `OFFSET … FETCH`,
      and the parameter placeholder syntax.
  11. **Transport security** (`O10.7`) — TLS modes, or why there is no
      connection.
  12. **Unmet core requirements** — every core requirement this port does not
      satisfy, as a departure (`C0.12`).

- **X15.7** A departure MUST cite the core requirement it amends by number and
  state what holds instead. Prose that merely describes the engine is not a
  departure and does not license one (`C0.14`).
- **X15.8** An annex MUST NOT restate core requirements it does not change.
  Restating is how the six copies came to exist; the annex is a diff, and a diff
  that includes unchanged lines is not one.
- **X15.9** An annex MUST carry a status — **proposed** or **ratified** — and a
  proposed annex MUST NOT be cited as evidence for a conformance level
  (`C0.9`). All six annexes are currently **proposed**, which is a fact worth
  seeing in one place rather than discovering per file.

## Cross-engine interoperability

- **X15.10** Two ports at Store level or above MUST agree on the **logical
  content** of a store: the same resource shredded by both produces the same
  logical rows under the same identifiers, and either port's reconstruction of
  the other's rows yields the same resource. Physical form differs by binding;
  logical content does not.
- **X15.11** A history chain written by one port MUST be verifiable by another,
  given the same key material. This follows from `X15.2` and is the single
  sharpest test of whether the canonicalization is genuinely shared — a chain is
  a fixed point that either reproduces or does not.

  This holds as of **F-07**: `fhir-postgresql` was the last port deriving its
  pre-image from `jsonb` and the only one without `map/src/canon.rs`. All six now
  share it, and `chain_portability.rs` recomputes a PostgreSQL chain from the
  exported row columns alone ([`audit.md`](audit.md) **F-07**, fixed).

- **X15.12** A **cross-engine conformance test** SHOULD exist: shred a corpus
  under two ports and diff the logical rows. `X15.1` is now tested and
  CI-gated — `scripts/check-shared-core.sh` compares 100 files token-wise
  across all six ports on every push (`gates.yml`; **F-10** fixed, **F-49**
  first half closed). `X15.4`, `X15.5`, and `X15.10` remain untested across
  ports, and the shred-a-corpus-under-two-ports diff this requirement asks
  for does not exist yet. *(An earlier revision said nothing tested `X15.1`
  either — stale since F-10 closed, **F-77**.)*

---

Part of the [fhir-databases specification](index.md).
