//! Command-line entry point for the FHIR specifications code generator.
//!
//! ```sh
//! cargo run -- r4                  # regenerate fhir-release-4/src from the R4 definitions
//! cargo run -- r5 --out tmp/out/r5 # regenerate R5 somewhere safe, to compare
//! ```
//!
//! See [`fhir::codegen`] for what the generator emits and how.

use std::path::PathBuf;
use std::process::ExitCode;

use fhir::codegen::{self, Version};

/// How to run the generator.
struct Options {
    /// The release to generate.
    version: Version,
    /// Where to write it.
    out: PathBuf,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = match parse_args(&args) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    println!(
        "Generating {} into {}",
        options.version.label(),
        options.out.display()
    );
    match codegen::generate_into(options.version, &options.out) {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("generation failed: {e}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "\
Usage: cargo run -- <release> [--out <dir>]

  <release>     r2, r3, r4, r5, or r6 (also accepts dstu2/stu3 and
                version numbers like 1.0.2, 6.0.0-ballot3)
  --out <dir>   where to write the model (default: fhir-release-N/src)

R5 has no default output directory: the shipped fhir-release-5/src modules carry
hand-written documentation that regeneration would overwrite, so an explicit
--out is required for it.";

/// Parse the command line, defaulting the output directory where it is safe to.
fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut version = None;
    let mut out = None;
    let mut rest = args.iter();

    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--out" => {
                let dir = rest.next().ok_or("--out needs a directory")?;
                out = Some(PathBuf::from(dir));
            }
            "-h" | "--help" => return Err("Showing usage.".to_string()),
            other => {
                version = Some(
                    Version::parse(other).ok_or_else(|| format!("unknown release {other:?}"))?,
                );
            }
        }
    }

    let version = version.ok_or("no release given")?;
    let out = match (out, version) {
        (Some(dir), _) => dir,
        // R5 is the one release whose modules carry hand-written prose that
        // regeneration would destroy.
        (None, Version::R5) => {
            return Err(
                "refusing to regenerate over fhir-release-5/src, which is hand-documented; \
                 pass --out to write elsewhere"
                    .to_string(),
            );
        }
        (None, _) => version.source_dir(),
    };
    Ok(Options { version, out })
}
