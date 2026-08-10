# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.1.0 — 2026-08-10

- **R4B modelled**: new crate `fhir-r4b` (FHIR 4.3.0), feature `r4b`,
  module `fhir::r4b` — 141 resources, 44 datatypes, 20 primitives, 531
  code enums, generated from the official definitions and gated against
  the full 3,022-example corpus (11 known failures, each with a stated
  reason).
- **Crates renamed** (owner-directed): `fhir-release-N` → `fhir-rN`,
  matching the module names. The old names' published versions remain on
  crates.io, immutable; new publishes use the new names.
- Versions: `fhir` 3.1.0, `fhir-core` 3.0.1, `fhir-derive-macros` 1.3.0
  (the `r4b` version token). Release crates stay at 3.0.0 for their first
  publish under the new names.
- Known model defects surfaced by the R4B corpus and recorded (monorepo
  audit F-86, F-87): null-padded primitive arrays are rejected, and a
  choice element whose content fails to parse is silently dropped. Both
  affect every release crate and predate R4B.

## Unreleased - 4.0.0-dev

### Added - the typed-`Reference<T>` machinery in every release (T11, phase 1)

The generator now emits what R5's hand-written prototype carried alone: the
generic `Reference<T = Any>` with its zero-sized target marker, the
`ResourceType` trait with one impl per resource (beside each release's
`Resource` enum — the prototype had implemented exactly one, `Patient`), and
`cast`/`into_any`/`resolve`. Additive: the `Any` default keeps a bare
`types::Reference` meaning what it always did, and the wire form is
unchanged. The `r5_drift` gate's `Reference::*` sanctioned entry is gone —
the trees genuinely converged, down to the one recorded cycle-break
difference (`Reference::identifier` vs `Identifier::assigner`; one `Box`
either way, at opposite edges). **Breaking, narrowly**: generated
`Reference` no longer derives `Builder` (the derive does not survive the
generic parameter; R5 dropped it when the marker landed).

### Changed - **breaking**: reference fields are typed by `targetProfile` (T11, phase 2)

Where the specification gives a reference element exactly one target, the
field now says so: `AllergyIntolerance.patient` is `Reference<Patient>`,
`Medication.definition` is `Reference<MedicationKnowledge>` — 376 fields in
R5 alone, and proportionally in every release. Multiple targets
(`Observation.subject`) and abstract ones (`Resource`, which names the
tagged enum, not a markable struct) stay `Reference<Any>` via the default
parameter; the wire form is identical either way, and `resolve` now rejects
a matching id under the wrong `resourceType` when the marker is concrete.
`Default` on `Reference<T>` is a hand-written bound-free impl — the derive's
`T: Default` bound would have broken defaults for targets with required
fields — and the type-cycle breaker sees through the generic (the phantom
embeds nothing), keeping the `Identifier.assigner` box where it was.
Breaking for code that names reference field types; `cast`/`into_any` are
the escape hatches, and `examples/typed_references.rs` is the tour.

### Changed - **breaking**: `contained` is typed (T47)

Every DomainResource's `contained` field is now
`Vec<crate::<release>::resources::Resource>` instead of
`Vec<serde_json::Value>` — the one polymorphic slot narrow enough to type,
since it holds the release's own resources and nothing else.
`Bundle.entry.resource` and the other `Resource`-typed slots deliberately
stay raw JSON (`bundle_util` and `Parameters` dispatch on them). The wire
form is unchanged — the corpus gate is the proof — and two things improve:
contained resources are now **validated** when their container is (the
`Validate` derive recurses the typed field), and the `dom-2`/`dom-4` checks
read typed accessors (`Resource::has_contained`, `Resource::meta`, both
generated) instead of probing raw JSON. Breaking for any code that built or
matched `contained` as `Value`; construct the release's `Resource` enum
instead.

## [fhir-derive-macros 1.2.0] - 2026-08-06 (retroactive entry)

