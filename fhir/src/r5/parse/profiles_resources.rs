//! Parser for the FHIR R5 `profiles-resources.json` definitions bundle, which is the
//! official specification file that describes the base `Resource` type and its
//! structural building blocks (such as `Bundle`, `Element`, `Differential`, `Snapshot`,
//! and related `StructureDefinition` components). This module deserializes that JSON
//! bundle into the Rust types declared in its submodules, and the `resource_into_rust`
//! and `element_into_rust_struct_attribute` helpers then translate the parsed data into
//! generated Rust source for the crate's R5 data model. As one of the `src/r5/parse`
//! submodules, it plugs into the broader code-generation pipeline that turns the raw
//! FHIR specification files into the typed structs under `src/r5/types`.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("profiles_resources"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("profiles-resources.json"));

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

pub mod parameter {
    pub mod parameter;
}
pub use parameter::parameter::Parameter;

pub mod operation {
    pub mod operation;
}
pub use operation::operation::Operation;

pub mod resource {
    pub mod resource;
    pub mod resource_into_rust;
}
pub use resource::resource::Resource;
pub use resource::resource_into_rust::resource_into_rust;

pub mod resource_inner {
    pub mod resource_inner;
}
pub use resource_inner::resource_inner::ResourceInner;

pub mod resource_type {
    pub mod resource_type;
}
pub use resource_type::resource_type::ResourceType;

pub mod rest {
    pub mod rest;
}
pub use rest::rest::Rest;

pub mod rest_resource {
    pub mod rest_resource;
}
pub use rest_resource::rest_resource::RestResource;

pub mod interaction {
    pub mod interaction;
}
pub use interaction::interaction::Interaction;

pub mod rest_search_param {
    pub mod rest_search_param;
}
pub use rest_search_param::rest_search_param::RestSearchParam;

pub mod security {
    pub mod security;
}
pub use security::security::Security;

pub mod slicing {
    pub mod slicing;
}
pub use slicing::slicing::Slicing;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

pub mod software {
    pub mod software;
}
pub use software::software::Software;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::profiles_resources::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
