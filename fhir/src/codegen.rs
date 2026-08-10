//! The spec-JSON to Rust code generator, parameterized by FHIR release.
//!
//! This crate is spec-driven: the official FHIR definition JSON is the input
//! and the `src/<release>` module tree is the output. Point the generator at a
//! [`Version`] and it writes that release's complete model — primitives,
//! complex datatypes, resources, code-system enums, the `Resource` enum, and
//! the `meta` element table:
//!
//! ```sh
//! cargo run -- r4      # writes fhir-r4/src/
//! ```
//!
//! # What it emits
//!
//! Unlike the older R5-only parse layer in [`crate::r5::parse`], which produced
//! a *starting point* for hand-finishing in `tmp/out/`, this generator emits
//! the finished shape in one pass: nested backbone structs (not flattened
//! ones), `value[x]` choice enums, `Coded<E>` fields for required bindings,
//! `Vec1` for `1..*`, primitive-extension siblings, builders, and per-module
//! round-trip tests. `fhir-r4/src` is therefore fully regenerable; `fhir-r5/src` is not,
//! because its documentation was written by hand on top of generated shapes.
//!
//! # The pipeline
//!
//! ```text
//! doc/fhir-specifications/<release>/fhir-definitions-json/*.json
//!         │  spec::read_structure_definitions / read_code_systems
//!         ▼
//! codes_gen::plan ─────────────────────────┐  (enum names feed field typing)
//!         │                                ▼
//! plan::plan_type ──▶ break_type_cycles ──▶ resolve_defaults
//!         │
//!         ▼  render::render_type / primitives::render / codes_gen::render
//! src/<release>/{types,resources,codes.rs,meta/generated.rs}
//! ```

use std::fmt::Write as _;
use std::path::Path;

pub mod codes_gen;
pub mod extension_ext_gen;
pub mod meta_gen;
pub mod naming;
pub mod plan;
pub mod primitives;
pub mod reference_gen;
pub mod render;
pub mod spec;
pub mod version;

pub use version::Version;

/// Generate the complete model for one FHIR release into its `src/<release>`
/// directory.
///
/// Existing generated files are overwritten. Every step is deterministic, so
/// re-running on unchanged inputs produces a byte-identical tree and a clean
/// `git diff`.
pub fn generate(version: Version) -> std::io::Result<Summary> {
    generate_into(version, &version.source_dir())
}

