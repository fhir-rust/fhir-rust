//! Utilities for working with R4 `Bundle`s: iterating the contained resources,
//! following `next`-link paging, and building transaction/batch bundles.
//!
//! ```
//! use fhir::r4::resources::{Bundle, Patient};
//!
//! let bundle: Bundle = serde_json::from_value(serde_json::json!({
//!     "resourceType": "Bundle",
//!     "type": "collection",
//!     "entry": [
//!         { "resource": { "resourceType": "Patient", "id": "a" } },
//!         { "resource": { "resourceType": "Observation", "id": "o", "status": "final",
//!                         "code": {} } }
//!     ]
//! })).unwrap();
//!
//! // Every entry resource, parsed into the polymorphic `Resource` enum.
//! assert_eq!(bundle.iter_resources().count(), 2);
//! // Just the Patients, typed.
//! let patients: Vec<Patient> = bundle.resources::<Patient>("Patient").collect();
//! assert_eq!(patients.len(), 1);
//! assert_eq!(patients[0].id.as_ref().unwrap().0, "a");
//! ```

use ::serde::Serialize;
use ::serde::de::DeserializeOwned;

use crate::r4::coded::Coded;
use crate::r4::codes::{BundleType, HttpVerb};
use crate::r4::resources::Resource;
use crate::r4::resources::bundle::{Bundle, BundleEntry, BundleEntryRequest};
use crate::r4::types;

impl Bundle {
    /// Iterate every entry's resource, parsed into the polymorphic [`Resource`]
    /// enum. Entries without a resource, or that fail to parse, are skipped.
    pub fn iter_resources(&self) -> impl Iterator<Item = Resource> + '_ {
        self.entry
            .iter()
            .filter_map(|e| e.resource.as_ref())
            .filter_map(|v| ::serde_json::from_value(v.clone()).ok())
    }

    /// Iterate the entries whose `resourceType` is `resource_type`, parsed into
    /// `T` (e.g. `bundle.resources::<Patient>("Patient")`).
    pub fn resources<'a, T: DeserializeOwned + 'a>(
        &'a self,
        resource_type: &'a str,
    ) -> impl Iterator<Item = T> + 'a {
        self.entry
            .iter()
            .filter_map(|e| e.resource.as_ref())
            .filter(move |v| v.get("resourceType").and_then(|t| t.as_str()) == Some(resource_type))
            .filter_map(|v| ::serde_json::from_value(v.clone()).ok())
    }

    /// The URL of the `next` paging link, if present (relation `"next"`).
    ///
    /// R4 types `Bundle.link.relation` as a plain `string`; R5 later bound it
    /// to a value set, which is why this reads the raw value rather than a code.
    #[must_use]
    pub fn next_link(&self) -> Option<&str> {
        self.link
            .iter()
            .find(|l| l.relation.0 == "next")
            .map(|l| l.url.0.as_str())
    }

    /// Start building a `transaction` bundle. Use [`create`](TransactionBuilder::create),
    /// [`update`](TransactionBuilder::update), and [`delete`](TransactionBuilder::delete),
    /// then [`build`](TransactionBuilder::build).
    #[must_use]
    pub fn transaction() -> TransactionBuilder {
        TransactionBuilder {
            bundle_type: BundleType::Transaction,
            entries: Vec::new(),
        }
    }

    /// Start building a `batch` bundle (same API as [`transaction`](Self::transaction)).
    #[must_use]
    pub fn batch() -> TransactionBuilder {
        TransactionBuilder {
            bundle_type: BundleType::Batch,
            entries: Vec::new(),
        }
    }
}

/// Builder for a transaction or batch `Bundle` (see [`Bundle::transaction`]).
pub struct TransactionBuilder {
    bundle_type: BundleType,
    entries: Vec<BundleEntry>,
}

