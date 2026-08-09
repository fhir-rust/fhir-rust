# Unbounded string search: bounded adjunct and checksum adjunct

Normative rules for making a **search-reachable column** searchable on an engine
that cannot index or compare it as bound (`P6.4a`). Requirements are numbered
`U<n>` and use RFC 2119 keywords.

**The filename is narrower than the section.** It says "unbounded string"
because that is the case the problem was found in, and the file is not renamed
because a citation must keep resolving (`C0.5` is about ids, and the same
argument applies to a path other documents link). `U1a` states the actual
trigger: any column a search reaches that the dialect cannot index or compare as
bound — unbounded character, binary large object, or a fixed-shape column no
per-type rule reaches. `U11` says which columns the generator must walk to find
them.

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

The Oracle row was measured on 2026-08-02 against Oracle AI Database 26ai Free
(23.26.2.0.0), not taken from documentation: `ORA-22848: cannot use CLOB type as
comparison key` and `ORA-02327: cannot create index on expression with data type
LOB`. It is the row this whole section exists for, so it is the one worth having
evidence for.

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
  | `<col>_h` | SHA-256 of the **full** value, stored as 32 binary bytes (`U4a`) | equality, `:exact`, token match |

- **U1a** *Generalizes `U1`.* The trigger is **not the FHIR type and not the SQL
  type**. It is the pair *(a search reaches this column, this dialect cannot
  index or compare it as bound)*. Wherever both hold, adjuncts are required —
  whatever the column is called and whatever it stores.

  `U1` was written from the `string` case because that is where the problem was
  found. Stating it as "a text column" made the rule read as though it were
  about FHIR strings, and three other classes of column meet the same condition:

  | Class | Example | Why it qualifies |
  |---|---|---|
  | Unbounded character | `CLOB`, `TEXT`, `NVARCHAR(MAX)` | the original case (`U1`) |
  | Binary large object | `BLOB`, `bytea`, `VARBINARY(MAX)` | a `BLOB` is no more comparable than a `CLOB`; on Oracle both fail with `ORA-22848` |
  | Fixed-shape table columns | `url`, `leaf`, `v_text` in the extension and deep tables | not `ColTy`-driven, so no per-type rule reaches them (`U11`, **F-46**) |
  | Bounded-but-unindexable | a column past the engine's index key limit | bounded is not the same as indexable |

  A rule that names a type will be wrong the next time a column of a different
  type meets the same condition, which is precisely how the extension tables
  were missed.

- **U2** **Both are required; neither substitutes for the other.** A bounded
  adjunct cannot answer equality, because two values agreeing in their first *n*
  characters are indistinguishable in it. A checksum adjunct cannot answer a
  prefix or a range, because a digest destroys order.

  A port emitting only the bounded adjunct has equality that silently returns
  the wrong rows. A port emitting only the checksum adjunct has no prefix search
  at all. Emitting one and calling the problem solved is the failure this
  requirement is written to prevent.

- **U2a** *Qualifies `U2`.* "Both" means **both of the operations the search
  actually performs on that column**, not both adjuncts unconditionally.

  The pairing in `U2` holds because a `string` search does prefix *and*
  equality. Where a column is only ever compared for equality — a binary blob, a
  digest, a token that has no meaningful prefix — the bounded adjunct answers a
  question nobody asks, and ordering the raw bytes of a JPEG is not a search
  anyone performs.

  So: a column a search compares for **equality** MUST have the checksum
  adjunct. A column a search compares by **prefix, range, or ordering** MUST
  have the bounded adjunct. A column subject to both MUST have both, which is
  every `string` target and is why `U2` reads as it does.

  This MUST NOT be read as licence to omit one because a port has not
  implemented that search yet. The test is what the **search parameter**
  requires, not what the port currently supports — omitting on those grounds is
  how `U2`'s failure arrives by a different road.

- **U2b** The map MUST record, per column, **which** adjuncts exist. A query
  builder that assumes a pairing `U2a` no longer guarantees would emit a
  predicate against a column that was never generated, and it would do so only
  for the search shapes nobody exercised.

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

- **U4a** The digest MUST be **SHA-256**, and MUST be stored as its **32 raw
  bytes** in a binary column — `BINARY(32)`, `RAW(32)`, `bytea`, `BLOB`, as the
  engine spells it. It MUST NOT be stored as hexadecimal text.

  Naming the algorithm makes the column comparable across ports and makes a
  change to it a visible migration rather than a silent one; `M3.16` already
  fixes SHA-256 for the hash chain, and a second digest function in the same
  system would be a second answer to "are these two values the same".

  Hexadecimal doubles the width of a column that exists to be compared and
  indexed, on exactly the engines that adopted it because they could not index
  the source. It also invites the comparison to be written against a rendering
  rather than a value: two encoders that disagree on case produce two texts for
  one digest, which is `L1`'s failure in a new place.

  The cost is real and is accepted: a binary column obliges every store to bind
  a byte-valued parameter, and per-port binding of a new value type is where
  **F-20** was found — booleans, integers, and dates silently dropped on one
  engine and panicking on two others. `T11.10` therefore applies with force
  here: a port that materializes this column MUST have a test that round-trips a
  digest through its driver and fails if the binding is wrong.

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

## Which columns are in scope

