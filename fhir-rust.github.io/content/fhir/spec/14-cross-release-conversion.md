# 14 — Cross-release conversion

Defines how a resource is moved from one FHIR® release to another, and what the
caller is told about what that cost.

## Background

Spec 12 establishes that the releases share no model types and that no
`From`/`Into` exists between them (R12.4). That decision stands, and this spec
does not weaken it: a conversion that compiled silently would be a conversion
whose losses were invisible, and in a health record an invisible loss is worse
than a loud failure.

But callers still have to do it. Cross-version exchange is routine — a national
deployment on R4 talking to a partner on R5, an archive of R3 documents being
migrated — and until now the only answer this crate offered was "serialize to
JSON and see what the target refuses" (spec 13, Future work). That answer has a
hole in it. Serde reports the *first* error and stops, so a document with three
problems reveals one. Worse, the most common difference between releases is an
element the target simply does not have, and that is not an error at all: serde
ignores unknown keys, so the field vanishes and nothing is reported. The
mechanism that was supposed to make conversion safe is quietest exactly where
the data loss is.

So the requirement is not "convert between releases". It is **convert between
releases and account for every difference**.

## What is converted

- **R14.1** Conversion MUST operate on the **wire form** — a `serde_json::Value`
  — and MUST NOT introduce any type shared between releases. This is what keeps
  R12.4 intact: no R4 type ever becomes an R5 type, and no common supertype is
  invented to let them meet.
- **R14.2** Conversion MUST be **explicit**. It MUST NOT be reachable through
  `From`, `Into`, `Deref`, or any other coercion the compiler applies on the
  caller's behalf.
- **R14.3** Conversion MUST be driven by the two releases' generated
  `ElementMeta` tables (spec 08), not by hand-written per-resource rules. A
  release that is added to the workspace MUST become convertible by existing,
  without anyone editing the conversion layer.
- **R14.4** Every ordered pair of modelled releases MUST be convertible, in both
  directions, including pairs that skip a release (R3 → R5) and the degenerate
  pair of a release with itself.

## The loss report

- **R14.5** Conversion MUST return a **loss report** alongside the converted
  document, naming every difference it acted on. Each entry MUST carry the path
  it occurred at, a kind, and a human-readable detail.
- **R14.6** The report MUST distinguish losses that **discarded data** from
  warnings about data that was **kept**, because the two demand different
  responses from a caller.
- **R14.7** The following MUST be reported:

  | Kind | Meaning | Data |
  | --- | --- | --- |
  | `ElementRemoved` | the target has no element at this path | dropped |
  | `ResourceRemoved` | the target has no such resource type | dropped |
  | `ChoiceVariantUnsupported` | a `value[x]` type the target's choice does not admit | dropped |
  | `CardinalityNarrowed` | repeats in the source, single in the target | truncated |
  | `TypeChanged` | the element's JSON kind differs between the releases | dropped |
  | `RequiredMissing` | the target requires an element the result does not have | kept |
  | `BindingChanged` | the target binds the element to a different value set with `required` strength | kept |

- **R14.8** A conversion MUST NOT invent a value for a `RequiredMissing`
  element. There is no honest default for a missing clinical field, and
  supplying one would produce a document that validates and lies.
- **R14.9** Where a difference is purely representational and loses nothing, it
  MUST be applied **silently**. An element that is singular in the source and
  repeating in the target is wrapped in an array and is not a loss; a report
  padded with non-losses is a report nobody reads.

## Refusing instead of converting

- **R14.15** A **strict** mode MUST be offered, which yields a document only
  when the conversion was lossless and otherwise yields the report. For a
  receiver that is a clinical system, a dropped element is a dropped fact and
  the receiver cannot tell it was ever there; refusing the exchange is a
  legitimate — often the correct — response.
- **R14.16** Strict mode MUST reject on *any* report entry, not only on the
  ones that discarded data. A `RequiredMissing` means the result will not
  validate in the target and a `BindingChanged` means a code that was legal may
  no longer be, so neither is something a strict caller should receive
  silently. Measured against the committed corpora this rejects no more than it
  should: entries that warn without discarding account for one document per
  release pair, against roughly half of each corpus that converts cleanly.

## Fidelity

- **R14.10** Converting a release **to itself** MUST be lossless and MUST return
  a document equal to its input. This is the layer's cleanest available oracle:
  whatever the tables say, a release can always represent its own documents, so
  anything dropped is a defect in the walk rather than a difference between
  releases.
- **R14.11** A converted document that the target's model **rejects** MUST have
  been predicted by the report. Concretely: if deserializing the result into the
  target's `Resource` fails, the report MUST contain an entry that accounts for
  the failure. A parse failure the report did not foresee is a silent loss, and
  is the defect this spec exists to prevent.
- **R14.12** Recursive backbone elements MUST be resolved through the
  `contentReference` recorded in the element table, not inferred from the path.
  `Questionnaire.item.item` re-enters `Questionnaire.item`, but
  `TestScript.test.action.operation` re-enters
  `TestScript.setup.action.operation` — a *sibling* subtree — so no rule over
  path segments gets both right. Getting this wrong reports every element of
  every nested item as absent from the target, which R14.10 catches.

## What this layer does not do

- **R14.13** Conversion is **structural**, and its limits MUST be documented as
  such. It knows which elements each release has, which types a choice admits,
  what repeats, and what is required. It does **not** know that one release's
  element was *renamed* into another's, or that a value's meaning shifted under
  a stable name.
- **R14.14** A renamed element MUST be reported as `ElementRemoved` rather than
  guessed at. Inventing a mapping is precisely the silent mangling that R12.4
  exists to prevent, and a wrong mapping is worse than an absent one because it
  is invisible in the result.

## Acceptance criteria

1. Converting each committed official example from a release to **itself**
   returns an equal document and an empty loss report (R14.10).
2. For every committed R4 example converted to R5, and every R3 example
   converted to R4, either the result deserializes into the target's `Resource`
   or the report names the field the target rejected (R14.11). A test asserts
   that at least one example exercises the failing path, so the check cannot go
   vacuous.
3. `Patient.animal` (R3, removed in R4) converts to a document without it and a
   report naming it (R14.7).
4. `Observation.valueAttachment` (admitted in R3, not in R4) is dropped and
   reported as an unsupported choice variant (R14.7).
5. A resource type the target lacks yields a null document and a
   `ResourceRemoved` entry, rather than an empty object (R14.7).
6. The element table carries `contentReference`, and the generator emits it
   (R14.12).
7. Strict mode passes a lossless conversion through, refuses one that dropped
   an element, and refuses one whose only entry is a warning about data it kept
   (R14.15, R14.16).

## Future work

- Driving the semantic remappings of R14.13 from HL7®'s official cross-version
  extension maps, which state them explicitly. That would let a renamed element
  be carried across *and* reported as a rename, rather than reported as a
  removal. It is the natural next step and needs the map packages vendored
  alongside the definitions.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