impl TransactionBuilder {
    fn push(&mut self, method: HttpVerb, url: &str, resource: Option<::serde_json::Value>) {
        self.entries.push(BundleEntry {
            resource,
            request: Some(BundleEntryRequest {
                method: Coded::Known(method),
                url: types::Uri(url.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    /// Add a `POST` (create) entry: the resource is created at `resource_type`.
    #[must_use]
    pub fn create<T: Serialize>(mut self, resource: &T, resource_type: &str) -> Self {
        let value = serialize_entry(resource, resource_type);
        self.push(HttpVerb::Post, resource_type, value);
        self
    }

    /// Add a `PUT` (update) entry at `url` (e.g. `"Patient/123"`).
    #[must_use]
    pub fn update<T: Serialize>(mut self, resource: &T, url: &str) -> Self {
        // The type is the first segment of the URL: `Patient/123` is a Patient.
        let resource_type = url.split('/').next().unwrap_or_default();
        let value = serialize_entry(resource, resource_type);
        self.push(HttpVerb::Put, url, value);
        self
    }

    /// Add a `DELETE` entry at `url` (e.g. `"Patient/123"`).
    #[must_use]
    pub fn delete(mut self, url: &str) -> Self {
        self.push(HttpVerb::Delete, url, None);
        self
    }

    /// Finish building the bundle.
    #[must_use]
    pub fn build(self) -> Bundle {
        Bundle {
            r#type: Coded::Known(self.bundle_type),
            entry: self.entries,
            ..Default::default()
        }
    }
}

/// Serialize a resource for a bundle entry, ensuring it carries its
/// `resourceType`.
///
/// A bare resource struct does not serialize one — the polymorphic `Resource`
/// enum is what adds the discriminator — but a bundle entry without it is not a
/// valid FHIR resource, and cannot be read back through `Resource`. The caller
/// has already told us the type, so fill it in.
fn serialize_entry<T: Serialize>(resource: &T, resource_type: &str) -> Option<::serde_json::Value> {
    let mut value = ::serde_json::to_value(resource).ok()?;
    if let Some(map) = value.as_object_mut() {
        map.entry("resourceType".to_string())
            .or_insert_with(|| ::serde_json::Value::String(resource_type.to_string()));
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4::resources::Patient;

    #[test]
    fn transaction_builder_produces_entries() {
        let patient = Patient {
            id: Some(types::String("p1".to_string())),
            ..Default::default()
        };
        let bundle = Bundle::transaction()
            .create(&patient, "Patient")
            .delete("Patient/old")
            .build();

        assert!(matches!(
            bundle.r#type,
            Coded::Known(BundleType::Transaction)
        ));
        let entries = &bundle.entry;
        assert_eq!(entries.len(), 2);
        assert!(matches!(
            entries[0].request.as_ref().unwrap().method,
            Coded::Known(HttpVerb::Post)
        ));
        assert_eq!(entries[1].request.as_ref().unwrap().url.0, "Patient/old");
        assert!(entries[1].resource.is_none());
    }

    #[test]
    fn entries_carry_their_resource_type() {
        // Without `resourceType` a bundle entry is not a valid FHIR resource,
        // and reading it back through the `Resource` enum silently yields
        // nothing.
        let patient = Patient {
            id: Some(types::String("p1".to_string())),
            ..Default::default()
        };
        let bundle = Bundle::transaction()
            .create(&patient, "Patient")
            .update(&patient, "Patient/p1")
            .build();

        for entry in &bundle.entry {
            let value = entry.resource.as_ref().expect("entry has a resource");
            assert_eq!(
                value.get("resourceType").and_then(|v| v.as_str()),
                Some("Patient")
            );
        }
        assert_eq!(bundle.iter_resources().count(), 2);
        assert_eq!(bundle.resources::<Patient>("Patient").count(), 2);
    }

    #[test]
    fn next_link_paging() {
        let bundle: Bundle = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Bundle",
            "type": "searchset",
            "link": [{ "relation": "next", "url": "http://x/page2" }]
        }))
        .unwrap();
        assert_eq!(bundle.next_link(), Some("http://x/page2"));
    }
}
