//! End-to-end engine test: generate the map from the local FHIR definitions,
//! shred every example resource, reconstruct it, and require semantic
//! equality. Runs for every version whose definitions directory exists.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use fhir_mysql_map::reconstruct::{InRow, ReconIn, reconstruct};
use fhir_mysql_map::shred::{SqlVal, shred};
use serde_json::Value;

fn spec_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_MYSQL_SPEC_DIR").ok().map(PathBuf::from),
        // This monorepo: the model family carries the specification packages.
        Some(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../fhir/doc/fhir-specifications"
        ))),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

fn examples_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var("FHIR_MYSQL_EXAMPLES_DIR")
            .ok()
            .map(PathBuf::from),
        Some(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../fhir/tests/data"
        ))),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

/// Convert shredder output into reconstructor input, the way the store's
/// text-image round trip would.
fn to_recon(rm: &fhir_mysql_map::ResourceMap, out: &fhir_mysql_map::ShredOut) -> ReconIn {
    let mut tables: Vec<Vec<InRow>> = vec![Vec::new(); rm.tables.len()];
    for row in &out.rows {
        let mut cols = HashMap::new();
        for (name, val) in &row.cols {
            let text = match val {
                SqlVal::Bool(b) => b.to_string(),
                SqlVal::Int(n) => n.to_string(),
                SqlVal::Num(s) | SqlVal::Text(s) | SqlVal::Ts(s) | SqlVal::Date(s) => s.clone(),
                SqlVal::Jsonb(s) => s.clone(),
                // U3: adjuncts are derived and the reconstructor must never
                // read them. Skipping here is not a convenience — it is the
                // assertion: if a Bytes column ever reached reconstruction,
                // round-trip would be deciding on a value the resource never
                // contained.
                SqlVal::Bytes(_) => continue,
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

/// Semantic equality: object key order is irrelevant; numbers compare by
/// their serde_json (arbitrary-precision) representation.
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

fn run_version(defs: &Path, examples: &Path, schema: &str) -> (usize, Vec<String>) {
    let map = fhir_mysql_gen::generate(defs, schema).expect("generate map");
    let mut failures = Vec::new();
    let mut count = 0;
    let mut entries: Vec<_> = std::fs::read_dir(examples)
        .expect("examples dir")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    entries.sort();
    for path in entries {
        let bytes = std::fs::read(&path).expect("read example");
        let v: Value = serde_json::from_slice(&bytes).expect("parse example");
        let rt = v["resourceType"].as_str().expect("resourceType");
        let Some(rm) = map.resources.get(rt) else {
            failures.push(format!("{}: no map for {rt}", path.display()));
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
            diffs.truncate(5);
            failures.push(format!("{name}: {}", diffs.join(" | ")));
        }
    }
    (count, failures)
}

#[test]
fn roundtrip_examples_all_versions() {
    let Some(spec) = spec_root() else {
        eprintln!("skipping: no spec dir");
        return;
    };
    let Some(examples) = examples_root() else {
        eprintln!("skipping: no examples dir");
        return;
    };
    let mut all_failures = Vec::new();
    let mut total = 0;
    for (ver, schema) in [("r3", "r3"), ("r4", "r4"), ("r5", "r5")] {
        let defs = spec.join(ver).join("fhir-definitions-json");
        let ex = examples.join(format!("roundtrip_examples_{ver}"));
        if !defs.exists() || !ex.exists() {
            continue;
        }
        let (count, failures) = run_version(&defs, &ex, schema);
        eprintln!("{ver}: {count} examples, {} failures", failures.len());
        total += count;
        for f in &failures {
            all_failures.push(format!("[{ver}] {f}"));
        }
    }
    assert!(
        total > 100,
        "expected examples across versions, ran {total}"
    );
    assert!(
        all_failures.is_empty(),
        "{} round-trip failures:\n{}",
        all_failures.len(),
        all_failures.join("\n")
    );
}
