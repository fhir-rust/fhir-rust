# Before you commit, and before you push

## Local checks

```sh
cd fhir-<engine>              # or fhir/, fhir-store/, fhir-loco/
cargo build
cargo test                    # passes with no database; proves little on its own
scripts/db.sh up              # start that port's engine in a container
scripts/db.sh test            # the real gate: snapshot isolation, row locks,
                               # the append-only audit trigger, index-using
                               # search plans — none of this is exercised
                               # without a live server
```

If the change touched any shared-core file (see
[`shared-core-checklist.md`](shared-core-checklist.md)), also run, from the
repo root:

```sh
./scripts/check-shared-core.sh --diff
```

Other root-level gates, run as relevant to the change:

```sh
./scripts/check-doc-examples.sh      # doc code samples still compile/match
./scripts/check-forbid-unsafe.sh     # no unsafe crept in
./scripts/check-published-match.sh   # published version matches source
./scripts/check-tags.sh              # tag naming (TG1.x)
./scripts/check-trademarks.sh        # HL7®/FHIR® usage
```

## Commit message

- **One logical change per commit.** A shared-core edit across all six ports
  is one logical change (rule 2), not six.
- **Reference requirement ids**: `fix(sqlite): fold NFD before lowercase (L4)`.
- **Name the port that drove a spec amendment** (`C0.22`) if the commit
  brings the spec in line with what a port already does — otherwise a
  considered generalization and a rubber stamp are indistinguishable later.
- **Say `closes F-NN`** if the commit closes an audit finding. Findings close
  when the underlying thing is fixed — rewriting the text that described the
  finding does not close it.
- **State what you didn't verify** — a skipped live test, an unset DSN, an
  engine you didn't try — directly in the message (`T11.12`). A silent skip
  reads as a pass to the next person.
- End with the required trailer if instructed by the environment you're
  working in (this project's agent harness appends a `Co-Authored-By`/
  `Claude-Session` trailer automatically — don't duplicate it by hand).

## Before pushing

**Ask first**, every time — this is a standing rule regardless of the
history below. The two older warnings that used to justify extra caution are
both resolved and shouldn't be revived:

- The six ports do **not** have their own `origin`s (**F-11** resolved) —
  they're directories in one repository, one remote,
  `git@github.com:fhir-rust/fhir-rust.git`.
- The old `fhir-store/`-nested-repository warning (**F-37**) was about a
  *different* directory that used to carry this name (the HTTP surface,
  since renamed `fhir-loco/`, **F-45**) — the persistence-core crate that
  holds the name `fhir-store` today has never had a nested `.git`
  (**F-72**, verified 2026-08-04: `git ls-files fhir-store/` lists its
  files tracked normally, no `160000` gitlink entries in the index).

Neither is a reason to skip asking; it's just no longer a reason to add
*extra* caution beyond the standing rule.

## Sanity checks specific to this repo

- Did a `tasks.md` checkbox change? Don't trust or write a `[x]` without
  checking the [conformance matrix](../../spec/databases/conformance-matrix.md)
  first — `tasks.md` files in this repo have a documented history of ticked
  boxes ahead of the code (**F-27**).
- Did the change touch a README, book chapter, or `tasks.md` across more
  than one port? Check it isn't a text-substitution edit that renamed the
  engine but kept another engine's claims (the **F-01**/**F-08**/**F-16**
  failure mode) — see [`agents/documentation.md`](../../agents/documentation.md).
- Did a search turn up "the same finding" in all six ports? That's usually
  one finding, not six — scope greps to one port unless you mean to search
  everywhere.