Published without a changelog entry — recorded here after the 2026-08-06
audit found the gap (T41). 1.1.0 → 1.2.0 carried the derive-macro side of
the 3.0.0 model work (the `Validate`/`FhirChoice`/`Builder` derives as the
release crates now consume them); the publish-match gate
(`scripts/check-published-match.sh`) is what caught the 206-line divergence
that forced the bump (F-35, in the database family's register).

## [3.0.0] - 2026-08-01

`fhir`, `fhir-core` and `fhir-r2` … `fhir-r6` all move to 3.0.0
together. The whole family takes the major bump because the breaking change is
in a type every one of them re-exports, so a version that merely *looked*
unaffected would still break its dependents. `fhir-derive-macros` moved to
1.2.0 (entry above — this paragraph originally said "untouched at 1.1.0"),
and the reservation crates sit at 0.0.1 (originally written as 0.0.0).

### Added — cross-release conversion with a loss report (`fhir::convert`)

Moving a resource between releases had one documented answer: serialize to JSON
and let serde tell you what the target refuses. That answer has a hole in it.
Serde reports the first error and stops, so a document with three problems
reveals one — and the commonest difference between releases is an element the
target does not have, which is not an error at all. Unknown keys are ignored, so
the field disappears and nothing is said. The mechanism meant to make conversion
safe was quietest exactly where the data loss was.

`fhir::convert::between::<R4, R5>(&json)` converts the wire form and returns a
`LossReport` naming every difference it acted on: elements the target lacks,
`value[x]` variants it does not admit, repeating elements it does not repeat,
JSON kinds that changed, elements it requires that are absent, and required
bindings that moved to another value set. The report distinguishes losses that
discarded data from warnings about data that was kept.

It is driven by both releases' generated `ElementMeta` tables, not by
hand-written per-resource rules, so every pair of modelled releases converts in
both directions and a newly added release is convertible by existing. It is a
*structural* conversion: it does not know that an element was renamed between
releases, and reports such an element as removed rather than guessing at a
mapping — a wrong mapping is worse than an absent one, because it is invisible
in the result. Specified in [spec 14](spec/14-cross-release-conversion.md).

There is still no `From`/`Into` between releases (spec 12, R12.4) and there will
not be. The report is the point, and a conversion the compiler performs on the
caller's behalf cannot deliver one.

`fhir::convert::strict::<S, T>(&json)` is the same conversion for callers who
would rather refuse a document than forward a lossy version of it: it yields the
document only when nothing was changed, and the whole report otherwise. It
rejects on *any* entry rather than only on discarded data, because a
`RequiredMissing` means the result will not validate in the target and a
`BindingChanged` means a code that was legal may not be. Measured against the
committed corpora that is not over-strict — entries which warn without
discarding account for one document per release pair, against roughly half of
each corpus that converts cleanly.

### Added — `ele-1` is enforced, and did not need FHIRPath after all

`ele-1` — "all FHIR elements must have a `@value` or children" — is the single
most restated constraint in FHIR: **8,363 of R5's 10,992 constraint
occurrences**. Spec 10 recorded that enforcing it "needs FHIRPath, and is
deferred". That was wrong, and worth correcting rather than quietly fixing.

The expression is `hasValue() or (children().count() > id.count())`. It names
only `children()` and `id`, and against a statically typed model "children other
than `id`" is just "fields other than `id`" — something the derive macro can
see. The primitives carry their value in a newtype and always satisfy the first
clause, so only the complex types can be empty at all. What actually needs an
evaluator is *traversal*: `dom-3` resolving references, `csd-1` walking
`descendants()`, the rules projecting a collection and testing `isDistinct()`.
Traversal is the dividing line, not FHIRPath syntax.

Two edges the specification is precise about and a first attempt got wrong:

- **A resource root is not an `Element`.** `Resource` descends from `Base`, and
  the definitions bear it out — of the 71 root elements carrying `ele-1` in R5,
  every one is a datatype. An empty `Patient` is unhelpful but is not an
  `ele-1` violation. Resources are identified by `implicitRules`, which comes
  from the `Resource` base and appears in no datatype or backbone. Two doctests
  caught this by failing; they were right and the check was wrong, so the check
  changed and they did not.
- **`id` alone is not a child.** The expression is a strict inequality against
  `id.count()`, so an element whose only content is its `id` fails.

### Added — four more enforced invariants

`att-1`, `qty-3`, `drq-1` and `inv-1` join `ext-1`, `dom-2` and `dom-4` in
`invariant_stmts`, so they are checked in every release at once. Each is a
presence, absence or exclusive-choice test that needs no FHIRPath evaluator:

- `att-1` — an `Attachment` with data must say what type it is. Bytes with no
  media type cannot be interpreted by a receiver.
- `qty-3` — a unit code means nothing without the system that defines it.
  Applies to `Quantity` and its six specializations.
- `drq-1` — a `DataRequirement` filter needs exactly one of `path` and
  `searchParam`, so both and neither are equally wrong.
- `inv-1` — a `Parameters.parameter` carries parts, or exactly one of a value
  and a resource, never a mixture.

Two rules came out of getting this wrong first. A check must be gated on the
fields existing *with the shape it expects*, not on the struct's name: DSTU2
models `Age` as an empty struct beside an `AgeQuantity` holding the fields, and
R3 types `DataRequirement.codeFilter.path` as `1..1` where R4 has `0..1`, so a
name-keyed check does not merely fail to apply — it fails to compile that
release. And a check must not be inferred from field names either, because
`Coding` carries `code` and `system` exactly as `Quantity` does; a test asserts
`qty-3` stays off it.

The unit tests prove the checks fire when a rule is broken. A new corpus test,
`tests/invariants_corpus.rs`, proves the converse — that they stay silent on
all 53 committed official R5 examples — which is the half a too-eager check
gets wrong.

### Fixed — the release crates' unit tests never ran in CI either

`release-crate-doctests` ran `cargo test -p fhir-rN --doc`, and `--doc`
runs doctests and nothing else, so between 276 and 500 unit tests per release
crate were never compiled in CI. The job is now `release-crate-tests` and runs
`--lib` as well.

This was not hypothetical: adding the four invariants above broke the R2 and R3
builds outright, and the default gate stayed green throughout, because neither
release is in the default feature set and nothing compiled their test targets.

### Fixed — `fhir-core`'s unit tests never ran in CI

A workspace with a root package does not descend into its members, so the
`test` job's `cargo test` covered the `fhir` package alone and `fhir-core`'s
own unit tests — 42 of them, including the whole conversion engine — were never
compiled there. They were shipped broken exactly once, in this release: when
`ElementMeta` gained `content_reference`, four `#[cfg(test)]` literals in
`meta.rs` and `summary.rs` were missed, and `cargo build --all-targets`,
`cargo test` and `cargo clippy --all-targets` were all green while
`cargo test -p fhir-core --lib` did not compile.

The `test` job now runs `cargo test -p fhir-core --lib` under both the default
and `xml client` feature sets. This is the same hole `release-crate-doctests`
was added to close, in the one package that job does not reach.

### Fixed — the element table could not describe recursive backbones

`ElementMeta` gained `content_reference`, and the generator now emits it. FHIR
expresses recursion with `contentReference`: `Questionnaire.item.item` does not
restate an item's elements, it points at `Questionnaire.item`. Without that
field the table simply has no children under the deeper path.

Inferring the target from the path does not work, which is worth recording
because it looks like it should. Matching on the final segment resolves
`Questionnaire.item.item` correctly, sends `TestScript.test.action.operation` to
the wrong subtree — it refers to `TestScript.setup.action.operation`, a
*sibling* — and sends `QuestionnaireResponse.item.item` to `Claim.item`, with
which it shares nothing but a word. The conversion layer's identity test caught
all three.

### Fixed — a stale `target_profiles` in the R5 element table

`fhir-r5/src/meta/generated.rs` had drifted from the generator: 63
elements whose type carries a profile (`SimpleQuantity`, and one
`OperationOutcome`) had an empty `target_profiles` where the generator produces
the profile URL. R5's table is generated into a hand-documented crate and had
not been regenerated since the generator learned to record those. Regenerating
changed nothing else in any release.

### Changed — `Release` (breaking)

- `Release::elements()` is new and required: it returns the release's element
  table, which is what lets conversion work on a release named only as a type
  parameter.
- `Release::Resource` now also requires `Serialize`. It is the only type in a
  release that carries its own `resourceType` — the resource structs do not, the
  enum's `#[serde(tag)]` supplies it — so it is what typed conversion has to
  take.

Both are breaking changes to a published trait, as is the new public field on
`ElementMeta`. Only the release crates in this workspace implement `Release`.

`ElementMeta` deliberately does **not** gain `#[non_exhaustive]`, which is the
obvious lesson to draw from `XmlError` in 2.2.0 and is the wrong one here.
`#[non_exhaustive]` forbids struct-literal construction outside the defining
crate, and constructing `ElementMeta` literals outside `fhir-core` is exactly
what the generated `meta/generated.rs` in every release crate does. Adding it
would break the workspace's own code to spare a hypothetical external one. The
same applies to `TypeRef` and `BindingMeta`. A future field on any of the three
is therefore a major bump again, and that is the accepted cost of a table the
release crates build themselves. The new types that are *not* built outside
`fhir-core` — `LossKind`, `Loss`, `Converted` — are `#[non_exhaustive]`.

## fhir-core 2.2.0

### Fixed — a remote denial of service in the XML reader

`read_children` and `read_element` recursed once per nested element with no
bound, so a document supplied its own recursion depth. At roughly 2,700
levels — about 160 KB of XML, well under any sane request-size limit — this
overflowed the stack and aborted the process. A stack overflow is not
unwindable: `catch_unwind` does not see it, a worker thread cannot contain
it, and the process dies. One request was enough.

`xml::MAX_DEPTH` bounds it at 128, matching the limit `serde_json` already
applied to the JSON path so both entry points refuse the same documents.
Real FHIR nests around 15 deep at the very most.

### Changed

- `XmlError` gains a `TooDeep { limit }` variant and is now
  `#[non_exhaustive]`. Adding a variant to a public enum is strictly a
  breaking change, so this is a minor release only because `fhir-core` 2.1.0
  had no reverse dependencies and 14 downloads when the fix landed, two days
  after publication. `#[non_exhaustive]` means the next reader failure mode
  will not force the same decision again.
- Dependents now require `fhir-core >= 2.2.0` rather than `>= 2.1.0`, so a
  resolver cannot select the version without the fix.

## [2.1.0] - 2026-07-27

### Changed — the crate is now a workspace, and the public API is not

Each FHIR release moved into its own crate — `fhir-core`, `fhir-r3`,
`fhir-r4`, `fhir-r5` — with `fhir` re-exporting them behind the same
features. Every public path is unchanged: `fhir::r5::types::Patient`,
`fhir::validate::Validate`, `fhir::r5::parse`, and
`default-features = false, features = ["r4"]` all resolve exactly as before,
which is why this is a minor release.

The reason is memory. The three models were modules of one crate, so
enabling them together was a single `rustc` process over ~554k lines of
generated code. Measured peak RSS for `cargo build --all-targets --features
"r3 r4"`:

    before   12.9 GB
    after     5.0 GB

A standard CI runner has 7 GB, so the combined build had been dying with
SIGTERM partway through. Peak is now the largest crate rather than the sum
of all three, and the all-releases job is back.

Consumers of one release also stop paying for the others: `fhir-r4` no
longer ships R3 and R5, and `fhir-r5` sheds a further 37k lines of
work-in-progress generator machinery that never belonged in a data model.

### Added

- `fhir-r6`, generated from the real 6.0.0-ballot3 specification: 161
  resources, 459 code enums, 9,713 elements. Behind the `r6` feature, off by
  default, unpublished, and outside the semver promise — it tracks a ballot
  draft that can still change.
- `fhir-r7`, `fhir-r8`, `fhir-r9` as name reservations at `0.0.0`. No such
  specifications exist; the crates contain no model, deliberately.
- `fhir-r2` (DSTU2, 1.0.2): 94 resources, 28 datatypes, 18 primitives, 265
  code enums.

  A `fhir-1` (DSTU1, 0.0.82) crate was built and then withdrawn before
  release. It is recorded here because a reader may have seen it in the
  repository: DSTU1 predates most of what the other releases share — no
  `Bundle`, no `isSummary`, its own resource names — and carrying a trial
  model of a 2012 draft was not worth the maintenance. Nothing was published,
  so no version was orphaned.
- Round-trip tests for DSTU1 and DSTU2 over the real resources embedded in
  their committed definition bundles — 255 and 1,572 respectively. Neither
  release publishes a separate example corpus, so these run with no fetch
  step.
- `doc/adding-a-release.md`, the procedure, written from adding R6.
- `ValidationIssue::new` and `is_valid_code`/`is_valid_id`/`is_valid_uri_like`
  are now public. They were `pub(crate)`, which generated release code could
  reach when it was the same crate and cannot across a crate boundary.

### Fixed

- The generator emitted `crate::decimal::Decimal`, which resolves only in
  the pre-split layout, and `Version::source_dir` still pointed at
  `src/<release>`. Either would have silently undone the split on the next
  regeneration.
- **DSTU2 silently dropped every recursive element.** R3 renamed
  `nameReference` to `contentReference`, and only the modern spelling was
  understood, so 92 DSTU2 elements had no type and were omitted from the
  model — among them `Bundle.entry.link`,
  `ValueSet.codeSystem.concept.concept` and
  `ValueSet.expansion.contains.contains`. Nothing errored; parsing a DSTU2
  bundle just discarded its entry links and nested concepts. `fhir-r2` gains
  exactly 92 fields. Found by round-tripping published data, which is the
  only kind of test that can: a test written in Rust can only name fields
  that exist.
- Primitive synthesis treated any lowercase type code as a primitive name.
  R4 onwards type `Element.id` as `http://hl7.org/fhirpath/System.String`,
  which also starts lowercase, so every modern release carried a junk
  metadata element named after a URL.
- `fhir-derive-macros` is **1.1.0**. The published 1.0.1 knew only releases
  r3/r4/r5 while this tree's 1.0.1 knew r1–r6, and a crates.io version is
  immutable — so packaging `fhir-1` failed with 225 copies of
  `unknown FHIR version "r1"` even though every local build was green. CI now
  compares each already-published version against this tree and fails if they
  disagree.
- The `r2` and `r3` re-exports had lost their doc comments to a misplaced
  insertion, which stacked three of them onto `r1`.

## [2.0.0] - 2026-07-26

### Fixed — two denial-of-service advisories in the XML reader

`quick-xml` moves from 0.36 to 0.41, clearing RUSTSEC-2026-0194 (checking a
start tag for duplicate attribute names was quadratic in the number of
attributes, with no bound on that number) and RUSTSEC-2026-0195 (`NsReader`
allocated a namespace binding per `xmlns` declaration before returning the
event, with no upper bound). XML is a first-class FHIR wire format, so a
parser that degrades on attacker-shaped input is an availability problem for
anything accepting outside documents, not a theoretical one.

Both were found by `cargo deny check --all-features`. Plain `cargo deny
check` does not see an optional feature's dependency tree, and `quick-xml`
is behind the `xml` feature — so the shorter command reported a clean bill
of health on a crate with two live advisories.

### Fixed — decimal precision, which FHIR treats as clinical data

`decimal` silently lost the precision it was given. In a default build,
`0.50` serialized back as `0.5`, `1.000` as `1.0`, and
`12345678901234567890.5` as `1.2345678901234567e+19`. FHIR states that a
decimal's precision has significance — `0.50` mmol/L asserts two significant
figures where `0.5` asserts one, and a dose of `1.000` mg is not the same
claim as `1.0` mg — so this changed what stored results and doses said.

The `precise-decimal` feature repaired it, but by enabling a *global*
`serde_json` feature, which meant whether a build was correct depended on
what unrelated crates elsewhere in the dependency graph happened to enable.

`serde_json/arbitrary_precision` is now a **non-optional** dependency
feature. Cargo features are additive and a dependent cannot switch one off,
so precision is a guarantee rather than a default that can be lost.
`precise-decimal` remains as a deprecated no-op for one release.

The round-trip test suite could not have caught this: it compares
`serde_json::Value`s, where `0.50 == 0.5`. It can now, because `Value`
numbers compare by lexeme — and `src/decimal.rs` pins that property directly
so the corpus suite cannot go blind again (spec R13.3).

### Changed
- `Decimal` is now a hand-written type at the crate root
  (`fhir::decimal::Decimal`), shared by every release rather than generated
  three times, and re-exported as before at `fhir::{r3,r4,r5}::types::Decimal`.
  It presents a lexical API — `Decimal::new("0.50")`, `as_str`, `as_f64`,
  `as_number` — with **lexical equality** (`Decimal("1.0") != Decimal("1.00")`,
  because they assert different precision) and **numeric ordering** (they
  compare `Equal`, because they denote the same quantity).
  *Breaking:* the tuple field is private; construct with `Decimal::new` or
  `Decimal::from_json_number`.
- A `serde_json::value::RawValue`-based representation was tried first and
  rejected: it preserves precision on `from_str` but fails through
  `#[serde(flatten)]`, which every `value[x]` choice element uses — serde's
  `Content` buffering has no representation for a raw token, so the entire
  choice variant was *silently dropped*. Losing `Observation.valueQuantity`
  is worse than rounding it. Recorded in `spec/02-primitive-types.md` R2.2.

### Fixed — the generator and `cargo fmt` no longer fight

Generated code was not rustfmt-canonical, in two ways: an empty
`StructureDefinition.version` left `//! Version: ` with a trailing space
(~280 files), and emitted lines past the width limit were rewrapped by
rustfmt and unwrapped again by the next generation (~16 files per release).
Running `cargo fmt` and running the generator therefore undid each other
indefinitely, which made `cargo fmt --check` a gate that could only stay
green until someone regenerated — worse than no gate, on a crate that is
mostly generated.

The generator now omits the `Version:` line entirely when the specification
states none, and pipes its output through `rustfmt` as it writes, so
generated code is already canonical. Regenerating R3 and R4 now produces
zero rustfmt disagreements. A missing `rustfmt` is not fatal: generation
still succeeds unformatted and CI reports it.

This release also brings the committed tree into line with that output — a
one-time whitespace and line-wrapping change across the generated modules,
with no semantic content.

### Added — the REST client stops assuming the happy path

The client implemented the interactions and none of the adversity. It now
handles the network it is actually pointed at (spec R13.5–R13.10):

- **Timeouts by default.** `Client::new` sets a 30 s request and 10 s connect
  timeout; `reqwest::Client::new()` has neither, so the old client waited
  forever on a server that accepted the connection and then went quiet.
  A caller-supplied `reqwest::Client` still owns its own policy.
- **URLs are built, not interpolated.** Path segments are percent-encoded, so
  an id of `../Patient/other` addresses `Patient/..%2FPatient%2Fother`
  instead of a different resource.
- **Concurrency-safe writes.** `read_with_etag`, `update_if_match`,
  `delete_if_match`, and `create_conditional` (`If-None-Exist`). Without an
  ETag every update was last-write-wins, and a lost create response meant a
  duplicate patient.
- **Bounded retry** (`with_retry`) for `GET`/`PUT`/`DELETE` only — retrying a
  FHIR `POST` is how a patient is entered twice.
- **Paging** via `next_page` / `search_all`, the latter bounded so a server
  with non-terminating paging costs a known number of requests.
- **A bearer-token supplier** (`with_bearer_token`), called per request so a
  token can be refreshed without rebuilding the client.
- **A response body cap** (64 MiB default, `with_max_body`), enforced while
  streaming rather than after allocating.
- **`Debug` no longer prints error bodies.** A server that fails to produce
  an `OperationOutcome` may echo the resource instead, and `Debug` output
  ends up in logs and panic messages.

`Release` gains `next_link`, the one thing paging needs from inside a
release's `Bundle`. R5 binds `Bundle.link.relation` to a value set where R3
and R4 use a plain string, so the implementations differ per release — the
kind of divergence this crate exists to make visible.

### Fixed — `ConceptMap...target.product` silently dropped its contents

R5 typed `ConceptMap.group.element.target.product` as a bare `types::Element`,
which carries only `id` and `extension`. FHIR defines the element by
`contentReference` to `dependsOn`, so it has that backbone's shape —
`attribute` plus a `value[x]`. Every one of those fields was therefore
**dropped on deserialize, with no error**, in a terminology mapping's
statement of what else the mapping produces.

R3 and R4 were correct throughout: they are generated, and the generator
resolves `contentReference` properly. R5 is hand-documented and never
regenerated, so it silently predates that fix — the default-enabled release
was the stale one. `tasks.md` T35 records the wider drift this exposed.

The defect had been noted in `tasks-roundtrip-failures.md` against the wrong
filename, so it read as handled while it was live. Only running the whole
corpus contradicted the document.

### Fixed — code enums that could not represent their own binding

A `required` binding names a **ValueSet**, but the generator built enums from
**CodeSystem**s and matched them by URL segment. That agrees for most
bindings, and silently fails when a value set *composes* several systems —
the generator then saw only the like-named one:

| enum | was | now | bound to |
| --- | ---: | ---: | --- |
| `EventTiming` | 13 | **27** | `Timing.repeat.when` |
| `TaskIntent` | 1 | 9 | `Task.intent` |
| `TransportIntent` | 1 | 9 | `Transport.intent` |
| `DetectedissueStatus` | 1 | 4 | `DetectedIssue.status` |
| `TimingAbbreviation` | 1 | 17 | — (preferred binding) |

`EventTiming` is the consequential one. Partially populated is worse than
empty: 14 of its 27 codes deserialized to `Coded::Unknown` while the rest
came back `Known`, on the element that says *when a dose is taken* — `AC`
(before meals), `PC` (after meals), `HS` (at bedtime) and the meal-specific
variants. Preserved on the wire; absent from the type.

Nothing in the example corpus could reveal this, because `Coded::Unknown`
round-trips perfectly. `tests/coded_bindings.rs` asserts instead that a
genuine code from each bound value set resolves to `Known(_)`.

An `include` that references another value set is still not followed — the
resolver returns nothing rather than answering partially — but no R5 enum
currently needs that path.

Eighteen enums remain single-variant, and that is correct: the specification
defines exactly one concept for each (FHIR ships them as placeholder
terminologies), and all thirteen bindings to them are `example` strength, so
none would be typed as `Coded<E>` in any case.

### Changed — `src/r5/codes.rs` regenerated (**breaking**)

R5's code enums were 23 behind the generator (419 -> 442), which is why 13
fields with `required` bindings were typed as a plain `types::Code`: the bound
enum did not exist. Regenerating adds the missing enums and converts **10** of
those fields to `Coded<E>`.

Two breaking changes come with it, listed because a downstream build failure
is the wrong way to learn about them:

- **12 enums are renamed**, not added — the generator's naming policy moved
  and the committed tree predated it: `FHIRVersion` -> `FhirVersion`,
  `Imagingselection2dgraphictype` -> `Imagingselection2Dgraphictype`,
  `Imagingselection3dgraphictype` -> `Imagingselection3Dgraphictype`,
  `Icd10Procedures` -> `ExIcd10Procedures`, `CdshooksIndicator` ->
  `Indicator`, `NhinPurposeofuse` -> `Purposeofuse`,
  `ReasonMedicationNotGivenCodes` -> `ReasonMedicationNotGiven`, plus
  `AstmSignatureType`, `EtsiSignatureType`, `ListItemFlag`, `Tldc`,
  `W3cProvenanceActivityType`. None was referenced inside the model, so
  nothing broke internally — but they are public exports.
- **`HttpVerb` variants are renamed**: `POST` -> `Post`, `PUT` -> `Put`,
  `DELETE` -> `Delete`. This one broke `bundle_util.rs`, which is how it was
  found — a reminder that enums nobody uses internally can change shape with
  nothing to catch it.

**Three conversions were deliberately not made.** `Task.intent`,
`Transport.intent` and `DetectedIssue.status` stay `types::Code`, because
`TaskIntent`, `TransportIntent` and `DetectedissueStatus` are generated with a
single `Unknown` variant — the generator cannot resolve those value sets. A
`Coded<E>` over such an enum turns every real value into `Unknown("order")`
while the signature claims type safety, which is worse than the `Code` it
replaced. `tests/coded_bindings.rs` asserts a genuine code from each bound
value set deserializes to `Known(_)`; it rejected these three on its first
run. `tasks.md` T36 tracks the generator fix.

### Fixed — four more R5 modelling defects (T35 drift audit)

Auditing the hand-maintained R5 tree against the generator, prompted by the
`product` bug above, found four more. Each is spec-verified and pinned by a
regression test:

- **`Ratio.denominator`** was required. The specification says `0..1`, so a
  valid `Ratio` carrying only a numerator failed to deserialize — a harder
  failure than dropping data, since nothing parses at all. Three existing
  tests changed with it: they asserted a defaulted denominator, which is to
  say they encoded the defect.
- **`Address.use`, `HumanName.use`, `ContactPoint.use`** had no `_use`
  sibling, so primitive extensions on them were silently dropped.
- **`ProductShelfLife.modifierExtension`** was absent entirely. A modifier
  extension changes the meaning of the element carrying it, so a consumer
  that cannot understand one must refuse the resource; dropping it silently
  produces a resource that reads as understood and no longer means what it
  said.

- **`PackagedProductDefinition.characteristic`** was `Vec<serde_json::Value>`
  — an untyped escape hatch where FHIR defines a `contentReference` to
  `packaging.property`. The data round-tripped fine; what was lost was
  structure, validation, and compile-time help, which is why no corpus run
  could ever have flagged it.

Two of these (`product`, `characteristic`) share one root cause: a
`contentReference` the hand-maintained tree never resolved. All 78 R5
`contentReference` elements have now been checked, and
`tests/content_reference.rs` keeps the class closed.

None of these could be caught by the example corpus, because no published
example exercises them. The corpus proves the model handles the data that
exists; it says nothing about data the specification permits.

`tasks.md` T35 records what remains untriaged — chiefly ~11 fields where R5
has a plain `Code` and the generator emits a bound `Coded<E>` (type safety,
not data loss), and two `ImagingSelection` elements absent altogether.

### Added — the corpus is now a gate, not a claim

`roundtrip_full_official_examples` was `#[ignore]`d with an uncommitted
corpus, so CI exercised a curated subset *chosen to pass*. All three releases
now gate on every published example — 7,400 of them — in CI.

- R5 2823/2824, R4 2713/2912, R3 1490/1664 on first measurement.
- The R4 and R3 shortfalls are **three** understood classes, not 372 bugs:
  HL7's generated examples omit `Questionnaire.item.linkId` (1..1),
  `SearchParameter.base` (1..*), and `SearchParameter.code` (1..1). The
  specification is explicit that all three are required, so the model is
  right and the examples are not. They are recorded as counted
  `KnownFailureClass` entries — deliberately *not* fixed by relaxing the
  model, which would have made the suite green by making a clinical data
  model accept resources missing required elements.
- The gate fails on a new mismatch, on an allowlist entry that starts
  passing, and on any change to a class count in either direction. An
  allowlist that only grows is a slow way of switching a test suite off.

### Added
- `spec/13-assurance.md`: what must be true before the crate is depended on
  for clinical work — proving fidelity rather than sampling it (the full
  official corpus is still `#[ignore]` in CI), client robustness (no default
  timeout, unencoded path interpolation, no `If-Match`), honest description
  of what validation covers, and release evidence (advisories, SBOM).

### Added
- **FHIR R3 (3.0.2, STU3) support** under `fhir::r3`, behind a new `r3` cargo
  feature: 18 primitive datatypes, 36 complex datatypes, 117 resources, and 386
  code-system enums, with the same choice enums, `Coded<E>`, `Vec1`,
  primitive-extension siblings, builders, prelude, extension helpers, `Bundle`
  utilities, summary serialization and `client`/`xml` support as the other
  releases. Generated by `cargo run -- r3`.
- The generator now normalizes the ways older releases spell the same fact, at
  the input boundary rather than downstream (`spec/12-fhir-releases.md`,
  R12.17): `targetProfile` as a string or a list; a binding's value set as
  `valueSet`, `valueSetReference` or `valueSetUri`; a `type` entry with no
  `code`; and infrastructure elements (`<Type>.id`, `Extension.url`) that R3
  does not mark with a FHIRPath system type.
- Example `r3_patient`.
- **FHIR R4 (4.0.1) support** under `fhir::r4`, behind a new `r4` cargo
  feature. A complete, independent model: 20 primitive datatypes, 43 complex
  datatypes, 146 resources, 486 code-system enums, `value[x]` choice enums,
  `Coded<E>` for required bindings, `Vec1` for `1..*`, primitive-extension
  siblings, builders, prelude, extension helpers, `Bundle` utilities, summary
  serialization, and the `client`/`xml` features. `fhir::r4` mirrors
  `fhir::r5` module for module, so porting code between releases is a matter of
  changing one path segment.
- `fhir::codegen` — a release-parameterized code generator that emits a whole
  release's finished module tree in one pass. Run it with `cargo run -- r4`.
  Everything that varies by release is reachable from `codegen::Version`.
- `fhir::release::Release` — a FHIR release as a type, implemented by
  `fhir::r4::R4` and `fhir::r5::R5`, for code generic over a release.
- Release-independent modules at the crate root, re-exported by every release
  module: `validate`, `coded`, `builder`, `meta`, `temporal`, `summary`, and
  `xml`. `fhir::r4::validate::Validate` and `fhir::r5::validate::Validate` are
  the *same* trait, so one `#[derive(Validate)]` and one generic bound serve
  both releases.
- `#[fhir_version("r4")]` in `fhir-derive-macros`, naming the release for the
  few paths that are release-specific. Defaults to `r5`; an unknown release is
  a compile error.
- Examples `tutorial`, `r4_patient`, and `r4_and_r5_side_by_side`. The
  `tutorial` example is the book's new end-to-end chapter, compiled and run by
  the test suite so the guide cannot drift from the crate.
- `spec/12-fhir-releases.md` and a "FHIR releases" chapter in the book, defining
  how releases coexist and why they are separate types.
- A "Tutorial: a patient record end to end" chapter in the book.
- The official R4 definition JSON under
  `doc/fhir-specifications/r4/fhir-definitions-json/`, so R4 generation is
  reproducible from a clean clone.

### Fixed
- A definition that fails to parse now stops generation instead of being
  skipped. Skipping one silently removed a whole resource from the generated
  model; this surfaced immediately when R3's differently-shaped JSON was first
  read.
- `Bundle::transaction()`/`batch()` entries now carry their `resourceType`. A
  bare resource struct does not serialize one — the polymorphic `Resource` enum
  is what adds the discriminator — so a built transaction bundle was not valid
  FHIR, and reading it back with `iter_resources()` or `resources::<T>()`
  silently yielded nothing. Affects R5 as well as R4.
- `Coding.display` and `CodeableConcept.text` are `types::String` rather than a
  bare `std::string::String`, matching every other primitive field and the
  convention in spec 03 (R3.3). The JSON is unchanged; only the Rust type
  differs. **Breaking** for code that sets those two fields.
- `Reference::_marker` is public (and `#[doc(hidden)]`), so `Reference` can be
  built with the `Type { field: …, ..Default::default() }` idiom the rest of the
  model documents. Previously the private phantom field made that a compile
  error from outside the crate.

### Changed
- **`r5` is now a cargo feature**, on by default. Existing dependants are
  unaffected; `default-features = false` now means no release model, so pick one
  explicitly.
- `bin/fetch-examples` accepts `r3` (which downloads the STU3 archive; note it
  contains only definitional resources, not the clinical examples).
- `fhir::client::Client` is a type alias for `ReleaseClient<R5>`, which is
  generic over a `Release`. Existing uses of `Client` and `ClientError` continue
  to work unchanged; `fhir::r4::client::Client` is the R4 counterpart.
- `fhir::prelude` is the R5 prelude and now re-exports `fhir::r5::prelude`,
  which is its new canonical home. `fhir::r4::prelude` is the R4 counterpart.
- `bin/fetch-examples` takes a release argument (`bin/fetch-examples r4`).
- The round-trip tests are per release —
  `tests/roundtrip_r4_examples.rs` and `tests/roundtrip_r5_examples.rs`, sharing
  `tests/common/` — and the curated example subsets moved to
  `tests/data/roundtrip_examples_<release>/`.

### Notes
- R4 is verified against the 2911 official R4 example resources: 2713
  round-trip exactly, 0 mismatch. The 198 that fail all omit an element the R4
  specification makes mandatory (188 auto-generated questionnaires missing
  `Questionnaire.item.linkId`, 10 SearchParameters missing
  `SearchParameter.base`), so the model rejects them correctly.
- R3 is verified against 1693 official R3 resources (the STU3 definitional
  archive plus the clinical examples, which STU3 publishes separately): 1519
  round-trip exactly, 0 mismatch. The 174 that fail are the same class of
  non-conformant data — 144 questionnaires without `linkId`, 12
  SearchParameters without `base`, and 18 of R3's own primitive
  `StructureDefinition`s that omit `ElementDefinition.type.code`, which R3
  itself makes `1..1`.

## [1.2.1] - 2026-07-25

### Fixed
- **`--features r4` and `--features r3` did not compile for anyone using the
  published crate.** Every `Validate` derive in `src/r3` and `src/r4` expanded
  to `crate::r5::…` paths, so an R4-only build failed with thousands of
  errors — `cannot find r5 in crate`, the module being feature-gated off.

  The repository never showed this: it resolves `fhir-derive-macros` through
  its `path` dependency, where the release-aware macro (reading
  `#[fhir_version("r4")]`) has existed for some time. Only crates.io users got
  the stale published macro. A build that passes locally and fails for every
  downstream consumer is worth naming as its own failure mode — the fix was
  written, tested, and simply never shipped.

  Requires `fhir-derive-macros` 1.0.1.

## [1.0.0] - 2026-07-12

First stable release. No API changes from 0.4.0 — this promotes the crate to
1.0.0 (and `fhir-derive-macros` to 1.0.0) to commit to semantic-versioning
stability for the R5 model, choice enums, `Coded<E>`, cardinality mapping
(`0..*`→`Vec<T>`, `1..*`→`vec1::Vec1<T>`), builders, prelude, and the
`client`/`xml`/`precise-decimal` features.

## [0.4.0] - 2026-07-12

Also bumps `fhir-derive-macros` to 0.2.0 (new `FhirChoice` and `Builder`
derives, plus cardinality/invariant checks in `Validate`).

### Added
- FHIR XML (T21, feature `xml`): `fhir::r5::xml::to_xml`/`from_xml` convert a
  resource to/from FHIR XML via a metadata-driven serde_json::Value bridge
  (primitives as `value` attributes, `id`/extension-`url` attributes, arrays by
  cardinality, `value[x]` choices). Scope note: `_field` extensions, XHTML
  `div`, and contained resources round-trip through JSON but are not yet
  canonical FHIR XML.
- Documentation guide (T23): an mdBook in `book/` (getting started, model
  mapping, JSON serialization, validation, terminology, extensions, bundles,
  generator internals); CI builds it, and README links to it.
- REST client (T19, feature `client`): async `fhir::client::Client` (reqwest +
  tokio) with read/vread/create/update/delete/search/capabilities; error
  responses surface the server `OperationOutcome`. wiremock unit tests;
  `examples/client_crud.rs` runs against the public HAPI R5 server.
- Summary serialization (T20): `fhir::r5::summary::to_summary_value` returns the
  FHIR `_summary=true` view (isSummary + mandatory top-level elements) using the
  meta table; doctest on Patient.
- Bundle utilities (T18): `Bundle::iter_resources`, `Bundle::resources::<T>(rt)`,
  `Bundle::next_link` paging, and a `Bundle::transaction()`/`batch()` builder
  (`create`/`update`/`delete`). New `examples/transaction_bundle.rs`. (Typed
  `contained` — `Vec<Resource>` — is deferred: it is invasive and would change
  the JSON-based `dom-2`/`dom-4` checks.)
- Builders (T16): `#[derive(Builder)]` generates `Type::builder()` → a chainable
  `TypeBuilder` with a setter per field and `build() -> Result<Type,
  BuilderError>` that enforces required (`1..1`) fields. Applied to the 10
  most-used resources and all general-purpose datatypes (59 types).
  `examples/build_patient.rs` rewritten with the builder.
- Ergonomics (T17): a `fhir::prelude` (`use fhir::prelude::*;`) and the
  `ExtensionExt`/`ModifierExtensionExt` traits (`extension(url)`,
  `extensions(url)`, `set_extension`, `add_extension`) on every resource and
  datatype that carries extensions. New `examples/extensions.rs`.
- Common invariants (T14): `Validate` enforces `ext-1` (an extension has a value
  xor nested extensions) and `dom-2`/`dom-4` (rules on contained resources). A
  generated coverage report of all 314 constraint keys is committed at
  `spec/10-invariants-coverage.md`.
- `OperationOutcome` bridge (T15): `From<Vec<ValidationIssue>>` +
  `examples/operation_outcome.rs`.
- Deeper validation (T13): `Validate` now reports empty `1..*` elements (read
  from `meta` at runtime, since bare `Vec` is also used for some `0..*`) and
  required-binding codes outside the value set (a `Coded::Unknown`). See
  `spec/07-validation.md`.
- Typed references (T11): `Reference` is now `Reference<T = Any>`, a phantom-typed
  newtype over the same wire form (`Reference<Any>` = the old untyped reference,
  so existing code is unaffected). Adds the `ResourceType` marker trait, the
  `Any` target, `.cast()`/`.into_any()`, and `.resolve(&bundle)` to look a
  reference up in a `Bundle`. (Machinery only; typing individual reference fields
  from `targetProfile` is a follow-up rollout.)
- Primitive value APIs (T12): `fhir::r5::temporal` — precision-aware parsing for
  the date/time primitives (`Date::parse_parts`/`DateTime::date_parts`/
  `Instant::date_parts`/`Time::parse_parts`), a `DateParts` type with FHIR
  precision ordering (`"2024"` vs `"2024-03"` is indeterminate), and `TimeParts`.
  Storage is unchanged (still `String`).
- `precise-decimal` feature: back `serde_json::Number` with arbitrary precision
  to preserve exact `decimal` values.

### Changed (breaking)
- Cardinality now maps to the Rust type more precisely:
  - **`1..*` elements are `vec1::Vec1<T>`** (non-empty), so the constraint is
    enforced at compile time. 59 fields across 58 structs; those structs no
    longer derive `Default` (there is no empty value).
  - **`0..*` elements are bare `Vec<T>`** (empty when absent) instead of
    `Option<Vec<T>>`, with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`.
    3815 fields. Construct with `vec![…]` (not `Some(vec![…])`); read as a slice
    (no `Option` unwrap). `HasExtension::extension_mut` now returns `&mut Vec`.
- Coded fields (T10) with a `required` binding are now typed as their `codes`
  enum via the new `fhir::r5::coded::Coded<E>` wrapper (`Known(E)` |
  `Unknown(String)` fallback for wire compatibility), instead of the opaque
  `types::Code`. 343 fields retyped. `Coded::code()` returns the wire string;
  `Coded::known()` the enum. See `spec/05-code-systems.md`.

## [0.3.0] - 2026-07-11

### Changed (breaking)
- **Choice types (`value[x]`) are now enums.** Every FHIR choice element is
  modelled as a generated `#[derive(FhirChoice)]` enum (one variant per allowed
  type), held via `#[serde(flatten)]`, replacing the flattened `value_<type>`
  fields (e.g. `Observation.value_quantity`/`value_string`/… → `value:
  Option<ObservationValue>`). This makes "at most one" a compile-time property
  and models the paired `_value<Type>` primitive extensions. 258 choice elements
  across all datatypes and resources were converted by the `choice_gen`
  generator. See `spec/11-choice-types.md`.
  - Primitive choice variants use `fhir::r5::choice::Primitive<T>` to carry the
    `_value<Type>` extension; complex variants hold `Box<T>`.
  - The choice enums live in their type's module (e.g.
    `resources::observation::ObservationValue`,
    `types::extension::ExtensionValue`).
  - Deserialization is lenient (a malformed choice → `None`); see the spec for
    why strict rejection isn't possible under `flatten`.

