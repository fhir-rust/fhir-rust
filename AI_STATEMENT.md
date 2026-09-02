# AI statement

**Version 1.2.0 — effective 2026-09-02.**

## 1. Scope

This document covers the use of AI tools in producing everything in this
repository: the Rust source of all four families, the specification packages'
generator, the tests, the specifications themselves, the CI workflows, the
documentation, and this document.

It does not cover an AI system in the product, because there is none:
**fhir-rust ships no AI.** No model is trained, embedded, or called at run time;
no inference happens anywhere in any published crate. The libraries open no
socket except to the database you configure, and the HTTP surface speaks FHIR®
REST and nothing else. AI is used to *build* the software, in the same sense a
compiler and a linter are used to build it.

## 2. Which frameworks apply here, and which do not

Stated plainly, because borrowed authority is worse than none. These are the
project's readings, not legal advice, and no legal review has been performed.

- **The EU AI Act imposes no obligation on this project.** The Act binds
  providers and deployers of AI *systems*; this repository is not one, and
  Article 50's marking duties bind an AI tool's provider rather than the tool's
  user. This document is voluntary.
- **These crates are not a medical device, and cannot make a deployment
  compliant.** [`spec/databases/13-compliance-mapping.md`](spec/databases/13-compliance-mapping.md)
  already puts it in the form this project uses: *"these are components, not
  certified systems: they cannot make a deployment compliant, but they must not
  be the reason a deployment cannot be."* A downstream integrator who gives
  their product a medical purpose brings *their* product into scope; that
  classification is theirs to make, and this document exists partly so they can
  answer their own supplier questions.
- **No standard is claimed as conformity.** No certification exists, no audit
  has occurred, and the words "certified", "audited" and "validated" appear in
  this document only in this sentence, to say they do not apply.

## 3. Terms

This document reuses the W3C AI Content Disclosure vocabulary rather than
inventing one: **none** (entirely human-authored), **ai-assisted**
(human-authored; AI edited, refined, or filled in boilerplate), **ai-generated**
(AI-generated with human prompting and review), **autonomous** (AI-generated
without meaningful human oversight). An **agentic tool** is one that plans and
executes multi-step work — editing files, running builds and tests — under a
human's direction, as opposed to inline completion.

## 4. Accountability

One named human — the maintainer, in [`MAINTAINERS.md`](MAINTAINERS.md) — is the
author of and accountable for every change in this repository, whatever tool
produced the bytes. Every commit in the history carries that person, and only
that person, as git **author and committer** — the field this section's
accountability actually rests on. A tool **shall not** hold that field or sign
a commit or tag; a signature is a claim of responsibility a tool cannot make.

A commit produced in an agentic Claude Code session also carries a
`Co-Authored-By:` trailer naming the tool, and a `Claude-Session:` link to the
session transcript. This is **disclosure, not authorship**: it moves neither
the author/committer field above nor accountability under this section — it
records which tool assisted and where that session's record lives, nothing
more. Version 1.0.0 of this document banned naming a tool as co-author of
anything here, full stop, with no such distinction — while the repository's
own commits, both before that version was written and after, already carried
exactly this trailer. That was §8's own failure mode, found inside the
document meant to prevent it: prose asserting a practice the tree did not
follow. This revision describes the practice as it has actually run, rather
than continuing to assert one that was never true.

## 5. Where AI is used, and at what level

The tooling is agentic AI coding assistance (currently Claude Code, by
Anthropic), operated in sessions the maintainer directs and reviews. The
repository is explicitly organised for it: [`AGENTS.md`](AGENTS.md) is the
operational guidance every contributor and every agent shares,
[`CLAUDE.md`](CLAUDE.md) adds what is specific to working through an agent
harness, and [`agents/`](agents/index.md) holds eight topic guides. Deliberately,
no percentage appears anywhere in this document: no defensible method exists for
measuring one.

