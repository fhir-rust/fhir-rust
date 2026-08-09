//! Turning a `StructureDefinition` into a plan for one Rust module.
//!
//! Planning is separated from rendering so that the decisions with real
//! consequences — cardinality, backbone nesting, choice enums, coded fields,
//! primitive-extension siblings, and where a `Box` is needed to break a type
//! cycle — are made against the specification data and can be tested on their
//! own, while [`super::render`] only turns the result into text.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::naming;
use super::spec::{ElementDefinition, StructureDefinition};

/// Everything needed to write one datatype or resource module.
#[derive(Debug, Clone)]
pub struct TypePlan {
    /// The FHIR type name, e.g. `"Observation"`.
    pub type_name: String,
    /// The module file stem, e.g. `"observation"`.
    pub module: String,
    /// The canonical `StructureDefinition` URL.
    pub url: String,
    /// The FHIR version the definition was published in.
    pub version: String,
    /// The specification's one-line summary of the type.
    pub short: String,
    /// The specification's prose description of the type.
    pub description: String,
    /// The root struct first, then every nested backbone struct.
    pub structs: Vec<StructPlan>,
    /// One enum per `value[x]` choice element in this type.
    pub choices: Vec<ChoicePlan>,
    /// Whether this is a resource (rather than a datatype).
    pub is_resource: bool,
}

/// One generated `struct`: the type itself, or one of its backbone elements.
#[derive(Debug, Clone)]
pub struct StructPlan {
    /// The Rust struct name, e.g. `"ObservationComponent"`.
    pub name: String,
    /// The FHIR element path the struct represents, e.g. `"Observation.component"`.
    pub path: String,
    /// The specification's description of the element.
    pub doc: String,
    /// The struct's fields, in specification order.
    pub fields: Vec<FieldPlan>,
    /// Whether this is the type's root struct.
    pub is_root: bool,
    /// Whether the struct can derive `Default`.
    ///
    /// A `1..*` field is a non-empty [`Vec1`](::vec1::Vec1), which has no empty
    /// value, and a `1..1` field of such a struct inherits the problem, so this
    /// is settled across the whole model by [`resolve_defaults`].
    pub has_default: bool,
}

/// How an element's cardinality wraps its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrapper {
    /// `0..1` — `Option<T>`.
    Option,
    /// `1..1` — bare `T`.
    Required,
    /// `0..*` — `Vec<T>`, empty when absent.
    Vec,
    /// `1..*` — `vec1::Vec1<T>`, non-empty by construction.
    Vec1,
}

/// One generated struct field.
#[derive(Debug, Clone)]
pub struct FieldPlan {
    /// The Rust identifier, e.g. `reference_range` or `r#type`.
    pub ident: String,
    /// The FHIR element name exactly as it appears in JSON, e.g.
    /// `referenceRange` or `truthTP`.
    pub wire: String,
    /// The FHIR element path, e.g. `"Observation.component.code"`.
    pub path: String,
    /// The specification's one-line summary of the element.
    pub doc: String,
    /// The unwrapped Rust type, e.g. `types::CodeableConcept`.
    pub inner_type: String,
    /// How cardinality wraps [`Self::inner_type`].
    pub wrapper: Wrapper,
    /// Whether the inner type needs a `Box` to break a type cycle.
    pub boxed: bool,
    /// Minimum cardinality, for documentation.
    pub min: u32,
    /// Maximum cardinality as the raw FHIR token, for documentation.
    pub max: String,
    /// The choice enum this field holds, when the element is `value[x]`.
    pub choice: Option<String>,
    /// The primitive-extension sibling this field needs, if any.
    pub sibling: Option<SiblingPlan>,
}

/// A `_field` primitive-extension sibling (see `spec/09-primitive-extensions.md`).
#[derive(Debug, Clone)]
pub struct SiblingPlan {
    /// The Rust identifier, e.g. `birth_date_ext`.
    pub ident: String,
    /// The FHIR JSON key, e.g. `_birthDate`.
    pub wire: String,
    /// Whether the primitive repeats, making the sibling a `Vec<Option<Element>>`.
    pub is_multiple: bool,
}

