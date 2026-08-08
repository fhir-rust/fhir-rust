//! The shape of the per-element metadata table, shared by every release.
//!
//! Each release generates its own table of [`ElementMeta`] — the facts the
//! specification states about an element that the Rust types cannot carry, such
//! as whether a repeating field was `0..*` or `1..*`, which value set a code is
//! bound to, and which resources a `Reference` may point at. The *types* in
//! that table, and the lookups over it, do not vary by release, so they are
//! defined once here and used by [`r4::meta`](crate::r4::meta) and
//! [`r5::meta`](crate::r5::meta).
//!
//! ```
//! use fhir::r5::meta;
//!
//! let gender = meta::element("Patient.gender").unwrap();
//! assert_eq!(gender.binding.unwrap().strength, fhir::meta::BindingStrength::Required);
//! ```

use std::collections::HashMap;

/// Binding strength for a coded element
/// (`ElementDefinition.binding.strength`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingStrength {
    /// The value must come from the bound value set.
    Required,
    /// Codes from the value set should be used; others allowed if none fit.
    Extensible,
    /// The value set is a suggestion.
    Preferred,
    /// The value set is illustrative only.
    Example,
}

impl BindingStrength {
    /// Parse a FHIR strength token (`"required"`, …); unknown tokens map to
    /// [`Example`](Self::Example).
    #[must_use]
    pub fn from_token(token: &str) -> Self {
        match token {
            "required" => Self::Required,
            "extensible" => Self::Extensible,
            "preferred" => Self::Preferred,
            _ => Self::Example,
        }
    }
}

/// A value-set binding on a coded element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingMeta {
    /// How strictly the value set applies.
    pub strength: BindingStrength,
    /// Canonical `ValueSet` URL (may carry a `|version` suffix), if declared.
    pub value_set: Option<&'static str>,
}

/// One allowed type for an element (an entry of `ElementDefinition.type`).
///
/// A `value[x]` choice element has one `TypeRef` per allowed type; a reference
/// element carries its allowed target resource profiles.
#[derive(Debug, Clone, Copy)]
pub struct TypeRef {
    /// FHIR type code, e.g. `"Quantity"`, `"string"`, `"Reference"`.
    pub code: &'static str,
    /// For `Reference`/`canonical` types, the allowed target resource profiles
    /// as canonical URLs; empty otherwise.
    pub target_profiles: &'static [&'static str],
}

impl TypeRef {
    /// The bare target resource names (final path segment of each profile URL).
    ///
    /// ```
    /// use fhir::r5::meta;
    /// let subject = meta::element("Observation.subject").unwrap();
    /// let targets: Vec<_> = subject.types[0].target_names().collect();
    /// assert!(targets.contains(&"Patient"));
    /// ```
    pub fn target_names(&self) -> impl Iterator<Item = &'static str> {
        self.target_profiles
            .iter()
            .map(|url| url.rsplit(['/', '#']).next().unwrap_or(url))
    }
}

/// Metadata for one element of a FHIR resource or datatype, keyed by its full
/// `ElementDefinition` path.
#[derive(Debug, Clone, Copy)]
pub struct ElementMeta {
    /// Full FHIR path, e.g. `"Patient.gender"` or `"Observation.value[x]"`.
    pub path: &'static str,
    /// Minimum cardinality.
    pub min: u32,
    /// Maximum cardinality as the raw FHIR token: `"0"`, `"1"`, `"*"`, or a
    /// number.
    pub max: &'static str,
    /// Whether the element is part of the summary view
    /// (`ElementDefinition.isSummary`).
    pub is_summary: bool,
    /// Coded-value binding, if any.
    pub binding: Option<BindingMeta>,
    /// Allowed types; more than one for a `value[x]` choice element.
    pub types: &'static [TypeRef],
    /// The element path whose content defines this one, for the recursive
    /// backbones FHIR expresses with `contentReference`.
    ///
    /// `Questionnaire.item.item` does not restate an item's elements; it points
    /// at `Questionnaire.item`. The target is not always an ancestor —
    /// `TestScript.test.action.operation` refers to
    /// `TestScript.setup.action.operation` — so it cannot be recovered from the
    /// path and has to be carried here.
    pub content_reference: Option<&'static str>,
}

