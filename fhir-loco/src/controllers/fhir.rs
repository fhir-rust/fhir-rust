//! FHIR RESTful endpoints, mounted per version at `/{r3|r4|r5}`.
//!
//! Deliberately thin. Every guarantee this service makes — history, the audit
//! chain, search semantics, erasure — belongs to the storage crate; this layer
//! translates HTTP to store calls and back, and its own job is to get the status
//! codes right. Where a distinction exists in the store it must survive the
//! translation: "deleted" and "never existed" are 410 and 404, and collapsing
//! them would tell a caller that a record it once held never was.

use axum::body::Bytes;
use axum::extract::{Path, Query};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::prelude::*;

use crate::store;

/// FHIR's own JSON media type. Returning `application/json` would be wrong
/// enough that conformance tooling rejects it.
pub(crate) const FHIR_JSON: &str = "application/fhir+json";

/// An OperationOutcome, which is how FHIR reports a problem.
///
/// The text is deliberately about the request, never about storage: it names
/// what the caller asked for, so it can be returned verbatim without leaking
/// schema names or stored values.
pub(crate) fn outcome(status: StatusCode, severity: &str, code: &str, text: &str) -> AxumResponse {
    let body = serde_json::json!({
        "resourceType": "OperationOutcome",
        "issue": [{
            "severity": severity,
            "code": code,
            "diagnostics": text,
        }]
    });
    (
        status,
        [(header::CONTENT_TYPE, FHIR_JSON)],
        serde_json::to_string(&body).unwrap_or_default(),
    )
        .into_response()
}

fn fhir_json(
    status: StatusCode,
    body: &serde_json::Value,
    version_id: Option<i64>,
) -> AxumResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, FHIR_JSON.parse().expect("static"));
    if let Some(v) = version_id {
        // A weak ETag, because FHIR versions are not byte-identity: two
        // representations of the same version may differ in whitespace.
        if let Ok(tag) = format!("W/\"{v}\"").parse() {
            headers.insert(header::ETAG, tag);
        }
    }
    (
        status,
        headers,
        serde_json::to_string(body).unwrap_or_default(),
    )
        .into_response()
}

/// Resolve a mounted version, or explain that it is not served.
///
/// The error side is a whole `AxumResponse`, which clippy notes is large. That
/// is deliberate: the alternative is an error enum that every caller must
/// re-translate into the same response, which trades one wide return value for
/// duplicated status-code logic — and getting status codes right is this
/// layer's only real job.
#[allow(clippy::result_large_err)]
pub(crate) fn version_of(v: &str) -> Result<&'static store::AnyStore, AxumResponse> {
    let Some(versions) = store::versions() else {
        return Err(outcome(
            StatusCode::SERVICE_UNAVAILABLE,
            "fatal",
            "transient",
            "the store is not initialised",
        ));
    };
    versions.get(v).ok_or_else(|| {
        outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-supported",
            &format!(
                "FHIR version {v:?} is not served here; mounted: {}",
                versions.mounted().join(", ")
            ),
        )
    })
}

/// `GET /{version}/{type}/{id}`
#[debug_handler]
async fn read(
    Path((version, rtype, id)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };

    match store.get(&rtype, &id).await {
        Ok(Some(mut body)) => {
            // The stored version travels in `meta.versionId` and the ETag; a
            // client cannot do optimistic concurrency without it.
            let v = match store.status(&rtype, &id).await {
                Ok(fhir_sqlite_store::ResourceStatus::Active(v)) => Some(v),
                _ => None,
            };
            if let (Some(v), Some(obj)) = (v, body.as_object_mut()) {
                obj.insert(
                    "meta".to_string(),
                    serde_json::json!({ "versionId": v.to_string() }),
                );
            }
            disclose(
                store,
                &headers,
                "read",
                Some(&rtype),
                Some(&id),
                v,
                "ok",
                Some(1),
            )
            .await;
            fhir_json(StatusCode::OK, &body, v)
        }
        Ok(None) => {
            disclose(
                store,
                &headers,
                "read",
                Some(&rtype),
                Some(&id),
                None,
                "not-found",
                Some(0),
            )
            .await;
            match store.status(&rtype, &id).await {
                // Deleted and never-existed are different answers, and a caller
                // that once held this record needs to be able to tell them apart.
                Ok(fhir_sqlite_store::ResourceStatus::Deleted(v)) => outcome(
                    StatusCode::GONE,
                    "error",
                    "deleted",
                    &format!("{rtype}/{id} was deleted at version {v}"),
                ),
                _ => outcome(
                    StatusCode::NOT_FOUND,
                    "error",
                    "not-found",
                    &format!("{rtype}/{id} not found"),
                ),
            }
        }
        Err(e) => store_error(e),
    }
}

