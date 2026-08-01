//! Permissive views of the official FHIR definition JSON.
//!
//! The definition bundles mix `StructureDefinition` with `OperationDefinition`,
//! `SearchParameter`, `CompartmentDefinition` and more, and each release adds
//! fields the previous one did not have. These structs therefore deserialize
//! only what the generator uses and ignore everything else — no
//! `deny_unknown_fields` — which is why one set of types reads both the R4 and
//! R5 bundles unchanged.

use std::collections::BTreeMap;
use std::path::Path;

use ::serde::Deserialize;

/// A definition bundle file: a FHIR `Bundle` of definition resources.
#[derive(Debug, Deserialize)]
pub struct Bundle {
    /// The bundle's entries; each wraps one definition resource.
    #[serde(default)]
    pub entry: Vec<Entry>,
}

/// One `Bundle.entry`.
#[derive(Debug, Deserialize)]
pub struct Entry {
    /// The contained definition resource, left as raw JSON so that entries of
    /// an unwanted `resourceType` cost nothing to skip.
    pub resource: ::serde_json::Value,
}

/// A FHIR `StructureDefinition`: one datatype or resource.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinition {
    /// The definition's name, e.g. `"Patient"`.
    pub name: String,
    /// The FHIR type this defines, e.g. `"Patient"`. Equals the root element path.
    ///
    /// Absent in DSTU1 and DSTU2, which predate the field: R2 carries
    /// `constrainedType` for a constrained type and nothing at all for a
    /// base one, where the type is the definition's own name. Resolved by
    /// [`Self::resolved_type`] rather than by serde, so the difference stays
    /// in one place instead of spreading through the generator.
    #[serde(default, rename = "type")]
    type_name_raw: Option<String>,
    /// DSTU1/DSTU2: the type this definition constrains — its parent, not
    /// itself. Read through [`Self::constrains`].
    #[serde(default)]
    #[allow(dead_code, reason = "read via constrains(); kept as the parsed shape")]
    constrained_type: Option<String>,
    /// `primitive-type`, `complex-type`, `resource`, or `logical`.
    ///
    /// DSTU2 says `datatype` where later releases distinguish
    /// `primitive-type` from `complex-type`; [`Self::resolved_kind`] maps it.
    pub kind: String,
    /// Whether this is an abstract base (`Resource`, `DomainResource`, …).
    #[serde(default, rename = "abstract")]
    pub is_abstract: bool,
    /// The canonical URL, e.g. `http://hl7.org/fhir/StructureDefinition/Patient`.
    pub url: String,
    /// The release version this definition was published in, e.g. `"4.0.1"`.
    pub version: Option<String>,
    /// The specification's prose description of the type.
    pub description: Option<String>,
    /// The fully resolved element list. Definitions without one are skipped.
    pub snapshot: Option<Snapshot>,
    /// DSTU1/DSTU2: the canonical URL of the type this constrains.
    #[serde(default)]
    pub base: Option<String>,
}

impl StructureDefinition {
    /// The FHIR type this defines, across every release's spelling.
    ///
    /// R3 onwards say `type`. DSTU2 says `constrainedType` when the
    /// definition constrains another type, and says nothing when it *is* the
    /// type — in which case the name is the type. Resolving here keeps three
    /// releases' worth of schema drift out of the rest of the generator.
    /// `primitive-type`, `complex-type`, `resource`, or `logical`, across
    /// every release's vocabulary.
    ///
    /// DSTU1 and DSTU2 say `datatype` for both primitive and complex types.
    /// FHIR's own naming convention separates them reliably — every
    /// primitive is lowercase (`string`, `dateTime`, `markdown`) and every
    /// complex type is TitleCase (`HumanName`, `Period`) — and that rule
    /// holds in every release, which is why it is safe to lean on here.
    #[must_use]
    pub fn kind_name(&self) -> &str {
        if self.kind != "datatype" {
            return &self.kind;
        }
        if self.type_name().starts_with(char::is_lowercase) {
            "primitive-type"
        } else {
            "complex-type"
        }
    }

    #[must_use]
    pub fn type_name(&self) -> &str {
        // `constrainedType` is deliberately not consulted: it names the type
        // being *constrained* — the parent — not this one. Reading it as the
        // type makes every constrained primitive resolve to its base, so
        // `code`, `id`, `markdown`, `oid`, `positiveInt`, `unsignedInt` and
        // `uuid` all become `string` or `uri`, collide, and vanish. That is
        // seven of DSTU2's seventeen primitives.
        self.type_name_raw.as_deref().unwrap_or(&self.name)
    }
}

