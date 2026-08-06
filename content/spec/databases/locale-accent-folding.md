# Locale and accent folding

Normative rules for the fold that backs case- and accent-insensitive string
search (P6.6). Requirements are numbered `L<n>` and use RFC 2119 keywords.

This is its own section because the fold is the single definition of "the same
string" in the system. Every `:contains`, `:text`, and default string match
compares folded values; only `:exact` does not. A change here silently changes
which patients a clinician can find, which is why the rules are written down
rather than left to read off the implementation.

It is also, by `X15.4`, byte-identical across every port. Because `L1` puts the
fold in Rust rather than in SQL, no engine's collation tables, Unicode version,
or extension availability enters into it, and `fold("Ærø") == "aero"` on all
six by construction rather than by coincidence. That is the strongest form the
portability requirement takes anywhere in this spec, and it is free.

References to PostgreSQL's `unaccent` below are historical rationale — the
design this replaced — not a dependency.

## Where folding happens

- **L1** Folding MUST be performed in Rust, once, at write time, into the
  companion `_norm` column. It MUST NOT be performed in SQL.

  Two implementations of a fold — one in SQL for the write path, one in Rust for
  the query path — must agree for every codepoint in Unicode or the system
  quietly loses matches. One implementation cannot disagree with itself.

- **L2** A query MUST fold its search term with the same function that produced
  the stored value, and compare against the `_norm` column.

- **L3** The engine MUST NOT emit a SQL folding function. An earlier design used
  PostgreSQL's `unaccent`, which needed an `IMMUTABLE` wrapper, an expression
  index the planner would not use against a bound parameter, and a
  deployment-time check that the extension was installable at all. None of that
  is needed, and the function it emitted was never called.

## The algorithm

- **L4** `fold` MUST apply these steps, in order:

  1. Decompose to NFD.
  2. Drop combining marks.
  3. Lowercase, locale-independently.
  4. Decompose and drop combining marks again.
  5. Expand the letters in L6.

  Step 4 is not redundant. Lowercasing can *introduce* a mark: `İ` (U+0130)
  lowercases to `i` followed by a combining dot above.

- **L5** `fold` MUST be idempotent: `fold(fold(s)) == fold(s)`.

  This is load-bearing rather than tidy. The stored value is folded at write
  time and the search term is folded again at query time; a fold that changed on
  a second pass would stop matching its own output. Nothing produced by L6 is
  itself foldable, which is what keeps this true.

## Letters decomposition cannot reach

- **L6** After mark-stripping, `fold` MUST expand the following. These are
  single codepoints carrying a stroke, a bar, or a ligature; they have no
  canonical decomposition, so steps 1–4 leave them untouched.

  | | | | |
  |---|---|---|---|
  | `æ` → `ae` | `œ` → `oe` | `ø` → `o` | `đ` → `d` |
  | `ð` → `d` | `ł` → `l` | `ß` → `ss` | `þ` → `th` |
  | `ħ` → `h` | `ŋ` → `n` | `ŧ` → `t` | `ĸ` → `k` |
  | `ı` → `i` | | | |

  The mappings follow PostgreSQL's `unaccent` rules, so that a folded value
  means the same thing whichever engine stores it.

  Without this step a search for `aero` does not find `Ærø` — which is one of
  the names P6.6 gives as its reason for existing. `å` was never affected,
  because it is `a` plus a combining ring and step 2 handles it; the distinction
  between the two cases is exactly why L6 is separate from L4.

- **L7** Expansions MAY be multi-character. A fold restricted to substituting
  one character for one character cannot express `æ` → `ae`, and a fold that
  dropped the second character instead would make `Ærø` and `Ørø` the same
  string.

## What the fold deliberately does not do

- **L8** The fold MUST NOT transliterate between scripts. Greek and Cyrillic
  fold their combining marks like any other script — `ό` → `ο`, `й` → `и`, which
  is accent-insensitive search working consistently — but romanising them would
  make "the same string" a property of a romanisation policy rather than of the
  text. CJK has no marks to strip and MUST pass through unchanged.

- **L9** Lowercasing MUST be locale-independent. A locale-sensitive fold would
  make stored values depend on the server's locale, so the same database would
  answer differently after a configuration change — and `_norm` values written
  under one locale would silently stop matching terms folded under another.

  The visible consequence is Turkish: `İ` folds to `i` rather than to the
  dotless `ı` that Turkish casing rules would give. This is a deliberate
  trade, and it is the same trade every locale-independent index makes.

- **L10** The fold MUST NOT be treated as reversible or as a collation. It
  defines an equivalence class for matching. Ordering is a separate concern,
  handled by declaring the column with a binary, codepoint-ordered collation so
  that the prefix range scan in P6.6 is sound.

- **L11** `fold` MUST NOT be applied to `:exact`. That modifier is defined as
  the literal string, so it compares the stored column rather than `_norm`.

## Changing the fold

- **L12** Any change to L4 or L6 is a **data migration**, not a code change. It
  alters stored `_norm` values, so a database written before the change holds
  values folded under the old rules.

- **L13** A deployment that changes the fold MUST backfill `_norm` before
  serving searches against affected data.

  Deploying a widened fold without backfilling is **worse than not changing it
  at all**: stored values carry the old folding and search terms the new one, so
  a query matches neither the old spelling nor the new. The failure is silent —
  no error, just a patient who cannot be found.

- **L14** The backfill MUST fold distinct *values* rather than rows, in bounded
  batches, and MUST be resumable, since an interrupted run on a large dataset
  otherwise leaves a partially-folded column with no way to tell which rows were
  done.

## Testing

- **L15** The conformance suite MUST include, at minimum: a name whose folding
  depends on L6 (`Ærø`), one that depends only on L4 (`Muñoz`), a case proving
  idempotence under L5, and a case proving no transliteration under L8.

- **L16** Tests asserting the fold MUST be verified by mutation — disabling the
  expansion and confirming the test fails. A folding test that passes with the
  folding removed is asserting nothing, and this has happened here.

---

Part of the [fhir-databases specification](index.md).
