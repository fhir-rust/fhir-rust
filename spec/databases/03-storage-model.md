# 3. Storage model

The storage model is stated in terms of an **abstract column type** (`ColTy`)
rather than any engine's SQL. Each port binds the abstract types to its dialect
in one function (`ddl::col_sql`), and that binding is the only place engine
types appear. Everything above it — the map, the shredder, the reconstructor —
is engine-independent by construction.

### Base tables

- **M3.1** Every resource type gets a base table named for the resource
  (`r5.patient`). Its primary key is `id`, of type `Text`.
- **M3.2** Base-table system columns: `id Text PRIMARY KEY`,
  `version_id BigInt NOT NULL` (monotonic per resource, starts at 1),
  `last_updated Timestamptz NOT NULL`. `Resource.meta` is otherwise stored like
  any other element.
- **M3.3** Every scalar (non-repeating, primitive-typed) element of the resource
  becomes a typed column on the base table.

### Child tables

- **M3.4** Every **repeating** element becomes a child table. A child table
  carries:
  - `rid Text NOT NULL` — the root resource id, referencing the base table with
    `ON DELETE CASCADE` semantics,
  - `ords` — the 1-based index at each repeating ancestor crossing from the
    resource root down to and including this element (`{2,1}` = second parent
    instance, first child instance),
  - primary key `(rid, ords)`,
  - typed columns for every scalar element reachable without crossing another
    repeating element.

  The array form (rather than one ordinal column per level) is what lets
  recursive elements (`Questionnaire.item.item`, via `contentReference`) share
  one table at any depth: recursion appears as longer `ords` paths.

- **M3.4a** `ords` is a **path of ordinals**, not an engine array type. Its
  storage binding is a dialect concern (`X15.5`), and three properties of its
  value domain MUST survive whatever binding a port chooses:
  - **Negative ordinals are meaningful.** When two cyclic `contentReference`
    referrers share one table, the second pushes negated ordinals so paths stay
    unambiguous. The domain is `-32767..=-1 ∪ 1..=32767`; `0` never occurs. Any
    unsigned or magnitude-only encoding is wrong.
  - **The empty path is valid and frequent.** Resource-level extensions and
    element ids shred with `ords = {}` into a `NOT NULL` primary-key column.
    Empty MUST remain storable and distinguishable from null and from every
    other value.
  - **Depth is unbounded for recursive types.** Non-recursive tables reach depth
    6 at most in R5, but 23 R5 resource types (18 R4, 13 R3) own recursive
    tables whose depth is data-dependent and uncapped. A fixed-width encoding
    covers ~99.9% of tables and still fails.

  The database MUST NOT be required to order, compare, subscript, or unnest
  `ords`. It enforces uniqueness as part of a primary key and returns the value
  intact; everything else happens in Rust. That is what makes a text image an
  adequate binding on engines without an array type.

- **M3.4b** Whatever the binding, the **stored image** of `ords` MUST be the
  array literal produced by `fmt_ords` — `{1,2}`, `{}`, `{-1,3}` — so
  `fmt_ords`/`parse_ords` are shared unmodified and a database can be compared
  value-for-value against another port's.

- **M3.5** Non-repeating complex elements (datatypes and backbone elements)
  **flatten** into the nearest enclosing table as prefixed columns
  (`Patient.maritalStatus.text` → `patient.marital_status_text`); only their
  repeating descendants open tables. Two exceptions force a table for a
  non-repeating element, with a fixed ordinal of 1: (a) a flattened width that
  would approach the engine's column limit (generator threshold 150 columns,
  `G2.6` — this catches the open `value[x]` choices with ~54 types), and (b)
  backbone elements targeted cyclically by a `contentReference`
  (`ImplementationGuide.definition.page`). There are no shared "coding" tables;
  each usage site owns its rows.

### Type mapping

- **M3.6** FHIR primitive → abstract column type:

  | FHIR | `ColTy` |
  | --- | --- |
  | boolean | `Bool` |
  | integer, unsignedInt, positiveInt | `Int` |
  | integer64 (R5) | `BigInt` |
  | decimal | `Numeric` — original textual precision MUST survive round-trip |
  | string, code, id, markdown, uri, url, canonical, oid, uuid, xhtml, base64Binary | `Text` |
  | date | `Text` + derived `Date` column `<name>_sort` |
  | dateTime, instant | `Text` (verbatim) + derived `Timestamptz` column `<name>_sort` for ordering/search |
  | time | `Text` (fractional-second lexical fidelity) |

  Partial dates ("2026", "2026-07") make FHIR temporal values non-representable
  in native types without loss, hence verbatim text plus a derived sort column,
  computed by the engine at write time (partial values sort at their period
  start; offset-less dateTimes sort as UTC).