/// One generated `value[x]` choice enum.
#[derive(Debug, Clone)]
pub struct ChoicePlan {
    /// The enum name, e.g. `"ObservationValue"`.
    pub name: String,
    /// The choice element's path, e.g. `"Observation.value[x]"`.
    pub path: String,
    /// The variants, in specification order.
    pub variants: Vec<ChoiceVariant>,
}

/// One variant of a choice enum.
#[derive(Debug, Clone)]
pub struct ChoiceVariant {
    /// The Rust variant name, e.g. `DateTime`.
    pub name: String,
    /// The FHIR JSON key, e.g. `valueDateTime`.
    pub key: String,
    /// The payload type, e.g. `Box<types::Quantity>`.
    pub payload: String,
}

/// The facts about a release that planning needs but a single
/// `StructureDefinition` does not carry.
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// FHIR primitive datatype codes, e.g. `string`, `dateTime`, `xhtml`.
    pub primitives: BTreeSet<String>,
    /// Names of the enums generated into the release's `codes` module.
    pub code_enums: BTreeSet<String>,
    /// Names of the resources this release models, e.g. `"Patient"` — the
    /// candidates for a typed `Reference<T>` target (T11).
    pub resources: BTreeSet<String>,
    /// The release module name, e.g. `"r4"`, for absolute paths.
    pub module: String,
}

impl Context {
    /// Whether a FHIR type code names a primitive datatype.
    ///
    /// FHIRPath system types (`http://hl7.org/fhirpath/System.String`) are not
    /// primitive *datatypes*: FHIR gives them no `_field` sibling and no
    /// choice-element `Primitive` wrapper.
    #[must_use]
    pub fn is_primitive(&self, code: &str) -> bool {
        self.primitives.contains(code)
    }

    /// The `codes` enum a `required` binding maps to, if one was generated.
    ///
    /// The value set URL's last segment names the enum, e.g.
    /// `http://hl7.org/fhir/ValueSet/observation-status|4.0.1` maps to
    /// `ObservationStatus`.
    #[must_use]
    pub fn code_enum_for(&self, value_set: &str) -> Option<String> {
        let name = value_set.split('|').next()?.rsplit('/').next()?;
        let candidate = naming::enum_name(name);
        self.code_enums.contains(&candidate).then_some(candidate)
    }
}

/// Plan the Rust module for one datatype or resource.
///
/// Returns `None` for definitions the generator does not model: those without a
/// snapshot, and primitive types (which [`super::primitives`] emits from a
/// table, because their Rust representation is not derivable from the spec).
#[must_use]
pub fn plan_type(sd: &StructureDefinition, ctx: &Context) -> Option<TypePlan> {
    let snapshot = sd.snapshot.as_ref()?;
    if sd.kind_name() == "primitive-type" {
        return None;
    }
    let elements = &snapshot.element;
    let root = elements.first()?;

    // A definition's Rust name is its `name`, not its `type`. Most of the time
    // they agree, but a *profile* constrains an existing type and keeps that
    // type's name: `MoneyQuantity`, `SimpleQuantity`, `Age`, `Distance`,
    // `Count` and `Duration` are all `type: "Quantity"`. Element paths still
    // start with the underlying type, so the two are tracked separately.
    let rust_name = naming::pascal(&sd.name);

    // Which paths own children, and therefore become nested structs. Doing this
    // first means an element's kind never depends on its declared type code,
    // which R4 and R5 spell differently for datatype backbones (`Element`) and
    // resource backbones (`BackboneElement`).
    let backbones = backbone_paths(elements, sd.type_name());

    let mut structs: Vec<StructPlan> = Vec::new();
    let mut choices: Vec<ChoicePlan> = Vec::new();
    let rename = Rename {
        fhir_type: sd.type_name().to_string(),
        rust_name: rust_name.clone(),
    };

    // The root struct, plus one struct per backbone path.
    let mut owners: Vec<&str> = vec![sd.type_name()];
    owners.extend(backbones.iter().map(String::as_str));

    for owner in owners {
        let owner_doc = if owner == sd.type_name() {
            element_doc(root, sd.description.as_deref())
        } else {
            elements
                .iter()
                .find(|e| e.base_path() == owner)
                .map(|e| element_doc(e, None))
                .unwrap_or_default()
        };

        let mut fields = Vec::new();
        for element in elements.iter().filter(|e| e.owner_path() == Some(owner)) {
            let Some(field) = plan_field(element, owner, &backbones, ctx, &mut choices, &rename)
            else {
                continue;
            };
            fields.push(field);
        }

        let has_default = !fields.iter().any(|f| f.wrapper == Wrapper::Vec1);
        structs.push(StructPlan {
            name: struct_name_in(owner, sd.type_name(), &rust_name),
            path: owner.to_string(),
            doc: owner_doc,
            fields,
            is_root: owner == sd.type_name(),
            has_default,
        });
    }

    Some(TypePlan {
        type_name: rust_name.clone(),
        module: naming::module_name(&sd.name),
        url: sd.url.clone(),
        version: sd.version.clone().unwrap_or_default(),
        short: root.short.clone().unwrap_or_default(),
        description: sd.description.clone().unwrap_or_default(),
        structs,
        choices,
        is_resource: sd.kind_name() == "resource",
    })
}