/// A `StructureDefinition.snapshot`: the fully resolved element list.
#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    /// Every element, in specification order, starting with the root.
    #[serde(default)]
    pub element: Vec<ElementDefinition>,
}

/// One `ElementDefinition` — a single element of a datatype or resource.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition {
    /// Dotted FHIR path, e.g. `"Patient.contact.name"`. Choice elements end
    /// in `[x]`.
    pub path: String,
    /// Minimum cardinality.
    #[serde(default)]
    pub min: u32,
    /// Maximum cardinality: a number, or `"*"` for unbounded.
    pub max: Option<String>,
    /// The one-line summary shown in the specification's tables.
    pub short: Option<String>,
    /// The full prose definition.
    pub definition: Option<String>,
    /// A pointer to another element whose children this one reuses, e.g.
    /// `"#Observation.referenceRange"` (R4) or a full URL with the same
    /// fragment (R5).
    pub content_reference: Option<String>,
    /// The allowed types. A choice element has more than one; a backbone
    /// element has `BackboneElement` or `Element`.
    #[serde(default, rename = "type")]
    pub types: Vec<ElementType>,
    /// The value-set binding, when the element is coded.
    pub binding: Option<Binding>,
    /// Whether the element is part of the `_summary=true` view.
    pub is_summary: Option<bool>,
}

/// One allowed type of an element (`ElementDefinition.type`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementType {
    /// The FHIR type code, e.g. `"Quantity"`, `"string"`, or a FHIRPath system
    /// type URL such as `http://hl7.org/fhirpath/System.String`.
    ///
    /// Empty when the definition states no code at all. R3 does this on a
    /// primitive's own `value` element, where the type is carried only by a
    /// `structuredefinition-json-type` extension — those elements are not
    /// modelled, because a primitive's Rust representation comes from
    /// [`super::primitives`] rather than from the snapshot.
    #[serde(default)]
    pub code: String,
    /// For `Reference`/`canonical`, the resource profiles that may be targeted.
    ///
    /// R3 writes a single string here and repeats the whole type entry once per
    /// target; R4 and R5 write a list. Both are read into a list.
    #[serde(default, rename = "targetProfile", deserialize_with = "string_or_seq")]
    target_profile_raw: Vec<String>,
    /// DSTU1/DSTU2 spelling: the targets live in `profile`, which later
    /// releases repurposed for constraining the *reference itself*.
    #[serde(default, deserialize_with = "string_or_seq")]
    profile: Vec<String>,
}

impl ElementType {
    /// The resource profiles a reference may target, across every spelling.
    ///
    /// Without the DSTU1/DSTU2 fallback every reference in those models has
    /// no targets at all — `Observation.subject` would not know it points at
    /// a Patient — and nothing would fail loudly enough to notice, because an
    /// empty target list is indistinguishable from an unconstrained
    /// reference.
    #[must_use]
    pub fn target_profiles(&self) -> &[String] {
        if self.target_profile_raw.is_empty() {
            &self.profile
        } else {
            &self.target_profile_raw
        }
    }
}

/// An element's value-set binding (`ElementDefinition.binding`).
///
/// The releases spell the bound value set three different ways, so read
/// [`Binding::value_set`] rather than any one field.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// `required`, `extensible`, `preferred`, or `example`.
    ///
    /// DSTU1 spells this `conformance` and carries `isExtensible` beside it;
    /// `strength` arrived with DSTU2. Absent entirely, a binding is treated
    /// as `example`, the weakest reading — guessing `required` would type a
    /// field as a closed enum on no evidence, and a wrong closed enum
    /// rejects valid data.
    #[serde(default = "weakest_binding", alias = "conformance")]
    pub strength: String,
    /// R4/R5: the canonical `ValueSet` URL, possibly with a `|version` suffix.
    #[serde(rename = "valueSet")]
    value_set_canonical: Option<String>,
    /// R3: the value set as a `Reference`. DSTU1 calls it
    /// `referenceResource`.
    #[serde(alias = "referenceResource")]
    value_set_reference: Option<BindingReference>,
    /// R3: the value set as a bare URI.
    value_set_uri: Option<String>,
}

/// The strength assumed when a release does not record one.
fn weakest_binding() -> String {
    "example".to_string()
}

/// The `Reference` form of an R3 binding's value set.
#[derive(Debug, Clone, Deserialize)]
pub struct BindingReference {
    /// The referenced `ValueSet` URL.
    pub reference: Option<String>,
}

impl Binding {
    /// The bound `ValueSet` URL, whichever way this release spells it.
    #[must_use]
    pub fn value_set(&self) -> Option<&str> {
        self.value_set_canonical
            .as_deref()
            .or_else(|| self.value_set_reference.as_ref()?.reference.as_deref())
            .or(self.value_set_uri.as_deref())
    }
}

