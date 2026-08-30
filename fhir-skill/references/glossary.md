# FHIR® terminology

Plain-language definitions of the terms that come up most. This is general
FHIR® knowledge (from the HL7® specification), not specific to this
repository, so it applies whether someone is reading Rust types, SQL tables,
or raw JSON.

**Resource** — the basic unit of FHIR data: a `Patient`, an `Observation`, a
`MedicationRequest`, and so on. Every resource has a `resourceType`, an `id`,
and (once stored) a `meta` block with a version and last-updated timestamp.
There are 140+ resource types in a given release, covering clinical,
administrative, and infrastructure concerns.

**Datatype** — a reusable shape that appears inside resources rather than
standing alone: `HumanName`, `Address`, `Period`, `CodeableConcept`,
`Identifier`, `Quantity`. Complex datatypes are structs with several fields;
**primitive** datatypes (`string`, `code`, `dateTime`, `boolean`, …) are
single values that can still carry an `id` and extensions, which is why FHIR
primitives are not just native JSON scalars.

**Reference** — a pointer from one resource to another, e.g. an
`Observation.subject` pointing at a `Patient`. Can be a relative URL
(`Patient/123`), an absolute URL, or a contained/logical reference. FHIR is
a graph of resources connected by references, not a set of independent rows.

**Extension** — FHIR's mechanism for adding data that isn't in the base
specification, identified by a URL rather than a fixed field name. Every
element, not just every resource, can carry extensions. This is why
"lossless round-trip" is a real engineering problem: an extension a system
doesn't recognize still has to survive being stored and returned.

**CodeableConcept / Coding** — how FHIR represents coded values. A `Coding`
is one code from one system (a URL identifying, say, LOINC or SNOMED CT)
plus a display string; a `CodeableConcept` is one or more `Coding`s plus
optional free text, because real-world data is often coded more than one way
or not coded at all.

**Bundle** — a resource that contains other resources: a search result set,
a transaction, a document, a message. The type of bundle (`searchset`,
`transaction`, `document`, …) changes what the entries mean.

**Profile / StructureDefinition** — a constraint on a resource (required
fields, fixed values, extra extensions) for a specific use case, expressed
as its own resource. FHIR "base" resources are permissive by design;
profiles are how an implementation guide narrows that down.

**Version (R2, R3, R4, R4B, R5, R6 / DSTU2, STU3, …)** — FHIR is not one
frozen schema; it has released versions, and resources differ between them
(fields added, removed, or reshaped). "R4" and "4.0.1" refer to the same
release. This repository models several releases side by side rather than
picking one, because a real deployment often has to speak a specific one.

**RESTful interactions** — the standard FHIR HTTP verbs on a resource type:
`create`, `read`, `vread` (read a specific version), `update`, `delete`,
`history`, and `search`. A FHIR server's conformance is largely a statement
about which of these it supports, on which resource types, with which
search parameters — captured in its `CapabilityStatement`.

**Search parameter** — a named, indexed way to query resources
(`Patient?family=Smith`, `Observation?code=http://loinc.org|1234-5`), defined
per resource type and per FHIR version rather than being free-form SQL.

**Audit / provenance** — FHIR distinguishes the clinical record from the
record of who touched it and when. This repository's persistence layer keeps
a tamper-evident audit chain (see `references/examples.md`) as a separate
concern from the resource data itself.

**PHI** — Protected Health Information. FHIR resources routinely carry it;
handling it correctly (encryption, access logging, redaction, erasure) is a
first-class concern in this repository's database ports, not an
afterthought.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
