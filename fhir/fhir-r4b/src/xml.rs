//! FHIR R4B XML serialization (feature `xml`).
//!
//! FHIR XML differs from JSON: a primitive is `<field value="…"/>`, an element's
//! `id` (and an extension's `url`) are XML attributes, complex elements nest, and
//! repeating elements are simply repeated. The conversion itself does not vary
//! by release and lives in [`::fhir_core::xml`]; this module binds it to the R4B
//! [`meta`](crate::r4b::meta) table.
//!
//! ```
//! use fhir::r4b::resources::Patient;
//! use fhir::r4b::xml::{to_xml, from_xml};
//!
//! let patient: Patient = serde_json::from_value(serde_json::json!({
//!     "resourceType": "Patient", "id": "pat-1", "active": true,
//!     "name": [{ "family": "Chalmers", "given": ["Peter"] }]
//! })).unwrap();
//!
//! let xml = to_xml(&patient, "Patient");
//! assert!(xml.contains("<active value=\"true\"/>"));
//!
//! let back: Patient = from_xml(&xml).unwrap();
//! assert_eq!(back, patient);
//! ```

use ::serde::de::DeserializeOwned;
use ::serde_json::Value;

use crate::r4b::meta;

pub use ::fhir_core::xml::{XmlError, to_xml};

/// Parse a FHIR R4B XML string into a resource.
pub fn from_xml<T: DeserializeOwned>(xml: &str) -> Result<T, XmlError> {
    ::fhir_core::xml::from_xml(meta::elements(), xml)
}

/// Parse a FHIR R4B XML string into untyped JSON.
pub fn xml_to_value(xml: &str) -> Result<Value, XmlError> {
    ::fhir_core::xml::xml_to_value(meta::elements(), xml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4b::resources::{Observation, Patient};

    fn roundtrip_patient(json: ::serde_json::Value) {
        let patient: Patient = ::serde_json::from_value(json).unwrap();
        let xml = to_xml(&patient, "Patient");
        let back: Patient =
            from_xml(&xml).unwrap_or_else(|e| panic!("from_xml failed: {e}\n{xml}"));
        assert_eq!(back, patient, "\nXML was:\n{xml}");
    }

    #[test]
    fn patient_roundtrips() {
        roundtrip_patient(::serde_json::json!({
            "resourceType": "Patient",
            "id": "pat-1",
            "active": true,
            "gender": "male",
            "birthDate": "1970-03-25",
            "name": [{ "family": "Chalmers", "given": ["Peter", "James"] }],
            "telecom": [{ "system": "phone", "value": "555" }]
        }));
    }

    #[test]
    fn primitive_written_as_value_attribute() {
        let patient: Patient = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Patient", "active": true
        }))
        .unwrap();
        let xml = to_xml(&patient, "Patient");
        assert!(xml.contains("<active value=\"true\"/>"), "{xml}");
        assert!(xml.contains("xmlns=\"http://hl7.org/fhir\""), "{xml}");
    }

    #[test]
    fn choice_element_roundtrips() {
        // Observation.value[x] — a choice — plus a required 1..1 code.
        let obs: Observation = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "Observation",
            "status": "final",
            "code": { "text": "temp" },
            "valueString": "warm"
        }))
        .unwrap();
        let xml = to_xml(&obs, "Observation");
        let back: Observation = from_xml(&xml).unwrap_or_else(|e| panic!("{e}\n{xml}"));
        assert_eq!(back, obs, "\nXML was:\n{xml}");
    }
}