/// How a definition's element paths map onto Rust struct names.
///
/// For an ordinary type these are the same; for a profile the leading path
/// segment is the constrained type (`Quantity`) while the Rust name is the
/// profile's (`MoneyQuantity`).
struct Rename {
    /// The FHIR type every element path starts with.
    fhir_type: String,
    /// The Rust name that leading segment becomes.
    rust_name: String,
}

impl Rename {
    /// The struct name for an element path.
    fn struct_name(&self, path: &str) -> String {
        struct_name_in(path, &self.fhir_type, &self.rust_name)
    }
}

/// The struct name for `path`, with its leading `fhir_type` segment rendered as
/// `rust_name`.
fn struct_name_in(path: &str, fhir_type: &str, rust_name: &str) -> String {
    let rest = path.strip_prefix(fhir_type).unwrap_or(path);
    if rest.is_empty() {
        return rust_name.to_string();
    }
    format!(
        "{rust_name}{}",
        naming::struct_name(rest.trim_start_matches('.'))
    )
}

/// The paths of every element that owns children, i.e. every backbone element
/// that becomes a nested struct.
fn backbone_paths(elements: &[ElementDefinition], type_name: &str) -> BTreeSet<String> {
    let owners: HashSet<&str> = elements
        .iter()
        .filter_map(ElementDefinition::owner_path)
        .collect();
    elements
        .iter()
        .map(|e| e.base_path().to_string())
        .filter(|p| p != type_name && owners.contains(p.as_str()))
        .collect()
}

/// The documentation for an element: its prose definition, falling back to the
/// short summary, then to a caller-supplied default.
fn element_doc(element: &ElementDefinition, fallback: Option<&str>) -> String {
    element
        .definition
        .clone()
        .or_else(|| element.short.clone())
        .or_else(|| fallback.map(str::to_string))
        .unwrap_or_default()
}

