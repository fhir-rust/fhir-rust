# Governance

How decisions get made here, by whom, and where they are written down.

This document describes the project as it is, not a structure it hopes to grow
into. Read it with [`MAINTAINERS.md`](MAINTAINERS.md), which is the roster and
the continuity risk, and [`CONTRIBUTING.md`](CONTRIBUTING.md), which is how to
participate.

## The shape of it

**One maintainer decides.** There is no steering committee, no technical
oversight body, no vote, and no legal entity. Calling that "benevolent dictator"
would dress it up; it is simply a one-person project with the decision-making
that implies, and the risks in [`MAINTAINERS.md`](MAINTAINERS.md) follow
directly from it.

What makes that tolerable — and what this project does have instead of
governance bodies — is that **decisions are constrained by written
specifications and recorded in public**. The maintainer can decide anything, but
cannot decide it *silently*, and cannot decide it in a way that contradicts a
requirement without amending that requirement in the open.

The repository is owned by the `fhir-rust` GitHub organisation rather than by a
personal account. That is a small but real difference: a second owner could be
added without moving the repository. It has not been.

## What is decided where

| Decision | Decided by | Recorded in |
| --- | --- | --- |
| What the software must do | the maintainer, after discussion | [`spec/`](spec/index.md), as a numbered requirement |
| Whether a port meets a requirement | evidence, then the maintainer | the [conformance matrix](spec/databases/conformance-matrix.md) |
| That a port may differ from the core | the maintainer | an `M14.x` departure in that port's dialect annex, **naming the requirement it amends** |
| That the code diverges from the spec | anyone who finds it | the [audit register](spec/databases/audit.md), as a numbered finding with evidence |
| What ships, and when | the maintainer, or an authorized Claude Code session judging readiness against [`agents/release.md`](agents/release.md)'s gates (delegated 2026-09-02) | [`CHANGELOG.md`](CHANGELOG.md), and crates.io |
| Whether a change is merged | the maintainer, per [`CODEOWNERS`](CODEOWNERS) | the pull request |
| Conduct | the maintainer | [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) |

**Release readiness is a delegation, not an abdication.** Until 2026-09-02,
only the maintainer could judge a specific release ready; the maintainer has
since authorized an agentic Claude Code session to make that call too,
against the same checklist ([`agents/release.md`](agents/release.md):
published-artefact match, live suite, changelog, conformance matrix, no open
High finding) that bound the maintainer's own judgment — not a looser one
invented for the occasion. This is delegation of a specific, checkable
decision, not a general grant: the maintainer remains accountable for every
release regardless of who judged it ready
([`AI_STATEMENT.md`](AI_STATEMENT.md) §4), can revoke the delegation at any
time, and the "decisions are recorded" rule below applies to this decision
exactly as to any other — a release a session judged ready is recorded in
`CHANGELOG.md` and a commit citing which gates passed, not asserted from
inside the session alone.

## The rules that bind the decision-maker

These are the parts worth reading even if you never contribute, because they are
what a dependent is actually relying on.

**Specification before code.** Behaviour is decided in [`spec/`](spec/index.md)
before it is written in Rust. A change to what the software promises changes the
specification in the same pull request.

**Requirement ids are permanent** (`C0.5`). Never renumbered, never reused, not
even across a file move. A number in a commit message from a year ago still
resolves to the same obligation.

**An undeclared departure is a defect, not an amendment.** If the code does not
do what a requirement says, the code is wrong until someone amends the
requirement deliberately and by number. This is the rule that stops "the
implementation is the spec" from creeping in.

**Documents have precedence, and it is written down**
([`spec/index.md`](spec/index.md)): within a family, that family's specification
is normative for its code; no family is normative for another; and **nothing in
a `README.md`, `book/` chapter, `plan.md`, `tasks.md`, or code comment is
normative.** Those describe; the specifications decide. This project has been
burned specifically by documentation that claimed authority it did not have, so
the ordering is enforced rather than assumed.

**Claims may not exceed evidence** (`C0.11`, `C0.9`). A conformance level is
what has been *verified for that port*, not what its code contains. Shared code
from a port where a test passes is `?` in the matrix, not `•`. A green developer
machine is weaker evidence than a green pipeline, and both are named when a
claim is made.

**Say what you did not verify.** A skipped test, an unset DSN, an untried
engine — in the commit message, and in the audit register if it persists.

## Where decisions are recorded

**A decision that exists only in a chat log, an email, or an AI tool session is
not a decision this project made.** If it matters, it is in the specification,
the audit register, the changelog, or a commit message that cites an id. That
rule is the reason this repository can be handed to someone else.

Owner rulings that resolve an ambiguity — a specification silence, a naming
choice, a scheduling call — are recorded where the ambiguity lives, with the
reasoning, and marked as decisions rather than presented as though they had
always been obvious.

## Disagreeing

Open an issue and cite the requirement id, the finding, or the file. Evidence
wins arguments here: a reproducible case beats an opinion, and a specification
contradiction with both ids beats either.

**Being right is enough; you do not need to be polite about being right**,
though it helps. Telling this project that a claim is wrong is a service,
including when the claim was written by the maintainer — see
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), which makes overstating what the
software does a conduct matter, and [`RFC.md`](RFC.md), which lists what the
project is actively unsure about.

**There is no escalation path beyond the maintainer.** With one person holding
every role, an appeal goes to the person who made the decision. That is a real
limitation and it is stated rather than papered over. The genuine alternative is
below.

## Forking

The licence is a five-way choice ([`LICENSE.md`](LICENSE.md)), the history is
public, every gate is a committed script, and every design decision is in the
tree with its reasoning. **A fork is a complete and legitimate continuation**,
and this project's position is that it should be taken rather than waited on —
whether because the maintainer is unavailable, or because you disagree with a
decision and it matters enough.

That is not a threat and it is not defeatism. It is the actual governance
guarantee a one-person project can offer: you are never locked in by anything
except your own migration cost.

## Becoming a maintainer

The route is open and it is deliberately unbureaucratic:

1. Contribute sustainedly — code, specification review, dialect expertise,
   documentation, or triage. [`CONTRIBUTING.md`](CONTRIBUTING.md) lists what is
   most useful.
2. Demonstrate the judgement the rules above require: claims matched to
   evidence, departures declared, unverified things named as unverified.
3. Be invited.

When someone takes it, three edits are the whole mechanism:
[`MAINTAINERS.md`](MAINTAINERS.md) gains a row, [`CODEOWNERS`](CODEOWNERS) gains
their handle on the areas they own, and the publishing-identity table gains a
second holder wherever the identity permits one — which, for the GitHub
organisation, it does.

There is no probationary period and no maintainer agreement to sign. There is
also no pretence that this is likely to happen soon; it is written down so that
it *can*.

## What would change this document

- **A second maintainer.** Most of the honest caveats above exist because there
  is one. Adding a second changes the escalation path, the review model, and the
  continuity story, and this document changes with them.
- **A legal entity.** None exists. If one is formed, ownership, trademark, and
  liability questions arrive with it.
- **An outside dependency of consequence.** If an organisation deploys this in
  patient care, the informality above becomes less defensible, and the project
  should say so rather than continue as if nothing changed.

## What this project does not govern

**FHIR® is not ours.** The HL7® FHIR® standard is governed by Health Level
Seven International through its own balloting and work-group process, and this
project follows it. Nothing decided here changes the standard, and a
disagreement with FHIR itself belongs at [chat.fhir.org](https://chat.fhir.org)
or in an HL7 work group, not in this issue tracker.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
