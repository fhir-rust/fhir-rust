//! Canonical JSON: the exact bytes a history row's hash chain commits to.
//!
//! The PostgreSQL original delegated this to the database, hashing
//! `(($1::text)::jsonb)::text` — whatever `jsonb` happened to produce when it
//! reordered keys and rewrote number spellings. That is not portable: no other
//! engine reproduces those bytes, so a chain written by one dialect could
//! never be verified by another, and this port could not reuse the format at
//! all.
//!
//! Canonicalization therefore lives here, in Rust, above the driver. One
//! function, used by both the writer and the verifier, so the two cannot drift
//! into disagreeing about what was signed (spec M14.20).
//!
//! # Why not RFC 8785
//!
//! JCS is the obvious standard and it is wrong for FHIR. §3.2.2.3 serializes
//! numbers via ECMAScript `Number::toString` — as IEEE-754 doubles. That turns
//! `1.50` into `1.5` and loses everything past the seventeenth significant
//! digit, which collides resources that differ in a clinically meaningful
//! decimal and violates the round-trip precision M3.6 requires. Numbers here
//! are emitted as their parsed lexeme instead.
//!
//! # The form
//!
//! - Object keys sorted by UTF-8 byte order.
//! - Numbers verbatim, as parsed (`serde_json`'s `arbitrary_precision`).
//! - Strings with only the escapes JSON requires; `\u00XX` lowercase hex for
//!   other control characters; non-ASCII passed through as UTF-8.
//! - No insignificant whitespace.
//!
//! Duplicate object keys are deliberately *not* handled here, because by the
//! time a resource is a `Value` they cannot exist: `serde_json` resolves them
//! during parsing (last one wins) and `Map` keys are unique by construction.
//! Rejecting them is a parse-time concern, not a canonicalization one, and
//! pretending otherwise here would be an unreachable check that reads like a
//! guarantee.

use std::fmt::Write as _;

use serde_json::Value;

/// Canonicalize a resource into the bytes its hash chain commits to.
///
/// Infallible. Every `Value` has exactly one canonical form: duplicate keys
/// cannot exist in a parsed `Map`, and a non-finite number cannot be
/// constructed (`Number::from_f64` rejects them), so there is no input this
/// can refuse. An error return would be a branch no caller could ever take.
#[must_use]
pub fn canonicalize(v: &Value) -> String {
    let mut out = String::new();
    write_value(&mut out, v);
    out
}

fn write_value(out: &mut String, v: &Value) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        // With `arbitrary_precision`, `Display` yields the original lexeme, so
        // `1.50` stays `1.50` and a thirty-digit decimal stays thirty digits.
        // The one normalization it applies — an explicit exponent sign, `1e2`
        // becoming `1e+2` — is deterministic, so it cannot break a chain.
        Value::Number(n) => {
            let _ = write!(out, "{n}");
        }
        Value::String(s) => write_string(out, s),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(out, item);
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Sort by UTF-8 bytes. `serde_json` is built here with
            // `preserve_order`, so the map arrives in document order and the
            // sort is what makes the output independent of input key order.
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_unstable_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
            out.push('{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(out, k);
                out.push(':');
                write_value(out, &map[*k]);
            }
            out.push('}');
        }
    }
}

