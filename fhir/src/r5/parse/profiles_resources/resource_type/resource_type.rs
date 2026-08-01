//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ResourceType {
    Bool(bool),
    String(String),
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::Bool(false)
    }
}
