//! Property test: map-driven random resources round-trip losslessly.
//!
//! The generator walks the relational map itself and fabricates arbitrary
//! valid-shaped resources — deep recursion, sparse primitive arrays with
//! extensions, nested extensions, choice variants, high-precision decimals,
//! partial dates — probing shapes the official examples never exercise.
//!
//! FHIR_MSSQL_PROPTEST_CASES overrides the case count (default 500 locally;
//! CI runs 10000).

use std::collections::HashMap;

use fhir_mssql_map::model::{Elem, ElemKind, Prim, ResourceMap};
use fhir_mssql_map::reconstruct::{InRow, ReconIn, reconstruct};
use fhir_mssql_map::shred::{SqlVal, shred};
use serde_json::{Map, Value};

// ---------- deterministic rng (SplitMix64) ----------

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }

    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len() as u64) as usize]
    }
}

// ---------- generator ----------

struct Gen<'m> {
    rm: &'m ResourceMap,
    rng: Rng,
}

const STRINGS: &[&str] = &[
    "alpha",
    "Bénédicte du Marché",
    "line one\nline two",
    "  padded  ",
    "quote \" backslash \\ slash /",
    "日本語テキスト",
    "emoji 🩺",
    "<b>not markup</b>",
];
const DATES: &[&str] = &["2026", "2026-07", "2026-07-24", "1899-12-31", "0001-01-01"];
const DATETIMES: &[&str] = &[
    "2026",
    "2026-07",
    "2026-07-24",
    "2026-07-24T10:00:00Z",
    "2026-07-24T10:00:00.123+02:00",
    "2026-07-24T23:59:59.999999-05:00",
    "1970-01-01T00:00:00Z",
];
const DECIMALS: &[&str] = &[
    "0",
    "1.50",
    "-3.140",
    "0.00010",
    "123456789.987654321",
    "-0.5",
    "42",
];
const URLS: &[&str] = &[
    "http://example.org/ext/a",
    "http://example.org/ext/b",
    "http://hl7.org/fhir/StructureDefinition/patient-birthPlace",
];
const REFS: &[&str] = &[
    "Patient/p1",
    "Organization/org-42",
    "http://example.org/fhir/Patient/absolute",
    "#contained-1",
    "urn:uuid:9e3779b9-7f4a-7c15-0000-000000000000",
    "Patient/p1/_history/2",
];

impl<'m> Gen<'m> {
    fn prim_value(&mut self, prim: Prim) -> Value {
        match prim {
            Prim::Bool => Value::Bool(self.rng.chance(50)),
            Prim::Int => Value::Number(serde_json::Number::from(
                (self.rng.below(2_000_001) as i64) - 1_000_000,
            )),
            Prim::Int64 => Value::String(((self.rng.next() as i64) / 2).to_string()),
            Prim::Decimal => Value::Number(serde_json::Number::from_string_unchecked(
                (*self.rng.pick(DECIMALS)).to_string(),
            )),
            Prim::Date => Value::String((*self.rng.pick(DATES)).to_string()),
            Prim::DateTime | Prim::Instant => {
                Value::String((*self.rng.pick(DATETIMES)).to_string())
            }
            Prim::Time => Value::String("10:20:30.400".to_string()),
            Prim::Str => Value::String((*self.rng.pick(STRINGS)).to_string()),
        }
    }

    fn extension(&mut self, depth: usize) -> Value {
        let mut m = Map::new();
        m.insert(
            "url".to_string(),
            Value::String((*self.rng.pick(URLS)).to_string()),
        );
        if depth < 2 && self.rng.chance(30) {
            let n = 1 + self.rng.below(2);
            let exts: Vec<Value> = (0..n).map(|_| self.extension(depth + 1)).collect();
            m.insert("extension".to_string(), Value::Array(exts));
        } else {
            match self.rng.below(4) {
                0 => {
                    m.insert(
                        "valueString".to_string(),
                        Value::String((*self.rng.pick(STRINGS)).to_string()),
                    );
                }
                1 => {
                    m.insert(
                        "valueDecimal".to_string(),
                        Value::Number(serde_json::Number::from_string_unchecked(
                            (*self.rng.pick(DECIMALS)).to_string(),
                        )),
                    );
                }
                2 => {
                    m.insert("valueBoolean".to_string(), Value::Bool(self.rng.chance(50)));
                }
                _ => {
                    let mut cc = Map::new();
                    let mut coding = Map::new();
                    coding.insert(
                        "system".to_string(),
                        Value::String("http://loinc.org".to_string()),
                    );
                    coding.insert("code".to_string(), Value::String("1234-5".to_string()));
                    cc.insert(
                        "coding".to_string(),
                        Value::Array(vec![Value::Object(coding)]),
                    );
                    cc.insert(
                        "text".to_string(),
                        Value::String((*self.rng.pick(STRINGS)).to_string()),
                    );
                    m.insert("valueCodeableConcept".to_string(), Value::Object(cc));
                }
            }
        }
        Value::Object(m)
    }

