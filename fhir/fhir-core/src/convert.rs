//! Converting a resource from one FHIR release to another, saying what was
//! lost.
//!
//! The releases do not share model types, and they never will: an R3, R4 and R5
//! `Patient` disagree about enough that a common type would either accept data
//! no release permits or silently drop data a release requires (spec 12,
//! R12.4). So this module does not convert *types*. It converts the **wire
//! form** — a `serde_json::Value` — from the shape one release's model accepts
//! into the shape another's does, and it returns a [`LossReport`] naming
//! everything it had to change or discard.
//!
//! That report is the point. "Serialize to JSON and see what the target
//! refuses" already worked; what it could not tell you is *what* it refused, or
//! that it refused anything at all — serde reports the first error and stops,
//! and a field the target simply does not have is not an error, it is silence.
//! Cross-version exchange is routine in national deployments, and a conversion
//! whose losses are invisible is worse than one that fails.
//!
//! # What it is driven by
//!
//! Both releases' [`ElementMeta`] tables, which are generated from the official
//! `ElementDefinition`s. Nothing here is a hand-written rule about a particular
//! resource, so the layer does not rot as releases are added: `fhir-release-6`
//! became convertible by existing, not by anyone editing this file.
//!
//! The consequence worth stating plainly: this is a **structural** conversion.
//! It knows that R4 `Observation` has no `bodyStructure`, that R3
//! `Observation.value[x]` admits `Attachment` where R4 does not, and that
//! `Bundle.entry` repeats in both. It does *not* know that R3's
//! `MedicationRequest.requester.agent` became R4's `MedicationRequest.requester`
//! — that is a semantic remapping, and inventing one here would be exactly the
//! silent data-mangling the type split exists to prevent. Such elements are
//! reported as [`LossKind::ElementRemoved`], which is honest: this layer did not
//! carry them over.

use ::serde_json::{Map, Value};

use crate::meta::{self, ElementMeta};

/// Why a piece of the source did not survive the conversion unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LossKind {
    /// The target release has no element at this path. The value was dropped.
    ElementRemoved,
    /// The target release has no such resource type. The resource was dropped.
    ResourceRemoved,
    /// The document is not a resource: it has no `resourceType` to convert by.
    /// Serializing a bare resource struct rather than the release's `Resource`
    /// enum produces exactly this, because the tag lives on the enum.
    NotAResource,
    /// A `value[x]` variant whose type the target's choice does not allow. The
    /// value was dropped.
    ChoiceVariantUnsupported,
    /// The element repeats in the source and does not in the target. Everything
    /// after the first entry was dropped.
    CardinalityNarrowed,
    /// The element's JSON kind differs between the releases — a string became a
    /// number, or a primitive became a complex type — so the value cannot be
    /// carried across as it stands. It was dropped.
    TypeChanged,
    /// The target requires this element (`min >= 1`) and the converted resource
    /// does not have it. Nothing was dropped; the result will not validate.
    RequiredMissing,
    /// The target binds this element to a *different* value set with `required`
    /// strength, so the code carried over may not be a legal value there. The
    /// value was kept.
    BindingChanged,
}

impl LossKind {
    /// Whether the loss discarded data, as opposed to reporting a problem with
    /// data that was kept.
    ///
    /// [`RequiredMissing`](Self::RequiredMissing) and
    /// [`BindingChanged`](Self::BindingChanged) are warnings about the result;
    /// every other kind means something is gone.
    #[must_use]
    pub fn discards_data(self) -> bool {
        !matches!(self, Self::RequiredMissing | Self::BindingChanged)
    }
}

impl ::std::fmt::Display for LossKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let s = match self {
            Self::ElementRemoved => "element not in target",
            Self::ResourceRemoved => "resource type not in target",
            Self::NotAResource => "not a resource",
            Self::ChoiceVariantUnsupported => "choice variant not in target",
            Self::CardinalityNarrowed => "does not repeat in target",
            Self::TypeChanged => "incompatible type in target",
            Self::RequiredMissing => "required by target but absent",
            Self::BindingChanged => "different required binding in target",
        };
        f.write_str(s)
    }
}

/// One thing the conversion changed or discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Loss {
    /// Where it happened, as a JSON-ish path into the source document, e.g.
    /// `"Observation.component[1].valueAttachment"`.
    pub path: String,
    /// What happened.
    pub kind: LossKind,
    /// The specifics — the offending type name, the two value-set URLs, the
    /// number of entries dropped.
    pub detail: String,
}

