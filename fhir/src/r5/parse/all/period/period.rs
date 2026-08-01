//! # period
//!
//! ## Description
//!
//! The `period` property contains a time period defined by start and end
//! date/time values. It represents duration-based data such as treatment
//! periods, enrollment spans, validity ranges, or any time-bounded clinical
//! events.
//!
//! ## Purpose
//!
//! - Define time ranges for clinical events and treatments
//! - Specify validity periods for data and authorizations
//! - Represent duration-based measurements and observations
//! - Support temporal queries and filtering
//! - Enable time-based business rule evaluation
//!
//! ## Usage
//!
//! The `period` property is used in extensions, parameters, and data elements
//! where time ranges need to be specified, such as treatment periods, coverage
//! spans, or observation intervals.
//!
//! ## Data Type
//!
//! **Period** - A complex type containing:
//! - `start` - Start date/time
//! - `end` - End date/time
//!
//! ## Constraints
//!
//! - Start must be before or equal to end if both are present
//! - At least one of start or end should be present
//! - Date/time values must be valid ISO 8601 format
//! - Open-ended periods can omit start or end
//! - Should include time zones for precision
//!
//! ## Examples
//!
//! ### Extension with Treatment Period
//!
//! ```json
//! {
//!   "extension": [
//!     {
//!       "url": "http://example.org/fhir/StructureDefinition/treatment-period",
//!       "period": {
//!         "start": "2024-01-15T09:00:00Z",
//!         "end": "2024-03-15T17:00:00Z"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ### Parameter with Coverage Period
//!
//! ```json
//! {
//!   "parameter": [
//!     {
//!       "name": "coveragePeriod",
//!       "period": {
//!         "start": "2024-01-01T00:00:00Z",
//!         "end": "2024-12-31T23:59:59Z"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ## Related Keys
//!
//! - `valueDateTime` - Single date/time values
//! - `effectivePeriod` - Effective time ranges
//! - `start` - Period start times
//! - `end` - Period end times
//! - `period` - Direct period objects
//!
//! ## Specification Reference
//!
//! FHIR R5 Data Types: [Period](http://hl7.org/fhir/R5/datatypes.html#Period)

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Period {
    /// # start
    ///
    /// ## Description
    ///
    /// The `start` property specifies the beginning date/time of a period,
    /// event, or time range in FHIR resources.
    ///
    /// ## Purpose
    ///
    /// - Define temporal boundaries and periods
    /// - Enable time-based queries and filtering
    /// - Support scheduling and temporal logic
    /// - Provide precise timing information
    /// - Enable period calculations and comparisons
    ///
    /// ## Usage
    ///
    /// The `start` property is used in Period data types and temporal elements
    /// throughout FHIR resources.
    ///
    /// ## Data Type
    ///
    /// **dateTime** - A date and time value
    ///
    /// ## Constraints
    ///
    /// - Must be a valid dateTime format
    /// - Should precede associated end times
    /// - Should include timezone when needed
    /// - Must support intended temporal logic
    ///
    /// ## Examples
    ///
    /// ### Period Start
    ///
    /// ```json
    /// {
    ///   "period": {
    ///     "start": "2024-01-15T09:00:00Z",
    ///     "end": "2024-01-15T17:00:00Z"
    ///   }
    /// }
    /// ```
    ///
    /// ## Related Keys
    ///
    /// - `end` - Period end times
    /// - `period` - Time periods
    /// - `effectiveDateTime` - Effective times
    /// - `date` - Date values
    ///
    /// ## Specification Reference
    ///
    /// - [FHIR R5 Period
    ///   Datatype](https://hl7.org/fhir/R5/datatypes.html#Period)
    /// - [FHIR R5 DateTime](https://hl7.org/fhir/R5/datatypes.html#dateTime)
    /// - [FHIR R5 Timing](https://hl7.org/fhir/R5/datatypes.html#Timing)
    ///
    pub start: String,

    /// # end
    ///
    /// ## Description
    ///
    /// The `end` field specifies the ending point of a time period, date range,
    /// or temporal interval. It is commonly used within Period data types to
    /// define when something concludes or becomes inactive.
    ///
    /// ## Purpose
    ///
    /// - Define the conclusion of time periods and intervals
    /// - Establish temporal boundaries for clinical events
    /// - Support period-based queries and filtering
    /// - Enable time-based business logic and rules
    /// - Document the completion or termination of activities
    ///
    /// ## Usage
    ///
    /// The `end` field is commonly used in:
    ///
    /// - **Period data types**: End of time periods
    /// - **Appointment**: End time of scheduled appointments
    /// - **Encounter**: End of healthcare encounters
    /// - **Coverage**: End date of insurance coverage
    /// - **PractitionerRole**: End of practitioner assignments
    ///
    /// ## Data Type
    ///
    /// - **Type**: dateTime
    /// - **Cardinality**: 0..1 (within Period)
    /// - **Format**: YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss+zz:zz
    /// - **Precision**: Can specify year, month, day, or full datetime
    ///
    /// ## Constraints
    ///
    /// - Must be after or equal to the start date when both are present
    /// - Should use appropriate timezone information
    /// - Must be valid datetime according to ISO 8601
    /// - Should align with business and clinical requirements
    ///
    /// ## Examples
    ///
    /// See the accompanying `example.json` for practical usage examples.
    ///
    /// ## Related Keys
    ///
    /// - `start`: Beginning of time periods
    /// - `period`: Complete period specification
    /// - `effectivePeriod`: Validity time periods
    /// - `date`: Specific dates without periods
    /// - `timing`: Complex timing specifications
    ///
    /// ## Specification Reference
    ///
    /// - [FHIR R5 Period
    ///   Datatype](https://hl7.org/fhir/R5/datatypes.html#Period)
    /// - [FHIR R5 DateTime](https://hl7.org/fhir/R5/datatypes.html#dateTime)
    /// - [FHIR R5 Timing](https://hl7.org/fhir/R5/datatypes.html#Timing)
    ///
    pub end: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Period;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::all::DIR
            .join("period")
            .join("period.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.start, "my start");
    }
}
