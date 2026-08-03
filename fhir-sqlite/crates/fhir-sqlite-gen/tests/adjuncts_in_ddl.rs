//! `U2b`/**F-46**: every adjunct the map names must exist in the emitted DDL.
//!
//! The map is the authority the store reads to decide what to bind and what to
//! query. If it names a column the schema does not have, every use of that
//! column is a runtime error on a path no unit test reaches; if the schema has
//! one the map does not name, nothing will ever read it.
//!
//! Both halves have happened here. `add_adjunct_columns` attached adjuncts to
//! the extension and deep tables while `create_table` hardcoded their columns
//! and emitted none of them — the map over-promised. The first attempt to fix
//! that went the other way, describing `path` as a bounded type while the DDL
//! emitted an unbounded one.
//!
//! So this asserts the agreement directly, over every table of every resource,
//! rather than trusting either side's tests.

use fhir_sqlite_gen::assets;
use fhir_sqlite_map::ddl;

#[test]
fn every_adjunct_column_in_the_map_is_emitted_by_the_ddl() {
    let Some(defs) = assets::definitions_root() else {
        eprintln!(
            "SKIPPING every_adjunct_column_in_the_map_is_emitted_by_the_ddl: \
             no FHIR definitions. Set {} to run it.",
            assets::ENV_SPEC_DIR
        );
        return;
    };
    // `definitions_root` is the root of all three releases; `generate` wants
    // one release's JSON, the same join `assets::regenerate` does.
    let defs = defs.join("r5").join("fhir-definitions-json");
    if !defs.exists() {
        eprintln!(
            "SKIPPING every_adjunct_column_in_the_map_is_emitted_by_the_ddl: {} is absent",
            defs.display()
        );
        return;
    }
    let map = fhir_sqlite_gen::generate(&defs, "r5").expect("generate r5");

    let mut checked = 0usize;
    for rm in map.resources.values() {
        for t in &rm.tables {
            let sql = ddl::create_table("r5", rm, t);
            for a in &t.adjunct_cols {
                // The source column must exist too: an adjunct over a column
                // that is not there is the same defect one step earlier.
                assert!(
                    t.cols.iter().any(|c| c.name == a.source),
                    "{}.{}: adjunct source column is not in the table",
                    t.name,
                    a.source
                );
                for name in [a.bounded.as_ref(), a.digest.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    assert!(
                        names_column(&sql, name),
                        "{}: map names adjunct column `{name}` but create_table \
                         does not emit it.\n{sql}",
                        t.name
                    );
                    checked += 1;
                }
            }
        }
    }

    // U9 makes zero the correct answer on a dialect that indexes its unbounded
    // text type, so an empty run is not a failure — but it must be visible,
    // because a silently empty assertion loop reads exactly like a pass
    // (`T11.12`).
    eprintln!(
        "checked {checked} adjunct column(s); TEXT_ADJUNCTS={}",
        ddl::TEXT_ADJUNCTS
    );
    assert_eq!(
        checked > 0,
        ddl::TEXT_ADJUNCTS,
        "a dialect with TEXT_ADJUNCTS={} produced {checked} adjunct column(s)",
        ddl::TEXT_ADJUNCTS
    );
}

/// Does this DDL define a column with exactly this name?
///
/// A plain `contains` would accept `url_h` on the strength of a `url_hash`
/// somewhere in the statement, which is the kind of near-miss that makes a test
/// look stronger than it is. Identifier quoting differs per dialect, so rather
/// than encode six sets of delimiters this just requires the match not to be
/// part of a longer identifier.
fn names_column(sql: &str, name: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    sql.match_indices(name).any(|(i, _)| {
        let before = sql[..i].chars().next_back().is_none_or(|c| !word(c));
        let after = sql[i + name.len()..]
            .chars()
            .next()
            .is_none_or(|c| !word(c));
        before && after
    })
}