| Activity | Level | Notes |
| --- | --- | --- |
| Rust source, generator, CI | ai-generated | written in directed sessions against the specifications in [`spec/`](spec/index.md); reviewed and merged by the maintainer |
| Tests and fixtures | ai-generated | held to the same authority as the code they test; `T11.10` requires mutation-verified tests, so a test that cannot fail is a defect |
| Specifications in `spec/` and `fhir/spec/` | ai-generated | the normative text itself; adjudications about what it should *say* are the maintainer's (below) |
| Documentation, books, and this statement | ai-generated | held to [`agents/documentation.md`](agents/documentation.md) |
| Requirement adjudications, conformance-level rulings | none | the maintainer's, recorded in the specification or the audit register with reasoning |
| Judging a specific release ready, and executing it (`cargo publish`, tagging) | ai-assisted | authorized 2026-09-02; a session works through [`agents/release.md`](agents/release.md) §§1–4 (artifact integrity, build/lint/test, supply chain, claims-match-reality), judges whether they pass, and carries out §5 itself — the same gates that bound the maintainer's own judgment, not a looser one. Delegated, revocable, and recorded per §6 below |
| Review verdicts on others' contributions | none | prohibited use; see §11 |

**autonomous** appears in no row, and that is the point of the next section.

## 6. Human oversight

The maintainer directs the work, reads the result, and merges every change;
nothing lands on its own authority. Where the tools run multi-step sessions,
the decisions with consequences — what a specification silence means, whether
a port has reached a conformance level, what may be claimed publicly — are the
maintainer's. A decision that exists only inside a tool session is not a
decision this project made.

**Release readiness is the one exception, and it is a delegation, not a
lapse.** Until 2026-09-02, "is this release ready" was the maintainer's call
alone; the maintainer has since authorized a Claude Code session to make that
call too, against [`agents/release.md`](agents/release.md)'s own checklist —
the same gates that bound the maintainer's judgment, not a lighter one written
for the occasion. Three things keep this from being an autonomous exception to
everything above: it is **specific** (one checkable question, not a general
license to decide what ships); it is **revocable** (the maintainer can take it
back at any time, per [`GOVERNANCE.md`](GOVERNANCE.md)); and it is **recorded**
— a release a session judged ready still needs a `CHANGELOG.md` entry and a
commit or release note citing which gates passed, exactly as this section's
"a decision that exists only inside a tool session is not a decision this
project made" rule requires of every other decision. Accountability for the
release does not move with the decision: §4 still applies.

## 7. Quality controls, and what each one proves

AI-produced work is not a shortcut around engineering process. Each control
below names its enforcement, because a control without a failing check is a
wish.

- **Specification authority.** [`spec/`](spec/index.md) is normative and code is
  not; requirement ids are permanent (`C0.5`); precedence between the four
  families is written down. A change that departs from a requirement is a defect
  in the code, not an amendment to the specification.
- **The shared-core gate.** `scripts/check-shared-core.sh` compares the tokens of
  every file that must be identical across all six database ports. It exists
  because "edit one of six identical files" is a failure mode agents and humans
  share, and its exemption list is empty.
- **Published-artefact verification.** `scripts/check-published-match.sh` checks
  every crate's source version against what is actually on crates.io — a
  published version is immutable, so this is the control that catches a claim
  the registry does not support.
- **Documentation examples are executed.** `scripts/check-doc-examples.sh`, plus
  clippy at `pedantic` with zero warnings and rustdoc with zero warnings.
- **Live database testing.** Each port's CI provisions its real engine in a
  container and runs the store suite against it; a port whose CI cannot run its
  own engine is marked as such rather than assumed to work.
- **An audit register with evidence.** [`spec/databases/audit.md`](spec/databases/audit.md)
  records every known divergence between specification, documentation and code,
  with the command that demonstrates it. The
  [conformance matrix](spec/databases/conformance-matrix.md) is the status
  document, and a `?` there means "the code is shared from a port where this
  works and nothing tests it here".

What these controls do **not** prove is stated in §12.

## 8. The failure mode this project has already had

This section is not boilerplate, and it is the reason the rest of this document
is worth reading.

The characteristic failure of AI-generated work in this repository has not been
broken code. It has been **confident prose that nothing substantiates** — and it
happened repeatedly, at scale, before it was caught:

