//! SNOMED CT® terminology operations (`SV2.20`): `CodeSystem/$lookup`,
//! `CodeSystem/$subsumes` and `ValueSet/$expand`, by `GET`.
//!
//! Deliberately thin, for `SV1.1`'s reason: the semantics are
//! `snomed-fhir`'s, and this layer parses parameters, calls it, and
//! translates the result and its errors into FHIR. Every parameter it does
//! not serve is refused by name (`SV2.13`), and no `OperationOutcome` here
//! echoes a submitted value (`SV2.7`) — which is why `snomed-fhir`'s own
//! error messages, which do, are never returned verbatim.

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::Response as AxumResponse;
use loco_rs::prelude::*;
use serde_json::{json, Value};
use snomed::core::concrete_value::ConcreteValue;
use snomed::core::constants;
use snomed::core::sctid::SctId;
use snomed::fhir::{
    DesignationUse, ExpandOptions, Expansion, FhirError, LookupProperty, LookupResult,
    SNOMED_CT_SYSTEM,
};

use super::fhir::{fhir_json, outcome, version_of};
use crate::terminology::{self, Terminology};

/// What a handler step fails with: the whole response, for the reason
/// `version_of` gives.
type Refusal = AxumResponse;

fn bad_request(text: &str) -> Refusal {
    outcome(StatusCode::BAD_REQUEST, "error", "invalid", text)
}

fn not_supported(text: &str) -> Refusal {
    outcome(StatusCode::BAD_REQUEST, "error", "not-supported", text)
}

/// The loaded release, or a `404` naming the setting that loads one.
#[allow(clippy::result_large_err)]
pub(crate) fn loaded(
    release: Option<&'static Terminology>,
) -> Result<&'static Terminology, Refusal> {
    release.ok_or_else(|| {
        outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-supported",
            &format!(
                "no SNOMED CT release is loaded; set {} to serve terminology operations (SV2.20)",
                terminology::ENV_RF2
            ),
        )
    })
}

/// The request's query parameters, checked against what an operation
/// serves.
struct Params(Vec<(String, String)>);

impl Params {
    /// Refuse the first parameter not in `served`, by name (`SV2.13`).
    #[allow(clippy::result_large_err)]
    fn only(self, operation: &str, served: &[&str]) -> Result<Self, Refusal> {
        if let Some((name, _)) = self.0.iter().find(|(n, _)| !served.contains(&n.as_str())) {
            return Err(not_supported(&format!(
                "{operation} does not serve the parameter `{name}`; served: {}",
                served.join(", ")
            )));
        }
        Ok(self)
    }

    fn all<'a>(&'a self, name: &str) -> impl Iterator<Item = &'a str> + 'a {
        let name = name.to_owned();
        self.0
            .iter()
            .filter(move |(n, _)| *n == name)
            .map(|(_, v)| v.as_str())
    }

    /// A single-valued parameter: absent is `None`, repeated is refused.
    #[allow(clippy::result_large_err)]
    fn single(&self, name: &str) -> Result<Option<&str>, Refusal> {
        let mut values = self.all(name);
        let first = values.next();
        if values.next().is_some() {
            return Err(bad_request(&format!(
                "the parameter `{name}` may appear only once"
            )));
        }
        Ok(first)
    }

    #[allow(clippy::result_large_err)]
    fn required(&self, name: &str) -> Result<&str, Refusal> {
        self.single(name)?
            .ok_or_else(|| bad_request(&format!("the parameter `{name}` is required")))
    }

    #[allow(clippy::result_large_err)]
    fn code(&self, name: &str) -> Result<SctId, Refusal> {
        SctId::parse(self.required(name)?).map_err(|_| {
            bad_request(&format!(
                "the parameter `{name}` is not a valid SNOMED CT identifier"
            ))
        })
    }

    #[allow(clippy::result_large_err)]
    fn flag(&self, name: &str) -> Result<bool, Refusal> {
        match self.single(name)? {
            None => Ok(false), // RCA-IN-3: absent only
            Some("true") => Ok(true),
            Some("false") => Ok(false),
            Some(_) => Err(bad_request(&format!(
                "the parameter `{name}` must be true or false"
            ))),
        }
    }

    #[allow(clippy::result_large_err)]
    fn whole(&self, name: &str) -> Result<Option<usize>, Refusal> {
        self.single(name)?
            .map(|v| {
                v.parse::<usize>().map_err(|_| {
                    bad_request(&format!(
                        "the parameter `{name}` must be a non-negative whole number"
                    ))
                })
            })
            .transpose()
    }
}

