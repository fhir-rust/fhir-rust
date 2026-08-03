//! Regenerate the committed relational-map assets (`G2.1`, `G2.2`).
//!
//! ```text
//! cargo run -p fhir-sqlite-gen --bin regen-assets            # rewrite assets/ and CHECKSUMS.txt
//! cargo run -p fhir-sqlite-gen --bin regen-assets -- --check # report drift, write nothing
//! ```
//!
//! `--check` is what CI runs: it exits non-zero when the committed assets are
//! not what this tree's generator produces, which is `G2.2` stated as a gate
//! rather than as a hope.
//!
//! This is a developer tool inside the `gen` crate, not a product CLI. `C0.18`
//! is explicit that `gen` names a **library operation**; this binary is the
//! literal reading of `G2.1`'s "generated ... by the port's `gen` operation",
//! and there is still no CLI crate in any port.

use fhir_sqlite_gen::assets;

fn main() -> std::process::ExitCode {
    let check = std::env::args().any(|a| a == "--check");

    let Some(defs) = assets::definitions_root() else {
        eprintln!(
            "no FHIR definitions found. Set {} or check out the model family \
             at fhir/doc/fhir-specifications.",
            assets::ENV_SPEC_DIR
        );
        // Absent input is not drift. Failing here would make the gate red for
        // a reason that says nothing about the assets (T11.12: say which).
        return std::process::ExitCode::from(2);
    };
    let out = assets::assets_root();

    match assets::regenerate(&defs, &out, !check) {
        Err(e) => {
            eprintln!("regen-assets: {e}");
            std::process::ExitCode::FAILURE
        }
        Ok(arts) => {
            let mut drifted = 0;
            for a in &arts {
                let state = match (&a.previous_sha256, a.is_current()) {
                    (None, _) => "NEW",
                    (Some(_), true) => "ok",
                    (Some(_), false) => {
                        drifted += 1;
                        "DRIFTED"
                    }
                };
                println!("  {state:<8} {}  {}", &a.sha256[..16], a.name);
            }
            if check && drifted > 0 {
                eprintln!(
                    "\n{drifted} asset(s) differ from what this tree generates (G2.2).\n\
                     The committed map is not the output of this generator. Run without \
                     --check to regenerate, and treat the result as a data migration \
                     (L12, O10.4a) rather than a file edit."
                );
                return std::process::ExitCode::FAILURE;
            }
            if !check {
                println!("\nwrote {} artifact(s) + CHECKSUMS.txt", arts.len());
            }
            std::process::ExitCode::SUCCESS
        }
    }
}
