//! SNOMED CT® terminology operations (SV2.20), against a real release.
//!
//! The release is a hand-written RF2 Snapshot of three well-known top-level
//! concepts — no licensed SNOMED CT content is committed or needed. It is
//! loaded through the same `terminology::load` the boot uses, so these tests
//! exercise the real `snomed-fhir` semantics, not a mock.

use std::sync::OnceLock;

use fhir_loco::app::App;
use loco_rs::testing::prelude::*;
use serde_json::Value;
use serial_test::serial;
use snomed::core::sctid::{ComponentType, SctId};

use super::fhir::{bearer, body_of, store_ready};

const VERSION: &str = "http://snomed.info/sct/900000000000207008/version/20190731";
const ROOT: &str = "138875005";
const FINDING: &str = "404684003";
const DISEASE: &str = "64572001";
const SYSTEM: &str = "http://snomed.info/sct";

fn write(root: &std::path::Path, rel: &str, content: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, content).expect("write fixture");
}

/// Root ← Clinical finding ← Disease, with FSNs, one preferred synonym in
/// the US English language refset, and inferred IS-A rows.
fn write_release(root: &std::path::Path) {
    let module = "900000000000207008";
    write(
        root,
        "Snapshot/Terminology/sct2_Concept_Snapshot_INT_20190731.txt",
        &format!(
            "id\teffectiveTime\tactive\tmoduleId\tdefinitionStatusId\n\
             {ROOT}\t20190731\t1\t{module}\t900000000000074008\n\
             {FINDING}\t20190731\t1\t{module}\t900000000000074008\n\
             {DISEASE}\t20190731\t1\t{module}\t900000000000074008\n"
        ),
    );
    let desc = |item: u64| SctId::compose(item, ComponentType::Description, None).unwrap();
    let fsn = "900000000000003001";
    let synonym = "900000000000013009";
    let cs = "900000000000448009";
    write(
        root,
        "Snapshot/Terminology/sct2_Description_Snapshot-en_INT_20190731.txt",
        &format!(
            "id\teffectiveTime\tactive\tmoduleId\tconceptId\tlanguageCode\ttypeId\tterm\tcaseSignificanceId\n\
             {}\t20190731\t1\t{module}\t{ROOT}\ten\t{fsn}\tSNOMED CT Concept (SNOMED RT+CTV3)\t{cs}\n\
             {}\t20190731\t1\t{module}\t{FINDING}\ten\t{fsn}\tClinical finding (finding)\t{cs}\n\
             {}\t20190731\t1\t{module}\t{FINDING}\ten\t{synonym}\tClinical finding\t{cs}\n\
             {}\t20190731\t1\t{module}\t{DISEASE}\ten\t{fsn}\tDisease (disorder)\t{cs}\n",
            desc(1001),
            desc(1002),
            desc(1004),
            desc(1003),
        ),
    );
    write(
        root,
        "Snapshot/Refset/Language/der2_cRefset_LanguageSnapshot-en_INT_20190731.txt",
        &format!(
            "id\teffectiveTime\tactive\tmoduleId\trefsetId\treferencedComponentId\tacceptabilityId\n\
             80000000-0000-4000-8000-000000000001\t20190731\t1\t{module}\t900000000000509007\t{}\t900000000000548007\n",
            desc(1004),
        ),
    );
    let rel = |item: u64| SctId::compose(item, ComponentType::Relationship, None).unwrap();
    write(
        root,
        "Snapshot/Terminology/sct2_Relationship_Snapshot_INT_20190731.txt",
        &format!(
            "id\teffectiveTime\tactive\tmoduleId\tsourceId\tdestinationId\trelationshipGroup\ttypeId\tcharacteristicTypeId\tmodifierId\n\
             {}\t20190731\t1\t{module}\t{FINDING}\t{ROOT}\t0\t116680003\t900000000000011006\t900000000000451002\n\
             {}\t20190731\t1\t{module}\t{DISEASE}\t{FINDING}\t0\t116680003\t900000000000011006\t900000000000451002\n",
            rel(1001),
            rel(1002),
        ),
    );
}

