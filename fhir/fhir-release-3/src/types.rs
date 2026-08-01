//! FHIR R3 datatypes.
//!
//! This module contains every FHIR R3 datatype: the **complex** datatypes
//! (structs such as [`Period`], [`HumanName`], [`CodeableConcept`], [`Coding`])
//! and the **primitive** datatypes (transparent newtypes such as [`Code`],
//! [`Id`], [`DateTime`], [`Boolean`]).
//!
//! Each datatype derives `serde::Serialize` and `serde::Deserialize` and
//! round-trips to and from the canonical FHIR JSON representation. Primitives
//! serialize *transparently* to their underlying JSON value:
//!
//! ```
//! use fhir::r3::types::Code;
//!
//! assert_eq!(serde_json::to_value(Code("final".to_string())).unwrap(), "final");
//! ```
//!
//! Every datatype is re-exported at this module's root, so you can write
//! `fhir::r3::types::Period` rather than `fhir::r3::types::period::Period`.

pub mod address;
pub mod age;
pub mod annotation;
pub mod attachment;
pub mod backbone_element;
pub mod base_64_binary;
pub mod boolean;
pub mod code;
pub mod codeable_concept;
pub mod coding;
pub mod contact_detail;
pub mod contact_point;
pub mod contributor;
pub mod count;
pub mod data_requirement;
pub mod date;
pub mod date_time;
pub mod decimal;
pub mod distance;
pub mod dosage;
pub mod duration;
pub mod element;
pub mod element_definition;
pub mod extension;
pub mod human_name;
pub mod id;
pub mod identifier;
pub mod instant;
pub mod integer;
pub mod markdown;
pub mod meta;
pub mod money;
pub mod narrative;
pub mod oid;
pub mod parameter_definition;
pub mod period;
pub mod positive_int;
pub mod quantity;
pub mod range;
pub mod ratio;
pub mod reference;
pub mod related_artifact;
pub mod sampled_data;
pub mod signature;
pub mod simple_quantity;
pub mod string;
pub mod time;
pub mod timing;
pub mod trigger_definition;
pub mod unsigned_int;
pub mod uri;
pub mod usage_context;
pub mod uuid;
pub mod xhtml;

pub use address::Address;
pub use age::Age;
pub use annotation::Annotation;
pub use attachment::Attachment;
pub use backbone_element::BackboneElement;
pub use base_64_binary::Base64Binary;
pub use boolean::Boolean;
pub use code::Code;
pub use codeable_concept::CodeableConcept;
pub use coding::Coding;
pub use contact_detail::ContactDetail;
pub use contact_point::ContactPoint;
pub use contributor::Contributor;
pub use count::Count;
pub use data_requirement::DataRequirement;
pub use date::Date;
pub use date_time::DateTime;
pub use decimal::Decimal;
pub use distance::Distance;
pub use dosage::Dosage;
pub use duration::Duration;
pub use element::Element;
pub use element_definition::ElementDefinition;
pub use extension::Extension;
pub use human_name::HumanName;
pub use id::Id;
pub use identifier::Identifier;
pub use instant::Instant;
pub use integer::Integer;
pub use markdown::Markdown;
pub use meta::Meta;
pub use money::Money;
pub use narrative::Narrative;
pub use oid::Oid;
pub use parameter_definition::ParameterDefinition;
pub use period::Period;
pub use positive_int::PositiveInt;
pub use quantity::Quantity;
pub use range::Range;
pub use ratio::Ratio;
pub use reference::Reference;
pub use related_artifact::RelatedArtifact;
pub use sampled_data::SampledData;
pub use signature::Signature;
pub use simple_quantity::SimpleQuantity;
pub use string::String;
pub use time::Time;
pub use timing::Timing;
pub use trigger_definition::TriggerDefinition;
pub use unsigned_int::UnsignedInt;
pub use uri::Uri;
pub use usage_context::UsageContext;
pub use uuid::Uuid;
pub use xhtml::Xhtml;
