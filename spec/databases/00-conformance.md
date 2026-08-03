# 0. Conformance

This section defines how to read the rest of the specification: what the
keywords bind, how requirements are numbered, what it means for a port to
conform, and how the text is changed.

## Normative language

- **C0.1** The keywords MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD,
  SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL are to be interpreted as described
  in RFC 2119. They are normative only when capitalized.
- **C0.2** Prose that carries no keyword is **rationale**. Rationale explains
  why a requirement exists and MUST NOT be read as imposing an obligation of its
  own. It is kept in the text deliberately: a requirement whose reason is
  unrecorded is a requirement that will be removed by someone who does not know
  what it was protecting.
- **C0.3** Examples, tables of measured numbers, and file paths in rationale are
  illustrative. Where an example and a requirement disagree, the requirement
  governs and the example is a defect.

## Requirement identifiers

- **C0.4** Every normative statement carries an identifier of the form
  `<prefix><section>.<ordinal>[<suffix>]` — `M3.16b`, `PR12.6`, `T11.12`. The
  prefix is fixed per section and listed in [`index.md`](index.md).
- **C0.5** Identifiers are **stable and never reused**. A requirement that is
  withdrawn keeps its number, marked withdrawn; its number MUST NOT be assigned
  to anything else. A requirement that is amended keeps its number. A
  requirement that is split gains lettered suffixes (`M3.16` → `M3.16a`,
  `M3.16b`), and the parent number continues to exist.

  Reuse is the failure this rule exists to prevent: a citation in a test name, a
  commit message, a regulator's finding, or an auditor's workpaper is written
  once and read years later. If `M3.16` means something different in 2029 than
  it meant in 2026, every one of those citations silently becomes a lie, and
  nothing in the tree reports it.
