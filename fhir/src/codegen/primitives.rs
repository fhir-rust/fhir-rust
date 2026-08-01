//! Generating the FHIR primitive datatypes.
//!
//! A primitive is the one part of the model the specification JSON cannot
//! describe on its own: it says `positiveInt` is "an integer with a value that
//! is positive", not that Rust should store it in a `u32`. That mapping is a
//! design decision, so it lives here as a table (see
//! `spec/02-primitive-types.md`) and the prose around it still comes from the
//! specification.
//!
//! Every primitive is a tuple newtype, which serializes transparently as its
//! inner value — `Code("final")` is the JSON string `"final"`, not an object.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::naming;
use super::render::doc_version;
use super::spec::StructureDefinition;
use super::version::Version;

/// How one FHIR primitive is represented in Rust.
#[derive(Debug, Clone, Copy)]
pub struct Primitive {
    /// The FHIR type code, e.g. `positiveInt`.
    pub code: &'static str,
    /// The Rust type inside the newtype, e.g. `u32`.
    pub inner: &'static str,
    /// What the inner value holds, for the field's doc comment.
    pub inner_doc: &'static str,
    /// The value `Default` produces, written out. A primitive whose default is
    /// not the inner type's own default (`decimal` is zero, not nothing) also
    /// needs a hand-written `impl Default`, which [`Primitive::needs_default_impl`]
    /// decides.
    pub default_value: &'static str,
    /// Whether the value is carried on the wire as a JSON string even though
    /// Rust stores a number (FHIR does this for `integer64`, to survive
    /// languages whose numbers are 64-bit floats).
    pub string_encoded: bool,
    /// A literal used in the round-trip unit test.
    pub sample: &'static str,
    /// The JSON that sample serializes to.
    pub sample_json: &'static str,
    /// Whether the type is hand-written once at the crate root and merely
    /// re-exported per release, rather than generated as a newtype. Only
    /// `decimal` is: its lexical precision needs custom serde (R2.2).
    pub shared_at_crate_root: bool,
}

/// The default of a string-backed primitive: the empty string.
const DEFAULT_STRING: &str = "std::string::String::new()";