- **U11** The generator MUST consider **every column a search can reach**, not
  only the columns named by a `string` search parameter.

  Three sets are in scope, and the narrow reading — columns named by a `string`
  parameter — covers only part of the first:

  - columns reached by `token`, `reference`, and `uri` parameters, whose targets
    are compared for equality and therefore need the checksum adjunct under
    `U2a` even where no prefix search exists;
  - the **extension and deep tables** — `url`, `leaf`, `v_text` — which searches
    filter on and which no `ColTy` rule reaches, because their shape is fixed
    rather than derived from the map. A generator MUST reach them by whatever
    second path this costs; being awkward to enumerate is not an exemption.

    `path` is deliberately **not** in this set. It is a structural locator the
    store filters by exact value, and a port MAY bind it to an indexable type
    instead, which `U12` then requires in preference to adjuncts. Adding it to
    the map as bounded while the DDL emits it unbounded is the specific error
    **F-46** records — a map that misdescribes the schema is worse than one that
    omits the column, because omission is visible and a wrong type reads as
    authoritative.
  - any column a **dialect** chooses to bind to an unindexable type even though
    another port binds it to an indexable one. The requirement is per-dialect
    (`U9`), so the *set* of adjunct columns legitimately differs between ports.

  **U11a** Where the map and the DDL emitter can disagree about which adjunct
  columns exist — which is anywhere a table's shape is not map-driven — a port
  MUST carry a test asserting they agree, over every table of every resource.
  Neither side's own tests can catch this: each is self-consistent while the two
  contradict each other, and the contradiction only surfaces as a runtime error
  on a query path. The test MUST also fail if it inspects nothing, so that a
  dialect with no adjuncts does not turn it vacuous (`T11.12`).

  A port that materializes adjuncts MUST NOT claim `P6.4a` while a search-
  reachable column of an unindexable type has none.

- **U12** Where a fixed-shape column is **bounded in practice**, a port SHOULD
  bind it to an indexable type rather than give it adjuncts.

  Adjuncts are the answer to "this cannot be indexed or compared". They are not
  the answer to "this was declared larger than it needs to be". An extension
  `path` is a FHIR element path — bounded by the specification's own naming
  rules — so binding it to a bounded character type is both simpler and faster
  than binding it to a LOB and then adding two derived columns to reach it.

  Recording that judgement is `U10`'s job: a port states which columns got
  adjuncts and why the others did not need them.

- **U12a** **`path`'s bound is the map's to compute and record** (decided
  2026-08-09, **F-47** step 2; the physical adoption is F-47 steps 3–5).

  `path` is bounded because of how it is produced, and by nothing else: an
  attach path is a chain of JSON property names the map already knows, the
  chain is finite because type recursion spills to a `Deep` table rather
  than extending the path, and a nested extension grows the `leaf` column,
  never `path`. So the longest `path` a conformant shred can write is a
  fact of the release map, computable at generation time — and not large:
  measured across the bundled maps, the longest fully qualified element
  path (a `StructureDefinition.differential…` chain) is 131 characters in
  R4/R5 and 121 in R3, and the attach form the shredder actually writes is
  shorter still.

  Each release map MUST therefore record a **`path_bound`**: the length in
  characters of the longest attach path reachable in that map, rounded up
  to the next multiple of 64, and never below 128. Attach paths are ASCII —
  JSON property names from the FHIR specification — so characters, bytes,
  and UTF-16 code units agree, and each dialect may read the one recorded
  number in its own unit.

  Three consequences, each the reason for a clause:

  - **Recorded in the asset, not recomputed at the point of use** (`G2.2`):
    the DDL a port emits follows the asset it was handed, so two builds of
    one asset agree, and a future generator bug cannot silently narrow a
    deployed column — a wrong bound is visible in the asset diff before it
    is a schema change.
  - **Rounded with headroom**: the bound outlives releases. Rounding to a
    64-character step means a release whose longest path grows by a few
    characters changes nothing physically; only growth past the next step
    is a schema change at all — and that change is a *widening*, which an
    `upgrade` MAY apply additively. An `upgrade` MUST refuse to *narrow*
    `path` — a smaller recorded bound is a manual migration, the same
    refusal as any other type change.
  - **Enforced at the write path**: a shred that produces a path longer
    than the recorded bound MUST fail loudly rather than leave the outcome
    to the engine — one engine would truncate and another reject, and a
    truncated `path` reconstructs the wrong resource shape, an `L4`
    violation arriving through a column nobody searches.

  Which *type* the bound produces stays per-dialect (`U9`): an engine whose
  unbounded text type indexes and compares fine MAY keep it — the four
  ports on `TEXT` satisfy `U12` for `path` already. `fhir-mssql` binds
  `NVARCHAR(path_bound)` (`M14.37` in its annex) and `fhir-oracle`
  `VARCHAR2(path_bound CHAR)` (`M14.38` in its annex).

  **`v_kind` needs no map field.** Its values are exactly the four kind
  characters the shared core writes (`z`, `b`, `n`, `s` —
  `value.rs`, `LeafVal::cols`), so it binds to a one-character type on
  every port. Only `fhir-oracle`'s `CLOB` violates that today; the decided
  binding there is `VARCHAR2(1 CHAR)` — never `CHAR`, which pads
  (`M3.6b`).

- **U13** A column holding **opaque bytes** — `Attachment.data` and its kin —
  MUST NOT be given a bounded adjunct.

  `U2a` already implies it, and it is stated separately because the mistake is
  attractive: a bounded adjunct over binary looks like it enables a "starts
  with" search, and there is no such FHIR search. What it would actually enable
  is ordering by the first *n* bytes of an encoded payload, which is not a
  question about the resource. The checksum adjunct alone serves the one
  meaningful operation, equality — "is this the same attachment".

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
