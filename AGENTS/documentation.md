# Documentation

**An example must compile** (`T11.9` in spirit, **F-60**). Run
`scripts/check-doc-examples.sh`. A ```` ```rust ```` block is a promise that the
code works; if it cannot compile — it targets another port, continues a previous
block, elides a struct — mark it ```` ```rust,ignore ```` and say why in an HTML
comment. Marking it to silence the gate is the failure this repository keeps
finding in other forms.

Listings of values or constructors are not code. Use ```` ```text ````: nine
blocks were ```` ```rust ```` while being tuples of search parameters, and the
gate has no way to distinguish those from a broken example.

## What is normative and what is not

Only [`/spec`](../spec/index.md) is normative. READMEs, books, `plan.md`,
`tasks.md`, and code comments describe; the spec decides. If a book chapter and
a requirement disagree, the chapter is wrong — even when the chapter is right
about the code, because then the code is wrong too.

## The substitution trap

**Do not text-substitute an engine name through a document.** This is the single
rule this file exists for, and breaking it produced the three most serious
findings in the register.

What it looks like when it goes wrong. Every port's README carried this until
2026-07-31, with the engine name swapped:

> all **7,399 official FHIR example resources** (R3 + R4 + R5) round-trip
> **losslessly** through the fully normalized schema — in memory, through live
> \<engine\>, and 10,000 generated property-test cases besides. 94.8% of R5
> search parameters compile to indexed SQL, and `fhir-<engine> serve` mounts
> every installed version with CRUD, history, ETag concurrency, search, and
> all-or-nothing transaction Bundles.

In `fhir-mssql` and `fhir-oracle` there is **no store crate implementation at
all**, and no CLI crate exists in any port. Nothing round-tripped through either
engine. That paragraph was a claim about clinical software, made in a product's
own name, that nothing substantiated (**F-01**, since fixed — the `book/`
directories still carry it).

The same substitution produced `fhir-oracle`'s `ddl.rs`, which emits MySQL types
(**F-08**), and the MSSQL and Oracle dialect annexes, which are the MySQL annex
with three lines changed and are titled "14. MySQL dialect" (**F-16**).

The rule, stated positively (`W16.8`–`W16.10`):

- A port's documentation describes **that port**: its engine, its conformance
  level, its own measurements, its own limitations.
- **Do not claim above the port's level** (`C0.11`). A Scaffold port's README
  says it is a scaffold.
- **A measured number names what measured it and when** (`W16.10`).
- **An example must run against the code as shipped** (`W16.9`). Every README
  documented `cargo install --path crates/fhir-<engine>` — a directory existing
  in no workspace. An example that cannot run is worse than none, because it
  costs the reader the time to find out. When rewriting one, check the
  constructor names and signatures against the source; three of the six needed
  correcting that way.

## Where documentation lives

| Path | Audience | Scope |
| --- | --- | --- |
| `/README.md` | someone arriving | the monorepo: what it is, which port to pick |
| `/index.md` | someone reading | the documentation hub, all entry points |
| `/doc/` | someone learning | tutorials, examples, comparisons, FAQ |
| `/spec/` | someone implementing or auditing | normative requirements |
| `fhir-<engine>/README.md` | someone choosing that port | that engine, honestly levelled |
| `fhir-<engine>/book/` | someone using that port | getting started through architecture |
| `fhir-<engine>/doc/` | someone operating it | benchmarks, CI, containers |

Shared conceptual material — the storage model, the fold, the trust boundary —
belongs in `/doc` or `/spec`, once. A port's book covers what is specific to
that engine and links out for the rest. Six copies of an explanation is the same
failure as six copies of a requirement, and it decays the same way.

## Style

The house voice is direct, and it explains why the obvious alternative is wrong.
Keep it.

- Lead with what the thing does, not with what it is built from.
- Give the reason next to the rule, especially where the rule looks arbitrary.
  "Not `DECIMAL`, because `DECIMAL(65,30)` returns `1.50` as
  `1.500000000000000000000000000000`" is the whole argument in one line.
- Cite requirement ids inline when a statement is normative — `(M3.6a)` — so a
  reader can get from prose to the decision.
- Prefer a table to a list of parallel sentences.
- State limitations in the same breath as capabilities. A reader who finds the
  limitation later trusts nothing else on the page.
- Do not hedge a fact you have verified, and do not assert one you have not.

## Links

Root-relative from the root; relative from a port. A port's spec index links the
core as `../../spec/…`. Check links after moving a file — three port spec
indexes carried a broken link to `14-mysql-dialect.md`, a filename that exists
only in `fhir-mysql`, for as long as the forks have existed.

## Updating documentation after a change

1. Does the change alter behaviour? Then `/spec` first
   ([spec-workflow](spec-workflow.md)).
2. Does it change what a port can do? Update
   [`spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md).
3. Does it close or open a finding? Update
   [`spec/audit.md`](../spec/databases/audit.md).
4. Does it change a documented example? Run the example.
5. Does it apply to all six ports? Then it is six edits, in one commit
   (`W16.7`).

Step 4 is the one that catches the CLI problem: the moment anyone runs a README
command, `cargo install --path crates/fhir-postgresql` fails and the gap
becomes visible.