    /// A primitive-extension object: id and/or extensions, never empty.
    fn prim_ext(&mut self) -> Value {
        let mut m = Map::new();
        if self.rng.chance(40) {
            m.insert(
                "id".to_string(),
                Value::String(format!("e{}", self.rng.below(1000))),
            );
        }
        if m.is_empty() || self.rng.chance(70) {
            let n = 1 + self.rng.below(2);
            let exts: Vec<Value> = (0..n).map(|_| self.extension(0)).collect();
            m.insert("extension".to_string(), Value::Array(exts));
        }
        Value::Object(m)
    }

    fn gen_elem(&mut self, elem: &Elem, depth: usize, out: &mut Map<String, Value>) {
        match &elem.kind {
            ElemKind::Prim(pc) => {
                if elem.repeats {
                    let n = 1 + self.rng.below(3) as usize;
                    let mut vals = Vec::with_capacity(n);
                    let mut pexts = Vec::with_capacity(n);
                    let mut any_val = false;
                    let mut any_ext = false;
                    for _ in 0..n {
                        let with_ext = self.rng.chance(25);
                        let null_val = with_ext && self.rng.chance(40);
                        if null_val {
                            vals.push(Value::Null);
                        } else {
                            vals.push(self.prim_value(pc.prim));
                            any_val = true;
                        }
                        if with_ext {
                            pexts.push(self.prim_ext());
                            any_ext = true;
                        } else {
                            pexts.push(Value::Null);
                        }
                    }
                    if any_val {
                        out.insert(elem.json.clone(), Value::Array(vals));
                    }
                    if any_ext {
                        out.insert(format!("_{}", elem.json), Value::Array(pexts));
                    }
                } else {
                    let with_ext = self.rng.chance(10);
                    let with_val = !with_ext || self.rng.chance(70);
                    if with_val {
                        out.insert(elem.json.clone(), self.prim_value(pc.prim));
                    }
                    if with_ext {
                        out.insert(format!("_{}", elem.json), self.prim_ext());
                    }
                }
            }
            ElemKind::RefStr(_) => {
                out.insert(
                    elem.json.clone(),
                    Value::String((*self.rng.pick(REFS)).to_string()),
                );
            }
            ElemKind::Group(node) => {
                if elem.repeats {
                    let n = 1 + self.rng.below(2) as usize;
                    let mut arr = Vec::new();
                    for _ in 0..n {
                        let m = self.gen_node(*node, depth + 1);
                        if !m.is_empty() {
                            arr.push(Value::Object(m));
                        }
                    }
                    if !arr.is_empty() {
                        out.insert(elem.json.clone(), Value::Array(arr));
                    }
                } else {
                    let m = self.gen_node(*node, depth + 1);
                    if !m.is_empty() {
                        out.insert(elem.json.clone(), Value::Object(m));
                    }
                }
            }
            ElemKind::Choice(variants) => {
                let var = variants[self.rng.below(variants.len() as u64) as usize].clone();
                self.gen_elem(&var, depth, out);
            }
            ElemKind::Spill => {
                let mut m = Map::new();
                m.insert(
                    "reference".to_string(),
                    Value::String((*self.rng.pick(REFS)).to_string()),
                );
                m.insert(
                    "display".to_string(),
                    Value::String((*self.rng.pick(STRINGS)).to_string()),
                );
                if elem.repeats {
                    out.insert(elem.json.clone(), Value::Array(vec![Value::Object(m)]));
                } else {
                    out.insert(elem.json.clone(), Value::Object(m));
                }
            }
            ElemKind::ResourceValue(_) => {
                let mut p = Map::new();
                p.insert(
                    "resourceType".to_string(),
                    Value::String("Patient".to_string()),
                );
                p.insert("id".to_string(), Value::String("inline".to_string()));
                p.insert("active".to_string(), Value::Bool(true));
                out.insert(elem.json.clone(), Value::Object(p));
            }
            ElemKind::Contained => {
                let mut c = Map::new();
                c.insert(
                    "resourceType".to_string(),
                    Value::String("Patient".to_string()),
                );
                c.insert("id".to_string(), Value::String("contained-1".to_string()));
                c.insert(
                    "birthDate".to_string(),
                    Value::String((*self.rng.pick(DATES)).to_string()),
                );
                out.insert(elem.json.clone(), Value::Array(vec![Value::Object(c)]));
            }
        }
    }

    fn gen_node(&mut self, node: u32, depth: usize) -> Map<String, Value> {
        let mut out = Map::new();
        if depth > 5 {
            return out;
        }
        // Occasional element id + extensions on complex elements.
        if depth > 0 && self.rng.chance(8) {
            out.insert(
                "id".to_string(),
                Value::String(format!("el{}", self.rng.below(100))),
            );
        }
        if depth > 0 && self.rng.chance(10) {
            let n = 1 + self.rng.below(2);
            let exts: Vec<Value> = (0..n).map(|_| self.extension(0)).collect();
            out.insert("extension".to_string(), Value::Array(exts));
        }
        let elems: Vec<Elem> = self.rm.node(node).elems.clone();
        // Presence probability decays with depth to bound resource size.
        let pct = match depth {
            0 => 35,
            1 => 25,
            2 => 15,
            _ => 8,
        };
        for elem in &elems {
            if self.rng.chance(pct) {
                self.gen_elem(elem, depth, &mut out);
            }
        }
        out
    }
}

