//! Full-corpus round trip: every example shipped with the FHIR
//! specifications (examples-json.zip), when a corpus directory is present.
//! Set FHIR_ORACLE_CORPUS_DIR to a directory containing r5/, r4/, stu3/
//! subdirectories of raw example JSON files.

use std::collections::HashMap;
use std::path::PathBuf;

use fhir_oracle_map::reconstruct::{InRow, ReconIn, reconstruct};
use fhir_oracle_map::shred::{SqlVal, shred};
use serde_json::Value;

/// Where the FHIR example corpus lives, if it has been fetched.
///
/// `FHIR_ORACLE_CORPUS_DIR` first, then `corpus/` beside the workspace root —
/// the same name and layout the CI fetch step creates, so a local run and a
/// CI run look for it in the same place.
///
/// The previous fallback was an absolute path into one machine's temporary
/// scratchpad. It resolved nowhere in CI, so this test silently skipped
/// there, and on the one machine where the directory survived it resolved to
/// an *empty* corpus — which failed the `total > 1000` assertion with
/// "expected a real corpus" rather than skipping. A hardcoded absolute path
/// makes a test pass, fail, or vanish depending on whose disk it runs on.
fn corpus_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_ORACLE_CORPUS_DIR")
            .ok()
            .map(PathBuf::from),
        Some(workspace_root().join("corpus")),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

/// The workspace root, derived from this crate's manifest rather than the
/// current directory, which `cargo test` does not guarantee.
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map_or_else(|| PathBuf::from("."), PathBuf::from)
}

/// Where the FHIR definition bundles live, if they have been fetched.
///
/// `FHIR_ORACLE_SPEC_DIR` first, then `spec-cache/` beside the workspace root, to
/// match the CI fetch step. The previous fallback pointed into a sibling
/// checkout in one developer's home directory.
fn spec_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_ORACLE_SPEC_DIR")
            .ok()
            .map(PathBuf::from),
        Some(workspace_root().join("spec-cache")),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

fn to_recon(rm: &fhir_oracle_map::ResourceMap, out: &fhir_oracle_map::ShredOut) -> ReconIn {
    let mut tables: Vec<Vec<InRow>> = vec![Vec::new(); rm.tables.len()];
    for row in &out.rows {
        let mut cols = HashMap::new();
        for (name, val) in &row.cols {
            let text = match val {
                SqlVal::Bool(b) => b.to_string(),
                SqlVal::Int(n) => n.to_string(),
                SqlVal::Num(s) | SqlVal::Text(s) | SqlVal::Ts(s) | SqlVal::Date(s) => s.clone(),
                SqlVal::Jsonb(s) => s.clone(),
            };
            cols.insert(name.clone(), text);
        }
        tables[row.table as usize].push(InRow {
            ords: row.ords.clone(),
            cols,
        });
    }
    ReconIn {
        tables,
        ext: out.ext.clone(),
        deep: out.deep.clone(),
        contained: out.contained.clone(),
    }
}

fn sem_eq(a: &Value, b: &Value, path: &str, diffs: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ma), Value::Object(mb)) => {
            for (k, va) in ma {
                match mb.get(k) {
                    Some(vb) => sem_eq(va, vb, &format!("{path}.{k}"), diffs),
                    None => diffs.push(format!("{path}.{k}: missing in reconstruction")),
                }
            }
            for k in mb.keys() {
                if !ma.contains_key(k) {
                    diffs.push(format!("{path}.{k}: extra in reconstruction"));
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
        _ => {
            if a != b {
                diffs.push(format!("{path}: {a} vs {b}"));
            }
        }
    }
}

#[test]
fn roundtrip_full_corpus() {
    let (Some(corpus), Some(spec)) = (corpus_root(), spec_root()) else {
        eprintln!("skipping: corpus or spec dir missing");
        return;
    };
    let mut all_failures: Vec<String> = Vec::new();
    let mut total = 0usize;
    for (cdir, sdir, schema) in [("stu3", "r3", "r3"), ("r4", "r4", "r4"), ("r5", "r5", "r5")] {
        let ex = corpus.join(cdir);
        let defs = spec.join(sdir).join("fhir-definitions-json");
        if !ex.exists() || !defs.exists() {
            continue;
        }
        let map = fhir_oracle_gen::generate(&defs, schema).expect("generate");
        let mut entries: Vec<_> = std::fs::read_dir(&ex)
            .expect("dir")
            .map(|e| e.expect("entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        entries.sort();
        let mut count = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for path in entries {
            let bytes = std::fs::read(&path).expect("read");
            let Ok(v): Result<Value, _> = serde_json::from_slice(&bytes) else {
                continue;
            };
            let Some(rt) = v.get("resourceType").and_then(Value::as_str) else {
                continue;
            };
            let Some(rm) = map.resources.get(rt) else {
                // Conformance/abstract artifacts not in the concrete set.
                continue;
            };
            count += 1;
            let name = path
                .file_name()
                .expect("name")
                .to_string_lossy()
                .to_string();
            let out = match shred(rm, &v) {
                Ok(o) => o,
                Err(e) => {
                    failures.push(format!("{name}: shred: {e}"));
                    continue;
                }
            };
            let rin = to_recon(rm, &out);
            let back = match reconstruct(rm, &rin, out.id.as_deref()) {
                Ok(b) => b,
                Err(e) => {
                    failures.push(format!("{name}: reconstruct: {e}"));
                    continue;
                }
            };
            let mut diffs = Vec::new();
            sem_eq(&v, &back, "$", &mut diffs);
            if !diffs.is_empty() {
                diffs.truncate(3);
                failures.push(format!("{name}: {}", diffs.join(" | ")));
            }
        }
        eprintln!(
            "{sdir}: {count} corpus examples, {} failures",
            failures.len()
        );
        total += count;
        for f in failures.iter().take(40) {
            all_failures.push(format!("[{sdir}] {f}"));
        }
        if failures.len() > 40 {
            all_failures.push(format!("[{sdir}] … and {} more", failures.len() - 40));
        }
    }
    // A present-but-empty corpus directory is a fetch that did not happen,
    // not a round-trip failure. Say which, so the next person does not read
    // this as a data-fidelity bug.
    assert!(
        total > 1000,
        "{}",
        if total == 0 {
            "the corpus directory exists but holds no stu3/r4/r5 examples: \
             the fetch did not happen. Fetch it, or unset FHIR_ORACLE_CORPUS_DIR \
             to skip this test."
                .to_string()
        } else {
            // A partial corpus is the dangerous case: it runs, it passes its
            // per-file checks, and it looks like coverage. The threshold is
            // what stops 150 examples from standing in for 4,000.
            format!(
                "ran only {total} examples, expected the full corpus (>1000). \
                 A partial corpus passes every file it does have and proves \
                 much less than it appears to."
            )
        }
    );
    assert!(
        all_failures.is_empty(),
        "round-trip failures:\n{}",
        all_failures.join("\n")
    );
}
