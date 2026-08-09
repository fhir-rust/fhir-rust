//! fhir-postgresql-gen: generates the fhir-postgresql relational map from the official FHIR
//! specification packages (profiles-resources.json, profiles-types.json).

pub mod assets;
pub mod build;
pub mod names;
pub mod search;
pub mod spec;

use std::path::Path;

use fhir_postgresql_map::RelMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("spec: {0}")]
    Spec(String),
    #[error("build: {0}")]
    Build(String),
}

/// Generate the relational map for one FHIR version from a definitions
/// directory containing profiles-resources.json and profiles-types.json.
pub fn generate(definitions_dir: &Path, schema: &str) -> Result<RelMap, GenError> {
    let spec = spec::load_spec(definitions_dir)?;
    let mut map = build::build_map(&spec, schema)?;
    search::compile_search(&mut map, definitions_dir)?;
    search::add_norm_columns(&mut map);
    // U1/U9: after the fold columns, and only where the dialect needs them.
    search::add_adjunct_columns(&mut map);
    // U12a: last, so the bound is a fact of the finished map.
    build::record_path_bound(&mut map);
    Ok(map)
}
