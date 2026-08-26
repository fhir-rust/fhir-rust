# Round-trip failures — historical burn-down record (T2/T28)

**Historical, not operative** (relabelled 2026-08-06, T45). This was the
burn-down list produced by task **T2**; the *operative* allowlists now live
in the test files themselves — `KNOWN_FAILURES`/`KNOWN_CLASSES` in
`tests/roundtrip_r5_examples.rs`, `_r4_`, and `_r3_` — and the corpus runs
as a hard CI gate (T28). The regenerate command this file used to give
(`cargo test --test roundtrip_official_examples`) names a test target that
no longer exists; the current run is:

```sh
bin/fetch-examples
cargo test --test roundtrip_r5_examples   # and _r4_, _r3_
```

The narrative below — the failure categories and why each was fixed — is
kept as the record of how the corpus went green.

## Summary

**As of the T28 corpus gate: R5 2823/2824, R4 2713/2912, R3 1490/1664.** The
R4 and R3 shortfalls are three understood classes of *non-conformant HL7®
example* — omitted `Questionnaire.item.linkId` (1..1), `SearchParameter.base`
(1..*), and `SearchParameter.code` (1..1) — recorded as counted classes in the
per-release test files. The model is correct in each case; relaxing it to
accept them would make the suite green by making the model wrong.

| Result | T2 baseline | After T7 (`_field`) | After C/D | After B |
|---|---:|---:|---:|---:|
| Files scanned | 2824 | 2824 | 2824 | 2824 |
| **Passed** | **2760** | **2780** | **2801** | **2820** (99.86%) |
| Deserialize failures | 10 | 10 | 1 | 1 |
| Round-trip mismatches | 54 | 34 | 22 | 3 |

**Status.** Resolved:
- **Category A** (`_field` primitive extensions) — rolled across the whole model
  in T6/T7.
- **Category B** — complex datatypes now model the FHIR® `Element` base
  (`id` + `extension`); added to 19 datatype structs by
  `siblings::apply_element_base_group`.
- **Category C** (missing scalar fields) — the fields existed but were dropped by
  acronym-casing (`carrierAIDC`/`carrierHRF`/`requestURL`/`productionIdentifierInUDI`
  serialized as `carrierAidc`/…); fixed with explicit `#[serde(rename)]`. Only
  these four elements in all of R5 have caps acronyms.
- **Category D** — `Range.low`/`Range.high` are 0..1 in R5 but were modelled as
  required; made `Option` (fixed 9 of 10 deserialize failures).

Choice `value[x]` primitive extensions (`_valueCode` on `Extension.value[x]`)
were resolved by T9 (the `ExtensionValue` enum's `Primitive` variants now carry
`_value<Type>`). Full-set round-trip after T9: **2822/2824 (99.93%)**.

Remaining (2 files):
- `ConceptMap.group.element.target.product` — reuses the `dependsOn` structure
  (`attribute` + `value[x]` choice); a genuinely unmodelled field. 1 file.
- `QuestionnaireResponse.questionnaire` — genuinely 1..1 required in R5 (meta
  confirms `min=1`); `questionnaireresponse-example-f201-lifelines.json` omits it
  and is non-conformant. Left required by design (the last deserialize failure).

Historical baseline notes (from T2) follow; categories A–E describe the original
54 mismatches + 10 failures.

---

## A. Primitive-extension `_field` siblings dropped  → **T6 / T7** (Phase 1)

The model has no sibling field for primitive extensions, so the JSON `_birthDate`,
`_gender`, etc. objects are silently dropped. This is exactly what Phase 1 adds.

Affected primitives seen in the example set: `_active`, `_availableEndTime`,
`_availableStartTime`, `_birthDate`, `_code`, `_event`, `_family`, `_gender`,
`_given`, `_profile`, `_type`, `_valueCode`.

Files:

- `activitydefinition-example.json` — `timingTiming/_event`
- `activitydefinition-order-serum-dengue-virus-igm.json` — `timingTiming/_event`
- `activitydefinition-order-serum-zika-dengue-virus-igm.json` — `timingTiming/_event`
- `activitydefinition-predecessor-example.json` — `timingTiming/_event`
- `activitydefinition-provide-mosquito-prevention-advice.json` — `timingTiming/_event`
- `activitydefinition-servicerequest-example.json` — `timingTiming/_event`
- `codesystem-discriminator-type.json` — `concept/2/extension/0/_valueCode`
- `codesystem-map-transform.json` — `property/0/_code`
- `codesystem-search-entry-mode.json` — `concept/2/extension/0/_valueCode`
- `organization-example.json` — `_availableStartTime`, `_availableEndTime`
- `patient-example-b.json` — `_gender`
- `patient-example-dicom.json` — `_gender`
- `patient-example-infant-twin-1.json` — `_birthDate`
- `patient-example-infant-twin-2.json` — `_birthDate`
- `patient-example-newborn.json` — `_birthDate`
- `patient-example.json` — `_birthDate`, `contact/0/name/_family`
- `plandefinition-example-kdn5-simplified.json` — `relatedArtifact/1/_type`
- `relatedperson-example.json` — `name/0/_family`
- `structuredefinition-example-composition.json` — `differential/element/*/type/0/_profile`
- `testscript-example-history.json` — `_profile`
- `testscript-example-readtest.json` — `_profile`
- `testscript-example-search.json` — `_profile`, plus category C below
- `testscript-example-update.json` — `_profile`
- `json-edge-cases.json` — `_active`, `contact/0/name/_family`, `contact/0/name/_given`
  (also categories B below)

