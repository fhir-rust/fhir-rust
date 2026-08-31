# 10 — Invariant coverage

FHIR® states many rules as `ElementDefinition.constraint` **invariants** —
FHIRPath expressions such as "an extension must have a value or children, not
both". This spec records which of them the crate enforces, and enumerates the
rest so that unenforced rules are visible rather than silently dropped.

## Requirements

- **R10.1** Every invariant the crate does not enforce MUST be listed here.
  Silence is not permitted: a reader must be able to tell what validation does
  and does not cover.
- **R10.2** An invariant is enforced only if it can be checked structurally,
  without a FHIRPath evaluator. Enforcement lives in `invariant_stmts` in
  `fhir-derive-macros`, so it applies to every release at once.

## Scale

| Release | Distinct invariant keys | Constraint occurrences | `ele-1` occurrences |
|---|---:|---:|---:|
| R6 | 360 | 9,782 | 8,705 |
| R5 | 314 | 10,992 | 8,363 |
| R4 | 240 | 8,971 | 6,710 |
| R3 | 187 | 1,039 | 383 |
| R2 | 147 | 155 | 1 |

Every figure in this spec is produced by [`bin/invariant-counts`](../bin/invariant-counts);
run it and paste when the vendored definitions change.

Both counts are taken over the **`snapshot`** element lists of the
`StructureDefinition`s in a release's `profiles-resources` and `profiles-types`
definitions. The qualifiers matter, and were previously unrecorded: counting
`differential` as well adds a few hundred (R5 goes to 11,374), and including
`profiles-others` changes the key column too (R5 goes to 321).

The occurrence column is not a measure of how much a release asks for, and the
right-hand column is why. It is dominated by `ele-1` — "all FHIR elements must
have a @value or children" — which the later releases restate on every element
of every snapshot and the earlier ones do not. R2 states 147 distinct rules in
155 occurrences, because it records each one essentially once; R5 states 314 in
10,992, because 8,363 of those are one universal constraint repeated. R2 is
thus not two orders of magnitude simpler than R5, though the middle column says
so.

Compare the *keys* column when reading this as coverage.

## Enforced

| Key | R6 | R5 | R4 | R3 | R2 | Rule |
|---|---:|---:|---:|---:|---:|---|
| `ele-1` | 8705 | 8363 | 6710 | 383 | 1 | All FHIR elements must have a `@value` or children |
| `ext-1` | 22 | 1637 | 1295 | 1 | 0 | An extension must have either extensions or `value[x]`, not both |
| `dom-2` | 124 | 122 | 145 | 116 | 1 | A contained resource SHALL NOT itself contain nested resources |
| `dom-4` | 124 | 122 | 145 | 116 | 1 | A contained resource SHALL NOT have a `meta.versionId` or `meta.lastUpdated` |
| `qty-3` | 7 | 7 | 7 | 7 | 7 | If a unit code is present, the system SHALL also be present |
| `inv-1` | 1 | 3 | 3 | 4 | 2 | A parameter must have one and only one of (value, resource, part) |
| `att-1` | 1 | 1 | 1 | 1 | 1 | If an Attachment has data, it SHALL have a `contentType` |
| `drq-1` | 1 | 1 | 1 | 0 | 0 | Either a path or a searchParam must be provided, but not both |

These are the structurally checkable invariants: each is a presence, absence, or
exclusive-choice test that needs no FHIRPath evaluator. Between them they cover
**8,363 of R5's 10,992 constraint occurrences**. `ele-1` is almost all of that;
`ext-1` covers 1,637 R5 elements; the two `dom-*` rules apply to every domain
resource's `contained` list; `qty-3` applies to `Quantity` and each of its six
specializations.

- **R10.5** `ele-1` — `hasValue() or (children().count() > id.count())` — MUST
  be enforced, and does **not** require a FHIRPath evaluator in this
  representation. This spec previously said it did, which was wrong: the
  expression names only `children()` and `id`, and against a statically typed
  model "children other than `id`" is just "fields other than `id`". The
  primitives carry their value in a newtype and always satisfy the first
  clause, so only the complex types can be empty.
- **R10.6** `ele-1` MUST NOT be applied to a resource root. In FHIR `Resource`
  descends from `Base`, not from `Element`, and the definitions agree: of the 71
  root elements carrying `ele-1` in R5, every one is a datatype. An empty
  `Patient` is unhelpful but is not an `ele-1` violation. A resource is
  identified by its `implicitRules` field, which comes from the `Resource` base
  and appears in no datatype or backbone element.
- **R10.7** An element whose only child is its `id` MUST fail `ele-1`. The
  expression is a strict inequality against `id.count()`, so `id` does not count
  towards the children that satisfy it.

