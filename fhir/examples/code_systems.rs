//! Use FHIR R5 code systems as type-safe Rust enums.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example code_systems
//! ```
//!
//! Every FHIR `CodeSystem` is generated into an enum under [`fhir::r5::codes`].
//! Each variant serializes to its canonical FHIR code string, so you get
//! compile-time checking and editor autocomplete instead of stringly-typed
//! codes, while the wire format stays exactly as FHIR specifies.

use fhir::r5::codes::{AdministrativeGender, ObservationStatus};

fn main() {
    // Enum -> canonical code string.
    let gender = AdministrativeGender::Female;
    println!(
        "AdministrativeGender::Female => {}",
        serde_json::to_value(&gender).unwrap()
    );

    // Canonical code string -> enum.
    let status: ObservationStatus =
        serde_json::from_value("final".into()).expect("known observation status");
    println!("\"final\" => {status:?}");

    // Unknown codes fail to parse, catching typos at the boundary.
    let bad: Result<ObservationStatus, _> = serde_json::from_value("finel".into());
    println!("\"finel\" parses? {}", bad.is_ok());
}
