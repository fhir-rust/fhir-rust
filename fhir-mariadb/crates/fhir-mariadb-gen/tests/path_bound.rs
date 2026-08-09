//! `U12a`: the recorded `path_bound` and its shred-time enforcement (F-47).
//!
//! The bound is a declared capacity limit, so the test that matters is
//! behavioural: a resource nested past it is refused loudly, and one
//! within it shreds untouched.

use fhir_mariadb_map::model::RelMap;
use fhir_mariadb_map::shred::shred;
use serde_json::json;

/// Every bundled release records the same shape of bound: at least the 128
/// floor, a multiple of 64, and identical on every resource map.
#[test]
fn every_bundled_map_records_the_bound() {
    for ver in RelMap::bundled_versions() {
        let m = RelMap::bundled(ver).expect("bundled");
        let bound = m.path_bound();
        assert!(bound >= 128, "{ver}: bound {bound} is below the floor");
        assert_eq!(bound % 64, 0, "{ver}: bound {bound} not a 64 multiple");
        for (name, rm) in &m.resources {
            assert_eq!(rm.path_bound, bound, "{ver}/{name}: differing bound");
        }
    }
}

/// Eight levels of cyclic nesting are guaranteed to fit (`U12a`'s cycle
/// cap), and nesting past the bound is refused loudly, never truncated.
#[test]
fn nesting_is_capped_loudly_at_the_bound() {
    let Ok(m) = RelMap::bundled("r5") else {
        panic!("r5 map required for this test");
    };
    let rm = m.resources.get("QuestionnaireResponse").expect("QR");

    let deep_item = |levels: usize| {
        let mut item = json!({"linkId": "leaf",
            "extension": [{"url": "http://x.example/e", "valueString": "v"}]});
        for i in (0..levels).rev() {
            item = json!({"linkId": format!("l{i}"), "item": [item]});
        }
        json!({
            "resourceType": "QuestionnaireResponse", "id": "q",
            "status": "completed", "questionnaire": "http://x.example/q",
            "item": [item]
        })
    };

    shred(rm, &deep_item(8)).expect("eight levels fit (U12a)");

    let levels = (rm.path_bound as usize / 4) + 8;
    let err = shred(rm, &deep_item(levels)).expect_err("over-bound nesting");
    let msg = err.to_string();
    assert!(
        msg.contains("capacity"),
        "the refusal must say it is a capacity limit: {msg}"
    );
}
