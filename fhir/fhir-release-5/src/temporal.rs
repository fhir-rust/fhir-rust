//! Parsing and precision-aware comparison for the FHIR R5 date/time
//! primitives.
//!
//! FHIR stores `date`, `dateTime`, `instant`, and `time` as strings, and this
//! crate keeps that representation unchanged. The parsing itself does not vary
//! by release and lives in [`::fhir_core::temporal`]; this module attaches it to the
//! R5 newtypes and re-exports the shared types.
//!
//! ```
//! use fhir::r5::types::Date;
//! use fhir::r5::temporal::DatePrecision;
//!
//! let d = Date("2024-03".to_string());
//! let parts = d.parse_parts().unwrap();
//! assert_eq!(parts.year, 2024);
//! assert_eq!(parts.month, Some(3));
//! assert_eq!(parts.day, None);
//! assert_eq!(parts.precision(), DatePrecision::Month);
//! ```

use crate::r5::types::{Date, DateTime, Instant, Time};

pub use ::fhir_core::temporal::{DateParts, DatePrecision, TimeParts};

impl Date {
    /// Parse this `date` into its calendar components.
    #[must_use]
    pub fn parse_parts(&self) -> Option<DateParts> {
        DateParts::parse(&self.0)
    }
}

impl DateTime {
    /// Parse the calendar (date) part of this `dateTime`. The value may be just
    /// a date (`2024`, `2024-03`) or a full timestamp; this returns the date
    /// components in either case.
    #[must_use]
    pub fn date_parts(&self) -> Option<DateParts> {
        let date = self.0.split(['T', 't']).next().unwrap_or(&self.0);
        DateParts::parse(date)
    }
}

impl Instant {
    /// Parse the calendar (date) part of this `instant`.
    #[must_use]
    pub fn date_parts(&self) -> Option<DateParts> {
        let date = self.0.split(['T', 't']).next().unwrap_or(&self.0);
        DateParts::parse(date)
    }
}

impl Time {
    /// Parse this `time` into its clock components.
    #[must_use]
    pub fn parse_parts(&self) -> Option<TimeParts> {
        TimeParts::parse(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_precisions() {
        assert_eq!(
            Date("2024".into()).parse_parts().unwrap(),
            DateParts {
                year: 2024,
                month: None,
                day: None
            }
        );
        assert_eq!(
            Date("2024-03-25".into()).parse_parts().unwrap().precision(),
            DatePrecision::Day
        );
    }

    #[test]
    fn datetime_and_instant_date_part() {
        let dt = DateTime("2015-02-07T13:28:17-05:00".into());
        assert_eq!(dt.date_parts().unwrap().precision(), DatePrecision::Day);
        let inst = Instant("2015-02-07T13:28:17.239+02:00".into());
        assert_eq!(inst.date_parts().unwrap().day, Some(7));
    }

    #[test]
    fn parse_time() {
        let t = Time("13:28:17".into()).parse_parts().unwrap();
        assert_eq!((t.hour, t.minute, t.second), (13, 28, 17));
        assert!(Time("25:00:00".into()).parse_parts().is_none());
    }
}
