//! Utility code for this crate.

/// Given a a string, return its first word.
///
/// This implementation does a split on non-alphanumeric characters.
///
/// Example:
///
/// ```
/// use fhir::util::first_word;
/// assert_eq!(first_word("alfa"), "alfa");
/// assert_eq!(first_word("alfa bravo"), "alfa");
/// assert_eq!(first_word("alfa bravo charlie"), "alfa");
/// ```
///
#[allow(dead_code)]
pub fn first_word(s: &str) -> &str {
    s.split(|c: char| !(c.is_alphanumeric()))
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
}

#[cfg(test)]
mod test_first_word {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(first_word("alfa"), "alfa");
        assert_eq!(first_word("alfa bravo"), "alfa");
        assert_eq!(first_word("alfa bravo charlie"), "alfa");
    }
}

/// Given a a string, return its last word.
///
/// This implementation does a split on non-alphanumeric characters.
///
/// Example:
///
/// ```
/// use fhir::util::last_word;
/// assert_eq!(last_word("alfa"), "alfa");
/// assert_eq!(last_word("alfa bravo"), "bravo");
/// assert_eq!(last_word("alfa bravo charlie"), "charlie");
/// ```
///
#[allow(dead_code)]
pub fn last_word(s: &str) -> &str {
    s.rsplit(|c: char| !(c.is_alphanumeric()))
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
}

#[cfg(test)]
mod test_last_word {
    use super::*;
    #[test]
    fn test_last_word() {
        assert_eq!(last_word("alfa"), "alfa");
        assert_eq!(last_word("alfa bravo"), "bravo");
        assert_eq!(last_word("alfa bravo charlie"), "charlie");
    }
}
