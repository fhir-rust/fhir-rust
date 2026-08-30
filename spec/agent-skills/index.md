# Agent skills

Create repository top-level agent skills folders:

- `fhir-skill` -> general-purpose skill for end users, about concepts, ideas, terminology, examples from this repo.
- `fhir-rust-maintainer-skill` -> technical implementation skill for maintainers working on this repository

Commit each skill separately.

## Status in this repository

Both exist at the repository top level, each with a `SKILL.md`.
`fhir-skill/SKILL.md` targets end users: it explains FHIR® concepts,
terminology, and the four-piece architecture, and points to real examples
in this codebase, deferring implementation questions to the other skill.
`fhir-rust-maintainer-skill/SKILL.md` targets maintainers and agents: it is
a task-oriented layer over the canonical docs — `AGENTS.md`, the `agents/`
directory, and the specs — with checklists and exact commands for
shared-core changes across the six database ports, spec-driven changes, and
pre-commit/pre-push checks, rather than restating what those documents
already say.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
