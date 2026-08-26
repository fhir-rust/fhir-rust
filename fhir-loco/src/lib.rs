//! `fhir-loco` — a FHIR® RESTful API server mounted over `fhir-sqlite`.
//!
//! # Trademarks
//!
//! HL7®, and FHIR® are the registered trademarks of Health Level Seven
//! International and their use of these trademarks does not constitute an
//! endorsement by HL7.
// Nothing here has any business dereferencing a raw pointer: this code
// parses and reshapes untrusted clinical data, and memory safety is the
// property that keeps a malformed resource from becoming a vulnerability.
#![forbid(unsafe_code)]

pub mod admin;
pub mod app;
pub mod auth;
pub mod controllers;
pub mod data;
pub mod initializers;
pub mod store;
pub mod tasks;
pub mod views;
