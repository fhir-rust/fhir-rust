//! Identifier construction: snake_case conversion and deterministic fitting
//! into PostgreSQL's 63-byte identifier limit.

use std::collections::HashSet;

use sha2::{Digest, Sha256};

pub const PG_MAX_IDENT: usize = 63;

/// camelCase / PascalCase → snake_case.
pub fn snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Uppercase the first character (choice-variant JSON names).
pub fn ucfirst(s: &str) -> String {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
        None => String::new(),
    }
}

/// A per-scope identifier registry: fits names under the limit
/// deterministically and guarantees uniqueness within the scope.
#[derive(Debug, Default)]
pub struct Registry {
    used: HashSet<String>,
}

impl Registry {
    /// A registry that already considers `names` taken. Used by passes that
    /// add columns to a table built earlier, so they cannot reuse a name.
    pub fn seeded<'a>(names: impl Iterator<Item = &'a str>) -> Self {
        Self {
            used: names.map(str::to_string).collect(),
        }
    }

    pub fn claim(&mut self, full: &str) -> String {
        let name = fit(full);
        let name = if self.used.contains(&name) {
            hashed(full)
        } else {
            name
        };
        assert!(
            self.used.insert(name.clone()),
            "identifier collision even after hashing: {full}"
        );
        name
    }
}

/// Fallback for [`joined_len`] (RCA-NONE-3): `usize::MAX`.
///
/// Why it is correct: a length that does not fit a `usize` is longer than
/// `PG_MAX_IDENT`, and `usize::MAX` is too, so `fit` keeps abbreviating exactly
/// as it would for the true length.
const JOINED_LEN_OVERFLOW: usize = usize::MAX;

/// Length of `segs` joined by one separator byte each.
fn joined_len(segs: &[String]) -> usize {
    // Fallback: 0 separators for no segments; `split` yields at least one.
    #[expect(
        clippy::manual_saturating_arithmetic,
        reason = "RCA-INT-4: checked_sub with a declared 0 fallback is the required form"
    )]
    let separators = segs.len().checked_sub(1).unwrap_or(0);
    segs.iter()
        .map(String::len)
        .try_fold(separators, usize::checked_add)
        .unwrap_or(JOINED_LEN_OVERFLOW) // JOINED_LEN_OVERFLOW: longer than any limit
}

/// Shorten a snake_case identifier under the limit: abbreviate the longest
/// segments to 4 characters until it fits, falling back to a hash suffix.
fn fit(full: &str) -> String {
    if full.len() <= PG_MAX_IDENT {
        return full.to_string();
    }
    let mut segs: Vec<String> = full.split('_').map(str::to_string).collect();
    loop {
        let total = joined_len(&segs);
        if total <= PG_MAX_IDENT {
            return segs.join("_");
        }
        // Deterministic: abbreviate the longest still-abbreviatable segment,
        // earliest first on ties.
        let Some(i) = segs
            .iter()
            .enumerate()
            .filter(|(_, s)| s.len() > 4)
            .max_by(|(ai, a), (bi, b)| a.len().cmp(&b.len()).then(bi.cmp(ai)))
            .map(|(i, _)| i)
        else {
            return hashed(full);
        };
        segs[i].truncate(4);
    }
}

fn hashed(full: &str) -> String {
    let mut h = Sha256::new();
    h.update(full.as_bytes());
    let digest = h.finalize();
    let hex: String = digest[..4].iter().map(|b| format!("{b:02x}")).collect();
    let keep = PG_MAX_IDENT - hex.len() - 1;
    let head: String = full.chars().take(keep).collect();
    format!("{head}_{hex}")
}

#[cfg(test)]
mod joined_len_tests {
    use super::*;

    #[test]
    fn joined_len_counts_bytes_and_separators() {
        let segs: Vec<String> = ["ab", "cde"].iter().map(|s| s.to_string()).collect();
        assert_eq!(joined_len(&segs), 6);
        assert_eq!(joined_len(&[]), 0);
        assert_eq!(joined_len(&["x".to_string()]), 1);
    }

    #[test]
    fn joined_len_is_over_any_limit_when_it_overflows() {
        // A `String` cannot be `usize::MAX` long; the fallback is exercised
        // through the fold's own type instead.
        assert_eq!(
            [usize::MAX, 1]
                .into_iter()
                .try_fold(0_usize, usize::checked_add)
                .unwrap_or(JOINED_LEN_OVERFLOW),
            JOINED_LEN_OVERFLOW
        );
        assert_eq!(JOINED_LEN_OVERFLOW, usize::MAX);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_cases() {
        assert_eq!(snake("birthDate"), "birth_date");
        assert_eq!(snake("CodeableConcept"), "codeable_concept");
        assert_eq!(snake("base64Binary"), "base64_binary");
        assert_eq!(
            snake("MedicinalProductDefinition"),
            "medicinal_product_definition"
        );
    }

    #[test]
    fn fits() {
        let mut r = Registry::default();
        assert_eq!(r.claim("patient_name"), "patient_name");
        let long = "medicinal_product_definition_name_country_language_jurisdiction_coding";
        let fitted = r.claim(long);
        assert!(fitted.len() <= PG_MAX_IDENT, "{fitted}");
        // Deterministic.
        let mut r2 = Registry::default();
        assert_eq!(r2.claim(long), fitted);
    }

    #[test]
    fn collision_gets_hash() {
        let mut r = Registry::default();
        let a = r.claim("x_page");
        let b = r.claim("x_page");
        assert_ne!(a, b);
        assert!(b.len() <= PG_MAX_IDENT);
    }
}
