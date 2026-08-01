//! Map construction: walk each resource's element definitions, expanding
//! complex datatypes in place, and decide for every element whether it
//! flattens into the current table, opens a child table, spills (type
//! cycles), or routes to the extension machinery.

use std::collections::HashMap;

use fhir_oracle_map::model::{
    ColTy, Column, Elem, ElemKind, Node, Prim, PrimCol, RefCols, RelMap, ResourceMap, Table,
    TableKind,
};

use crate::GenError;
use crate::names::{Registry, snake, ucfirst};
use crate::spec::{Def, Spec, SpecElem};

/// A flattened expansion wider than this many columns is forced into its own
/// table, bounding every table well below PostgreSQL's 1600-column limit.
const SPLIT_WIDTH: usize = 150;

pub fn build_map(spec: &Spec, schema: &str) -> Result<RelMap, GenError> {
    let mut table_reg = Registry::default();
    let mut resources = std::collections::BTreeMap::new();
    let mut width_cache: HashMap<String, usize> = HashMap::new();
    for def in spec.resources.values() {
        let cyclic_targets = def
            .elems
            .iter()
            .filter_map(|e| {
                let t = e.content_ref.as_ref()?;
                e.path.starts_with(&format!("{t}.")).then(|| t.clone())
            })
            .collect();
        let rm = ResourceBuilder {
            spec,
            root_def: def,
            table_reg: &mut table_reg,
            width_cache: &mut width_cache,
            tables: Vec::new(),
            col_regs: Vec::new(),
            nodes: Vec::new(),
            ref_ctx: HashMap::new(),
            cyclic_targets,
            cyclic_lanes: HashMap::new(),
        }
        .build()?;
        resources.insert(def.name.clone(), rm);
    }
    Ok(RelMap {
        fhir_version: spec.fhir_version.clone(),
        schema: schema.to_string(),
        resources,
    })
}

struct RefBind {
    node: u32,
    table: Option<u32>,
    in_progress: bool,
}

struct ResourceBuilder<'s> {
    spec: &'s Spec,
    root_def: &'s Def,
    table_reg: &'s mut Registry,
    width_cache: &'s mut HashMap<String, usize>,
    tables: Vec<Table>,
    col_regs: Vec<Registry>,
    nodes: Vec<Node>,
    /// Definition path → built binding, for contentReference resolution.
    ref_ctx: HashMap<String, RefBind>,
    /// Backbone paths referenced cyclically from inside their own subtree;
    /// these must own a table even when they do not repeat.
    cyclic_targets: std::collections::HashSet<String>,
    /// Per-target count of cyclic referrers, for lane assignment.
    cyclic_lanes: HashMap<String, u32>,
}

impl<'s> ResourceBuilder<'s> {
    fn build(mut self) -> Result<ResourceMap, GenError> {
        let rname = self.root_def.name.clone();
        let base_name = self.table_reg.claim(&snake(&rname));
        self.tables.push(Table {
            norm_cols: Vec::new(),
            name: base_name.clone(),
            kind: TableKind::Base,
            path: rname.clone(),
            cols: Vec::new(),
        });
        let mut base_reg = Registry::default();
        for reserved in ["id", "version_id", "last_updated"] {
            base_reg.claim(reserved);
        }
        self.col_regs.push(base_reg);

        let root = self.alloc_node();
        let mut stack: Vec<String> = vec![rname.clone()];
        self.build_children(root, self.root_def, &rname, 0, "", &rname, &mut stack)?;

        for (suffix, kind) in [
            ("_ext", TableKind::Ext),
            ("_deep", TableKind::Deep),
            ("_contained", TableKind::Contained),
            ("_history", TableKind::History),
        ] {
            let name = self.table_reg.claim(&format!("{base_name}{suffix}"));
            self.tables.push(Table {
                norm_cols: Vec::new(),
                name,
                kind,
                path: String::new(),
                cols: Vec::new(),
            });
            self.col_regs.push(Registry::default());
        }

        for t in &self.tables {
            if t.cols.len() > 1500 {
                return Err(GenError::Build(format!(
                    "table {} has {} columns; raise the split threshold",
                    t.name,
                    t.cols.len()
                )));
            }
        }
        Ok(ResourceMap {
            name: rname,
            tables: self.tables,
            nodes: self.nodes,
            root,
            search: Vec::new(),
        })
    }