fn gen_resource(rm: &ResourceMap, seed: u64) -> Value {
    let mut g = Gen { rm, rng: Rng(seed) };
    let mut body = g.gen_node(rm.root, 0);
    let mut out = Map::new();
    out.insert("resourceType".to_string(), Value::String(rm.name.clone()));
    out.insert("id".to_string(), Value::String(format!("prop-{seed}")));
    // Resource-level extensions.
    if g.rng.chance(30) {
        let exts: Vec<Value> = (0..1 + g.rng.below(2)).map(|_| g.extension(0)).collect();
        out.insert("extension".to_string(), Value::Array(exts));
    }
    out.append(&mut body);
    Value::Object(out)
}

// ---------- harness ----------

fn to_recon(rm: &ResourceMap, out: &fhir_mssql_map::ShredOut) -> ReconIn {
    let mut tables: Vec<Vec<InRow>> = vec![Vec::new(); rm.tables.len()];
    for row in &out.rows {
        let mut cols = HashMap::new();
        for (name, val) in &row.cols {
            let text = match val {
                SqlVal::Bool(b) => b.to_string(),
                SqlVal::Int(n) => n.to_string(),
                SqlVal::Num(s) | SqlVal::Text(s) | SqlVal::Ts(s) | SqlVal::Date(s) => s.clone(),
                SqlVal::Jsonb(s) => s.clone(),
                // U3: adjuncts are derived and the reconstructor must never
                // read them. Skipping here is not a convenience — it is the
                // assertion: if a Bytes column ever reached reconstruction,
                // round-trip would be deciding on a value the resource never
                // contained.
                SqlVal::Bytes(_) => continue,
            };
            cols.insert(name.clone(), text);
        }
        tables[row.table as usize].push(InRow {
            ords: row.ords.clone(),
            cols,
        });
    }
    ReconIn {
        tables,
        ext: out.ext.clone(),
        deep: out.deep.clone(),
        contained: out.contained.clone(),
    }
}

fn sem_eq(a: &Value, b: &Value, path: &str, diffs: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ma), Value::Object(mb)) => {
            for (k, va) in ma {
                match mb.get(k) {
                    Some(vb) => sem_eq(va, vb, &format!("{path}.{k}"), diffs),
                    None => diffs.push(format!("{path}.{k}: missing")),
                }
            }
            for k in mb.keys() {
                if !ma.contains_key(k) {
                    diffs.push(format!("{path}.{k}: extra"));
                }
            }
        }
        (Value::Array(aa), Value::Array(ab)) => {
            if aa.len() != ab.len() {
                diffs.push(format!("{path}: length {} vs {}", aa.len(), ab.len()));
                return;
            }
            for (i, (va, vb)) in aa.iter().zip(ab).enumerate() {
                sem_eq(va, vb, &format!("{path}[{i}]"), diffs);
            }
        }
        _ => {
            if a != b {
                diffs.push(format!("{path}: {a} vs {b}"));
            }
        }
    }
}

#[test]
fn property_roundtrip_random_resources() {
    let spec = std::env::var("FHIR_MSSQL_SPEC_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../fhir/doc/fhir-specifications"
            ))
        });
    let defs = spec.join("r5").join("fhir-definitions-json");
    if !defs.exists() {
        eprintln!("skipping: no spec dir");
        return;
    }
    let cases: u64 = std::env::var("FHIR_MSSQL_PROPTEST_CASES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500);
    let map = fhir_mssql_gen::generate(&defs, "r5").expect("generate");
    // Recursion, wide choices, references, contained, narratives, and the
    // Bundle inline-resource path all get coverage from this set.
    let types = [
        "Patient",
        "Observation",
        "Questionnaire",
        "QuestionnaireResponse",
        "Task",
        "Bundle",
        "CapabilityStatement",
        "MedicationRequest",
    ];
    let mut failures = Vec::new();
    for seed in 0..cases {
        let rt = types[(seed % types.len() as u64) as usize];
        let rm = map.resources.get(rt).expect("type");
        let v = gen_resource(rm, seed.wrapping_mul(0x2545F4914F6CDD1D) + seed);
        let out = match shred(rm, &v) {
            Ok(o) => o,
            Err(e) => {
                failures.push(format!("seed {seed} ({rt}): shred: {e}"));
                continue;
            }
        };
        let rin = to_recon(rm, &out);
        let back = match reconstruct(rm, &rin, out.id.as_deref()) {
            Ok(b) => b,
            Err(e) => {
                failures.push(format!("seed {seed} ({rt}): reconstruct: {e}"));
                continue;
            }
        };
        let mut diffs = Vec::new();
        sem_eq(&v, &back, "$", &mut diffs);
        if !diffs.is_empty() {
            diffs.truncate(3);
            failures.push(format!("seed {seed} ({rt}): {}", diffs.join(" | ")));
        }
        if failures.len() > 10 {
            break;
        }
    }
    assert!(
        failures.is_empty(),
        "property round-trip failures:\n{}",
        failures.join("\n")
    );
}
