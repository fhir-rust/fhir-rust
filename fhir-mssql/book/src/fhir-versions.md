# FHIR versions

fhir-mssql supports R5 (5.0.0), R4 (4.0.1), and R3 (3.0.2) — `S1.1`, assets
committed for all three. Each version is generated from its own
specification package into its own map asset
(`fhir-mssql-relmap-{r3,r4,r5}.json.gz`, loaded via `RelMap::bundled(version)`
or `RelMap::from_gz_bytes`) and installs into its own SQL Server schema —
`[r5]`, `[r4]`, `[r3]` — inside one database, so the three can coexist side
by side (`M14.4`). There is no `serve` and no process that mounts them: this
is a library (`C0.17`), and which version a caller writes into is a plain
function argument (`MsSqlStore::connect(dsn, map)`, where `map` already
names its schema), not a CLI flag.

There is no cross-version translation: a resource shreds against whichever
map the `MsSqlStore` was built with, and the shredder rejects elements the
selected version does not define, naming the offending path. The whole
official example corpus of each version round-trips losslessly — 7,399
resources (R3 1,664 / R4 2,911 / R5 2,824), 0 failures, map layer, no
database needed (`R4.2`, **F-42**).

Version-specific storage differences fall out of the specs themselves —
R5's `integer64` maps to `BigInt`/`BIGINT` (a JSON string on the wire per R5
rules, an ordinary 64-bit integer once shredded), R3 lacks several
datatypes, choice-type membership differs — but the storage model, the
shred/reconstruct engine, and the search machinery are identical across
versions and across all six ports (`X15.1`); only the generated maps
differ. This port's SQL Server-specific decisions — bracketed identifiers,
`NVARCHAR` throughout, the `VARBINARY` `ords` encoding, the 900-byte index
key limit — apply uniformly to whichever version's schema they operate on.
