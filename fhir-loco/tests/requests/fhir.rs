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

    fhir_loco::store::init(db.to_str().expect("utf-8"), assets.to_str().expect("utf-8"))
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
        for want in ["read", "vread", "search-type", "create", "update", "delete"] {
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