- **M3.6a** A port's binding of `Numeric` MUST NOT be a fixed-scale decimal or a
  binary float. Both lose the lexical form `M3.6` requires: `DECIMAL(65,30)`
  returns `1.50` as `1.500000000000000000000000000000`, and `REAL` cannot hold
  `1.50` distinctly from `1.5` at all. `Numeric` binds to a text type, and range
  search is served by a **derived sort column**, not by the storage column.

- **M3.6b** A port's binding of `TextC` MUST be a collation that is **binary and
  NO PAD**. Binary, because `TextC` backs `:exact` matching and key identity,
  and a case- or accent-insensitive default collation would silently turn exact
  equality into fuzzy equality. NO PAD, because under a PAD SPACE collation
  `'Smith' = 'Smith '` is true, which widens `:exact` and weakens primary keys.

- **M3.6c** A port's binding of `Jsonb` MUST NOT be a JSON-typed column on any
  engine that re-normalizes JSON on storage or retrieval. The history hash chain
  commits to bytes canonicalized in Rust (`X15.2`); a column that rewrites key
  order or number spelling would make the bytes read back differ from the bytes
  signed, and every chain would fail verification. Bind it to a text type.

- **M3.7** Elements bound `required` to a FHIR value set get a `CHECK (col IN
  (…))` constraint generated from the code system; other binding strengths are
  unconstrained columns. Where an engine does not enforce `CHECK`, the port MUST
  say so in its annex rather than emit a constraint that is accepted and
  ignored.

### Choice elements

- **M3.8** A choice element `value[x]` becomes one column (or child table, for
  complex types) per allowed type — `value_boolean`, `value_quantity_…` — plus a
  generated `CHECK` that at most one alternative is populated.

### References

- **M3.9** A Reference element stores: `<name>_ref_type Text`, `<name>_ref_id
  Text` (parsed from relative literal references), `<name>_ref_url Text`
  (absolute/other references, verbatim), plus columns for `display` and expanded
  `identifier`. Parsing MUST be reversible: the original `reference` string
  reconstructs exactly.
- **M3.10** Referential integrity across resources is NOT enforced by foreign
  keys (FHIR permits dangling references). A port MAY offer an advisory
  integrity report; it MUST NOT reject writes for dangling refs.

### Extensions and primitive extensions

- **M3.11** Extensions are stored relationally as **typed leaf rows** in one
  generated table per resource type: `<resource>_ext(rid, path, ords, modifier,
  ext_ord, url, leaf, v_kind, v_text, v_num, v_bool)`, PK `(rid, path, ords,
  modifier, ext_ord, leaf)`. `path`/`ords` locate the attach point (dotted
  JSON-name path, `""` for the resource itself; ordinals at each repeating
  crossing). `ext_ord` is the 1-based index in the extension array (`modifier`
  distinguishes modifierExtension); `url` is the top-level extension url,
  denormalized for querying. `leaf` addresses one scalar inside the extension's
  content as a dotted path whose all-digit segments are 0-based array indexes
  (`valueCodeableConcept.coding.0.code`); nested extensions are ordinary leaves
  (`extension.0.valueString`). `v_kind` ∈ s/n/b/z tags the JSON scalar kind;
  numbers keep their lexical form in `v_text` and a queryable numeric in
  `v_num`. This one uniform encoding covers every extension value type —
  including arbitrarily nested complex values — with no JSON and no per-type
  tables.
- **M3.12** Primitive extensions (`_birthDate` etc.) reuse `M3.11` with the
  primitive's path (and the entry index, for repeating primitives); element ids
  ride the same table as `ext_ord = 0, leaf = 'id'` rows. Reconstruction MUST
  re-emit the `_field` form exactly, including null padding in parallel arrays.
- **M3.13** `Resource.contained` resources are stored in a per-resource table
  `<resource>_contained(rid, ord, resource Jsonb)`. Elements typed `Resource`
  (`Bundle.entry.resource`, `Parameters.parameter.resource`) become `Jsonb`
  columns the same way. These are the sanctioned `Jsonb` usages besides history:
  such values are anonymous whole resources of unknowable type, so normalizing
  them buys nothing. Subject to `M3.6c`.