/// `GET /{version}/{type}/{id}/_history/{vid}`
#[debug_handler]
async fn vread(
    Path((version, rtype, id, vid)): Path<(String, String, String, i64)>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    let found = matches!(store.vread(&rtype, &id, vid).await, Ok(Some(_)));
    disclose(
        store,
        &headers,
        "vread",
        Some(&rtype),
        Some(&id),
        Some(vid),
        if found { "ok" } else { "not-found" },
        Some(i64::from(found)),
    )
    .await;
    match store.vread(&rtype, &id, vid).await {
        Ok(Some(entry)) => match entry.resource {
            Some(body) => fhir_json(StatusCode::OK, &body, Some(entry.version_id)),
            // A deletion is a real version with no content: 410, not 404.
            None => outcome(
                StatusCode::GONE,
                "error",
                "deleted",
                &format!("{rtype}/{id} version {vid} is a deletion"),
            ),
        },
        Ok(None) => outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            &format!("{rtype}/{id} has no version {vid}"),
        ),
        Err(e) => store_error(e),
    }
}

/// `GET /{version}/metadata` — the CapabilityStatement.
#[debug_handler]
async fn metadata(Path(version): Path<String>) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    let mut types: Vec<serde_json::Value> = store
        .map()
        .resources
        .iter()
        .map(|(t, rm)| {
            // SV2.16: exactly the reference parameters the map compiled.
            // searchRevInclude stays undeclared — reference columns are
            // untyped here, so the honest list is every reference parameter
            // of every type, thousands of entries serving nobody.
            let search_include: Vec<String> = rm
                .search
                .iter()
                .filter(|d| {
                    d.targets.iter().any(|tg| {
                        matches!(
                            tg.kind,
                            fhir_sqlite_map::model::TargetKind::Reference { .. }
                        )
                    })
                })
                .map(|d| format!("{t}:{}", d.code))
                .collect();
            // Every interaction this router actually serves. `A7.12` is
            // usually read as "do not declare what you cannot do", and the
            // reverse was true here: the routes have carried `POST`, `PUT` and
            // `DELETE` since they were written, while this list advertised
            // three read-only interactions. A client doing conformance-driven
            // discovery would have concluded the server was read-only.
            serde_json::json!({
                "type": t,
                "interaction": [
                    { "code": "read" },
                    { "code": "vread" },
                    { "code": "search-type" },
                    { "code": "create" },
                    { "code": "update" },
                    { "code": "delete" },
                    { "code": "history-instance" },
                    { "code": "history-type" },
                ],
                // If-None-Exist is served (SV2.14); a conformance-driven
                // client discovers it here rather than by trying it.
                "conditionalCreate": true,
                "searchInclude": search_include,
            })
        })
        .collect();
    types.sort_by(|a, b| a["type"].as_str().cmp(&b["type"].as_str()));

    let body = serde_json::json!({
        "resourceType": "CapabilityStatement",
        "status": "active",
        "kind": "instance",
        "fhirVersion": store.map().fhir_version,
        "format": ["application/fhir+json"],
        // `fhir-loco`, not `fhir-store`: the split (F-45) gave the old name to
        // the engine-agnostic persistence core, and this is the HTTP surface.
        "software": { "name": "fhir-loco", "version": env!("CARGO_PKG_VERSION") },
        "rest": [{
            "mode": "server",
            // Whole-system history is served (SV2.17).
            "interaction": [{ "code": "history-system" }],
            "resource": types,
            // System-level Bulk Data export is served (SV2.15); a
            // conformance-driven client discovers it here (SV2.9).
            "operation": [{
                "name": "export",
                "definition": "http://hl7.org/fhir/uv/bulkdata/OperationDefinition/export",
            }],
        }],
    });
    fhir_json(StatusCode::OK, &body, None)
}

