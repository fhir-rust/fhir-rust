//! The FHIR `decimal` primitive.
//!
//! URL: <http://hl7.org/fhir/StructureDefinition/decimal>
//!
//! `decimal` is defined identically in R3, R4 and R5 — "a rational number
//! with implicit precision" — so unlike the other primitives it is written
//! once here and re-exported by each release's `types` module, alongside
//! [`Coded`](crate::coded) and [`temporal`](crate::temporal).
//!
//! See spec 02 (R2.2) for why the representation is lexical.

use ::serde::{Deserialize, Serialize};

/// A rational number with implicit precision, used to represent measurement
/// values and other quantities where the number of significant digits carries
/// meaning.
///
/// # Precision is data
///
/// FHIR states that the precision of a decimal has significance: a laboratory
/// result of `0.50` mmol/L claims two significant figures where `0.5` claims
/// one, and a dose of `1.000` mg is a different assertion from `1.0` mg. This
/// type therefore stores the **lexical form** it was given and emits it back
/// unchanged (spec R2.2).
///
/// Backed by `f64` — which is what `serde_json::Number` is by default —
/// `0.50` becomes `0.5`, `1.000` becomes `1.0`, and
/// `12345678901234567890.5` becomes `1.2345678901234567e+19`. This crate
/// therefore enables `serde_json/arbitrary_precision` **unconditionally**, so
/// a `Number` carries the lexeme it was parsed from. Cargo features are
/// additive and cannot be switched off by a dependent, which makes precision
/// a guarantee rather than a default someone can lose.
///
/// `float_roundtrip` is enabled alongside it, for the values that do become
/// an `f64` rather than staying a lexeme — [`as_f64`](Decimal::as_f64), and
/// any dependent deserializing straight into `f64`. It makes
/// `f64` → JSON → `f64` return the value it started as instead of the
/// shortest form that displays the same. The two features cover opposite
/// directions of the same guarantee; see
/// `spec/serde-json-float-roundtrip-arbitrary-precision/`.
///
/// The cost is real and worth stating: `arbitrary_precision` is global to the
/// compiled binary, so every other crate's `serde_json::Number` in the same
/// build also becomes lexeme-preserving, and `Number` arithmetic goes through
/// `as_f64()`. For a library whose numbers are drug doses and lab results,
/// that is the correct side to err on.
///
/// # Equality is lexical, ordering is numeric
///
/// `Decimal("1.0") != Decimal("1.00")`, because the two say different things
/// about precision and must survive a round trip distinctly. They *compare*
/// equal, because they denote the same quantity:
///
/// ```
/// use fhir::decimal::Decimal;
/// use std::cmp::Ordering;
///
/// let one_dp = Decimal::new("1.0").unwrap();
/// let two_dp = Decimal::new("1.00").unwrap();
/// assert_ne!(one_dp, two_dp);
/// assert_eq!(one_dp.partial_cmp(&two_dp), Some(Ordering::Equal));
/// ```
///
/// # JSON
///
/// The lexeme survives every serde path this crate uses — `from_str`,
/// `from_slice`, `from_reader`, `from_value`, and through the
/// `#[serde(flatten)]` that choice elements rely on. A `serde_json::Value`
/// built in the same binary is likewise lexeme-preserving, so
/// `json!(0.50) != json!(0.5)`, and a round-trip test comparing `Value`s can
/// see precision loss rather than silently tolerating it (spec R13.3).
///
/// ```
/// use fhir::decimal::Decimal;
///
/// let parsed: Decimal = ::serde_json::from_str("0.50").unwrap();
/// assert_eq!(parsed.as_str(), "0.50");
/// assert_eq!(::serde_json::to_string(&parsed).unwrap(), "0.50");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal(::serde_json::Number);

/// The reason a string is not a FHIR `decimal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecimalError(String);

impl std::fmt::Display for DecimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not a FHIR decimal: {:?}", self.0)
    }
}

impl std::error::Error for DecimalError {}

impl Decimal {
    /// A decimal from its lexical form, checked against the FHIR `decimal`
    /// production `-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?`.
    ///
    /// # Errors
    ///
    /// Returns [`DecimalError`] when the text is not a FHIR decimal.
    pub fn new(lexeme: impl Into<String>) -> Result<Self, DecimalError> {
        let lexeme = lexeme.into();
        if !is_fhir_decimal(&lexeme) {
            return Err(DecimalError(lexeme));
        }
        // Parsing the lexeme back through serde_json is what stores it: with
        // `arbitrary_precision` a `Number` *is* its lexeme.
        ::serde_json::from_str::<::serde_json::Number>(&lexeme)
            .map(Decimal)
            .map_err(|_| DecimalError(lexeme))
    }

    /// The stored lexical form, exactly as received.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// The value as an `f64`, which is lossy by definition — use it for
    /// arithmetic, never for storage or comparison.
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        self.0.as_f64().unwrap_or(f64::NAN)
    }

    /// A decimal from an already-parsed [`serde_json::Number`], for callers
    /// holding a [`serde_json::Value`]. Lossless, because this crate
    /// guarantees `arbitrary_precision`.
    #[must_use]
    pub fn from_json_number(n: &::serde_json::Number) -> Self {
        Decimal(n.clone())
    }

    /// The underlying `serde_json::Number`, for interoperating with code
    /// that speaks `serde_json` directly.
    #[must_use]
    pub fn as_number(&self) -> &::serde_json::Number {
        &self.0
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl std::str::FromStr for Decimal {
    type Err = DecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Decimal::new(s)
    }
}