/// The store, plus the release — once per binary, like the store itself.
async fn terminology_ready() {
    store_ready().await;
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        let dir =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-scratch/snomed");
        let _ = std::fs::remove_dir_all(&dir);
        write_release(&dir);
        let language = SctId::parse("900000000000509007").ok();
        let release = fhir_loco::terminology::load(&dir, Some(VERSION.to_string()), language)
            .expect("the fixture release loads");
        fhir_loco::terminology::install(release).expect("installed once");
    });
}

/// A query string, encoded the way a client would.
fn query(pairs: &[(&str, &str)]) -> String {
    serde_urlencoded::to_string(pairs).expect("encodes")
}

/// `GET` with a credential: `(status, body)`. A macro because the test
/// server's type is `axum_test`'s, which Loco does not re-export by name.
macro_rules! get {
    ($request:expr, $path:expr, $pairs:expr $(,)?) => {{
        let res = $request
            .get(&format!("{}?{}", $path, query($pairs)))
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(
            res.headers()
                .get("content-type")
                .map(|v| v.to_str().unwrap()),
            Some("application/fhir+json"),
            "{}",
            $path
        );
        (res.status_code().as_u16(), body_of(&res.text()))
    }};
}

/// A refusal case: the query, the status it must get, and text its
/// diagnostics must contain.
type Case<'a> = (Vec<(&'a str, &'a str)>, u16, &'a str);

fn parameter<'a>(body: &'a Value, name: &str) -> Vec<&'a Value> {
    body["parameter"]
        .as_array()
        .expect("Parameters.parameter")
        .iter()
        .filter(|p| p["name"] == name)
        .collect()
}

fn diagnostics(body: &Value) -> &str {
    assert_eq!(body["resourceType"], "OperationOutcome", "{body}");
    body["issue"][0]["diagnostics"]
        .as_str()
        .expect("diagnostics")
}