/// Translate a store failure into a response.
///
/// `Unsupported` is the only variant safe to return verbatim: it describes the
/// request in the caller's own terms. Everything else may name schema objects or
/// stored values, so it is logged and answered with a generic 500 — the detail
/// belongs in an operator's log, not a response body.
fn store_error(e: store::StoreFailure) -> AxumResponse {
    use store::StoreFailure as E;
    match e {
        E::Unsupported(msg) => outcome(StatusCode::BAD_REQUEST, "error", "not-supported", &msg),
        E::Conflict { expected, found } => outcome(
            StatusCode::PRECONDITION_FAILED,
            "error",
            "conflict",
            &format!("version conflict: expected {expected}, found {found}"),
        ),
        other => {
            tracing::error!(error = %other, "store failure");
            outcome(
                StatusCode::INTERNAL_SERVER_ERROR,
                "fatal",
                "exception",
                "internal error",
            )
        }
    }
}

/// The version named by `If-Match`, if any.
///
/// FHIR uses weak ETags (`W/"3"`), so the prefix and quotes are stripped. A
/// header that is present but unparseable is *not* silently ignored: a client
/// asking for optimistic concurrency and not getting it would be worse than an
/// error, because it would believe a write was checked when it was not.
#[allow(clippy::result_large_err)]
fn if_match(headers: &HeaderMap) -> Result<Option<i64>, AxumResponse> {
    let Some(raw) = headers.get(header::IF_MATCH) else {
        return Ok(None);
    };
    let text = raw.to_str().unwrap_or("");
    let trimmed = text.trim().trim_start_matches("W/").trim_matches('"');
    trimmed.parse::<i64>().map(Some).map_err(|_| {
        outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "structure",
            &format!("If-Match {text:?} is not a version"),
        )
    })
}

/// Parse a request body as a FHIR resource of the expected type.
#[allow(clippy::result_large_err)]
fn parse_body(bytes: &Bytes, rtype: &str) -> Result<serde_json::Value, AxumResponse> {
    let v: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| {
        outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "structure",
            &format!("body is not JSON: {e}"),
        )
    })?;
    match v.get("resourceType").and_then(serde_json::Value::as_str) {
        Some(t) if t == rtype => Ok(v),
        Some(t) => Err(outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "invariant",
            &format!("resourceType {t:?} does not match {rtype:?} in the URL"),
        )),
        None => Err(outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "required",
            "resource has no resourceType",
        )),
    }
}

/// Wrap ids in a searchset Bundle.
fn bundle(kind: &str, entries: Vec<serde_json::Value>, total: Option<i64>) -> serde_json::Value {
    let mut b = serde_json::json!({
        "resourceType": "Bundle",
        "type": kind,
        "entry": entries,
    });
    if let (Some(t), Some(o)) = (total, b.as_object_mut()) {
        o.insert("total".to_string(), serde_json::json!(t));
    }
    b
}

/// Record a disclosure (PR12.5).
///
/// A store that logs only mutations cannot answer "who looked at this patient",
/// which is the question an audit usually opens with — so this is a read-path
/// obligation, and reads are where it is easiest to forget.
///
/// A logging failure is reported but never fails the request: refusing to serve
/// because the audit sink is unavailable is a trade some deployments want and
/// most do not, and making that choice silently here would be wrong either way.
// Eight parameters, which clippy dislikes. They are the columns of one access
// log row; grouping them into a struct would mean building that struct at every
// call site to satisfy a lint, and the call sites are where forgetting a field
// would be easiest to miss.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn disclose(
    store: &store::AnyStore,
    headers: &HeaderMap,
    interaction: &str,
    rtype: Option<&str>,
    id: Option<&str>,
    version_id: Option<i64>,
    outcome: &str,
    result_count: Option<i64>,
) {
    // A disclosure is written even when the token is rejected: the read did
    // not happen, but the *attempt* is the thing PR12 wants recorded, and an
    // unattributable attempt is more interesting than a missing line.
    let rec = fhir_sqlite_store::AccessRecord {
        audit: crate::auth::audit_from(headers).unwrap_or_else(|_| fhir_sqlite_store::Audit {
            actor: "unauthenticated".to_string(),
            actor_source: Some("rejected-token".to_string()),
            client: None,
            request_id: None,
            reason: None,
        }),
        interaction: interaction.to_string(),
        rtype: rtype.map(str::to_string),
        id: id.map(str::to_string),
        version_id,
        outcome: outcome.to_string(),
        result_count,
    };
    if let Err(e) = store.log_access(&rec).await {
        tracing::error!(error = %e, interaction, "failed to record a disclosure");
    }
}