/// Plan one field, recording any choice enum it introduces.
fn plan_field(
    element: &ElementDefinition,
    owner: &str,
    backbones: &BTreeSet<String>,
    ctx: &Context,
    choices: &mut Vec<ChoicePlan>,
    rename: &Rename,
) -> Option<FieldPlan> {
    // `max: "0"` removes an inherited element from this profile.
    if element.max.as_deref() == Some("0") {
        return None;
    }

    let leaf = element.leaf();
    let ident = naming::field_ident(leaf);
    let wrapper = wrapper_for(element);

    let mut choice = None;
    let mut sibling = None;
    let inner_type;

    if element.is_choice() {
        let owner_struct = rename.struct_name(owner);
        let name = format!("{owner_struct}{}", naming::pascal(leaf));
        choices.push(ChoicePlan {
            name: name.clone(),
            path: element.path.clone(),
            variants: choice_variants(element, leaf, ctx),
        });
        inner_type = name.clone();
        choice = Some(name);
    } else if backbones.contains(element.base_path()) {
        inner_type = rename.struct_name(element.base_path());
    } else if let Some(target) = element.content_reference_path() {
        // The element reuses another backbone's children rather than declaring
        // its own, so it is typed as that backbone's struct.
        inner_type = rename.struct_name(target);
    } else {
        let type_code = element.types.first()?.code.as_str();
        if type_code.is_empty() {
            return None; // a type the definition never names cannot be modelled
        }
        inner_type = scalar_type(type_code, element, ctx);
        // `Element.id`, `<Type>.id` and `Extension.url` are bare JSON
        // attributes: FHIR gives them no `_field` sibling even though R3 types
        // them as ordinary primitives.
        if ctx.is_primitive(type_code) && !element.is_system_element() {
            sibling = Some(SiblingPlan {
                ident: format!("{}_ext", naming::snake(leaf)),
                wire: format!("_{leaf}"),
                is_multiple: element.is_multiple(),
            });
        }
    }

    Some(FieldPlan {
        ident,
        wire: leaf.to_string(),
        path: element.path.clone(),
        doc: element.short.clone().unwrap_or_default(),
        inner_type,
        wrapper,
        boxed: false,
        min: element.min,
        max: element.max.clone().unwrap_or_else(|| "1".to_string()),
        choice,
        sibling,
    })
}

/// The Rust type for a non-choice, non-backbone element.
fn scalar_type(type_code: &str, element: &ElementDefinition, ctx: &Context) -> String {
    // A reference whose `targetProfile` names exactly one modelled resource is
    // typed as `Reference<ThatResource>` (T11); anything else — multiple
    // targets, an abstract target like `Resource`, or a profile this release
    // does not model — stays `Reference<Any>` via the default parameter.
    // R3 repeats the whole type entry once per target, so the targets are
    // collected across every `Reference` entry rather than read off one.
    if type_code == "Reference" {
        let targets: std::collections::BTreeSet<&str> = element
            .types
            .iter()
            .filter(|t| t.code == "Reference")
            .flat_map(super::spec::ElementType::target_profiles)
            .map(|url| url.rsplit(['/', '#']).next().unwrap_or(url))
            .collect();
        if targets.len() == 1 {
            let name = targets.iter().next().expect("one target");
            // `Resource`/`DomainResource` mean "any" — and `resources::Resource`
            // names the tagged enum, not a markable struct — so both stay
            // untyped even though the resources bundle defines them.
            if *name != "Resource" && *name != "DomainResource" && ctx.resources.contains(*name) {
                let module = &ctx.module;
                return format!("types::Reference<crate::{module}::resources::{name}>");
            }
        }
    }
    // `contained` is the one polymorphic slot narrow enough to type: it holds
    // this release's own resources and nothing else, so it is the `Resource`
    // enum rather than raw JSON (T47). `Bundle.entry.resource` and the other
    // `Resource`-typed slots stay raw — their consumers (`bundle_util`,
    // `Parameters`) dispatch on the JSON themselves.
    if type_code == "Resource" && element.path.ends_with(".contained") {
        let module = &ctx.module;
        return format!("crate::{module}::resources::Resource");
    }
    // A `required` binding is modelled as the matching enum rather than an
    // opaque `code`, wrapped so that codes outside the value set still parse.
    if type_code == "code"
        && let Some(binding) = &element.binding
        && binding.strength == "required"
        && let Some(enum_name) = binding.value_set().and_then(|vs| ctx.code_enum_for(vs))
    {
        let module = &ctx.module;
        return format!("crate::coded::Coded<crate::{module}::codes::{enum_name}>");
    }
    type_reference(type_code)
}

/// The Rust type naming a FHIR type code, outside any cardinality wrapper.
fn type_reference(type_code: &str) -> String {
    // A polymorphic resource slot (`Bundle.entry.resource`, `contained`) holds
    // any resource at all, so it stays raw JSON.
    if type_code == "Resource" || type_code == "DomainResource" {
        return "::serde_json::Value".to_string();
    }
    // FHIRPath system types name the primitive that backs them.
    if let Some(system) = type_code.strip_prefix("http://hl7.org/fhirpath/System.") {
        return format!("types::{system}");
    }
    format!("types::{}", naming::pascal(type_code))
}

