//! Bulk Data `$export` (`SV2.15`): the conformant async slice.
//!
//! The Bulk Data protocol *is* the async contract: kick-off answers `202`
//! with a `Content-Location`, the client polls that status URL, completion
//! lists NDJSON files, `DELETE` cancels and cleans up. This module serves
//! the system-level export only — `_type` filters it; `_since` and the
//! compartment-based `Patient/$export` are refused by name, not ignored
//! (`SV2.13`'s principle: a dropped filter returns more than was asked).
//!
//! Jobs run in-process on a spawned task and live in a registry; files are
//! written under a per-job directory and every file fetch is
//! disclosure-logged — an export is the largest disclosure this server can
//! make, and `PR12` wants each one recorded. Exported PHI on disk has a
//! lifetime: a sweep on each kick-off removes jobs past their TTL.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use axum::extract::{Path, Query};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::prelude::*;

use super::fhir::{disclose, outcome, version_of};

const NDJSON: &str = "application/fhir+ndjson";

/// One completed output file.
#[derive(Clone)]
struct Output {
    rtype: String,
    count: u64,
}

enum JobStatus {
    Running { done: usize, total: usize },
    Complete,
    Failed(String),
}

struct Job {
    version: String,
    request_url: String,
    transaction_time: String,
    status: JobStatus,
    outputs: Vec<Output>,
    dir: PathBuf,
    created: Instant,
    cancelled: Arc<AtomicBool>,
}

fn jobs() -> &'static Mutex<HashMap<String, Job>> {
    static JOBS: OnceLock<Mutex<HashMap<String, Job>>> = OnceLock::new();
    JOBS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Where export files live: `FHIR_LOCO_EXPORT_DIR`, defaulting to a sibling
/// of the database file so tests and deployments stay self-contained.
fn export_root() -> PathBuf {
    if let Some(dir) = std::env::var_os("FHIR_LOCO_EXPORT_DIR") {
        return PathBuf::from(dir);
    }
    // RCA-IN-3: absent only (the database file's default name, as in app.rs)
    let mut db = std::env::var_os("FHIR_LOCO_DB").unwrap_or_else(|| "fhir.sqlite".into());
    db.push(".exports");
    PathBuf::from(db)
}

/// The export lifetime. `FHIR_LOCO_EXPORT_TTL_SECS` is validated at startup
/// (RCA-IN-4); an invalid value found here is an error, never the default.
fn ttl() -> Result<std::time::Duration, String> {
    crate::config::export_ttl_secs().map(std::time::Duration::from_secs)
}

/// Remove jobs past their TTL, and their files. Exported PHI is a disclosure
/// with a lifetime; nothing here may outlive it silently.
fn sweep_expired() -> Result<(), String> {
    let ttl = ttl()?;
    let mut jobs = jobs().lock().expect("registry");
    let expired: Vec<String> = jobs
        .iter()
        .filter(|(_, j)| j.created.elapsed() > ttl)
        .map(|(id, _)| id.clone())
        .collect();
    for id in expired {
        if let Some(job) = jobs.remove(&id) {
            job.cancelled.store(true, Ordering::Relaxed);
            let _ = std::fs::remove_dir_all(&job.dir);
            tracing::info!(job = %id, "export job expired and was removed");
        }
    }
    Ok(())
}

/// Remove export directories under `root` that no live job owns.
///
/// The registry is in memory, so after a restart every directory a previous
/// process left is unreachable — no status URL resolves to it — yet still
/// holds exported PHI on disk with nothing left to expire it. Only
/// subdirectories named as a job id (a UUID) are touched: a misconfigured
/// `FHIR_LOCO_EXPORT_DIR` pointing somewhere shared must not cost anyone
/// their files. Returns how many were removed. A missing root is zero.
fn remove_orphans(
    root: &std::path::Path,
    live: &std::collections::HashSet<String>,
) -> std::io::Result<usize> {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e),
    };
    let mut removed = 0;
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if uuid::Uuid::parse_str(name).is_err()
            || live.contains(name)
            || !entry.file_type()?.is_dir()
        {
            continue;
        }
        std::fs::remove_dir_all(entry.path())?;
        removed += 1;
    }
    Ok(removed)
}

