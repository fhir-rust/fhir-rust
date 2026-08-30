# FHIR concepts

Deeper explanations of the ideas behind the terms in
[`glossary.md`](glossary.md). Still general FHIR® knowledge — the point is to
understand the standard, with a note on how this repository reflects each
idea where that helps.

## Resources are the unit, not tables or objects

Everything in FHIR is a resource or lives inside one. There is no
"orphan field" — a lab value belongs to an `Observation`, an allergy to an
`AllergyIntolerance`. This matters when someone asks "where does X live in
FHIR" — the answer is almost always "inside a specific resource type," and
finding which one is most of the work. `fhir/fhir.md` in this repository is
a generated, searchable index of every resource and field for exactly this
kind of lookup.

## A resource graph, not a row per record

Resources reference each other by id (`Reference` datatype) rather than
embedding everything inline. An `Observation` references its `Patient`
rather than repeating the patient's name and birth date. This keeps data
from being duplicated across resources, but it means reading "the full
picture" for a patient means following references, or asking a server to
follow them (`_include`/`_revinclude` search parameters). When this
repository's database ports store a resource, references become foreign
keys into other resource tables — the graph shape survives being shredded
into relational form, which is a large part of what "lossless" means.

## Extensions: the escape hatch that's used constantly

Base FHIR resources are deliberately minimal — the "80%" that most
implementers need. Everything else goes through extensions, identified by a
canonical URL rather than a name registered with HL7. A US Core
`race`/`ethnicity` extension on `Patient`, a lab-specific device extension on
`Observation` — these are normal, not edge cases. Any system that silently
drops unrecognized extensions is not round-tripping FHIR correctly, which is
why this repository's model and storage layers both treat extension fidelity
as a correctness requirement rather than a nice-to-have.

## Coded data is deliberately imprecise sometimes

`CodeableConcept` allowing multiple codings plus free text isn't sloppiness —
it reflects that clinical data is sometimes coded in more than one
terminology at once (a diagnosis in both ICD-10 and SNOMED CT) and sometimes
not coded at all (free-text notes that haven't been coded yet). Code systems
themselves (LOINC, SNOMED CT, RxNorm, HL7's own value sets, …) are external
to FHIR; FHIR just gives them a consistent place to live. This repository's
model crate represents HL7-defined code systems as type-safe Rust enums
(400+ of them) precisely because those *are* fixed by the spec, while
leaving room for codings from external terminologies as plain strings.

## Versions are a real fork, not a superset

R4 is not a strict superset of R3, and R5 is not a strict superset of R4 —
fields get renamed, restructured, or removed between releases, not just
added. "Which version" is often the first question to ask when something
about a resource's shape looks surprising. This repository keeps releases
R2 through R6 as separate, independently generated modules
(`fhir::r2` .. `fhir::r6`) rather than one merged type, specifically so that
a field that only exists in one release doesn't leak into another.

## REST interactions map onto a small, fixed vocabulary

`create`, `read`, `vread`, `update`, `delete`, `history`, `search` — a FHIR
server's capabilities are usually described as which of these it supports on
which resource types. This repository's HTTP surface,
[`fhir-loco`](../../fhir-loco/), implements this vocabulary directly (see its
own docs for exact endpoints and status codes); the database ports
underneath expose the same operations as a Rust API (`put`, `get`, `delete`,
`history`, `vread`, `search`, …) without an HTTP layer, so the same
interaction vocabulary shows up at both levels.

## Audit is a separate resource of trust, not a side effect

Knowing *who* changed a resource and *when*, in a way that can't be quietly
edited after the fact, is treated as its own guarantee — separate from
storing the resource correctly in the first place. This repository's ports
each maintain a tamper-evident audit chain (hash-linked records) alongside
the resource data, verifiable independently of the resource content itself.
See `references/examples.md` for where that lives in the code.
