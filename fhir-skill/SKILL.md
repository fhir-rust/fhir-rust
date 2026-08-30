---
name: fhir-skill
description: Explains FHIR® concepts, terminology, and ideas for people using or learning about this repository — resources, datatypes, references, extensions, versions, and the RESTful interactions — and points to real examples in this codebase. Use when someone asks what FHIR is, what a term means, how a concept works, or wants an example grounded in this repo, as opposed to how to build or maintain it.
---

# FHIR concepts and examples

This skill is for **end users**: people who want to understand FHIR® itself,
or who are using this repository's Rust types, database ports, or REST API
and need a concept explained. It is not the maintainer's guide — for
implementation, spec compliance, or contribution questions, the
`fhir-rust-maintainer-skill` covers that instead.

FHIR (Fast Healthcare Interoperability Resources, pronounced "fire") is the
HL7® standard for exchanging electronic health records.

## How this repo implements FHIR

Four pieces, stacked one way — see the root [`README.md`](../README.md) and
[`spec/index.md`](../spec/index.md) for the authoritative version of this:

1. **[`fhir/`](../fhir/)** — the data model. Every FHIR resource and datatype
   as a Rust struct, generated from HL7's own specification packages, for
   releases R2 through R6 (R4 is the default; others are cargo features).
2. **[`fhir-store/`](../fhir-store/)** — the engine-agnostic half of
   persistence: the audit chain and shared result types.
3. **Six database ports** (`fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`,
   `fhir-mariadb`, `fhir-mssql`, `fhir-oracle`) — the same resources stored as
   real relational tables, not JSON blobs, and read back losslessly.
4. **[`fhir-loco/`](../fhir-loco/)** — a FHIR RESTful API server over one of
   those stores.

A resource never skips a layer: the Rust type is what gets shredded into
tables, and the REST API is what serves those tables over HTTP.

## Where to go for what

| You want | Read |
| --- | --- |
| A term defined | [`references/glossary.md`](references/glossary.md) |
| A concept explained (resources, references, extensions, versioning, search) | [`references/concepts.md`](references/concepts.md) |
| A worked example in this repo's own code | [`references/examples.md`](references/examples.md) |
| The full, guided walkthroughs | [`doc/`](../doc/index.md) tutorials (database ports), [`fhir/doc/`](../fhir/doc/) (the model crate), [`fhir-loco/`](../fhir-loco/) (the server) |
| The official standard itself | [hl7.org/fhir](https://hl7.org/fhir/) |

## How to use this skill

- Start from the glossary or concepts reference for a definition or an
  explanation; both are general FHIR knowledge, not specific to this repo, so
  they stay correct even as the code changes.
- Reach for `references/examples.md` when the question is "show me where this
  shows up in this codebase" — it points at real files rather than repeating
  code that can drift out of date. Prefer reading the linked file over
  trusting a remembered snippet, especially for anything version-specific.
- If a question turns out to be about contributing, fixing a bug, or how a
  port is implemented, that is the maintainer skill's territory, not this
  one — say so rather than guessing at implementation details.
- If a question is about the FHIR standard in general and this repo has no
  special angle on it, answer from FHIR knowledge directly; there is no need
  to force a repo reference where none is relevant.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