impl ::std::fmt::Display for Loss {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}: {} ({})", self.path, self.kind, self.detail)
    }
}

/// Everything the conversion changed or discarded, in document order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LossReport {
    losses: Vec<Loss>,
}

impl LossReport {
    /// Whether the conversion carried the whole document across untouched.
    #[must_use]
    pub fn is_lossless(&self) -> bool {
        self.losses.is_empty()
    }

    /// Whether any loss actually discarded data, as opposed to warning about
    /// data that was kept (see [`LossKind::discards_data`]).
    #[must_use]
    pub fn discarded_data(&self) -> bool {
        self.losses.iter().any(|l| l.kind.discards_data())
    }

    /// How many losses were recorded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.losses.len()
    }

    /// Whether no losses were recorded; the same question as
    /// [`is_lossless`](Self::is_lossless).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.losses.is_empty()
    }

    /// The losses, in document order.
    pub fn iter(&self) -> impl Iterator<Item = &Loss> {
        self.losses.iter()
    }

    /// Only the losses of a given kind.
    pub fn of_kind(&self, kind: LossKind) -> impl Iterator<Item = &Loss> {
        self.losses.iter().filter(move |l| l.kind == kind)
    }
}

impl<'a> IntoIterator for &'a LossReport {
    type Item = &'a Loss;
    type IntoIter = ::std::slice::Iter<'a, Loss>;

    fn into_iter(self) -> Self::IntoIter {
        self.losses.iter()
    }
}

impl ::std::fmt::Display for LossReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        if self.losses.is_empty() {
            return f.write_str("lossless");
        }
        for (i, loss) in self.losses.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{loss}")?;
        }
        Ok(())
    }
}

/// The result of a conversion: what the target release will accept, and what
/// that cost.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Converted {
    /// The converted document, ready to deserialize into the target release's
    /// model. [`Value::Null`] if the resource type does not exist in the target.
    pub value: Value,
    /// What was changed or discarded getting there.
    pub report: LossReport,
}

impl Converted {
    /// The converted document, or the report if anything was changed at all.
    ///
    /// For callers who would rather refuse a document than transmit a lossy
    /// version of it — a reasonable default when the receiver is a clinical
    /// system and a dropped element is a dropped fact.
    ///
    /// The bar is [`LossReport::is_lossless`], not
    /// [`LossReport::discarded_data`]: a `RequiredMissing` means the result
    /// will not validate in the target, and a `BindingChanged` means a code
    /// that was legal may not be, so neither is something a strict caller
    /// should be handed silently. Both are rare in practice — across the
    /// committed corpora they account for one document each per release pair,
    /// against roughly half that convert cleanly — so this does not reject
    /// everything.
    ///
    /// # Errors
    ///
    /// The [`LossReport`], whenever the conversion was not lossless.
    pub fn strict(self) -> Result<Value, LossReport> {
        if self.report.is_lossless() {
            Ok(self.value)
        } else {
            Err(self.report)
        }
    }
}

/// Convert one resource's JSON from the `source` release to the `target`
/// release, driven by the two element tables.
///
/// Pass each release's `meta::elements()`. The `fhir` crate's
/// `convert::between` wraps this so the tables come from the release marker
/// types rather than by hand.
///
/// The returned [`Converted::value`] contains only what the target's model
/// accepts, so deserializing it into the target's `Resource` succeeds where
/// deserializing the source's JSON directly would have failed or silently
/// dropped fields.
#[must_use]
pub fn resource(
    source: &'static [ElementMeta],
    target: &'static [ElementMeta],
    value: &Value,
) -> Converted {
    let mut losses = Vec::new();
    let converted = convert_resource(source, target, value, "", &mut losses);
    Converted {
        value: converted.unwrap_or(Value::Null),
        report: LossReport { losses },
    }
}