/// The variants of a `value[x]` choice element, one per allowed type.
fn choice_variants(element: &ElementDefinition, leaf: &str, ctx: &Context) -> Vec<ChoiceVariant> {
    let module = &ctx.module;
    let mut seen = HashSet::new();
    element
        .types
        .iter()
        .filter(|t| seen.insert(t.code.clone()))
        .map(|t| {
            let payload = if ctx.is_primitive(&t.code) {
                // A primitive variant carries its `_value[x]` extension sibling.
                format!(
                    "crate::{module}::choice::Primitive<{}>",
                    type_reference(&t.code)
                )
            } else {
                format!("Box<{}>", type_reference(&t.code))
            };
            ChoiceVariant {
                name: naming::pascal(&t.code),
                key: naming::choice_key(leaf, &t.code),
                payload,
            }
        })
        .collect()
}

/// The cardinality wrapper for an element.
///
/// Choice elements are always `Option`, even when the specification makes them
/// mandatory: the enum has no default, and every struct in the model must
/// derive `Default`.
fn wrapper_for(element: &ElementDefinition) -> Wrapper {
    let multiple = element.is_multiple();
    let required = element.min >= 1;
    if element.is_choice() {
        return Wrapper::Option;
    }
    match (required, multiple) {
        (false, false) => Wrapper::Option,
        (true, false) => Wrapper::Required,
        (false, true) => Wrapper::Vec,
        (true, true) => Wrapper::Vec1,
    }
}

/// Insert a `Box` wherever a field would otherwise make a type infinitely
/// sized, e.g. `Reference` holds an `Identifier` which holds a `Reference`.
///
/// Only fields that store their type inline (`Option<T>`, `T`) can close a
/// cycle; `Vec<T>` already indirects. The traversal visits types in name order
/// and boxes the field that closes each cycle, so the choice is stable across
/// runs.
pub fn break_type_cycles(plans: &mut [TypePlan]) {
    // Every struct in the model, and the inline edges between them.
    let mut owner_of: HashMap<String, (usize, usize)> = HashMap::new();
    for (ti, plan) in plans.iter().enumerate() {
        for (si, structure) in plan.structs.iter().enumerate() {
            owner_of.insert(structure.name.clone(), (ti, si));
        }
    }

    let edges: BTreeMap<String, Vec<(usize, String)>> = plans
        .iter()
        .flat_map(|plan| plan.structs.iter())
        .map(|structure| {
            let targets = structure
                .fields
                .iter()
                .enumerate()
                .filter(|(_, f)| matches!(f.wrapper, Wrapper::Option | Wrapper::Required))
                .filter_map(|(i, f)| inline_struct_target(f).map(|t| (i, t.to_string())))
                .collect();
            (structure.name.clone(), targets)
        })
        .collect();

    // Depth-first search; an edge back to a node still on the stack is the one
    // that closes the cycle, so that is the field to box.
    let mut state: HashMap<&str, u8> = HashMap::new();
    let mut to_box: Vec<(String, usize)> = Vec::new();
    let names: Vec<&str> = edges.keys().map(String::as_str).collect();
    for name in names {
        visit(name, &edges, &mut state, &mut to_box);
    }

    for (struct_name, field_index) in to_box {
        if let Some(&(ti, si)) = owner_of.get(&struct_name) {
            plans[ti].structs[si].fields[field_index].boxed = true;
        }
    }
}

/// The struct a field stores, if it stores one.
///
/// Only generated struct types can participate in a cycle; primitives,
/// `Coded<_>` and raw JSON cannot.
fn struct_target(field: &FieldPlan) -> Option<&str> {
    if field.choice.is_some() {
        return None;
    }
    if let Some(name) = field.inner_type.strip_prefix("types::") {
        // A typed `Reference<crate::…::X>` participates in cycles as
        // `Reference`: the target parameter is a zero-sized phantom and embeds
        // nothing, so the struct edge is to `Reference` itself (T11).
        return Some(name.split('<').next().unwrap_or(name));
    }
    // A nested backbone struct is named directly, e.g. `ObservationComponent`.
    field
        .inner_type
        .starts_with(|c: char| c.is_ascii_uppercase())
        .then_some(field.inner_type.as_str())
}

