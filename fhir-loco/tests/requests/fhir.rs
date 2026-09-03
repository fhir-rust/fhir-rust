//! The FHIR endpoints, against a real store.
//!
//! These were verified by hand with curl first, which is not the same as being
//! tested: a manual check proves the code worked once, on one machine, at one
//! moment. This file is what keeps it working.
//!
//! There is no mock store. The interesting behaviour here is precisely the
//! translation between the store's distinctions and HTTP's — deleted versus
//! never-existed, stale version versus current — and a mock would be asserting
//! that the test author understands those, not that the code does.

use std::sync::OnceLock;

use fhir_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

const FHIR_JSON: &str = "application/fhir+json";

/// A throwaway issuer for the suite.
///
/// There is no unauthenticated path to test through any more, so every request
/// that reaches a handler carries a real token verified by a real signature.
/// The keypair is generated once per binary: the public half goes into the
/// environment before the app boots, the private half stays here to mint.
fn issuer() -> &'static pasetors::keys::AsymmetricKeyPair<pasetors::version4::V4> {
    use pasetors::keys::Generate;
    static KP: OnceLock<pasetors::keys::AsymmetricKeyPair<pasetors::version4::V4>> =
        OnceLock::new();
    KP.get_or_init(|| {
        pasetors::keys::AsymmetricKeyPair::<pasetors::version4::V4>::generate()
            .expect("generate test keypair")
    })
}

/// `Authorization` value for a principal.
fn bearer(subject: &str) -> String {
    use pasetors::claims::Claims;
    let mut claims = Claims::new().expect("claims");
    claims.subject(subject).expect("subject");
    let token =
        pasetors::public::sign(&issuer().secret, &claims, None, None).expect("sign test token");
    format!("Bearer {token}")
}

/// A scratch database with the R5 schema installed, shared by every test here.
///
/// Shared because the store is a process-global `OnceLock`: initialising it
/// twice in one test binary is impossible, so tests coexist in one database and
/// keep out of each other's way by using distinct resource ids rather than
/// distinct databases.
async fn store_ready() {
    static ONCE: OnceLock<()> = OnceLock::new();
    if ONCE.get().is_some() {
        return;
    }
    // Before anything boots: the app refuses to start without this.
    unsafe {
        std::env::set_var(
            fhir_loco::auth::ENV_PUBLIC_KEY,
            hex::encode(issuer().public.as_bytes()),
        );
    }
    let assets = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fhir-sqlite/crates/fhir-sqlite-map/assets");
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-scratch");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let db = dir.join("fhir.sqlite");

    // Install through the library, the way an operator would: this service
    // deliberately does not own schema installation.
    let bytes = std::fs::read(assets.join("fhir-sqlite-relmap-r5.json.gz"))
        .expect("relmap asset; run from a checkout with fhir-sqlite beside it");
    let map = fhir_sqlite_map::model::RelMap::from_gz_bytes(&bytes).expect("relmap decodes");
    let s = fhir_sqlite_store::sqlite::SqliteStore::open(&db, std::sync::Arc::new(map))
        .await
        .expect("open");
    s.init("test").await.expect("install schema");
    drop(s);

    fhir_loco::store::init(fhir_loco::store::BackendConfig::Sqlite {
        db_path: db.to_str().expect("utf-8"),
        assets_dir: assets.to_str().expect("utf-8"),
    })
    .await
    .expect("mount");
    let _ = ONCE.set(());
}

/// Parse a response body. Generic over the response type so this does not have
/// to name `axum_test`'s type, which Loco re-exports but does not expose by
/// name in its prelude.
fn body_of(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("response is JSON")
}

#[tokio::test]
#[serial]
async fn metadata_lists_what_is_actually_served() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/r5/metadata").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(
            res.headers()
                .get("content-type")
                .map(|v| v.to_str().unwrap()),
            Some(FHIR_JSON),
            "plain application/json is wrong enough that conformance tooling rejects it"
        );
        let body = body_of(&res.text());
        assert_eq!(body["resourceType"], "CapabilityStatement");
        assert_eq!(body["fhirVersion"], "5.0.0");
        assert!(
            body["rest"][0]["resource"]
                .as_array()
                .is_some_and(|r| r.len() > 100),
            "a CapabilityStatement that lists almost nothing is a sign the map did not load"
        );
    })
    .await;
}