/// `POST /{version}/{type}` — create.
///
/// FHIR lets the server assign the id. The storage layer requires one, so a
/// body without an id gets a UUID here — and *never* reuses a client-supplied
/// id on POST, which is what `PUT` is for.
#[debug_handler]
async fn create(
    Path((version, rtype)): Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    // Refuse before touching the store: an unattributable write is exactly
    // what PR12 exists to prevent, so the token is checked first.
    let audit = match crate::auth::audit_from(&headers) {
        Ok(a) => a,
        Err(r) => return r,
    };
    let mut resource = match parse_body(&body, &rtype) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let id = uuid::Uuid::new_v4().to_string();
    if let Some(obj) = resource.as_object_mut() {
        obj.insert("id".to_string(), serde_json::json!(id));
    }

    // `If-None-Exist` makes this a conditional create (SV2.14). The store's
    // `conditional_create_audited` holds its write gate across the
    // search-then-create, so the sequence is indivisible with respect to
    // other writers — searching here and then calling `put_audited` would be
    // the same race with extra steps.
    if let Some(raw) = headers.get("if-none-exist") {
        // Present-but-unreadable must be an error, not an unconditional
        // create: dropping the precondition silently is the duplicate-writing
        // failure the header exists to prevent (SV2.14, SV2.5's reason).
        let criteria: Vec<(String, String)> = match raw
            .to_str()
            .ok()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| serde_urlencoded::from_str(s).ok())
        {
            Some(c) => c,
            None => {
                return outcome(
                    StatusCode::BAD_REQUEST,
                    "error",
                    "invalid",
                    "the If-None-Exist header is not readable as search criteria",
                );
            }
        };
        return match store
            .conditional_create_audited(&rtype, &criteria, &resource, &audit)
            .await
        {
            Ok(fhir_sqlite_store::CondCreate::Created(out)) => {
                created_response(&version, &rtype, &out, &resource)
            }
            // Exactly one match: FHIR says return it unchanged. The read goes
            // through the same disclosure logging as `GET` — a returned
            // resource is a disclosure whichever verb carried it.
            Ok(fhir_sqlite_store::CondCreate::Existing(existing_id)) => {
                match store.get(&rtype, &existing_id).await {
                    Ok(Some(mut body)) => {
                        let v = match store.status(&rtype, &existing_id).await {
                            Ok(fhir_sqlite_store::ResourceStatus::Active(v)) => Some(v),
                            _ => None,
                        };
                        if let (Some(v), Some(obj)) = (v, body.as_object_mut()) {
                            obj.insert(
                                "meta".to_string(),
                                serde_json::json!({ "versionId": v.to_string() }),
                            );
                        }
                        disclose(
                            store,
                            &headers,
                            "conditional-create",
                            Some(&rtype),
                            Some(&existing_id),
                            v,
                            "existing",
                            Some(1),
                        )
                        .await;
                        fhir_json(StatusCode::OK, &body, v)
                    }
                    Ok(None) => outcome(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error",
                        "exception",
                        "the matched resource could not be read back",
                    ),
                    Err(e) => store_error(e),
                }
            }
            Ok(fhir_sqlite_store::CondCreate::Multiple) => outcome(
                StatusCode::PRECONDITION_FAILED,
                "error",
                "multiple-matches",
                "If-None-Exist matched more than one resource; the criteria are not selective enough",
            ),
            Err(e) => store_error(e),
        };
    }

    match store.put_audited(&resource, None, &audit).await {
        Ok(out) => created_response(&version, &rtype, &out, &resource),
        Err(e) => store_error(e),
    }
}

