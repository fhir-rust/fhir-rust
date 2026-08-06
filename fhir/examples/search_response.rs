//! Consume a FHIR search response: typed entries, the total, and paging.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example search_response
//! ```
//!
//! A search returns a `searchset` [`Bundle`]: a page of matches, a `total`
//! that counts the whole result set (not the page), and a `next` link when
//! there are more pages. This example parses one, pulls the `Patient` matches
//! out as typed values with [`Bundle::resources`], and walks the `next` links
//! with [`Bundle::next_link`] the way a client pages through a real server —
//! here the "server" is a closure handing out canned pages, so the example
//! runs offline.
//!
//! [`Bundle`]: fhir::r5::resources::Bundle
//! [`Bundle::resources`]: fhir::r5::resources::Bundle::resources
//! [`Bundle::next_link`]: fhir::r5::resources::Bundle::next_link

use fhir::r5::resources::{Bundle, Patient};

/// A pretend server: two pages of a Patient search, linked by `next`.
fn fetch(url: &str) -> Bundle {
    let page = match url {
        "https://example.org/fhir/Patient?name=chalmers" => serde_json::json!({
            "resourceType": "Bundle",
            "type": "searchset",
            "total": 3,
            "link": [
                { "relation": "self", "url": "https://example.org/fhir/Patient?name=chalmers" },
                { "relation": "next", "url": "https://example.org/fhir/Patient?name=chalmers&page=2" }
            ],
            "entry": [
                { "resource": { "resourceType": "Patient", "id": "pat-1", "active": true,
                                "name": [{ "family": "Chalmers", "given": ["Peter"] }] },
                  "search": { "mode": "match" } },
                { "resource": { "resourceType": "Patient", "id": "pat-2", "active": false,
                                "name": [{ "family": "Chalmers", "given": ["Jane"] }] },
                  "search": { "mode": "match" } }
            ]
        }),
        _ => serde_json::json!({
            "resourceType": "Bundle",
            "type": "searchset",
            "total": 3,
            "link": [
                { "relation": "self", "url": url }
            ],
            "entry": [
                { "resource": { "resourceType": "Patient", "id": "pat-3", "active": true,
                                "name": [{ "family": "Chalmers", "given": ["Ada"] }] },
                  "search": { "mode": "match" } }
            ]
        }),
    };
    serde_json::from_value(page).expect("parse searchset Bundle")
}

fn main() {
    let mut url = String::from("https://example.org/fhir/Patient?name=chalmers");
    let mut seen = 0usize;

    loop {
        let page = fetch(&url);

        // `total` counts the whole result set; the page holds a slice of it.
        println!(
            "page: {} of {} total match(es)",
            page.entry.len(),
            page.total.as_ref().map_or(0, |t| t.0),
        );

        // Typed extraction: only entries whose resourceType matches parse; a
        // searchset may also carry `include` entries of other types, and an
        // `OperationOutcome` describing warnings, which this skips over.
        for patient in page.resources::<Patient>("Patient") {
            seen += 1;
            let family = patient
                .name
                .first()
                .and_then(|n| n.family.as_ref())
                .map_or("?", |f| f.0.as_str());
            println!(
                "- Patient {} ({}, active: {:?})",
                patient.id.as_ref().map_or("?", |id| &id.0),
                family,
                patient.active.map(|b| b.0),
            );
        }

        // Follow `next` until the server stops offering one.
        match page.next_link() {
            Some(next) => url = next.to_owned(),
            None => break,
        }
    }

    println!("walked every page: {seen} patients");
    assert_eq!(seen, 3);
}
