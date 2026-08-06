# `tutorial` — The guide's end-to-end tutorial, as a runnable program

Run with:

```sh
cargo run --example tutorial
```

This is the code from the book's
[Tutorial](https://github.com/fhir-rust/fhir-rust/blob/main/book/src/tutorial.md)
chapter, kept here so that it is compiled and executed by the test suite
rather than merely written down. If you change one, change the other.

## The program

```rust
use fhir::r5::coded::Coded;
use fhir::r5::codes::{AdministrativeGender, ContactPointSystem, ObservationStatus};
use fhir::r5::resources::observation::ObservationValue;
use fhir::r5::resources::operation_outcome::OperationOutcome;
use fhir::r5::resources::{Bundle, Observation, Patient, Resource};
use fhir::r5::summary::to_summary_value;
use fhir::r5::types::{
    Boolean, Code, CodeableConcept, Coding, ContactPoint, Date, Decimal, HumanName, Quantity,
    Reference, String as FhirString, Uri,
};
use fhir::r5::validate::Validate;

fn main() {
    // 1. A patient -----------------------------------------------------------
    let patient = Patient::builder()
        .id(FhirString("pat-1".to_string()))
        .active(Boolean(true))
        .gender(Coded::Known(AdministrativeGender::Female))
        .birth_date(Date("1974-12-25".to_string()))
        .name(vec![HumanName {
            family: Some(FhirString("Chalmers".to_string())),
            given: vec![FhirString("Jane".to_string())],
            ..Default::default()
        }])
        .telecom(vec![ContactPoint {
            system: Some(Coded::Known(ContactPointSystem::Phone)),
            value: Some(FhirString("+1 555 0100".to_string())),
            ..Default::default()
        }])
        .build()
        .expect("no required fields");

    // 2. Validate before you send -------------------------------------------
    assert!(patient.validate().is_empty());

    let mut broken = patient.clone();
    broken.implicit_rules = Some(Uri(" http://bad ".to_string()));
    let issues = broken.validate();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].path, "implicit_rules.uri");

    let outcome: OperationOutcome = broken.validate().into();
    assert_eq!(outcome.issue.len(), 1);

    // 3. An observation about that patient ----------------------------------
    // `status` and `code` are 1..1, so the builder refuses an empty one.
    assert!(Observation::builder().build().is_err());

    let body_weight = CodeableConcept {
        coding: vec![Coding {
            system: Some(Uri("http://loinc.org".to_string())),
            code: Some(Code("29463-7".to_string())),
            display: Some(FhirString("Body weight".to_string())),
            ..Default::default()
        }],
        ..Default::default()
    };

    let observation = Observation::builder()
        .id(FhirString("obs-1".to_string()))
        .status(Coded::Known(ObservationStatus::Final))
        .code(body_weight)
        .subject(Reference {
            reference: Some(FhirString("Patient/pat-1".to_string())),
            ..Default::default()
        })
        .value(ObservationValue::Quantity(Box::new(Quantity {
            // Decimals are written as the lexeme they mean: "72.5" claims
            // three significant figures and comes back out as `72.5`, not
            // as whatever an f64 round trip would make of it.
            value: Some(Decimal::new("72.5").expect("a FHIR decimal")),
            unit: Some(FhirString("kg".to_string())),
            system: Some(Uri("http://unitsofmeasure.org".to_string())),
            code: Some(Code("kg".to_string())),
            ..Default::default()
        })))
        .build()
        .expect("status and code are set");

    // 4. Serialize to canonical FHIR JSON ------------------------------------
    let json = serde_json::to_string_pretty(&observation).expect("serialize");
    println!("{json}\n");
    // The choice element became the single key `valueQuantity`.
    assert!(json.contains("\"valueQuantity\""));

    // 5. Put both in a bundle -----------------------------------------------
    let bundle = Bundle::transaction()
        .create(&patient, "Patient")
        .create(&observation, "Observation")
        .build();
    assert_eq!(bundle.entry.len(), 2);

    for resource in bundle.iter_resources() {
        match resource {
            Resource::Patient(p) => println!("patient {:?}", p.id),
            Resource::Observation(o) => println!("observation {:?}", o.id),
            other => println!("something else: {other:?}"),
        }
    }

    let patients: Vec<Patient> = bundle.resources::<Patient>("Patient").collect();
    assert_eq!(patients.len(), 1);

    // 6. Sending it needs the `client` feature; see examples/client_crud.rs.

    // 7. Ask for less: the summary view --------------------------------------
    let summary = to_summary_value(&patient, "Patient");
    assert!(summary.get("gender").is_some()); // isSummary
    assert!(summary.get("photo").is_none()); // not isSummary
    println!("\nsummary keys: {:?}", summary.as_object().map(|o| o.len()));

    // 8. Round-trip, always --------------------------------------------------
    let value = serde_json::to_value(&observation).expect("to_value");
    let parsed: Observation = serde_json::from_value(value).expect("from_value");
    assert_eq!(parsed, observation);
    println!("round-trip: ok");
}
```

*Source: [`fhir/examples/tutorial.rs`](../fhir/examples/tutorial.rs) in the repository.*