/// Deserialize a field that may be either a single string or a list of them.
fn string_or_seq<'de, D: ::serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(match OneOrMany::deserialize(deserializer)? {
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    })
}

/// A FHIR `CodeSystem`, the source of the generated `codes` enums.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
    /// The code system's name, e.g. `"AdministrativeGender"`.
    pub name: Option<String>,
    /// The canonical URL that value sets reference.
    pub url: Option<String>,
    /// The specification's prose description.
    pub description: Option<String>,
    /// `complete`, `example`, `fragment`, or `not-present`. Only `complete`
    /// systems can become an exhaustive Rust enum.
    pub content: Option<String>,
    /// The concept tree; nested concepts are flattened by [`CodeSystem::codes`].
    #[serde(default)]
    pub concept: Vec<Concept>,
}

/// A FHIR `ValueSet`, which is what a `required` binding actually names.
///
/// Enums are built from `CodeSystem`s, and for most bindings the ValueSet and
/// its like-named CodeSystem hold the same codes, so the distinction never
/// showed. It matters when a ValueSet **composes** several systems: the
/// `task-intent` ValueSet is the `task-intent` CodeSystem (one code,
/// `unknown`) *plus* `request-intent` (eight). Reading only the CodeSystem
/// yields an enum that cannot represent its own binding.
#[derive(Debug, Clone, Deserialize)]
pub struct ValueSet {
    /// The canonical URL a binding refers to.
    pub url: Option<String>,
    /// How the value set is assembled from code systems.
    pub compose: Option<Compose>,
}

/// `ValueSet.compose`.
#[derive(Debug, Clone, Deserialize)]
pub struct Compose {
    #[serde(default)]
    pub include: Vec<Include>,
    /// Codes removed from the composed set.
    #[serde(default)]
    pub exclude: Vec<Include>,
}

/// One `ValueSet.compose.include` (or `exclude`).
#[derive(Debug, Clone, Deserialize)]
pub struct Include {
    /// The code system drawn from.
    pub system: Option<String>,
    /// Specific concepts. Absent means "every code in `system`".
    #[serde(default)]
    pub concept: Vec<IncludeConcept>,
    /// Other value sets composed in.
    #[serde(default, rename = "valueSet")]
    pub value_set: Vec<String>,
}

/// One `ValueSet.compose.include.concept`.
#[derive(Debug, Clone, Deserialize)]
pub struct IncludeConcept {
    pub code: String,
    pub display: Option<String>,
}

/// One `CodeSystem.concept`.
#[derive(Debug, Clone, Deserialize)]
pub struct Concept {
    /// The code as it appears on the wire.
    pub code: String,
    /// A short human-readable label.
    pub display: Option<String>,
    /// The concept's prose definition.
    pub definition: Option<String>,
    /// Child concepts, which FHIR nests but Rust enums flatten.
    #[serde(default)]
    pub concept: Vec<Concept>,
}

impl CodeSystem {
    /// Every concept in the system, flattened depth-first and de-duplicated by
    /// code (FHIR permits a code to appear once, but hierarchies are nested).
    #[must_use]
    pub fn codes(&self) -> Vec<&Concept> {
        fn walk<'a>(concepts: &'a [Concept], out: &mut Vec<&'a Concept>) {
            for concept in concepts {
                out.push(concept);
                walk(&concept.concept, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.concept, &mut out);
        let mut seen = std::collections::HashSet::new();
        out.retain(|c| seen.insert(c.code.clone()));
        out
    }
}

impl ElementDefinition {
    /// Whether the element repeats (`max` is `*` or greater than one).
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        match self.max.as_deref() {
            Some("*") => true,
            Some(other) => other.parse::<u32>().is_ok_and(|n| n > 1),
            None => false,
        }
    }

    /// Whether this is a `value[x]`-style choice element.
    #[must_use]
    pub fn is_choice(&self) -> bool {
        self.path.ends_with("[x]")
    }