- **C0.6** Section numbering has gaps (7, 8, and 14 in the core). Gaps are
  deliberate — see [retired sections](#retired-sections) — and MUST NOT be
  closed by renumbering.
- **C0.7** `M14.x` is reserved, in every port, for that port's own dialect
  annex. The core MUST NOT define an `M14.x` requirement, because six different
  files would then define the same identifier.

## Conformance profiles

- **C0.8** A port conforms at exactly one of four levels. The level is a claim
  about what has been *verified*, not about what has been written.

  | Level | Means |
  | --- | --- |
  | **Scaffold** | The workspace builds. DDL and store are inherited or absent; nothing is verified against the target engine. |
  | **Schema** | Generated DDL executes on the target engine, verified by a live test. No store. |
  | **Store** | Shred, reconstruct, history, and search work against the target engine, verified by a live round-trip over the FHIR example corpora. |
  | **Reference** | Store, plus the full `T11.x` suite green — concurrency, audit, redaction, tamper evidence, upgrade, and benchmarks. |

- **C0.9** A port MUST state its level in its own `spec/index.md`, and the level
  MUST be justified by tests that run in that port's CI. A level claimed on the
  strength of inherited code is Scaffold, whatever the code does.
- **C0.10** A port MUST NOT claim a level whose evidence comes from a different
  engine than the one it targets. Running a port's live suite against a
  substitute engine — because that is the container that was already in the
  pipeline — produces a green build and no evidence, which is worse than a red
  one. See [`audit.md`](audit.md) finding **F-06**.
- **C0.11** Documentation MUST NOT describe a capability at a level above the
  port's. A README inherited from a Reference-level port and text-substituted
  into a Scaffold-level one asserts, in the new product's name, results that
  were never obtained for it.

## Departures

- **C0.12** A port that cannot satisfy a core requirement MUST record a
  **departure** in its dialect annex, as a numbered `M14.x` requirement that
  names the core requirement it amends and says what holds instead.
- **C0.13** A departure MUST NOT weaken an invariant listed in
  [§15](15-portability-and-dialects.md) as engine-independent — round-trip
  fidelity, search semantics, the fold, canonical form, or the history chain.
  Those are the properties that make six ports one product; a port that departs
  from them is a different product wearing the name.
- **C0.14** An undeclared departure is a **defect in the port**. Discovering
  that a port has behaved differently all along does not retroactively make it
  an amendment; it makes it a finding.

## Retired sections

- **C0.15** Sections 7 (REST API) and 8 (CLI) are **retired**. These projects
  are embeddable libraries: the workspaces contain `-map`, `-gen`, and `-store`
  crates and no server or binary crate. Requirements that assumed an HTTP
  service or a command-line tool are therefore not in force.

- **C0.16** Retired identifiers remain reserved (C0.5) and MUST NOT be reused.
  The following are cited by sections that are still in force, and those
  citations are **unresolved**. They are recorded here rather than quietly
  deleted, because deleting a citation is indistinguishable from satisfying it.

  | Retired id | Cited by | Subject, as far as the citation reveals |
  | --- | --- | --- |
  | `A7.8` | §13 (HIPAA §164.312(e)) | transport security at the service edge |
  | `A7.10` | `T11.6` | racing conditional creates yield exactly one resource |
  | `A7.11` | `T11.7`, `T11.11` | no submitted value echoed in an OperationOutcome |
  | `A7.12` | §13 (ONC/HTI) | CapabilityStatement conformance |
  | `M8` | §13 (ONC/HTI Bulk Data) | `$export` |

  **Where that behaviour lives, as of 2026-08-03.** These ids were retired as
  "out of scope", meaning out of scope *for the ports*. The REST API they
  describe exists — it is `fhir-loco` — so they are dangling in this
  specification rather than obsolete in the repository. `A7.12` in particular is
  cited by a live audit finding (**F-57**), where it caught a
  CapabilityStatement that declared a read-only server while the router served
  writes.

  They are **not** un-retired here: `spec/databases/` governs the ports, and
  §7 does not describe them. Moving them to a specification that governs
  `fhir-loco` — or giving `fhir-loco` one that adopts them — is the open
  decision in [`audit.md`](audit.md) **F-04** and **F-58**.
  | §7 | `T11.2` | "every REST interaction in §7" |

  A reader MUST treat a requirement citing one of these as **partially
  specified**: the citing requirement is in force to the extent it can be
  evaluated against the core, and the cited obligation is unstated. Two of them
  have surviving core-side substance — the store implements conditional create
  and conditional delete, so `T11.6`'s atomicity assertion is testable today —
  and the rest do not.

  The disposition is deliberately *not* to reconstruct the retired text from its
  citations. Reconstructed requirements would carry the authority of ratified
  ones without ever having been reviewed, and §13 maps them to regulatory
  obligations, which is the worst possible place for text nobody wrote. Tracked
  as [`audit.md`](audit.md) **F-04**; resolving it means the owner either
  restores the sections or amends the citing requirements.

- **C0.17** Text elsewhere in the core that specifies a service — `serve`,
  `--bind`, `--admin-bind`, request headers, HTTP status codes, `/metrics`,
  `/health`, `/ready` — describes a **service layer that does not exist in this
  monorepo**. It is retained because the obligations it encodes are real and
  will bind whatever service is eventually built, but it MUST NOT be read as
  describing shipped behaviour, and a port MUST NOT be marked non-conformant for
  failing to implement it. Affected: parts of `O10.1`, `O10.3`, `O10.5`,
  `O10.7`–`O10.9`, `V9.2`, `V9.3`, and most of `PR12.1`–`PR12.8`. Each such
  requirement is marked **[service]** at its point of use. Tracked as
  [`audit.md`](audit.md) **F-05**.

- **C0.18** Similarly, requirements phrased as CLI subcommands — `init`, `load`,
  `gen`, `verify-audit`, `chain-witness`, `chain-resign`, `chain-key-new`,
  `purge`, `export` — name **library operations** in this monorepo. The
  obligation attaches to the operation, not to the existence of a command. Where
  a store exposes the operation as a method, the requirement is in force; where
  no such method exists, the requirement is unmet and the
  [conformance matrix](conformance-matrix.md) says so.

## Amending this specification

- **C0.19** A change to a normative statement MUST change its text in this
  directory, in one commit, with the reason stated. Changing six copies is no
  longer possible and no longer permitted (`W16.5`).
- **C0.20** A new requirement MUST take the next unused ordinal in its section.
  A requirement MUST NOT be added mid-sequence by shifting the ones after it.
- **C0.21** A requirement MUST be traceable to evidence: a test, a CI gate, or
  an explicit entry in the [conformance matrix](conformance-matrix.md) recording
  that it is unverified. "Specified, implemented, untested" is a state the
  matrix names rather than hides — see `T11.12`.
- **C0.22** Amending the core to match what a port already does is permitted and
  expected; doing it **silently** is not. The commit MUST say which port's
  behaviour drove the change, so a reader can tell a considered generalization
  from a rubber stamp.

---

Part of the [fhir-databases specification](index.md).
