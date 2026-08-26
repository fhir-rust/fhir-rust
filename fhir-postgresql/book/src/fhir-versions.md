# FHIR® versions

fhir-postgresql supports R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2). Each
version is generated from its own specification package into its own map
asset (`RelMap::bundled("r5" | "r4" | "r3")`) and installs into its own
PostgreSQL schema — `r5`, `r4`, `r3` (`M14.4`) — so one database can host any
subset side by side, each behind its own `Store` built from that version's
map. Mounting more than one version in a single process is a caller decision
(construct one `Store` per schema); nothing here does it for you, and there
is no `serve` — see the [introduction](introduction.md).

There is no cross-version translation: a resource loads into whichever
version's `Store` you constructed it against, and the engine rejects
elements the selected version does not define, naming the offending path.
The whole official example corpus of each version round-trips losslessly —
7,399 resources across all three (`R4.2`).

Version-specific storage differences fall out of the specs themselves —
R5's `integer64` maps to `bigint` (JSON string per R5 rules), R3 lacks
several datatypes, choice-type membership differs — but the storage
model, engine, and search machinery are identical across versions; only
the generated maps differ.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