#[tokio::test]
#[serial]
async fn lookup_answers_parameters_for_a_loaded_concept() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let (status, body) = get!(
            request,
            "/r5/CodeSystem/$lookup",
            &[
                ("system", SYSTEM),
                ("code", FINDING),
                ("property", "parent"),
                ("property", "inactive"),
            ],
        );
        assert_eq!(status, 200, "{body}");
        assert_eq!(body["resourceType"], "Parameters");
        assert_eq!(parameter(&body, "name")[0]["valueString"], "SNOMED CT");
        assert_eq!(parameter(&body, "version")[0]["valueString"], VERSION);
        // The configured language refset's preferred synonym, not the FSN.
        assert_eq!(
            parameter(&body, "display")[0]["valueString"],
            "Clinical finding"
        );
        let preferred = parameter(&body, "designation")
            .into_iter()
            .find(|d| d["part"][2]["valueString"] == "Clinical finding")
            .expect("the synonym is a designation");
        assert_eq!(
            preferred["part"][1]["valueCoding"]["code"],
            "900000000000548007"
        );
        let properties: Vec<(String, Value)> = parameter(&body, "property")
            .into_iter()
            .map(|p| {
                (
                    p["part"][0]["valueCode"].as_str().unwrap().to_string(),
                    p["part"][1].clone(),
                )
            })
            .collect();
        assert!(
            properties.contains(&(
                "parent".to_string(),
                serde_json::json!({"name": "value", "valueCode": ROOT})
            )),
            "{properties:?}"
        );
        assert!(
            properties.contains(&(
                "inactive".to_string(),
                serde_json::json!({"name": "value", "valueBoolean": false})
            )),
            "{properties:?}"
        );

        // The matching version is served.
        let (status, _) = get!(
            request,
            "/r5/CodeSystem/$lookup",
            &[("system", SYSTEM), ("code", FINDING), ("version", VERSION)],
        );
        assert_eq!(status, 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn lookup_refusals_name_the_parameter_never_the_value() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let unknown = SctId::compose(9999, ComponentType::Concept, None)
            .unwrap()
            .to_string();
        let cases: Vec<Case> = vec![
            (
                vec![("system", SYSTEM), ("code", &unknown)],
                404,
                "not a concept in the loaded",
            ),
            (
                vec![("system", SYSTEM), ("code", "12345")],
                400,
                "`code` is not a valid SNOMED CT identifier",
            ),
            (vec![("code", FINDING)], 400, "`system` is required"),
            (
                vec![("system", "http://loinc.org"), ("code", FINDING)],
                400,
                "the only code system served",
            ),
            (
                vec![("system", SYSTEM), ("code", FINDING), ("code", ROOT)],
                400,
                "`code` may appear only once",
            ),
            (
                vec![
                    ("system", SYSTEM),
                    ("code", FINDING),
                    ("displayLanguage", "en-US"),
                ],
                400,
                "`displayLanguage`",
            ),
            (
                vec![
                    ("system", SYSTEM),
                    ("code", FINDING),
                    ("property", "normalForm"),
                ],
                400,
                "computes no classification",
            ),
            (
                vec![
                    ("system", SYSTEM),
                    ("code", FINDING),
                    ("property", "colour"),
                ],
                400,
                "a requested `property` is not supported",
            ),
            (
                vec![
                    ("system", SYSTEM),
                    ("code", FINDING),
                    (
                        "version",
                        "http://snomed.info/sct/900000000000207008/version/20200131",
                    ),
                ],
                404,
                "not the loaded release",
            ),
        ];
        for (pairs, want, text) in cases {
            let (status, body) = get!(request, "/r5/CodeSystem/$lookup", &pairs);
            let said = diagnostics(&body);
            assert_eq!(status, want, "{pairs:?}: {said}");
            assert!(said.contains(text), "{pairs:?}: {said}");
            // SV2.7: no submitted value is echoed.
            for (name, value) in &pairs {
                // `system` and `normalForm` are the server's own vocabulary,
                // named in its own refusals.
                if !(*name == "system" && *value == SYSTEM) && *value != "normalForm" {
                    assert!(!said.contains(value), "{said} echoes {value}");
                }
            }
        }
    })
    .await;
}

