# FHIR versions

fhir-mariadb supports R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2). Each
version is generated from its own specification package into its own map
asset and installs into its own MariaDB **database** — `r5`, `r4`, `r3`.
MariaDB has no schema concept separate from a database (`M14.21`), so the
database *is* the namespace, and the three can coexist side by side on one
server. There is no `serve` and no process that mounts them: this is a
library (`C0.17`).

There is no cross-version translation and no `--fhir-version` flag — there is
no CLI at all (`C0.17`). A caller picks the version by which map it loads,
`RelMap::bundled("r5")` (or `"r4"`, `"r3"`, each behind its own Cargo
feature) or a map read from an asset file, and connects a `MariaDbStore` to
the matching database. The engine rejects elements the selected version does
not define, naming the offending path. The whole official example corpus of
each version round-trips losslessly (7,399 resources: r3 1,664 / r4 2,911 /
r5 2,824 — `R4.2`, audit **F-42**).

Version-specific storage differences fall out of the specs themselves — R5's
`integer64` maps to `BIGINT` (a JSON string on the wire, per R5's own rule),
R3 lacks several datatypes, choice-type membership differs — but the storage
model, engine, and search machinery are identical across versions; only the
generated maps differ.
