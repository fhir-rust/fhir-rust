//! `src/r5` has not drifted from the generator (tasks.md T35).
//!
//! R3 and R4 are regenerated wholesale, so they inherit every generator fix
//! automatically. R5 is hand-documented and never regenerated — `cargo run --
//! r5` refuses without `--out` precisely because it carries prose the
//! generator cannot produce. (The typed `Reference<T>` used to be a second
//! reason; the generator learned to emit it under T11.) The cost of that
//! protection is that R5 silently inherits *no* generator fix either, and the
//! release that ships by default was the one carrying the risk.
//!
//! Five real defects came from exactly that gap: `ConceptMap...product` and
//! `PackagedProductDefinition.characteristic` (unresolved `contentReference`),
//! `Ratio.denominator` (wrong cardinality), three missing `_use` siblings, and
//! `ProductShelfLife.modifierExtension`. Each dropped data or rejected valid
//! resources, and none was visible in a build or a corpus run.
//!
//! So: generate R5 to a temporary directory and compare field types per
//! struct. Every difference must be listed below with a reason. The point is
//! not that the two trees are identical — they deliberately are not — but
//! that each place they differ is a decision somebody made, rather than a fix
//! that never arrived.

#![cfg(feature = "r5")]

use std::collections::BTreeMap;
use std::path::Path;

/// A difference that is intended, and why.
struct Sanctioned {
    /// `struct::field`, or `struct::*` for a whole type.
    what: &'static str,
    why: &'static str,
}

const SANCTIONED: &[Sanctioned] = &[
    Sanctioned {
        what: "Task::intent",
        why: "`TaskIntent` generates with a single `Unknown` variant, so \
              `Coded<E>` would turn every real value into Unknown while \
              claiming type safety (T36)",
    },
    Sanctioned {
        what: "Transport::intent",
        why: "as Task::intent — `TransportIntent` is a one-variant enum (T36)",
    },
    Sanctioned {
        what: "DetectedIssue::status",
        why: "as Task::intent — `DetectedissueStatus` is a one-variant enum (T36)",
    },
    Sanctioned {
        what: "ImagingSelectionInstance::image_region_2_d",
        why: "spelled `image_region2_d` in src/r5; both derive the same               `imageRegion2D` wire name, and the corpus exercises it",
    },
    Sanctioned {
        what: "ImagingSelectionInstance::image_region_3_d",
        why: "as image_region_2_d — an identifier difference, not a wire one",
    },
    Sanctioned {
        what: "Identifier::assigner",
        why: "the Reference/Identifier type cycle needs one Box; the generator \
              breaks it here, src/r5 breaks it at Reference::identifier — one \
              Box either way, at opposite edges (see that entry)",
    },
    Sanctioned {
        what: "Reference::identifier",
        why: "src/r5's half of the cycle break: it boxes this edge and leaves \
              Identifier::assigner unboxed, the mirror image of the \
              generator's choice. Converging would touch every constructor of \
              either type for zero wire or size difference",
    },
];

fn normalize(ty: &str) -> String {
    let ty: String = ty.split_whitespace().collect();
    ty.replace("::vec1::", "vec1::")
        .replace("crate::r5::coded::", "crate::coded::")
        .replace("crate::r5::codes::", "codes::")
        .replace("crate::r5::types::", "types::")
        .replace("crate::coded::", "coded::")
        .trim_end_matches(',')
        .to_string()
}

