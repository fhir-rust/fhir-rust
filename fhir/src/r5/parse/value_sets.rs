//! Parser for the FHIR R5 `valuesets.json` specification bundle, i.e. the official
//! FHIR definitions file that describes `ValueSet` resources and the related
//! `Bundle`, `Compose`, `Concept`, `Expansion`, and other structures used to
//! encode terminology bindings. This module organizes the deserialization
//! target types for that bundle into one submodule per FHIR structure, each of
//! which mirrors the shape of the JSON so that `serde_json` can parse the
//! specification file directly into strongly typed Rust values. Those parsed
//! values are the intermediate representation consumed by the broader code
//! generation pipeline in `crate::r5::parse`, which reads the official FHIR
//! specification JSON files and emits the generated Rust source found under
//! `src/r5/types`. The `DIR` and `DEFINITIONS_FILE` statics locate the
//! submodule output directory and the source `valuesets.json` file
//! respectively, and are exercised by the accompanying test to confirm the
//! bundle can be loaded successfully.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("value_sets"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("valuesets.json"));

// Submodules

pub mod base {
    pub mod base;
}
pub use base::base::Base;

pub mod bundle {
    pub mod bundle;
}
pub use bundle::bundle::Bundle;

pub mod compose {
    pub mod compose;
}
pub use compose::compose::Compose;

pub mod concept {
    pub mod concept;
}
pub use concept::concept::Concept;

pub mod designation {
    pub mod designation;
}
pub use designation::designation::Designation;

pub mod differential {
    pub mod differential;
}
pub use differential::differential::Differential;

pub mod element {
    pub mod element;
}
pub use element::element::Element;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod expansion {
    pub mod expansion;
}
pub use expansion::expansion::Expansion;

pub mod parameter {
    pub mod parameter;
}
pub use parameter::parameter::Parameter;

pub mod resource {
    pub mod resource;
}
pub use resource::resource::Resource;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

pub mod use_context {
    pub mod use_context;
}
pub use use_context::use_context::UseContext;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::value_sets::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
