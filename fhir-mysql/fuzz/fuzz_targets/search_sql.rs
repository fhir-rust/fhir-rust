//! Search parameters arrive from the network; the SQL they compile to must
//! never contain them (spec T11.9, A7.11).
//!
//! `build_search_sql` turns `?name=x&birthdate=gt2020` into SQL plus a bind
//! list. The security property is that every attacker-controlled value ends
//! up in `binds`, never spliced into `sql` — that separation is what makes
//! injection impossible, and it is a property a fuzzer can check directly
//! rather than a thing to be careful about.
//!
//! It also checks the obvious: no panic, no unwrap on malformed input, and
//! no unbounded recursion. A search endpoint that can be crashed by a query
//! string is a denial of service on a server holding clinical data.
#![no_main]

use libfuzzer_sys::fuzz_target;
use std::sync::OnceLock;

use fhir_mysql_map::model::RelMap;

/// A committed two-resource map, so the target needs no specification
/// download and no database. Resolved from `CARGO_MANIFEST_DIR`, never an
/// absolute path outside the repository (T11.12).
fn map() -> &'static RelMap {
    static MAP: OnceLock<RelMap> = OnceLock::new();
    MAP.get_or_init(|| {
        serde_json::from_str(include_str!("../fixtures/relmap_r4.json")).expect("fixture parses")
    })
}

/// Characters that end a SQL string literal or start a comment. A value
/// containing one of these appearing verbatim in the SQL is the signature of
/// an injection; a value without one could coincide with a column name.
fn is_dangerous(value: &str) -> bool {
    value.contains('\'') || value.contains(';') || value.contains("--") || value.contains('"')
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // Each line is one `name=value` pair, the shape a query string decodes to.
    let params: Vec<(String, String)> = text
        .lines()
        .take(32)
        .filter_map(|line| {
            let (k, v) = line.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect();
    if params.is_empty() {
        return;
    }

    let map = map();
    let Some(rm) = map.resources.get("Patient") else {
        return;
    };

    // Sort keys are attacker-controlled too — `?_sort=whatever`.
    let sort: Vec<fhir_mysql_store::search::SortKey> = params
        .iter()
        .take(2)
        .map(|(k, _)| fhir_mysql_store::search::SortKey {
            code: k.clone(),
            descending: k.starts_with('-'),
        })
        .collect();

    let Ok(query) = fhir_mysql_store::search::build_search_sql(
        map,
        rm,
        &params,
        50,
        0,
        &sort,
        params.first().map(|(_, v)| v.as_str()),
    ) else {
        // Rejecting a query is the correct outcome for most inputs.
        return;
    };

    for (_, value) in &params {
        if value.len() < 4 || !is_dangerous(value) {
            continue;
        }
        assert!(
            !query.sql.contains(value.as_str()),
            "a search value reached the SQL instead of the bind list:\n  \
             value: {value:?}\n  sql: {}",
            query.sql
        );
        assert!(
            !query.count_sql.contains(value.as_str()),
            "a search value reached the count SQL instead of the bind list:\n  \
             value: {value:?}"
        );
    }
});