/// The `201 Created` response both the plain and conditional create share.
fn created_response(
    version: &str,
    rtype: &str,
    out: &fhir_sqlite_store::PutOutcome,
    resource: &serde_json::Value,
) -> AxumResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, FHIR_JSON.parse().expect("static"));
    if let Ok(loc) = format!("/{version}/{rtype}/{}", out.id).parse() {
        headers.insert(header::LOCATION, loc);
    }
    if let Ok(tag) = format!("W/\"{}\"", out.version_id).parse() {
        headers.insert(header::ETAG, tag);
    }
    (
        StatusCode::CREATED,
        headers,
        serde_json::to_string(resource).unwrap_or_default(),
    )
        .into_response()
}

/// `PUT /{version}/{type}/{id}` — update, or create at a client-chosen id.
#[debug_handler]
async fn update(
    Path((version, rtype, id)): Path<(String, String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    let mut resource = match parse_body(&body, &rtype) {
        Ok(v) => v,
        Err(r) => return r,
    };
    // A body whose id disagrees with the URL is a mistake worth refusing rather
    // than resolving: either answer would silently discard what the caller
    // meant.
    match resource.get("id").and_then(serde_json::Value::as_str) {
        Some(b) if b != id => {
            return outcome(
                StatusCode::BAD_REQUEST,
                "error",
                "invariant",
                &format!("body id {b:?} does not match {id:?} in the URL"),
            );
        }
        _ => {
            if let Some(obj) = resource.as_object_mut() {
                obj.insert("id".to_string(), serde_json::json!(id));
            }
        }
    }
    // Refuse before touching the store: an unattributable write is exactly
    // what PR12 exists to prevent, so the token is checked first.
    let audit = match crate::auth::audit_from(&headers) {
        Ok(a) => a,
        Err(r) => return r,
    };
    let expected = match if_match(&headers) {
        Ok(v) => v,
        Err(r) => return r,
    };
    match store.put_audited(&resource, expected, &audit).await {
        Ok(out) => {
            let status = if out.created {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            fhir_json(status, &resource, Some(out.version_id))
        }
        Err(e) => store_error(e),
    }
}

/// `DELETE /{version}/{type}/{id}`
///
/// Deleting something already gone is not an error: FHIR treats delete as
/// idempotent, and a client retrying after a dropped response must not get a
/// failure for having succeeded.
#[debug_handler]
async fn delete_(
    Path((version, rtype, id)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    // Refuse before touching the store: an unattributable write is exactly
    // what PR12 exists to prevent, so the token is checked first.
    let audit = match crate::auth::audit_from(&headers) {
        Ok(a) => a,
        Err(r) => return r,
    };
    match store.delete_audited(&rtype, &id, &audit).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => store_error(e),
    }
}

/// `SV2.16`: more included resources than this refuses the request.
const INCLUDE_CAP: i64 = 1000;

/// Parse and validate one `_include`/`_revinclude` value (`SV2.16`):
/// `<type>:<param>` with an optional third segment. Everything invalid is
/// refused **by name** — a silently dropped include returns less than the
/// client asked for while looking complete.
fn include_spec(
    store: &store::AnyStore,
    spec: &str,
    rev: bool,
    searched: &str,
) -> Result<(String, String, Option<String>), Box<AxumResponse>> {
    let what = if rev { "_revinclude" } else { "_include" };
    let refuse = |msg: String| {
        Err(Box::new(outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "not-supported",
            &msg,
        )))
    };
    let parts: Vec<&str> = spec.split(':').collect();
    if parts.contains(&"iterate") {
        return refuse(format!(
            "{what}={spec}: iterated includes (:iterate) are not served (SV2.16)"
        ));
    }
    let [src, param, rest @ ..] = parts.as_slice() else {
        return refuse(format!("{what}={spec}: expected <type>:<param> (SV2.16)"));
    };
    if rest.len() > 1 {
        return refuse(format!(
            "{what}={spec}: expected <type>:<param>[:<target-type>] (SV2.16)"
        ));
    }
    let target = rest.first().map(|s| (*s).to_string());
    if !rev && *src != searched {
        return refuse(format!(
            "{what}={spec}: the source type must be the searched type \
             {searched} (SV2.16)"
        ));
    }
    if rev {
        if let Some(t) = &target {
            if t != searched {
                return refuse(format!(
                    "{what}={spec}: the target type of a _revinclude is the \
                     searched type {searched} (SV2.16)"
                ));
            }
        }
    }
    let Some(rm) = store.map().resources.get(*src) else {
        return refuse(format!("{what}={spec}: unknown resource type {src}"));
    };
    let Some(def) = rm.search.iter().find(|d| d.code == *param) else {
        return refuse(format!(
            "{what}={spec}: {src} has no search parameter {param:?}"
        ));
    };
    let is_ref = def
        .targets
        .iter()
        .any(|t| matches!(t.kind, fhir_sqlite_map::model::TargetKind::Reference { .. }));
    if !is_ref {
        return refuse(format!(
            "{what}={spec}: {param:?} is not a reference parameter (SV2.16)"
        ));
    }
    Ok(((*src).to_string(), (*param).to_string(), target))
}

/// `GET /{version}/{type}?name=value…` — search.
#[debug_handler]
async fn search(
    Path((version, rtype)): Path<(String, String)>,
    Query(params): Query<Vec<(String, String)>>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    // `_count`/`_offset`/`_total` are FHIR control parameters, not search
    // criteria; passing them through would look like an unknown search
    // parameter and fail the whole request.
    let mut count: i64 = 50;
    let mut offset: i64 = 0;
    let mut want_total = false;
    let mut criteria: Vec<(String, String)> = Vec::new();
    let mut includes: Vec<String> = Vec::new();
    let mut revincludes: Vec<String> = Vec::new();
    for (k, v) in params {
        match k.as_str() {
            "_count" => count = v.parse().unwrap_or(50).clamp(1, 1000),
            "_offset" => offset = v.parse().unwrap_or(0).max(0),
            "_total" => want_total = v != "none",
            "_include" => includes.push(v),
            "_revinclude" => revincludes.push(v),
            // Iteration is transitive closure; refusing by name beats
            // pretending to bound it (SV2.16).
            "_include:iterate" | "_revinclude:iterate" => {
                return outcome(
                    StatusCode::BAD_REQUEST,
                    "error",
                    "not-supported",
                    "iterated includes (:iterate) are not served (SV2.16)",
                );
            }
            _ => criteria.push((k, v)),
        }
    }
    let page = match store
        .search_full(&rtype, &criteria, count, offset, want_total)
        .await
    {
        Ok(p) => p,
        Err(e) => return store_error(e),
    };
    // SV2.16: includes are computed from this page's matches, deduplicated,
    // and never repeat a match.
    let mut included: std::collections::BTreeSet<(String, String)> =
        std::collections::BTreeSet::new();
    for spec in &includes {
        let (_, param, target) = match include_spec(store, spec, false, &rtype) {
            Ok(t) => t,
            Err(r) => return *r,
        };
        let pairs = match store.refs_of(&rtype, &page.ids, &param).await {
            Ok(p) => p,
            Err(e) => return store_error(e),
        };
        for (t, id) in pairs {
            if target.as_deref().is_none_or(|f| f == t) {
                included.insert((t, id));
            }
        }
    }
    for spec in &revincludes {
        let (src, param, _) = match include_spec(store, spec, true, &rtype) {
            Ok(t) => t,
            Err(r) => return *r,
        };
        for id in &page.ids {
            let hits = match store
                .search(
                    &src,
                    &[(param.clone(), format!("{rtype}/{id}"))],
                    INCLUDE_CAP + 1,
                    0,
                )
                .await
            {
                Ok(h) => h,
                Err(e) => return store_error(e),
            };
            for hid in hits {
                included.insert((src.clone(), hid));
            }
        }
    }
    for id in &page.ids {
        included.remove(&(rtype.clone(), id.clone()));
    }
    if included.len() > usize::try_from(INCLUDE_CAP).unwrap_or(usize::MAX) {
        // A truncated include silently returns less than it claims; refusing
        // names the cap instead (SV2.16, C0.11's shape).
        return outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "too-costly",
            &format!(
                "this search would include more than {INCLUDE_CAP} resources; \
                 narrow it (SV2.16)"
            ),
        );
    }

    let mut entries = Vec::with_capacity(page.ids.len() + included.len());
    for id in &page.ids {
        match store.get(&rtype, id).await {
            Ok(Some(resource)) => entries.push(serde_json::json!({
                "fullUrl": format!("/{version}/{rtype}/{id}"),
                "resource": resource,
                "search": { "mode": "match" },
            })),
            // A result that vanished between the search and the read is a race,
            // not a failure: skip it rather than fail the page.
            Ok(None) => tracing::debug!(%rtype, %id, "search hit disappeared before read"),
            Err(e) => return store_error(e),
        }
    }
    for (t, id) in &included {
        match store.get(t, id).await {
            Ok(Some(resource)) => entries.push(serde_json::json!({
                "fullUrl": format!("/{version}/{t}/{id}"),
                "resource": resource,
                "search": { "mode": "include" },
            })),
            // Dangling references are data, not a request error (SV2.16).
            Ok(None) => tracing::debug!(%t, %id, "included reference does not resolve"),
            Err(e) => return store_error(e),
        }
    }
    disclose(
        store,
        &headers,
        "search",
        Some(&rtype),
        None,
        None,
        "ok",
        i64::try_from(entries.len()).ok(),
    )
    .await;
    fhir_json(
        StatusCode::OK,
        &bundle("searchset", entries, page.total),
        None,
    )
}