impl ElementMeta {
    /// Whether the element is mandatory (minimum cardinality ≥ 1).
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.min >= 1
    }

    /// Whether the element repeats (maximum cardinality greater than one).
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        self.max != "0" && self.max != "1"
    }

    /// Whether the element is a `value[x]`-style choice element.
    #[must_use]
    pub fn is_choice(&self) -> bool {
        self.path.ends_with("[x]")
    }

    /// The FHIR type codes allowed for this element.
    pub fn type_codes(&self) -> impl Iterator<Item = &'static str> {
        self.types.iter().map(|t| t.code)
    }
}

/// Look up an element by full FHIR path in a release's table.
///
/// The table is generated sorted by path, so this is a binary search.
#[must_use]
pub fn find(table: &'static [ElementMeta], path: &str) -> Option<&'static ElementMeta> {
    table
        .binary_search_by(|e| e.path.cmp(path))
        .ok()
        .map(|i| &table[i])
}

/// Look up an element, resolving a `value[x]` choice key to its choice element.
///
/// `path` is the literal path being looked up (`"Observation.valueQuantity"`),
/// `context` the path or datatype name the element sits in (`"Observation"`),
/// and `name` the JSON/XML key (`"valueQuantity"`). A direct hit wins; failing
/// that, the choice element whose base name prefixes `name` at a type-name
/// boundary is returned, so `valueQuantity` resolves to `Observation.value[x]`.
///
/// ```
/// use fhir_core::meta;
///
/// let table = fhir::r5::meta::elements();
/// let el = meta::resolve(table, "Observation.valueQuantity", "Observation", "valueQuantity").unwrap();
/// assert_eq!(el.path, "Observation.value[x]");
/// ```
#[must_use]
pub fn resolve(
    table: &'static [ElementMeta],
    path: &str,
    context: &str,
    name: &str,
) -> Option<&'static ElementMeta> {
    if let Some(e) = find(table, path) {
        return Some(e);
    }
    let prefix = format!("{context}.");
    table
        .iter()
        .filter(|e| e.path.starts_with(&prefix))
        .find(|e| {
            e.path.ends_with("[x]") && {
                let base = &e.path[context.len() + 1..e.path.len() - 3];
                name.len() > base.len()
                    && name.starts_with(base)
                    && name[base.len()..]
                        .chars()
                        .next()
                        .is_some_and(char::is_uppercase)
            }
        })
}

/// The type-name suffix of a choice key, e.g. `"Quantity"` for `valueQuantity`
/// against the choice element `Observation.value[x]`.
///
/// Returns `None` if `name` is not a variant of `choice`.
#[must_use]
pub fn choice_suffix<'a>(choice: &ElementMeta, name: &'a str) -> Option<&'a str> {
    let base = choice.path.rsplit('.').next()?.strip_suffix("[x]")?;
    let rest = name.strip_prefix(base)?;
    rest.chars()
        .next()
        .is_some_and(char::is_uppercase)
        .then_some(rest)
}

/// The JSON shape a FHIR type code takes on the wire.
///
/// FHIR's primitives do not all map to JSON strings: `integer` and `decimal`
/// are JSON numbers, `boolean` is a JSON boolean, and — the case that catches
/// people — `integer64` is a *string*, so 64-bit values survive parsers whose
/// numbers are doubles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonKind {
    /// A JSON string.
    String,
    /// A JSON number.
    Number,
    /// A JSON boolean.
    Boolean,
    /// A JSON object: a complex datatype or backbone element.
    Complex,
}