- **F-01** — all six ports' READMEs described a CLI that existed in none of them
  and claimed 7,399 FHIR example resources round-tripped, in ports where no
  store existed at all; three were titled for the wrong database engine.
- **F-27** — `tasks.md` checkboxes marked work complete that had never been
  done, including `T32 Encrypted database transport`, a **security** claim
  ticked in four ports where the machinery existed in one.
- **F-56** — every port's book described another engine's tooling.
- **F-75** — two ports' changelogs advertised a `serve` binary and a CLI that
  have never existed in any port.

All are fixed and all are recorded with evidence. The controls in §7 exist
because of them, and [`CLAUDE.md`](CLAUDE.md) carries a "verifying a claim
before you make one" section for the same reason. A reader evaluating this
project's use of AI should weigh that history in both directions: the errors
were real and numerous, and they were found, written down in public, and gated
against.

## 9. Licensing and provenance of AI output

The project is licensed under a five-way choice ([`LICENSE.md`](LICENSE.md)). An
AI tool's output does not launder anyone's copyright; the full provenance of
generated text is generally not knowable; and prompting alone is not treated as
authorship. In practice: contributions of substantially copied third-party
material are refused however they were produced; generated code is held to the
same originality expectations as human code; and if identifiable third-party
material is found in the tree, it is removed or licensed properly.

The generated Rust in [`fhir/`](fhir/) is a separate matter and not an AI
question at all: it is produced deterministically by a committed generator from
HL7®'s published specification packages, whose terms are HL7's. The licence here
covers the Rust source in this repository, not the standard.

## 10. Data

No patient data, no personally identifiable health information, and no customer
data exists anywhere in this project — not in the repository, not in test
fixtures, and therefore not in any prompt. Test data comes from the example
resources published inside the official FHIR specification packages, which are
modelling artifacts rather than records about people. This is a structural
property a reader can check against the tree, not a promise about tool
behaviour.

The software's own posture on PHI is a different question with a different
answer, and it is documented separately in
[`doc/trust-boundary.md`](doc/trust-boundary.md) and under `O10.2` (no PHI in
logs at default level, tested by `T11.7`).

Vendor-side data handling is governed by the tool vendor's terms; this document
deliberately makes no claim on the vendor's behalf, because such claims go stale
silently.

## 11. Prohibited uses

In this project, AI **shall not**: merge anything; adjudicate or answer reviews
of others' contributions; sign anything; decide a requirement-level question;
mark a conformance level as met; or weaken a test, an expectation, or a gate to
make something pass. The last is a standing hard rule for humans and tools
alike, and `T11.10`'s mutation requirement is how it is enforced.

## 12. Limitations and residual risks

This section exists because a disclosure without one is marketing.

- **The gates prove what they test, not correctness.** Four of six ports carry
  `?` against the concurrency and audit guarantees; the conformance matrix says
  so, and that boundary is published rather than smoothed over.
- **Review depth is one person's.** The project has a single maintainer
  ([`MAINTAINERS.md`](MAINTAINERS.md)); machine gates stand in for the review
  capacity a larger team would have. "The maintainer understands and can explain
  every merged change" is the honest claim; "every line was independently
  re-derived" would not be.
- **The §8 history is a sample, not a bound.** Those findings are the ones that
  were found. The audit register is open and one finding is open in it today
  (F-98) — a count this bullet will always be catching up to, since the
  register changes more often than this document does; the register itself is
  the thing to check, not this sentence.
- **Retroactivity.** Commits predating this statement's first issue already
  carried the same `Co-Authored-By`/`Claude-Session` trailers §4 now describes
  — this document is catching up to a practice, not dating its start.
- **Disclosure trailers are session-asserted, not independently verified.** A
  `Co-Authored-By`/`Claude-Session` trailer records what a session was
  instructed to attach, not an audited fact; nothing in this repository checks
  that one is present, accurate, or omitted correctly for a human-only commit.
- **Provenance uncertainty survives.** Whether any generated fragment echoes
  unlicensed training material is not fully knowable with current tools; §9
  states the handling, not a guarantee.