## B. `id` / `extension` on complex datatypes dropped  → extension work (relate to T7 / T17)

Complex datatypes (`Coding`, `Reference`, `Attachment`, `CodeableConcept`,
`HumanName`, …) do not model the FHIR `Element` base (`id`, `extension`), so
extensions and element ids attached to a datatype instance are dropped. Not
called out as its own task in `tasks.md` — recommend folding into T7 (it touches
the same generator/struct surface) or adding a dedicated task before T17.

Files:

- `observation-example-10minute-apgar-score.json` — `component/*/valueCodeableConcept/coding/0/extension`
- `observation-example-1minute-apgar-score.json` — same shape
- `observation-example-20minute-apgar-score.json` — same shape
- `observation-example-2minute-apgar-score.json` — same shape
- `observation-example-5minute-apgar-score.json` — same shape
- `observation-example-glasgow.json` — `component/*/valueCodeableConcept/coding/1/extension`
- `provenance-example1.json` — `target/0/extension` (Reference.extension)
- `provenance-example2.json` — `target/0/extension`
- `provenance-example3.json` — `target/0/extension`
- `questionnaireresponse-example-gcs.json` — `item/*/answer/0/valueCoding/extension`
- `plandefinition-example.json` — `.../document/extension` (Attachment.extension)
- `plandefinition-opioidcds-04.json` — `.../document/extension`
- `plandefinition-opioidcds-05.json` — `.../document/extension`
- `plandefinition-opioidcds-07.json` — `.../document/extension`
- `plandefinition-opioidcds-08.json` — `.../document/extension`
- `plandefinition-opioidcds-10.json` — `.../document/extension` (×5)
- `plandefinition-opioidcds-11.json` — `.../document/extension`
- `patient-example-c.json` — `name/0/id`, `name/1/id` (HumanName.id)
- `json-edge-cases.json` — `maritalStatus/extension`, `extension/1/extension/0/valueCoding/extension`

## C. Missing scalar fields on specific types  → unmapped model bugs (fix opportunistically)

Genuine omissions in the generated model, unrelated to extensions. Cheap,
independent fixes; not tied to a plan task.

- **Device.udiCarrier.carrierHRF / carrierAIDC** missing:
  - `device-example-AND20601BPMonitor.json`, `device-example-ANDThermometer.json`,
    `device-example-Bodimetrics.json`, `device-example-KinsaThermometer.json`,
    `device-example-Nonin20601PulseOx.json`, `device-example-NoninBlePulseOx.json`,
    `device-example-PhilipsThermometer.json`, `device-example-RocheGlucoseMonitor.json`,
    `device-example-udi1.json` (also `carrierAIDC`), `device-example-udi3.json`,
    `device-example-udi4.json`
- **ConceptMap …target.product.attribute / valueCode** missing:
  - `conceptmap-example-specimen-type.json`
- **TestScript …assert.requestUrl** missing:
  - `testscript-example-search.json`

## D. Over-strict required fields — deserialize failures  → model cardinality bugs

The model marks these fields as required (non-`Option`) but FHIR allows them to
be absent, so deserialization fails outright. All should become `Option`.

- **`Range.low` / `Range.high` should be `Option`** (FHIR cardinality 0..1):
  - `missing field high`: `activitydefinition-administer-zika-virus-exposure-assessment.json`,
    `library-zika-virus-intervention-logic.json`, `observation-example-f205-egfr.json`,
    `observationdefinition-example.json`, `plandefinition-zika-virus-intervention.json`,
    `testscript-example.json`
  - `missing field low`: `biologicallyderivedproduct-example-autologousHCT.json`,
    `plandefinition-protocol-example.json`, `riskassessment-example.json`
- **`QuestionnaireResponse.questionnaire`** required in model but absent in example:
  - `questionnaireresponse-example-f201-lifelines.json` (`missing field questionnaire`) —
    confirm true FHIR cardinality before relaxing.

## E. Notes

- The full-run test does not panic on first failure; it prints the whole report,
  so this list can be re-diffed after each fix.
- The always-on `roundtrip_curated_subset` test (in
  `tests/data/roundtrip_examples/`) deliberately contains only files that pass
  today, so it stays green as a regression guard while A–D are burned down.
- As each category is fixed, move representative files from the failing set into
  the curated subset so future regressions are caught by the always-on test.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
