//! Regenerating the committed relational-map assets (`G2.1`, `G2.2`).
//!
//! `G2.1` requires the maps and DDL to be generated from the official FHIR
//! packages by the port's `gen` operation and **committed** under `assets/`, so
//! that a build never needs the packages. `G2.2` requires that generation be
//! deterministic — same spec input, byte-identical output — and records a
//! SHA-256 per artifact in `assets/CHECKSUMS.txt`.
//!
//! Until this module existed, `generate()` was a library function called only
//! from tests: nothing in the repository could reproduce the committed assets,
//! so `G2.2` was unverifiable and the checksums attested to artifacts of
//! unknown provenance (**F-40**).
//!
//! Both entry points below share one implementation on purpose. A writer and a
//! checker that drift apart would let CI pass against a rule the tool does not
//! apply.
//!
//! Determinism note: `flate2`'s `GzEncoder` writes `MTIME = 0`, so the gzip
//! container carries no timestamp and repeated runs are byte-identical. That is
//! load-bearing for `G2.2` and is asserted by `regenerate_is_deterministic`.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// Env var that overrides the definitions location.
pub const ENV_SPEC_DIR: &str = "FHIR_MYSQL_SPEC_DIR";

/// Filename stem of this port's committed maps.
const ASSET_PREFIX: &str = "fhir-mysql";

/// The FHIR versions carried in `assets/`. Adding one here is what makes it a
/// shipped artifact; the generator itself knows about more.
pub const VERSIONS: &[&str] = &["r3", "r4", "r5"];

/// What one artifact did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub name: String,
    pub sha256: String,
    /// `None` when the file did not exist; otherwise its content checksum
    /// before this run.
    pub previous_sha256: Option<String>,
    /// SHA-256 of the compressed file as written. Recorded in `CHECKSUMS.txt`
    /// so `shasum -a 256 -c` still verifies the artifact on disk — that is a
    /// download-integrity check, and a different question from drift (F-41).
    pub file_sha256: String,
    /// Whether this run actually rewrote the file. False when the content was
    /// already current, so a differing compression backend does not churn the
    /// working tree.
    pub rewritten: bool,
}

impl Artifact {
    #[must_use]
    pub fn is_current(&self) -> bool {
        self.previous_sha256.as_deref() == Some(self.sha256.as_str())
    }
}

#[derive(Debug)]
pub enum AssetError {
    Definitions(PathBuf),
    Generate(String),
    Io(std::io::Error),
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Definitions(p) => write!(f, "no FHIR definitions at {}", p.display()),
            Self::Generate(m) => write!(f, "generate: {m}"),
            Self::Io(e) => write!(f, "io: {e}"),
        }
    }
}
impl std::error::Error for AssetError {}
impl From<std::io::Error> for AssetError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Where the FHIR specification packages live.
///
/// The env var first, so a caller can point at any checkout; then this
/// monorepo's model family, which is where they actually are. There is no
/// third candidate: a fallback that cannot exist is what made every
/// spec-dependent test skip while reporting success (**F-39**).
#[must_use]
pub fn definitions_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var(ENV_SPEC_DIR).ok().map(PathBuf::from),
        Some(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../fhir/doc/fhir-specifications"
        ))),
    ];
    candidates.into_iter().flatten().find(|p| p.exists())
}

/// The port's `assets/` directory.
#[must_use]
pub fn assets_root() -> PathBuf {
    PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fhir-mysql-map/assets"
    ))
}

/// The map's canonical bytes: the JSON inside the gzip container.
///
/// `G2.2` asks for byte-identical output, and the *content* is byte-identical
/// while the container is not. `flate2` picks its backend by Cargo feature
/// unification, so a workspace that also pulls in `mysql_async` — which enables
/// `flate2/zlib` — compresses with the C zlib backend while the gen crate alone
/// uses `miniz_oxide`. Same input, same map, different compressed bytes
/// (**F-41**). Comparing content makes drift detection a property of the
/// generator rather than of an unrelated dependency's feature flags.
fn decompress(gz: &[u8]) -> Result<Vec<u8>, AssetError> {
    use std::io::Read as _;
    let mut out = Vec::new();
    flate2::read::GzDecoder::new(gz)
        .read_to_end(&mut out)
        .map_err(AssetError::Io)?;
    Ok(out)
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Generate every shipped version and compare — or write.
///
/// With `write = false` nothing on disk is touched, which is what the test and
/// the `--check` flag use. The returned artifacts say what *would* change.
///
/// # Errors
/// If the definitions are absent, a version fails to generate, or I/O fails.
pub fn regenerate(
    definitions: &Path,
    assets: &Path,
    write: bool,
) -> Result<Vec<Artifact>, AssetError> {
    if !definitions.exists() {
        return Err(AssetError::Definitions(definitions.to_path_buf()));
    }
    let mut out = Vec::new();
    for ver in VERSIONS {
        let defs = definitions.join(ver).join("fhir-definitions-json");
        if !defs.exists() {
            return Err(AssetError::Definitions(defs));
        }
        let map = crate::generate(&defs, ver).map_err(|e| AssetError::Generate(format!("{e}")))?;
        let bytes = map
            .to_gz_bytes()
            .map_err(|e| AssetError::Generate(e.to_string()))?;
        // Identity is the map's *content*, not the gzip container (F-41).
        let content = decompress(&bytes)?;
        let sha = hex(&Sha256::digest(&content));

        let name = format!("{ASSET_PREFIX}-relmap-{ver}.json.gz");
        let path = assets.join(&name);
        let previous = std::fs::read(&path)
            .ok()
            .and_then(|b| decompress(&b).ok())
            .map(|c| hex(&Sha256::digest(&c)));

        let file_sha = hex(&Sha256::digest(&bytes));
        let unchanged = previous.as_deref() == Some(sha.as_str());
        if write && !unchanged {
            std::fs::create_dir_all(assets)?;
            std::fs::write(&path, &bytes)?;
        }
        out.push(Artifact {
            name,
            sha256: sha,
            previous_sha256: previous,
            file_sha256: file_sha,
            rewritten: write && !unchanged,
        });
    }

    if write {
        let mut txt = String::new();
        for a in &out {
            use std::fmt::Write as _;
            // `sha256sum` format: two spaces, so `shasum -a 256 -c` accepts it.
            let _ = writeln!(txt, "{}  {}", a.file_sha256, a.name);
        }
        std::fs::write(assets.join("CHECKSUMS.txt"), txt)?;
    }
    Ok(out)
}
