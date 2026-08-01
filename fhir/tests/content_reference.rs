//! Every recursive element resolves to its backbone struct, in every release
//! (spec R13.1 support; tasks.md T35).
//!
//! Two defects came from this one construct: `ConceptMap...target.product`
//! and `PackagedProductDefinition.characteristic` were both typed as a bare
//! `Element` or raw JSON instead of the backbone their `contentReference`
//! names, so `product` silently dropped data and `characteristic` silently
//! dropped structure.
//!
//! The example corpus found the first and could never have found the second,
//! because raw JSON round-trips fine. So this test enumerates the construct
//! from the specification instead of waiting for data to expose it — the
//! difference between checking a class of defect and sampling for it.
//!
//! # Why it runs against every release, not just R5
//!
//! It used to check R5 alone, and that is exactly where it was blind. The
//! releases do not spell this construct the same way: R3 onwards write
//! `contentReference` holding a path, DSTU2 writes `nameReference` naming an
//! element, and DSTU1 writes `nameReference` holding a full path nested
//! under `definition`. Only the modern spelling was understood, so 92 DSTU2
//! elements — `Bundle.entry.link` and every nested `ValueSet` concept among
//! them — were dropped from the model entirely. A test pinned to R5 could
//! never have seen it.
//!
//! Enumeration goes through `codegen::spec`, which normalizes all three
//! spellings, so this covers whatever spelling a release actually uses.
//!
//! # Why it snapshots paths rather than counting them
//!
//! The first version asserted a floor — "R5 has at least 70". A floor only
//! catches a collapse. Losing three elements, or one element quietly
//! changing from `Vec<QuestionnaireItem>` to `Vec<types::Element>`, passed
//! it without complaint, and those are the failures this construct actually
//! produces: the two original defects were both a *type* degrading, not a
//! field disappearing.
//!
//! So each release has a committed snapshot of every recursive element and
//! the Rust type it resolved to. A regression names the element and shows
//! the type it changed to. Regenerate deliberately with:
//!
//! ```sh
//! UPDATE_EXPECT=1 cargo test --test content_reference
//! ```
//!
//! and read the diff before committing it — a shrinking snapshot is the
//! signal this test exists to raise.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One modelled release: its specification directory and generated crate.
struct Release {
    name: &'static str,
    defs: &'static str,
    crate_dir: &'static str,
    snapshot: &'static str,
}

const RELEASES: &[Release] = &[
    Release {
        name: "R2",
        defs: "doc/fhir-specifications/r2/fhir-definitions-json",
        crate_dir: "fhir-release-2",
        snapshot: "tests/data/recursive_elements_r2.txt",
    },
    Release {
        name: "R3",
        defs: "doc/fhir-specifications/r3/fhir-definitions-json",
        crate_dir: "fhir-release-3",
        snapshot: "tests/data/recursive_elements_r3.txt",
    },
    Release {
        name: "R4",
        defs: "doc/fhir-specifications/r4/fhir-definitions-json",
        crate_dir: "fhir-release-4",
        snapshot: "tests/data/recursive_elements_r4.txt",
    },
    Release {
        name: "R5",
        defs: "doc/fhir-specifications/r5/fhir-definitions-json",
        crate_dir: "fhir-release-5",
        snapshot: "tests/data/recursive_elements_r5.txt",
    },
];

/// What a recursive element resolved to in the generated code.
#[derive(PartialEq, Eq)]
enum Resolved {
    /// The field exists, with this Rust type.
    Type(String),
    /// The module exists but declares no such field — the failure mode that
    /// cost DSTU2 92 elements.
    NoField,
    /// The owning module was not found on disk.
    NoModule,
}

impl std::fmt::Display for Resolved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(t) => write!(f, "{t}"),
            Self::NoField => write!(f, "!! no field generated"),
            Self::NoModule => write!(f, "!! module not found"),
        }
    }
}

