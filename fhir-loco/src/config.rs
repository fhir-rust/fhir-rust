//! Environment configuration, read so that absent and invalid differ.
//!
//! A setting that is not set takes its declared default (RCA-IN-3); a setting
//! that is set but not usable (not Unicode, not a number, not a flag) stops
//! startup with an error naming it (RCA-IN-4). Nothing here replaces an invalid
//! value with a default.

use std::ffi::OsString;

/// The value of `name`: `Ok(None)` when unset, an error when set but not
/// Unicode.
pub fn env_opt(name: &str) -> Result<Option<String>, String> {
    text(name, std::env::var_os(name))
}

/// The value of `name`, or `default` when it is unset.
pub fn env_or(name: &str, default: &str) -> Result<String, String> {
    // RCA-IN-3: absent only (FHIR_LOCO_* defaults are listed in INSTALL.md)
    Ok(env_opt(name)?.unwrap_or_else(|| default.to_string()))
}

/// A boolean setting: unset is `false`; `true`/`1` and `false`/`0` are the
/// only values accepted.
pub fn env_flag(name: &str) -> Result<bool, String> {
    flag(name, env_opt(name)?.as_deref())
}

/// `FHIR_LOCO_DISCLOSURE_FAIL_OPEN`: serve content even when its disclosure
/// could not be recorded (`PR12.5`). Off unless set to `true`/`1`.
pub fn disclosure_fail_open() -> Result<bool, String> {
    env_flag("FHIR_LOCO_DISCLOSURE_FAIL_OPEN")
}

/// `FHIR_LOCO_EXPORT_TTL_SECS`: whole seconds, default 3600 when unset.
pub fn export_ttl_secs() -> Result<u64, String> {
    ttl_secs(env_opt("FHIR_LOCO_EXPORT_TTL_SECS")?.as_deref())
}

fn text(name: &str, raw: Option<OsString>) -> Result<Option<String>, String> {
    match raw {
        None => Ok(None),
        Some(v) => v
            .into_string()
            .map(Some)
            .map_err(|_| format!("{name} is set but is not valid Unicode")),
    }
}

fn flag(name: &str, raw: Option<&str>) -> Result<bool, String> {
    match raw {
        None => Ok(false), // RCA-IN-3: absent only (an unset flag is off)
        Some("true" | "1") => Ok(true),
        Some("false" | "0") => Ok(false),
        Some(other) => Err(format!("{name} must be true, 1, false or 0, got {other:?}")),
    }
}

fn ttl_secs(raw: Option<&str>) -> Result<u64, String> {
    match raw {
        None => Ok(3600), // RCA-IN-3: absent only (export file lifetime, one hour)
        Some(v) => v.trim().parse::<u64>().map_err(|_| {
            format!("FHIR_LOCO_EXPORT_TTL_SECS must be a whole number of seconds, got {v:?}")
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttl_absent_is_the_default() {
        assert_eq!(ttl_secs(None), Ok(3600));
    }

    #[test]
    fn ttl_supplied_is_used_or_rejected_never_defaulted() {
        assert_eq!(ttl_secs(Some("0")), Ok(0));
        assert_eq!(ttl_secs(Some(" 60 ")), Ok(60));
        for bad in ["", "abc", "-1", "1.5", "99999999999999999999"] {
            let err = ttl_secs(Some(bad)).expect_err(bad);
            assert!(
                err.starts_with("FHIR_LOCO_EXPORT_TTL_SECS must be"),
                "{err}"
            );
        }
    }

    #[test]
    fn flag_absent_is_off_and_malformed_is_rejected() {
        assert_eq!(flag("X", None), Ok(false));
        assert_eq!(flag("X", Some("true")), Ok(true));
        assert_eq!(flag("X", Some("1")), Ok(true));
        assert_eq!(flag("X", Some("false")), Ok(false));
        assert_eq!(flag("X", Some("0")), Ok(false));
        for bad in ["", "yes", "TRUE", "2"] {
            let err = flag("X", Some(bad)).expect_err(bad);
            assert!(err.starts_with("X must be"), "{err}");
        }
    }

    #[test]
    fn a_setting_that_is_not_unicode_is_rejected() {
        assert_eq!(text("X", None), Ok(None));
        assert_eq!(text("X", Some("a".into())), Ok(Some("a".to_string())));
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let bad = OsString::from_vec(vec![0x66, 0xff]);
            assert_eq!(
                text("X", Some(bad)),
                Err("X is set but is not valid Unicode".to_string())
            );
        }
    }
}
