//! The tamper-evidence hash chain, computed in this process (spec M3.16,
//! M3.16a).
//!
//! # Why not in the database
//!
//! Earlier versions computed these digests in SQL, inside the same statement
//! that inserted the row. That bought two real properties — the hashed
//! timestamp was the database's own `now()`, and the read of the previous
//! digest could not race the insert — and cost something more important.
//!
//! The digests are **unkeyed** and their pre-image format is public. Any
//! party who can write SQL can therefore also produce a correct digest for
//! whatever they wrote. Keeping the computation inside the database puts the
//! means of forgery in the same place as the data being forged, and
//! forecloses the only fix: a **keyed** digest (a MAC) whose key the database
//! never holds. A key stored where the attacker already has write access
//! protects nothing.
//!
//! So the honest statement of what this buys today, unkeyed:
//!
//! - It detects **careless or unaware modification** — a migration, a stray
//!   `UPDATE`, a restored-from-the-wrong-backup row.
//! - It supports an **external witness**: record the chain head somewhere the
//!   database cannot reach, and truncation or wholesale rewriting becomes
//!   detectable even against an attacker who can recompute digests.
//!
//! It does **not** by itself stop an attacker with SQL write access who knows
//! the format. That is what the **keyed** mode below is for.
//!
//! # Keyed mode (the actual fix)
//!
//! With `<prefix>_CHAIN_KEY` set (see [`ChainKey::from_env`]), each link is an HMAC rather than a bare hash:
//! `HMAC-SHA-256` and `HMAC-SHA3-256`, both FIPS-approved constructions
//! (198-1 over 180-4 and 202). The key lives in this process — from the
//! environment, or a file the database role cannot read — and is **never**
//! written to the database, never logged, and never sent in a query.
//!
//! That is the whole point. An attacker with SQL write access can rewrite any
//! row, but cannot produce a digest that verifies, because forging one
//! requires a secret that is not in the place they broke into. Storing the
//! key in the database would return the design to where it started.
//!
//! Each row records the **key id** that signed it, so keys can rotate without
//! invalidating history, and so a verifier can say *"signed with k2, which I
//! do not have"* — which is not the same claim as *"this row was tampered
//! with"*, and must never be reported as if it were.
//!
//! # Keeping the two lost properties
//!
//! Both survive without SQL-side hashing:
//!
//! - The timestamp is still the database's. It is read with `now()` in the
//!   same transaction and written back verbatim, so the value hashed is the
//!   value stored.
//! - The read of the previous digest still cannot race, because the write
//!   path already holds a `SELECT … FOR UPDATE` row lock on the base row for
//!   this resource id before it appends history.

use hmac::{Hmac, Mac};
use sha2::{Digest as _, Sha256};
use sha3::Sha3_256;

/// The signing key for keyed mode, and the id recorded alongside each row.
///
/// The secret is zeroed when the key is dropped. Freed memory is not
/// scrubbed by default, so a key would otherwise linger in the heap and be
/// recoverable from a core dump or a crash report — which is a longer life
/// than a secret should have.
#[derive(Clone, zeroize::ZeroizeOnDrop)]
pub struct ChainKey {
    #[zeroize(skip)]
    id: String,
    secret: Vec<u8>,
}