/// `GET /{version}/{type}/{id}/_history`
#[debug_handler]
async fn history(
    Path((version, rtype, id)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    match store.history(&rtype, &id).await {
        Ok(entries) if entries.is_empty() => outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            &format!("{rtype}/{id} has no history"),
        ),
        Ok(entries) => {
            let out: Vec<serde_json::Value> = entries
                .iter()
                .map(|e| {
                    // A deletion is a real entry with no resource. Emitting it
                    // is the point: history that hid its deletions would not be
                    // an audit trail.
                    let method = match e.op {
                        'C' => "POST",
                        'D' => "DELETE",
                        'X' => "DELETE",
                        _ => "PUT",
                    };
                    let mut entry = serde_json::json!({
                        "fullUrl": format!("/{version}/{rtype}/{id}"),
                        "request": { "method": method, "url": format!("{rtype}/{id}") },
                        "response": {
                            "status": if e.resource.is_some() { "200" } else { "204" },
                            "etag": format!("W/\"{}\"", e.version_id),
                            "lastModified": e.last_updated,
                        },
                    });
                    if let (Some(r), Some(o)) = (&e.resource, entry.as_object_mut()) {
                        o.insert("resource".to_string(), r.clone());
                    }
                    entry
                })
                .collect();
            let total = i64::try_from(out.len()).ok();
            disclose(
                store,
                &headers,
                "history",
                Some(&rtype),
                Some(&id),
                None,
                "ok",
                total,
            )
            .await;
            fhir_json(StatusCode::OK, &bundle("history", out, total), None)
        }
        Err(e) => store_error(e),
    }
}

