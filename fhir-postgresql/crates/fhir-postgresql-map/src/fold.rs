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

/// The checksum adjunct for an unbounded text column (`U1`, `U4`).
///
/// SHA-256 of the value's UTF-8 bytes, lowercase hex. Serves equality on an
/// engine that cannot `=` compare the source type.
///
/// **Computed in Rust, never by a SQL function** (`U4`). This is `L1`'s
/// argument in a second place: two implementations of "the same string" — one
/// in SQL, one here — have to agree for every codepoint in Unicode, or the
/// system quietly loses matches. One implementation cannot disagree with
/// itself.
///
/// Returned as the **32 raw bytes**, not hexadecimal text (`U4a`). Hex would
/// double the width of a column that exists to be indexed and compared, and
/// would invite the comparison to be written against a rendering rather than a
/// value — two encoders disagreeing on case give two texts for one digest,
/// which is `L1`'s failure in a new place.
///
/// The cost `U4a` accepts: this obliges every store to bind a byte-valued
/// parameter, and per-port binding of a new value type is where **F-20** was
/// found. `U4a` therefore requires a driver round-trip test wherever the column
/// is materialized.
///
/// The digest is **not** folded: it answers exact equality (`:exact`, token
/// match), where `fold` would erase the case and accents the caller asked to
/// keep. The bounded adjunct is the folded one (`U5`).
#[must_use]
pub fn digest(s: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().into()
}

/// The bounded adjunct for an unbounded text column (`U1`, `U5`).
///
/// The folded form, truncated to `n` characters — **characters, not bytes**, so
/// the bound means the same thing on every engine and cannot split a UTF-8
/// sequence.
///
/// `U5` requires the folded form so that a prefix search over the adjunct is
/// insensitive to case and accents exactly as one over `_norm` is. An adjunct
/// that folded differently from its source would be a third definition of
/// string identity, and two is already the number `L1` warns about.
///
/// Truncation is why `U7` exists: this column narrows, and the comparison
/// against the source column decides.
#[must_use]
pub fn bounded(s: &str, n: usize) -> String {
    fold(s).chars().take(n).collect()
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

#[cfg(test)]
mod adjunct_tests {
    use super::*;

    // U4: the digest is fixed-width, and it is over the *whole* value — two
    // strings agreeing in their first 450 characters must still differ.
    #[test]
    fn digest_is_fixed_width_and_covers_the_whole_value() {
        let a = "x".repeat(450) + "a";
        let b = "x".repeat(450) + "b";
        assert_eq!(digest(&a).len(), 32);
        assert_eq!(digest(&b).len(), 32);
        assert_ne!(
            digest(&a),
            digest(&b),
            "U2: a digest that ignored the tail would make equality wrong"
        );
        assert!(digest("").len() == 32, "empty string still digests");
    }

    // U4: computed over the same bytes everywhere. A known vector pins it, so
    // a change of hash function is a deliberate migration and not a silent one.
    // U4a: SHA-256, and 32 raw bytes rather than hex. A known vector pins
    // both the algorithm and the encoding, so changing either is a deliberate
    // migration rather than a silent one.
    #[test]
    fn digest_is_sha256_raw_bytes() {
        let d = digest("abc");
        assert_eq!(d.len(), 32, "U4a: 32 bytes, not 64 hex characters");
        assert_eq!(
            d,
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x00, 0x15, 0xad
            ]
        );
    }

    // U5: the bounded adjunct is the folded form, so a prefix search over it
    // is case- and accent-insensitive exactly as one over `_norm` is.
    #[test]
    fn bounded_is_folded() {
        assert_eq!(bounded("\u{c6}r\u{f8}", 450), fold("\u{c6}r\u{f8}"));
        assert_eq!(bounded("AERO", 450), bounded("aero", 450));
    }

    // U1: the bound counts characters, not bytes. Truncating UTF-8 by bytes
    // would split a sequence and produce an invalid string.
    #[test]
    fn bounded_truncates_by_characters_not_bytes() {
        // Deliberately a character the fold leaves multi-byte. An accented
        // Latin letter is the wrong probe: the fold decomposes and strips the
        // mark, leaving one ASCII byte per character, so byte-truncation and
        // character-truncation agree and the test cannot fail. Mutation
        // verification (T11.10) caught exactly that — this assertion passed
        // against a `bounded` that sliced by bytes until the probe changed.
        let s = "\u{5b57}".repeat(500); // CJK, 3 bytes each, unchanged by the fold
        let b = bounded(&s, 450);
        assert_eq!(
            b.chars().count(),
            450,
            "U1: the bound counts characters; slicing 450 *bytes* would keep 150"
        );
        assert!(b.is_char_boundary(b.len()));
    }

    // U2: the two adjuncts are not substitutes. Values sharing a 450-character
    // prefix collide in the bounded column and must not in the digest — this
    // is the case where a port emitting only `_idx` returns the wrong row.
    #[test]
    fn bounded_collides_where_digest_must_not() {
        let a = "y".repeat(450) + "1";
        let b = "y".repeat(450) + "2";
        assert_eq!(bounded(&a, 450), bounded(&b, 450), "they share the prefix");
        assert_ne!(
            digest(&a),
            digest(&b),
            "U2: the digest still separates them"
        );
    }
}
