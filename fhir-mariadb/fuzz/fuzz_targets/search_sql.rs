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
#![forbid(unsafe_code)]

use libfuzzer_sys::fuzz_target;
use std::sync::OnceLock;

use fhir_mariadb_map::model::RelMap;

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
    value.contains('\'')
        || value.contains(';')
        || value.contains("--")
        || value.contains('"')
        || value.contains('`')
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
    let sort: Vec<fhir_mariadb_store::mariadb_search::SortKey> = params
        .iter()
        .take(2)
        .map(|(k, _)| fhir_mariadb_store::mariadb_search::SortKey {
            code: k.clone(),
            descending: k.starts_with('-'),
        })
        .collect();

    let Ok(query) = fhir_mariadb_store::mariadb_search::build_search_sql(
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

    // Two oracles, each covering the other's blind spot.
    //
    // The old single oracle substring-searched each "dangerous" value in the
    // SQL, and ~a million executions minimized to `= p.` plus a quote — a value that is
    // a substring of the *structure* (the join predicate), a false positive
    // by construction.
    //
    // Oracle 1 — structural invariance: rebuild with every value replaced by
    // a *shape-preserving* sentinel (letters flattened, digits rotated,
    // punctuation and comparison prefixes kept, so the builder takes the
    // same branches) and require the SQL text to be identical: the text must
    // be a function of parameter names and value *shapes*, never value
    // content. Oracle 2 — differential leak check: a value appearing in the
    // real SQL but not the sentinel SQL got there from the value itself, not
    // from the structure.
    fn sentinel_of(v: &str) -> String {
        // The builder splits a value on commas (an OR-list) and branches per
        // segment, so the sentinel must too — CI's fuzzer found a mid-list
        // `urn:` segment (`link=1,…,urn:c,…`) flipping the URL-column branch
        // after the whole-value pass only preserved a *leading* prefix.
        fn sentinel_segment(v: &str) -> String {
            // Prefixes that steer the builder's branch and must survive the
            // flattening: comparison operators, and `urn:` (a reference value
            // that routes to the URL column — CI's fuzzer found the sentinel
            // flipping that branch and flagging a legitimate difference).
            const PREFIXES: [&str; 9] = ["ge", "le", "gt", "lt", "ne", "eq", "sa", "eb", "urn:"];
            let keep = PREFIXES
                .iter()
                .find(|p| v.starts_with(**p))
                .map_or(0, |p| p.len());
            v.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i < keep {
                        c
                    } else {
                        match c {
                            'a'..='z' | 'A'..='Z' => 'z',
                            '0'..='9' => {
                                char::from_digit((c.to_digit(10).unwrap() + 1) % 10, 10).unwrap()
                            }
                            other => other,
                        }
                    }
                })
                .collect()
        }
        v.split(',').map(sentinel_segment).collect::<Vec<_>>().join(",")
    }
    let replaced: Vec<(String, String)> = params
        .iter()
        .map(|(k, v)| (k.clone(), sentinel_of(v)))
        .collect();
    if let Ok(shadow) = fhir_mariadb_store::mariadb_search::build_search_sql(
        map,
        rm,
        &replaced,
        50,
        0,
        &sort,
        replaced.first().map(|(_, v)| v.as_str()),
    ) {
        assert!(
            query.sql == shadow.sql && query.count_sql == shadow.count_sql,
            "the SQL text depends on parameter values beyond their shape — an \
             injection shape:\n  with values:   {}\n  with sentinel: {}",
            query.sql,
            shadow.sql
        );
        for (_, value) in &params {
            if value.len() < 4 || !is_dangerous(value) {
                continue;
            }
            if query.sql.contains(value.as_str()) && !shadow.sql.contains(value.as_str()) {
                panic!(
                    "a search value reached the SQL instead of the bind list:\n  \
                     value: {value:?}\n  sql: {}",
                    query.sql
                );
            }
        }
    }
});
