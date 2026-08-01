# Bundles

A `Bundle` carries a heterogeneous list of resources — a search result, a
document, or a transaction. This crate provides utilities for reading and
building them.

## Reading a bundle

Each `Bundle.entry.resource` is raw JSON (its type is known only at runtime).
Iterate the entries as the polymorphic `Resource` enum, or filter to one type:

```rust
use fhir::r5::resources::{Bundle, Patient};

let bundle: Bundle = serde_json::from_value(serde_json::json!({
    "resourceType": "Bundle",
    "type": "collection",
    "entry": [
        { "resource": { "resourceType": "Patient", "id": "a" } },
        { "resource": { "resourceType": "Observation", "id": "o",
                        "status": "final", "code": {} } }
    ]
})).unwrap();

assert_eq!(bundle.iter_resources().count(), 2);

let patients: Vec<Patient> = bundle.resources::<Patient>("Patient").collect();
assert_eq!(patients.len(), 1);
```

## Paging

For a search bundle, `next_link()` returns the `next` relation's URL:

```rust
# use fhir::r5::resources::Bundle;
let bundle: Bundle = serde_json::from_value(serde_json::json!({
    "resourceType": "Bundle", "type": "searchset",
    "link": [{ "relation": "next", "url": "http://server/fhir/Patient?page=2" }]
})).unwrap();
assert_eq!(bundle.next_link(), Some("http://server/fhir/Patient?page=2"));
```

## Building a transaction

`Bundle::transaction()` (and `batch()`) builds a bundle of create/update/delete
operations:

```rust
use fhir::r5::resources::{Bundle, Patient};

let patient = Patient::default();
let bundle = Bundle::transaction()
    .create(&patient, "Patient")   // POST Patient
    .update(&patient, "Patient/1") // PUT  Patient/1
    .delete("Patient/2")           // DELETE Patient/2
    .build();

assert_eq!(bundle.entry.len(), 3);
```

## Talking to a server

With the `client` feature, `fhir::r5::client::Client` sends these interactions to
a FHIR server; a transaction bundle can be `POST`ed to the base URL. See the
`client_crud` example. `fhir::r4::client::Client` is the R4 counterpart — one
implementation, generic over the release, so both return their own release's
`Resource` and `Bundle`.