/// At boot: delete what a previous process left behind (see
/// [`remove_orphans`]), then sweep expired jobs on a timer — every
/// `min(ttl, 60 s)` — rather than only when the next kick-off happens to
/// arrive, which on a quiet server could be never.
///
/// # Errors
/// If the TTL is invalid or the export directory cannot be read; both stop
/// the boot, because the alternative is PHI on disk with no lifetime.
pub fn start_retention() -> Result<(), String> {
    let ttl = ttl()?;
    let root = export_root();
    let live: std::collections::HashSet<String> =
        jobs().lock().expect("registry").keys().cloned().collect();
    let removed = remove_orphans(&root, &live)
        .map_err(|e| format!("export directory {}: {e}", root.display()))?;
    if removed > 0 {
        tracing::warn!(removed, "removed export files orphaned by a restart");
    }
    let every = ttl.min(std::time::Duration::from_secs(60));
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(every);
        loop {
            tick.tick().await;
            if let Err(message) = sweep_expired() {
                tracing::error!(error = %message, "export sweep failed");
            }
        }
    });
    Ok(())
}

/// `GET /{version}/$export` — kick-off.
#[debug_handler]
async fn kickoff(
    Path(version): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
    headers: HeaderMap,
) -> AxumResponse {
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    // An export is the largest read this server can perform; it is
    // authenticated like every write, before anything else happens.
    if let Err(r) = crate::auth::audit_from(&headers) {
        return r;
    }
    // The Bulk Data kick-off SHALL carry `Prefer: respond-async`. A missing
    // preference is refused rather than defaulted: a client that did not ask
    // for the async contract would misread the 202.
    let prefers_async = headers
        .get("prefer")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.split(',').any(|p| p.trim() == "respond-async"));
    if !prefers_async {
        return outcome(
            StatusCode::BAD_REQUEST,
            "error",
            "invalid",
            "$export requires the header `Prefer: respond-async` (Bulk Data kick-off)",
        );
    }

    // Parameters: `_type` filters; everything else is refused by name.
    // Silently ignoring `_since` would return more than was asked for, which
    // is the failure SV2.13 exists to prevent.
    let mut types: Vec<String> = Vec::new();
    for (k, v) in &params {
        match k.as_str() {
            "_type" => {
                for t in v.split(',').map(str::trim).filter(|t| !t.is_empty()) {
                    if !store.map().resources.contains_key(t) {
                        return outcome(
                            StatusCode::BAD_REQUEST,
                            "error",
                            "not-supported",
                            &format!("_type names {t:?}, which this version does not serve"),
                        );
                    }
                    types.push(t.to_string());
                }
            }
            other => {
                return outcome(
                    StatusCode::BAD_REQUEST,
                    "error",
                    "not-supported",
                    &format!(
                        "$export does not support the parameter {other:?} \
                         (supported: _type); refusing rather than ignoring it"
                    ),
                );
            }
        }
    }
    if types.is_empty() {
        types = store.map().resources.keys().cloned().collect();
    }
    types.sort();
    types.dedup();

    if let Err(message) = sweep_expired() {
        tracing::error!(error = %message, "export configuration is invalid");
        return outcome(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error",
            "exception",
            "the export service is misconfigured",
        );
    }

    let id = uuid::Uuid::new_v4().to_string();
    let dir = export_root().join(&id);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        tracing::error!(error = %e, "export directory could not be created");
        return outcome(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error",
            "exception",
            "the export directory could not be created",
        );
    }

    let cancelled = Arc::new(AtomicBool::new(false));
    let job = Job {
        version: version.clone(),
        request_url: format!("/{version}/$export"),
        transaction_time: jiff::Timestamp::now().to_string(),
        status: JobStatus::Running {
            done: 0,
            total: types.len(),
        },
        outputs: Vec::new(),
        dir: dir.clone(),
        created: Instant::now(),
        cancelled: Arc::clone(&cancelled),
    };
    jobs().lock().expect("registry").insert(id.clone(), job);

    let store = store.clone();
    let worker_id = id.clone();
    tokio::spawn(async move {
        run_export(&store, &worker_id, &types, &dir, &cancelled).await;
    });

    let mut response_headers = HeaderMap::new();
    if let Ok(loc) = format!("/{version}/$export-status/{id}").parse() {
        response_headers.insert(header::CONTENT_LOCATION, loc);
    }
    (StatusCode::ACCEPTED, response_headers).into_response()
}

