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

/// Shorten a snake_case identifier under the limit: abbreviate the longest
/// segments to 4 characters until it fits, falling back to a hash suffix.
fn fit(full: &str) -> String {
    if full.len() <= PG_MAX_IDENT {
        return full.to_string();
    }
    let mut segs: Vec<String> = full.split('_').map(str::to_string).collect();
    loop {
        let total: usize = segs.iter().map(String::len).sum::<usize>() + segs.len() - 1;
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
