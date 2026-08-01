//! Hostile XML nesting is refused, not fatal (spec R13.4).
//!
//! The XML reader descends once per nested element via a mutual recursion
//! between `read_children` and `read_element`. It had no bound, so the
//! document supplied its own recursion depth: at roughly 2,700 levels —
//! about 160 KB of XML, well under any sane request-size limit — it
//! overflowed the stack and the process aborted.
//!
//! That is worse than a panic. A stack overflow in Rust is not unwindable:
//! `catch_unwind` does not see it, a worker thread cannot contain it, and the
//! whole process dies. For a crate whose job is parsing documents that
//! arrive over a network, one request ending the process is a denial of
//! service that needs no cleverness to trigger.
//!
//! The JSON path never had this problem, because `serde_json` applies its own
//! 128-level limit while parsing. The XML path built its `Value` in this
//! crate's own code and so bypassed that protection entirely — the kind of
//! gap that opens when two entry points to the same model are written at
//! different times.
//!
//! These tests run on the **default stack**, deliberately. The round-trip
//! suites use a 64 MB thread, which would mask exactly the failure being
//! checked here.

// The depth bound lives in fhir-core and is release-independent, but the
// reader needs *a* metadata table and this uses R5's. Gating on `xml` alone
// let it compile under `--no-default-features --features "r4 xml"`, where
// `fhir::r5` does not exist.
#![cfg(all(feature = "xml", feature = "r5"))]

use fhir::r5::meta;
use fhir::xml::{MAX_DEPTH, XmlError};

/// FHIR XML nesting `Questionnaire.item` to the requested depth.
fn nested(depth: usize) -> String {
    let mut inner = String::from(r#"<linkId value="leaf"/><type value="string"/>"#);
    for _ in 0..depth {
        inner = format!(r#"<linkId value="n"/><type value="group"/><item>{inner}</item>"#);
    }
    format!(
        r#"<Questionnaire xmlns="http://hl7.org/fhir"><status value="active"/><item>{inner}</item></Questionnaire>"#
    )
}

#[test]
fn nesting_past_the_limit_is_an_error_not_an_abort() {
    // Far past the limit, and past the depth that used to abort. If the
    // bound regresses, this test does not fail — it takes the whole test
    // binary down with it, which is itself an unmistakable signal.
    for depth in [MAX_DEPTH + 1, 1_000, 5_000, 50_000] {
        let xml = nested(depth);
        match fhir::xml::xml_to_value(meta::elements(), &xml) {
            Err(XmlError::TooDeep { limit }) => assert_eq!(limit, MAX_DEPTH),
            Err(other) => panic!("depth {depth}: expected TooDeep, got {other}"),
            Ok(_) => panic!("depth {depth}: accepted a document nesting past {MAX_DEPTH}"),
        }
    }
}

#[test]
fn ordinary_nesting_still_parses() {
    // The bound must refuse hostile input without refusing real documents.
    // Published FHIR nests around 15 deep at the very most.
    let xml = nested(10);
    let value = fhir::xml::xml_to_value(meta::elements(), &xml)
        .expect("a 10-deep Questionnaire is ordinary and must parse");
    assert_eq!(value["resourceType"], "Questionnaire");
}

#[test]
fn the_limit_matches_the_json_path() {
    // Both entry points feed the same model, so accepting a document through
    // one and refusing it through the other would be a bug in itself — and
    // whichever is laxer becomes the way in.
    assert_eq!(
        MAX_DEPTH, 128,
        "serde_json bounds the JSON path at 128; the XML path must agree"
    );

    // Demonstrate the JSON side actually holds, rather than asserting it
    // from memory: serde_json counts each container, so a structure nesting
    // objects and arrays alternately hits the limit at half the FHIR depth.
    let mut json = String::from(r#"{"linkId":"leaf","type":"string"}"#);
    for _ in 0..200 {
        json = format!(r#"{{"linkId":"n","type":"group","item":[{json}]}}"#);
    }
    let json = format!(r#"{{"resourceType":"Questionnaire","status":"active","item":[{json}]}}"#);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json);
    assert!(
        parsed.is_err(),
        "serde_json accepted 200-deep JSON; the assumption behind MAX_DEPTH \
         no longer holds and the XML limit needs revisiting"
    );
}

/// Serializing is recursive too — `write_child` and `write_complex` call each
/// other once per level — and unlike the reader it has no bound.
///
/// It is not reachable with hostile depth today: every route into a resource
/// is now bounded. JSON is capped at 128 containers by `serde_json`, and XML
/// at [`MAX_DEPTH`] by the reader. So the deepest resource a program can hold
/// from untrusted input is one the writer provably survives — which is what
/// this test establishes, on the default stack.
///
/// If a future entry point (a database row, a binary format, a builder API
/// fed by user input) can produce a deeper resource, the writer needs its own
/// bound and `to_xml` needs to become fallible. This test is the tripwire:
/// it takes the maximum the reader now admits and pushes it back out.
#[test]
fn writing_back_what_the_reader_admits_is_safe() {
    // The reader counts a level for each of `read_children` and
    // `read_element`, so MAX_DEPTH of 128 admits about 63 nested elements.
    // Find the deepest document it actually accepts, then serialize it.
    let deepest = (1..80)
        .rev()
        .find_map(|d| {
            fhir::xml::xml_to_value(meta::elements(), &nested(d))
                .ok()
                .map(|v| (d, v))
        })
        .expect("some depth is accepted");
    let (depth, value) = deepest;
    assert!(
        depth > 10,
        "the reader admits only {depth} levels, which would reject ordinary \
         documents"
    );

    let xml = fhir::xml::to_xml(&value, "Questionnaire");
    assert!(
        xml.contains("Questionnaire"),
        "serializing the deepest admissible document must not be lossy"
    );
}
