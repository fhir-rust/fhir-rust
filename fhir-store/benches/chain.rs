//! Micro-benchmarks for the audit chain: the work every `put` and every
//! `verify_audit` row pays. Pure CPU, no database, so the numbers are this
//! crate's alone and say nothing about any engine (see BENCHMARKS.md, W16.10).
//!
//! Run: `cargo bench -p fhir-store` (from `fhir-store/`).

use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use fhir_store::chain::{ChainKey, KeyRing, link, mac, preimage};

fn resource(bytes: usize) -> String {
    format!(
        "{{\"resourceType\":\"Patient\",\"note\":\"{}\"}}",
        "x".repeat(bytes)
    )
}

fn key() -> ChainKey {
    ChainKey::from_hex("bench", &"ab".repeat(32)).expect("bench key")
}

/// Cost of linking one row, by resource size.
fn link_one_row(c: &mut Criterion) {
    let mut g = c.benchmark_group("link_one_row");
    for size in [256usize, 4 * 1024, 64 * 1024] {
        let body = resource(size);
        let pre = preimage("p1", 1, "2026-01-01T00:00:00Z", "U", Some(&body), "alice");
        g.throughput(Throughput::Bytes(pre.len() as u64));
        g.bench_with_input(BenchmarkId::new("sha256+sha3", size), &pre, |b, pre| {
            b.iter(|| link(None, None, black_box(pre)));
        });
        let k = key();
        g.bench_with_input(BenchmarkId::new("hmac", size), &pre, |b, pre| {
            b.iter(|| mac(&k, None, black_box(pre)));
        });
    }
    g.finish();
}

/// The keyed check a verifier runs per row.
fn check_one_row(c: &mut Criterion) {
    let k = key();
    let ring = KeyRing::new(vec![k.clone()]);
    let pre = preimage(
        "p1",
        1,
        "2026-01-01T00:00:00Z",
        "U",
        Some(&resource(4096)),
        "alice",
    );
    let tag = mac(&k, None, &pre);
    c.bench_function("keyring_check_ok", |b| {
        b.iter(|| ring.check(Some(black_box(&tag)), None, black_box(&pre)));
    });
}

/// Verifying a whole history, as `verify_audit` does.
fn verify_history(c: &mut Criterion) {
    let k = key();
    let mut g = c.benchmark_group("verify_history");
    for rows in [10usize, 1_000] {
        g.throughput(Throughput::Elements(rows as u64));
        g.bench_function(BenchmarkId::from_parameter(rows), |b| {
            b.iter_batched(
                || {
                    (0..rows)
                        .map(|i| preimage("p1", i as i64 + 1, "t", "U", Some(&resource(1024)), "a"))
                        .collect::<Vec<_>>()
                },
                |pres| {
                    let (mut p256, mut p3): (Option<Vec<u8>>, Option<Vec<u8>>) = (None, None);
                    for pre in &pres {
                        let (a, s) = link(p256.as_deref(), p3.as_deref(), pre);
                        black_box(mac(&k, p256.as_deref(), pre));
                        (p256, p3) = (Some(a), Some(s));
                    }
                    p256
                },
                BatchSize::SmallInput,
            );
        });
    }
    g.finish();
}

criterion_group!(benches, link_one_row, check_one_row, verify_history);
criterion_main!(benches);