### Added
- `fhir::r5::choice` module with `Primitive<T>` and the `FhirChoice` derive
  (`fhir-derive-macros`).

## [0.2.0] - 2026-07-11

### Added
- **Primitive extensions (`_field`).** Every FHIR primitive element now has a
  sibling field `<field>_ext` (serde-renamed to the `_field` key) of type
  `Element` — `Option<Element>` for scalars, `Option<Vec<Option<Element>>>` for
  repeating primitives — so `id`/`extension` on primitive values round-trip
  instead of being dropped. Applied across all datatypes and resources via a new
  metadata-driven generator (`src/r5/parse/siblings.rs`). See
  `spec/09-primitive-extensions.md`. `#[derive(Validate)]` recurses these
  siblings automatically. New example `examples/primitive_extensions.rs`.
- **Element metadata table** (`fhir::r5::meta`): a compile-time, path-keyed table
  of per-element cardinality, coded-value bindings, `value[x]` choice types,
  reference target profiles, and summary membership, generated from the spec
  (`src/r5/parse/meta.rs`).
- GitHub Actions CI: build, test + doctest, `clippy -D warnings`,
  `doc -D warnings`, MSRV (1.88), llms sync check, and `cargo publish --dry-run`.
- Official-examples round-trip test suite (`tests/roundtrip_official_examples.rs`)
  with a committed curated subset and an `#[ignore]` full-set run, plus
  `bin/fetch-examples`. Full-set round-trip improved from 2760 to 2780 passing.