/// The struct a field stores inline (not behind a `Vec`), if it stores one.
fn inline_struct_target(field: &FieldPlan) -> Option<&str> {
    matches!(field.wrapper, Wrapper::Option | Wrapper::Required)
        .then(|| struct_target(field))
        .flatten()
}

/// Depth-first visit for [`break_type_cycles`]: 1 marks "on the stack", 2
/// marks "finished".
fn visit<'a>(
    name: &'a str,
    edges: &'a BTreeMap<String, Vec<(usize, String)>>,
    state: &mut HashMap<&'a str, u8>,
    to_box: &mut Vec<(String, usize)>,
) {
    if state.contains_key(name) {
        return;
    }
    state.insert(name, 1);
    if let Some(targets) = edges.get(name) {
        for (field_index, target) in targets {
            match state.get(target.as_str()) {
                // An edge back to a type still on the stack closes a cycle.
                Some(1) => to_box.push((name.to_string(), *field_index)),
                Some(_) => {}
                None => visit(target, edges, state, to_box),
            }
        }
    }
    state.insert(name, 2);
}

/// Settle which structs can derive `Default` across the whole model.
///
/// A struct starts without a default if it has a `1..*` ([`Vec1`](::vec1::Vec1))
/// field, and loses it if any mandatory `1..1` field holds a struct that has no
/// default. That second rule feeds back into itself, so it is iterated to a
/// fixed point.
pub fn resolve_defaults(plans: &mut [TypePlan]) {
    let mut without_default: BTreeSet<String> = plans
        .iter()
        .flat_map(|p| p.structs.iter())
        .filter(|s| !s.has_default)
        .map(|s| s.name.clone())
        .collect();

    loop {
        let mut added = false;
        for plan in plans.iter() {
            for structure in &plan.structs {
                if without_default.contains(&structure.name) {
                    continue;
                }
                let inherits = structure.fields.iter().any(|f| {
                    // `Box<T>` is `Default` only when `T` is, so boxing does
                    // not rescue a mandatory field here.
                    f.wrapper == Wrapper::Required
                        && struct_target(f).is_some_and(|t| without_default.contains(t))
                });
                if inherits {
                    without_default.insert(structure.name.clone());
                    added = true;
                }
            }
        }
        if !added {
            break;
        }
    }

    for plan in plans.iter_mut() {
        for structure in &mut plan.structs {
            structure.has_default = !without_default.contains(&structure.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::spec::StructureDefinition;

    fn ctx() -> Context {
        Context {
            primitives: ["string", "boolean", "dateTime", "code", "xhtml"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            code_enums: ["ObservationStatus"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            resources: ["Patient", "Observation"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            module: "r4".to_string(),
        }
    }

    fn sd(json: ::serde_json::Value) -> StructureDefinition {
        ::serde_json::from_value(json).unwrap()
    }

    #[test]
    fn cardinality_maps_to_wrappers() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Sample", "type": "Sample", "kind": "complex-type",
                "url": "http://example.org/Sample", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Sample" },
                    { "path": "Sample.a", "min": 0, "max": "1", "type": [{ "code": "Period" }] },
                    { "path": "Sample.b", "min": 1, "max": "1", "type": [{ "code": "Period" }] },
                    { "path": "Sample.c", "min": 0, "max": "*", "type": [{ "code": "Period" }] },
                    { "path": "Sample.d", "min": 1, "max": "*", "type": [{ "code": "Period" }] },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        let root = &plan.structs[0];
        let wrappers: Vec<Wrapper> = root.fields.iter().map(|f| f.wrapper).collect();
        assert_eq!(
            wrappers,
            [
                Wrapper::Option,
                Wrapper::Required,
                Wrapper::Vec,
                Wrapper::Vec1
            ]
        );
        // A `1..*` field costs the struct its `Default`.
        assert!(!root.has_default);
    }

    #[test]
    fn backbone_elements_become_nested_structs() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Obs", "type": "Obs", "kind": "resource",
                "url": "http://example.org/Obs", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Obs" },
                    { "path": "Obs.component", "min": 0, "max": "*",
                      "type": [{ "code": "BackboneElement" }] },
                    { "path": "Obs.component.code", "min": 1, "max": "1",
                      "type": [{ "code": "CodeableConcept" }] },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        assert_eq!(plan.structs.len(), 2);
        assert_eq!(plan.structs[1].name, "ObsComponent");
        // The parent field references the nested struct, not a flattened copy.
        assert_eq!(plan.structs[0].fields[0].inner_type, "ObsComponent");
        assert_eq!(
            plan.structs[1].fields[0].inner_type,
            "types::CodeableConcept"
        );
    }

    #[test]
    fn content_reference_reuses_another_backbone() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Obs", "type": "Obs", "kind": "resource",
                "url": "http://example.org/Obs", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Obs" },
                    { "path": "Obs.referenceRange", "min": 0, "max": "*",
                      "type": [{ "code": "BackboneElement" }] },
                    { "path": "Obs.referenceRange.low", "min": 0, "max": "1",
                      "type": [{ "code": "Quantity" }] },
                    { "path": "Obs.component", "min": 0, "max": "*",
                      "type": [{ "code": "BackboneElement" }] },
                    { "path": "Obs.component.referenceRange", "min": 0, "max": "*",
                      "contentReference": "#Obs.referenceRange" },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        let component = plan
            .structs
            .iter()
            .find(|s| s.name == "ObsComponent")
            .unwrap();
        assert_eq!(component.fields[0].inner_type, "ObsReferenceRange");
    }

    #[test]
    fn choice_elements_become_enums() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Obs", "type": "Obs", "kind": "resource",
                "url": "http://example.org/Obs", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Obs" },
                    { "path": "Obs.value[x]", "min": 0, "max": "1",
                      "type": [{ "code": "Quantity" }, { "code": "string" }] },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        assert_eq!(plan.choices.len(), 1);
        let choice = &plan.choices[0];
        assert_eq!(choice.name, "ObsValue");
        assert_eq!(choice.variants[0].key, "valueQuantity");
        assert_eq!(choice.variants[0].payload, "Box<types::Quantity>");
        assert_eq!(choice.variants[1].key, "valueString");
        assert_eq!(
            choice.variants[1].payload,
            "crate::r4::choice::Primitive<types::String>"
        );
        // The field holds the enum, flattened into the parent object.
        assert_eq!(
            plan.structs[0].fields[0].choice.as_deref(),
            Some("ObsValue")
        );
    }

    #[test]
    fn required_bindings_become_coded_enums() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Obs", "type": "Obs", "kind": "resource",
                "url": "http://example.org/Obs", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Obs" },
                    { "path": "Obs.status", "min": 1, "max": "1", "type": [{ "code": "code" }],
                      "binding": { "strength": "required",
                                   "valueSet": "http://hl7.org/fhir/ValueSet/observation-status|4.0.1" } },
                    { "path": "Obs.other", "min": 0, "max": "1", "type": [{ "code": "code" }],
                      "binding": { "strength": "required",
                                   "valueSet": "http://hl7.org/fhir/ValueSet/not-generated" } },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        assert_eq!(
            plan.structs[0].fields[0].inner_type,
            "crate::coded::Coded<crate::r4::codes::ObservationStatus>"
        );
        // A binding with no generated enum stays an opaque code.
        assert_eq!(plan.structs[0].fields[1].inner_type, "types::Code");
    }

    #[test]
    fn primitive_fields_get_extension_siblings() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Pat", "type": "Pat", "kind": "resource",
                "url": "http://example.org/Pat", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Pat" },
                    { "path": "Pat.id", "min": 0, "max": "1",
                      "type": [{ "code": "http://hl7.org/fhirpath/System.String" }] },
                    { "path": "Pat.birthDate", "min": 0, "max": "1",
                      "type": [{ "code": "dateTime" }] },
                    { "path": "Pat.given", "min": 0, "max": "*", "type": [{ "code": "string" }] },
                    { "path": "Pat.link", "min": 0, "max": "1", "type": [{ "code": "Reference" }] },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        let fields = &plan.structs[0].fields;
        // A FHIRPath system type is not a primitive datatype: no sibling.
        assert!(fields[0].sibling.is_none());
        assert_eq!(fields[0].inner_type, "types::String");

        let birth = fields[1].sibling.as_ref().unwrap();
        assert_eq!(birth.ident, "birth_date_ext");
        assert_eq!(birth.wire, "_birthDate");
        assert!(!birth.is_multiple);

        assert!(fields[2].sibling.as_ref().unwrap().is_multiple);
        // A complex type has no primitive value to extend.
        assert!(fields[3].sibling.is_none());
    }

    #[test]
    fn removed_elements_are_skipped() {
        let plan = plan_type(
            &sd(::serde_json::json!({
                "name": "Sample", "type": "Sample", "kind": "complex-type",
                "url": "http://example.org/Sample", "version": "4.0.1",
                "snapshot": { "element": [
                    { "path": "Sample" },
                    { "path": "Sample.gone", "min": 0, "max": "0", "type": [{ "code": "string" }] },
                    { "path": "Sample.kept", "min": 0, "max": "1", "type": [{ "code": "string" }] },
                ]}
            })),
            &ctx(),
        )
        .unwrap();
        let idents: Vec<&str> = plan.structs[0]
            .fields
            .iter()
            .map(|f| f.ident.as_str())
            .collect();
        assert_eq!(idents, ["kept"]);
    }

    #[test]
    fn polymorphic_slots_stay_raw_json() {
        assert_eq!(type_reference("Resource"), "::serde_json::Value");
        assert_eq!(type_reference("DomainResource"), "::serde_json::Value");
        assert_eq!(
            type_reference("http://hl7.org/fhirpath/System.String"),
            "types::String"
        );
        assert_eq!(type_reference("CodeableConcept"), "types::CodeableConcept");
        assert_eq!(type_reference("dateTime"), "types::DateTime");
    }

    #[test]
    fn cycles_are_broken_with_a_box() {
        // Reference holds an Identifier which holds a Reference: without a
        // `Box` neither type has a size.
        let mut plans = vec![
            plan_type(
                &sd(::serde_json::json!({
                    "name": "Identifier", "type": "Identifier", "kind": "complex-type",
                    "url": "u", "version": "4.0.1",
                    "snapshot": { "element": [
                        { "path": "Identifier" },
                        { "path": "Identifier.assigner", "min": 0, "max": "1",
                          "type": [{ "code": "Reference" }] },
                    ]}
                })),
                &ctx(),
            )
            .unwrap(),
            plan_type(
                &sd(::serde_json::json!({
                    "name": "Reference", "type": "Reference", "kind": "complex-type",
                    "url": "u", "version": "4.0.1",
                    "snapshot": { "element": [
                        { "path": "Reference" },
                        { "path": "Reference.identifier", "min": 0, "max": "1",
                          "type": [{ "code": "Identifier" }] },
                    ]}
                })),
                &ctx(),
            )
            .unwrap(),
        ];
        break_type_cycles(&mut plans);
        let boxed: Vec<bool> = plans
            .iter()
            .flat_map(|p| p.structs.iter())
            .flat_map(|s| s.fields.iter())
            .map(|f| f.boxed)
            .collect();
        // Exactly one of the two edges is boxed — enough to break the cycle.
        assert_eq!(boxed.iter().filter(|b| **b).count(), 1, "{boxed:?}");
    }

    #[test]
    fn repeating_fields_never_need_a_box() {
        // `Extension.extension` is recursive but repeating, and a `Vec` already
        // indirects.
        let mut plans = vec![
            plan_type(
                &sd(::serde_json::json!({
                    "name": "Extension", "type": "Extension", "kind": "complex-type",
                    "url": "u", "version": "4.0.1",
                    "snapshot": { "element": [
                        { "path": "Extension" },
                        { "path": "Extension.extension", "min": 0, "max": "*",
                          "type": [{ "code": "Extension" }] },
                    ]}
                })),
                &ctx(),
            )
            .unwrap(),
        ];
        break_type_cycles(&mut plans);
        assert!(!plans[0].structs[0].fields[0].boxed);
    }
}
