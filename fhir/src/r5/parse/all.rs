//! Parse the FHIR R5 `all.json` specification bundle.
//!
//! This module reads the official FHIR R5 definitions file `all.json`, located
//! under the crate's `doc/fhir-specifications/r5/fhir-definitions-json`
//! directory and referenced here via `DEFINITIONS_FILE`. That bundle aggregates
//! the shared structural building blocks used throughout the specification, such
//! as codings, codeable concepts, identifiers, periods, quantities, ranges,
//! extensions, and related metadata. For each of these building blocks this
//! module exposes a corresponding Rust type, defined in a dedicated submodule
//! and re-exported here for convenient access, for example `Coding`,
//! `CodeableConcept`, `Identifier`, `Period`, and `Quantity`. Each submodule
//! deserializes its slice of the `all.json` bundle into an intermediate Rust
//! struct and then renders that struct as formatted Rust source code, which the
//! generator writes out under the crate's tracked `tmp/out` directory for later
//! use, or hand-adaptation, as part of the `src/r5/types` datatype layer. Within
//! the wider generation pipeline this module is one of several parsers under
//! `crate::r5::parse`, alongside the parsers for concept maps, data elements,
//! profiles, search parameters, and value sets, that together turn the raw FHIR
//! JSON specifications into strongly typed Rust source code.

// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("all"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("all.json"));

// Submodules

pub mod additional {
    pub mod additional;
}
pub use additional::additional::Additional;

pub mod additional_attribute {
    pub mod additional_attribute;
}
pub use additional_attribute::additional_attribute::AdditionalAttribute;

pub mod binding {
    pub mod binding;
}
pub use binding::binding::Binding;

pub mod codeable_concept {
    pub mod codeable_concept;
}
pub use codeable_concept::codeable_concept::CodeableConcept;

pub mod coding {
    pub mod coding;
}
pub use coding::coding::Coding;

pub mod constraint {
    pub mod constraint;
}
pub use constraint::constraint::Constraint;

pub mod contact {
    pub mod contact;
}
pub use contact::contact::Contact;

pub mod depends_on {
    pub mod depends_on;
}
pub use depends_on::depends_on::DependsOn;

pub mod example {
    pub mod example;
}
pub use example::example::Example;

pub mod extension {
    pub mod extension;
}
pub use extension::extension::Extension;

pub mod element_type {
    pub mod element_type;
}
pub use element_type::element_type::ElementType;

pub mod identifier {
    pub mod identifier;
}
pub use identifier::identifier::Identifier;

pub mod jurisdiction {
    pub mod jurisdiction;
}
pub use jurisdiction::jurisdiction::Jurisdiction;

pub mod meta {
    pub mod meta;
}
pub use meta::meta::Meta;

pub mod period {
    pub mod period;
}
pub use period::period::Period;

pub mod property {
    pub mod property;
}
pub use property::property::Property;

pub mod quantity {
    pub mod quantity;
}
pub use quantity::quantity::Quantity;

pub mod range {
    pub mod range;
}
pub use range::range::Range;

pub mod related_artifact {
    pub mod related_artifact;
}
pub use related_artifact::related_artifact::RelatedArtifact;

pub mod target {
    pub mod target;
}
pub use target::target::Target;

pub mod topic {
    pub mod topic;
}
pub use topic::topic::Topic;

pub mod r#type {
    pub mod r#type;
}
pub use r#type::r#type::r#Type;

pub mod underscore_code {
    pub mod underscore_code;
}
pub use underscore_code::underscore_code::UnderscoreCode;
