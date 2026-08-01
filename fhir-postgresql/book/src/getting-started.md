# Getting started

You need PostgreSQL 18 and Rust.

```sh
cargo install --path crates/fhir-postgresql
export PGHOST=localhost PGUSER=you PGDATABASE=clinic

fhir-postgresql init --fhir-version r5      # install the generated schema (~6 s)
fhir-postgresql load export/*.ndjson        # load NDJSON / Bundles / single files
fhir-postgresql serve                       # FHIR REST API on 127.0.0.1:8080
```

`load` detects format by content, not filename: NDJSON, a Bundle, or a
single resource, gzipped or plain. Failures are reported per resource with
file and line; the exit code is nonzero if anything failed. Add
`--validate` (in builds with the `validate` feature) to also check every
resource against the typed FHIR model.

Useful commands while exploring:

```sh
fhir-postgresql transform patient.json      # show exactly which rows a resource becomes
fhir-postgresql search Patient family=Smith birthdate=ge1970
fhir-postgresql get Patient example         # reconstruct one resource
fhir-postgresql export Patient > patients.ndjson
```

Connection settings come from the standard `PG*` environment variables or
`--dsn`. Each FHIR version installs into its own PostgreSQL schema (`r5`,
`r4`, `r3`) inside whatever database you point at.