/// Convert a resource object, using its own `resourceType` as the context.
///
/// `Returns` `None` when the target release has no such resource type, which is
/// how a dropped `contained` entry and a dropped root resource share a path.
fn convert_resource(
    source: &'static [ElementMeta],
    target: &'static [ElementMeta],
    value: &Value,
    path: &str,
    losses: &mut Vec<Loss>,
) -> Option<Value> {
    // A document with no `resourceType` cannot be converted, and must not fail
    // quietly: an empty report beside a null result would read as "nothing to
    // do" when the truth is "nothing was done".
    let type_name = value
        .as_object()
        .and_then(|o| o.get("resourceType"))
        .and_then(Value::as_str);
    let Some(type_name) = type_name else {
        losses.push(Loss {
            path: if path.is_empty() { "(root)".to_string() } else { path.to_string() },
            kind: LossKind::NotAResource,
            detail: "no resourceType; serialize the release's Resource enum, \
                     which carries the tag, rather than the resource struct"
                .to_string(),
        });
        return None;
    };
    let obj = value.as_object()?;
    let here = if path.is_empty() {
        type_name.to_string()
    } else {
        path.to_string()
    };

    if !has_type(target, type_name) {
        losses.push(Loss {
            path: here,
            kind: LossKind::ResourceRemoved,
            detail: format!("no {type_name} in the target release"),
        });
        return None;
    }

    let mut ctx = Ctx {
        source,
        target,
        losses,
    };
    Some(Value::Object(ctx.object(obj, type_name, type_name, &here)))
}

/// Whether a release's table knows a resource or datatype by name.
fn has_type(table: &'static [ElementMeta], name: &str) -> bool {
    let prefix = format!("{name}.");
    table.iter().any(|e| e.path.starts_with(&prefix))
}

/// The walk's fixed state: the two tables and the accumulating report.
struct Ctx<'a> {
    source: &'static [ElementMeta],
    target: &'static [ElementMeta],
    losses: &'a mut Vec<Loss>,
}

