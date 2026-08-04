# FHIR versions

fhir-oracle supports R5 (5.0.0, the default), R4 (4.0.1), and R3 (3.0.2).
Each version is generated from its own specification package into its own
map asset (selected by Cargo feature — `r3`/`r4`/`r5`, `r5` is the crate
default) and installs into its own Oracle **user** — `R3`, `R4`, `R5`,
uppercase and unquoted — because Oracle conflates user and schema (`M14.5`).
This port does not create them (`M14.28`); provisioning is a deployment
prerequisite, or `scripts/db.sh up`'s `post_ready` step for local
development. There is no `serve` and no process that mounts them: this is a
library (`C0.17`).

There is no cross-version translation: a resource loads into whichever
`RelMap` you constructed the store with (`RelMap::bundled("r5")`, for
example — see [Getting started](getting-started.md)), and the shred engine
rejects elements the selected version does not define, naming the offending
path. The whole official example corpus of each version round-trips
losslessly through the map layer — 1,664 R3, 2,911 R4, 2,824 R5 resources,
0 failures.

Version-specific storage differences fall out of the specs themselves —
R5's `integer64` maps to `NUMBER(19)` (a JSON string on the wire, per R5's
own rule for 64-bit integers, since JSON numbers cannot losslessly hold the
full range); R3 lacks several datatypes; choice-type membership differs
— but the storage model, engine, and search machinery are identical across
versions; only the generated maps differ.

Querying across two versions in the same statement is not possible the way
it might be with schema-qualified tables in a single database — `R4` and
`R5` are separate Oracle **users**, and a cross-user join is no more
convenient here than a cross-database one would be elsewhere.