    /// The element path with any `[x]` suffix removed.
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path.strip_suffix("[x]").unwrap_or(&self.path)
    }

    /// The last path segment, without any `[x]` suffix.
    #[must_use]
    pub fn leaf(&self) -> &str {
        self.base_path().rsplit('.').next().unwrap_or_default()
    }

    /// The path of the element that owns this one, e.g. `"Patient.contact"`
    /// for `"Patient.contact.name"`. `None` for a root element.
    #[must_use]
    pub fn owner_path(&self) -> Option<&str> {
        self.base_path().rsplit_once('.').map(|(owner, _)| owner)
    }

    /// Whether this element is FHIR *infrastructure* rather than a primitive
    /// element that can carry its own extensions.
    ///
    /// `Element.id`, every `<Type>.id`, and `Extension.url` are serialized as
    /// bare JSON attributes with no `_field` sibling. R4 and R5 say so by
    /// giving them a FHIRPath system type (`http://hl7.org/fhirpath/System.*`);
    /// R3 predates that convention and types them as ordinary `string`, `id` or
    /// `uri`, so the rule is expressed structurally and holds for all three.
    #[must_use]
    pub fn is_system_element(&self) -> bool {
        if self
            .types
            .iter()
            .any(|t| t.code.starts_with("http://hl7.org/fhirpath/System."))
        {
            return true;
        }
        self.leaf() == "id" || self.path == "Extension.url"
    }

    /// The element path a `contentReference` points at, e.g.
    /// `"Observation.referenceRange"`.
    ///
    /// R4 writes a bare fragment (`#Observation.referenceRange`) and R5 a full
    /// canonical URL with the same fragment, so both reduce to the text after
    /// the `#`.
    #[must_use]
    pub fn content_reference_path(&self) -> Option<&str> {
        self.content_reference.as_deref()?.rsplit('#').next()
    }
}

/// Read a definition bundle and return every `StructureDefinition` in it.
///
/// Entries of any other `resourceType`, and definitions without a snapshot, are
/// skipped: the generator can only work from fully resolved element lists.
pub fn read_structure_definitions(path: &Path) -> std::io::Result<Vec<StructureDefinition>> {
    let mut defs: Vec<StructureDefinition> =
        read_resources::<StructureDefinition>(path, "StructureDefinition")?
            .into_iter()
            .filter(|sd| sd.snapshot.is_some())
            .collect();
    // DSTU1 and DSTU2 treat `xhtml` as built in rather than declaring it,
    // yet `Narrative.div` still carries the type code. Without a definition
    // the generator emits a reference to a type it never writes, and the
    // release does not compile. Later releases declare it, so this adds
    // nothing there.
    // DSTU1 publishes no primitive definitions at all: they were built into
    // the specification rather than declared. Recover them from what the
    // elements reference, which is evidence rather than a guess at what that
    // release happened to include.
    if !defs.iter().any(|d| d.kind_name() == "primitive-type") {
        // Union of what this bundle references and the set DSTU1 defines in
        // prose. Recovery from references alone is per-bundle and therefore
        // incomplete: `date` appears in resources but not in the datatype
        // bundle, so reading only the latter yields a model missing types its
        // own elements use.
        let mut referenced: std::collections::BTreeSet<String> = [
            "base64Binary",
            "boolean",
            "code",
            "date",
            "dateTime",
            "decimal",
            "id",
            "idref",
            "instant",
            "integer",
            "oid",
            "string",
            "time",
            "uri",
            "uuid",
        ]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
        for d in &defs {
            for el in d.snapshot.iter().flat_map(|s| &s.element) {
                for t in &el.types {
                    if is_primitive_code(&t.code) {
                        referenced.insert(t.code.clone());
                    }
                }
            }
        }
        for code in referenced {
            if let Ok(sd) = ::serde_json::from_value::<StructureDefinition>(::serde_json::json!({
                "name": code, "type": code, "kind": "primitive-type",
                "url": format!("http://hl7.org/fhir/StructureDefinition/{code}"),
                "description": format!("The FHIR `{code}` primitive, built in to this release rather than declared."),
                "snapshot": { "element": [{ "path": code }] }
            })) {
                defs.push(sd);
            }
        }
    }
    // DSTU1 writes a single type code `*` for "any datatype"; later releases
    // enumerate them. Expanding it to what this release actually declares
    // keeps the choice a closed Rust enum instead of a variant literally
    // named `*`, which does not parse.
    let declared: Vec<String> = defs
        .iter()
        .filter(|d| matches!(d.kind_name(), "complex-type" | "primitive-type"))
        .map(|d| d.type_name().to_string())
        .filter(|t| t != "Extension")
        .collect();
    if !declared.is_empty() {
        for d in &mut defs {
            for el in d.snapshot.iter_mut().flat_map(|s| &mut s.element) {
                if el.types.iter().any(|t| t.code == "*") {
                    el.types = declared
                        .iter()
                        .map(|code| ElementType {
                            code: code.clone(),
                            target_profile_raw: Vec::new(),
                            profile: Vec::new(),
                        })
                        .collect();
                }
            }
        }
    }
    // DSTU1 has no `Element` type: it predates primitive extensions, so
    // there was nothing for `_field` siblings to hold. The generated code
    // still emits those siblings, so the type has to exist. `{ id, extension }`
    // is what it became in DSTU2 and has stayed since.
    if !defs.iter().any(|d| d.type_name() == "Element")
        && defs.iter().any(|d| d.type_name() == "Extension")
        && let Ok(sd) = ::serde_json::from_value::<StructureDefinition>(::serde_json::json!({
            "name": "Element", "type": "Element", "kind": "complex-type",
            "url": "http://hl7.org/fhir/StructureDefinition/Element",
            "description": "The base of every element: an id and extensions. Absent from DSTU1, which had no primitive extensions.",
            "snapshot": { "element": [
                { "path": "Element" },
                { "path": "Element.id", "min": 0, "max": "1", "type": [{ "code": "id" }] },
                { "path": "Element.extension", "min": 0, "max": "*", "type": [{ "code": "Extension" }] }
            ] }
        }))
    {
        defs.push(sd);
    }
    let declares_any_primitive = defs.iter().any(|d| d.kind_name() == "primitive-type");
    if declares_any_primitive
        && !defs.iter().any(|d| d.type_name() == "xhtml")
        && let Ok(sd) = ::serde_json::from_value::<StructureDefinition>(::serde_json::json!({
            "name": "xhtml",
            "type": "xhtml",
            "kind": "primitive-type",
            "url": "http://hl7.org/fhir/StructureDefinition/xhtml",
            "description": "XHTML, as used by Narrative.div. Built in to this release rather than declared.",
            "snapshot": { "element": [{ "path": "xhtml" }] }
        }))
    {
        defs.push(sd);
    }
    Ok(defs)
}