/// The worker: page every requested type through the store, one NDJSON file
/// per type that has any resources. Each read is a store-level snapshot read;
/// the export as a whole is **not** one snapshot, and the spec says so.
async fn run_export(
    store: &crate::store::AnyStore,
    id: &str,
    types: &[String],
    dir: &std::path::Path,
    cancelled: &AtomicBool,
) {
    use std::io::Write;

    let mut outputs = Vec::new();
    for (i, rtype) in types.iter().enumerate() {
        if cancelled.load(Ordering::Relaxed) {
            return; // DELETE already cleaned up the registry entry.
        }
        let mut count: u64 = 0;
        let mut file: Option<std::fs::File> = None;
        let mut after: Option<String> = None;
        loop {
            let page = match store
                .search_page(rtype, &[], 500, 0, false, after.as_deref())
                .await
            {
                Ok(p) => p,
                Err(e) => return fail(id, &format!("listing {rtype}: {e}")),
            };
            let Some(last) = page.ids.last().cloned() else {
                break;
            };
            for rid in &page.ids {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }
                match store.get(rtype, rid).await {
                    Ok(Some(resource)) => {
                        let f = match &mut file {
                            Some(f) => f,
                            None => {
                                match std::fs::File::create(dir.join(format!("{rtype}.ndjson"))) {
                                    Ok(f) => file.insert(f),
                                    Err(e) => {
                                        return fail(id, &format!("creating {rtype} file: {e}"))
                                    }
                                }
                            }
                        };
                        let line = serde_json::to_string(&resource).unwrap_or_default();
                        if let Err(e) = writeln!(f, "{line}") {
                            return fail(id, &format!("writing {rtype} file: {e}"));
                        }
                        count += 1;
                    }
                    // Deleted between listing and read: a race, not a failure.
                    Ok(None) => {}
                    Err(e) => return fail(id, &format!("reading {rtype}: {e}")),
                }
            }
            after = Some(last);
            if page.ids.len() < 500 {
                break;
            }
        }
        if count > 0 {
            outputs.push(Output {
                rtype: rtype.clone(),
                count,
            });
        }
        if let Some(job) = jobs().lock().expect("registry").get_mut(id) {
            job.status = JobStatus::Running {
                done: i + 1,
                total: types.len(),
            };
        }
    }
    if let Some(job) = jobs().lock().expect("registry").get_mut(id) {
        job.outputs = outputs;
        job.status = JobStatus::Complete;
    }
}

fn fail(id: &str, message: &str) {
    // The message describes the operation, never resource content (O10.2).
    tracing::error!(job = %id, %message, "export failed");
    if let Some(job) = jobs().lock().expect("registry").get_mut(id) {
        job.status = JobStatus::Failed(message.to_string());
    }
}

/// `GET /{version}/$export-status/{job}` — poll, per the Bulk Data contract.
#[debug_handler]
async fn status(Path((version, id)): Path<(String, String)>, headers: HeaderMap) -> AxumResponse {
    if let Err(r) = crate::auth::audit_from(&headers) {
        return r;
    }
    let jobs = jobs().lock().expect("registry");
    let Some(job) = jobs.get(&id).filter(|j| j.version == version) else {
        return outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            "no such export job",
        );
    };
    match &job.status {
        JobStatus::Running { done, total } => {
            let mut h = HeaderMap::new();
            if let Ok(v) = format!("{done}/{total} resource types").parse() {
                h.insert("x-progress", v);
            }
            (StatusCode::ACCEPTED, h).into_response()
        }
        JobStatus::Failed(message) => outcome(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error",
            "exception",
            message,
        ),
        JobStatus::Complete => {
            let body = serde_json::json!({
                "transactionTime": job.transaction_time,
                "request": job.request_url,
                // Every file URL goes through the same PASETO gate as the API.
                "requiresAccessToken": true,
                "output": job.outputs.iter().map(|o| serde_json::json!({
                    "type": o.rtype,
                    "url": format!("/{version}/$export-file/{id}/{}", o.rtype),
                    "count": o.count,
                })).collect::<Vec<_>>(),
                "error": [],
            });
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                serde_json::to_string(&body).unwrap_or_default(),
            )
                .into_response()
        }
    }
}

