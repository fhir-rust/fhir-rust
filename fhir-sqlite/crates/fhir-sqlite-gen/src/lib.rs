//! fhir-sqlite-gen: generates the fhir-sqlite relational map from the official FHIR
//! specification packages (profiles-resources.json, profiles-types.json).

pub mod build;
pub mod names;
pub mod search;
pub mod spec;

use std::path::Path;

use fhir_sqlite_map::RelMap;
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
    Ok(map)
}
