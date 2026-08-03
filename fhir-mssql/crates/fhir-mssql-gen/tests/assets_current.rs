//! G2.2 as a gate: the committed assets must be what this generator produces.
//!
//! Skips — loudly — when the FHIR packages are absent, because absent input is
//! not drift. `T11.12`: a skip that reads as a pass is the failure this whole
//! suite exists to avoid, so it says which one happened.

use fhir_mssql_gen::assets;

#[test]
fn committed_assets_match_this_generator() {
    let Some(defs) = assets::definitions_root() else {
        eprintln!(
            "SKIPPING committed_assets_match_this_generator: no FHIR definitions. \
             Set {} to run it.",
            assets::ENV_SPEC_DIR
        );
        return;
    };
    let arts = match assets::regenerate(&defs, &assets::assets_root(), false) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("SKIPPING committed_assets_match_this_generator: {e}");
            return;
        }
    };
    let drifted: Vec<_> = arts.iter().filter(|a| !a.is_current()).collect();
    assert!(
        drifted.is_empty(),
        "G2.2: {} committed asset(s) are not what this generator produces: {}. \
         Regenerate with `cargo run --bin regen-assets`, and treat it as a data \
         migration (L12, O10.4a).",
        drifted.len(),
        drifted
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// The determinism G2.2 actually asserts. Two runs over the same input must
/// agree byte for byte — which holds only because the gzip container carries
/// MTIME = 0.
#[test]
fn regenerate_is_deterministic() {
    let Some(defs) = assets::definitions_root() else {
        eprintln!("SKIPPING regenerate_is_deterministic: no FHIR definitions.");
        return;
    };
    let root = assets::assets_root();
    let a = assets::regenerate(&defs, &root, false).expect("first run");
    let b = assets::regenerate(&defs, &root, false).expect("second run");
    let sha = |v: &[assets::Artifact]| v.iter().map(|x| x.sha256.clone()).collect::<Vec<_>>();
    assert_eq!(sha(&a), sha(&b), "G2.2: generation is not deterministic");
}
