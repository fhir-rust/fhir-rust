//! Search folding for FHIR `string` parameters (P6.6).
//!
//! FHIR requires `string` search to be case-insensitive **and**
//! accent-insensitive, worldwide. Doing that at query time means either an
//! expression index (which a parameterised `LIKE` will not use) or folding in
//! SQL and in Rust, two implementations that must agree for every codepoint.
//!
//! Instead the engine folds once, in Rust, at write time, into a companion
//! `_norm` column. Queries fold the search term with the *same* function and
//! compare against that column, so there is exactly one definition of "the
//! same string" in the system. The column is declared `COLLATE "C"` so that
//! ordering is by Unicode codepoint, which is what makes [`prefix_upper`]
//! sound as a range scan.

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

/// Fold a string for accent- and case-insensitive comparison.
///
/// Decomposes (NFD), drops combining marks, lowercases, then expands the
/// letters that decomposition cannot reach. Lowercasing can itself introduce
/// marks — Turkish `İ` lowercases to `i` plus a combining dot above — so marks
/// are stripped again before the expansion. The result is idempotent:
/// `fold(fold(s)) == fold(s)`, because nothing [`expand`] produces is itself
/// expandable.
pub fn fold(s: &str) -> String {
    let stripped: String = s.nfd().filter(|c| !is_combining_mark(*c)).collect();
    let lowered: String = stripped
        .to_lowercase()
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect();
    let mut out = String::with_capacity(lowered.len());
    for c in lowered.chars() {
        match expand(c) {
            Some(sub) => out.push_str(sub),
            None => out.push(c),
        }
    }
    out
}

/// Letters that carry no combining mark, so NFD leaves them whole.
///
/// This is the half of accent folding that decomposition cannot do. `å` is
/// `a` plus a combining ring, so stripping marks handles it; `ø` is a single
/// codepoint with a stroke through it and no decomposition at all, and neither
/// is `æ`, `ł`, `đ` or `ß`. Without this step a search for `aero` does not find
/// `Ærø` — and that is the example this module's own reasoning is built on: in a
/// system serving Ærø, Muñoz and Ślusarczyk, it is a patient not found rather
/// than a cosmetic difference.
///
/// The mappings follow PostgreSQL's `unaccent` rules so that a folded value
/// means the same thing whichever engine stores it. Multi-character expansions
/// (`æ` → `ae`, `ß` → `ss`, `þ` → `th`) are why this returns a string rather
/// than a character: a fold that only ever substituted one character for one
/// character could not express them.
///
/// Deliberately not included: scripts where a Latin transliteration would be a
/// guess rather than a fold. Greek, Cyrillic and CJK pass through unchanged,
/// because "the same string" has to stay a property of the text and not of a
/// romanisation policy.
fn expand(c: char) -> Option<&'static str> {
    Some(match c {
        'æ' => "ae",
        'œ' => "oe",
        'ø' => "o",
        'đ' | 'ð' => "d",
        'ł' => "l",
        'ß' => "ss",
        'þ' => "th",
        'ħ' => "h",
        'ŋ' => "n",
        'ŧ' => "t",
        'ĸ' => "k",
        // Dotless i, which lowercasing a Turkish `I` can leave behind.
        'ı' => "i",
        _ => return None,
    })
}

/// The least string strictly greater than every string having `prefix` as a
/// prefix, under codepoint order — or `None` when no such string exists
/// (empty prefix, or a prefix of all `char::MAX`), meaning the range is
/// unbounded above.
///
/// This turns a prefix match into `col >= prefix AND col < upper`, a plain
/// btree range scan. The planner does not have to recognise a `LIKE` pattern,
/// so it works with a bound parameter under a generic plan — which is exactly
/// where `LIKE $1` silently falls back to a sequential scan.
///
/// Codepoint order equals UTF-8 byte order, so a `COLLATE "C"` index on the
/// folded column orders the same way this function assumes.
pub fn prefix_upper(prefix: &str) -> Option<String> {
    let mut chars: Vec<char> = prefix.chars().collect();
    while let Some(last) = chars.pop() {
        if let Some(next) = next_char(last) {
            chars.push(next);
            return Some(chars.into_iter().collect());
        }
        // `last` is char::MAX: nothing at this position sorts higher, so carry
        // by dropping it and incrementing the position before it.
    }
    None
}

