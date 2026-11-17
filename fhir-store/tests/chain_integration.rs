//! Integration tests: the public API used the way a database port uses it —
//! write a chain of history rows, then verify it, including after tampering,
//! truncation, reordering and key rotation. Pure; no database, no environment.

use fhir_store::chain::{ChainKey, GENESIS, KeyRing, MacCheck, digests_equal, link, mac, preimage};
use fhir_store::{Audit, ChainBreak};

/// One stored history row, as a port would persist it.
struct Row {
    pre: Vec<u8>,
    sha256: Vec<u8>,
    sha3: Vec<u8>,
    tag: Option<String>,
}

fn write_chain(signing: Option<&ChainKey>, bodies: &[&str]) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for (i, body) in bodies.iter().enumerate() {
        let pre = preimage(
            "p1",
            i as i64 + 1,
            "2026-01-01T00:00:00Z",
            "U",
            Some(body),
            "alice",
        );
        let prev256 = rows.last().map(|r| r.sha256.as_slice());
        let prev3 = rows.last().map(|r| r.sha3.as_slice());
        let (sha256, sha3) = link(prev256, prev3, &pre);
        let tag = signing.map(|k| mac(k, prev256, &pre));
        rows.push(Row {
            pre,
            sha256,
            sha3,
            tag,
        });
    }
    rows
}

/// Re-derive every link and tag; return the 1-based version of the first break.
fn first_break(ring: &KeyRing, rows: &[Row]) -> Option<usize> {
    let mut p256: Option<Vec<u8>> = None;
    let mut p3: Option<Vec<u8>> = None;
    for (i, r) in rows.iter().enumerate() {
        let (a, b) = link(p256.as_deref(), p3.as_deref(), &r.pre);
        if !digests_equal(&a, &r.sha256) || !digests_equal(&b, &r.sha3) {
            return Some(i + 1);
        }
        if ring.check(r.tag.as_deref(), p256.as_deref(), &r.pre) == MacCheck::Mismatch {
            return Some(i + 1);
        }
        p256 = Some(r.sha256.clone());
        p3 = Some(r.sha3.clone());
    }
    None
}

fn key(id: &str, byte: &str) -> ChainKey {
    ChainKey::from_hex(id, &byte.repeat(32)).unwrap()
}

#[test]
fn an_untampered_chain_verifies_keyed_and_unkeyed() {
    let k = key("k1", "ab");
    let ring = KeyRing::new(vec![k.clone()]);
    let bodies = ["{\"a\":1}", "{\"a\":2}", "{\"a\":3}"];
    assert_eq!(first_break(&ring, &write_chain(Some(&k), &bodies)), None);
    assert_eq!(
        first_break(&KeyRing::default(), &write_chain(None, &bodies)),
        None
    );
}

#[test]
fn the_first_row_chains_from_genesis() {
    let rows = write_chain(None, &["{}"]);
    let (a, b) = link(Some(&GENESIS), Some(&GENESIS), &rows[0].pre);
    assert_eq!((a, b), (rows[0].sha256.clone(), rows[0].sha3.clone()));
}

#[test]
fn editing_a_middle_row_is_found_at_that_row() {
    let k = key("k1", "ab");
    let ring = KeyRing::new(vec![k.clone()]);
    let mut rows = write_chain(Some(&k), &["{\"a\":1}", "{\"a\":2}", "{\"a\":3}"]);
    rows[1].pre = preimage(
        "p1",
        2,
        "2026-01-01T00:00:00Z",
        "U",
        Some("{\"a\":99}"),
        "alice",
    );
    assert_eq!(first_break(&ring, &rows), Some(2));
}

#[test]
fn reordering_rows_is_detected() {
    let k = key("k1", "ab");
    let ring = KeyRing::new(vec![k.clone()]);
    let mut rows = write_chain(Some(&k), &["{\"a\":1}", "{\"a\":2}", "{\"a\":3}"]);
    rows.swap(1, 2);
    assert!(first_break(&ring, &rows).is_some());
}

