//! Parser for the FHIR R5 `profiles-others.json` definitions bundle.
//!
//! This module reads the official FHIR specification file `profiles-others.json`
//! (located under the crate's definitions directory) into a typed [`Bundle`] of
//! [`Entry`] items, each wrapping a [`Resource`] whose differential and snapshot
//! elements describe structure definitions that do not belong to the core
//! resources or datatypes bundles. The submodules here mirror the JSON schema
//! of that bundle, including `Element`, `Slicing`, `Discriminator`, `Differential`,
//! and `Snapshot`, so the file can be deserialized faithfully via `serde_json`.
//! Once parsed, `resource_into_rust` and `element_into_rust_struct_attribute`
//! convert each resource's structure definition into generated Rust source
//! code, making this module one of the code-generation entry points that
//! turns the official FHIR specification JSON into the crate's Rust types.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("profiles_others"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("profiles-others.json"));

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
    pub mod element_into_rust_struct_attribute;
}
pub use element::element::Element;
pub use element::element_into_rust_struct_attribute::element_into_rust_struct_attribute;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod resource {
    pub mod resource;
    pub mod resource_into_rust;
}
pub use resource::resource::Resource;
pub use resource::resource_into_rust::resource_into_rust;

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
    type T = crate::r5::parse::profiles_others::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
