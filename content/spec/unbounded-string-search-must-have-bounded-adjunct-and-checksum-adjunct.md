# Unbounded string search: bounded adjunct and checksum adjunct

Normative rules for making an unbounded text column searchable on an engine that
cannot index or compare it as bound (`P6.4a`). Requirements are numbered `U<n>`
and use RFC 2119 keywords.

This is its own section for the same reason [locale and accent
folding](locale-accent-folding.md) is: it is one decision that several sections
depend on and no single section owns. It changes the **generated map**, so it is
upstream of every dialect — and the map is shared verbatim across all six ports
(`X15.1`), which makes it the widest-blast-radius change in the specification.

## The problem it exists to solve

A FHIR `string` has no length bound. The specification cannot declare one, and a
generator cannot infer one, so any column holding a FHIR string is unbounded by
construction. Engines disagree about what may then be done with it:

| Engine | Bound type | Can index? | Can `=` compare? |
|---|---|---|---|
| PostgreSQL | `text` | yes | yes |
| SQLite | `TEXT` | yes | yes |
| MySQL / MariaDB | `TEXT` | with a prefix length | yes |
| SQL Server | `NVARCHAR(MAX)` | **no** | yes |
| Oracle | `CLOB` | **no** | **no** |

The last row is the sharp one. On SQL Server an unindexable column still answers
`=`, so the affected searches are correct and merely scan. On Oracle a `CLOB`
answers neither, so the same design makes those searches **fail rather than slow
down** — and a search that returns an error is better than one that scans, but
both are worse than one that works.

`fhir-mssql` and `fhir-oracle` reached the same conclusion independently — in
`M14.16` and `M14.9` respectively — that the fix belongs in the generated map
rather than in `ddl.rs`. This section is that conclusion made normative.

## The two adjuncts

- **U1** A text column that a search **indexes or compares** MUST be given two
  generated adjunct columns in the relational map, wherever the target engine
  cannot index or compare the column as bound:

  | Adjunct | Type | Serves |
  |---|---|---|
  | `<col>_idx` | bounded character, binary collation | prefix, range, ordering |
  | `<col>_h` | fixed-width digest of the **full** value | equality, `:exact`, token match |

- **U2** **Both are required; neither substitutes for the other.** A bounded
  adjunct cannot answer equality, because two values agreeing in their first *n*
  characters are indistinguishable in it. A checksum adjunct cannot answer a
  prefix or a range, because a digest destroys order.

  A port emitting only the bounded adjunct has equality that silently returns
  the wrong rows. A port emitting only the checksum adjunct has no prefix search
  at all. Emitting one and calling the problem solved is the failure this
  requirement is written to prevent.

## What they are, and are not

- **U3** Both adjuncts are **derived**. The shredder MUST write them from the
  source column; the reconstructor MUST NOT read them.

  They are not part of the resource. It follows that they MUST NOT affect
  `R4.2` round-trip fidelity, and that `M3.16`'s hash-chain pre-image MUST NOT
  include them — a chain that committed to a derived column would break the
  moment the derivation changed, which is a migration concern the chain has no
  business carrying.

- **U4** The checksum MUST be computed in Rust, over the same canonical bytes
  the rest of the project uses (`X15.2`), and MUST NOT be computed by a SQL
  function.

  This is `L1`'s argument in a second place: two implementations of "the same
  string" — one in SQL, one in Rust — must agree for every codepoint in Unicode
  or the system quietly loses matches. One implementation cannot disagree with
  itself.

- **U5** The bounded adjunct MUST use the folded form where one exists
  (`P6.6`, `L2`), so that a prefix search over it is insensitive to case and
  accents exactly as a prefix search over `_norm` already is. An adjunct that
  folded differently from its source column would be a third definition of
  string identity.

## How a query uses them

- **U6** An equality predicate MUST match the checksum adjunct **and** confirm
  against the source column.

  A digest match alone is one collision away from returning another patient's
  record. The confirming comparison costs one row, and on Oracle — where the
  source column is a `CLOB` that cannot be `=` compared — the confirmation MUST
  use whatever comparison that engine does offer (`DBMS_LOB.COMPARE`), which is
  exactly the case that made the checksum necessary in the first place.

- **U7** A prefix predicate MUST use the bounded adjunct as a **filter** and
  then confirm against the source column, for the reason `P6.6a` gives about
  range predicates: the index narrows, the comparison decides.

- **U8** A search MUST NOT return a row that only the adjunct matched. Adjuncts
  are an access path, never an answer. A test asserting a search's results MUST
  therefore be written so that it fails if the confirmation step is removed —
  mutation-verified (`T11.10`), because a missing confirmation is invisible
  until two values collide.

## Which ports materialize them

- **U9** A port whose engine indexes and compares the bound type directly —
  PostgreSQL, SQLite, MySQL, MariaDB — MUST NOT emit either adjunct.

  The map records that an adjunct is *available* for a column; the dialect
  decides whether to materialize it. Emitting them everywhere would put two
  derived columns on every indexed text column in four ports that have no use
  for them, which is a storage and write-amplification cost paid for nothing.

- **U10** A port that materializes adjuncts MUST record in its annex which
  columns get them and what the bound *n* is, and MUST NOT claim `P6.4a` until
  it does.

## Why not the alternatives

Recorded because each was considered and each looks reasonable until examined.

**Declare a searchable-length limit.** Bind the compared columns to a bounded
type and document that longer values are not exactly-searchable. No map change,
no shared-core churn — and a permanent, silent functional gap in two ports, in
which a clinician searching for a long identifier gets no result and no error.
Rejected: `P6.4a` exists precisely to stop a port trading search correctness for
implementation convenience.

**Truncate the stored value.** Loses data, violates `R4.2`, and is not worth
further discussion.

**Overflow the value across a bounded column and a `CLOB` tail.** Preserves both
indexability and losslessness, and pushes reassembly into `shred.rs` and
`reconstruct.rs` — the two files where a bug corrupts resources rather than
merely failing to find them. The adjunct design keeps the source column whole
and adds beside it, which is the same benefit at a fraction of the risk.

---

Part of the [fhir-databases specification](index.md).
