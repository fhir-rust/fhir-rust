//! Live-database round trip: put every corpus example through PostgreSQL and
//! require the reconstruction to be semantically identical.
//!
//! Live tests. They find their server automatically — see `common::test_db`:
//! FHIR_POSTGRESQL_TEST_DB if set, otherwise the `scripts/db.sh` container if
//! it is listening. A skip is printed, not silent.
//! FHIR_POSTGRESQL_TEST_CORPUS_LIMIT bounds the number of examples (default 400;
//! 0 = unlimited).

use std::path::PathBuf;
use std::sync::Arc;

use serde_json::Value;

mod common;

fn spec_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_POSTGRESQL_SPEC_DIR")
            .ok()
            .map(PathBuf::from),
        // This monorepo: the model family carries the specification packages.
        Some(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../fhir/doc/fhir-specifications"
        ))),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

fn corpus_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_POSTGRESQL_CORPUS_DIR")
            .ok()
            .map(PathBuf::from),
        Some(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../target/test-corpus"
        ))),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

fn sem_eq(a: &Value, b: &Value, path: &str, diffs: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ma), Value::Object(mb)) => {
            for (k, va) in ma {
                match mb.get(k) {
                    Some(vb) => sem_eq(va, vb, &format!("{path}.{k}"), diffs),
                    None => diffs.push(format!("{path}.{k}: missing after round trip")),
                }
            }
            for k in mb.keys() {
                if !ma.contains_key(k) {
                    diffs.push(format!("{path}.{k}: extra after round trip"));
                }
            }
        }
        (Value::Array(aa), Value::Array(ab)) => {
            if aa.len() != ab.len() {
                diffs.push(format!("{path}: array length {} vs {}", aa.len(), ab.len()));
                return;
            }
            for (i, (va, vb)) in aa.iter().zip(ab).enumerate() {
                sem_eq(va, vb, &format!("{path}[{i}]"), diffs);
            }
        }
        (Value::Number(na), Value::Number(nb)) => {
            // jsonb normalizes number spellings (1e2 → 100); compare
            // numerically before declaring a diff.
            if na != nb && !num_eq(&na.to_string(), &nb.to_string()) {
                diffs.push(format!("{path}: {a} vs {b}"));
            }
        }
        _ => {
            if a != b {
                diffs.push(format!("{path}: {a} vs {b}"));
            }
        }
    }
}

/// Decimal-exact numeric comparison of two JSON number literals.
fn num_eq(a: &str, b: &str) -> bool {
    fn norm(s: &str) -> Option<(bool, String, i64)> {
        let s = s.trim();
        let (neg, s) = match s.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, s),
        };
        let (mant, exp) = match s.split_once(['e', 'E']) {
            Some((m, e)) => (m, e.parse::<i64>().ok()?),
            None => (s, 0),
        };
        let (int, frac) = match mant.split_once('.') {
            Some((i, f)) => (i, f),
            None => (mant, ""),
        };
        let digits = format!("{int}{frac}");
        let scale = exp - frac.len() as i64;
        let trimmed = digits.trim_start_matches('0');
        let (trimmed, scale) = {
            let t2 = trimmed.trim_end_matches('0');
            let dropped = (trimmed.len() - t2.len()) as i64;
            (t2.to_string(), scale + dropped)
        };
        if trimmed.is_empty() {
            return Some((false, String::new(), 0));
        }
        Some((neg, trimmed, scale))
    }
    matches!((norm(a), norm(b)), (Some(x), Some(y)) if x == y)
}

#[tokio::test]
async fn live_roundtrip_corpus() {
    let Some(_db) = common::test_db() else {
        return;
    };
    let (Some(spec), Some(corpus)) = (spec_root(), corpus_root()) else {
        eprintln!("skipping: spec or corpus missing");
        return;
    };
    let limit: usize = std::env::var("FHIR_POSTGRESQL_TEST_CORPUS_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(400);

    let mut failures: Vec<String> = Vec::new();
    let mut total = 0usize;
    for (cdir, sdir) in [("stu3", "r3"), ("r4", "r4"), ("r5", "r5")] {
        let defs = spec.join(sdir).join("fhir-definitions-json");
        let ex = corpus.join(cdir);
        if !defs.exists() || !ex.exists() {
            continue;
        }
        let map = Arc::new(fhir_postgresql_gen::generate(&defs, sdir).expect("generate"));
        let cfg = fhir_postgresql_store::pg_config(None).expect("pg config");
        let store = fhir_postgresql_store::Store::connect(cfg, map.clone())
            .await
            .expect("connect");
        // Fresh install per version under this test database.
        store.drop_schema().await.expect("drop schema");
        store.init("test-checksum").await.expect("init");

        let mut entries: Vec<_> = std::fs::read_dir(&ex)
            .expect("dir")
            .map(|e| e.expect("entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        entries.sort();
        if limit > 0 {
            entries.truncate(limit);
        }
        let mut count = 0usize;
        for path in entries {
            let bytes = std::fs::read(&path).expect("read");
            let Ok(v): Result<Value, _> = serde_json::from_slice(&bytes) else {
                continue;
            };
            let Some(rt) = v.get("resourceType").and_then(Value::as_str) else {
                continue;
            };
            if !map.resources.contains_key(rt) || v.get("id").and_then(Value::as_str).is_none() {
                continue;
            }
            let name = path.file_name().expect("n").to_string_lossy().to_string();
            count += 1;
            let put = match store.put(&v).await {
                Ok(p) => p,
                Err(e) => {
                    failures.push(format!("[{sdir}] {name}: put: {e}"));
                    continue;
                }
            };
            let got = match store.get(rt, &put.id).await {
                Ok(Some(g)) => g,
                Ok(None) => {
                    failures.push(format!("[{sdir}] {name}: vanished after put"));
                    continue;
                }
                Err(e) => {
                    failures.push(format!("[{sdir}] {name}: get: {e}"));
                    continue;
                }
            };
            let mut diffs = Vec::new();
            sem_eq(&v, &got.resource, "$", &mut diffs);
            if !diffs.is_empty() {
                diffs.truncate(3);
                failures.push(format!("[{sdir}] {name}: {}", diffs.join(" | ")));
            }
        }
        eprintln!("{sdir}: {count} live round trips");
        total += count;
        if failures.len() > 60 {
            break;
        }
    }
    assert!(total > 0, "no examples ran");
    let shown: Vec<_> = failures.iter().take(60).cloned().collect();
    assert!(
        failures.is_empty(),
        "{} live round-trip failures:\n{}",
        failures.len(),
        shown.join("\n")
    );
}