/// The CapabilityStatement must name every interaction the router serves.
///
/// It did not. The routes have carried `POST`, `PUT` and `DELETE` since they
/// were written, while `metadata` advertised `read`, `vread` and `search-type`
/// alone — so a client doing conformance-driven discovery would have concluded
/// this server was read-only and never attempted a write.
///
/// `A7.12` is normally read as "do not declare what you cannot do". This is the
/// same requirement in the other direction, and it went unnoticed because
/// nothing compared the two lists. That is the shape `U11a` names: where two
/// artifacts must agree, assert the agreement, because each is self-consistent
/// while contradicting the other.
#[tokio::test]
#[serial]
async fn metadata_declares_every_interaction_the_router_serves() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let body = body_of(&request.get("/r5/metadata").await.text());
        let res = &body["rest"][0]["resource"];
        let patient = res
            .as_array()
            .expect("resource array")
            .iter()
            .find(|r| r["type"] == "Patient")
            .expect("Patient must be listed");
        let declared: Vec<&str> = patient["interaction"]
            .as_array()
            .expect("interaction array")
            .iter()
            .filter_map(|i| i["code"].as_str())
            .collect();
        // Keep this list in step with `controllers::fhir::routes`.
        for want in [
            "read",
            "vread",
            "search-type",
            "create",
            "update",
            "delete",
            "history-instance",
            "history-type",
        ] {
            assert!(
                declared.contains(&want),
                "the router serves `{want}` but the CapabilityStatement does not \
                 declare it; a conformance-driven client would never try it. \
                 Declared: {declared:?}"
            );
        }
    })
    .await;
}

