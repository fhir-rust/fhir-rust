# 1. Scope

- **S1.1** Every port MUST support FHIR R5 (5.0.0), R4 (4.0.1), and R3 (3.0.2).
  R5 is the default everywhere a version is optional.
- **S1.2** Each FHIR version's data lives in its own **namespace**: `r5`, `r4`,
  `r3`. Versions are independent; a database MAY host any subset. The namespace
  is realized by whatever mechanism the engine provides for isolating a set of
  tables under a name, and the mechanism is a dialect concern (`X15.6`) — the
  requirement is that the three versions never share a table.
- **S1.3** All resource types defined by the version's specification MUST be
  supported — no unsupported-type errors for spec-defined types.
- **S1.4** Each port MUST declare a **minimum engine version** and MUST NOT
  silently work below it. Features requiring at least that version MAY be used;
  older servers are unsupported. The floor is a dialect decision because it is
  driven by dialect facts — an identifier length, a boolean type, a JSON type, a
  window function — and it MUST be stated in the port's annex rather than
  inherited by assumption.

## Engine bindings

Non-normative summary; each port's annex is authoritative for its own row.

| Port | Engine | Declared floor | Namespace mechanism |
| --- | --- | --- | --- |
| `fhir-postgresql` | PostgreSQL | 18 | `CREATE SCHEMA r5` |
| `fhir-sqlite` | SQLite | 3.35+ | one database file, or `ATTACH` per version |
| `fhir-mysql` | MySQL | 8.4 | database (`CREATE DATABASE r5`) |
| `fhir-mariadb` | MariaDB | 11.4 | database (`CREATE DATABASE r5`) |
| `fhir-mssql` | SQL Server | 2019+ | `CREATE SCHEMA r5` |
| `fhir-oracle` | Oracle Database | 12.2+, undeclared | user/schema |

Oracle's row is the open one: identifiers were 30 bytes before 12.2 and 128
after, so the generator's identifier budget (`G2.4`) is only safe on 12.2+ and
that floor has not yet been written into the annex. Tracked as
[`audit.md`](audit.md) **F-09**.

## Out of scope

Stated here so that "not implemented" and "not intended" are distinguishable:

- Authentication and authorization in the library core. Identity is established
  at the deployment perimeter; §12 defines what the core records regardless.
- Terminology services — `$expand`, `$validate-code`, code-system-aware
  validation beyond a value set's literal codes (§9).
- A FHIRPath evaluation engine, subscriptions, and GraphQL.
- Profile and implementation-guide validation beyond the base specification.
- An HTTP service and a command-line tool (`C0.15`).

---

Part of the [fhir-databases specification](index.md).