/// A requested version is served only when it is the loaded release's
/// (`SV2.20`; snomed-rust `spec/11-fhir.md`, "System and version URIs").
#[allow(clippy::result_large_err)]
fn check_version(release: &Terminology, requested: Option<&str>) -> Result<(), Refusal> {
    match requested {
        None => Ok(()),
        Some(v) if release.version.as_deref() == Some(v) => Ok(()),
        Some(_) => Err(outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            match release.version {
                Some(_) => "the requested SNOMED CT version is not the loaded release",
                None => {
                    "no SNOMED CT version is configured, so a requested version cannot be verified"
                }
            },
        )),
    }
}

/// Translate a `snomed-fhir` error. Its messages echo input, so each is
/// restated in terms of the parameter (`SV2.7`).
fn fhir_error(e: &FhirError) -> Refusal {
    match e {
        FhirError::UnsupportedSystem(_) => not_supported(&format!(
            "the `system` must be {SNOMED_CT_SYSTEM}; it is the only code system served"
        )),
        FhirError::UnknownCode(_) => outcome(
            StatusCode::NOT_FOUND,
            "error",
            "not-found",
            "the code is not a concept in the loaded SNOMED CT release",
        ),
        FhirError::UnsupportedProperty(_) => not_supported(
            "a requested `property` is not supported; supported: inactive, moduleId, \
             sufficientlyDefined, parent, child, or a concept model attribute SCTID",
        ),
        FhirError::MissingClassification(_) => not_supported(
            "normalForm and normalFormTerse are not served: this server computes no \
             classification (SV2.20)",
        ),
        FhirError::UnsupportedValueSet(_) => not_supported(
            "the `url` is not a SNOMED CT implicit value set; served: ?fhir_vs, \
             ?fhir_vs=isa/[sctid], ?fhir_vs=refset, ?fhir_vs=refset/[sctid], ?fhir_vs=ecl/[ecl]",
        ),
        FhirError::MalformedUrlEncoding(_) => {
            bad_request("the `url` has malformed percent-encoding")
        }
        FhirError::InvalidEcl(_) => bad_request("the ECL in the `url` does not parse"),
        other => {
            tracing::error!(error = %other, "unexpected snomed-fhir error");
            outcome(
                StatusCode::INTERNAL_SERVER_ERROR,
                "error",
                "exception",
                "the terminology operation failed",
            )
        }
    }
}

/// The version-path and release checks every operation starts with.
#[allow(clippy::result_large_err)]
fn preflight(version: &str) -> Result<&'static Terminology, Refusal> {
    version_of(version)?;
    loaded(terminology::get())
}

/// `GET /{version}/CodeSystem/$lookup`
#[debug_handler]
async fn lookup(
    Path(version): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
) -> AxumResponse {
    match lookup_response(&version, Params(params)) {
        Ok(r) | Err(r) => r,
    }
}

#[allow(clippy::result_large_err)]
fn lookup_response(version: &str, params: Params) -> Result<AxumResponse, Refusal> {
    let release = preflight(version)?;
    let params = params.only("$lookup", &["system", "code", "version", "property"])?;
    let system = params.required("system")?;
    let code = params.code("code")?;
    let requested = params.single("version")?;
    check_version(release, requested)?;
    let properties: Vec<&str> = params.all("property").collect();
    let result = snomed::fhir::lookup(
        &release.store,
        system,
        code,
        release.version.as_deref(),
        release.language_refset,
        &properties,
        None,
    )
    .map_err(|e| fhir_error(&e))?;
    Ok(fhir_json(
        StatusCode::OK,
        &lookup_parameters(&result, version == "r5")?,
        None,
    ))
}