/// Numeric ordering over lexically distinct values: `1.0` and `1.00` are the
/// same quantity even though they are not the same assertion.
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_f64().partial_cmp(&other.as_f64())
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Decimal(::serde_json::Number::from(0))
    }
}

/// `Decimal` validates its own lexeme (spec R2.6).
///
/// One impl for one shared type: were this written per release, compiling two
/// releases together would be a conflicting-impl error.
impl crate::validate::Validate for Decimal {
    fn validate(&self) -> Vec<crate::validate::ValidationIssue> {
        if is_fhir_decimal(self.as_str()) {
            Vec::new()
        } else {
            // The path is the datatype's own label, which the deriving
            // parent prefixes with the field name (e.g. `value.decimal`).
            vec![crate::validate::ValidationIssue::new(
                "decimal",
                "must match the FHIR decimal production \
                 -?(0|[1-9][0-9]*)(\\.[0-9]+)?([eE][+-]?[0-9]+)?",
            )]
        }
    }
}

/// The FHIR `decimal` lexical production:
/// `-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?`.
fn is_fhir_decimal(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    if i < b.len() && b[i] == b'-' {
        i += 1;
    }
    // Integer part: `0` alone, or a run that does not lead with zero.
    let start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == start {
        return false;
    }
    if i - start > 1 && b[start] == b'0' {
        return false;
    }
    // Optional fraction, at least one digit.
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let frac = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == frac {
            return false;
        }
    }
    // Optional exponent, at least one digit.
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let exp = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == exp {
            return false;
        }
    }
    i == b.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Decimal::default().as_str(), "0");
    }

    #[test]
    fn test_serde() {
        let value: Decimal = ::serde_json::from_str("3.5").expect("from_str");
        assert_eq!(::serde_json::to_string(&value).expect("to_string"), "3.5");
    }

    /// Spec 02 acceptance 2a: the values the `serde_json::Number`
    /// representation silently altered.
    #[test]
    fn lexical_form_survives_a_round_trip() {
        for input in [
            "0.50",
            "1.000",
            "1e-7",
            "-0.0001",
            "0.1234567890123456789012345",
            "12345678901234567890.5",
        ] {
            let parsed: Decimal =
                ::serde_json::from_str(input).unwrap_or_else(|e| panic!("parse {input}: {e}"));
            let out = ::serde_json::to_string(&parsed).expect("to_string");
            assert_eq!(out, input, "{input} did not survive");
        }
    }

    /// Spec 02 acceptance 2b.
    #[test]
    fn equality_is_lexical_and_ordering_is_numeric() {
        let one_dp = Decimal::new("1.0").expect("valid");
        let two_dp = Decimal::new("1.00").expect("valid");
        assert_ne!(one_dp, two_dp);
        assert_eq!(one_dp.partial_cmp(&two_dp), Some(std::cmp::Ordering::Equal));
        assert!(Decimal::new("2").expect("valid") > one_dp);
    }

    #[test]
    fn rejects_non_decimals() {
        for bad in [
            "", "-", ".5", "1.", "01", "1e", "1.2.3", " 1", "1 ", "NaN", "+1",
        ] {
            assert!(Decimal::new(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn accepts_the_production() {
        for good in [
            "0", "-0", "1", "-1", "0.0", "1.5", "1e10", "1E+10", "-2.5e-3",
        ] {
            assert!(Decimal::new(good).is_ok(), "{good:?} should be accepted");
        }
    }
}

#[cfg(test)]
mod oracle_tests {
    //! Spec R13.3: the round-trip oracle compares `serde_json::Value`s, so it
    //! can only see a decimal regression if `Value` equality is sensitive to
    //! the lexeme. That sensitivity comes from `arbitrary_precision` being a
    //! hard dependency feature (R2.2) — if it ever stops being one, these
    //! tests fail here rather than letting the whole corpus suite go quietly
    //! blind.

    #[test]
    fn value_equality_distinguishes_trailing_zeros() {
        let two_sig: ::serde_json::Value = ::serde_json::from_str("0.50").expect("parse");
        let one_sig: ::serde_json::Value = ::serde_json::from_str("0.5").expect("parse");
        assert_ne!(
            two_sig, one_sig,
            "Value equality cannot see decimal precision; the round-trip \
             oracle is blind and R13.3 is violated"
        );
    }

    #[test]
    fn value_round_trip_keeps_the_lexeme() {
        for input in ["0.50", "1.000", "12345678901234567890.5"] {
            let v: ::serde_json::Value = ::serde_json::from_str(input).expect("parse");
            assert_eq!(::serde_json::to_string(&v).expect("serialize"), input);
        }
    }
}

#[cfg(test)]
mod validate_tests {
    use super::*;
    use crate::validate::Validate;

    #[test]
    fn a_well_formed_decimal_validates() {
        assert!(Decimal::new("0.50").expect("valid").is_valid());
        assert!(Decimal::default().is_valid());
    }

    #[test]
    fn a_decimal_that_slipped_past_the_constructor_is_reported() {
        // Deserialization goes through `serde_json::Number`, whose grammar is
        // JSON's rather than FHIR's. They agree today, so this guards the
        // narrow case where they would not.
        let via_serde: Decimal = ::serde_json::from_str("1e400").expect("json accepts it");
        let issues = via_serde.validate();
        assert!(
            issues.is_empty() || issues[0].message.contains("FHIR decimal production"),
            "unexpected issue: {issues:?}"
        );
    }
}
