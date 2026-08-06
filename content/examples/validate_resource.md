# `validate_resource` — Validate FHIR R5 values with the [`Validate`] trait

Run with:

```sh
cargo run --example validate_resource
```

Validation is *recursive*: calling `.validate()` on a resource walks every
field, so primitive-format problems anywhere in the tree are reported with a
dotted `path` that shows where they occurred.

[`Validate`]: fhir::r5::validate::Validate

## The program

```rust
use fhir::r5::coded::Coded;
use fhir::r5::codes::AdministrativeGender;
use fhir::r5::resources::Patient;
use fhir::r5::types::{HumanName, Id, String as FhirString, Uri};
use fhir::r5::validate::Validate;

fn main() {
    // --- Primitive-level checks -------------------------------------------
    // `Id` must match FHIR's `[A-Za-z0-9-.]{1,64}` constraint.
    println!(
        "`patient-1` valid id? {}",
        Id("patient-1".to_string()).is_valid()
    );
    println!(
        "`has spaces` valid id? {}",
        Id("has spaces".to_string()).is_valid()
    );

    // --- A valid resource -------------------------------------------------
    let mut patient = Patient {
        id: Some(FhirString("pat-1".to_string())),
        // A required-binding coded field is the code enum, wrapped in `Coded`.
        gender: Some(Coded::Known(AdministrativeGender::Male)),
        name: vec![HumanName {
            family: Some(FhirString("Chalmers".to_string())),
            ..Default::default()
        }],
        ..Default::default()
    };
    report(&patient);

    // --- Introduce a problem ----------------------------------------------
    // A FHIR `uri` must not be empty or surrounded by whitespace. Because the
    // check is recursive, the reported path points at the offending field.
    patient.implicit_rules = Some(Uri(" http://example.org/bad ".to_string()));
    report(&patient);
}

/// Print each resource's validation result.
fn report(patient: &Patient) {
    let issues = patient.validate();
    if issues.is_empty() {
        println!("valid ✓");
    } else {
        println!("{} issue(s):", issues.len());
        for issue in issues {
            println!("  - {}: {}", issue.path, issue.message);
        }
    }
}
```

*Source: [`fhir/examples/validate_resource.rs`](../fhir/examples/validate_resource.rs) in the repository.*
