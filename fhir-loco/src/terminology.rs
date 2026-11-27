//! The SNOMED CT® release behind the terminology operations (`SV1.11`,
//! `SV2.20`).
//!
//! One RF2 Snapshot, loaded into memory once at boot and never replaced —
//! the same lifetime as the FHIR stores, and a `OnceLock` for the same
//! reason (`crate::store`). The terminology semantics are `snomed-fhir`'s;
//! this module only holds the release and the two pieces of context
//! snomed-rust's `spec/11-fhir.md` says only the host has: which release
//! this is (`version`), and which language refset stands for the display
//! language.

use std::path::Path;
use std::sync::OnceLock;

use snomed::core::sctid::SctId;
use snomed::rf2::ReleaseType;
use snomed::store::{SnapshotStore, SnapshotStoreBuilder};

/// `FHIR_LOCO_SNOMED_RF2`: the extracted RF2 release directory.
pub const ENV_RF2: &str = "FHIR_LOCO_SNOMED_RF2";
/// `FHIR_LOCO_SNOMED_VERSION`: the loaded release's version URI.
pub const ENV_VERSION: &str = "FHIR_LOCO_SNOMED_VERSION";
/// `FHIR_LOCO_SNOMED_LANGUAGE_REFSET`: the language refset for `display`.
pub const ENV_LANGUAGE_REFSET: &str = "FHIR_LOCO_SNOMED_LANGUAGE_REFSET";

/// The loaded release and the host-supplied context around it.
pub struct Terminology {
    pub store: SnapshotStore,
    /// The configured version URI, if any. A requested version is served
    /// only when it equals this (`SV2.20`).
    pub version: Option<String>,
    pub language_refset: Option<SctId>,
}

static TERMINOLOGY: OnceLock<Terminology> = OnceLock::new();

/// The loaded release, or `None` when none was configured.
pub fn get() -> Option<&'static Terminology> {
    TERMINOLOGY.get()
}

/// Make `terminology` the process's release. A second call is an error:
/// the release is immutable for the process lifetime.
pub fn install(terminology: Terminology) -> Result<(), String> {
    TERMINOLOGY
        .set(terminology)
        .map_err(|_| "a SNOMED CT release is already loaded".to_string())
}

/// Load `dir` as an RF2 Snapshot. Loading no concepts is an error: a
/// terminology server answering "unknown code" to everything looks like a
/// working one (`SV2.20`).
pub fn load(
    dir: &Path,
    version: Option<String>,
    language_refset: Option<SctId>,
) -> Result<Terminology, String> {
    let mut builder = SnapshotStoreBuilder::new();
    let report = builder
        .load_release_dir(dir, ReleaseType::Snapshot)
        .map_err(|e| format!("loading {ENV_RF2}: {e}"))?;
    for (path, reason) in &report.skipped {
        tracing::debug!(path = %path.display(), reason, "SNOMED CT file skipped");
    }
    let store = builder.build();
    if store.concept_count() == 0 {
        return Err(format!(
            "{ENV_RF2} loaded no concepts ({} file(s) read, {} skipped); \
             is it an extracted RF2 Snapshot?",
            report.loaded.len(),
            report.skipped.len()
        ));
    }
    Ok(Terminology {
        store,
        version,
        language_refset,
    })
}

/// Read the three settings and, when `FHIR_LOCO_SNOMED_RF2` is set, load
/// and install the release. `Ok(None)` when it is unset; every malformed
/// setting is an error naming it (RCA-IN-4), never a default.
pub fn init_from_env() -> Result<Option<usize>, String> {
    let version = crate::config::env_opt(ENV_VERSION)?
        .map(|v| parse_version(&v))
        .transpose()?;
    let language_refset = crate::config::env_opt(ENV_LANGUAGE_REFSET)?
        .map(|v| parse_refset(&v))
        .transpose()?;
    let Some(dir) = crate::config::env_opt(ENV_RF2)? else {
        if version.is_some() || language_refset.is_some() {
            return Err(format!(
                "{ENV_VERSION}/{ENV_LANGUAGE_REFSET} are set but {ENV_RF2} is not"
            ));
        }
        return Ok(None);
    };
    let terminology = load(Path::new(&dir), version, language_refset)?;
    let concepts = terminology.store.concept_count();
    install(terminology)?;
    Ok(Some(concepts))
}

/// `http://snomed.info/sct/[edition SCTID]/version/[YYYYMMDD]`
/// (snomed-rust `spec/11-fhir.md`, "System and version URIs").
fn parse_version(raw: &str) -> Result<String, String> {
    let malformed = || {
        format!(
            "{ENV_VERSION} must be http://snomed.info/sct/[edition]/version/[YYYYMMDD], got {raw:?}"
        )
    };
    let rest = raw
        .strip_prefix("http://snomed.info/sct/")
        .ok_or_else(malformed)?;
    let (edition, date) = rest.split_once("/version/").ok_or_else(malformed)?;
    let valid_date = date.len() == 8 && date.bytes().all(|b| b.is_ascii_digit());
    if SctId::parse(edition).is_err() || !valid_date {
        return Err(malformed());
    }
    Ok(raw.to_string())
}

fn parse_refset(raw: &str) -> Result<SctId, String> {
    SctId::parse(raw.trim())
        .map_err(|_| format!("{ENV_LANGUAGE_REFSET} must be an SCTID, got {raw:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_version_must_be_a_snomed_version_uri() {
        let good = "http://snomed.info/sct/900000000000207008/version/20240101";
        assert_eq!(parse_version(good), Ok(good.to_string()));
        for bad in [
            "",
            "20240101",
            "http://snomed.info/sct",
            "http://snomed.info/sct/900000000000207008",
            "http://snomed.info/sct/900000000000207008/version/2024",
            "http://snomed.info/sct/900000000000207008/version/2024010x",
            "http://snomed.info/sct/123/version/20240101",
            "https://snomed.info/sct/900000000000207008/version/20240101",
        ] {
            let err = parse_version(bad).expect_err(bad);
            assert!(err.starts_with(ENV_VERSION), "{err}");
        }
    }

    #[test]
    fn a_language_refset_must_be_an_sctid() {
        assert_eq!(
            parse_refset("900000000000509007").map(|id| id.to_string()),
            Ok("900000000000509007".to_string())
        );
        for bad in ["", "en-US", "123"] {
            assert!(parse_refset(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn an_empty_directory_is_refused_not_served() {
        let dir = std::env::temp_dir().join(format!("fhir-loco-empty-rf2-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch");
        let err = load(&dir, None, None)
            .err()
            .expect("no concepts is an error");
        assert!(err.contains("loaded no concepts"), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