impl Ctx<'_> {
    /// Convert every member of one object, in document order.
    ///
    /// `src_context` and `tgt_context` are the FHIR paths (or datatype names)
    /// this object sits at in each release; they differ whenever an element's
    /// type was renamed between the two.
    fn object(
        &mut self,
        obj: &Map<String, Value>,
        src_context: &str,
        tgt_context: &str,
        path: &str,
    ) -> Map<String, Value> {
        // A recursive backbone re-enters an ancestor rather than nesting for
        // ever, so `QuestionnaireResponse.item.answer.item` has no children of
        // its own and must be read as `QuestionnaireResponse.item`.
        let src_context = resolve_recursion(self.source, src_context);
        let tgt_context = resolve_recursion(self.target, tgt_context);
        let mut out = Map::new();

        for (key, value) in obj {
            if key == "resourceType" {
                out.insert(key.clone(), value.clone());
                continue;
            }
            // `_field` carries the primitive extensions of `field`; it stands or
            // falls with the element it annotates.
            let sibling = key.starts_with('_');
            let base = key.strip_prefix('_').unwrap_or(key);
            let here = format!("{path}.{key}");

            let src_meta = meta::resolve(self.source, &format!("{src_context}.{base}"), src_context, base);
            let Some(tgt_meta) =
                meta::resolve(self.target, &format!("{tgt_context}.{base}"), tgt_context, base)
            else {
                // Report a dropped `_field` only when its element is not present
                // to be reported in its own right, so one removal is one loss.
                if !sibling || !obj.contains_key(base) {
                    self.losses.push(Loss {
                        path: here,
                        kind: LossKind::ElementRemoved,
                        detail: format!("{tgt_context} has no {base}"),
                    });
                }
                continue;
            };

            let src_type = src_meta.and_then(|m| chosen_type(m, base));
            let tgt_type = chosen_type(tgt_meta, base);

            // A choice whose variant the target does not offer.
            if tgt_meta.is_choice() && tgt_type.is_none() {
                if !sibling || !obj.contains_key(base) {
                    let allowed = tgt_meta.type_codes().collect::<Vec<_>>().join(", ");
                    self.losses.push(Loss {
                        path: here,
                        kind: LossKind::ChoiceVariantUnsupported,
                        detail: format!("{} allows only: {allowed}", tgt_meta.path),
                    });
                }
                continue;
            }

            // A primitive that became a number, or a complex type that replaced
            // a primitive: the value cannot cross as it stands. `_field`
            // siblings are always `Element`s, so this does not apply to them.
            if !sibling
                && let (Some(s), Some(t)) = (src_type, tgt_type)
                && meta::json_kind(s) != meta::json_kind(t)
            {
                self.losses.push(Loss {
                    path: here,
                    kind: LossKind::TypeChanged,
                    detail: format!("{s} in the source, {t} in the target"),
                });
                continue;
            }

            if !sibling {
                self.check_binding(src_meta, tgt_meta, &here);
            }

            let value = self.fit_cardinality(value, tgt_meta, &here);
            // A `_field` sibling holds an `Element` (id and extensions),
            // whatever the element it annotates is typed as.
            let (child_src, child_tgt) = if sibling {
                ("Element", "Element")
            } else {
                (
                    src_meta.map_or(src_context, |m| child_context(m, src_type)),
                    child_context(tgt_meta, tgt_type),
                )
            };
            let converted = self.value(&value, child_src, child_tgt, tgt_type, &here);
            out.insert(key.clone(), converted);
        }

        self.check_required(&out, tgt_context, path);
        out
    }

    /// Convert one value: recurse into objects and arrays, pass scalars through.
    fn value(
        &mut self,
        value: &Value,
        src_context: &str,
        tgt_context: &str,
        type_code: Option<&str>,
        path: &str,
    ) -> Value {
        match value {
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let at = format!("{path}[{i}]");
                        self.value(item, src_context, tgt_context, type_code, &at)
                    })
                    .collect(),
            ),
            Value::Object(obj) => {
                // `contained`, and `Bundle.entry.resource`, hold whole resources
                // whose context is their own `resourceType`, not this path.
                if type_code == Some("Resource") || obj.contains_key("resourceType") {
                    return convert_resource(self.source, self.target, value, path, self.losses)
                        .unwrap_or(Value::Null);
                }
                Value::Object(self.object(obj, src_context, tgt_context, path))
            }
            other => other.clone(),
        }
    }

    /// Match the value's JSON shape to the target's cardinality.
    ///
    /// FHIR JSON writes a repeating element as an array and a singular one as a
    /// bare value, so an element that repeats in only one of the two releases
    /// has to be wrapped or unwrapped. Wrapping loses nothing and is silent;
    /// unwrapping past the first entry does, and is reported.
    fn fit_cardinality(
        &mut self,
        value: &Value,
        tgt_meta: &'static ElementMeta,
        path: &str,
    ) -> Value {
        let Some(items) = value.as_array() else {
            // Singular in the source, repeating in the target: wrap it. `null`
            // is left alone — it is a placeholder in a `_field` array, not a
            // value to promote.
            if tgt_meta.is_multiple() && !value.is_null() {
                return Value::Array(vec![value.clone()]);
            }
            return value.clone();
        };
        if tgt_meta.is_multiple() || items.len() <= 1 {
            // A one-entry array for a singular target still has to be unwrapped,
            // but nothing is lost by doing it.
            if !tgt_meta.is_multiple() && items.len() == 1 {
                return items[0].clone();
            }
            return value.clone();
        }
        self.losses.push(Loss {
            path: path.to_string(),
            kind: LossKind::CardinalityNarrowed,
            detail: format!(
                "{} entries, but {} is {}..{}",
                items.len(),
                tgt_meta.path,
                tgt_meta.min,
                tgt_meta.max
            ),
        });
        items[0].clone()
    }

    /// Warn when the target binds the element to a different value set with
    /// `required` strength, so a code that was legal may no longer be.
    fn check_binding(
        &mut self,
        src_meta: Option<&'static ElementMeta>,
        tgt_meta: &'static ElementMeta,
        path: &str,
    ) {
        let Some(tgt) = tgt_meta.binding else { return };
        if tgt.strength != meta::BindingStrength::Required {
            return;
        }
        let src = src_meta.and_then(|m| m.binding);
        let same = src.is_some_and(|s| {
            s.strength == meta::BindingStrength::Required
                && canonical_vs(s.value_set) == canonical_vs(tgt.value_set)
        });
        if same {
            return;
        }
        self.losses.push(Loss {
            path: path.to_string(),
            kind: LossKind::BindingChanged,
            detail: match src.and_then(|s| s.value_set) {
                Some(from) => format!("{from} → {}", tgt.value_set.unwrap_or("(none)")),
                None => format!("now required: {}", tgt.value_set.unwrap_or("(none)")),
            },
        });
    }

    /// Report the target's mandatory elements that the converted object lacks.
    ///
    /// This does not repair anything — there is nothing honest to put in a
    /// missing required field — but it is the difference between a document that
    /// will fail validation and one you know will.
    fn check_required(&mut self, out: &Map<String, Value>, tgt_context: &str, path: &str) {
        let prefix = format!("{tgt_context}.");
        for el in self.target.iter().filter(|e| e.path.starts_with(&prefix)) {
            let Some(name) = el.path.strip_prefix(&prefix) else {
                continue;
            };
            // Direct children only; grandchildren are checked when built.
            if !el.is_required() || name.contains('.') {
                continue;
            }
            let present = if el.is_choice() {
                let base = name.trim_end_matches("[x]");
                out.keys().any(|k| meta::choice_suffix(el, k).is_some() && k.starts_with(base))
            } else {
                out.contains_key(name)
            };
            if !present {
                self.losses.push(Loss {
                    path: format!("{path}.{name}"),
                    kind: LossKind::RequiredMissing,
                    detail: format!("{} is {}..{}", el.path, el.min, el.max),
                });
            }
        }
    }
}

