# Tutorial: a patient record end to end

The other chapters each explain one topic. This one builds a small but complete
record and carries it through every stage a real integration goes through:
construct, validate, serialize, bundle, summarize, and read back.

Everything here is R5. It works for R4 or R3 by replacing `r5` in the imports,
with the caveats in [FHIR releases](fhir-releases.md) — R3, for instance, types
a resource's `id` as an `Id` rather than a `String`.

Every step below is also a runnable program, so it is compiled and executed by
the test suite rather than merely written down:

```sh
cargo run --example tutorial
```

To follow along in your own scratch binary:

```sh
cargo new fhir-tutorial && cd fhir-tutorial
cargo add fhir serde_json
```

## 1. A patient

Start with the subject of care. Use the builder, which enforces mandatory
(`1..1`) fields at `build()` — `Patient` has none, so this cannot fail:

```rust
use fhir::r5::coded::Coded;
use fhir::r5::codes::{AdministrativeGender, ContactPointSystem};
use fhir::r5::resources::Patient;
use fhir::r5::types::{Boolean, ContactPoint, Date, HumanName, String as FhirString};

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
```

Three things to notice:

- **`gender` is not a string.** It is `Coded<AdministrativeGender>`, because
  FHIR binds that element to a value set with `required` strength. The compiler
  now rejects a typo, while unrecognized codes coming off the wire still parse
  (see [Terminology and codes](terminology-and-codes.md)).
- **`name` is a bare `Vec`, not `Option<Vec>`.** FHIR `0..*` maps to `Vec<T>`,
  empty when absent, so there is no `Option` to unwrap.
- **`birth_date` is a `Date` newtype over `String`.** FHIR dates are partial —
  `1974`, `1974-12`, and `1974-12-25` are all valid — so the crate keeps the
  written form and offers parsing on top (`Date::parse_parts`).

## 2. Validate before you send

```rust
use fhir::r5::validate::Validate;

let issues = patient.validate();
assert!(issues.is_empty());
```

`validate()` walks the whole value, so an invalid primitive deep inside a
nested element is reported with the path that leads to it:

```rust
use fhir::r5::types::Uri;

let mut broken = patient.clone();
broken.implicit_rules = Some(Uri(" http://bad ".to_string())); // stray whitespace

let issues = broken.validate();
assert_eq!(issues.len(), 1);
assert_eq!(issues[0].path, "implicit_rules.uri");
```

If you are answering an HTTP request, turn the issues straight into the
resource FHIR expects for errors:

```rust
use fhir::r5::resources::operation_outcome::OperationOutcome;

let outcome: OperationOutcome = broken.validate().into();
assert_eq!(outcome.issue.len(), 1);
```

## 3. An observation about that patient

Now something clinical. `Observation` *does* have mandatory fields — `status`
and `code` — and the builder will not let you forget them:

```rust
use fhir::r5::codes::ObservationStatus;
use fhir::r5::resources::Observation;
use fhir::r5::resources::observation::ObservationValue;
use fhir::r5::types::{CodeableConcept, Coding, Decimal, Quantity, Reference, Uri};

assert!(Observation::builder().build().is_err()); // status and code are 1..1

let body_weight = CodeableConcept {
    coding: vec![Coding {
        system: Some(Uri("http://loinc.org".to_string())),
        code: Some(fhir::r5::types::Code("29463-7".to_string())),
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
        value: Some(Decimal(serde_json::Number::from_f64(72.5).unwrap())),
        unit: Some(FhirString("kg".to_string())),
        system: Some(Uri("http://unitsofmeasure.org".to_string())),
        code: Some(fhir::r5::types::Code("kg".to_string())),
        ..Default::default()
    })))
    .build()
    .expect("status and code are set");
```

`value` is the interesting one. FHIR's `Observation.value[x]` may be a
`Quantity`, a `CodeableConcept`, a `string`, a `boolean`, and nine other things.
The crate models that as one enum, so **exactly one** is set by construction —
you cannot accidentally populate both a quantity and a string. See
[Model mapping](model-mapping.md).

## 4. Serialize to canonical FHIR JSON

```rust
let json = serde_json::to_string_pretty(&observation).unwrap();
```

```json
{
  "id": "obs-1",
  "status": "final",
  "code": { "coding": [ { "system": "http://loinc.org", "code": "29463-7", "display": "Body weight" } ] },
  "subject": { "reference": "Patient/pat-1" },
  "valueQuantity": { "value": 72.5, "unit": "kg", "system": "http://unitsofmeasure.org", "code": "kg" }
}
```

Absent optional fields are omitted, empty lists are omitted, keys are
camelCase, and the choice element became the single key `valueQuantity`. That
is canonical FHIR — nothing to post-process.

## 5. Put both in a bundle

```rust
use fhir::r5::resources::Bundle;

let bundle = Bundle::transaction()
    .create(&patient, "Patient")
    .create(&observation, "Observation")
    .build();

assert_eq!(bundle.entry.len(), 2);
```

Reading a bundle back, you usually do not know each entry's type ahead of time.
Deserialize into the polymorphic `Resource` enum and match:

```rust
use fhir::r5::resources::Resource;

for resource in bundle.iter_resources() {
    match resource {
        Resource::Patient(p) => println!("patient {:?}", p.id),
        Resource::Observation(o) => println!("observation {:?}", o.id),
        other => println!("something else: {other:?}"),
    }
}
```

Or pull out just one type:

```rust
let patients: Vec<Patient> = bundle.resources::<Patient>("Patient").collect();
assert_eq!(patients.len(), 1);
```

## 6. Send it

With the `client` feature:

```rust,ignore
use fhir::r5::client::Client;

let client = Client::new("https://hapi.fhir.org/baseR5");
let created = client.create("Patient", &patient).await?;
let found = client.search("Patient", &[("family", "Chalmers")]).await?;
```

Error responses come back as `ClientError::Outcome`, with the server's
`OperationOutcome` already parsed.

## 7. Ask for less: the summary view

FHIR servers support `_summary=true`, which returns only the elements marked
`isSummary` plus the mandatory ones. The crate can produce that view from the
element metadata:

```rust
use fhir::r5::summary::to_summary_value;

let summary = to_summary_value(&patient, "Patient");
assert!(summary.get("gender").is_some());   // isSummary
assert!(summary.get("photo").is_none());    // not isSummary
```

## 8. Round-trip, always

Every value in the model satisfies one rule: what you serialize, you can read
back unchanged.

```rust
let json = serde_json::to_value(&observation).unwrap();
let parsed: Observation = serde_json::from_value(json).unwrap();
assert_eq!(parsed, observation);
```

This is the property the crate's test suite checks against every official FHIR
example resource, and it is why nothing in the model silently drops a field.

## Where to go next

- [Model mapping](model-mapping.md) — why each FHIR shape became the Rust type
  it did.
- [Extensions](extensions.md) — reading and writing the data FHIR does not
  define.
- [FHIR releases](fhir-releases.md) — doing all of the above in R4, or in both.
- The runnable programs in
  [`examples/`](https://github.com/joelparkerhenderson/fhir-rust-crate/tree/main/examples),
  each of which expands on one step above.
