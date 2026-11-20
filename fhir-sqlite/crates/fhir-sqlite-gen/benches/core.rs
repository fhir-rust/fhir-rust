//! Criterion benchmarks for the pure-Rust core: `fold`, `shred`, `reconstruct`.
//!
//! No database is involved, so these measure CPU cost of the engine-agnostic
//! half only. They say nothing about store throughput — that is `W16.10`'s
//! distinction — and a figure from this file must name the machine and date
//! that produced it. Skips (prints why, exits cleanly) when the FHIR
//! specification packages or the example corpus are absent.

use std::collections::HashMap;
use std::hint::black_box;
use std::path::PathBuf;

use criterion::{Criterion, criterion_group, criterion_main};
use fhir_sqlite_map::reconstruct::{InRow, ReconIn, reconstruct};
use fhir_sqlite_map::shred::{SqlVal, shred};
use fhir_sqlite_map::{ResourceMap, ShredOut, fold::fold};
use serde_json::Value;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fhir")
        .join(rel)
}

fn to_recon(rm: &ResourceMap, out: &ShredOut) -> ReconIn {
    let mut tables: Vec<Vec<InRow>> = vec![Vec::new(); rm.tables.len()];
    for row in &out.rows {
        let mut cols = HashMap::new();
        for (name, val) in &row.cols {
            let text = match val {
                SqlVal::Bool(b) => b.to_string(),
                SqlVal::Int(n) => n.to_string(),
                SqlVal::Num(s) | SqlVal::Text(s) | SqlVal::Ts(s) | SqlVal::Date(s) => s.clone(),
                SqlVal::Jsonb(s) => s.clone(),
                SqlVal::Bytes(_) => continue, // derived; never read back (U3)
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

fn bench_fold(c: &mut Criterion) {
    let names = [
        "Ærø",
        "Müller",
        "Muñoz",
        "Ślusarczyk",
        "Ångström",
        "Henderson",
    ];
    c.bench_function("fold/name", |b| {
        b.iter(|| {
            for n in &names {
                black_box(fold(black_box(n)));
            }
        })
    });
}

fn bench_resources(c: &mut Criterion) {
    let defs = repo_path("doc/fhir-specifications/r5/fhir-definitions-json");
    let examples = repo_path("tests/data/roundtrip_examples_r5");
    if !defs.exists() || !examples.exists() {
        eprintln!("skipping shred/reconstruct: no spec or examples under fhir/");
        return;
    }
    let map = fhir_sqlite_gen::generate(&defs, "r5").expect("generate map");
    for file in [
        "patient-example-ihe-pcd.json",
        "observation-example-eye-color.json",
    ] {
        let path = examples.join(file);
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("skipping {file}: not in corpus");
            continue;
        };
        let v: Value = serde_json::from_slice(&bytes).expect("parse");
        let rt = v["resourceType"].as_str().expect("resourceType").to_owned();
        let rm = map.resources.get(&rt).expect("resource map");
        let out = shred(rm, &v).expect("shred");
        let rin = to_recon(rm, &out);
        let id = out.id.clone();
        let stem = file.trim_end_matches(".json");
        c.bench_function(&format!("shred/{stem}"), |b| {
            b.iter(|| black_box(shred(rm, black_box(&v)).expect("shred")))
        });
        c.bench_function(&format!("reconstruct/{stem}"), |b| {
            b.iter(|| black_box(reconstruct(rm, black_box(&rin), id.as_deref()).expect("rebuild")))
        });
    }
}

criterion_group!(benches, bench_fold, bench_resources);
criterion_main!(benches);