/// `Foo.barBaz` -> `bar_baz`, matching the generator's field naming.
fn snake_leaf(path: &str) -> String {
    to_snake(path.rsplit('.').next().unwrap_or(path))
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// The Rust struct that owns an element, from its parent path.
///
/// `Bundle.entry.link` lives on `BundleEntry`, `Composition.section.section`
/// on `CompositionSection`: the generator names a backbone struct by
/// Pascal-joining the segments of its path.
fn owning_struct(path: &str) -> Option<String> {
    let mut segments: Vec<&str> = path.split('.').collect();
    segments.pop()?;
    if segments.len() < 2 {
        // A top-level field lives on the resource struct itself.
        return segments.first().map(|s| pascal(s));
    }
    Some(segments.iter().map(|s| pascal(s)).collect())
}

fn pascal(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

/// The body of `pub struct <name>` up to its closing brace.
fn struct_body<'a>(module: &'a str, name: &str) -> Option<&'a str> {
    let at = module.find(&format!("pub struct {name} "))?;
    let rest = &module[at..];
    let end = rest.find("\n}")?;
    Some(&rest[..end])
}

/// The declared Rust type of `pub <field>:` **within the owning struct**.
///
/// Scoping to the struct is not a detail. Searching the whole module and
/// taking the first `pub link:` matched `Bundle.link` when the element was
/// `Bundle.entry.link` — a different field on a different struct that
/// happens to share a leaf name and, in that case, a type. The check passed
/// while reading the wrong declaration entirely, so degrading
/// `BundleEntry.link` to `Vec<types::Element>` went undetected.
fn field_type(module: &str, path: &str) -> Option<String> {
    let owner = owning_struct(path)?;
    let body = struct_body(module, &owner)?;
    let at = body.find(&format!("pub {}:", snake_leaf(path)))?;
    let rest = &body[at..];
    let colon = rest.find(':')?;
    // A field declaration ends at the comma that closes it. Generic
    // arguments never contain one in this generator's output
    // (`Vec<Foo>`, `Option<Box<Foo>>`), so this is unambiguous.
    let end = rest.find(',')?;
    Some(rest[colon + 1..end].trim().to_string())
}

/// Every recursive element in one release, and what it resolved to.
fn resolve(release: &Release) -> BTreeMap<String, Resolved> {
    let dir = PathBuf::from(release.defs);
    let mut found = BTreeMap::new();

    for pkg in ["profiles-resources.json", "profiles-types.json"] {
        let path = dir.join(pkg);
        if !path.exists() {
            continue;
        }
        let Ok(defs) = fhir::codegen::spec::read_structure_definitions(&path) else {
            continue;
        };
        for sd in &defs {
            let Some(snapshot) = sd.snapshot.as_ref() else {
                continue;
            };
            for el in &snapshot.element {
                if el.content_reference_path().is_none() {
                    continue;
                }
                let file = to_snake(sd.type_name());
                let src = ["resources", "types"]
                    .iter()
                    .map(|d| PathBuf::from(format!("{}/src/{d}/{file}.rs", release.crate_dir)))
                    .find(|p| p.exists());
                let resolved = match src.and_then(|p| std::fs::read_to_string(p).ok()) {
                    None => Resolved::NoModule,
                    Some(body) => match field_type(&body, &el.path) {
                        Some(t) => Resolved::Type(t),
                        None => Resolved::NoField,
                    },
                };
                found.insert(el.path.clone(), resolved);
            }
        }
    }
    found
}

fn render(found: &BTreeMap<String, Resolved>) -> String {
    let mut out = String::new();
    for (path, resolved) in found {
        out.push_str(&format!("{path} -> {resolved}\n"));
    }
    out
}

#[test]
fn every_content_reference_resolves_to_a_struct() {
    let update = std::env::var("UPDATE_EXPECT").is_ok();
    let mut ran = 0usize;
    let mut problems: Vec<String> = Vec::new();

    for release in RELEASES {
        if !Path::new(release.defs).exists() || !Path::new(release.crate_dir).exists() {
            eprintln!("skipping {}: definitions or crate absent", release.name);
            continue;
        }
        ran += 1;
        let found = resolve(release);
        let actual = render(&found);

        // A degraded type is the defect this test was written for; say so
        // separately from a snapshot drift, because the fix is different.
        for (path, resolved) in &found {
            let bad = match resolved {
                Resolved::Type(t) => {
                    t.contains("types::Element") || t.contains("serde_json::Value")
                }
                Resolved::NoField | Resolved::NoModule => true,
            };
            if bad {
                problems.push(format!(
                    "{} {path}\n    resolved to: {resolved}\n    expected the backbone struct \
                     its contentReference names; as written it silently drops \
                     the element's contents or its structure",
                    release.name
                ));
            }
        }

        if update {
            std::fs::write(release.snapshot, &actual).expect("write snapshot");
            eprintln!(
                "{}: wrote {} ({} elements)",
                release.name,
                release.snapshot,
                found.len()
            );
            continue;
        }

        let expected = std::fs::read_to_string(release.snapshot).unwrap_or_default();
        if expected.is_empty() {
            problems.push(format!(
                "{}: no snapshot at {}. Create it with \
                 `UPDATE_EXPECT=1 cargo test --test content_reference`.",
                release.name, release.snapshot
            ));
            continue;
        }
        if actual != expected {
            problems.push(diff(release.name, &expected, &actual));
        } else {
            eprintln!(
                "{}: {} recursive elements match snapshot",
                release.name,
                found.len()
            );
        }
    }

    assert!(
        ran > 0,
        "no release could be checked; this test proved nothing"
    );
    assert!(update || problems.is_empty(), "{}", problems.join("\n\n"));
    assert!(!update, "snapshots rewritten; re-run without UPDATE_EXPECT");
}

