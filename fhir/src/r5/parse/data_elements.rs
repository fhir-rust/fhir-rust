//! Parses the FHIR R5 "Data Elements" specification bundle, loaded from
//! `definitions/dataelements.json`, which is a FHIR `Bundle` resource whose entries hold the
//! `StructureDefinition`s for reusable data elements. The submodules here (`Bundle`, `Entry`,
//! `Resource`, `Element`, `Snapshot`, `Differential`, `Slicing`, `Discriminator`, and `Base`)
//! mirror the JSON shape of that bundle and its nested structure definitions, with each type
//! deriving `serde::Deserialize` so the raw specification file can be loaded directly into
//! Rust values. This parsed representation is the input that the crate's code generator walks
//! in order to emit the strongly typed FHIR data-element model used elsewhere in the crate,
//! making this module one stage in the overall specification-to-Rust generation pipeline.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("data_elements"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("dataelements.json"));

// Submodules

pub mod base {
    pub mod base;
}
pub use base::base::Base;

pub mod bundle {
    pub mod bundle;
}
pub use bundle::bundle::Bundle;

pub mod differential {
    pub mod differential;
}
pub use differential::differential::Differential;

pub mod discriminator {
    pub mod discriminator;
}
pub use discriminator::discriminator::Discriminator;

pub mod element {
    pub mod element;
}
pub use element::element::Element;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod resource {
    pub mod resource;
}
pub use resource::resource::Resource;

pub mod slicing {
    pub mod slicing;
}
pub use slicing::slicing::Slicing;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::data_elements::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