/// The next scalar value after `c`, skipping the surrogate range, or `None`
/// at `char::MAX`.
fn next_char(c: char) -> Option<char> {
    let mut n = c as u32 + 1;
    if n == 0xD800 {
        n = 0xE000;
    }
    char::from_u32(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_that_do_not_decompose_are_still_folded() {
        // The case this module's own reasoning is built on. `å` already worked,
        // because it is `a` plus a combining ring; `ø` and `æ` are single
        // codepoints with no decomposition, so mark-stripping never touched
        // them and a search for "aero" missed "Ærø" entirely.
        assert_eq!(fold("Ærø"), "aero");
        assert_eq!(fold("Ångström"), "angstrom");
        assert_eq!(fold("Łódź"), "lodz");
        assert_eq!(fold("Đorđe"), "dorde");
        assert_eq!(fold("Straße"), "strasse");
        assert_eq!(fold("Þórður"), "thordur");
        assert_eq!(fold("Œuvre"), "oeuvre");
    }

    #[test]
    fn the_cases_that_already_worked_still_do() {
        // These decompose, so they were never broken — but a change to the
        // fold could easily break them, and the whole point is one definition
        // of "the same string".
        assert_eq!(fold("Muñoz"), "munoz");
        assert_eq!(fold("Müller"), "muller");
        assert_eq!(fold("Ślusarczyk"), "slusarczyk");
    }

    #[test]
    fn expansion_keeps_the_fold_idempotent() {
        // `fold(fold(s)) == fold(s)` is load-bearing: the stored `_norm` value
        // is folded once at write time and the search term folded again at
        // query time, so a fold that changed on a second pass would stop
        // matching its own output. Nothing `expand` produces is expandable.
        for s in ["Ærø", "Straße", "Þórður", "Œuvre", "Łódź", "Ångström"] {
            let once = fold(s);
            assert_eq!(fold(&once), once, "not idempotent for {s}");
        }
    }

    #[test]
    fn non_latin_scripts_fold_but_are_never_romanised() {
        // Greek accents are combining marks, so they strip like any other —
        // that is accent-insensitive search working, not a bug: `ό` becomes `ο`.
        assert_eq!(fold("Ασκληπιός"), "ασκληπιος");
        // Cyrillic folds for the same reason: `й` is `и` plus a combining
        // breve, so it strips to `и`. Consistent across scripts, which is the
        // property that matters — one definition of "the same string".
        assert_eq!(fold("Достоевский"), "достоевскии");
        // CJK has no marks to strip and is left exactly alone.
        assert_eq!(fold("中文"), "中文");
        // What must never happen is transliteration: that would be a
        // romanisation policy rather than a fold, and "the same string" has to
        // stay a property of the text.
        assert!(
            fold("Достоевский")
                .chars()
                .all(|c| !c.is_ascii_alphabetic())
        );
    }

    #[test]
    fn folds_case_and_accents() {
        assert_eq!(fold("MÜLLER"), "muller");
        assert_eq!(fold("Müller"), fold("Muller"));
        assert_eq!(fold("José"), "jose");
        assert_eq!(fold("ÅNGSTRÖM"), "angstrom");
        // Precomposed and decomposed spellings must fold alike; this is the
        // case an ILIKE comparison gets wrong.
        assert_eq!(fold("é"), fold("e\u{301}"));
    }

    #[test]
    fn folds_beyond_latin() {
        assert_eq!(fold("ΑΘΉΝΑ"), "αθηνα");
        assert_eq!(fold("ЙОСИФ"), "иосиф");
        // Scripts without case or marks pass through unchanged.
        assert_eq!(fold("東京"), "東京");
        assert_eq!(fold("مُحَمَّد"), "محمد");
    }

    #[test]
    fn fold_is_idempotent() {
        for s in ["MÜLLER", "José", "ΑΘΉΝΑ", "İstanbul", "e\u{301}", ""] {
            assert_eq!(fold(&fold(s)), fold(s), "not idempotent: {s:?}");
        }
    }

    #[test]
    fn turkish_dotted_i_loses_its_mark() {
        // to_lowercase('İ') yields "i\u{307}"; the second strip removes it.
        assert_eq!(fold("İstanbul"), "istanbul");
    }

    #[test]
    fn prefix_upper_bounds_the_range() {
        assert_eq!(prefix_upper("abc").unwrap(), "abd");
        assert_eq!(prefix_upper("ab\u{10FFFF}").unwrap(), "ac");
        assert_eq!(prefix_upper(""), None);
        assert_eq!(prefix_upper("\u{10FFFF}"), None);
        // Never lands inside the surrogate gap.
        assert_eq!(prefix_upper("\u{D7FF}").unwrap(), "\u{E000}");
    }

    #[test]
    fn prefix_upper_excludes_exactly_the_non_matches() {
        let prefix = "mul";
        let upper = prefix_upper(prefix).unwrap();
        let in_range = |s: &str| s >= prefix && s < upper.as_str();
        for s in ["mul", "muller", "mulz", "mul\u{10FFFF}"] {
            assert!(in_range(s), "{s:?} should be in range");
        }
        for s in ["mu", "mum", "mv", "n", "mula".trim_end_matches("mula")] {
            assert!(!in_range(s) || s.starts_with(prefix), "{s:?} leaked in");
        }
    }
}