/// `$lookup`'s output `Parameters`. `definition` is an R5 output; R3 and
/// R4 have no such parameter.
#[allow(clippy::result_large_err)]
fn lookup_parameters(result: &LookupResult, with_definition: bool) -> Result<Value, Refusal> {
    let mut parameter = vec![json!({ "name": "name", "valueString": result.name })];
    if let Some(v) = &result.version {
        parameter.push(json!({ "name": "version", "valueString": v }));
    }
    if let Some(d) = &result.display {
        parameter.push(json!({ "name": "display", "valueString": d }));
    }
    if let (true, Some(d)) = (with_definition, &result.definition) {
        parameter.push(json!({ "name": "definition", "valueString": d }));
    }
    for d in &result.designation {
        let mut part = vec![json!({ "name": "language", "valueCode": d.language_code })];
        if let Some(coding) = designation_use(d.use_) {
            part.push(json!({ "name": "use", "valueCoding": coding }));
        }
        part.push(json!({ "name": "value", "valueString": d.value }));
        parameter.push(json!({ "name": "designation", "part": part }));
    }
    for p in &result.property {
        let (key, value) = property_value(p)?;
        parameter.push(json!({
            "name": "property",
            "part": [
                { "name": "code", "valueCode": p.code() },
                { "name": "value", key: value },
            ],
        }));
    }
    Ok(json!({ "resourceType": "Parameters", "parameter": parameter }))
}

/// A designation's acceptability in the configured language refset, as
/// the SNOMED CT acceptability concept. `Unspecified` carries no `use`.
fn designation_use(use_: DesignationUse) -> Option<Value> {
    let (code, display) = match use_ {
        DesignationUse::Preferred => (constants::PREFERRED, "Preferred"),
        DesignationUse::Acceptable => (constants::ACCEPTABLE, "Acceptable"),
        DesignationUse::Unspecified => return None,
    };
    Some(json!({ "system": SNOMED_CT_SYSTEM, "code": code.to_string(), "display": display }))
}

/// A property's `value[x]` key and value.
#[allow(clippy::result_large_err)]
fn property_value(p: &LookupProperty) -> Result<(&'static str, Value), Refusal> {
    Ok(match p {
        LookupProperty::Inactive(b) | LookupProperty::SufficientlyDefined(b) => {
            ("valueBoolean", json!(b))
        }
        LookupProperty::ModuleId(id) | LookupProperty::Parent(id) | LookupProperty::Child(id) => {
            ("valueCode", json!(id.to_string()))
        }
        LookupProperty::ConceptModelAttribute { value, .. } => {
            ("valueCode", json!(value.to_string()))
        }
        LookupProperty::ConceptModelConcreteValue { value, .. } => concrete(value),
        LookupProperty::NormalForm(s) | LookupProperty::NormalFormTerse(s) => {
            ("valueString", json!(s))
        }
        other => {
            tracing::error!(property = ?other, "unrepresentable $lookup property");
            return Err(outcome(
                StatusCode::INTERNAL_SERVER_ERROR,
                "error",
                "exception",
                "a $lookup property could not be represented",
            ));
        }
    })
}

/// A concrete value, keeping a decimal's written precision: the number is
/// parsed from its own text, and `serde_json`'s `arbitrary_precision`
/// keeps every digit (the same fidelity rule as spec `M3.6`).
fn concrete(value: &ConcreteValue) -> (&'static str, Value) {
    match value {
        ConcreteValue::Number(text) => {
            let key = if text.contains('.') {
                "valueDecimal"
            } else {
                "valueInteger"
            };
            match serde_json::from_str::<serde_json::Number>(text) {
                Ok(n) => (key, Value::Number(n)),
                Err(_) => ("valueString", json!(text)),
            }
        }
        ConcreteValue::String(s) => ("valueString", json!(s)),
    }
}

/// `GET /{version}/CodeSystem/$subsumes`
#[debug_handler]
async fn subsumes(
    Path(version): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
) -> AxumResponse {
    match subsumes_response(&version, Params(params)) {
        Ok(r) | Err(r) => r,
    }
}