    fn alloc_node(&mut self) -> u32 {
        self.nodes.push(Node { elems: Vec::new() });
        (self.nodes.len() - 1) as u32
    }

    fn new_table(&mut self, parent_table: u32, col_base: &str, res_path: &str) -> u32 {
        let parent = &self.tables[parent_table as usize].name;
        let name = self.table_reg.claim(&format!("{parent}_{col_base}"));
        self.tables.push(Table {
            norm_cols: Vec::new(),
            name,
            kind: TableKind::Elem,
            path: res_path.to_string(),
            cols: Vec::new(),
        });
        let mut reg = Registry::default();
        for reserved in ["rid", "ords"] {
            reg.claim(reserved);
        }
        self.col_regs.push(reg);
        (self.tables.len() - 1) as u32
    }

    fn add_col(&mut self, table: u32, name: &str, ty: ColTy, path: &str) -> String {
        let claimed = self.col_regs[table as usize].claim(name);
        self.tables[table as usize].cols.push(Column {
            name: claimed.clone(),
            ty,
            path: path.to_string(),
        });
        claimed
    }

    /// Build the elements of `def_path` (within `def`) into `node`, writing
    /// columns into `table` under `prefix`.
    #[allow(clippy::too_many_arguments)]
    fn build_children(
        &mut self,
        node: u32,
        def: &'s Def,
        def_path: &str,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<(), GenError> {
        for &i in def.kids(def_path) {
            let e = &def.elems[i];
            if e.omitted || e.name == "id" {
                continue;
            }
            if e.types.iter().any(|t| t == "Extension") {
                continue;
            }
            if e.name == "contained" && e.types.iter().any(|t| t == "Resource") {
                let elem = Elem {
                    json: "contained".to_string(),
                    path: e.path.clone(),
                    repeats: true,
                    table: None,
                    neg_lane: false,
                    kind: ElemKind::Contained,
                };
                self.nodes[node as usize].elems.push(elem);
                continue;
            }
            let child_res_path = format!("{res_path}.{}", e.name);
            let elem = self.build_elem(e, def, table, prefix, &child_res_path, stack)?;
            self.nodes[node as usize].elems.push(elem);
        }
        Ok(())
    }

    fn build_elem(
        &mut self,
        e: &'s SpecElem,
        def: &'s Def,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        // Backbone: children defined in place take precedence over the
        // BackboneElement/Element type code.
        if !def.kids(&e.path).is_empty() && e.content_ref.is_none() && !e.choice {
            return self.build_backbone(e, def, table, prefix, res_path, stack);
        }
        if let Some(target) = &e.content_ref {
            return self.build_content_ref(e, target, table, prefix, res_path, stack);
        }
        if e.choice {
            return self.build_choice(e, table, prefix, res_path, stack);
        }
        let [ty] = e.types.as_slice() else {
            return Err(GenError::Build(format!(
                "{}: expected exactly one type, found {:?}",
                e.path, e.types
            )));
        };
        self.build_typed(
            e.name.clone(),
            &e.path,
            e.repeats,
            ty,
            table,
            prefix,
            res_path,
            stack,
        )
    }

    fn build_backbone(
        &mut self,
        e: &'s SpecElem,
        def: &'s Def,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        let col_base = format!("{prefix}{}", snake(&e.name));
        let split = e.repeats
            || self.cyclic_targets.contains(&e.path)
            || self.width_children(def, &e.path, stack) > SPLIT_WIDTH;
        let node = self.alloc_node();
        let (t, new_prefix): (Option<u32>, String) = if split {
            let t = self.new_table(table, &col_base, res_path);
            (Some(t), String::new())
        } else {
            (None, format!("{col_base}_"))
        };
        let register = std::ptr::eq(def as *const _, self.root_def as *const _);
        if register {
            self.ref_ctx.insert(
                e.path.clone(),
                RefBind {
                    node,
                    table: t,
                    in_progress: true,
                },
            );
        }
        let (bt, bp) = match t {
            Some(t) => (t, new_prefix.as_str()),
            None => (table, new_prefix.as_str()),
        };
        self.build_children(node, def, &e.path, bt, bp, res_path, stack)?;
        if register && let Some(b) = self.ref_ctx.get_mut(&e.path) {
            b.in_progress = false;
        }
        Ok(Elem {
            json: e.name.clone(),
            path: e.path.clone(),
            repeats: e.repeats,
            table: t,
            neg_lane: false,
            kind: ElemKind::Group(node),
        })
    }

    fn build_content_ref(
        &mut self,
        e: &'s SpecElem,
        target: &str,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        if let Some(bind) = self.ref_ctx.get(target)
            && bind.in_progress
        {
            // Cyclic self-recursion: reuse the ancestor's node and table;
            // depth shows up as longer ordinal paths. Multiple referrers
            // into one table get distinct ordinal-sign lanes so their paths
            // cannot collide (QuestionnaireResponse item vs answer.item).
            let Some(t) = bind.table else {
                return Err(GenError::Build(format!(
                    "{}: cyclic contentReference to a flattened element {target}",
                    e.path
                )));
            };
            let node = bind.node;
            let lane = self.cyclic_lanes.entry(target.to_string()).or_insert(0);
            let neg_lane = match *lane {
                0 => false,
                1 => true,
                _ => {
                    return Err(GenError::Build(format!(
                        "{}: more than two cyclic referrers for {target}",
                        e.path
                    )));
                }
            };
            *lane += 1;
            return Ok(Elem {
                json: e.name.clone(),
                path: e.path.clone(),
                repeats: e.repeats,
                table: Some(t),
                neg_lane,
                kind: ElemKind::Group(node),
            });
        }
        // Sibling-branch reference: copy-expand the target's subtree here,
        // with the copy shadowing the target for any nested self-recursion.
        let col_base = format!("{prefix}{}", snake(&e.name));
        let node = self.alloc_node();
        let split = e.repeats
            || self.cyclic_targets.contains(target)
            || self.width_children(self.root_def, target, stack) > SPLIT_WIDTH;
        let (t, new_prefix): (Option<u32>, String) = if split {
            let t = self.new_table(table, &col_base, res_path);
            (Some(t), String::new())
        } else {
            (None, format!("{col_base}_"))
        };
        let shadow = self.ref_ctx.insert(
            target.to_string(),
            RefBind {
                node,
                table: t,
                in_progress: true,
            },
        );
        let (bt, bp) = match t {
            Some(t) => (t, new_prefix.as_str()),
            None => (table, new_prefix.as_str()),
        };
        let root_def = self.root_def;
        self.build_children(node, root_def, target, bt, bp, res_path, stack)?;
        match shadow {
            Some(prev) => {
                self.ref_ctx.insert(target.to_string(), prev);
            }
            None => {
                self.ref_ctx.remove(target);
            }
        }
        Ok(Elem {
            json: e.name.clone(),
            path: e.path.clone(),
            repeats: e.repeats,
            table: t,
            neg_lane: false,
            kind: ElemKind::Group(node),
        })
    }

    fn build_choice(
        &mut self,
        e: &'s SpecElem,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        if e.repeats {
            return Err(GenError::Build(format!(
                "{}: repeating choice elements are not supported",
                e.path
            )));
        }
        if e.types.is_empty() {
            return Err(GenError::Build(format!("{}: choice without types", e.path)));
        }
        let col_base = format!("{prefix}{}", snake(&e.name));
        let split = self.width_choice(&e.types, stack) > SPLIT_WIDTH;
        let (t, var_table, var_prefix): (Option<u32>, u32, String) = if split {
            let t = self.new_table(table, &col_base, res_path);
            (Some(t), t, format!("{}_", snake(&e.name)))
        } else {
            (None, table, format!("{col_base}_"))
        };
        let mut variants = Vec::with_capacity(e.types.len());
        for ty in &e.types {
            let json = format!("{}{}", e.name, ucfirst(ty));
            let var_res_path = format!("{res_path}:{ty}");
            let var_col_base = format!("{}{}", var_prefix, snake(ty));
            let var = self.build_typed_named(
                json,
                &e.path,
                false,
                ty,
                var_table,
                &var_col_base,
                &var_res_path,
                stack,
            )?;
            variants.push(var);
        }
        Ok(Elem {
            json: e.name.clone(),
            path: e.path.clone(),
            repeats: false,
            table: t,
            neg_lane: false,
            kind: ElemKind::Choice(variants),
        })
    }