/// Read a definition bundle and return every `CodeSystem` in it.
pub fn read_code_systems(path: &Path) -> std::io::Result<Vec<CodeSystem>> {
    let mut out = read_resources::<CodeSystem>(path, "CodeSystem")?;
    out.extend(inline_code_systems(path)?);
    Ok(out)
}

/// DSTU1 and DSTU2 have no `CodeSystem` resource: a value set carries its
/// codes inline in `ValueSet.codeSystem`. R3 split them into their own
/// resource.
///
/// Without this, those releases generate **zero** code enums — every
/// required binding silently degrades to a bare `Code`, which is exactly the
/// "truthful but under-typed" outcome the crate exists to avoid. Reading the
/// inline form recovers them.
///
/// Returns nothing for releases that have real `CodeSystem` resources, so
/// this costs them a single pass over the bundle and changes nothing.
fn inline_code_systems(path: &Path) -> std::io::Result<Vec<CodeSystem>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let raw: ::serde_json::Value = ::serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::other(format!("{}: {e}", path.display())))?;
    let bundle: Bundle = ::serde_json::from_value(raw)
        .map_err(|e| std::io::Error::other(format!("{}: {e}", path.display())))?;
    let mut out = Vec::new();
    for entry in bundle.entry {
        if entry.resource.get("resourceType").and_then(|v| v.as_str()) != Some("ValueSet") {
            continue;
        }
        let Some(cs) = entry
            .resource
            .get("codeSystem")
            // DSTU1 calls it `define`; DSTU2 renamed it `codeSystem`.
            .or_else(|| entry.resource.get("define"))
        else {
            continue;
        };
        // The enum takes its name and URL from the *value set*, because that
        // is what a binding points at; the inline block supplies only the
        // system URL and the concepts.
        let mut synthesized = cs.clone();
        if let Some(obj) = synthesized.as_object_mut() {
            obj.insert("resourceType".into(), "CodeSystem".into());
            for field in ["name", "id", "title", "description"] {
                if let Some(v) = entry.resource.get(field) {
                    obj.insert(field.into(), v.clone());
                }
            }
            // DSTU1/DSTU2 have no `content` element — it arrived with the
            // CodeSystem resource in R3 — but an inline block *is* complete
            // by construction: it enumerates the codes in place rather than
            // pointing elsewhere. Without this the enum builder treats every
            // one as a fragment and emits nothing.
            obj.entry("content".to_string())
                .or_insert_with(|| "complete".into());
            // `url` must stay the code system's own, since bindings that name
            // a system rather than a value set resolve through it.
            if !obj.contains_key("url")
                && let Some(sys) = cs.get("system")
            {
                obj.insert("url".into(), sys.clone());
            }
        }
        // An inline block too sparse to be a code system is not an error:
        // DSTU2 value sets often reference an external system instead.
        if let Ok(c) = ::serde_json::from_value::<CodeSystem>(synthesized) {
            out.push(c);
        }
    }
    Ok(out)
}

/// Read a definition bundle and return every `ValueSet` in it.
pub fn read_value_sets(path: &Path) -> std::io::Result<Vec<ValueSet>> {
    read_resources::<ValueSet>(path, "ValueSet")
}