/// Minimal JSON string escaping.
///
/// Only what the grammar requires, so the output is the shortest valid
/// spelling and there is exactly one of them. Non-ASCII is emitted as UTF-8
/// rather than `\u` escapes; lone surrogates cannot occur because Rust `String`
/// is well-formed UTF-8.
fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canon(s: &str) -> String {
        canonicalize(&serde_json::from_str(s).unwrap())
    }

    #[test]
    fn keys_sort_by_utf8_bytes_not_length() {
        // PostgreSQL's jsonb sorts by length first, which would put "z" before
        // "aa". This form does not, and the difference is exactly why an
        // existing PostgreSQL chain cannot be reverified against this output.
        assert_eq!(canon(r#"{"z":1,"aa":2}"#), r#"{"aa":2,"z":1}"#);
    }

    #[test]
    fn key_order_does_not_affect_output() {
        let a = canon(r#"{"b":1,"a":{"d":2,"c":3}}"#);
        let b = canon(r#"{"a":{"c":3,"d":2},"b":1}"#);
        assert_eq!(a, b);
        assert_eq!(a, r#"{"a":{"c":3,"d":2},"b":1}"#);
    }

    #[test]
    fn decimal_precision_survives() {
        // The whole reason RFC 8785 is unusable here: under JCS these would
        // become 1.5, 0.1, and 1.2345678901234568e29.
        assert_eq!(canon(r#"{"v":1.50}"#), r#"{"v":1.50}"#);
        assert_eq!(canon(r#"{"v":1.000}"#), r#"{"v":1.000}"#);
        assert_eq!(
            canon(r#"{"v":123456789012345678901234567890.123}"#),
            r#"{"v":123456789012345678901234567890.123}"#
        );
    }

    #[test]
    fn trailing_zeros_are_not_equal_to_their_absence() {
        // Two resources differing only in decimal precision MUST NOT collide,
        // because M3.6 makes that difference observable on read.
        assert_ne!(canon(r#"{"v":1.50}"#), canon(r#"{"v":1.5}"#));
    }

    #[test]
    fn array_order_is_preserved() {
        // Arrays are ordered data in FHIR; canonicalization must not sort them.
        assert_eq!(canon(r#"{"a":[3,1,2]}"#), r#"{"a":[3,1,2]}"#);
    }

    #[test]
    fn no_insignificant_whitespace() {
        assert_eq!(
            canon("{ \"a\" : [ 1 , 2 ] , \"b\" : null }"),
            r#"{"a":[1,2],"b":null}"#
        );
    }

    #[test]
    fn strings_escape_minimally() {
        assert_eq!(canon(r#"{"a":"x\"y\\z"}"#), r#"{"a":"x\"y\\z"}"#);
        assert_eq!(canon(r#"{"a":"l1\nl2\tx"}"#), r#"{"a":"l1\nl2\tx"}"#);
        // Control characters without a short escape use lowercase \u00XX. The
        // input carries the JSON escape and the output must carry it too.
        assert_eq!(canon(r#"{"a":"\u0001"}"#), r#"{"a":"\u0001"}"#);
        // Non-ASCII passes through as UTF-8 rather than being \u-escaped, so a
        // literal and an escaped input converge on the same canonical bytes.
        assert_eq!(
            canon("{\"a\":\"\u{c6}r\u{f8}\"}"),
            "{\"a\":\"\u{c6}r\u{f8}\"}"
        );
        assert_eq!(
            canon(r#"{"a":"\u00c6r\u00f8"}"#),
            "{\"a\":\"\u{c6}r\u{f8}\"}"
        );
    }

    #[test]
    fn empty_containers() {
        assert_eq!(canon(r#"{"a":{},"b":[]}"#), r#"{"a":{},"b":[]}"#);
        assert_eq!(canon("{}"), "{}");
    }

    #[test]
    fn nesting_is_canonicalized_at_every_depth() {
        assert_eq!(
            canon(r#"{"o":[{"b":1,"a":2},{"d":3,"c":4}]}"#),
            r#"{"o":[{"a":2,"b":1},{"c":4,"d":3}]}"#
        );
    }

    #[test]
    fn output_reparses_to_the_same_value() {
        // Canonical output must itself be valid JSON denoting the same value,
        // so canonicalization is idempotent.
        for src in [
            r#"{"b":1,"a":[1.50,"Ærø",null,true],"c":{"z":{}}}"#,
            r#"{"a":"tab\there","b":1e2}"#,
        ] {
            let once = canon(src);
            let twice = canon(&once);
            assert_eq!(once, twice, "not idempotent for {src}");
            let v1: Value = serde_json::from_str(src).unwrap();
            let v2: Value = serde_json::from_str(&once).unwrap();
            assert_eq!(v1, v2, "value changed for {src}");
        }
    }

    #[test]
    fn duplicate_keys_are_resolved_by_the_parser_before_we_see_them() {
        // Documents where the responsibility actually lies. A duplicate key
        // cannot reach `canonicalize` — `serde_json` collapses it at parse time,
        // last one winning — so any policy about rejecting duplicates has to be
        // enforced on ingest, not here. What matters for the chain is only that
        // the collapse is deterministic, which this pins down.
        let a: Value = serde_json::from_str(r#"{"a":1,"a":2}"#).unwrap();
        assert_eq!(canonicalize(&a), r#"{"a":2}"#);
        let b: Value = serde_json::from_str(r#"{"a":2,"a":2}"#).unwrap();
        assert_eq!(canonicalize(&a), canonicalize(&b));
    }

    #[test]
    fn non_finite_numbers_cannot_be_constructed() {
        // Why `canonicalize` is infallible: the only value with no canonical
        // form is unrepresentable in the first place.
        assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
        assert!(serde_json::Number::from_f64(f64::NAN).is_none());
    }

    #[test]
    fn realistic_resource_is_stable() {
        let src = r#"{
            "resourceType": "Observation",
            "id": "abc",
            "valueQuantity": { "value": 9.60, "unit": "mg/dL" },
            "code": { "coding": [ { "system": "http://loinc.org", "code": "2339-0" } ] }
        }"#;
        let got = canon(src);
        assert_eq!(
            got,
            r#"{"code":{"coding":[{"code":"2339-0","system":"http://loinc.org"}]},"id":"abc","resourceType":"Observation","valueQuantity":{"unit":"mg/dL","value":9.60}}"#
        );
        // The precision of 9.60 is what a lab reported; it must survive.
        assert!(got.contains("9.60"));
    }
}
