//! Shredding accepts a resource straight off the wire (spec T11.9).
//!
//! `shred` walks arbitrary JSON against the relational map and produces rows.
//! It recurses through nested elements, so a document supplies its own
//! recursion depth — the same shape that made the sibling `fhir` crate's XML
//! reader abort the process on 160 KB of input. A stack overflow is not
//! unwindable: the server does not return a 400, it dies.
//!
//! Every input must therefore either shred or return an error. Nothing may
//! panic, and nothing may abort.
#![no_main]
#![forbid(unsafe_code)]

use libfuzzer_sys::fuzz_target;
use std::sync::OnceLock;

use fhir_sqlite_map::model::RelMap;

fn map() -> &'static RelMap {
    static MAP: OnceLock<RelMap> = OnceLock::new();
    MAP.get_or_init(|| {
        serde_json::from_str(include_str!("../fixtures/relmap_r4.json")).expect("fixture parses")
    })
}

fuzz_target!(|data: &[u8]| {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) else {
        return;
    };
    let map = map();
    let rt = value
        .get("resourceType")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("Patient");
    let Some(rm) = map.resources.get(rt) else {
        return;
    };
    let _ = fhir_sqlite_map::shred::shred(rm, &value);
});