#[tokio::test]
#[serial]
async fn subsumes_reports_each_outcome() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        for (a, b, want) in [
            (ROOT, DISEASE, "subsumes"),
            (DISEASE, ROOT, "subsumed-by"),
            (FINDING, FINDING, "equivalent"),
        ] {
            let (status, body) = get!(
                request,
                "/r5/CodeSystem/$subsumes",
                &[("system", SYSTEM), ("codeA", a), ("codeB", b)],
            );
            assert_eq!(status, 200, "{body}");
            assert_eq!(
                parameter(&body, "outcome")[0]["valueCode"],
                want,
                "{a} vs {b}"
            );
        }
        let (status, body) = get!(
            request,
            "/r5/CodeSystem/$subsumes",
            &[("system", SYSTEM), ("codeA", ROOT)],
        );
        assert_eq!(status, 400);
        assert!(diagnostics(&body).contains("`codeB` is required"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn expand_serves_implicit_value_sets_and_pages_them() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let isa = format!("{SYSTEM}?fhir_vs=isa/{FINDING}");
        let (status, body) = get!(request, "/r5/ValueSet/$expand", &[("url", &isa)]);
        assert_eq!(status, 200, "{body}");
        assert_eq!(body["resourceType"], "ValueSet");
        assert_eq!(body["url"], isa.as_str());
        assert_eq!(body["expansion"]["total"], 2);
        let codes: Vec<&str> = body["expansion"]["contains"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["code"].as_str().unwrap())
            .collect();
        assert_eq!(codes, vec![DISEASE, FINDING], "sorted by SCTID");
        assert_eq!(body["expansion"]["contains"][0]["inactive"], false);
        assert!(body["expansion"]["timestamp"].is_string());

        // ECL, percent-encoded inside the url; paging.
        let ecl = format!("{SYSTEM}?fhir_vs=ecl/%3C%3C{FINDING}");
        let (status, body) = get!(
            request,
            "/r5/ValueSet/$expand",
            &[("url", &ecl), ("count", "1"), ("offset", "1")],
        );
        assert_eq!(status, 200, "{body}");
        assert_eq!(body["expansion"]["total"], 2);
        assert_eq!(body["expansion"]["offset"], 1);
        assert_eq!(body["expansion"]["contains"].as_array().unwrap().len(), 1);

        // The configured version's url is served; designations on request.
        let versioned = format!("{VERSION}?fhir_vs=isa/{FINDING}");
        let (status, body) = get!(
            request,
            "/r5/ValueSet/$expand",
            &[
                ("url", &versioned),
                ("includeDesignations", "true"),
                ("filter", "disease"),
            ],
        );
        assert_eq!(status, 200, "{body}");
        assert_eq!(body["expansion"]["total"], 1);
        assert_eq!(
            body["expansion"]["contains"][0]["designation"][0]["value"],
            "Disease (disorder)"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn expand_refusals_name_the_parameter() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let isa = format!("{SYSTEM}?fhir_vs=isa/{FINDING}");
        let other_version = format!(
            "http://snomed.info/sct/900000000000207008/version/20200131?fhir_vs=isa/{FINDING}"
        );
        let cases: Vec<Case> = vec![
            (vec![], 400, "`url` is required"),
            (vec![("url", &isa), ("offset", "3")], 400, "past the end"),
            (
                vec![("url", &isa), ("count", "-1")],
                400,
                "`count` must be a non-negative",
            ),
            (
                vec![("url", &isa), ("activeOnly", "yes")],
                400,
                "`activeOnly` must be true or false",
            ),
            (vec![("url", &isa), ("context", "x")], 400, "`context`"),
            (
                vec![("url", "http://loinc.org/vs")],
                400,
                "not a SNOMED CT implicit value set",
            ),
            (
                vec![("url", "http://snomed.info/sct?fhir_vs=ecl/%3C%3C%3C")],
                400,
                "does not parse",
            ),
            (vec![("url", &other_version)], 404, "not the loaded release"),
        ];
        for (pairs, want, text) in cases {
            let (status, body) = get!(request, "/r5/ValueSet/$expand", &pairs);
            let said = diagnostics(&body);
            assert_eq!(status, want, "{pairs:?}: {said}");
            assert!(said.contains(text), "{pairs:?}: {said}");
        }
    })
    .await;
}

#[tokio::test]
#[serial]
async fn operations_are_gated_versioned_and_declared() {
    terminology_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // SV3.1: the same credential gate as every FHIR route.
        let res = request
            .get(&format!(
                "/r5/CodeSystem/$lookup?{}",
                query(&[("system", SYSTEM), ("code", FINDING)])
            ))
            .await;
        assert_eq!(res.status_code(), 401);

        // SV2.2: an unmounted version says what is mounted.
        let (status, body) = get!(
            request,
            "/r9/CodeSystem/$lookup",
            &[("system", SYSTEM), ("code", FINDING)],
        );
        assert_eq!(status, 404);
        assert!(diagnostics(&body).contains("mounted"));

        // SV2.9/SV2.10: declared exactly where served.
        let (_, body) = get!(request, "/r5/metadata", &[]);
        let resources = body["rest"][0]["resource"].as_array().expect("resources");
        let ops = |t: &str| -> Vec<String> {
            resources
                .iter()
                .find(|r| r["type"] == t)
                .and_then(|r| r["operation"].as_array())
                .map(|o| {
                    o.iter()
                        .map(|op| op["name"].as_str().unwrap().to_string())
                        .collect()
                })
                .unwrap_or_default()
        };
        assert_eq!(ops("CodeSystem"), vec!["lookup", "subsumes"]);
        assert_eq!(ops("ValueSet"), vec!["expand"]);
        assert!(ops("Patient").is_empty());
    })
    .await;
}
