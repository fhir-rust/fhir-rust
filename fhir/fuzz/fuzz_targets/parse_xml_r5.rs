//! Adversarial XML must not crash the R5 (5.0.0) reader (spec R13.17).
//!
//! The reader is shared across releases but is driven by `r5`'s own
//! `ElementMeta` table: `element_meta`, `is_datatype`, and `coerce` all
//! branch on it, so a document that is harmless for one release can take a
//! different path here. Fuzzing R5 alone would not see it.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(value) = fhir::xml::xml_to_value(fhir::r5::meta::elements(), text) {
        // Whatever the reader admits, the writer must survive: a resource
        // that parses but cannot be serialized is a denial of service on the
        // way out.
        let _ = fhir::xml::to_xml(&value, "Resource");
    }
});