/// The Rust representation of every FHIR primitive, across releases.
///
/// A release uses the entries its `profiles-types.json` actually defines, so
/// R4 simply never reaches `integer64`.
const PRIMITIVES: &[Primitive] = &[
    Primitive {
        code: "base64Binary",
        inner: "std::string::String",
        inner_doc: "The base64-encoded string representation of the binary content.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"YWJj\".to_string()",
        sample_json: "\"YWJj\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "boolean",
        inner: "bool",
        inner_doc: "The underlying `true` or `false` value.",
        default_value: "false",
        string_encoded: false,
        sample: "true",
        sample_json: "true",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "canonical",
        inner: "std::string::String",
        inner_doc: "The canonical URL, optionally with a `|version` suffix.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"http://example.org/vs\".to_string()",
        sample_json: "\"http://example.org/vs\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "code",
        inner: "std::string::String",
        inner_doc: "The code as it appears on the wire.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"final\".to_string()",
        sample_json: "\"final\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "date",
        inner: "std::string::String",
        inner_doc: "The date, at whatever precision it was written.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"2019-11-01\".to_string()",
        sample_json: "\"2019-11-01\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "dateTime",
        inner: "std::string::String",
        inner_doc: "The date and time, at whatever precision it was written.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"2019-11-01T09:29:23Z\".to_string()",
        sample_json: "\"2019-11-01T09:29:23Z\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "decimal",
        inner: "::fhir_core::decimal::Decimal",
        inner_doc: "The numeric value, preserved as the lexeme it was given so that significant trailing zeros survive a round trip.",
        default_value: "::fhir_core::decimal::Decimal::default()",
        string_encoded: false,
        sample: "::fhir_core::decimal::Decimal::default()",
        sample_json: "0",
        shared_at_crate_root: true,
    },
    Primitive {
        code: "id",
        inner: "std::string::String",
        inner_doc: "The logical id: up to 64 characters of `[A-Za-z0-9-.]`.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"pat-1\".to_string()",
        sample_json: "\"pat-1\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "instant",
        inner: "std::string::String",
        inner_doc: "The instant, always to at least second precision with a timezone.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"2019-11-01T09:29:23Z\".to_string()",
        sample_json: "\"2019-11-01T09:29:23Z\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "integer",
        inner: "i32",
        inner_doc: "The signed 32-bit value.",
        default_value: "0",
        string_encoded: false,
        sample: "42",
        sample_json: "42",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "integer64",
        inner: "i64",
        inner_doc: "The signed 64-bit value.",
        default_value: "0",
        string_encoded: true,
        sample: "9_000_000_000",
        sample_json: "\"9000000000\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "markdown",
        inner: "std::string::String",
        inner_doc: "The Markdown source text.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"# Heading\".to_string()",
        sample_json: "\"# Heading\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "oid",
        inner: "std::string::String",
        inner_doc: "The OID as a `urn:oid:` URI.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"urn:oid:1.2.3\".to_string()",
        sample_json: "\"urn:oid:1.2.3\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "positiveInt",
        inner: "u32",
        inner_doc: "The value, which the specification constrains to be greater than zero.",
        default_value: "0",
        string_encoded: false,
        sample: "1",
        sample_json: "1",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "string",
        inner: "std::string::String",
        inner_doc: "The underlying Unicode string value.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"abc\".to_string()",
        sample_json: "\"abc\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "time",
        inner: "std::string::String",
        inner_doc: "The time of day, without a date or timezone.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"09:29:23\".to_string()",
        sample_json: "\"09:29:23\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "unsignedInt",
        inner: "u32",
        inner_doc: "The value, which the specification constrains to be zero or greater.",
        default_value: "0",
        string_encoded: false,
        sample: "0",
        sample_json: "0",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "uri",
        inner: "std::string::String",
        inner_doc: "The URI, which may be absolute or relative.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"http://example.org\".to_string()",
        sample_json: "\"http://example.org\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "url",
        inner: "std::string::String",
        inner_doc: "The URL.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"http://example.org\".to_string()",
        sample_json: "\"http://example.org\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "uuid",
        inner: "std::string::String",
        inner_doc: "The UUID as a `urn:uuid:` URI.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"urn:uuid:c757873d-ec9a-4326-a141-556f43239520\".to_string()",
        sample_json: "\"urn:uuid:c757873d-ec9a-4326-a141-556f43239520\"",
        shared_at_crate_root: false,
    },
    Primitive {
        code: "xhtml",
        inner: "std::string::String",
        inner_doc: "The XHTML fragment, which must be a well-formed `<div>`.",
        default_value: DEFAULT_STRING,
        string_encoded: false,
        sample: "\"<div xmlns=\\\"http://www.w3.org/1999/xhtml\\\"/>\".to_string()",
        sample_json: "\"<div xmlns=\\\"http://www.w3.org/1999/xhtml\\\"/>\"",
        shared_at_crate_root: false,
    },
];

impl Primitive {
    /// Whether the newtype needs a hand-written `impl Default` rather than
    /// deriving one.
    ///
    /// Deriving works whenever the default is the inner type's own default,
    /// which is every generated primitive: `decimal`, the one exception, is
    /// not generated at all (see [`Primitive::shared_at_crate_root`]).
    #[must_use]
    pub fn needs_default_impl(&self) -> bool {
        false
    }
}

impl Primitive {
    /// Can a caller write `types::Foo(value)` for this primitive?
    ///
    /// All but one wrap a plain `std` scalar and can be constructed by tuple
    /// syntax anywhere. `decimal` wraps [`fhir_core::decimal::Decimal`],
    /// whose field is private so that a lexical form cannot be corrupted by
    /// construction — which also means a generated example cannot build one.
    #[must_use]
    pub fn is_directly_constructible(&self) -> bool {
        self.inner.starts_with("std::")
            || matches!(self.inner, "bool" | "i32" | "i64" | "u32" | "u64" | "f64")
    }
}