/// Generate one FHIR release into an arbitrary directory.
///
/// Writing somewhere other than `src/<release>` is how R5 is regenerated: its
/// shipped modules carry hand-written prose that generation would destroy, so
/// it is emitted to a scratch directory and compared instead.
pub fn generate_into(version: Version, out_root: &Path) -> std::io::Result<Summary> {
    let type_definitions = spec::read_structure_definitions(&version.types_bundle())?;
    let resource_definitions = spec::read_structure_definitions(&version.resources_bundle())?;

    // Code enums first: a field bound to a value set is typed as its enum, so
    // field planning needs to know which enums exist.
    let code_enums = plan_code_enums(version)?;

    let primitive_defs = primitives::codes_of(&type_definitions);
    let ctx = plan::Context {
        primitives: primitive_defs.keys().cloned().collect(),
        // Only enums with real alternatives are offered for binding. A
        // one-variant enum cannot represent its value set (T36): binding a
        // field to it turns every genuine code into `Coded::Unknown` while
        // the signature advertises type safety, which is worse than leaving
        // the field a plain `Code`.
        code_enums: code_enums
            .iter()
            .filter(|e| e.variants.len() > 1)
            .map(|e| e.name.clone())
            .collect(),
        resources: resource_definitions
            .iter()
            .map(|sd| naming::pascal(sd.type_name()))
            .collect(),
        module: version.module().to_string(),
    };

    // Plan every datatype and resource, then settle the two facts that can only
    // be known across the whole model.
    let mut type_plans: Vec<plan::TypePlan> = type_definitions
        .iter()
        .filter(|sd| sd.kind_name() == "complex-type")
        .filter_map(|sd| plan::plan_type(sd, &ctx))
        .collect();
    let mut resource_plans: Vec<plan::TypePlan> = resource_definitions
        .iter()
        .filter(|sd| sd.kind_name() == "resource" && !sd.is_abstract)
        .filter_map(|sd| plan::plan_type(sd, &ctx))
        .collect();

    let mut all: Vec<plan::TypePlan> = type_plans
        .iter()
        .chain(resource_plans.iter())
        .cloned()
        .collect();
    plan::break_type_cycles(&mut all);
    plan::resolve_defaults(&mut all);
    let (planned_types, planned_resources) = all.split_at(type_plans.len());
    type_plans = planned_types.to_vec();
    resource_plans = planned_resources.to_vec();

    type_plans.sort_by(|a, b| a.type_name.cmp(&b.type_name));
    resource_plans.sort_by(|a, b| a.type_name.cmp(&b.type_name));

    let root = out_root.to_path_buf();
    let types_dir = root.join("types");
    let resources_dir = root.join("resources");
    std::fs::create_dir_all(&types_dir)?;
    std::fs::create_dir_all(&resources_dir)?;

    // Primitives come from a table rather than the snapshot, because their Rust
    // representation is a design decision the specification does not state.
    let mut primitive_names = Vec::new();
    for (name, sd) in &primitive_defs {
        let Some(source) = primitives::render(sd, version) else {
            return Err(std::io::Error::other(format!(
                "no Rust representation is defined for the FHIR primitive {name:?}; \
                 add one to codegen::primitives::PRIMITIVES"
            )));
        };
        write_if_changed(
            &types_dir.join(format!("{}.rs", naming::module_name(name))),
            &source,
        )?;
        primitive_names.push(naming::pascal(name));
    }

    for plan in &type_plans {
        let source = render::render_type(plan, version);
        write_if_changed(&types_dir.join(format!("{}.rs", plan.module)), &source)?;
    }
    for plan in &resource_plans {
        let source = render::render_type(plan, version);
        write_if_changed(&resources_dir.join(format!("{}.rs", plan.module)), &source)?;
    }

    write_if_changed(
        &root.join("types.rs"),
        &render_types_module(
            &primitive_defs.keys().cloned().collect::<Vec<_>>(),
            &type_plans,
            version,
        ),
    )?;
    write_if_changed(
        &root.join("resources.rs"),
        &render_resources_module(&resource_plans, version),
    )?;
    write_if_changed(
        &root.join("codes.rs"),
        &codes_gen::render(&code_enums, version),
    )?;
    write_if_changed(
        &root.join("extension_ext.rs"),
        &extension_ext_gen::render(&type_plans, &resource_plans, version),
    )?;

    let meta_table = meta_gen::collect(version)?;
    std::fs::create_dir_all(root.join("meta"))?;
    write_if_changed(
        &root.join("meta").join("generated.rs"),
        &meta_gen::render(&meta_table, version),
    )?;

    Ok(Summary {
        version,
        primitives: primitive_defs.len(),
        datatypes: type_plans.len(),
        resources: resource_plans.len(),
        code_enums: code_enums.len(),
        elements: meta_table.len(),
    })
}

/// Plan every code enum for a release.
///
/// Both the `CodeSystem`s and the `ValueSet`s are read, because a binding
/// names a *value set*: where one composes several systems, the composition
/// is what the enum must cover, and reading only the like-named system leaves
/// an enum that cannot represent its own binding (tasks.md T36).
fn plan_code_enums(version: Version) -> std::io::Result<Vec<codes_gen::CodeEnum>> {
    let mut systems = Vec::new();
    let mut value_sets = Vec::new();
    for bundle in version.code_system_bundles() {
        systems.extend(spec::read_code_systems(&bundle)?);
        value_sets.extend(spec::read_value_sets(&bundle)?);
    }
    Ok(codes_gen::plan_with_value_sets(&systems, &value_sets))
}

/// What one generation run produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summary {
    /// The release generated.
    pub version: Version,
    /// Primitive datatypes written.
    pub primitives: usize,
    /// Complex datatypes written.
    pub datatypes: usize,
    /// Resources written.
    pub resources: usize,
    /// Code-system enums written.
    pub code_enums: usize,
    /// Elements in the generated `meta` table.
    pub elements: usize,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} primitives, {} datatypes, {} resources, {} code enums, {} elements",
            self.version.label(),
            self.primitives,
            self.datatypes,
            self.resources,
            self.code_enums,
            self.elements,
        )
    }
}

/// Write `source` to `path`, leaving the file alone if it already matches.
///
/// Generation is a whole-tree rewrite, and most runs change only a few files;
/// skipping the rest keeps timestamps (and therefore rebuilds) stable.
///
/// The source is run through `rustfmt` first. Without that, the generator and
/// `cargo fmt` disagree — the generator emits lines past the width limit,
/// rustfmt rewraps them, and the next generation unwraps them again — so the
/// two churn against each other forever across ~150 files per release.
/// Formatting here makes generated output *already* canonical, which is what
/// lets `cargo fmt --check` be a meaningful gate on a mostly-generated crate.
fn write_if_changed(path: &Path, source: &str) -> std::io::Result<()> {
    let source = rustfmt(source).unwrap_or_else(|| source.to_string());
    if std::fs::read_to_string(path).is_ok_and(|existing| existing == source) {
        return Ok(());
    }
    std::fs::write(path, source)
}