/// Rewrite DSTU2's `nameReference` into the modern `contentReference`.
///
/// DSTU2 expresses a recursive element by giving the repeated element a
/// `name` and pointing at that name from the element that repeats it. R3
/// replaced the mechanism with `contentReference`, which holds the target's
/// *path* prefixed by `#`. The rest of the generator understands only the
/// modern form, so a DSTU2 element carrying only a `nameReference` has no
/// type, and is dropped.
///
/// That is not a cosmetic loss. In DSTU2 it silently removes
/// `Bundle.entry.link`, `ValueSet.codeSystem.concept.concept`,
/// `ValueSet.expansion.contains.contains` and `Parameters.parameter.part`
/// — 92 elements in all — so a round-trip through the model quietly discards
/// every nested concept and every entry link.
///
/// This runs for all releases and is a no-op where `nameReference` does not
/// occur (R3 onwards, and DSTU1 once [`normalize_dstu1`] has already mapped
/// its own full-path form).
fn normalize_name_references(bundle: &mut ::serde_json::Value) {
    let Some(entries) = bundle.get_mut("entry").and_then(|e| e.as_array_mut()) else {
        return;
    };
    for entry in entries.iter_mut() {
        let Some(resource) = entry.get_mut("resource") else {
            continue;
        };
        for section in ["snapshot", "differential"] {
            let Some(elements) = resource
                .get_mut(section)
                .and_then(|s| s.get_mut("element"))
                .and_then(|e| e.as_array_mut())
            else {
                continue;
            };
            // `name` is scoped to the one definition, so the map is rebuilt
            // per section rather than shared across the bundle.
            let by_name: BTreeMap<String, String> = elements
                .iter()
                .filter_map(|el| {
                    let name = el.get("name")?.as_str()?.to_string();
                    let path = el.get("path")?.as_str()?.to_string();
                    Some((name, path))
                })
                .collect();
            for el in elements.iter_mut() {
                let Some(target) = el.get("nameReference").and_then(|v| v.as_str()) else {
                    continue;
                };
                // An unresolvable name is left alone: the element then has no
                // type and is dropped, which is the same outcome as before
                // this function existed, rather than a wrong reference.
                if let Some(path) = by_name.get(target).cloned()
                    && let Some(obj) = el.as_object_mut()
                {
                    obj.insert("contentReference".into(), format!("#{path}").into());
                }
            }
        }
    }
}

/// Could this type code name a FHIR primitive?
///
/// A primitive is spelled in lower camel case — `dateTime`, `base64Binary`.
/// The test cannot be "starts lowercase" alone, because R4 onwards give
/// `Element.id` the type code `http://hl7.org/fhirpath/System.String`, a URL
/// that also starts lowercase. Synthesizing a primitive from it produces a
/// bogus type whose name is a URL, and a matching junk entry in the element
/// metadata for every modern release.
fn is_primitive_code(code: &str) -> bool {
    !code.is_empty()
        && code.starts_with(char::is_lowercase)
        && code.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Read a bundle and deserialize every entry whose `resourceType` matches.
fn read_resources<T: for<'de> Deserialize<'de>>(
    path: &Path,
    resource_type: &str,
) -> std::io::Result<Vec<T>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut raw: ::serde_json::Value = ::serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::other(format!("{}: {e}", path.display())))?;
    normalize_name_references(&mut raw);
    let bundle: Bundle = ::serde_json::from_value(raw)
        .map_err(|e| std::io::Error::other(format!("{}: {e}", path.display())))?;
    let mut out = Vec::new();
    for entry in bundle.entry {
        if entry.resource.get("resourceType").and_then(|v| v.as_str()) != Some(resource_type) {
            continue;
        }
        // Silently skipping a definition that fails to parse would drop a whole
        // resource from the generated model, which is far worse than stopping.
        let name = entry
            .resource
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("<unnamed>")
            .to_string();
        let parsed = ::serde_json::from_value::<T>(entry.resource).map_err(|e| {
            std::io::Error::other(format!(
                "{}: could not read {resource_type} {name:?}: {e}",
                path.display()
            ))
        })?;
        out.push(parsed);
    }
    Ok(out)
}

