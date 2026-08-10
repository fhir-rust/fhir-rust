//! Abstract classes a.k.a. abstract types
//!
//! <https://build.fhir.org/uml.html#abstract>
//!
//! 2.1.6.1.2 Abstract Classes
//!
//! Some resources and types are labeled as abstract. Such classes are never
//! instantiated with being specialized. This is the usual meaning of an
//! abstract class in UML. In the diagrams, a class that is abstract has the
//! class name in italics. This module includes all the abstract types.

//// Elements
pub mod backbone_element;
pub mod element;

//// Types
pub mod data_type;

//// Resources
pub mod canonical_resource;
pub mod domain_resource;
pub mod metadata_resource;

//// Todo
pub mod primitive_type;