/// The JSON shape a FHIR type code takes on the wire.
///
/// ```
/// use fhir::meta::{json_kind, JsonKind};
///
/// assert_eq!(json_kind("decimal"), JsonKind::Number);
/// assert_eq!(json_kind("Quantity"), JsonKind::Complex);
/// // `integer64` is a string in FHIR JSON, deliberately.
/// assert_eq!(json_kind("integer64"), JsonKind::String);
/// ```
#[must_use]
pub fn json_kind(code: &str) -> JsonKind {
    match code {
        "integer" | "decimal" | "positiveInt" | "unsignedInt" => JsonKind::Number,
        "boolean" => JsonKind::Boolean,
        _ if is_datatype(code) => JsonKind::Complex,
        _ => JsonKind::String,
    }
}

/// Whether a type code names a complex datatype, as opposed to a primitive
/// (lowercase) or a backbone element.
///
/// Traversal uses this to decide whether a child's metadata lives under the
/// named datatype (`"HumanName.given"`) or stays on the path
/// (`"Patient.contact.name"`).
#[must_use]
pub fn is_datatype(code: &str) -> bool {
    !code.is_empty()
        && code.chars().next().is_some_and(char::is_uppercase)
        && code != "BackboneElement"
        && code != "Element"
}

/// Map every generated struct name to the FHIR path prefix it represents, e.g.
/// `"AppointmentParticipant"` to `"Appointment.participant"`.
///
/// Backbone struct names are the PascalCase concatenation of their path
/// segments, which is not reversible on its own — `PatientContact` could split
/// in several places — so the mapping is built from the paths that actually
/// exist.
#[must_use]
pub fn struct_prefixes(table: &'static [ElementMeta]) -> HashMap<String, &'static str> {
    use ::convert_case::{Case, Casing};

    let mut map = HashMap::new();
    for e in table {
        let seg_count = e.path.split('.').count();
        for take in 1..seg_count {
            let name: String = e
                .path
                .split('.')
                .take(take)
                .map(|s| s.to_case(Case::Pascal))
                .collect();
            if let Some((end, _)) = e.path.match_indices('.').nth(take - 1) {
                map.entry(name).or_insert(&e.path[..end]);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    static TABLE: &[ElementMeta] = &[
        ElementMeta {
            path: "Patient.active",
            min: 0,
            max: "1",
            is_summary: true,
            binding: None,
            types: &[TypeRef {
                code: "boolean",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.contact",
            min: 0,
            max: "*",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "BackboneElement",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.contact.name",
            min: 0,
            max: "1",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "HumanName",
                target_profiles: &[],
            }],
            content_reference: None,
        },
        ElementMeta {
            path: "Patient.link.other",
            min: 1,
            max: "1",
            is_summary: false,
            binding: None,
            types: &[TypeRef {
                code: "Reference",
                target_profiles: &["http://hl7.org/fhir/StructureDefinition/Patient"],
            }],
            content_reference: None,
        },
    ];

    #[test]
    fn lookup_by_path() {
        assert_eq!(find(TABLE, "Patient.active").unwrap().max, "1");
        assert!(find(TABLE, "Patient.nope").is_none());
    }

    #[test]
    fn cardinality_helpers() {
        let active = find(TABLE, "Patient.active").unwrap();
        assert!(!active.is_required());
        assert!(!active.is_multiple());
        let contact = find(TABLE, "Patient.contact").unwrap();
        assert!(contact.is_multiple());
        assert!(find(TABLE, "Patient.link.other").unwrap().is_required());
    }

    #[test]
    fn target_names_strip_the_profile_url() {
        let other = find(TABLE, "Patient.link.other").unwrap();
        let names: Vec<&str> = other.types[0].target_names().collect();
        assert_eq!(names, ["Patient"]);
    }

    #[test]
    fn struct_names_map_back_to_paths() {
        let prefixes = struct_prefixes(TABLE);
        assert_eq!(prefixes.get("Patient").copied(), Some("Patient"));
        assert_eq!(
            prefixes.get("PatientContact").copied(),
            Some("Patient.contact")
        );
    }

    #[test]
    fn strength_tokens_parse() {
        assert_eq!(
            BindingStrength::from_token("required"),
            BindingStrength::Required
        );
        assert_eq!(
            BindingStrength::from_token("nonsense"),
            BindingStrength::Example
        );
    }
}