/// Index the given definitions by FHIR type name, e.g. `"Patient"`.
#[must_use]
pub fn by_type_name(definitions: &[StructureDefinition]) -> BTreeMap<String, StructureDefinition> {
    definitions
        .iter()
        .map(|sd| (sd.type_name().to_string(), sd.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element(path: &str, max: &str) -> ElementDefinition {
        ElementDefinition {
            path: path.to_string(),
            min: 0,
            max: Some(max.to_string()),
            short: None,
            definition: None,
            content_reference: None,
            types: Vec::new(),
            binding: None,
            is_summary: None,
        }
    }

    #[test]
    fn path_parts() {
        let el = element("Observation.component.value[x]", "1");
        assert!(el.is_choice());
        assert_eq!(el.base_path(), "Observation.component.value");
        assert_eq!(el.leaf(), "value");
        assert_eq!(el.owner_path(), Some("Observation.component"));
        assert!(!el.is_multiple());
    }

    #[test]
    fn root_has_no_owner() {
        assert_eq!(element("Patient", "1").owner_path(), None);
    }

    #[test]
    fn multiplicity() {
        assert!(element("Patient.name", "*").is_multiple());
        assert!(!element("Patient.gender", "1").is_multiple());
        assert!(element("X.y", "5").is_multiple());
    }

    #[test]
    fn dstu2_name_reference_resolves_through_the_element_name() {
        // DSTU2 names the target element's `name`, not its path — the
        // opposite of DSTU1 — so this needs a lookup. Dropping it silently
        // removed 92 elements from the DSTU2 model, `Bundle.entry.link`
        // among them.
        let mut bundle = ::serde_json::json!({
            "resourceType": "Bundle",
            "entry": [{ "resource": {
                "resourceType": "StructureDefinition",
                "name": "Bundle",
                "snapshot": { "element": [
                    { "path": "Bundle.link", "name": "link" },
                    { "path": "Bundle.entry.link", "nameReference": "link" },
                ]},
            }}]
        });
        normalize_name_references(&mut bundle);
        let els = &bundle["entry"][0]["resource"]["snapshot"]["element"];
        assert_eq!(els[1]["contentReference"], "#Bundle.link");
        // The element that defines the name is left alone.
        assert!(els[0].get("contentReference").is_none());
    }

    #[test]
    fn an_unresolvable_name_reference_is_left_alone() {
        // Inventing a target would produce a wrong type, which is worse than
        // the element being dropped for having none.
        let mut bundle = ::serde_json::json!({
            "resourceType": "Bundle",
            "entry": [{ "resource": {
                "resourceType": "StructureDefinition",
                "snapshot": { "element": [
                    { "path": "X.y", "nameReference": "nothing-has-this-name" },
                ]},
            }}]
        });
        normalize_name_references(&mut bundle);
        let el = &bundle["entry"][0]["resource"]["snapshot"]["element"][0];
        assert!(el.get("contentReference").is_none());
    }

    #[test]
    fn name_references_resolve_in_the_differential_too() {
        let mut bundle = ::serde_json::json!({
            "resourceType": "Bundle",
            "entry": [{ "resource": {
                "resourceType": "StructureDefinition",
                "differential": { "element": [
                    { "path": "P.part", "name": "part" },
                    { "path": "P.part.part", "nameReference": "part" },
                ]},
            }}]
        });
        normalize_name_references(&mut bundle);
        assert_eq!(
            bundle["entry"][0]["resource"]["differential"]["element"][1]["contentReference"],
            "#P.part"
        );
    }

    #[test]
    fn the_reader_applies_the_name_reference_normalization() {
        // The unit tests above call `normalize_name_references` directly, so
        // they pass even when nothing calls it. That is not hypothetical:
        // while this was being written it was twice spliced into
        // `inline_code_systems` instead of `read_resources` — the two have
        // byte-identical bodies at the insertion point and a scripted
        // replace took the first match. Every direct test stayed green while
        // the DSTU2 model silently lost 92 elements.
        //
        // This goes through the public reader, so it fails if the wiring is
        // wrong however correct the function is.
        let dir = std::env::temp_dir().join("fhir-spec-wiring-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dstu2.json");
        std::fs::write(
            &path,
            ::serde_json::json!({
                "resourceType": "Bundle",
                "entry": [{ "resource": {
                    "resourceType": "StructureDefinition",
                    "name": "Bundle", "type": "Bundle", "kind": "resource",
                    "url": "http://hl7.org/fhir/StructureDefinition/Bundle",
                    "snapshot": { "element": [
                        { "path": "Bundle" },
                        { "path": "Bundle.link", "name": "link",
                          "type": [{ "code": "BackboneElement" }] },
                        { "path": "Bundle.entry.link", "nameReference": "link" },
                    ]},
                }}]
            })
            .to_string(),
        )
        .unwrap();

        let defs = read_structure_definitions(&path).unwrap();
        let bundle = defs.iter().find(|d| d.type_name() == "Bundle").unwrap();
        let nested = bundle
            .snapshot
            .as_ref()
            .unwrap()
            .element
            .iter()
            .find(|e| e.path == "Bundle.entry.link")
            .expect("Bundle.entry.link survived the read");
        assert_eq!(nested.content_reference_path(), Some("Bundle.link"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_url_type_code_is_not_a_primitive() {
        // R4 onwards type `Element.id` with this; it starts lowercase but is
        // a URL, and synthesizing a primitive from it put a junk element into
        // every modern release's metadata.
        assert!(!is_primitive_code("http://hl7.org/fhirpath/System.String"));
        assert!(!is_primitive_code(""));
        assert!(!is_primitive_code("System.String"));
        assert!(is_primitive_code("dateTime"));
        assert!(is_primitive_code("base64Binary"));
        // Complex types start uppercase and are never synthesized here.
        assert!(!is_primitive_code("CodeableConcept"));
    }

    #[test]
    fn content_reference_forms_agree() {
        let mut el = element("Observation.component.referenceRange", "*");
        el.content_reference = Some("#Observation.referenceRange".to_string());
        assert_eq!(
            el.content_reference_path(),
            Some("Observation.referenceRange")
        );
        el.content_reference = Some(
            "http://hl7.org/fhir/StructureDefinition/Observation#Observation.referenceRange"
                .to_string(),
        );
        assert_eq!(
            el.content_reference_path(),
            Some("Observation.referenceRange")
        );
    }

    #[test]
    fn system_elements_are_recognized_in_every_release() {
        // R4/R5 mark them with a FHIRPath system type.
        let mut el = element("Element.id", "1");
        el.types = vec![ElementType {
            code: "http://hl7.org/fhirpath/System.String".to_string(),
            target_profile_raw: Vec::new(),
            profile: Vec::new(),
        }];
        assert!(el.is_system_element());

        // R3 types the same element as a plain `string`.
        let mut el = element("Element.id", "1");
        el.types = vec![ElementType {
            code: "string".to_string(),
            target_profile_raw: Vec::new(),
            profile: Vec::new(),
        }];
        assert!(el.is_system_element());

        // As it does `Extension.url`, which R3 calls a `uri`.
        let mut el = element("Extension.url", "1");
        el.types = vec![ElementType {
            code: "uri".to_string(),
            target_profile_raw: Vec::new(),
            profile: Vec::new(),
        }];
        assert!(el.is_system_element());

        // An ordinary primitive element is not one.
        let mut el = element("Patient.birthDate", "1");
        el.types = vec![ElementType {
            code: "date".to_string(),
            target_profile_raw: Vec::new(),
            profile: Vec::new(),
        }];
        assert!(!el.is_system_element());
    }

    #[test]
    fn target_profile_reads_both_shapes() {
        // R3 writes one string; R4/R5 write a list.
        let one: ElementType = ::serde_json::from_value(::serde_json::json!({ "code": "Reference",
                "targetProfile": "http://hl7.org/fhir/StructureDefinition/Patient" }))
        .unwrap();
        assert_eq!(
            one.target_profiles(),
            ["http://hl7.org/fhir/StructureDefinition/Patient"]
        );

        let many: ElementType =
            ::serde_json::from_value(::serde_json::json!({ "code": "Reference",
                "targetProfile": ["a", "b"] }))
            .unwrap();
        assert_eq!(many.target_profiles(), ["a", "b"]);

        let none: ElementType =
            ::serde_json::from_value(::serde_json::json!({ "code": "string" })).unwrap();
        assert!(none.target_profiles().is_empty());
    }

    #[test]
    fn binding_value_set_reads_every_spelling() {
        // R4/R5: a canonical string.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSet": "http://x/vs|4.0.1" }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs|4.0.1"));

        // R3: a Reference.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSetReference": { "reference": "http://x/vs" } }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs"));

        // R3: a bare URI.
        let b: Binding = ::serde_json::from_value(::serde_json::json!({
            "strength": "required", "valueSetUri": "http://x/vs" }))
        .unwrap();
        assert_eq!(b.value_set(), Some("http://x/vs"));

        // No value set at all.
        let b: Binding =
            ::serde_json::from_value(::serde_json::json!({ "strength": "example" })).unwrap();
        assert_eq!(b.value_set(), None);
    }

    #[test]
    fn code_system_flattens_nested_concepts() {
        let system: CodeSystem = ::serde_json::from_value(::serde_json::json!({
            "resourceType": "CodeSystem",
            "name": "Example",
            "content": "complete",
            "concept": [
                { "code": "a", "concept": [ { "code": "b" } ] },
                { "code": "c" }
            ]
        }))
        .unwrap();
        let codes: Vec<&str> = system.codes().iter().map(|c| c.code.as_str()).collect();
        assert_eq!(codes, ["a", "b", "c"]);
    }
}