impl std::fmt::Debug for ChainKey {
    /// Never renders the secret. A key that reaches a log is a key that has
    /// left the process, which is the one thing this design promises it does
    /// not do.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainKey")
            .field("id", &self.id)
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl ChainKey {
    /// Build from hex, rejecting keys too short to be worth having.
    ///
    /// # Errors
    /// If the hex is malformed or shorter than 32 bytes.
    pub fn from_hex(id: &str, hex: &str) -> Result<Self, String> {
        let hex = hex.trim();
        if !hex.len().is_multiple_of(2) {
            return Err("chain key must be hex (even number of digits)".into());
        }
        let secret: Result<Vec<u8>, _> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
            .collect();
        let secret = secret.map_err(|_| "chain key is not valid hex".to_string())?;
        if secret.len() < 32 {
            return Err(format!(
                "chain key is {} bytes; at least 32 are required",
                secret.len()
            ));
        }
        if id.is_empty() {
            return Err("chain key id must not be empty".into());
        }
        Ok(Self {
            id: id.to_string(),
            secret,
        })
    }

    /// Read a key from a file, rejecting one any other account can read.
    ///
    /// Preferred over the environment. A variable is visible in
    /// `/proc/<pid>/environ`, survives into crash dumps, is reported by
    /// orchestrators and `docker inspect`, and is inherited by every child
    /// process. A file is none of those, is what Kubernetes secrets and
    /// systemd credentials already produce, and can have its permissions
    /// checked — which this does, because a key readable by the whole
    /// machine is not a key.
    ///
    /// # Errors
    /// If the file is missing, group- or world-readable, or not a valid key.
    pub fn from_file(id: &str, path: &std::path::Path) -> Result<Self, String> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let meta = std::fs::metadata(path)
                .map_err(|e| format!("chain key {}: {e}", path.display()))?;
            let mode = meta.permissions().mode() & 0o077;
            if mode != 0 {
                return Err(format!(
                    "chain key {} is readable by group or other (mode {:o}); \
                     chmod 600 it",
                    path.display(),
                    meta.permissions().mode() & 0o777
                ));
            }
        }
        let hex = std::fs::read_to_string(path)
            .map_err(|e| format!("chain key {}: {e}", path.display()))?;
        Self::from_hex(id, &hex)
    }

    /// Generate a fresh 32-byte key and write it to `path` as hex, readable
    /// only by the owner.
    ///
    /// Creating the file with `0600` from the start matters more than it
    /// looks. The obvious shell equivalent, `openssl rand -hex 32 > key`,
    /// applies the process umask — commonly `022`, giving `0644` — which
    /// [`Self::from_file`] then refuses. Worse, the secret exists
    /// world-readable for the moment between creation and `chmod`.
    ///
    /// Refuses to overwrite: silently replacing a signing key would orphan
    /// every row it had signed.
    ///
    /// # Errors
    /// If the file exists, or cannot be created.
    pub fn generate_to_file(id: &str, path: &std::path::Path) -> Result<Self, String> {
        use std::io::Write as _;
        let mut secret = [0u8; 32];
        getrandom::fill(&mut secret).map_err(|e| format!("no entropy available: {e}"))?;
        let hex: String = secret.iter().map(|b| format!("{b:02x}")).collect();

        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            opts.mode(0o600);
        }
        let mut f = opts
            .open(path)
            .map_err(|e| format!("chain key {}: {e}", path.display()))?;
        writeln!(f, "{hex}").map_err(|e| format!("chain key {}: {e}", path.display()))?;
        Self::from_hex(id, &hex)
    }

    /// Read from `<prefix>_CHAIN_KEY` (hex) and `<prefix>_CHAIN_KEY_ID`.
    ///
    /// `prefix` is the caller's own port name, e.g. `"FHIR_POSTGRESQL"` —
    /// this function is shared, engine-independent code (`X15.1`), so it
    /// cannot know which port it is linked into and must be told. It used to
    /// hardcode `FHIR_SQLITE_CHAIN_KEY` regardless of caller, which meant
    /// every other port's own documented variable name
    /// (`FHIR_POSTGRESQL_CHAIN_KEY` and so on) silently did nothing — found
    /// and fixed as `audit.md` **F-70**.
    ///
    /// Absent means unkeyed, which is a supported but weaker mode: callers
    /// are expected to say so at startup rather than let it pass silently.
    ///
    /// # Errors
    /// If the key is present but unusable.
    pub fn from_env(prefix: &str) -> Result<Option<Self>, String> {
        let Ok(hex) = std::env::var(format!("{prefix}_CHAIN_KEY")) else {
            return Ok(None);
        };
        let id = std::env::var(format!("{prefix}_CHAIN_KEY_ID")).unwrap_or_else(|_| "k1".to_string());
        Self::from_hex(&id, &hex).map(Some)
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// The zero digest a chain starts from, for both algorithms (32 bytes).
pub const GENESIS: [u8; 32] = [0u8; 32];

/// The algorithms a chain is kept under, in report order.
///
/// Two design families, deliberately: SHA-256 is Merkle–Damgård and SHA3-256
/// is a sponge, so the line of cryptanalysis that took MD5 and SHA-1 — both
/// Merkle–Damgård — cannot take both. Both are FIPS-approved (180-4, 202).
pub const ALGORITHMS: [&str; 2] = ["sha256", "sha3-256"];

/// The bytes a history row commits to.
///
/// `resource` MUST be the **stored** normalized form (`jsonb::text`), not the
/// submitted text: `jsonb` reorders keys and rewrites number spellings, so
/// hashing the input would make every chain fail the moment it was checked
/// against what was actually saved.
///
/// Defined once, and used by both the writer and the verifier, so the two
/// cannot drift into disagreeing about what was signed.
#[must_use]
pub fn preimage(
    id: &str,
    version_id: i64,
    last_updated: &str,
    op: &str,
    resource: Option<&str>,
    actor: &str,
) -> Vec<u8> {
    format!(
        "{id}|{version_id}|{last_updated}|{op}|{}|{actor}",
        resource.unwrap_or("")
    )
    .into_bytes()
}

/// Link one row into both chains.
///
/// Unkeyed: `H(prev || preimage)`. Keyed: `HMAC-H(key, prev || preimage)`.
/// The chained predecessor is inside the MAC, so reordering or truncating
/// rows is as detectable as editing one.
#[must_use]
pub fn link(
    prev_sha256: Option<&[u8]>,
    prev_sha3: Option<&[u8]>,
    preimage: &[u8],
) -> (Vec<u8>, Vec<u8>) {
    let mut a = Sha256::new();
    a.update(prev_sha256.unwrap_or(&GENESIS));
    a.update(preimage);
    let mut b = Sha3_256::new();
    b.update(prev_sha3.unwrap_or(&GENESIS));
    b.update(preimage);
    (a.finalize().to_vec(), b.finalize().to_vec())
}

/// The keyed tag over the same pre-image, rendered `<key-id>:<hex>`.
///
/// HMAC-SHA-256 (FIPS 198-1 over 180-4), so the FIPS story stays clean.
///
/// The key id travels **with** the tag. Without it, rotating a key would
/// invalidate every historical row at once — which is indistinguishable from
/// mass tampering, and is the same trap as silently changing a hash format.
/// Retired keys stay loadable, so rotation is additive rather than a flag day.
#[must_use]
pub fn mac(key: &ChainKey, prev_sha256: Option<&[u8]>, preimage: &[u8]) -> String {
    let mut m =
        <Hmac<Sha256> as Mac>::new_from_slice(&key.secret).expect("HMAC accepts any key length");
    m.update(prev_sha256.unwrap_or(&GENESIS));
    m.update(preimage);
    let tag = m.finalize().into_bytes();
    let hex: String = tag.iter().map(|b| format!("{b:02x}")).collect();
    format!("{}:{hex}", key.id)
}

/// What a stored tag turned out to be. Only [`MacCheck::Mismatch`] is a
/// finding; the rest are reasons a row could not be checked, and reporting
/// any of them as tampering would burn an incident response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacCheck {
    /// Verified against the key it names.
    Ok,
    /// **A finding.** The tag does not match; the row or its chain changed.
    Mismatch,
    /// No tag stored — written unkeyed, or before keying was enabled.
    Absent,
    /// Names a key this process does not hold. Not a verdict on the row.
    Unverifiable { key_id: String },
    /// Stored, but not in `<key-id>:<hex>` form.
    Malformed,
}

