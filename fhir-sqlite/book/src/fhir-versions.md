# FHIR® versions

fhir-sqlite supports R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2).
Each version is generated from its own specification package into its own
map asset (`crates/fhir-sqlite-map/assets/fhir-sqlite-relmap-<ver>.json.gz`)
and installs into its own attached database file — because in SQLite a
schema is a file, not a namespace inside a server, the three can coexist
side by side under one `SqliteStore` (`M14.16`; see [Getting
started](getting-started.md)).

There is no `--fhir-version` flag, because there is no CLI (`C0.17`). Version
selection happens in Rust, in two places:

- **At compile time**, via Cargo features on `fhir-sqlite-map`: `r3`, `r4`,
  `r5`, with `r5` on by default. A feature gates whether
  `RelMap::bundled("r4")` can *succeed* in your binary — all three maps ship
  inside the crate regardless (~2.5 MB total), so the feature controls
  compilation, not download.
- **At runtime**, by which map you pass to `SqliteStore::open`:

  ```rust,ignore
  let map_r4 = std::sync::Arc::new(fhir_sqlite_map::model::RelMap::bundled("r4")?);
  let store_r4 = SqliteStore::open("clinic.sqlite", map_r4).await?;
  // Attaches (or creates) clinic-r4.sqlite, independent of any r5 store
  // opened against the same main path.
  ```

There is no cross-version translation. A resource shreds against whichever
map its store was opened with, and the shredder rejects any element that
map's version does not define — reporting the JSON path it failed at
(`ShredError::At { path, msg }`, e.g. `"foo.bar: unknown element"`), not a
generic failure. The whole official example corpus of each version round-trips
losslessly in memory, independent of any store (`R4.2`, audit **F-42**):
7,399 examples total, 1,664 for R3, 2,911 for R4, 2,824 for R5, 0 failures.

Version-specific storage differences fall out of the specifications
themselves — R5's `integer64` maps to `ColTy::BigInt`/`INTEGER` (a JSON
string on the wire, per R5's own rule for 64-bit integers), R3 lacks several
datatypes R4/R5 have, and choice-type (`value[x]`) membership differs by
version — but the storage model, the shred/reconstruct engine, and the search
compiler are identical across versions (`X15.1`); only the generated maps
differ. Nothing in this port's own code branches on FHIR version.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
