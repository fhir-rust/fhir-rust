//! `G2.6a` (**F-90**): no table in a shipped map asset may charge past
//! InnoDB's create-time row-size check, as the mysql/mariadb dialects would
//! render it — the tightest engine any port targets. The generator asserts
//! this over what it builds; this test asserts it over the *bundled* assets,
//! so the gate is on the artifact (`G2.2`), not on whichever generator build
//! happens to run.
//!
//! The 8126-byte refusal and the ~41-byte TEXT charge were measured by
//! bisection against MySQL 8.4 (195 TEXT columns install, 196 do not); the
//! margin below the measured limit is deliberate, mirroring the generator's
//! own budget.

use fhir_mssql_map::RelMap;
use fhir_mssql_map::model::{ColTy, TableKind};

/// One column's charge, as `build.rs::row_charge` counts it. Duplicated
/// here deliberately: weakening the generator's model and this test's model
/// would take two edits, not one.
fn charge(ty: ColTy) -> usize {
    match ty {
        ColTy::Bool => 1,
        ColTy::Int => 4,
        ColTy::BigInt => 8,
        ColTy::Date => 3,
        ColTy::Timestamptz => 8,
        ColTy::Numeric | ColTy::Text | ColTy::TextC | ColTy::Jsonb => 41,
        ColTy::TextIdx | ColTy::Digest => 0,
    }
}

fn fixed(kind: TableKind) -> usize {
    match kind {
        TableKind::Base => 274,
        TableKind::Elem => 515,
        _ => 0,
    }
}

#[test]
fn bundled_assets_fit_the_innodb_row_budget() {
    const BUDGET: usize = 7900;
    for version in RelMap::bundled_versions() {
        let map = RelMap::bundled(version).expect("bundled map decodes");
        let mut worst = (0usize, String::new());
        for rm in map.resources.values() {
            for t in &rm.tables {
                let c: usize =
                    fixed(t.kind) + t.cols.iter().map(|c| charge(c.ty)).sum::<usize>();
                if c > worst.0 {
                    worst = (c, t.name.clone());
                }
                assert!(
                    c <= BUDGET,
                    "{version}: table {} charges {c} bytes (budget {BUDGET}, G2.6a/F-90)",
                    t.name
                );
            }
        }
        println!(
            "{version}: widest table {} charges {} bytes",
            worst.1, worst.0
        );
    }
}