/// Every key this process can verify with: the signing key, plus any retired
/// keys kept so that history signed under them still checks out.
#[derive(Clone, Debug, Default)]
pub struct KeyRing {
    keys: Vec<ChainKey>,
}

impl KeyRing {
    /// Build from an explicit ordered list: the first signs, the rest verify.
    #[must_use]
    pub fn new(keys: Vec<ChainKey>) -> Self {
        Self { keys }
    }

    /// Load from files: one signing key, plus retired keys that verify.
    ///
    /// `retired` entries are `id=path`. Any key whose file cannot be read is
    /// an error rather than a silent omission — a retired key quietly
    /// dropped turns its rows *unverifiable*, and an operator who did not
    /// intend that should hear about it at startup, not from an audit.
    ///
    /// # Errors
    /// If any key is missing, badly permissioned, or malformed.
    pub fn from_files(
        signing: Option<(&str, &std::path::Path)>,
        retired: &[(String, std::path::PathBuf)],
    ) -> Result<Self, String> {
        let mut keys = Vec::new();
        if let Some((id, path)) = signing {
            keys.push(ChainKey::from_file(id, path)?);
        }
        for (id, path) in retired {
            keys.push(ChainKey::from_file(id, path)?);
        }
        Ok(Self { keys })
    }

    /// Load from the environment.
    ///
    /// `prefix` is the caller's own port name, e.g. `"FHIR_POSTGRESQL"` —
    /// see [`ChainKey::from_env`] for why this must be passed rather than
    /// assumed. `<prefix>_CHAIN_KEY` (hex) with optional
    /// `<prefix>_CHAIN_KEY_ID` is the signing key.
    /// `<prefix>_CHAIN_KEYS_RETIRED` holds `id=hex` pairs, comma-separated,
    /// which verify but never sign.
    ///
    /// Weaker than [`Self::from_files`]: see [`ChainKey::from_file`].
    ///
    /// # Errors
    /// If any key is present but unusable.
    pub fn from_env(prefix: &str) -> Result<Self, String> {
        let mut keys = Vec::new();
        if let Some(k) = ChainKey::from_env(prefix)? {
            keys.push(k);
        }
        if let Ok(retired) = std::env::var(format!("{prefix}_CHAIN_KEYS_RETIRED")) {
            for entry in retired.split(',').map(str::trim).filter(|e| !e.is_empty()) {
                let (id, hex) = entry
                    .split_once('=')
                    .ok_or_else(|| format!("retired key {entry:?} is not id=hex"))?;
                keys.push(ChainKey::from_hex(id, hex)?);
            }
        }
        Ok(Self { keys })
    }

