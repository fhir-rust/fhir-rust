# FHIR versions

fhir-mysql supports R5 (5.0.0, the default Cargo feature), R4 (4.0.1), and R3
(3.0.2). Each version is generated from its own specification package into its
own map asset — `crates/fhir-mysql-map/assets/fhir-mysql-relmap-{r3,r4,r5}.json.gz`,
0.5 MB (r3) to 1.2 MB (r5) — and each map's `schema` field names the MySQL
*database* it installs into (`r3`, `r4`, `r5`; MySQL has no separate schema
concept, so a database is the namespace). All three can be compiled in and
coexist side by side; the default build carries only `r5`.

There is **no** `--fhir-version` flag and no CLI at all (`C0.17`) — this is a
library call:

```rust,ignore
use std::sync::Arc;
use fhir_mysql_map::model::RelMap;

let map = Arc::new(RelMap::bundled("r4")?);   // needs the `r4` Cargo feature
```

There is no cross-version translation: a resource shreds against whichever
map you loaded, and the engine rejects elements that map's version does not
define, naming the offending path (`R4.1`). The whole official example corpus
of each version round-trips losslessly in the map layer with no store
involved — 1,664 (R3) + 2,911 (R4) + 2,824 (R5) = 7,399/7,399, 0 failures
(`R4.2`, **F-42**).

Version-specific storage differences fall out of the specs themselves — R5's
`integer64` shreds from a JSON string (per R5's own rule for 64-bit integers)
into `BIGINT`, where R3 and R4 have no such primitive at all; R3 lacks several
datatypes R4/R5 have; choice-type (`value[x]`) membership differs release to
release. The storage model, the shred/reconstruct engine, and the search
machinery (`fhir-mysql-map`, `fhir-mysql-gen`) are identical across versions —
only the generated maps differ, and `fhir-mysql-store`'s SQL does not know or
care which version it is talking to.
