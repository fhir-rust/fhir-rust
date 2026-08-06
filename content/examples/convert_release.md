# `convert_release` — Convert a resource between FHIR releases, and read the loss report

Run with:

```sh
cargo run --example convert_release --features "r4 r5"
```

[`fhir::convert::between`] converts a document's JSON wire form from one
release to another, driven by both releases' element metadata, and returns
a [`LossReport`] naming everything it changed or discarded. The report is
the point: serde alone would silently drop an element the target release
does not have, which is the failure mode a converter exists to prevent.

This example converts an R5 `Observation` down to R4 twice — once with a
shape both releases agree on (the report comes back empty, and that
emptiness is an assertion), and once carrying `triggeredBy`, an element R5
added, so the downgrade has something to report.

[`fhir::convert::between`]: fhir::convert::between
[`LossReport`]: fhir::convert::LossReport

## The program

```rust
use fhir::convert;
use fhir::r4::R4;
use fhir::r5::R5;

fn main() {
    // A wire document, as it might arrive from an R5 server. `between` works
    // on the JSON form, so nothing needs to be deserialized first — and the
    // `resourceType` tag in the document is what it converts by.
    let stable = serde_json::json!({
        "resourceType": "Observation",
        "id": "temp-1",
        "status": "final",
        "code": { "text": "Body temperature" },
        "valueQuantity": { "value": 36.6, "unit": "C" }
    });

    // R5 -> R4, on a shape the two releases agree about.
    let down = convert::between::<R5, R4>(&stable);
    assert!(down.report.is_lossless(), "unexpected loss:\n{}", down.report);
    println!("stable Observation: converted losslessly");

    // The result is ready for the target release's model.
    let as_r4: fhir::r4::resources::Resource =
        serde_json::from_value(down.value).expect("valid R4 after conversion");
    if let fhir::r4::resources::Resource::Observation(obs) = as_r4 {
        println!("R4 status: {}", obs.status.code());
    }

    // The same Observation carrying `triggeredBy` — an element R5 added, which
    // R4 has no place for. The conversion still succeeds; the report says what
    // it had to drop, instead of dropping it silently.
    let mut r5_only = stable.clone();
    r5_only["triggeredBy"] = serde_json::json!([
        { "observation": { "reference": "Observation/base-1" }, "type": "reflex" }
    ]);

    let lossy = convert::between::<R5, R4>(&r5_only);
    assert!(!lossy.report.is_lossless());
    println!("\nR5-only element on the way to R4 — the report:");
    println!("{}", lossy.report);

    // `discarded_data` separates real loss from warnings about data kept.
    println!(
        "discarded data: {} ({} loss(es) recorded)",
        lossy.report.discarded_data(),
        lossy.report.len(),
    );
}
```

*Source: [`fhir/examples/convert_release.rs`](../fhir/examples/convert_release.rs) in the repository.*
