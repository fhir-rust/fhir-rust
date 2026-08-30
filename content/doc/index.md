# Documentation

Learning material for the six **database ports**. The
[specification](../spec/databases/index.md) decides what must be true; these
pages explain how it works and how to use it.

The repository's other two families are documented elsewhere: the model crate in
[`fhir/`](../fhir/README.md), the HTTP surface in
[`fhir-loco/`](../fhir-loco/README.md) (`fhir-store/` is the engine-agnostic
persistence-core library the ports share, not the HTTP surface).
[`spec/index.md`](../spec/index.md)
routes between all three.

## Tutorials

Read in order the first time; each builds on the last.

1. **[Your first store](tutorial-01-getting-started.md)** — install a schema,
   write a resource, read it back, search for it. About 15 minutes, no server
   required.
2. **[The storage model](tutorial-02-storage-model.md)** — what a resource
   becomes: base tables, child tables, `ords`, choices, references, extensions.
3. **[Querying with SQL](tutorial-03-querying-sql.md)** — treat it as the
   relational schema it is. Joins, aggregates, and the idioms that differ per
   engine.
4. **[FHIR® search](tutorial-04-search.md)** — search parameters, modifiers,
   paging, and why "aero" finds "Ærø".
5. **[History and audit](tutorial-05-history-and-audit.md)** — versions,
   attribution, the tamper-evident chain, keys, and erasure.
6. **[Porting to a new database](tutorial-06-porting.md)** — what changes, what
   must not, and the order to do it in.

## Reference

- **[Choosing an engine](choosing-an-engine.md)** — the six ports compared:
  status, type bindings, what each costs you.
- **[The storage model](storage-model.md)** — the conceptual reference behind
  tutorial 2, for looking things up rather than reading through.
- **[The trust boundary](trust-boundary.md)** — what these libraries guarantee
  about PHI, and what your deployment must provide. One table, per `PR12.8`.
  Its plain-language companion, written for a privacy officer or a vendor
  questionnaire rather than a specification auditor, is
  [`PHI.md`](../PHI.md) at the repository root.
- **[Examples](examples.md)** — short recipes: bulk load, conditional create,
  chain verification, cross-version stores, integrity reports.
- **[FAQ](faq.md)** — why not JSONB, why so many tables, is it FHIR-compliant,
  can I use it in production.

## Status, before you rely on anything

- **[Conformance matrix](../spec/databases/conformance-matrix.md)** — what each port
  satisfies, requirement by requirement. Four of the six ports have `?` against
  the concurrency and audit guarantees, which means the code is shared from a
  port where they are tested and nothing tests them there.
- **[Audit findings](../spec/databases/audit.md)** — every known divergence between spec,
  docs, and code, with evidence.

The per-port `README.md` files were rewritten on 2026-07-31 and should now agree
with the matrix — all six had carried the PostgreSQL reference's measured
results with the engine name substituted, and documented a CLI existing in no
workspace (**F-01**). If a README and the matrix disagree, the README is the
defect.

The per-port `book/` directories were rewritten on 2026-08-03 (**F-56**): each
now names its own engine and its own backup tooling, opens with a "Read this
first" banner, and attributes every `serve`/endpoint/status code to
`fhir-loco`, the separate crate that actually provides them.

## Which engine does the examples use

Most examples use `fhir-sqlite`, because it needs no server and its code is
short enough to read. The API shape is the same across ports:

| | SQLite | PostgreSQL |
| --- | --- | --- |
| Open | `SqliteStore::open(path, map)` | `Store::connect(cfg, map)` |
| Install | `store.init(checksum)` | `store.init(checksum)` |
| Write | `store.put(&json, &audit)` | `store.put(&json)` / `put_audited` |
| Read | `store.get(rtype, id)` | `store.get(rtype, id)` |
| Search | `store.search(rtype, &params, count, offset)` | same |

Where a port differs materially, the page says so.

## Conventions in these pages

- Code is Rust against the crates as they exist, and
  `scripts/check-doc-examples.sh` compiles every one of them (**F-60**). If an
  example does not compile, that is a bug in the page (`W16.9`). No example here
  describes a CLI, because none exists; the REST server does exist, as the
  separate crate [`fhir-loco`](../fhir-loco/), and these pages are about the
  storage libraries rather than about it.
- Requirement ids appear inline — `(M3.4a)` — so you can get from an explanation
  to the decision behind it.
- Limitations are stated next to capabilities, not in a footnote.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