#[allow(clippy::result_large_err)]
fn subsumes_response(version: &str, params: Params) -> Result<AxumResponse, Refusal> {
    let release = preflight(version)?;
    let params = params.only("$subsumes", &["system", "codeA", "codeB", "version"])?;
    let system = params.required("system")?;
    let code_a = params.code("codeA")?;
    let code_b = params.code("codeB")?;
    check_version(release, params.single("version")?)?;
    let result = snomed::fhir::subsumes(&release.store, system, code_a, code_b)
        .map_err(|e| fhir_error(&e))?;
    let body = json!({
        "resourceType": "Parameters",
        "parameter": [{ "name": "outcome", "valueCode": result.as_fhir_code() }],
    });
    Ok(fhir_json(StatusCode::OK, &body, None))
}

/// `GET /{version}/ValueSet/$expand`
#[debug_handler]
async fn expand(
    Path(version): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
) -> AxumResponse {
    // An expansion can be the whole release: run it off the async workers.
    let respond = move || match expand_response(&version, Params(params)) {
        Ok(r) | Err(r) => r,
    };
    match tokio::task::spawn_blocking(respond).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "$expand task failed");
            outcome(
                StatusCode::INTERNAL_SERVER_ERROR,
                "error",
                "exception",
                "the terminology operation failed",
            )
        }
    }
}

/// The canonical, unversioned form of an implicit value set `url`. A
/// version-prefixed url is served only for the loaded release (`SV2.20`);
/// any other base passes through for `snomed-fhir` to refuse.
#[allow(clippy::result_large_err)]
fn canonical_url(release: &Terminology, url: &str) -> Result<String, Refusal> {
    let Some((base, query)) = url.split_once('?') else {
        return Ok(url.to_string());
    };
    if base == SNOMED_CT_SYSTEM || !base.starts_with("http://snomed.info/sct/") {
        return Ok(url.to_string());
    }
    check_version(release, Some(base))?;
    Ok(format!("{SNOMED_CT_SYSTEM}?{query}"))
}

#[allow(clippy::result_large_err)]
fn expand_response(version: &str, params: Params) -> Result<AxumResponse, Refusal> {
    let release = preflight(version)?;
    let params = params.only(
        "$expand",
        &[
            "url",
            "filter",
            "count",
            "offset",
            "activeOnly",
            "includeDesignations",
        ],
    )?;
    let url = params.required("url")?;
    let canonical = canonical_url(release, url)?;
    let options = ExpandOptions {
        active_only: params.flag("activeOnly")?,
        include_designations: params.flag("includeDesignations")?,
        language_refset: release.language_refset,
        filter: params.single("filter")?,
        count: params.whole("count")?,
        offset: params.whole("offset")?.unwrap_or(0), // RCA-IN-3: absent only
    };
    let expansion =
        snomed::fhir::expand(&release.store, &canonical, &options).map_err(|e| fhir_error(&e))?;
    // An offset past the end is refused, never clamped to an empty page that
    // looks like the end of a shorter expansion (RCA-IN-1).
    if options.offset > expansion.total {
        return Err(bad_request("the `offset` is past the end of the expansion"));
    }
    Ok(fhir_json(
        StatusCode::OK,
        &value_set(release, url, &expansion),
        None,
    ))
}

/// The expansion as a `ValueSet`. Each entry carries `inactive`, read from
/// the release (snomed-rust `spec/11-fhir.md` rule 7).
fn value_set(release: &Terminology, url: &str, expansion: &Expansion) -> Value {
    let contains: Vec<Value> = expansion
        .contains
        .iter()
        .map(|c| {
            let mut entry = json!({
                "system": c.system,
                "code": c.code.to_string(),
                "inactive": !release.store.is_active(c.code),
            });
            if let Some(d) = &c.display {
                entry["display"] = json!(d);
            }
            if !c.designation.is_empty() {
                let designations: Vec<Value> = c
                    .designation
                    .iter()
                    .map(|d| {
                        let mut v = json!({ "language": d.language_code, "value": d.value });
                        if let Some(coding) = designation_use(d.use_) {
                            v["use"] = coding;
                        }
                        v
                    })
                    .collect();
                entry["designation"] = json!(designations);
            }
            entry
        })
        .collect();
    json!({
        "resourceType": "ValueSet",
        "url": url,
        "status": "active",
        "expansion": {
            "timestamp": jiff::Timestamp::now().to_string(),
            "total": expansion.total,
            "offset": expansion.offset,
            "contains": contains,
        },
    })
}

