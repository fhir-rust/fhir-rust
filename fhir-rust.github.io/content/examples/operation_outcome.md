# `operation_outcome` — Turn validation results into a FHIR® `OperationOutcome`

Run with:

```sh
cargo run --example operation_outcome
```

`Resource::validate()` returns a `Vec<ValidationIssue>`. A FHIR server that
rejects a request reports problems as an `OperationOutcome`, and this crate
provides a `From<Vec<ValidationIssue>>` bridge so validation output drops
straight into that resource.

## The program

```rust
use fhir::r5::coded::Coded;
use fhir::r5::resources::Patient;
use fhir::r5::resources::operation_outcome::OperationOutcome;
use fhir::r5::types::Uri;
use fhir::r5::validate::Validate;

fn main() {
    // A patient with two problems: an out-of-value-set gender code, and a `uri`
    // surrounded by whitespace.
    let mut patient = Patient {
        gender: Some(Coded::Unknown("robot".to_string())),
        ..Default::default()
    };
    patient.implicit_rules = Some(Uri(" http://example.org/bad ".to_string()));

    let issues = patient.validate();
    println!("{} validation issue(s)", issues.len());

    // Bridge the issues into an OperationOutcome and serialize it.
    let outcome: OperationOutcome = issues.into();
    let json = serde_json::to_string_pretty(&outcome).expect("serialize OperationOutcome");
    println!("{json}");

    // Every entry is an `error`-severity, `invalid`-code issue.
    for issue in &outcome.issue {
        assert_eq!(
            issue.severity,
            Coded::Known(fhir::r5::codes::IssueSeverity::Error)
        );
        let Some(path) = issue.expression.first_value().map(|p| &p.0) else {
            continue;
        };
        let msg = issue.diagnostics.as_ref().unwrap();
        println!("- {path}: {}", msg.0);
    }
}
```

*Source: [`fhir/examples/operation_outcome.rs`](../fhir/examples/operation_outcome.rs) in the repository.*

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7.