/// Line-wise difference between two snapshots, naming what moved.
fn diff(release: &str, expected: &str, actual: &str) -> String {
    let old: BTreeMap<&str, &str> = expected.lines().filter_map(split_line).collect();
    let new: BTreeMap<&str, &str> = actual.lines().filter_map(split_line).collect();

    let mut out = format!("{release}: recursive elements changed\n");
    for (path, was) in &old {
        match new.get(path) {
            None => out.push_str(&format!("  LOST     {path} (was {was})\n")),
            Some(now) if now != was => {
                out.push_str(&format!(
                    "  RETYPED  {path}\n    was: {was}\n    now: {now}\n"
                ));
            }
            Some(_) => {}
        }
    }
    for (path, now) in &new {
        if !old.contains_key(path) {
            out.push_str(&format!("  NEW      {path} -> {now}\n"));
        }
    }
    out.push_str(
        "  If this is intended, regenerate with \
         `UPDATE_EXPECT=1 cargo test --test content_reference` and review the diff.",
    );
    out
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    let (path, ty) = line.split_once(" -> ")?;
    Some((path.trim(), ty.trim()))
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    /// A module with two structs sharing a field name, which is the shape
    /// that made the unscoped lookup wrong.
    const MODULE: &str = "\
pub struct Bundle {
    pub link: Vec<BundleLink>,
    pub entry: Vec<BundleEntry>,
}

pub struct BundleEntry {
    pub full_url: Option<types::Uri>,
    pub link: Vec<BundleLink>,
}
";

    #[test]
    fn the_owning_struct_comes_from_the_parent_path() {
        assert_eq!(
            owning_struct("Bundle.entry.link").as_deref(),
            Some("BundleEntry")
        );
        assert_eq!(
            owning_struct("Composition.section.section").as_deref(),
            Some("CompositionSection")
        );
        assert_eq!(
            owning_struct("QuestionnaireResponse.group.question.answer.group").as_deref(),
            Some("QuestionnaireResponseGroupQuestionAnswer")
        );
        // A top-level field belongs to the resource struct itself.
        assert_eq!(owning_struct("Bundle.link").as_deref(), Some("Bundle"));
    }

    #[test]
    fn a_field_is_read_from_its_own_struct() {
        // Both structs declare `link`. Before scoping, the lookup for
        // `Bundle.entry.link` returned `Bundle.link` — a different field on a
        // different struct — so 196 of 204 snapshot entries recorded a
        // declaration the element does not have, and degrading the real one
        // went unnoticed.
        let mut module = MODULE.to_string();
        assert_eq!(
            field_type(&module, "Bundle.entry.link").as_deref(),
            Some("Vec<BundleLink>")
        );

        // Change only BundleEntry.link; the lookup must follow it.
        module = module.replace(
            "    pub link: Vec<BundleLink>,\n}\n",
            "    pub link: Vec<types::Element>,\n}\n",
        );
        assert_eq!(
            field_type(&module, "Bundle.entry.link").as_deref(),
            Some("Vec<types::Element>"),
            "the lookup is reading a different struct's field"
        );
        // Bundle.link is untouched, so it must still read the original.
        assert_eq!(
            field_type(&module, "Bundle.link").as_deref(),
            Some("Vec<BundleLink>")
        );
    }

    #[test]
    fn an_absent_struct_or_field_is_not_silently_matched() {
        assert!(field_type(MODULE, "Nonexistent.thing.link").is_none());
        assert!(field_type(MODULE, "Bundle.entry.missing").is_none());
    }
}