/// Follow a recursive backbone back to the path it re-enters.
///
/// FHIR expresses recursion with `contentReference`: `Questionnaire.item.item`
/// does not restate the item's elements, it points at `Questionnaire.item`. The
/// generated table therefore has no children under the deeper path, and a walk
/// that took that at face value would report every element of every nested item
/// as missing from the target — which is what it did before this existed.
///
/// The element's own `contentReference` says where, so this is a lookup, not a
/// guess. Guessing was tried: matching on the final path segment resolves
/// `Questionnaire.item.item` correctly but sends
/// `TestScript.test.action.operation` to whichever `…operation` it finds first,
/// and `QuestionnaireResponse.item.item` to `Claim.item`.
///
/// References can chain, so this follows them, with a bound in case a future
/// specification ever ships a cycle.
fn resolve_recursion<'a>(table: &'static [ElementMeta], context: &'a str) -> &'a str {
    let mut at = context;
    for _ in 0..8 {
        if has_type(table, at) {
            return at;
        }
        match meta::find(table, at).and_then(|e| e.content_reference) {
            Some(target) => at = target,
            None => return at,
        }
    }
    at
}

/// The context a child of this element sits in: a named datatype switches to
/// that type, a backbone keeps the element's own path.
/// A `contentReference` element (a recursive backbone) declares no type at all;
/// its own path is the right answer there too, because [`resolve_recursion`]
/// maps that path back to the one it re-enters.
fn child_context(el: &'static ElementMeta, type_code: Option<&'static str>) -> &'static str {
    match type_code {
        Some(code) if meta::is_datatype(code) => code,
        _ => el.path,
    }
}