- **A session judging release readiness could be wrong about it.**
  `agents/release.md` §§1–4 are checkable, but a session concluding they pass
  is still a judgment, not a proof; nothing currently re-verifies that
  judgment independently before `cargo publish` runs. The mitigation is §6's
  three conditions (specific, revocable, recorded) plus `O10.11`'s own
  published-artefact gate catching a divergence after the fact — not a claim
  that misjudgment cannot happen.
- **Signing covers the git identity, not tool involvement.** Commits and tags
  have been SSH-signed with the maintainer's own key since 2026-08-27
  (`git log --show-signature` shows a good signature; `git config
  commit.gpgsign` is `true`, local to this repository). That verifies the
  named human authored and committed the change — it says nothing about
  whether a tool assisted, which is what §4's disclosure trailer is for
  instead. This corrects an earlier version of this bullet, which claimed no
  signature existed at all; that stopped being true 2026-08-27 and the claim
  was not updated until now.
- **This is a self-declaration.** No third party has audited it. The checkable
  artifacts in §7 are the counterweight: they can disagree with this document,
  and if they do, the document is wrong.

## 13. Review and change

This statement is reviewed at every release that changes what is published, and
revised off-cycle when any of these fires: the tooling changes materially, a
vendor's terms change in a way §9 or §10 relies on, a binding rule emerges, or a
claim in this document stops being true. The change lands as a pull request like
everything else.

## 14. Reporting

A suspected provenance, licensing, or quality problem in this repository —
including a claim in this document that does not survive checking — is a report
this project wants. Open an issue and cite this file. There is no private
security route yet; see [`MAINTAINERS.md`](MAINTAINERS.md) for what to do in the
meantime.

## 15. References

**Normative for this project:** [`LICENSE.md`](LICENSE.md);
[`spec/index.md`](spec/index.md) and the specifications it routes to;
[`AGENTS.md`](AGENTS.md), [`CLAUDE.md`](CLAUDE.md) and
[`agents/`](agents/index.md); [`MAINTAINERS.md`](MAINTAINERS.md);
[`spec/databases/audit.md`](spec/databases/audit.md).

**Informative:** the W3C AI Content Disclosure vocabulary, used for §3's terms;
the structure of this document follows the AI statement published by the
FerroEHR project, whose framing of accountability, prohibited uses and residual
risk this project adopted rather than reinvented.

## Annex A. Change log

| Version | Date | Change |
| --- | --- | --- |
| 1.2.0 | 2026-09-02 | §5/§6: judging a specific release *ready*, not only executing it, is now ai-assisted and maintainer-delegated (`GOVERNANCE.md`, `agents/release.md` §§1–5). Narrower than it sounds: one checkable question against an unchanged checklist, revocable, still recorded outside the session — §4's accountability is unmoved. |
| 1.1.0 | 2026-09-02 | §4 corrected: 1.0.0's ban on naming a tool as co-author did not match the repository's own commit trailers, before or after 1.0.0's issue; now describes that disclosure practice instead of denying it. §5/§6: added `cargo publish`/tagging execution as a maintainer-authorized, ai-assisted activity, distinct from the release *decision*, which stays `none`. §12: "Nothing is signed" corrected — commits and tags have been SSH-signed since 2026-08-27, a fact this document had not caught up to; two further limitations added (retroactivity restated accurately; disclosure trailers are session-asserted, not verified). |
| 1.0.0 | 2026-08-26 | First issue. |

## Annex B. Machine-readable summary

Levels per the W3C AI Content Disclosure vocabulary (§3); the prose above is
authoritative where the two could ever disagree.

```yaml
ai-statement:
  version: 1.2.0
  last-updated: 2026-09-02
  vocabulary: w3c-ai-content-disclosure
  disclosure-default: ai-generated
  tools:
    - name: Claude Code
      provider: Anthropic
  commit-disclosure:
    mechanism: "Co-Authored-By and Claude-Session trailers"
    changes-git-author-committer: false
    changes-accountability: false
  processes:
    design: ai-assisted
    implementation: ai-generated
    testing: ai-generated
    specification-text: ai-generated
    documentation: ai-generated
    review: none
    adjudication: none
    release-readiness: ai-assisted
    release-execution: ai-assisted
  ships-ai-system: false
  autonomous-use: none
```

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