/// `DELETE /{version}/$export-status/{job}` — cancel or clean up.
#[debug_handler]
async fn cancel(Path((version, id)): Path<(String, String)>, headers: HeaderMap) -> AxumResponse {
    if let Err(r) = crate::auth::audit_from(&headers) {
        return r;
    }
    let mut jobs = jobs().lock().expect("registry");
    let Some(job) = jobs.get(&id).filter(|j| j.version == version) else {
        return outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            "no such export job",
        );
    };
    job.cancelled.store(true, Ordering::Relaxed);
    let dir = job.dir.clone();
    jobs.remove(&id);
    let _ = std::fs::remove_dir_all(&dir);
    StatusCode::ACCEPTED.into_response()
}

/// `GET /{version}/$export-file/{job}/{rtype}` — one NDJSON output.
#[debug_handler]
async fn file(
    Path((version, id, rtype)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> AxumResponse {
    if let Err(r) = crate::auth::audit_from(&headers) {
        return r;
    }
    let store = match version_of(&version) {
        Ok(s) => s,
        Err(r) => return r,
    };
    let (path, count) = {
        let jobs = jobs().lock().expect("registry");
        let Some(job) = jobs
            .get(&id)
            .filter(|j| j.version == version && matches!(j.status, JobStatus::Complete))
        else {
            return outcome(
                StatusCode::NOT_FOUND,
                "error",
                "not-found",
                "no such export output",
            );
        };
        let Some(output) = job.outputs.iter().find(|o| o.rtype == rtype) else {
            return outcome(
                StatusCode::NOT_FOUND,
                "error",
                "not-found",
                "no such export output",
            );
        };
        (job.dir.join(format!("{rtype}.ndjson")), output.count)
    };
    match std::fs::read(&path) {
        Ok(bytes) => {
            // The whole file is a disclosure — count says how large a one.
            // `None` (unknown) when the count has no `i64`, never a wrong number.
            disclose(
                store,
                &headers,
                "export",
                Some(&rtype),
                None,
                None,
                "ok",
                i64::try_from(count).ok(),
            )
            .await;
            (StatusCode::OK, [(header::CONTENT_TYPE, NDJSON)], bytes).into_response()
        }
        Err(_) => outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            "no such export output",
        ),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/{version}/$export", get(kickoff))
        .add("/{version}/$export-status/{id}", get(status).delete(cancel))
        .add("/{version}/$export-file/{id}/{rtype}", get(file))
        .layer(axum::middleware::from_fn(super::fhir::require_credential))
}

#[cfg(test)]
mod tests {
    use super::remove_orphans;

    /// Orphaned job directories go; live jobs, non-UUID names and plain
    /// files stay. Mutation-verified: dropping the UUID check removes
    /// `keep-me`, dropping the live check removes the live job.
    #[test]
    fn orphaned_export_directories_are_removed_and_nothing_else() {
        let root = std::env::temp_dir().join(format!("fhir-loco-orphans-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let orphan = uuid::Uuid::new_v4().to_string();
        let live = uuid::Uuid::new_v4().to_string();
        let stray_file = uuid::Uuid::new_v4().to_string();
        for d in [&orphan, &live, &"keep-me".to_string()] {
            std::fs::create_dir_all(root.join(d)).expect("dir");
            std::fs::write(root.join(d).join("Patient.ndjson"), "{}\n").expect("file");
        }
        std::fs::write(root.join(&stray_file), "x").expect("file");

        let live_set = std::iter::once(live.clone()).collect();
        assert_eq!(remove_orphans(&root, &live_set).expect("sweep"), 1);
        assert!(!root.join(&orphan).exists(), "the orphan is removed");
        assert!(root.join(&live).exists(), "a live job is kept");
        assert!(
            root.join("keep-me").exists(),
            "a non-job name is never touched"
        );
        assert!(
            root.join(&stray_file).exists(),
            "a plain file is never touched"
        );

        let missing = root.join("absent");
        assert_eq!(
            remove_orphans(&missing, &live_set).expect("missing root"),
            0
        );
        std::fs::remove_dir_all(&root).ok();
    }
}
