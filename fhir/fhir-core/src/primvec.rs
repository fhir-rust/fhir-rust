//! The value array of a repeating FHIR primitive (audit **F-86**).
//!
//! FHIR JSON represents a repeating primitive as **parallel arrays**: the
//! value array, and an `_element` array carrying each position's
//! `id`/`extension`. A position that carries only an extension is a **null**
//! in the value array:
//!
//! ```json
//! { "event": [null], "_event": [{ "extension": [ … ] }] }
//! ```
//!
//! That is valid FHIR — HL7's own R4B examples use it — and `Vec<T>` cannot
//! hold it: there is no way to represent "no value at this position". Until
//! 2026-08-10 the model rejected the null outright (and, before **F-87**'s
//! fix the same day, then silently dropped the surrounding element).
//! [`PrimVec`] is the value array as the wire defines it: a sequence of
//! positions, each a value or an extension-only placeholder.

use serde::{Deserialize, Serialize};

use crate::validate::{Validate, ValidationIssue};

/// The values of a repeating FHIR primitive element (`0..*`).
///
/// A thin, transparent wrapper over `Vec<Option<T>>`: `None` is an
/// extension-only placeholder — the JSON `null` whose position in the
/// paired `_element` array carries the extension. Use [`values`] to iterate
/// the actual values, [`iter`] to see positions.
///
/// A placeholder with no corresponding `_element` entry is meaningless in
/// FHIR; nothing here prevents constructing one, but the serializer will
/// faithfully write the `null`, and validation flags it
/// ([`Validate for PrimVec`](#impl-Validate-for-PrimVec<T>)).
///
/// [`values`]: PrimVec::values
/// [`iter`]: PrimVec::iter
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrimVec<T>(pub Vec<Option<T>>);

impl<T> PrimVec<T> {
    /// An empty value array.
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Number of positions, placeholders included.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// True when there are no positions at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The values, skipping extension-only placeholders.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter().filter_map(Option::as_ref)
    }

    /// Every position: `Some(value)` or `None` for a placeholder.
    pub fn iter(&self) -> std::slice::Iter<'_, Option<T>> {
        self.0.iter()
    }

    /// Append a value.
    pub fn push(&mut self, value: T) {
        self.0.push(Some(value));
    }

    /// Append an extension-only placeholder (a JSON `null`; its extension
    /// lives at the same index of the `_element` sibling field).
    pub fn push_placeholder(&mut self) {
        self.0.push(None);
    }

    /// The first actual value, if any position holds one.
    #[must_use]
    pub fn first_value(&self) -> Option<&T> {
        self.values().next()
    }
}

impl<T> From<Vec<T>> for PrimVec<T> {
    fn from(values: Vec<T>) -> Self {
        Self(values.into_iter().map(Some).collect())
    }
}

impl<T> FromIterator<T> for PrimVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().map(Some).collect())
    }
}

impl<T> FromIterator<Option<T>> for PrimVec<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for PrimVec<T> {
    type Item = Option<T>;
    type IntoIter = std::vec::IntoIter<Option<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PrimVec<T> {
    type Item = &'a Option<T>;
    type IntoIter = std::slice::Iter<'a, Option<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: Validate> Validate for PrimVec<T> {
    fn validate(&self) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        for v in self.values() {
            issues.extend(v.validate());
        }
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_positions_round_trip() {
        // The F-86 shape: a value, then an extension-only placeholder.
        let json = ::serde_json::json!(["a", null]);
        let v: PrimVec<String> = ::serde_json::from_value(json.clone()).expect("nulls parse");
        assert_eq!(v.len(), 2);
        assert_eq!(v.values().collect::<Vec<_>>(), ["a"]);
        assert_eq!(::serde_json::to_value(&v).expect("ser"), json);
    }

    #[test]
    fn plain_arrays_are_unchanged() {
        let json = ::serde_json::json!(["a", "b"]);
        let v: PrimVec<String> = ::serde_json::from_value(json.clone()).expect("parse");
        assert_eq!(v.values().count(), 2);
        assert_eq!(::serde_json::to_value(&v).expect("ser"), json);
    }

    #[test]
    fn construction_from_plain_values() {
        let v: PrimVec<i32> = vec![1, 2].into();
        assert_eq!(v.len(), 2);
        let w: PrimVec<i32> = [1, 2].into_iter().collect();
        assert_eq!(v, w);
    }
}