- **M3.14** The FHIR type graph contains one true datatype cycle:
  `Reference.identifier: Identifier` and `Identifier.assigner: Reference`.
  Static expansion cuts a cycle at the element that would re-enter an
  in-expansion type (`….identifier.assigner`), and stores anything below the cut
  as leaf rows (`M3.11` encoding, minus extension columns) in a per-resource
  `<resource>_deep(rid, path, ords, leaf, v_kind, v_text, v_num, v_bool)` table
  — lossless, relational, and vanishingly rare in real data.

### Audit columns

- **M3.15** Every `<resource>_history` table carries, besides `H5.1`'s columns,
  an **audit envelope**: `actor Text` (the authenticated principal responsible
  for the change, or `'unauthenticated'`), `actor_source Text` (how the
  principal was established, e.g. `header:X-Fhir-Principal`), `client Text`
  (source address as observed), `request_id Text` (the correlation id), and
  `reason Text` (a caller-supplied purpose of use, when given). These columns
  are written by the same statement that appends the history row, inside the
  same transaction as the data change — an audit record that can be lost
  independently of the change it describes is not an audit record.

- **M3.16** History is **tamper-evident**. Each history row carries `prev_hash`
  and, for each hash algorithm of `M3.16a`, a digest column over the row's
  canonical serialization (`X15.2`) concatenated with the previous version's
  digest for the same algorithm and resource id (the first version chains from
  that algorithm's length in zero bytes). Chains are per resource id, so appends
  stay concurrent. The `verify-audit` operation MUST recompute every chain in
  every algorithm and report the first break in each.

- **M3.16a** The chain MUST be computed under **at least two hash algorithms of
  different design families**, and MUST include **SHA-256** (Merkle–Damgård,
  FIPS 180-4) and **SHA3-256** (sponge, FIPS 202).

  The point is family diversity, not digest length. MD5 and SHA-1 both fell to
  the same line of cryptanalysis, and both are Merkle–Damgård; two digests drawn
  from one family would buy far less than their bit counts suggest. A clinical
  record may be retained for decades — longer than anyone can confidently
  promise a single hash function will stand — so the chain should not rest on
  one construction.

  Both named algorithms are FIPS-approved, so a strict regime is satisfied by
  either. Where one must be named going forward, name **SHA-3**: NIST published
  FIPS 202 precisely so that an approved hash would exist that is not a SHA-2
  variant.

  Verification MUST recompute every configured algorithm and report each
  separately rather than reducing them to a single verdict, so that a reader can
  rely on whichever algorithm their regime recognises.

  BLAKE3 (ARX tree) would add a third family and is deliberately **not**
  required: it is not FIPS-approved and MUST NOT be treated as the control of
  record where that matters.

- **M3.16b** Digests MUST be computed by the application, never by the database,
  and a deployment SHOULD additionally keep a **keyed tag**.

  The digests are unkeyed over a published pre-image, so anyone who can write to
  the database can also produce a correct digest for what they wrote. Computing
  them in SQL puts the means of forgery in the same place as the data, and
  forecloses the only real fix: a **MAC whose key the database never holds**. A
  key stored where the attacker already has write access protects nothing.

  Computing in the application also has a portability consequence that decided
  the matter independently: a digest over whatever one engine's JSON type
  happened to produce could never be verified by another engine, so the chain
  format would not survive a port. See `X15.2`.

  What the unkeyed chain buys, stated honestly so nobody over-claims it: it
  detects **careless or unaware modification** — a migration, a stray `UPDATE`,
  a row restored from the wrong backup — and it supports an **external witness**,
  because a chain head recorded off-box makes truncation and wholesale rewriting
  detectable even against an attacker who can recompute digests. It does not,
  alone, stop an informed attacker with write access.

  The keyed tag is `HMAC-SHA-256` (FIPS 198-1 over 180-4, so the FIPS story
  stays clean) over the same pre-image, stored as `<key-id>:<hex>`:

  - The key MUST NOT be written to the database, logged, or sent in a query, and
    MUST be at least 32 bytes: a placeholder like `changeme` reaching production
    would yield tags an attacker could reproduce by guessing.
  - A **file** SHOULD be the source, and MUST be supported. An environment
    variable is visible in `/proc/<pid>/environ`, survives into crash dumps, is
    reported by orchestrators, and is inherited by every child process; a file is
    none of those, and is what Kubernetes secrets and systemd credentials already
    produce. A key file readable by group or other MUST be **refused**, not
    warned about — a warning is read once at startup while the file stays
    readable for the life of the deployment.
  - Key material SHOULD be zeroed when dropped. Freed memory is not scrubbed, so
    a key otherwise lingers in the heap and is recoverable from a core dump.
  - A retired key that cannot be read MUST be an error, never a silent omission.
    Dropping one turns its rows *unverifiable*, and an operator who did not
    intend that should learn it at startup rather than from an audit.
  - Key configuration MUST apply to every operation that reads history, not only
    to a service. Verification without the key reports every keyed row as
    unverifiable, which is correct and useless.
  - The key id MUST travel with the tag. Without it, rotating a key would
    invalidate every historical row at once — indistinguishable from mass
    tampering. Retired keys MUST stay loadable, so rotation is additive rather
    than a flag day.
  - Verification MUST be constant-time. A timing oracle would let an attacker
    with write access recover a valid tag byte by byte without ever holding the
    key.
  - **Only a tag mismatch is a finding.** A missing tag, a tag naming a key this
    process does not hold, and a malformed tag MUST each be reported as what
    they are and MUST NOT be reported as tampering. Reporting a key-distribution
    problem as a forgery would burn an incident response.

- **M3.16c** A port MUST be able to emit a **chain checkpoint**: a single value
  covering every chain head in the namespace — resource type, id, latest
  version, and its digests — such that the value changes if any chain gains a
  version, loses one, or has its head altered. The `chain-witness` operation
  prints it, and it MUST be keyed when a key is configured, so that whoever
  holds only the data cannot recompute a matching value.

  This is what the per-row tag cannot do. A MAC proves a row was not rewritten;
  it says nothing about a row that is **gone**, and a chain missing its most
  recent version verifies perfectly, because nothing left behind refers to what
  was removed. Only a value recorded outside the database closes that gap.

  Checkpoints are also emitted as **INFO log lines on an `audit_checkpoint`
  target**, so a deployment already shipping logs has a witness for free. The
  dedicated target is what makes this practical: an operator can route and
  retain `audit_checkpoint` on its own schedule without keeping every other
  line, and the checkpoint carries no PHI — only counts and digests — so it may
  be retained far longer than ordinary application logs, and in places patient
  data must not go.

  A checkpoint MUST be emitted at startup and after an erasure (`M3.18`), and
  SHOULD be emitted on an interval a deployment configures. Erasure is singled
  out because it is the one sanctioned deletion: a checkpoint taken immediately
  after it separates a recorded, intentional removal from the unrecorded kind.

  The value is only a witness if it lands somewhere the database cannot reach.
  Logs shipped off-host qualify; logs written to a table in the same database,
  or to a disk the same compromised account can rewrite, do not. A port cannot
  enforce this and MUST NOT imply it has: the guarantee is a property of the
  deployment's log path, and the documentation MUST say so.

- **M3.16d** A key that can no longer be trusted MUST be retirable without
  losing the history it signed. The `chain-resign` operation counter-signs every
  history row under the current key.

  This is only for a suspected compromise. Ordinary rotation is additive
  (`M3.16b`): keep the old key loadable and nothing needs re-signing.

  - Re-signing MUST verify every chain first and MUST abort entirely on any
    finding. Re-signing rows that do not currently verify would give forged
    history the new key's authority, which turns the recovery procedure into the
    attack. It MUST be one transaction, so a partial re-signing cannot be left
    behind.
  - Counter-signatures MUST be **appended**, never written over the original
    tag. History is append-only, and re-signing in place would be the
    application doing what the append-only guard exists to prevent. The original
    tag is also evidence: replacing it destroys the record of what the retired
    key attested and leaves no way to tell a legitimate re-signing from a forged
    one.
  - A counter-signature MAY stand in for an original tag only where that tag
    **cannot be checked** — absent, or naming a key no longer held. A row whose
    own tag *mismatches* MUST remain a finding whatever later vouched for it.
  - Each counter-signature MUST record who ran it, when, and why.

- **M3.16e** A port MUST be able to generate a signing key (`chain-key-new`),
  creating the file readable only by its owner from the moment it exists.

  The shell equivalent, `openssl rand -hex 32 > key`, applies the process umask
  — commonly `022`, giving a file `M3.16b` requires be refused — and leaves the
  secret world-readable in the window before `chmod`. Generation MUST refuse to
  overwrite: silently replacing a signing key would orphan every row it had
  signed. The key MUST NOT be printed, since a secret echoed to a terminal lives
  on in scrollback and shell history.

  On an existing install, `init --upgrade` adds the new digest columns but MUST
  NOT backfill them. The rows are recoverable and the digests could be computed,
  but a chain assembled after the fact attests only that the rows look
  consistent *now* — which is exactly what an attacker who rewrote them would
  also produce. `verify-audit` MUST therefore report the new chain as beginning
  where its first digest appears, the same treatment rows predating the audit
  columns already receive. Manufacturing evidence is worse than admitting its
  absence.

- **M3.17** History is **append-only in the database, not merely by
  convention**. `init` MUST emit a `BEFORE UPDATE OR DELETE` trigger (or the
  engine's nearest equivalent) on every history table that raises an exception,
  and the documentation MUST describe the `REVOKE UPDATE, DELETE` grants a
  deployment applies to the application role. Escaping this is then a deliberate
  DBA act, never an application bug.

  Where an engine cannot express the trigger, the port MUST say so in its annex
  and MUST NOT let the grant documentation stand in for the missing control —
  a revocable grant and an in-database guard fail differently, and the whole
  point is defence that survives an application bug.

- **M3.18** Erasure (GDPR Art. 17) is the one sanctioned exception, and it is
  explicit: `purge <Type> <id> --reason <text>` removes the resource's history
  rows and replaces them with a single tombstone row recording who purged what,
  when, why, and the chain it terminated — so an erased record leaves a
  verifiable hole rather than a silent one. Purge requires an explicit erasure
  acknowledgement and is logged at warn level.

## Engine bindings for `ColTy`

Non-normative summary of each port's `ddl::col_sql`; the annexes govern.

| `ColTy` | PostgreSQL | SQLite | MySQL | MariaDB | SQL Server | Oracle |
| --- | --- | --- | --- | --- | --- | --- |
| `Bool` | `boolean` | `INTEGER` | `TINYINT(1)` | `TINYINT(1)` | `BIT` | `NUMBER(1)` |
| `Int` | `integer` | `INTEGER` | `INT` | `INT` | `INT` | `NUMBER(10)` |
| `BigInt` | `bigint` | `INTEGER` | `BIGINT` | `BIGINT` | `BIGINT` | `NUMBER(19)` |
| `Numeric` | `numeric` | `TEXT` | `TEXT` | `TEXT` | `NVARCHAR(MAX)` | `VARCHAR2(64 CHAR)` |
| `Text` | `text` | `TEXT` | `TEXT` | `TEXT` | `NVARCHAR(MAX)` | `CLOB` |
| `TextC` | `text COLLATE "C"` | `TEXT COLLATE BINARY` | `TEXT COLLATE utf8mb4_0900_bin` | `TEXT COLLATE utf8mb4_nopad_bin` | `NVARCHAR(450) COLLATE Latin1_General_100_BIN2` | `VARCHAR2(450 CHAR)` |
| `Date` | `date` | `TEXT` (ISO-8601) | `DATE` | `DATE` | `DATE` | `DATE` |
| `Timestamptz` | `timestamptz` | `TEXT` (ISO-8601 UTC) | `DATETIME(6)` | `DATETIME(6)` | `DATETIME2(6)` | `TIMESTAMP(6)` |
| `Jsonb` | `jsonb` | `TEXT` | `LONGTEXT` | `LONGTEXT` | `NVARCHAR(MAX)` | `CLOB` |

The Oracle column was all `⚠` until 2026-08-03: `fhir-oracle`'s `col_sql` was
a verbatim copy of MySQL's, the port was Scaffold level, and the DDL it
generated was not an Oracle schema. That was **F-08**, now fixed — every
binding above has been executed against a live Oracle 26ai Free and read back
from `user_tab_columns` (`M14.23d`), and the port is Store level (**F-68**).
An earlier revision of this section still described the pre-F-08 state in the
present tense (**F-77**). `Numeric` binds `VARCHAR2`, not `NUMBER`, because
`NUMBER` normalizes `1.50` to `1.5` (`M3.6a`, `M14.7`); `Text` binds `CLOB`,
which can be neither indexed nor `=`-compared — the case the `U1`–`U10`
adjunct channel exists to answer.

PostgreSQL's `jsonb` binding is the one row that violates `M3.6c`, and the
consequence *was* live rather than theoretical: `fhir-postgresql` used to
derive the chain pre-image with `(($1::text)::jsonb)::text`, so its canonical
bytes were whatever `jsonb` produced when it reordered keys and rewrote
number spellings.
`M3.16b` had moved the *digest* into Rust there; `X15.2` moves the *canonical
form* too. `canon.rs` is now present in all six `map/src` directories and
`fhir-postgresql` derives its pre-image from it, so a PostgreSQL chain can be
verified by any port holding that file — proven by `chain_portability.rs`, which
recomputes one from the exported rows alone. Closing this was a **chain format
change**: a database written before it must be reloaded. Tracked as
[`audit.md`](audit.md) **F-07**, fixed.

---

Part of the [fhir-databases specification](index.md).