#[tokio::test]
#[serial]
async fn an_unmounted_version_says_what_is_mounted() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/r3/Patient/anything").await;
        assert_eq!(res.status_code(), 404);
        let body = body_of(&res.text());
        assert_eq!(body["resourceType"], "OperationOutcome");
        // Naming the mounted versions turns a dead end into a diagnosis.
        assert!(
            body["issue"][0]["diagnostics"]
                .as_str()
                .is_some_and(|d| d.contains("r5")),
            "the error should name what is served: {body}"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn create_read_update_delete_round_trip() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // Create. The server assigns the id; a client-supplied one on POST is
        // what PUT is for.
        let res = request
            .post("/r5/Observation")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(
                r#"{"resourceType":"Observation","status":"final",
                    "valueQuantity":{"value":9.60,"unit":"mg/dL"}}"#,
            )
            .await;
        assert_eq!(res.status_code(), 201);
        let location = res
            .headers()
            .get("location")
            .expect("Location header")
            .to_str()
            .expect("utf-8")
            .to_string();
        let id = location.rsplit('/').next().expect("id").to_string();
        assert!(!id.is_empty());

        // Read. The decimal must survive the whole path — HTTP in, shredded
        // into relational tables, reconstructed, HTTP out.
        let res = request.get(&format!("/r5/Observation/{id}")).await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        assert_eq!(
            body["valueQuantity"]["value"].to_string(),
            "9.60",
            "decimal precision was lost; M3.6 requires the written form to survive"
        );
        assert_eq!(body["meta"]["versionId"], "1");
        assert_eq!(
            res.headers().get("etag").map(|v| v.to_str().unwrap()),
            Some("W/\"1\"")
        );

        // Update at the version we hold.
        let res = request
            .put(&format!("/r5/Observation/{id}"))
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-match", "W/\"1\"")
            .text(format!(
                r#"{{"resourceType":"Observation","id":"{id}","status":"amended"}}"#
            ))
            .await;
        assert_eq!(res.status_code(), 200);

        // The same request again is now stale, and must be refused rather than
        // silently discarding whatever landed in between.
        let res = request
            .put(&format!("/r5/Observation/{id}"))
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-match", "W/\"1\"")
            .text(format!(
                r#"{{"resourceType":"Observation","id":"{id}","status":"final"}}"#
            ))
            .await;
        assert_eq!(res.status_code(), 412, "stale write was not refused");

        // Delete, then again: FHIR delete is idempotent, so a client retrying
        // after a dropped response must not be told it failed.
        let res = request
            .delete(&format!("/r5/Observation/{id}"))
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 204);
        let res = request
            .delete(&format!("/r5/Observation/{id}"))
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 204, "delete is not idempotent");

        // Gone, not missing. A caller that once held this record must be able
        // to tell the difference.
        let res = request.get(&format!("/r5/Observation/{id}")).await;
        assert_eq!(res.status_code(), 410);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn a_body_that_disagrees_with_the_url_is_refused() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // Either interpretation would silently discard what the caller meant.
        let res = request
            .put("/r5/Observation/abc")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(r#"{"resourceType":"Observation","id":"different","status":"final"}"#)
            .await;
        assert_eq!(res.status_code(), 400);

        // A resourceType that disagrees with the path is the same mistake.
        let res = request
            .put("/r5/Observation/abc")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(r#"{"resourceType":"Patient","id":"abc"}"#)
            .await;
        assert_eq!(res.status_code(), 400);

        // An If-Match that is present but unparseable is an error, not a
        // shrug: a client asking for optimistic concurrency and not getting it
        // is worse off than one told no.
        let res = request
            .put("/r5/Observation/abc")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-match", "not-a-version")
            .text(r#"{"resourceType":"Observation","id":"abc","status":"final"}"#)
            .await;
        assert_eq!(res.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn history_shows_every_version_including_the_deletion() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let id = "hist-1";
        for status in ["final", "amended"] {
            let res = request
                .put(&format!("/r5/Observation/{id}"))
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .text(format!(
                    r#"{{"resourceType":"Observation","id":"{id}","status":"{status}"}}"#
                ))
                .await;
            assert!(res.status_code().is_success());
        }
        assert_eq!(
            request
                .delete(&format!("/r5/Observation/{id}"))
                .add_header("authorization", &bearer("dr-who"))
                .await
                .status_code(),
            204
        );

        let res = request.get(&format!("/r5/Observation/{id}/_history")).await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        assert_eq!(body["type"], "history");
        let entries = body["entry"].as_array().expect("entries");
        assert_eq!(entries.len(), 3, "expected create, update, delete");

        // Newest first, and the deletion carries no resource. History that hid
        // its deletions would not be an audit trail.
        assert_eq!(entries[0]["request"]["method"], "DELETE");
        assert!(entries[0].get("resource").is_none());
        assert_eq!(entries[0]["response"]["etag"], "W/\"3\"");
        assert_eq!(entries[2]["request"]["method"], "POST");
        assert!(entries[2].get("resource").is_some());

        // vread reaches a version the live tables no longer hold...
        let res = request
            .get(&format!("/r5/Observation/{id}/_history/1"))
            .await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(body_of(&res.text())["status"], "final");
        // ...the deletion is 410, because it is a real version with no content...
        assert_eq!(
            request
                .get(&format!("/r5/Observation/{id}/_history/3"))
                .await
                .status_code(),
            410
        );
        // ...and a version never written is 404.
        assert_eq!(
            request
                .get(&format!("/r5/Observation/{id}/_history/99"))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn search_returns_a_bundle_and_respects_paging() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        for i in 0..3 {
            let res = request
                .put(&format!("/r5/Observation/srch-{i}"))
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .text(format!(
                    r#"{{"resourceType":"Observation","id":"srch-{i}","status":"registered"}}"#
                ))
                .await;
            assert!(res.status_code().is_success());
        }

        let res = request
            .get("/r5/Observation?status=registered&_total=accurate")
            .await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        assert_eq!(body["resourceType"], "Bundle");
        assert_eq!(body["type"], "searchset");
        assert_eq!(body["total"], 3);
        assert_eq!(body["entry"].as_array().map(Vec::len), Some(3));

        // `_count` bounds the page; `_total` must keep counting every match, or
        // a client cannot tell how far it has to page.
        let res = request
            .get("/r5/Observation?status=registered&_count=2&_total=accurate")
            .await;
        let body = body_of(&res.text());
        assert_eq!(body["entry"].as_array().map(Vec::len), Some(2));
        assert_eq!(body["total"], 3, "_total must ignore paging");

        // Control parameters must not be mistaken for search criteria.
        let res = request
            .get("/r5/Observation?status=registered&_offset=2")
            .await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(
            body_of(&res.text())["entry"].as_array().map(Vec::len),
            Some(1)
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn include_and_revinclude_resolve_references() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        for (path, body) in [
            (
                "/r5/Practitioner/prac-inc",
                r#"{"resourceType":"Practitioner","id":"prac-inc"}"#.to_string(),
            ),
            (
                "/r5/Patient/pt-inc",
                r#"{"resourceType":"Patient","id":"pt-inc",
                    "name":[{"family":"Includeme"}],
                    "generalPractitioner":[{"reference":"Practitioner/prac-inc"}]}"#
                    .to_string(),
            ),
            (
                "/r5/Encounter/enc-inc",
                r#"{"resourceType":"Encounter","id":"enc-inc","status":"completed",
                    "subject":{"reference":"Patient/pt-inc"}}"#
                    .to_string(),
            ),
        ] {
            let res = request
                .put(path)
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .text(body)
                .await;
            assert!(res.status_code().is_success(), "seed {path}");
        }

        // _include resolves the forward reference; modes distinguish the
        // match from the inclusion (SV2.16).
        let res = request
            .get("/r5/Patient?family=Includeme&_include=Patient:general-practitioner")
            .await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        let entries = body["entry"].as_array().expect("entries");
        assert_eq!(entries.len(), 2, "{body}");
        assert_eq!(entries[0]["resource"]["resourceType"], "Patient");
        assert_eq!(entries[0]["search"]["mode"], "match");
        assert_eq!(entries[1]["resource"]["resourceType"], "Practitioner");
        assert_eq!(entries[1]["search"]["mode"], "include");

        // A target-type filter that matches nothing filters the include out
        // without failing the search.
        let res = request
            .get("/r5/Patient?family=Includeme&_include=Patient:general-practitioner:Organization")
            .await;
        let body = body_of(&res.text());
        assert_eq!(body["entry"].as_array().map(Vec::len), Some(1), "{body}");

        // _revinclude finds what points here.
        let res = request
            .get("/r5/Patient?family=Includeme&_revinclude=Encounter:subject")
            .await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        let entries = body["entry"].as_array().expect("entries");
        assert_eq!(entries.len(), 2, "{body}");
        assert_eq!(entries[1]["resource"]["resourceType"], "Encounter");
        assert_eq!(entries[1]["search"]["mode"], "include");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn includes_are_refused_by_name_never_dropped() {
    // A silently dropped include returns less than the client asked for
    // while looking complete — every invalid form refuses (SV2.16).
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        for (query, expect) in [
            ("_include=Observation:subject", "source type"),
            ("_include=Patient:family", "not a reference parameter"),
            ("_include:iterate=Patient:link", "iterate"),
            ("_revinclude=Nonexistent:subject", "unknown resource type"),
            ("_include=Patient", "expected <type>:<param>"),
        ] {
            let res = request.get(&format!("/r5/Patient?{query}")).await;
            assert_eq!(res.status_code(), 400, "{query} must refuse");
            let body = res.text();
            assert!(
                body.contains(expect),
                "{query}: refusal must name the problem ({expect:?}): {body}"
            );
        }
    })
    .await;
}

#[tokio::test]
#[serial]
async fn metadata_declares_search_includes() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/r5/metadata").await;
        let body = body_of(&res.text());
        let resources = body["rest"][0]["resource"].as_array().expect("resources");
        let patient = resources
            .iter()
            .find(|r| r["type"] == "Patient")
            .expect("Patient declared");
        let includes = patient["searchInclude"].as_array().expect("searchInclude");
        assert!(
            includes.contains(&serde_json::json!("Patient:general-practitioner")),
            "the compiled reference parameters must be discoverable (SV2.16): {includes:?}"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn type_and_system_history_serve_the_stated_slice() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // Two versions of one patient, then a created-and-deleted basic —
        // history must show all of it, newest first, deletions included.
        for (method_put, path, body) in [
            (
                true,
                "/r5/Patient/hist-p",
                r#"{"resourceType":"Patient","id":"hist-p"}"#,
            ),
            (
                true,
                "/r5/Patient/hist-p",
                r#"{"resourceType":"Patient","id":"hist-p","active":true}"#,
            ),
            (
                true,
                "/r5/Basic/hist-b",
                r#"{"resourceType":"Basic","id":"hist-b","code":{"text":"x"}}"#,
            ),
        ] {
            assert!(method_put);
            let res = request
                .put(path)
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .text(body.to_string())
                .await;
            assert!(res.status_code().is_success(), "seed {path}");
        }
        let res = request
            .delete("/r5/Basic/hist-b")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert!(res.status_code().is_success(), "delete hist-b");

        // Type-level: only that type, newest first, the deletion present as
        // an entry with no resource.
        let res = request.get("/r5/Basic/_history").await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        assert_eq!(body["type"], "history");
        let entries = body["entry"].as_array().expect("entries");
        assert_eq!(entries.len(), 2, "{body}");
        assert_eq!(entries[0]["request"]["method"], "DELETE");
        assert!(
            entries[0].get("resource").is_none(),
            "a deletion carries no content"
        );
        assert_eq!(entries[1]["request"]["method"], "POST");

        // System-level spans types; _count bounds it.
        let res = request.get("/r5/_history?_count=100").await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        let entries = body["entry"].as_array().expect("entries");
        let types: std::collections::BTreeSet<&str> = entries
            .iter()
            .map(|e| {
                e["request"]["url"]
                    .as_str()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
            })
            .collect();
        assert!(
            types.contains("Patient") && types.contains("Basic"),
            "system history must span types: {types:?}"
        );
        let res = request.get("/r5/_history?_count=1").await;
        assert_eq!(
            body_of(&res.text())["entry"].as_array().map(Vec::len),
            Some(1)
        );

        // _since far in the future is empty; malformed _since and unknown
        // parameters are refused by name, never dropped (SV2.17).
        let res = request
            .get("/r5/_history?_since=9999-01-01T00:00:00Z")
            .await;
        assert_eq!(
            body_of(&res.text())["entry"].as_array().map(Vec::len),
            Some(0)
        );
        let res = request.get("/r5/_history?_since=yesterday").await;
        assert_eq!(res.status_code(), 400);
        assert!(res.text().contains("RFC 3339"));
        let res = request.get("/r5/Basic/_history?_at=2026").await;
        assert_eq!(res.status_code(), 400);
        assert!(res.text().contains("_at"), "{}", res.text());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn metadata_declares_history_scopes() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let body = body_of(&request.get("/r5/metadata").await.text());
        let rest = &body["rest"][0];
        let system: Vec<&str> = rest["interaction"]
            .as_array()
            .expect("rest interaction")
            .iter()
            .filter_map(|i| i["code"].as_str())
            .collect();
        assert!(system.contains(&"history-system"), "{system:?}");
        let patient = rest["resource"]
            .as_array()
            .expect("resources")
            .iter()
            .find(|r| r["type"] == "Patient")
            .expect("Patient");
        let declared: Vec<&str> = patient["interaction"]
            .as_array()
            .expect("interactions")
            .iter()
            .filter_map(|i| i["code"].as_str())
            .collect();
        assert!(
            declared.contains(&"history-type") && declared.contains(&"history-instance"),
            "{declared:?}"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn transaction_and_batch_bundles_are_refused_with_the_reason() {
    // SV2.18: the refusal is served to clients, not left as a bare 405.
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/r5")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(r#"{"resourceType":"Bundle","type":"transaction","entry":[]}"#.to_string())
            .await;
        assert_eq!(res.status_code(), 501);
        let body = res.text();
        assert!(
            body.contains("atomic") && body.contains("half-applied"),
            "the refusal must carry the reasoning: {body}"
        );

        let res = request
            .post("/r5")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(r#"{"resourceType":"Bundle","type":"batch","entry":[]}"#.to_string())
            .await;
        assert_eq!(res.status_code(), 501);
        assert!(
            res.text().contains("unbuilt rather than rejected"),
            "batch is refused by its own name"
        );

        let res = request
            .post("/r5")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .text(r#"{"resourceType":"Patient"}"#.to_string())
            .await;
        assert_eq!(res.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn reads_are_recorded_as_disclosures() {
    // "Who looked at this patient" is usually an audit's first question, and the
    // read path is where recording it is easiest to forget — it was missing here
    // until a check of the access log turned up empty.
    store_ready().await;
    let before = disclosure_count().await;
    request::<App, _, _>(|request, _ctx| async move {
        request
            .get("/r5/Observation/definitely-not-here")
            .add_header("authorization", &bearer("nurse"))
            .await;
        request.get("/r5/Observation?status=final").await;
    })
    .await;
    let after = disclosure_count().await;
    assert!(
        after >= before + 2,
        "a failed read and a search should both be disclosures: {before} -> {after}"
    );
}

/// Read the disclosure count straight from the store, not over HTTP: the point
/// is that the *service* recorded something, so asking the service would be
/// circular.
async fn disclosure_count() -> i64 {
    fhir_loco::store::versions()
        .expect("store mounted")
        .get("r5")
        .expect("r5 mounted")
        .access_log_len()
        .await
        .expect("access log")
}

/// The property Option 2 exists for: there is no way in.
///
/// Mutation-verified (`T11.10`) — restoring any header fallback in
/// `auth::audit_from` makes the first two assertions pass with `201`/`200`,
/// which is exactly the silent-downgrade this design removes.
#[tokio::test]
#[serial]
async fn a_write_without_a_token_is_refused() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // No Authorization header at all.
        let res = request
            .post("/r5/Observation")
            .add_header("content-type", FHIR_JSON)
            .json(
                &serde_json::json!({"resourceType": "Observation", "status": "final",
                "code": {"text": "unauthenticated attempt"}}),
            )
            .await;
        assert_eq!(
            res.status_code(),
            401,
            "a write with no token must be refused"
        );
        assert_eq!(
            res.headers()
                .get("www-authenticate")
                .map(|v| v.to_str().unwrap()),
            Some("Bearer"),
            "a 401 must say how to authenticate"
        );

        // A syntactically valid token signed by somebody else.
        use pasetors::claims::Claims;
        use pasetors::keys::Generate;
        let attacker =
            pasetors::keys::AsymmetricKeyPair::<pasetors::version4::V4>::generate().expect("kp");
        let mut claims = Claims::new().expect("claims");
        claims.subject("dr-who").expect("sub");
        let forged = pasetors::public::sign(&attacker.secret, &claims, None, None).expect("sign");
        let res = request
            .post("/r5/Observation")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &format!("Bearer {forged}"))
            .json(
                &serde_json::json!({"resourceType": "Observation", "status": "final",
                "code": {"text": "forged attribution"}}),
            )
            .await;
        assert_eq!(
            res.status_code(),
            401,
            "a token from an unknown issuer must not attribute a write"
        );

        // And the old header buys nothing.
        let res = request
            .post("/r5/Observation")
            .add_header("content-type", FHIR_JSON)
            .add_header("x-fhir-loco-principal", "dr-who")
            .json(
                &serde_json::json!({"resourceType": "Observation", "status": "final",
                "code": {"text": "header attempt"}}),
            )
            .await;
        assert_eq!(
            res.status_code(),
            401,
            "the removed header must not be honoured"
        );
    })
    .await;
}

/// SV2.14: `If-None-Exist` conditional create — all four outcomes the spec
/// table names. The store makes search-then-create indivisible; these assert
/// the HTTP layer preserves each verdict rather than flattening them.
#[tokio::test]
#[serial]
async fn conditional_create_serves_all_four_outcomes() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let patient = |family: &str| {
            serde_json::json!({
                "resourceType": "Patient",
                "identifier": [{ "system": "urn:cc", "value": "one" }],
                "name": [{ "family": family }]
            })
        };

        // No match: created, exactly like a plain POST.
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-none-exist", "identifier=urn:cc|one")
            .json(&patient("First"))
            .await;
        assert_eq!(
            res.status_code(),
            201,
            "no match must create: {}",
            res.text()
        );
        let first_id = res
            .headers()
            .get("location")
            .expect("Location header")
            .to_str()
            .expect("utf-8")
            .rsplit('/')
            .next()
            .expect("id")
            .to_string();

        // Exactly one match: the existing resource comes back unchanged —
        // same id, still version 1, and the submitted body is NOT written.
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-none-exist", "identifier=urn:cc|one")
            .json(&patient("Second"))
            .await;
        assert_eq!(
            res.status_code(),
            200,
            "one match must return it: {}",
            res.text()
        );
        let body = body_of(&res.text());
        assert_eq!(body["id"], serde_json::json!(first_id));
        assert_eq!(
            body["meta"]["versionId"], "1",
            "the match must be unchanged"
        );
        assert_eq!(
            body["name"][0]["family"], "First",
            "the submitted body must not overwrite the match"
        );

        // A second resource with the same identifier (plain POST bypasses the
        // precondition), then the same conditional create: ambiguous, 412.
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .json(&patient("Rival"))
            .await;
        assert_eq!(res.status_code(), 201);
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-none-exist", "identifier=urn:cc|one")
            .json(&patient("Third"))
            .await;
        assert_eq!(
            res.status_code(),
            412,
            "more than one match must refuse: {}",
            res.text()
        );
        let body = body_of(&res.text());
        assert_eq!(body["resourceType"], "OperationOutcome");

        // Present-but-empty is an error, never an unconditional create
        // (SV2.14): dropping the precondition silently is the duplicate the
        // header exists to prevent.
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .add_header("if-none-exist", "   ")
            .json(&patient("Fourth"))
            .await;
        assert_eq!(
            res.status_code(),
            400,
            "an unreadable precondition must be refused: {}",
            res.text()
        );
    })
    .await;
}

/// SV2.9/SV2.14: the CapabilityStatement declares conditional create, so a
/// conformance-driven client discovers it rather than trying it.
#[tokio::test]
#[serial]
async fn metadata_declares_conditional_create() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/r5/metadata").await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        let resources = body["rest"][0]["resource"]
            .as_array()
            .expect("rest.resource array");
        assert!(!resources.is_empty());
        assert!(
            resources
                .iter()
                .all(|r| r["conditionalCreate"] == serde_json::json!(true)),
            "every resource type must declare conditionalCreate (SV2.14)"
        );
    })
    .await;
}