    /// The key that signs new rows: the first loaded, if any.
    #[must_use]
    pub fn signing(&self) -> Option<&ChainKey> {
        self.keys.first()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Check a stored tag against a recomputed one.
    #[must_use]
    pub fn check(&self, stored: Option<&str>, prev_sha256: Option<&[u8]>, pre: &[u8]) -> MacCheck {
        let Some(stored) = stored else {
            return MacCheck::Absent;
        };
        let Some((id, _)) = stored.split_once(':') else {
            return MacCheck::Malformed;
        };
        let Some(key) = self.keys.iter().find(|k| k.id == id) else {
            return MacCheck::Unverifiable {
                key_id: id.to_string(),
            };
        };
        let expect = mac(key, prev_sha256, pre);
        if digests_equal(stored.as_bytes(), expect.as_bytes()) {
            MacCheck::Ok
        } else {
            MacCheck::Mismatch
        }
    }
}

/// Compare digests in constant time.
///
/// Verification compares attacker-influenced bytes against a computed MAC; a
/// short-circuiting `==` leaks how much of a forgery was right.
#[must_use]
pub fn digests_equal(a: &[u8], b: &[u8]) -> bool {
    use subtle::ConstantTimeEq as _;
    a.ct_eq(b).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_algorithms_disagree_on_the_same_input() {
        let (a, b) = link(None, None, b"x");
        assert_ne!(a, b, "two families must not produce one digest");
        assert_eq!(a.len(), 32);
        assert_eq!(b.len(), 32);
    }

    #[test]
    fn a_changed_preimage_changes_both() {
        let (a1, b1) = link(None, None, &preimage("p", 1, "t", "C", Some("{}"), "who"));
        let (a2, b2) = link(None, None, &preimage("p", 1, "t", "C", Some("{}"), "other"));
        assert_ne!(a1, a2, "sha256 must notice the actor");
        assert_ne!(b1, b2, "sha3 must notice the actor");
    }

    #[test]
    fn a_changed_predecessor_changes_both() {
        let pre = preimage("p", 2, "t", "U", Some("{}"), "who");
        let (a1, b1) = link(Some(&[1u8; 32]), Some(&[1u8; 32]), &pre);
        let (a2, b2) = link(Some(&[2u8; 32]), Some(&[2u8; 32]), &pre);
        assert_ne!(a1, a2);
        assert_ne!(b1, b2);
    }

    /// The genesis link is explicit, not "no bytes": a first version must
    /// commit to *something* fixed, or an attacker could truncate a chain to
    /// one row and have it verify.
    #[test]
    fn genesis_is_thirty_two_zero_bytes() {
        assert_eq!(GENESIS.len(), 32);
        assert_eq!(
            link(None, None, b"x"),
            link(Some(&GENESIS), Some(&GENESIS), b"x")
        );
    }

    fn key() -> ChainKey {
        ChainKey::from_hex("kt", &"ab".repeat(32)).expect("test key")
    }

    /// The property the keyed layer exists for, stated directly: the
    /// unkeyed digests are reproducible by anyone holding the data, and the
    /// MAC is not.
    #[test]
    fn digests_are_reproducible_but_the_mac_is_not() {
        let pre = preimage("p", 1, "t", "C", Some("{}"), "who");

        // Anyone with the row can recompute the digests. That is by design:
        // it is what lets an outside auditor check the chain unaided.
        assert_eq!(link(None, None, &pre), link(None, None, &pre));

        // The tag needs the secret. Guessing the format does not help, and
        // neither does guessing a plausible key.
        let real = mac(&key(), None, &pre);
        for guess in ["changeme", "00", "secret"] {
            if let Ok(k) = ChainKey::from_hex("kt", &"00".repeat(32)) {
                assert_ne!(mac(&k, None, &pre), real, "guessed key {guess} matched");
            }
        }
        assert!(real.starts_with("kt:"), "the tag names its key: {real}");
    }

    /// Three non-findings, each distinct from a mismatch. Reporting a
    /// key-distribution problem as tampering would burn an incident response.
    #[test]
    fn absent_unverifiable_and_malformed_are_not_findings() {
        let pre = preimage("p", 1, "t", "C", Some("{}"), "who");
        let ring = KeyRing { keys: vec![key()] };
        assert_eq!(ring.check(None, None, &pre), MacCheck::Absent);
        assert_eq!(
            ring.check(Some("k9:abcd"), None, &pre),
            MacCheck::Unverifiable {
                key_id: "k9".into()
            }
        );
        assert_eq!(
            ring.check(Some("no-colon"), None, &pre),
            MacCheck::Malformed
        );
        assert_eq!(
            ring.check(Some(&mac(&key(), None, &pre)), None, &pre),
            MacCheck::Ok
        );
        assert_eq!(
            ring.check(Some("kt:00"), None, &pre),
            MacCheck::Mismatch,
            "a wrong tag under a key we hold is the one real finding"
        );
    }

    /// Rotation is additive: a retired key still verifies its own rows, so
    /// rotating does not invalidate history all at once.
    #[test]
    fn a_retired_key_still_verifies_its_rows() {
        let old = ChainKey::from_hex("k1", &"11".repeat(32)).expect("key");
        let new = ChainKey::from_hex("k2", &"22".repeat(32)).expect("key");
        let pre = preimage("p", 1, "t", "C", Some("{}"), "who");
        let signed_with_old = mac(&old, None, &pre);
        let ring = KeyRing {
            keys: vec![new, old],
        };
        assert_eq!(ring.signing().map(ChainKey::id), Some("k2"), "newest signs");
        assert_eq!(ring.check(Some(&signed_with_old), None, &pre), MacCheck::Ok);
    }

    #[test]
    fn a_different_key_yields_a_different_digest() {
        let pre = preimage("p", 1, "t", "C", Some("{}"), "who");
        let other = ChainKey::from_hex("kt", &"cd".repeat(32)).expect("key");
        assert_ne!(mac(&key(), None, &pre), mac(&other, None, &pre));
    }

    #[test]
    fn short_and_malformed_keys_are_refused() {
        assert!(
            ChainKey::from_hex("k", &"ab".repeat(16)).is_err(),
            "16 bytes is too short"
        );
        assert!(ChainKey::from_hex("k", "not-hex").is_err());
        assert!(
            ChainKey::from_hex("", &"ab".repeat(32)).is_err(),
            "id is required"
        );
        assert!(ChainKey::from_hex("k", &"ab".repeat(32)).is_ok());
    }

    /// A secret that reaches a log has left the process.
    #[test]
    fn debug_never_renders_the_secret() {
        let rendered = format!("{:?}", key());
        assert!(!rendered.contains("abab"), "secret leaked: {rendered}");
        assert!(rendered.contains("redacted"));
        assert!(rendered.contains("kt"), "the id is safe and useful to show");
    }

    #[test]
    fn digest_comparison_is_available_and_correct() {
        assert!(digests_equal(&[1, 2, 3], &[1, 2, 3]));
        assert!(!digests_equal(&[1, 2, 3], &[1, 2, 4]));
        assert!(!digests_equal(&[1, 2, 3], &[1, 2]));
    }

    /// A key file the rest of the machine can read is not a key. Refusing
    /// is better than warning: a warning at startup is read once, and the
    /// file stays readable for the life of the deployment.
    #[test]
    #[cfg(unix)]
    fn a_group_readable_key_file_is_refused() {
        use std::io::Write as _;
        use std::os::unix::fs::PermissionsExt as _;

        let dir = std::env::temp_dir().join(format!("fhir-sqlite-keytest-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let path = dir.join("chain.key");
        let mut f = std::fs::File::create(&path).expect("create");
        f.write_all(&b"ab".repeat(32)).expect("write");
        drop(f);

        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).expect("chmod");
        let err = ChainKey::from_file("k", &path).expect_err("0640 must be refused");
        assert!(err.contains("group or other"), "unhelpful message: {err}");

        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("chmod");
        let key = ChainKey::from_file("k", &path).expect("0600 is fine");
        assert_eq!(key.id(), "k");

        // Trailing newline is what every editor and `openssl rand ... >` adds.
        std::fs::write(&path, format!("{}\n", "ab".repeat(32))).expect("write");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("chmod");
        assert!(
            ChainKey::from_file("k", &path).is_ok(),
            "a trailing newline must not break a key file"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    /// A retired key that cannot be read is an error, not a silent omission.
    /// Dropping one turns its rows unverifiable, and an operator who did not
    /// intend that should hear about it at startup rather than from an audit.
    #[test]
    fn an_unreadable_retired_key_is_an_error() {
        let missing = std::path::PathBuf::from("/nonexistent/fhir-sqlite/retired.key");
        let err = KeyRing::from_files(None, &[("k1".to_string(), missing)])
            .expect_err("must not silently skip");
        assert!(err.contains("k1") || err.contains("retired.key"), "{err}");
    }

    /// Pins the exact bytes committed to. If this changes, every stored chain
    /// stops verifying — which is precisely why it should be hard to change
    /// by accident.
    #[test]
    fn preimage_format_is_stable() {
        assert_eq!(
            preimage(
                "p1",
                3,
                "2026-01-01 00:00:00+00",
                "U",
                Some("{\"a\": 1}"),
                "clinician"
            ),
            b"p1|3|2026-01-01 00:00:00+00|U|{\"a\": 1}|clinician".to_vec()
        );
        assert_eq!(
            preimage("p1", 1, "t", "D", None, "who"),
            b"p1|1|t|D||who".to_vec()
        );
    }
}