/// `GET /{version}/{type}/_history` (`SV2.17`).
#[debug_handler]
async fn type_history(
    Path((version, rtype)): Path<(String, String)>,
    Query(params): Query<Vec<(String, String)>>,
    headers: HeaderMap,
) -> AxumResponse {
    scoped_history(version, Some(rtype), params, headers).await
}

/// `GET /{version}/_history` (`SV2.17`).
#[debug_handler]
async fn system_history(
    Path(version): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
    headers: HeaderMap,
) -> AxumResponse {
    scoped_history(version, None, params, headers).await
}

async fn scoped_history(
    version: String,
    rtype: Option<String>,
    params: Vec<(String, String)>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    let mut count: i64 = 50;
    let mut since: Option<String> = None;
    for (k, v) in params {
        match k.as_str() {
            "_count" => count = v.parse().unwrap_or(50).clamp(1, 1000),
            "_since" => {
                // A malformed instant silently compared as text would return
                // wrong slices while looking right (SV2.17).
                if chrono::DateTime::parse_from_rfc3339(&v).is_err() {
                    return outcome(
                        StatusCode::BAD_REQUEST,
                        "error",
                        "invalid",
                        &format!("_since must be an RFC 3339 instant, got {v:?} (SV2.17)"),
                    );
                }
                since = Some(v);
            }
            other => {
                // A silently dropped filter returns more than was asked.
                return outcome(
                    StatusCode::BAD_REQUEST,
                    "error",
                    "not-supported",
                    &format!("history does not serve parameter {other:?} (SV2.17)"),
                );
            }
        }
    }
    let rows = match store
        .history_page(rtype.as_deref(), count, since.as_deref())
        .await
    {
        Ok(r) => r,
        Err(e) => return store_error(e),
    };
    let out: Vec<serde_json::Value> = rows
        .iter()
        .map(|(t, id, e)| {
            let method = match e.op {
                'C' => "POST",
                'D' | 'X' => "DELETE",
                _ => "PUT",
            };
            let mut entry = serde_json::json!({
                "fullUrl": format!("/{version}/{t}/{id}"),
                "request": { "method": method, "url": format!("{t}/{id}") },
                "response": {
                    "status": if e.resource.is_some() { "200" } else { "204" },
                    "etag": format!("W/\"{}\"", e.version_id),
                    "lastModified": e.last_updated,
                },
            });
            if let (Some(r), Some(o)) = (&e.resource, entry.as_object_mut()) {
                o.insert("resource".to_string(), r.clone());
            }
            entry
        })
        .collect();
    let total = i64::try_from(out.len()).ok();
    disclose(
        store,
        &headers,
        "history",
        rtype.as_deref(),
        None,
        None,
        "ok",
        total,
    )
    .await;
    fhir_json(StatusCode::OK, &bundle("history", out, total), None)
}