/// Format Rust source with the toolchain's `rustfmt`, or `None` when it is
/// unavailable or rejects the input.
///
/// Failure is not fatal: an unformatted-but-correct tree is better than a
/// failed generation, and `cargo fmt --check` will say so in CI.
fn rustfmt(source: &str) -> Option<String> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(source.as_bytes()).ok()?;
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

/// Render `src/<release>/types.rs`: the datatype module declarations and
/// re-exports.
fn render_types_module(
    primitive_names: &[String],
    plans: &[plan::TypePlan],
    version: Version,
) -> String {
    let label = version.label();
    let module = version.module();
    let mut names: Vec<(String, String)> = primitive_names
        .iter()
        .map(|n| (naming::module_name(n), naming::pascal(n)))
        .chain(
            plans
                .iter()
                .map(|p| (p.module.clone(), p.type_name.clone())),
        )
        .collect();
    names.sort();

    let mut out = format!(
        "//! FHIR {label} datatypes.\n\
         //!\n\
         //! This module contains every FHIR {label} datatype: the **complex** datatypes\n\
         //! (structs such as [`Period`], [`HumanName`], [`CodeableConcept`], [`Coding`])\n\
         //! and the **primitive** datatypes (transparent newtypes such as [`Code`],\n\
         //! [`Id`], [`DateTime`], [`Boolean`]).\n\
         //!\n\
         //! Each datatype derives `serde::Serialize` and `serde::Deserialize` and\n\
         //! round-trips to and from the canonical FHIR JSON representation. Primitives\n\
         //! serialize *transparently* to their underlying JSON value:\n\
         //!\n\
         //! ```\n\
         //! use fhir::{module}::types::Code;\n\
         //!\n\
         //! assert_eq!(serde_json::to_value(Code(\"final\".to_string())).unwrap(), \"final\");\n\
         //! ```\n\
         //!\n\
         //! Every datatype is re-exported at this module's root, so you can write\n\
         //! `fhir::{module}::types::Period` rather than `fhir::{module}::types::period::Period`.\n\
         \n"
    );
    for (module_name, _) in &names {
        let _ = writeln!(out, "pub mod {module_name};");
    }
    out.push('\n');
    for (module_name, type_name) in &names {
        let _ = writeln!(out, "pub use {module_name}::{type_name};");
    }
    out
}

