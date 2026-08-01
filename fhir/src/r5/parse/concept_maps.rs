//! Code generator module for the FHIR R5 "concept maps" specification bundle.
//!
//! This module parses `conceptmaps.json`, the official FHIR R5 definitions file that
//! contains the `ConceptMap` example bundle, and drives generation of the corresponding
//! Rust source types. The JSON is deserialized into a `Bundle` of `Entry` values, each
//! wrapping a `Resource` (a `ConceptMap`) built from nested `Group`, `Element`, `Unmapped`,
//! and other structures declared in the submodules below. As part of the broader generation
//! pipeline under `src/r5/parse`, this module's output is used to emit or validate the
//! statically typed representations found under `src/r5/types` for concept maps.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("concept_maps"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("conceptmaps.json"));

// Submodules

pub mod base {
    pub mod base;
}
pub use base::base::Base;

pub mod bundle {
    pub mod bundle;
}
pub use bundle::bundle::Bundle;

pub mod element {
    pub mod element;
}
pub use element::element::Element;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod group {
    pub mod group;
}
pub use group::group::Group;

pub mod resource {
    pub mod resource;
}
pub use resource::resource::Resource;

pub mod unmapped {
    pub mod unmapped;
}
pub use unmapped::unmapped::Unmapped;

pub mod use_context {
    pub mod use_context;
}
pub use use_context::use_context::UseContext;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::concept_maps::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