/// `POST /{version}` — transaction/batch Bundles, refused by name
/// (`SV2.18`): the decision is served to clients, not left as a bare 405.
#[debug_handler]
async fn system_post(Path(version): Path<String>, body: Bytes) -> AxumResponse {
    if let Err(r) = version_of(&version) {
        return r;
    }
    // `Bytes`, not `axum::Json`: the body arrives as application/fhir+json,
    // which the Json extractor answers with 415.
    let b: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            return outcome(
                StatusCode::BAD_REQUEST,
                "error",
                "invalid",
                &format!("body is not JSON: {e}"),
            );
        }
    };
    if b["resourceType"] != "Bundle" {
        return outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "invalid",
            "the system endpoint takes a Bundle (SV2.18)",
        );
    }
    match b["type"].as_str() {
        Some("transaction") => outcome(
            StatusCode::NOT_IMPLEMENTED,
            "error",
            "not-supported",
            "transaction Bundles are not served: a FHIR transaction is atomic \
             by definition, and this store cannot yet hold one transaction \
             across the operations — emulating atomicity by compensation was \
             rejected because readers between operations would observe a \
             half-applied bundle (SV2.18)",
        ),
        Some("batch") => outcome(
            StatusCode::NOT_IMPLEMENTED,
            "error",
            "not-supported",
            "batch Bundles are not served yet — unbuilt rather than rejected; \
             batch claims no atomicity (SV2.18)",
        ),
        other => outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "invalid",
            &format!("unsupported Bundle type {other:?} (SV2.18)"),
        ),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/{version}/metadata", get(metadata))
        .add("/{version}", post(system_post))
        .add("/{version}/_history", get(system_history))
        .add("/{version}/{rtype}", get(search).post(create))
        .add("/{version}/{rtype}/_history", get(type_history))
        .add(
            "/{version}/{rtype}/{id}",
            get(read).put(update).delete(delete_),
        )
        .add("/{version}/{rtype}/{id}/_history", get(history))
        .add("/{version}/{rtype}/{id}/_history/{vid}", get(vread))
}