/// Find a primitive by the Rust type name generated for it.
///
/// The generated newtype is the FHIR code in Pascal case — `base64Binary`
/// becomes `Base64Binary` — which is not recoverable by lowercasing, so this
/// compares against the same transformation the generator applies.
#[must_use]
pub fn by_rust_name(name: &str) -> Option<&'static Primitive> {
    PRIMITIVES
        .iter()
        .find(|p| super::naming::pascal(p.code) == name)
}

/// Look up how a FHIR primitive code is represented in Rust.
#[must_use]
pub fn lookup(code: &str) -> Option<&'static Primitive> {
    PRIMITIVES.iter().find(|p| p.code == code)
}

/// Render the Rust module for one primitive datatype.
///
/// Returns `None` for a code with no entry in the table, which means the
/// release defines a primitive this crate has not been taught to represent.
#[must_use]
pub fn render(sd: &StructureDefinition, version: Version) -> Option<String> {
    let primitive = lookup(sd.type_name())?;
    let name = naming::pascal(sd.type_name());
    let module = version.module();

    let mut out = format!("//! {}\n//!\n", sd.type_name());
    let _ = write!(out, "//! URL: {}\n//!\n", sd.url);
    let _ = write!(
        out,
        "//!{}\n//!\n",
        doc_version(&sd.version.clone().unwrap_or_default())
    );
    let _ = write!(
        out,
        "//! FHIR {}: <{}>\n\n",
        version.label(),
        version.spec_url()
    );

    // `decimal` is the one primitive whose representation is not a plain
    // newtype: its lexical precision is clinical data, so it has a
    // hand-written type at the crate root that every release shares
    // (spec R2.2). Emitting a re-export keeps the module path each release
    // publishes while leaving one place to be correct.
    if primitive.shared_at_crate_root {
        out.push_str(
            "//! `decimal` is defined identically in every modelled release, so the type\n\
             //! lives in the `fhir-core` crate ([`::fhir_core::decimal`]) and each release\n\
             //! it. Its precision is preserved lexically — see spec 02 (R2.2).\n\n",
        );
        let _ = writeln!(
            out,
            "pub use ::fhir_core::decimal::{{{name}, {name}Error}};"
        );
        return Some(out);
    }

    out.push_str("use ::serde::{Deserialize, Serialize};\n");
    if primitive.string_encoded {
        out.push_str("use ::serde_with::{serde_as, DisplayFromStr};\n");
    }
    out.push('\n');

    out.push_str(&naming::doc_comment(
        sd.description.as_deref().unwrap_or_default(),
        "",
    ));
    let _ = write!(
        out,
        "///\n\
         /// # Examples\n\
         ///\n\
         /// ```\n\
         /// use fhir::{module}::types::{}::{name};\n\
         ///\n\
         /// let value = {name}::default();\n\
         /// let json = ::serde_json::to_value(&value).unwrap();\n\
         /// let back: {name} = ::serde_json::from_value(json).unwrap();\n\
         /// assert_eq!(value, back);\n\
         /// ```\n",
        naming::module_name(sd.type_name()),
    );

    if primitive.string_encoded {
        out.push_str("#[serde_as]\n");
    }
    let derive_default = if primitive.needs_default_impl() {
        ""
    } else {
        "Default, "
    };
    let _ = writeln!(
        out,
        "#[derive(Debug, {derive_default}Clone, Serialize, Deserialize, PartialEq, Eq)]"
    );
    let _ = writeln!(out, "pub struct {name}(");
    out.push_str(&naming::doc_comment(primitive.inner_doc, "    "));
    if primitive.string_encoded {
        out.push_str("    // FHIR carries this as a JSON string so that it survives\n");
        out.push_str("    // consumers whose numbers are 64-bit floats.\n");
        out.push_str("    #[serde_as(as = \"DisplayFromStr\")]\n");
    }
    let _ = writeln!(out, "    pub {},", primitive.inner);
    out.push_str(");\n");

    if primitive.needs_default_impl() {
        let _ = write!(
            out,
            "\nimpl Default for {name} {{\n    fn default() -> Self {{\n        {name}({})\n    }}\n}}\n",
            primitive.default_value,
        );
    }

    let default_expr = format!("{name}({})", primitive.default_value);
    let _ = write!(
        out,
        "\n#[cfg(test)]\n\
         mod tests {{\n\
         \x20   use super::*;\n\
         \x20   use ::serde_json::json;\n\
         \n\
         \x20   #[test]\n\
         \x20   fn test_default() {{\n\
         \x20       assert_eq!({name}::default(), {default_expr});\n\
         \x20   }}\n\
         \n\
         \x20   #[test]\n\
         \x20   fn test_serde() {{\n\
         \x20       let value = {name}({});\n\
         \x20       let json = ::serde_json::to_value(&value).expect(\"to_value\");\n\
         \x20       assert_eq!(json, json!({}));\n\
         \x20       let back: {name} = ::serde_json::from_value(json).expect(\"from_value\");\n\
         \x20       assert_eq!(value, back);\n\
         \x20   }}\n\
         }}\n",
        primitive.sample, primitive.sample_json,
    );

    Some(out)
}

