# FHIR versions

fhir-mysql supports R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2). Each
version is generated from its own specification package into its own map
asset and installs into its own MySQL database — `r5`, `r4`, `r3` — since MySQL has no
separate schema concept, so a database *is* the namespace,
so the three can coexist side by side. There is no `serve` and no process
that mounts them: this is a library (`C0.17`).

There is no cross-version translation: a resource loads into the version
schema you point at (`--fhir-version`), and the engine rejects elements
the selected version does not define, naming the offending path. The
whole official example corpus of each version round-trips losslessly.

Version-specific storage differences fall out of the specs themselves —
R5's `integer64` maps to `bigint` (JSON string per R5 rules), R3 lacks
several datatypes, choice-type membership differs — but the storage
model, engine, and search machinery are identical across versions; only
the generated maps differ.
