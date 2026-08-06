# fhir-mssql-gen

Reads the FHIR specification packages and emits the relational map and the DDL.

Part of **`fhir-mssql`**, which stores FHIR R3, R4 and R5 resources in SQL Server 2022 as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has no store.** It emits T-SQL DDL and nothing else — no driver, no transactions, no search, no way to read or write a resource.

## Install

```toml
[dependencies]
fhir-mssql-gen = "0.4.0"
```

## What it does

A **build-time** tool, not a runtime dependency. It reads one FHIR version's
`StructureDefinition`s and `SearchParameter`s and produces the map the `map`
crate consumes:

```rust
use fhir_mssql_gen::generate;

let map = generate(std::path::Path::new("fhir-definitions-json"), "r5")?;
let statements = fhir_mssql_map::ddl::ddl(&map);
```

The output ships as a committed `assets/*.json.gz`, so consumers never need the
FHIR packages themselves.

## Determinism

Same input, same map, same statements, **in the same order** (`G2.x`). This is
not a nicety: the map's checksum is what a store records at `init`, and what an
`upgrade` compares against to decide whether a migration is needed.

## Scale

The cost of generating from the specification rather than hand-writing is
thousands of tables per FHIR version — **7,355 for R5**. That is fine for a
database and impossible for a human, so every name is recorded in the map and
nothing is typed twice.

## What it emits

| Artifact | Contents |
| --- | --- |
| relational map | tables, columns, types, search-parameter compilation |
| DDL | T-SQL DDL |

Search-parameter compilation is partial by design: **92.4% of R5 parameters**
compile to a column target (an earlier revision said 94.8% — **F-38** removed
51 compilations that silently dropped a `where()` value restriction). The remainder — composites, and a handful of
expressions using FHIRPath functions the static walker does not implement — are
recorded as unsupported with a reason rather than guessed at.

This crate is identical across all six ports (`X15.1`). Only the `ddl.rs` it
drives, over in the `map` crate, differs.

## Further reading

| | |
| --- | --- |
| [Port README](../../README.md) | this engine's overview, quick start, and trust boundary |
| [Dialect annex](../../spec/14-mssql-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Scaffold** level: a T-SQL DDL emitter only; CI provisions SQL Server 2022 and fails rather than skips without it, so **Schema** level is reachable as soon as one green run exists to cite.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