/// The single type this key selects: for a choice, the one its suffix names;
/// otherwise the element's only type.
fn chosen_type(el: &'static ElementMeta, key: &str) -> Option<&'static str> {
    if el.is_choice() {
        let suffix = meta::choice_suffix(el, key)?;
        return el.type_codes().find(|c| c.eq_ignore_ascii_case(suffix));
    }
    el.types.first().map(|t| t.code)
}

/// A value-set URL without its `|version` suffix, so R4's
/// `…/observation-status` and R5's `…/observation-status|5.0.0` compare equal.
fn canonical_vs(url: Option<&'static str>) -> Option<&'static str> {
    url.map(|u| u.split('|').next().unwrap_or(u))
}

#[cfg(test)]
mod tests {
    use super::*;

    // The engine is release-agnostic, so its unit tests build small tables by
    // hand rather than depending on a release crate — `fhir-core` cannot see
    // one. The conversions against the real R3/R4/R5 tables live in the `fhir`
    // crate's tests, where the models are in scope.

    const EMPTY: &[&str] = &[];

    macro_rules! el {
        ($path:expr, $min:expr, $max:expr, $ty:expr) => {
            ElementMeta {
                path: $path,
                min: $min,
                max: $max,
                is_summary: false,
                binding: None,
                types: &[TypeRef {
                    code: $ty,
                    target_profiles: EMPTY,
                }],
                content_reference: None,
            }
        };
    }

    use crate::meta::TypeRef;

    static SRC: &[ElementMeta] = &[
        el!("Thing.gone", 0, "1", "string"),
        el!("Thing.kept", 0, "1", "string"),
        el!("Thing.many", 0, "*", "string"),
        el!("Thing.num", 0, "1", "string"),
    ];

    // Sorted by path, as the generated tables are: `meta::find` binary-searches.
    static TGT: &[ElementMeta] = &[
        el!("Thing.kept", 0, "1", "string"),
        el!("Thing.many", 0, "1", "string"),
        el!("Thing.needed", 1, "1", "string"),
        el!("Thing.num", 0, "1", "integer"),
    ];

    #[test]
    fn the_fixtures_are_sorted() {
        for table in [SRC, TGT] {
            assert!(
                table.windows(2).all(|w| w[0].path < w[1].path),
                "an unsorted table silently breaks the binary search in meta::find"
            );
        }
    }

    fn convert(json: &str) -> Converted {
        resource(SRC, TGT, &::serde_json::from_str(json).unwrap())
    }

    #[test]
    fn drops_an_element_the_target_lacks() {
        let out = convert(r#"{"resourceType":"Thing","gone":"x","kept":"y"}"#);
        assert_eq!(out.value["kept"], "y");
        assert!(out.value.get("gone").is_none());
        let loss = out.report.of_kind(LossKind::ElementRemoved).next().unwrap();
        assert_eq!(loss.path, "Thing.gone");
    }

    #[test]
    fn narrows_a_repeating_element_and_says_how_much() {
        let out = convert(r#"{"resourceType":"Thing","many":["a","b","c"]}"#);
        assert_eq!(out.value["many"], "a");
        let loss = out
            .report
            .of_kind(LossKind::CardinalityNarrowed)
            .next()
            .unwrap();
        assert!(loss.detail.contains("3 entries"));
    }

    #[test]
    fn drops_a_value_whose_json_kind_changed() {
        let out = convert(r#"{"resourceType":"Thing","num":"12"}"#);
        assert!(out.value.get("num").is_none());
        assert_eq!(
            out.report.of_kind(LossKind::TypeChanged).count(),
            1,
            "a string cannot be carried into an integer element"
        );
    }

    #[test]
    fn reports_a_required_element_it_cannot_invent() {
        let out = convert(r#"{"resourceType":"Thing","kept":"y"}"#);
        let loss = out.report.of_kind(LossKind::RequiredMissing).next().unwrap();
        assert_eq!(loss.path, "Thing.needed");
        assert!(
            !loss.kind.discards_data(),
            "nothing was dropped; the result merely will not validate"
        );
    }

    #[test]
    fn a_document_with_no_resource_type_is_reported_not_silently_nulled() {
        // Serializing a bare resource struct lands here, because `resourceType`
        // comes from the release's `Resource` enum tag. Returning null with an
        // empty report would be a silent failure.
        let out = resource(SRC, TGT, &::serde_json::json!({"kept": "y"}));
        assert_eq!(out.value, Value::Null);
        assert!(!out.report.is_lossless(), "a null result needs an explanation");
        assert_eq!(out.report.of_kind(LossKind::NotAResource).count(), 1);
    }

    #[test]
    fn an_unknown_resource_type_yields_null_not_an_empty_object() {
        let out = resource(SRC, TGT, &::serde_json::json!({"resourceType": "Other"}));
        assert_eq!(out.value, Value::Null);
        assert_eq!(out.report.of_kind(LossKind::ResourceRemoved).count(), 1);
    }

    #[test]
    fn a_lossless_conversion_says_so() {
        let out = convert(r#"{"resourceType":"Thing","kept":"y","needed":"z"}"#);
        assert!(out.report.is_lossless(), "{}", out.report);
        assert!(!out.report.discarded_data());
    }

    #[test]
    fn strict_passes_a_clean_conversion_through() {
        let out = convert(r#"{"resourceType":"Thing","kept":"y","needed":"z"}"#);
        let value = out.strict().expect("nothing was lost");
        assert_eq!(value["kept"], "y");
    }

    #[test]
    fn strict_refuses_a_lossy_one_and_hands_back_the_reason() {
        let out = convert(r#"{"resourceType":"Thing","gone":"x","needed":"z"}"#);
        let report = out.strict().expect_err("an element was dropped");
        assert_eq!(report.of_kind(LossKind::ElementRemoved).count(), 1);
    }

    #[test]
    fn strict_refuses_a_warning_too_even_though_nothing_was_dropped() {
        // `needed` is absent, so nothing was discarded — but the result will
        // not validate in the target, which a strict caller must not be handed
        // without being told.
        let out = convert(r#"{"resourceType":"Thing","kept":"y"}"#);
        assert!(!out.report.discarded_data(), "nothing was dropped");
        assert!(out.strict().is_err(), "and yet it must not pass strict");
    }

    #[test]
    fn a_primitive_extension_sibling_follows_its_element() {
        // `_gone` annotates an element the target does not have, so it goes too
        // — and the pair is reported once, not twice.
        let out = convert(r#"{"resourceType":"Thing","gone":"x","_gone":{"id":"a"}}"#);
        assert!(out.value.get("_gone").is_none());
        assert_eq!(out.report.of_kind(LossKind::ElementRemoved).count(), 1);
    }
}
