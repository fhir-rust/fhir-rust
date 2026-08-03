# Topic guides

Companion files to [`../AGENTS.md`](../AGENTS.md), which you should read first.
Each covers one area in enough depth to work in it without reading the others.

These guides are for the **database family** (`fhir-<engine>/`). The model crate
has its own set in [`../fhir/AGENTS/`](../fhir/AGENTS/architecture.md); read
those instead when working in [`../fhir/`](../fhir/).

| Guide | Read it when you are | Key rules |
| --- | --- | --- |
| [spec-workflow.md](spec-workflow.md) | changing behaviour, or amending a requirement | `C0.19`–`C0.22` |
| [rust.md](rust.md) | editing any `.rs` file | `X15.1`, `W16.7` |
| [testing.md](testing.md) | adding or changing a test | `T11.10`–`T11.14` |
| [databases.md](databases.md) | touching `ddl.rs`, a store, or a new engine | `X15.6`, `O10.12` |
| [documentation.md](documentation.md) | writing a README, book chapter, or doc | `W16.8`–`W16.10` |
| [security.md](security.md) | near PHI, audit, keys, or logging | `M3.16b`, `O10.2`, `PR12.6` |
| [release.md](release.md) | versioning, publishing, or CI gates | `O10.10`, `O10.11`, `W16.14` |

## The shortest possible summary

- `/spec/databases` is normative and lives once. Ports add annexes, not copies.
  [`/spec/index.md`](../spec/index.md) says which family's spec applies at all.
- The pure-Rust core is identical in all six ports. Change it in all six.
- A dialect difference is an `M14.x` departure that cites what it amends.
- Do not claim more than the port's conformance level.
- Say what you did not verify.
- Before releasing, run `scripts/check-published-match.sh` — a published version
  must match the source that claims it (`O10.11`), and nothing else checks it
  across all four families.
