//! FHIR R5 datatypes.
//!
//! This module contains every FHIR R5 datatype: the **complex** datatypes
//! (structs such as [`Period`], [`HumanName`], [`CodeableConcept`], [`Coding`])
//! and the **primitive** datatypes (transparent newtypes such as [`Code`],
//! [`Id`], [`DateTime`], [`Boolean`], [`Integer64`]).
//!
//! Each datatype derives `serde::Serialize` and `serde::Deserialize` and
//! round-trips to and from the canonical FHIR JSON representation. Primitives
//! serialize *transparently* to their underlying JSON value:
//!
//! ```
//! use fhir::r5::types::Code;
//!
//! assert_eq!(serde_json::to_value(Code("final".to_string())).unwrap(), "final");
//! ```
//!
//! Every datatype is re-exported at this module's root, so you can write
//! `fhir::r5::types::Period` rather than `fhir::r5::types::period::Period`.
//!
//! ## Generating the datatype id lists
//!
//! To list the primitive datatype ids from the specification, run:
//!
//! ```sh
//! cat profiles-types.json |
//! jq -r '.entry | map(select(.resource.kind == "primitive-type")) | map(.resource.id)[]'
//! ```
//!
//! To generate the complex datatypes, run the following command:
//!
//! ```sh
//! cat profiles-types.json |
//! jq -r '.entry | map(select(.resource.kind == "complex-type")) | map(.resource.id)[]'
//! ```

pub mod address;
pub mod age;
pub mod annotation;
pub mod attachment;
pub mod availability;
pub mod backbone_element;
pub mod backbone_type;
pub mod base;
pub mod base_64_binary;
pub mod boolean;
pub mod canonical;
pub mod code;
pub mod codeable_concept;
pub mod codeable_reference;
pub mod coding;
pub mod contact_detail;
pub mod contact_point;
pub mod contributor;
pub mod count;
pub mod data_requirement;
pub mod data_type;
pub mod date;
pub mod date_time;
pub mod decimal;
pub mod distance;
pub mod dosage;
pub mod duration;
pub mod element;
pub mod element_definition;
pub mod expression;
pub mod extended_contact_detail;
pub mod extension;
pub mod human_name;
pub mod id;
pub mod identifier;
pub mod instant;
pub mod integer;
pub mod integer_64;
pub mod markdown;
pub mod marketing_status;
pub mod meta;
pub mod monetary_component;
pub mod money;
pub mod money_quantity;
pub mod narrative;
pub mod oid;
pub mod parameter_definition;
pub mod period;
pub mod positive_int;
pub mod primitive_type;
pub mod product_shelf_life;
pub mod quantity;
pub mod range;
pub mod ratio;
pub mod ratio_range;
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
pub mod url;
pub mod usage_context;
pub mod uuid;
pub mod virtual_service_detail;
pub mod xhtml;

/// Convenience
pub use address::Address;
pub use age::Age;
pub use annotation::Annotation;
pub use attachment::Attachment;
pub use availability::Availability;
pub use backbone_element::BackboneElement;
pub use backbone_type::BackboneType;
pub use base::Base;
pub use base_64_binary::Base64Binary;
pub use boolean::Boolean;
pub use canonical::Canonical;
pub use code::Code;
pub use codeable_concept::CodeableConcept;
pub use codeable_reference::CodeableReference;
pub use coding::Coding;
pub use contact_detail::ContactDetail;
pub use contact_point::ContactPoint;
pub use contributor::Contributor;
pub use count::Count;
pub use data_requirement::DataRequirement;
pub use data_type::DataType;
pub use date::Date;
pub use date_time::DateTime;
pub use decimal::Decimal;
pub use distance::Distance;
pub use dosage::Dosage;
pub use duration::Duration;
pub use element::Element;
pub use element_definition::ElementDefinition;
pub use expression::Expression;
pub use extended_contact_detail::ExtendedContactDetail;
pub use extension::Extension;
pub use human_name::HumanName;
pub use id::Id;
pub use identifier::Identifier;
pub use instant::Instant;
pub use integer::Integer;
pub use integer_64::Integer64;
pub use markdown::Markdown;
pub use marketing_status::MarketingStatus;
pub use meta::Meta;
pub use monetary_component::MonetaryComponent;
pub use money::Money;
pub use money_quantity::MoneyQuantity;
pub use narrative::Narrative;
pub use oid::Oid;
pub use parameter_definition::ParameterDefinition;
pub use period::Period;
pub use positive_int::PositiveInt;
pub use primitive_type::PrimitiveType;
pub use product_shelf_life::ProductShelfLife;
pub use quantity::Quantity;
pub use range::Range;
pub use ratio::Ratio;
pub use ratio_range::RatioRange;
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
pub use url::Url;
pub use usage_context::UsageContext;
pub use uuid::Uuid;
pub use virtual_service_detail::VirtualServiceDetail;
pub use xhtml::Xhtml;