/// SV2.19: `DELETE /{version}/{rtype}?params` conditional delete — all three
/// outcomes the spec table names, plus the no-criteria refusal. No-match and
/// single-match both answer `204`: deletion is idempotent, the same rule
/// instance-level delete rests on.
#[tokio::test]
#[serial]
async fn conditional_delete_serves_all_outcomes() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let patient = |family: &str| {
            serde_json::json!({
                "resourceType": "Patient",
                "identifier": [{ "system": "urn:cd", "value": "one" }],
                "name": [{ "family": family }]
            })
        };

        // No match: still 204, per SV2.19 — a criteria set matching nothing
        // has already reached the end state a matching delete would produce.
        let res = request
            .delete("/r5/Patient?identifier=urn:cd|one")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(
            res.status_code(),
            204,
            "no match must be idempotent, not an error: {}",
            res.text()
        );

        // Exactly one match: deleted, 204, and a subsequent read is 410 —
        // the delete actually happened, not merely "reported success".
        let res = request
            .post("/r5/Patient")
            .add_header("content-type", FHIR_JSON)
            .add_header("authorization", &bearer("dr-who"))
            .json(&patient("First"))
            .await;
        assert_eq!(res.status_code(), 201);
        let id = res
            .headers()
            .get("location")
            .expect("Location header")
            .to_str()
            .expect("utf-8")
            .rsplit('/')
            .next()
            .expect("id")
            .to_string();
        let res = request
            .delete("/r5/Patient?identifier=urn:cd|one")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(
            res.status_code(),
            204,
            "one match must delete it: {}",
            res.text()
        );
        let res = request.get(&format!("/r5/Patient/{id}")).await;
        assert_eq!(
            res.status_code(),
            410,
            "the match must actually be gone, not just reported deleted"
        );

        // Two resources sharing the criteria: ambiguous, 412 — the same
        // status and reason shape SV2.14's conditional create uses for its
        // own "more than one match" case.
        for family in ["Second", "Third"] {
            let res = request
                .post("/r5/Patient")
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .json(&patient(family))
                .await;
            assert_eq!(res.status_code(), 201);
        }
        let res = request
            .delete("/r5/Patient?identifier=urn:cd|one")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(
            res.status_code(),
            412,
            "more than one match must refuse rather than delete either: {}",
            res.text()
        );
        let body = body_of(&res.text());
        assert_eq!(body["resourceType"], "OperationOutcome");

        // No criteria at all: refused outright (SV2.19) rather than treated
        // as "delete the type's one resource, if there happens to be one" —
        // silently allowing that turns a missing query parameter into a
        // request that deletes an entire type.
        let res = request
            .delete("/r5/Patient")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(
            res.status_code(),
            400,
            "criteria-less conditional delete must be refused: {}",
            res.text()
        );
    })
    .await;
}

