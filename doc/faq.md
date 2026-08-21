# FAQ

## Why not just store FHIR as JSONB?

Because JSON storage makes writing easy and querying painful, and a clinical
system does far more querying than writing.

With normalized storage you get:

- **Integrity the database enforces** — enum columns from FHIR value sets,
  `CHECK` constraints on choice elements, typed dates and decimals.
- **SQL that reads like the domain** — `SELECT family FROM r5.patient_name`
  rather than `->>'…'` path spelunking, and a planner that sees real column
  statistics rather than guessing at a document.
- **Search that is just SQL** — parameters compile to indexed predicates on
  ordinary columns.

The trade is real and it is deliberate: writes are more work, and there are
thousands of tables. For live queryable clinical data the trade is right.

JSON does survive in exactly three places, each for a stated reason: history
rows, contained resources, and elements typed `Resource`. See
[the storage model](storage-model.md#the-three-json-exceptions).

## Seven thousand tables? Really?

7,355 for R5. Every repeating element of every resource type gets one.

That is fine for a database and impossible for a human, which is why
**everything is generated** — the DDL, the relational map, the search
predicates, and the path-to-identifier index. Nobody writes or maintains a table
definition by hand, and nobody is expected to know the names; you look them up
in the generated index or read them off the element path.

Installing them takes about 9.5 seconds on PostgreSQL 18.

## Is this a FHIR server?

**The database ports are not; the repository has one.**

Each port is a library. Every port workspace contains three crates — `-map`,
`-gen`, `-store` — with no server crate and no CLI crate (`C0.17`, `C0.18`).
Sections 7 (REST API) and 8 (CLI) of the database specification are retired for
that reason (`C0.15`): they are out of scope *for the ports*.

The REST server is [`fhir-loco`](../fhir-loco/) — Loco.rs, Axum, Tokio, Hyper —
a separate crate that mounts a FHIR API over a store. It serves `metadata`,
search, create, read, update, delete, `_history` and `vread`, and requires a
PASETO v4.public token on every request.

That split is deliberate: a program that wants FHIR storage should not also
acquire a web framework.

**This answer used to be a flat "No."** It was written when nothing here served
HTTP, and it stayed that way after `fhir-loco` arrived — so a reader asking the
title question about *the repository* got the wrong answer. Corrected under
[audit](../spec/databases/audit.md) **F-63**.

Historical note: every per-port README documented `fhir-<engine> serve` and
`cargo install --path crates/fhir-<engine>` until 2026-07-31. Neither ever
worked — that crate has never existed in any port (**F-01**, fixed). The books
described the same fiction until 2026-08-03 (**F-56**, fixed); they now
attribute every endpoint to `fhir-loco`.

## Can I use it in production?

`fhir-postgresql`: it is the reference port, its full test suite runs against
live PostgreSQL 18 in CI, and its measured results are its own. It has one open
high-severity defect (**F-07**, chain portability) that does not affect
correctness within the port.

`fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`: working stores, and their
concurrency, redaction, and round-trip suites now exist and run green against
live engines (an earlier revision of this answer predated them — writing
those suites found five real defects, **F-20**–**F-24**). The remaining `?`
cells are narrower — see the
[conformance matrix](../spec/databases/conformance-matrix.md).

`fhir-mssql`: a working store, live-verified (**F-65**) — weigh the TLS
advisory risk (**F-67**) first. `fhir-oracle`: a working store (**F-68**),
but `R4.5` snapshot reads are a confirmed open gap and it has no concurrency
or redaction tests — not yet, for patient data.

In every case you also need the perimeter — authentication, authorization,
consent, TLS. See [the trust boundary](trust-boundary.md).

## Why is my search not matching?

Nine times in ten, fold both sides by hand:

```rust
use fhir_sqlite_map::fold::fold;
println!("{} vs {}", fold("Ærø"), fold("aero"));
```

String search compares folded values (`P6.6`). If those two do not agree, the
stored `_norm` value may predate a fold change — which is a **migration**, not a
code change (`L12`), and needs a backfill (`L13`, `O10.4a`). Four of the six
ports have no `upgrade` path, so for them it is a reload (**F-15**).

If you are writing SQL by hand, remember to compare against `family_norm` rather
than `family`, and to use a range rather than `LIKE` for prefixes (`P6.6a`).

## Why is `birthDate` a text column?

Because FHIR dates can be partial. `"1974"` and `"1974-12"` are valid
`date` values, and neither survives a native `DATE` column without inventing
precision the source did not have.

So the pattern is: store the lexical form, derive a typed `_sort` column at
write time for ordering and range search (`M3.6`). Filter on `birth_date_sort`;
display `birth_date`.

Decimals work the same way and for the same reason — `1.50` is not `1.5`
clinically, and no fixed-scale or floating-point column can hold the difference
(`M3.6a`).

## What is `ords`?

The position path: the 1-based index at each repeating ancestor crossing.
`{1,2}` is "the second given name of the first name".

An array rather than one column per level, because that is what lets recursive
elements — `Questionnaire.item.item.item…` — share one table at any depth.

The database never orders, compares, or subscripts it, which is why engines
without an array type can store the literal as text. Details:
[storage model](storage-model.md#ords).

## Why are there six copies of everything?

There were, and that was the problem this revision fixed.

The pure-Rust core — shredding, reconstruction, folding, canonical JSON, the
whole generator — is byte-identical across all six ports (`X15.1`), and that is
by design: it operates on Rust types and never emits SQL, so there is nothing in
it for a dialect to change.

The **specification** was also six copies, and that was not by design. Sections
1–13 were duplicated per port, identical apart from the product name, and they
had begun to drift. They are now one copy at `/spec`, with each port stating
only its departures (**F-13**, fixed).

## Can I move data between engines?

Yes, by export and load. The *logical* content of a store is engine-independent
(`X15.10`): the same resource shredded by two ports produces the same logical
rows under the same identifiers, because identifier budgets are set to the
tightest target precisely so names are comparable (`X15.3`).

What does not carry across is the hash chain. Verify it at the source before
exporting; the destination starts a new chain, reported as beginning where it
begins and never backfilled (`M3.16e`).

## Does it validate resources?

Structurally, always — element existence, cardinality, primitive lexical rules,
choice exclusivity, and required bindings, because that is inherent to shredding
against the map (`V9.1`). Unknown elements are rejected naming the path
(`R4.3`), never silently dropped.

Terminology validation is **out of scope** (`V9.4`). A `required` binding is
checked against the literal set of codes the generator extracted; no code system
is expanded, no subsumption computed, no `$validate-code` performed. Put a
terminology service in front if you need it.

Profile and implementation-guide validation beyond the base specification is
also out of scope.

## What FHIR versions?

R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2) — all resource types of
each, no exceptions (`S1.1`, `S1.3`). Versions are independent and never share a
table; a database may host any subset.

## Why two hash algorithms?

Family diversity, not digest length (`M3.16a`). MD5 and SHA-1 both fell to the
same line of cryptanalysis, and both are Merkle–Damgård. SHA-256 is
Merkle–Damgård; SHA3-256 is a sponge. A clinical record may be retained for
decades — longer than anyone can promise a single construction will stand.

Both are FIPS-approved, and verification reports each separately so a reader can
rely on whichever their regime recognises.

## Does the hash chain stop tampering?

Unkeyed, no — and the specification says so plainly rather than letting you
assume otherwise (`M3.16b`).

Unkeyed it detects **careless or unaware** modification and supports an external
witness. It does not stop an informed attacker with write access, because the
digests are unkeyed over a published pre-image: whoever can write the row can
compute a correct digest for it.

The keyed HMAC tag is the actual fix, and only because the key lives where the
database does not.

Neither catches a row that is **gone** — a truncated chain verifies perfectly.
Only an off-box checkpoint closes that gap.

## Who can I blame for the READMEs?

They were inherited from the PostgreSQL reference by text substitution, in ports
where the claims were never measured — including two with no store at all. Three
were even titled "FHIR in PostgreSQL" while targeting SQLite, MySQL, and
MariaDB. That was **F-01**, the most serious finding in the
[audit register](../spec/databases/audit.md), and all six were rewritten on 2026-07-31.

The same substitution produced an Oracle DDL emitter that emits MySQL types
(**F-08**, still open) and two dialect annexes titled "14. MySQL dialect" in
ports targeting neither (**F-16**, rewritten). The `book/` directories are the
remaining substituted documentation.

## How do I contribute?

[`AGENTS.md`](../AGENTS.md), then the relevant [topic guide](../agents/index.md).

The rule that catches everyone: the shared Rust core must be changed in **all
six ports in one commit** (`W16.7`). Check with `./scripts/check-shared-core.sh`
before and after; CI runs it too (**F-10** fixed).
