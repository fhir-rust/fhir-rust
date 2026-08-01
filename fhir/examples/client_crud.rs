//! FHIR REST client CRUD against the public HAPI test server (feature `client`).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example client_crud --features client
//! ```
//!
//! This talks to the shared public HAPI R5 test server, so it needs network
//! access. It creates a Patient, reads it back, searches, fetches the server's
//! capabilities, then deletes the Patient it created.

#[cfg(feature = "client")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use fhir::client::Client;
    use fhir::r5::resources::{Patient, Resource};
    use fhir::r5::types::String as FhirString;

    let client = Client::new("https://hapi.fhir.org/baseR5");

    // Create a Patient.
    let patient = Patient::builder()
        .active(fhir::r5::types::Boolean(true))
        .name(vec![fhir::r5::types::HumanName {
            family: Some(FhirString("Testalot".to_string())),
            ..Default::default()
        }])
        .build()?;
    let created = client.create("Patient", &patient).await?;
    let id = match &created {
        Resource::Patient(p) => p.id.clone().map(|i| i.0).unwrap_or_default(),
        _ => return Err("expected a Patient back".into()),
    };
    println!("created Patient/{id}");

    // Read it back.
    let read = client.read("Patient", &id).await?;
    println!("read: {}", matches!(read, Resource::Patient(_)));

    // Search and fetch capabilities.
    let bundle = client.search("Patient", &[("_count", "1")]).await?;
    println!(
        "search returned {} entries",
        bundle.iter_resources().count()
    );
    let caps = client.capabilities().await?;
    println!("server software present: {}", caps.software.is_some());

    // Clean up.
    client.delete("Patient", &id).await?;
    println!("deleted Patient/{id}");
    Ok(())
}

#[cfg(not(feature = "client"))]
fn main() {
    eprintln!(
        "This example requires the `client` feature: cargo run --example client_crud --features client"
    );
}
