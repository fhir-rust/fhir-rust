//! Build a FHIR transaction `Bundle` and read resources back out of one.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example transaction_bundle
//! ```
//!
//! A transaction `Bundle` batches several create/update/delete operations into a
//! single request. `Bundle::transaction()` builds one; `Bundle::iter_resources`
//! and `Bundle::resources::<T>` read resources back out, and `Bundle::next_link`
//! follows search paging.

use fhir::r5::resources::{Bundle, Patient};
use fhir::r5::types::String as FhirString;

fn main() {
    let patient = Patient::builder()
        .id(FhirString("pat-1".to_string()))
        .build()
        .expect("build patient");

    // Build a transaction: create one patient, delete another.
    let bundle = Bundle::transaction()
        .create(&patient, "Patient")
        .delete("Patient/old-1")
        .build();

    let json = serde_json::to_string_pretty(&bundle).expect("serialize");
    println!("transaction bundle:\n{json}\n");

    // Read a search bundle and iterate its resources.
    let search: Bundle = serde_json::from_value(serde_json::json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "link": [{ "relation": "next", "url": "http://server/fhir/Patient?page=2" }],
        "entry": [
            { "resource": { "resourceType": "Patient", "id": "a" } },
            { "resource": { "resourceType": "Patient", "id": "b" } },
            { "resource": { "resourceType": "Observation", "id": "o",
                            "status": "final", "code": {} } }
        ]
    }))
    .expect("parse search bundle");

    println!("resources in bundle: {}", search.iter_resources().count());
    let patients: Vec<Patient> = search.resources::<Patient>("Patient").collect();
    println!("patients: {}", patients.len());
    if let Some(next) = search.next_link() {
        println!("next page: {next}");
    }
}