/// The primitive type codes a release defines, e.g. `string`, `dateTime`.
#[must_use]
pub fn codes_of(definitions: &[StructureDefinition]) -> BTreeMap<String, StructureDefinition> {
    definitions
        .iter()
        .filter(|sd| sd.kind_name() == "primitive-type")
        .map(|sd| (sd.type_name().to_string(), sd.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sd(name: &str) -> StructureDefinition {
        ::serde_json::from_value(::serde_json::json!({
            "name": name, "type": name, "kind": "primitive-type",
            "url": format!("http://hl7.org/fhir/StructureDefinition/{name}"),
            "version": "4.0.1",
            "description": "A primitive.",
            "snapshot": { "element": [{ "path": name }] }
        }))
        .unwrap()
    }

    #[test]
    fn every_table_entry_has_a_unique_code() {
        let mut codes: Vec<&str> = PRIMITIVES.iter().map(|p| p.code).collect();
        let count = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), count);
    }

    #[test]
    fn string_primitive_is_a_transparent_newtype() {
        let out = render(&sd("string"), Version::R4).unwrap();
        assert!(out.contains("pub struct String("));
        assert!(out.contains("pub std::string::String,"));
        assert!(out.contains("String(std::string::String::new())"));
        assert!(
            out.contains("#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]")
        );
        assert!(out.contains("use fhir::r4::types::string::String;"));
    }

    #[test]
    fn decimal_re_exports_the_shared_type() {
        // `decimal` is not generated as a newtype: its lexical precision
        // needs custom serde, so one hand-written type at the crate root
        // serves every release (R2.2).
        let out = render(&sd("decimal"), Version::R4).unwrap();
        assert!(out.contains("pub use ::fhir_core::decimal::{Decimal, DecimalError};"));
        assert!(!out.contains("pub struct Decimal"));
        assert!(!out.contains("serde_json::Number"));
    }

    #[test]
    fn integer64_is_carried_as_a_json_string() {
        let out = render(&sd("integer64"), Version::R5).unwrap();
        assert!(out.contains("#[serde_as]"));
        assert!(out.contains("#[serde_as(as = \"DisplayFromStr\")]"));
        assert!(out.contains("pub i64,"));
    }

    #[test]
    fn an_unknown_primitive_is_reported_rather_than_guessed() {
        assert!(render(&sd("quantum"), Version::R4).is_none());
    }
}
