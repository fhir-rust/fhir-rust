//! A R6 (6.0.0-ballot3) resource that parses must serialize and re-parse identically.
//!
//! Each release has its own generated structs, choice enums, and code
//! enums, so serde behaviour is genuinely per-release — a `value[x]` variant
//! or a rename that round-trips in R5 says nothing about r6.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // Two entry points, because they are protected differently: `from_str`
    // gets serde_json's own depth limiting, `from_value` does not.
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
        let _ = serde_json::from_value::<fhir::r6::resources::Resource>(v);
    }
    let Ok(resource) = serde_json::from_str::<fhir::r6::resources::Resource>(text) else {
        return;
    };
    let once = serde_json::to_value(&resource).expect("a parsed resource must serialize");
    let again: fhir::r6::resources::Resource =
        serde_json::from_value(once.clone()).expect("its own output must re-parse");
    let twice = serde_json::to_value(&again).expect("and serialize again");
    assert_eq!(once, twice, "serialization is not stable across a round trip");
});