- `CHANGELOG.md`, `CONTRIBUTING.md`, and AI-readable `llms.txt` / `llms.json`.
- `rust-version` (MSRV) declared as `1.88` on both packages.

### Note

- The `_field` rollout adds many optional fields; construct resources with
  `..Default::default()` (as the examples do) to stay forward-compatible.

## [0.1.0] - 2026-07-11

Initial release: the complete FHIR R5 (5.0.0) data model in idiomatic,
`serde`-serializable Rust, generated from the official specification JSON.

### Added
- **158 R5 resources** as Rust structs under `fhir::r5::resources`, each
  round-tripping to and from canonical FHIR JSON via `serde`, plus a polymorphic
  `Resource` enum tagged by `resourceType`.
- **21 primitive datatypes** (transparent newtypes such as `Code`, `Id`,
  `DateTime`) and **50 complex datatypes** (`Period`, `HumanName`,
  `CodeableConcept`, …) under `fhir::r5::types`.
- **400+ FHIR `CodeSystem`s** as type-safe enums under `fhir::r5::codes`.
- **Lightweight validation** via the `Validate` trait and `#[derive(Validate)]`
  (`fhir-derive-macros`), reporting pathed `ValidationIssue`s and checking
  primitive format constraints recursively.
- **A spec-driven code generator** under `fhir::r5::parse` that reads the FHIR
  specification JSON shipped in `DEFINITIONS_DIR` and emits the Rust model.
- Runnable examples: `build_patient`, `validate_resource`, `read_bundle`,
  `code_systems`.

[Unreleased]: https://github.com/fhir-rust/fhir-rust/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/fhir-rust/fhir-rust/compare/v0.4.0...v1.0.0
[0.4.0]: https://github.com/fhir-rust/fhir-rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/fhir-rust/fhir-rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/fhir-rust/fhir-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/fhir-rust/fhir-rust/releases/tag/v0.1.0