- **R10.3** A check MUST be gated on the fields it reads actually existing with
  the shape it expects, not on the struct's name alone. The releases disagree
  about both: DSTU2 models `Age` as an empty struct beside an `AgeQuantity`
  holding the fields, and R3 types `DataRequirement.codeFilter.path` as `1..1`
  where R4 has `0..1`. A check written against one release's shape and keyed
  only by name does not merely fail to apply elsewhere — it fails to *compile*
  that release. Where the shape is absent the rule is skipped, which in the
  `drq-1` case agrees with the specification, R3 not stating that constraint at
  all.
- **R10.4** A check MUST NOT be inferred from field names alone either.
  `Coding` carries `code` and `system` exactly as `Quantity` does, and must not
  be subject to `qty-3`.

**These numbers count restatement, not enforcement.** All these rules are
checked identically in every release, because enforcement lives in
`invariant_stmts` in `fhir-derive-macros` and applies to all of them at once
(R10.2). What varies is how often a release's published snapshots repeat the
rule. R2 states `dom-2` and `dom-4` once each, on `DomainResource`, and does not
state `ext-1` at all; R3 states `ext-1` exactly once, on `Extension` itself;
R4 and R5 restate it on every extensible element, which is where the four-figure
counts come from; R6 returns to stating it only on the datatypes. A low number
in this table therefore says something about the specification's editorial
style, and nothing about this crate's coverage.

## Not yet enforced (most frequent)

| Key | R6 | R5 | R4 | R3 | R2 | Rule |
|---|---:|---:|---:|---:|---:|---|
| `dom-3` | 124 | 122 | 145 | 116 | 1 | A contained resource SHALL be referred to from elsewhere in the resource |
| `dom-5` | 124 | 122 | 145 | 0 | 0 | A contained resource SHALL NOT have a security label |
| `dom-6` | 124 | 122 | 145 | 0 | 0 | A resource should have narrative for robust management |
| `dom-1` | 0 | 0 | 0 | 116 | 1 | A contained resource SHALL NOT contain narrative (dropped after R3) |
| `cnl-1` | 34 | 33 | 0 | 0 | 0 | URL should not contain a pipe or `#` — they make canonical processing hard |
| `cnl-0` | 32 | 32 | 0 | 0 | 0 | Name should be usable as a machine-processing identifier |
| `org-3` | 2 | 2 | 1 | 1 | 1 | An organization's telecom can never be of use `home` |
| `age-1` | 1 | 1 | 1 | 1 | 1 | An Age SHALL have a code if it has a value, expressing time |
| `cnt-3` | 1 | 1 | 1 | 1 | 0 | A Count SHALL have code `1` if it has a value |

What is left here is the *tail*, not the bulk: `ele-1` accounted for the large
majority of all constraint occurrences and is now enforced (R10.5). The keys
below appear once or twice each. Of the 360 distinct keys in R6, 314 in R5, 240
in R4, 187 in R3 and 147 in R2, the enforced seven cover 8,363 of R5's 10,992
occurrences and the remainder are long and flat. Regenerate these tables with
`bin/invariant-counts` when coverage changes.

These need a FHIRPath evaluator in a way `ele-1` did not: `dom-3` has to
*resolve* references across the resource, `csd-1` walks `descendants()`, and
several others project a collection and test `isDistinct()`. Traversal is the
dividing line, not FHIRPath syntax as such.

The releases do not state the same rules: `cnl-0` and `cnl-1` arrived in R5 and
carry into R6, and `dom-1` was dropped after R3. A column per release is shown
rather than one because a zero can mean either "not stated in this release" or
"not restated per element" — and the two are indistinguishable from the count
alone, which is why the sentence above the previous table says what it says.

## Future work

- A FHIRPath evaluator, which would unlock the tail: the rules that traverse —
  `resolve()`, `descendants()`, `isDistinct()` over a projection. `ele-1` is no
  longer among them (R10.5).
- Enforcing the arithmetic invariants (`app-5`: start ≤ end, and similar) that
  are structurally checkable but element-specific, most likely as generated
  per-type checks driven by the `meta` table.
- The regex rules `cnl-0` and `cnl-1`, which are structural but are stated on
  particular elements (`url`, `name`) rather than on a type, so they need the
  element table rather than the struct's shape to place them.

## Acceptance criteria

1. Every invariant in the *Enforced* table has a test in the release crates'
   `validate` module: `invariant_ele_1`, `invariant_ext_1`, `invariant_dom_2`,
   `invariant_att_1`, `invariant_qty_3`, `invariant_drq_1`, `invariant_inv_1`.
   `invariant_ele_1` asserts all three of its edges: an empty element fails, an
   element whose only child is `id` fails (R10.7), and an empty resource root
   does not (R10.6).
2. A test asserts that `qty-3` does **not** fire on a `Coding`, which has the
   same two fields (R10.4).
3. Every release crate's unit tests run in CI, so a check that compiles against
   one release's shape and not another's fails the build (R10.3).
4. Unrecognized constraints are enumerated here rather than dropped.
5. The counts above match the shipped specification JSON, and are produced by
   `bin/invariant-counts`.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