/// Declare the operations on the `CodeSystem` and `ValueSet` entries of a
/// CapabilityStatement's resource list — exactly when they are served
/// (`SV2.9`).
pub(crate) fn declare_operations(resources: &mut [Value], served: bool) {
    if !served {
        return;
    }
    let op = |name: &str, definition: &str| json!({ "name": name, "definition": format!("http://hl7.org/fhir/OperationDefinition/{definition}") });
    for r in resources {
        match r["type"].as_str() {
            Some("CodeSystem") => {
                r["operation"] = json!([
                    op("lookup", "CodeSystem-lookup"),
                    op("subsumes", "CodeSystem-subsumes")
                ]);
            }
            Some("ValueSet") => r["operation"] = json!([op("expand", "ValueSet-expand")]),
            _ => {}
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/{version}/CodeSystem/$lookup", get(lookup))
        .add("/{version}/CodeSystem/$subsumes", get(subsumes))
        .add("/{version}/ValueSet/$expand", get(expand))
        .layer(axum::middleware::from_fn(super::fhir::require_credential))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_a_release_the_operations_name_the_setting() {
        let refusal = loaded(None).err().expect("no release");
        assert_eq!(refusal.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn operations_are_declared_only_when_served() {
        let mut resources = vec![
            json!({ "type": "CodeSystem" }),
            json!({ "type": "ValueSet" }),
            json!({ "type": "Patient" }),
        ];
        declare_operations(&mut resources, false);
        assert!(resources.iter().all(|r| r.get("operation").is_none()));
        declare_operations(&mut resources, true);
        assert_eq!(resources[0]["operation"][0]["name"], "lookup");
        assert_eq!(resources[0]["operation"][1]["name"], "subsumes");
        assert_eq!(resources[1]["operation"][0]["name"], "expand");
        assert!(resources[2].get("operation").is_none());
    }

    #[test]
    fn concrete_values_keep_their_written_precision() {
        assert_eq!(
            concrete(&ConcreteValue::Number("1.50".into())),
            (
                "valueDecimal",
                serde_json::from_str::<Value>("1.50").unwrap()
            )
        );
        assert_eq!(
            serde_json::to_string(&concrete(&ConcreteValue::Number("1.50".into())).1).unwrap(),
            "1.50"
        );
        assert_eq!(
            concrete(&ConcreteValue::Number("12".into())).0,
            "valueInteger"
        );
        assert_eq!(
            concrete(&ConcreteValue::String("250mg".into())),
            ("valueString", json!("250mg"))
        );
    }

    #[test]
    fn repeated_missing_and_malformed_parameters_are_refused() {
        let p = |pairs: &[(&str, &str)]| {
            Params(
                pairs
                    .iter()
                    .map(|(n, v)| (n.to_string(), v.to_string()))
                    .collect(),
            )
        };
        assert!(p(&[("code", "1"), ("code", "2")]).single("code").is_err());
        assert!(p(&[]).required("code").is_err());
        assert!(p(&[("code", "12345")]).code("code").is_err());
        assert!(p(&[("activeOnly", "yes")]).flag("activeOnly").is_err());
        assert_eq!(p(&[]).flag("activeOnly").ok(), Some(false));
        assert!(p(&[("count", "-1")]).whole("count").is_err());
        assert_eq!(p(&[("count", "3")]).whole("count").ok(), Some(Some(3)));
        let refused = p(&[("displayLanguage", "en")])
            .only("$lookup", &["code"])
            .err()
            .expect("refused");
        assert_eq!(refused.status(), StatusCode::BAD_REQUEST);
    }
}
