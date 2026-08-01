//! Parsing and precision-aware comparison for FHIR date/time primitives.
//!
//! FHIR stores `date`, `dateTime`, `instant`, and `time` as strings, and this
//! crate keeps that representation unchanged. This module adds *reading*
//! helpers on top: it parses a value into its calendar/clock components and
//! compares two partial dates per the FHIR precision rules, where a comparison
//! between values of different precision may be **indeterminate**.
//!
//! The rules are the same in every FHIR release, so the parsing lives here and
//! each release only attaches it to its own primitive newtypes — see
//! [`r4::temporal`](crate::r4::temporal) and [`r5::temporal`](crate::r5::temporal).
//!
//! ```
//! use fhir::temporal::{DateParts, DatePrecision};
//!
//! let parts = DateParts::parse("2024-03").unwrap();
//! assert_eq!(parts.year, 2024);
//! assert_eq!(parts.month, Some(3));
//! assert_eq!(parts.day, None);
//! assert_eq!(parts.precision(), DatePrecision::Month);
//! ```

use std::cmp::Ordering;

/// The calendar precision of a `date`/`dateTime` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatePrecision {
    /// `YYYY`
    Year,
    /// `YYYY-MM`
    Month,
    /// `YYYY-MM-DD`
    Day,
}

/// The calendar components of a FHIR `date` (or the date part of a `dateTime`).
///
/// `PartialOrd` follows the FHIR rule: two values compare only when the answer
/// is definite. `"2024"` vs `"2025-03"` is `Some(Less)` (different years), but
/// `"2024"` vs `"2024-03"` is `None` — the year-precision value spans the month,
/// so the order is indeterminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateParts {
    /// Four-digit year (0001–9999 in FHIR).
    pub year: i32,
    /// Month `1..=12`, if present.
    pub month: Option<u8>,
    /// Day `1..=31`, if present (only when `month` is present).
    pub day: Option<u8>,
}

impl DateParts {
    /// The precision implied by which components are present.
    #[must_use]
    pub fn precision(&self) -> DatePrecision {
        match (self.month, self.day) {
            (None, _) => DatePrecision::Year,
            (Some(_), None) => DatePrecision::Month,
            (Some(_), Some(_)) => DatePrecision::Day,
        }
    }

    /// Parse `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. Returns `None` if malformed or
    /// out of range.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let mut it = s.split('-');
        let year: i32 = it.next()?.parse().ok()?;
        if !(1..=9999).contains(&year) {
            return None;
        }
        let month = match it.next() {
            Some(m) => {
                let m: u8 = m.parse().ok()?;
                if !(1..=12).contains(&m) {
                    return None;
                }
                Some(m)
            }
            None => None,
        };
        let day = match it.next() {
            Some(d) => {
                month?; // a day without a month is malformed
                let d: u8 = d.parse().ok()?;
                if !(1..=31).contains(&d) {
                    return None;
                }
                Some(d)
            }
            None => None,
        };
        if it.next().is_some() {
            return None; // trailing junk
        }
        Some(DateParts { year, month, day })
    }
}

impl PartialOrd for DateParts {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year != other.year {
            return Some(self.year.cmp(&other.year));
        }
        indeterminate_or_equal(*self, *other)
    }
}

/// Resolve the same-year case: equal when both stop at the same
/// precision, a definite order when the finest shared component differs, or
/// `None` when one side is less precise than the other.
fn indeterminate_or_equal(a: DateParts, b: DateParts) -> Option<Ordering> {
    match (a.month, b.month) {
        (None, None) => Some(Ordering::Equal),
        (Some(_), None) | (None, Some(_)) => None,
        (Some(m1), Some(m2)) => {
            if m1 != m2 {
                return Some(m1.cmp(&m2));
            }
            match (a.day, b.day) {
                (None, None) => Some(Ordering::Equal),
                (Some(_), None) | (None, Some(_)) => None,
                (Some(d1), Some(d2)) => Some(d1.cmp(&d2)),
            }
        }
    }
}

/// The clock components of a FHIR `time` (`hh:mm:ss` with optional fraction).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeParts {
    /// Hour `0..=23`.
    pub hour: u8,
    /// Minute `0..=59`.
    pub minute: u8,
    /// Second `0..=59` (FHIR also permits `60` for leap seconds).
    pub second: u8,
    /// Fractional-seconds digits, if present (e.g. `"250"` for `.250`).
    pub fraction: Option<String>,
}

impl TimeParts {
    /// Parse `hh:mm:ss` or `hh:mm:ss.fff`. Returns `None` if malformed.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let (hms, fraction) = match s.split_once('.') {
            Some((h, f)) if f.chars().all(|c| c.is_ascii_digit()) && !f.is_empty() => {
                (h, Some(f.to_string()))
            }
            Some(_) => return None,
            None => (s, None),
        };
        let mut it = hms.split(':');
        let hour: u8 = it.next()?.parse().ok()?;
        let minute: u8 = it.next()?.parse().ok()?;
        let second: u8 = it.next()?.parse().ok()?;
        if it.next().is_some() || hour > 23 || minute > 59 || second > 60 {
            return None;
        }
        Some(TimeParts {
            hour,
            minute,
            second,
            fraction,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_precisions() {
        assert_eq!(
            DateParts::parse("2024").unwrap(),
            DateParts {
                year: 2024,
                month: None,
                day: None
            }
        );
        assert_eq!(
            DateParts::parse("2024-03").unwrap().precision(),
            DatePrecision::Month
        );
        assert_eq!(
            DateParts::parse("2024-03-25").unwrap().precision(),
            DatePrecision::Day
        );
    }

    #[test]
    fn rejects_malformed_dates() {
        assert!(DateParts::parse("2024-13").is_none()); // bad month
        assert!(DateParts::parse("2024-03-32").is_none()); // bad day
        assert!(DateParts::parse("2024-03-25T00:00").is_none()); // not a date
        assert!(DateParts::parse("").is_none());
    }

    #[test]
    fn date_ordering_same_precision() {
        let a = DateParts::parse("2024-03").unwrap();
        let b = DateParts::parse("2024-05").unwrap();
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    }

    #[test]
    fn date_ordering_different_precision() {
        let year = DateParts::parse("2024").unwrap();
        let month = DateParts::parse("2024-03").unwrap();
        // Same year, different precision -> indeterminate.
        assert_eq!(year.partial_cmp(&month), None);
        // Different year -> definite regardless of precision.
        let other = DateParts::parse("2025-03").unwrap();
        assert_eq!(year.partial_cmp(&other), Some(Ordering::Less));
    }

    #[test]
    fn parse_time() {
        let t = TimeParts::parse("13:28:17").unwrap();
        assert_eq!((t.hour, t.minute, t.second), (13, 28, 17));
        assert_eq!(t.fraction, None);
        assert_eq!(
            TimeParts::parse("13:28:17.250").unwrap().fraction,
            Some("250".into())
        );
        assert!(TimeParts::parse("25:00:00").is_none());
    }
}
