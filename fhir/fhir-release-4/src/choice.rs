//! Support types for FHIR R4 `value[x]` choice elements.
//!
//! A choice element is modelled as a generated enum with one variant per allowed
//! type, with serde derived by `#[derive(fhir_derive_macros::FhirChoice)]`. See
//! `spec/11-choice-types.md`.
//!
//! [`Primitive<T>`] is defined per release rather than once, because the
//! extension it carries is that release's `Element`.

use crate::r4::types::Element;
use crate::validate::{Validate, ValidationIssue};

/// A primitive choice value together with its optional `_value<Type>` extension.
///
/// Primitive `value[x]` variants (e.g. `valueString`, `valueBoolean`) may carry
/// a paired `_value<Type>` sibling holding the primitive's `id`/`extension`; this
/// wrapper keeps the value and that extension together. Complex variants hold
/// their datatype directly and do not use `Primitive`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Primitive<T> {
    /// The primitive value.
    pub value: T,
    /// The paired `_value<Type>` extension element, if any.
    pub extension: Option<Element>,
}

impl<T> Primitive<T> {
    /// Wrap a bare value with no extension.
    pub fn new(value: T) -> Self {
        Self {
            value,
            extension: None,
        }
    }
}

impl<T: Validate> Validate for Primitive<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        let mut issues = self.value.validate();
        issues.extend(self.extension.validate());
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r4::types::{Boolean, Quantity, String as FhirString};
    use fhir_derive_macros::{FhirChoice, Validate};

    // A representative generated choice enum: complex + primitive variants.
    #[derive(Debug, Clone, PartialEq, FhirChoice, Validate)]
    #[fhir_version("r4")]
    #[allow(clippy::large_enum_variant)]
    enum TestValue {
        #[fhir("valueQuantity")]
        Quantity(Box<Quantity>),
        #[fhir("valueString")]
        String(Primitive<FhirString>),
        #[fhir("valueBoolean")]
        Boolean(Primitive<Boolean>),
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    struct Wrapper {
        status: FhirString,
        #[serde(flatten, default)]
        value: Option<TestValue>,
    }

    fn roundtrip(v: &::serde_json::Value) -> ::serde_json::Value {
        let w: Wrapper = ::serde_json::from_value(v.clone()).expect("deserialize");
        ::serde_json::to_value(&w).expect("serialize")
    }

    #[test]
    fn complex_variant_roundtrips() {
        let json = ::serde_json::json!({ "status": "final", "valueQuantity": { "value": 98.6 } });
        assert_eq!(roundtrip(&json), json);
    }

    #[test]
    fn primitive_variant_roundtrips() {
        let json = ::serde_json::json!({ "status": "final", "valueString": "positive" });
        assert_eq!(roundtrip(&json), json);
    }

    #[test]
    fn primitive_variant_with_extension_roundtrips() {
        let json = ::serde_json::json!({
            "status": "final",
            "valueString": "positive",
            "_valueString": { "extension": [{ "url": "http://x", "valueDecimal": 0.9 }] }
        });
        assert_eq!(roundtrip(&json), json);
    }

    #[test]
    fn absent_is_none_and_emits_one_key() {
        let json = ::serde_json::json!({ "status": "final" });
        let w: Wrapper = ::serde_json::from_value(json.clone()).unwrap();
        assert_eq!(w.value, None);
        assert_eq!(::serde_json::to_value(&w).unwrap(), json);

        let built = Wrapper {
            status: FhirString("final".to_string()),
            value: Some(TestValue::Boolean(Primitive::new(Boolean(true)))),
        };
        assert_eq!(
            ::serde_json::to_value(&built).unwrap(),
            ::serde_json::json!({ "status": "final", "valueBoolean": true })
        );
    }

    #[test]
    fn validate_runs_on_active_variant() {
        // The `Validate` derive dispatches to the active variant's data.
        let valid = TestValue::Boolean(Primitive::new(Boolean(true)));
        assert!(valid.validate().is_empty());

        // An invalid code primitive inside the active variant is reported.
        let invalid = TestValue::Quantity(Box::new(Quantity {
            code: Some(crate::r4::types::Code(String::new())),
            ..Default::default()
        }));
        assert!(!invalid.validate().is_empty());
    }
}