    /// A typed element whose column base derives from its name.
    #[allow(clippy::too_many_arguments)]
    fn build_typed(
        &mut self,
        json: String,
        def_path: &str,
        repeats: bool,
        ty: &str,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        let col_base = format!("{prefix}{}", snake(&json));
        self.build_typed_named(
            json, def_path, repeats, ty, table, &col_base, res_path, stack,
        )
    }

    /// A typed element with an explicit column base (choice variants).
    #[allow(clippy::too_many_arguments)]
    fn build_typed_named(
        &mut self,
        json: String,
        def_path: &str,
        repeats: bool,
        ty: &str,
        table: u32,
        col_base: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<Elem, GenError> {
        if let Some(prim) = self.prim_of(ty) {
            if repeats {
                let t = self.new_table(table, col_base, res_path);
                let pc = self.prim_cols(t, "value", prim, res_path);
                return Ok(Elem {
                    json,
                    path: def_path.to_string(),
                    repeats,
                    table: Some(t),
                    neg_lane: false,
                    kind: ElemKind::Prim(pc),
                });
            }
            let pc = self.prim_cols(table, col_base, prim, res_path);
            return Ok(Elem {
                json,
                path: def_path.to_string(),
                repeats,
                table: None,
                neg_lane: false,
                kind: ElemKind::Prim(pc),
            });
        }
        if ty == "Resource" || ty == "DomainResource" {
            if repeats {
                return Err(GenError::Build(format!(
                    "{def_path}: repeating inline Resource elements are not supported"
                )));
            }
            let col = self.add_col(table, col_base, ColTy::Jsonb, res_path);
            return Ok(Elem {
                json,
                path: def_path.to_string(),
                repeats,
                table: None,
                neg_lane: false,
                kind: ElemKind::ResourceValue(col),
            });
        }
        // Complex datatype.
        if stack.iter().any(|s| s == ty) {
            return Ok(Elem {
                json,
                path: def_path.to_string(),
                repeats,
                table: None,
                neg_lane: false,
                kind: ElemKind::Spill,
            });
        }
        // `spec` outlives the builder, so copy the field reference to keep
        // the definition borrow independent of &mut self.
        let spec: &'s Spec = self.spec;
        let Some(tdef) = spec.types.get(ty) else {
            return Err(GenError::Build(format!("{def_path}: unknown type {ty:?}")));
        };
        let split = repeats || self.width_of_type(ty, stack) > SPLIT_WIDTH;
        let node = self.alloc_node();
        let (t, new_prefix): (Option<u32>, String) = if split {
            let t = self.new_table(table, col_base, res_path);
            (Some(t), String::new())
        } else {
            (None, format!("{col_base}_"))
        };
        stack.push(ty.to_string());
        let (bt, bp) = match t {
            Some(t) => (t, new_prefix.as_str()),
            None => (table, new_prefix.as_str()),
        };
        // Reference gets its `reference` string split into parsed columns.
        if ty == "Reference" {
            self.build_reference_node(node, tdef, bt, bp, res_path, stack)?;
        } else {
            self.build_children(node, tdef, ty, bt, bp, res_path, stack)?;
        }
        stack.pop();
        Ok(Elem {
            json,
            path: def_path.to_string(),
            repeats,
            table: t,
            neg_lane: false,
            kind: ElemKind::Group(node),
        })
    }

    fn build_reference_node(
        &mut self,
        node: u32,
        tdef: &'s Def,
        table: u32,
        prefix: &str,
        res_path: &str,
        stack: &mut Vec<String>,
    ) -> Result<(), GenError> {
        for &i in tdef.kids("Reference") {
            let e = &tdef.elems[i];
            if e.omitted || e.name == "id" || e.types.iter().any(|t| t == "Extension") {
                continue;
            }
            let child_res_path = format!("{res_path}.{}", e.name);
            if e.name == "reference" {
                let c_type = self.add_col(
                    table,
                    &format!("{prefix}ref_type"),
                    ColTy::Text,
                    &child_res_path,
                );
                let c_id = self.add_col(
                    table,
                    &format!("{prefix}ref_id"),
                    ColTy::Text,
                    &child_res_path,
                );
                let c_url = self.add_col(
                    table,
                    &format!("{prefix}ref_url"),
                    ColTy::Text,
                    &child_res_path,
                );
                self.nodes[node as usize].elems.push(Elem {
                    json: "reference".to_string(),
                    path: e.path.clone(),
                    repeats: false,
                    table: None,
                    neg_lane: false,
                    kind: ElemKind::RefStr(RefCols {
                        c_type,
                        c_id,
                        c_url,
                    }),
                });
            } else {
                let elem = self.build_elem(e, tdef, table, prefix, &child_res_path, stack)?;
                self.nodes[node as usize].elems.push(elem);
            }
        }
        Ok(())
    }

    fn prim_cols(&mut self, table: u32, base: &str, prim: Prim, res_path: &str) -> PrimCol {
        let col = self.add_col(table, base, prim.col_ty(), res_path);
        let sort = prim
            .sort_ty()
            .map(|ty| self.add_col(table, &format!("{base}_sort"), ty, res_path));
        PrimCol { col, sort, prim }
    }

    fn prim_of(&self, ty: &str) -> Option<Prim> {
        if !self.spec.primitives.contains(ty) {
            return None;
        }
        Some(match ty {
            "boolean" => Prim::Bool,
            "integer" | "positiveInt" | "unsignedInt" => Prim::Int,
            "integer64" => Prim::Int64,
            "decimal" => Prim::Decimal,
            "date" => Prim::Date,
            "dateTime" => Prim::DateTime,
            "instant" => Prim::Instant,
            "time" => Prim::Time,
            _ => Prim::Str,
        })
    }

    // ----- width estimation (mirrors the build decisions) -----

    fn width_of_type(&mut self, ty: &str, stack: &[String]) -> usize {
        if let Some(&w) = self.width_cache.get(ty) {
            return w;
        }
        let spec: &'s Spec = self.spec;
        let Some(tdef) = spec.types.get(ty) else {
            return 1;
        };
        let mut st: Vec<String> = stack.to_vec();
        st.push(ty.to_string());
        let w = self.width_children_of(tdef, ty, &st);
        self.width_cache.insert(ty.to_string(), w);
        w
    }

    fn width_children(&mut self, def: &'s Def, def_path: &str, stack: &[String]) -> usize {
        self.width_children_of(def, def_path, stack)
    }

    fn width_children_of(&mut self, def: &'s Def, def_path: &str, stack: &[String]) -> usize {
        let mut w = 0;
        for &i in def.kids(def_path) {
            let e = &def.elems[i];
            if e.omitted
                || e.name == "id"
                || e.types.iter().any(|t| t == "Extension")
                || e.repeats
                || e.content_ref.is_some()
            {
                continue;
            }
            if !def.kids(&e.path).is_empty() && !e.choice {
                let bw = self.width_children_of(def, &e.path, stack);
                w += if bw > SPLIT_WIDTH { 0 } else { bw };
                continue;
            }
            if e.choice {
                let cw = self.width_choice(&e.types, stack);
                w += if cw > SPLIT_WIDTH { 0 } else { cw };
                continue;
            }
            let [ty] = e.types.as_slice() else { continue };
            w += self.width_of_one(ty, stack);
        }
        w
    }

    fn width_choice(&mut self, types: &[String], stack: &[String]) -> usize {
        types.iter().map(|t| self.width_of_one(t, stack)).sum()
    }

    fn width_of_one(&mut self, ty: &str, stack: &[String]) -> usize {
        if let Some(prim) = self.prim_of(ty) {
            return if prim.sort_ty().is_some() { 2 } else { 1 };
        }
        if ty == "Resource" || ty == "DomainResource" {
            return 1;
        }
        if ty == "Reference" {
            // ref_type/ref_id/ref_url + display + type + identifier.
            return 12;
        }
        if stack.iter().any(|s| s == ty) {
            return 0; // spills
        }
        let w = self.width_of_type(ty, stack);
        if w > SPLIT_WIDTH { 0 } else { w }
    }
}