/// Render `src/<release>/resources.rs`: the resource modules, their re-exports,
/// and the polymorphic `Resource` enum.
fn render_resources_module(plans: &[plan::TypePlan], version: Version) -> String {
    let label = version.label();
    let module = version.module();
    let mut out = format!(
        "//! FHIR {label} resources.\n\
         //!\n\
         //! This module contains the FHIR {label} resource types (Patient, Observation,\n\
         //! Encounter, and so on). Each resource is a Rust struct that serializes to\n\
         //! and from the canonical FHIR JSON representation via `serde`.\n\
         \n\
         use ::serde::{{Deserialize, Serialize}};\n\
         use fhir_derive_macros::Validate;\n\
         \n"
    );
    for plan in plans {
        let _ = writeln!(out, "pub mod {};", plan.module);
    }
    out.push('\n');
    for plan in plans {
        let _ = writeln!(out, "pub use {}::{};", plan.module, plan.type_name);
    }

    let example = plans
        .iter()
        .find(|p| p.type_name == "Patient")
        .or_else(|| plans.first())
        .map(|p| p.type_name.clone())
        .unwrap_or_default();

    let _ = write!(
        out,
        "\n/// Any FHIR {label} resource, tagged by its `resourceType`.\n\
         ///\n\
         /// Used wherever a resource of any type may appear — for example a\n\
         /// `Bundle.entry.resource` or a `contained` resource. Serde reads and writes\n\
         /// the `resourceType` discriminator automatically.\n\
         ///\n\
         /// # Examples\n\
         ///\n\
         /// ```\n\
         /// use fhir::{module}::resources::Resource;\n\
         ///\n\
         /// let json = ::serde_json::json!({{\"resourceType\": \"{example}\"}});\n\
         /// let resource: Resource = ::serde_json::from_value(json).unwrap();\n\
         /// assert!(matches!(resource, Resource::{example}(_)));\n\
         /// ```\n\
         #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]\n\
         #[serde(tag = \"resourceType\")]\n"
    );
    out.push_str(&render::version_attribute(version));
    out.push_str("pub enum Resource {\n");
    for plan in plans {
        let _ = writeln!(
            out,
            "    {}(Box<{}::{}>),",
            plan.type_name, plan.module, plan.type_name
        );
    }
    out.push_str("}\n");

    // Accessors the `dom-2`/`dom-4` invariant checks need over typed
    // `contained` entries (T47). Only DomainResources carry `contained`;
    // every resource carries `meta`.
    let _ = write!(
        out,
        "\nimpl Resource {{\n\
         \x20   /// Whether this resource itself carries contained resources (`dom-2`).\n\
         \x20   #[must_use]\n\
         \x20   pub fn has_contained(&self) -> bool {{\n\
         \x20       match self {{\n"
    );
    for plan in plans {
        let has_contained = plan
            .structs
            .iter()
            .find(|s| s.is_root)
            .is_some_and(|s| s.fields.iter().any(|f| f.ident == "contained"));
        let _ = if has_contained {
            writeln!(
                out,
                "            Self::{}(r) => !r.contained.is_empty(),",
                plan.type_name
            )
        } else {
            writeln!(out, "            Self::{}(_) => false,", plan.type_name)
        };
    }
    let _ = write!(
        out,
        "        }}\n\
         \x20   }}\n\
         \n\
         \x20   /// The resource's `meta`, when present (`dom-4`).\n\
         \x20   #[must_use]\n\
         \x20   pub fn meta(&self) -> Option<&crate::{module}::types::Meta> {{\n\
         \x20       match self {{\n"
    );
    for plan in plans {
        let _ = writeln!(
            out,
            "            Self::{}(r) => r.meta.as_ref(),",
            plan.type_name
        );
    }
    let _ = write!(
        out,
        "        }}\n\
         \x20   }}\n\
         }}\n"
    );

    out.push_str(&render_reference_targets(plans, module));
    out
}

/// The compile-time reference-target markers (T11): `Reference<Patient>`
/// names this module's `Patient`. One impl per resource, emitted beside the
/// enum because this is the one place that knows every type.
fn render_reference_targets(plans: &[plan::TypePlan], module: &str) -> String {
    let mut out = String::from(
        "\n// The typed-reference target markers (T11): `types::Reference<Patient>`\n\
         // points at this module's `Patient`. See `types::reference::ResourceType`.\n",
    );
    for plan in plans {
        let _ = write!(
            out,
            "impl crate::{module}::types::reference::ResourceType for {}::{} {{\n\
             \x20   fn resource_type_name() -> Option<&'static str> {{\n\
             \x20       Some(\"{}\")\n\
             \x20   }}\n\
             }}\n",
            plan.module, plan.type_name, plan.type_name
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_module_declares_and_reexports_each_datatype() {
        let plans = vec![];
        let out = render_types_module(&["string".to_string()], &plans, Version::R4);
        assert!(out.contains("pub mod string;"));
        assert!(out.contains("pub use string::String;"));
        assert!(out.contains("//! FHIR R4 datatypes."));
        assert!(out.contains("use fhir::r4::types::Code;"));
    }

    #[test]
    fn resources_module_builds_the_tagged_enum() {
        let sd: spec::StructureDefinition = ::serde_json::from_value(::serde_json::json!({
            "name": "Patient", "type": "Patient", "kind": "resource",
            "url": "u", "version": "4.0.1",
            "snapshot": { "element": [{ "path": "Patient" }] }
        }))
        .unwrap();
        let plans = vec![plan::plan_type(&sd, &plan::Context::default()).unwrap()];
        let out = render_resources_module(&plans, Version::R4);
        assert!(out.contains("pub mod patient;"));
        assert!(out.contains("pub use patient::Patient;"));
        assert!(out.contains("#[serde(tag = \"resourceType\")]"));
        assert!(out.contains("Patient(Box<patient::Patient>),"));
        assert!(out.contains("#[fhir_version(\"r4\")]"));
    }

    #[test]
    fn summary_reads_as_a_sentence() {
        let summary = Summary {
            version: Version::R4,
            primitives: 20,
            datatypes: 43,
            resources: 146,
            code_enums: 400,
            elements: 5000,
        };
        assert_eq!(
            summary.to_string(),
            "R4: 20 primitives, 43 datatypes, 146 resources, 400 code enums, 5000 elements"
        );
    }
}