/// `(struct, field) -> type`, keyed by struct because one file holds many and
/// several share field names — comparing per file comparted unrelated structs
/// and produced a long list of phantom differences.
fn fields_of(path: &Path) -> BTreeMap<(String, String), String> {
    let mut out = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return out;
    };
    let text: String = text
        .lines()
        .map(|l| match l.find("//") {
            Some(i) if !l[..i].contains('"') => &l[..i],
            _ => l,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut rest = text.as_str();
    while let Some(at) = rest.find("pub struct ") {
        rest = &rest[at + "pub struct ".len()..];
        let Some(brace) = rest.find('{') else { break };
        let name: String = rest[..brace]
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .find(|s| !s.is_empty())
            .unwrap_or("")
            .to_string();
        let Some(end) = rest.find("\n}") else { break };
        let body = &rest[brace + 1..end];
        for decl in body.split("pub ").skip(1) {
            let Some(colon) = decl.find(':') else {
                continue;
            };
            let field = decl[..colon].trim();
            if field.is_empty()
                || !field
                    .chars()
                    .all(|c| c.is_lowercase() || c == '_' || c.is_ascii_digit())
            {
                continue;
            }
            // Scan to the terminating comma at generic depth zero. Splitting
            // on ",\n" breaks the moment a stripped trailing comment leaves a
            // space before the newline, which silently swallowed the next
            // field's attribute into the type.
            let after = &decl[colon + 1..];
            let mut depth = 0i32;
            let mut end = after.len();
            for (i, c) in after.char_indices() {
                match c {
                    '<' | '(' | '[' => depth += 1,
                    '>' | ')' | ']' => depth -= 1,
                    ',' if depth == 0 => {
                        end = i;
                        break;
                    }
                    _ => {}
                }
            }
            out.insert((name.clone(), field.to_string()), normalize(&after[..end]));
        }
        rest = &rest[end..];
    }
    out
}

#[test]
fn r5_matches_the_generator_except_where_stated() {
    let defs = Path::new("doc/fhir-specifications/r5/fhir-definitions-json");
    if !defs.exists() {
        eprintln!("skipping: no bundled R5 definitions");
        return;
    }
    let tmp = std::env::temp_dir().join(format!("fhir-release-5-drift-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    if fhir::codegen::generate_into(fhir::codegen::Version::R5, &tmp).is_err() {
        eprintln!("skipping: generator could not run");
        return;
    }

    // Print the sanctioned list every run. A reason nobody ever reads decays
    // into a rubber stamp; showing them keeps each one answerable.
    println!("sanctioned differences between src/r5 and the generator:");
    for s in SANCTIONED {
        println!("  {} — {}", s.what, s.why);
    }

    let mut unexplained: Vec<String> = Vec::new();
    let mut matched = 0usize;
    for sub in ["resources", "types"] {
        let Ok(entries) = std::fs::read_dir(tmp.join(sub)) else {
            continue;
        };
        for e in entries.flatten() {
            let gen_path = e.path();
            if gen_path.extension().is_none_or(|x| x != "rs") {
                continue;
            }
            let name = gen_path.file_name().expect("file name");
            let src_path = Path::new("fhir-release-5/src").join(sub).join(name);
            if !src_path.exists() {
                continue;
            }
            let (g, s) = (fields_of(&gen_path), fields_of(&src_path));
            for (key, gen_ty) in &g {
                let sanctioned = SANCTIONED.iter().any(|x| {
                    x.what == format!("{}::{}", key.0, key.1) || x.what == format!("{}::*", key.0)
                });
                if sanctioned {
                    matched += 1;
                    continue;
                }
                match s.get(key) {
                    Some(src_ty) if src_ty == gen_ty => {}
                    Some(src_ty) => unexplained.push(format!(
                        "{}::{}\n      generator: {gen_ty}\n      src/r5:    {src_ty}",
                        key.0, key.1
                    )),
                    None => unexplained.push(format!(
                        "{}::{} is absent from src/r5 (generator: {gen_ty})",
                        key.0, key.1
                    )),
                }
            }
        }
    }
    let _ = std::fs::remove_dir_all(&tmp);
    println!("fields covered by a sanctioned entry: {matched}");

    assert!(
        unexplained.is_empty(),
        "fhir-release-5/src differs from the generator in {} place(s) that nobody has \
         accounted for. Each is either a generator fix R5 never received, or \
         an intentional divergence that belongs in SANCTIONED with a reason:\n\n{}",
        unexplained.len(),
        unexplained.join("\n")
    );
}