/// SV2.9/SV2.19: the CapabilityStatement declares conditional delete, so a
/// conformance-driven client discovers it rather than trying it.
#[tokio::test]
#[serial]
async fn metadata_declares_conditional_delete() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/r5/metadata").await;
        assert_eq!(res.status_code(), 200);
        let body = body_of(&res.text());
        let resources = body["rest"][0]["resource"]
            .as_array()
            .expect("rest.resource array");
        assert!(!resources.is_empty());
        assert!(
            resources
                .iter()
                .all(|r| r["conditionalDelete"] == serde_json::json!("single")),
            "every resource type must declare conditionalDelete (SV2.19)"
        );
    })
    .await;
}

/// SV2.15: the Bulk Data `$export` async slice, end to end — kick-off, poll,
/// manifest, NDJSON fetch (disclosure-logged), cancel/cleanup.
#[tokio::test]
#[serial]
async fn export_serves_the_async_bulk_data_contract() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // Two patients this test can recognise among whatever earlier tests
        // created; the manifest count is asserted against the file, exactly.
        for family in ["ExportOne", "ExportTwo"] {
            let res = request
                .post("/r5/Patient")
                .add_header("content-type", FHIR_JSON)
                .add_header("authorization", &bearer("dr-who"))
                .json(&serde_json::json!({
                    "resourceType": "Patient",
                    "name": [{ "family": family }]
                }))
                .await;
            assert_eq!(res.status_code(), 201);
        }

        // Kick-off: 202 + Content-Location, per the Bulk Data contract.
        let res = request
            .get("/r5/$export?_type=Patient")
            .add_header("authorization", &bearer("dr-who"))
            .add_header("prefer", "respond-async")
            .await;
        assert_eq!(res.status_code(), 202, "{}", res.text());
        let status_url = res
            .headers()
            .get("content-location")
            .expect("Content-Location")
            .to_str()
            .expect("utf-8")
            .to_string();

        // Poll until complete. 202 + X-Progress while running, 200 + manifest
        // when done; the in-process worker finishes in well under a second.
        let mut manifest = None;
        for _ in 0..100 {
            let res = request
                .get(&status_url)
                .add_header("authorization", &bearer("dr-who"))
                .await;
            match res.status_code().as_u16() {
                202 => tokio::time::sleep(std::time::Duration::from_millis(50)).await,
                200 => {
                    manifest = Some(body_of(&res.text()));
                    break;
                }
                other => panic!("unexpected export status {other}: {}", res.text()),
            }
        }
        let manifest = manifest.expect("the export should complete");
        assert_eq!(manifest["requiresAccessToken"], true);
        let output = manifest["output"].as_array().expect("output array");
        let patient_output = output
            .iter()
            .find(|o| o["type"] == "Patient")
            .expect("a Patient output");
        let count = patient_output["count"].as_u64().expect("count");
        assert!(
            count >= 2,
            "at least the two patients created above: {count}"
        );
        assert!(
            !output.iter().any(|o| o["type"] == "Observation"),
            "_type=Patient must filter: {output:?}"
        );

        // Fetch the NDJSON. Every line parses, is a Patient, and the line
        // count equals the manifest count — the manifest is a promise.
        let file_url = patient_output["url"].as_str().expect("url");
        let res = request
            .get(file_url)
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 200);
        let lines: Vec<serde_json::Value> = res
            .text()
            .lines()
            .map(|l| serde_json::from_str(l).expect("NDJSON line"))
            .collect();
        assert_eq!(
            lines.len() as u64,
            count,
            "manifest count must match the file"
        );
        assert!(lines.iter().all(|l| l["resourceType"] == "Patient"));

        // DELETE cancels/cleans; afterwards the job and its files are gone.
        let res = request
            .delete(&status_url)
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 202);
        let res = request
            .get(&status_url)
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 404, "a deleted job is gone");
        let res = request
            .get(file_url)
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 404, "its files are gone too");
    })
    .await;
}

