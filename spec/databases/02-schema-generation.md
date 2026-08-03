# 2. Schema generation

- **G2.1** DDL and relational maps MUST be generated from the official FHIR
  specification packages (StructureDefinitions, SearchParameters) by the port's
  `gen` operation, and the generated artifacts MUST be committed under `assets/`
  so that builds and installs never require the spec packages.

  The commitment matters more than it looks: it makes a build reproducible
  without a network, without hl7.org being up, and without the spec packages
  having stayed byte-identical since the artifact was cut.

- **G2.2** Generation MUST be deterministic: same spec input → byte-identical
  output. `assets/CHECKSUMS.txt` records SHA-256 of every artifact.

- **G2.2a** *Amends `G2.2`.* The "output" whose determinism is required is the
  **map content** — the JSON inside the container — not the compressed file.
  Drift detection MUST compare content; `CHECKSUMS.txt` continues to record the
  file digest, which answers a different question (was this artifact corrupted
  in transit) and MUST remain verifiable by `shasum -a 256 -c`.

  Driven by `fhir-mysql` and `fhir-mariadb` (`C0.22`). Their store crates depend
  on `mysql_async`, which enables `flate2/zlib`; the map crate uses `flate2`'s
  default `miniz_oxide`. Cargo unifies features across a workspace, so the same
  map compresses to **different bytes** depending on which other crates are in
  the build — `cargo run -p <port>-gen` and `cargo test --workspace` disagreed,
  and the second failed against assets the first had just written (**F-41**).

  `G2.2` as originally written was therefore unsatisfiable for two ports, and
  satisfiable elsewhere only by accident of which backend happened to be
  selected. Compression is an encoding of the artifact, not the artifact.
- **G2.3** Identifier naming: element paths convert to snake_case (`birthDate` →
  `birth_date`). Table names concatenate the resource name and element path
  (`Patient.name.given` → `patient_name_given`).
- **G2.4** Every engine truncates or rejects identifiers past some length. The
  generator MUST fit every name into an **identifier budget** — a single
  constant, declared per port. Where a generated name would exceed the budget,
  the generator MUST abbreviate deterministically and, on residual collision,
  suffix with a 6-hex-digit hash of the full path. The full-path → identifier
  mapping MUST be recorded in the relational map and in a generated `doc/`
  index; two different paths MUST never map to the same identifier.

  The budget MUST be less than or equal to the engine's real limit. A port whose
  engine is *more* permissive MAY keep a tighter budget unchanged — the names
  stay comparable across ports, which is worth more than the extra characters —
  but MUST say so in its annex rather than leave a PostgreSQL-shaped constant
  sitting unexplained in a port that is not PostgreSQL.

- **G2.5** The `init` operation MUST be idempotent and effectively atomic.
  Init MUST record the applied artifact checksum in the port's metadata table,
  MUST no-op when the installed checksum matches, and MUST refuse to run against
  a namespace created from a different artifact (see §10 migrations).

  *Atomicity is a dialect problem.* Installing a FHIR version means creating
  thousands of tables — 7,355 for R5 — and engines differ in whether that can be
  one transaction. Where transactional DDL exists but the lock budget does not
  (PostgreSQL's `max_locks_per_transaction`), init stages the install under a
  temporary namespace in chunked transactions and renames it into place in a
  single statement; a failed init leaves only the staging namespace, which the
  next init removes. Where DDL is not transactional at all, "effectively atomic"
  means the same observable outcome by a different route, and the annex MUST say
  which route. Namespace drops are likewise chunked.

- **G2.6** The generator MUST bound generated table width below the engine's
  column limit, by forcing a flattened expansion wider than a declared threshold
  into its own table (`M3.5`). Like `G2.4` this is one constant, declared per
  port, and MUST be low enough for the tightest engine a port targets.

## Engine bindings

Non-normative; the annexes govern.

| Port | Identifier budget | Engine limit | Split width | Column limit |
| --- | --- | --- | --- | --- |
| `fhir-postgresql` | 63 | 63 bytes | 150 | 1600 |
| `fhir-sqlite` | 63 | effectively none | 150 | 2000 |
| `fhir-mysql` | 63 | 64 | 150 | 4096 |
| `fhir-mariadb` | 63 | 64 | 150 | 4096 |
| `fhir-mssql` | 63 | 128 | 150 | 1024 |
| `fhir-oracle` | 63 | 128 (30 before 12.2) | 150 | 1000 |

The shared value of 63 is not an accident of copying — it is the tightest
budget among the targets, so a name generated once is legal everywhere, and a
schema can be compared name-for-name across engines. That property is worth
keeping deliberately; `X15.3` requires it.

---

Part of the [fhir-databases specification](index.md).