#[test]
fn dropping_the_first_row_is_detected() {
    let k = key("k1", "ab");
    let ring = KeyRing::new(vec![k.clone()]);
    let mut rows = write_chain(Some(&k), &["{\"a\":1}", "{\"a\":2}"]);
    rows.remove(0);
    assert_eq!(first_break(&ring, &rows), Some(1));
}

/// A forger who recomputes the unkeyed digests still cannot produce the tag.
#[test]
fn recomputed_digests_without_the_key_fail_the_mac() {
    let k = key("k1", "ab");
    let ring = KeyRing::new(vec![k.clone()]);
    let mut rows = write_chain(Some(&k), &["{\"a\":1}"]);
    let forged = preimage(
        "p1",
        1,
        "2026-01-01T00:00:00Z",
        "U",
        Some("{\"a\":7}"),
        "alice",
    );
    let (a, b) = link(None, None, &forged);
    rows[0] = Row {
        pre: forged,
        sha256: a,
        sha3: b,
        tag: rows[0].tag.clone(),
    };
    assert_eq!(first_break(&ring, &rows), Some(1));
}

#[test]
fn rotation_keeps_old_rows_verifiable_and_new_signer_signs() {
    let old = key("old", "11");
    let new = key("new", "22");
    let rows_old = write_chain(Some(&old), &["{\"a\":1}"]);
    let rotated = KeyRing::new(vec![new.clone(), old.clone()]);
    assert_eq!(first_break(&rotated, &rows_old), None);
    assert_eq!(rotated.signing().map(ChainKey::id), Some("new"));

    // Dropping the retired key makes the rows unverifiable, not tampered.
    let only_new = KeyRing::new(vec![new]);
    let r = &rows_old[0];
    assert_eq!(
        only_new.check(r.tag.as_deref(), None, &r.pre),
        MacCheck::Unverifiable {
            key_id: "old".into()
        }
    );
    assert_eq!(first_break(&only_new, &rows_old), None);
}

#[test]
fn key_files_round_trip_and_refuse_overwrite() {
    let dir = std::env::temp_dir().join(format!("fhir-store-it-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("k.key");
    let _ = std::fs::remove_file(&path);

    let generated = ChainKey::generate_to_file("g1", &path).unwrap();
    assert!(
        ChainKey::generate_to_file("g1", &path).is_err(),
        "must not overwrite"
    );

    let loaded = ChainKey::from_file("g1", &path).unwrap();
    let pre = preimage("p", 1, "t", "C", Some("{}"), "a");
    assert_eq!(mac(&generated, None, &pre), mac(&loaded, None, &pre));

    let ring = KeyRing::from_files(Some(("g1", &path)), &[]).unwrap();
    assert!(!ring.is_empty());
    assert!(KeyRing::from_files(None, &[("gone".into(), dir.join("missing"))]).is_err());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(ChainKey::from_file("g1", &path).is_err());
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn audit_constructors_and_builders_compose() {
    let u = Audit::unattributed();
    assert_eq!(u.actor, "unauthenticated");
    assert!(u.actor_source.is_none());
    assert_eq!(Audit::default().actor, u.actor);

    let a = Audit::principal("dr-who", "header:X-Fhir-Principal")
        .with_client(Some("10.0.0.1".into()))
        .with_request_id(Some("req-1".into()))
        .with_reason(Some("treatment".into()));
    assert_eq!(a.actor, "dr-who");
    assert_eq!(a.actor_source.as_deref(), Some("header:X-Fhir-Principal"));
    assert_eq!(a.client.as_deref(), Some("10.0.0.1"));
    assert_eq!(a.request_id.as_deref(), Some("req-1"));
    assert_eq!(a.reason.as_deref(), Some("treatment"));

    let c = Audit::cli();
    assert!(c.actor.starts_with("cli:"));
    assert_eq!(c.actor_source.as_deref(), Some("cli"));
}

#[test]
fn chain_break_carries_its_coordinates() {
    let b = ChainBreak::new("Patient", "p1", 3, "sha256", "mismatch");
    assert_eq!(
        (b.rtype.as_str(), b.id.as_str(), b.version_id),
        ("Patient", "p1", 3)
    );
}