/// SV2.15: the kick-off is strict — auth first, the async preference is
/// required, and unsupported parameters are refused by name, never ignored.
#[tokio::test]
#[serial]
async fn export_kickoff_refuses_rather_than_ignores() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        // No token: 401 before anything else happens.
        let res = request
            .get("/r5/$export")
            .add_header("prefer", "respond-async")
            .await;
        assert_eq!(res.status_code(), 401);

        // No `Prefer: respond-async`: refused, naming the header.
        let res = request
            .get("/r5/$export")
            .add_header("authorization", &bearer("dr-who"))
            .await;
        assert_eq!(res.status_code(), 400);
        assert!(res.text().contains("respond-async"), "{}", res.text());

        // `_since` is not supported in this slice: refused by name, because
        // silently ignoring a filter returns more than was asked (SV2.13).
        let res = request
            .get("/r5/$export?_since=2024-01-01T00:00:00Z")
            .add_header("authorization", &bearer("dr-who"))
            .add_header("prefer", "respond-async")
            .await;
        assert_eq!(res.status_code(), 400);
        assert!(res.text().contains("_since"), "{}", res.text());

        // An unknown `_type` value is refused, naming it.
        let res = request
            .get("/r5/$export?_type=NotAResource")
            .add_header("authorization", &bearer("dr-who"))
            .add_header("prefer", "respond-async")
            .await;
        assert_eq!(res.status_code(), 400);
        assert!(res.text().contains("NotAResource"), "{}", res.text());
    })
    .await;
}

/// SV2.9/SV2.15: the CapabilityStatement declares the export operation.
#[tokio::test]
#[serial]
async fn metadata_declares_the_export_operation() {
    store_ready().await;
    request::<App, _, _>(|request, _ctx| async move {
        let body = body_of(&request.get("/r5/metadata").await.text());
        let ops = body["rest"][0]["operation"].as_array().expect("operations");
        assert!(
            ops.iter().any(|o| o["name"] == "export"),
            "the $export operation must be declared: {ops:?}"
        );
    })
    .await;
}
