# Rust conventions

## The shared core

These files are **identical across all six ports** modulo the crate-name
substitution (`fhir_sqlite_map` ↔ `fhir_postgresql_map`) and annex cross-
references. This is `X15.1`, and it is the single most important structural fact
about the repository.

```
crates/fhir-<engine>-map/src/
  model.rs        the relational map types
  shred.rs        JSON → rows
  reconstruct.rs  rows → JSON
  value.rs        leaf value representation
  fold.rs         the accent/case fold
  canon.rs        canonical JSON for the hash chain
  error.rs        shred/reconstruct errors
crates/fhir-<engine>-gen/src/
  build.rs  lib.rs  names.rs  search.rs  spec.rs
```

**Editing one is a divergence, not a fix.** Apply the change to all six in the
same commit (`W16.7`). Verify:

```sh
# from the repo root — compare a shared module across ports
norm() { sed -E 's/fhir[-_](postgresql|sqlite|mysql|mariadb|mssql|oracle)/fhir_X/g' "$1"; }
for d in fhir-postgresql fhir-mysql fhir-mariadb fhir-mssql fhir-oracle; do
  printf '%-18s ' "$d"
  diff <(norm fhir-sqlite/crates/fhir-sqlite-map/src/shred.rs) \
       <(norm "$d"/crates/*-map/src/shred.rs) | grep -c '^[<>]'
done
```

`scripts/check-shared-core.sh` gates this (**F-10** fixed) — the loop above is
what it automates, across 100 files including `gen/tests/`. It runs from
`.github/workflows/gates.yml`, the one workflow in this repository that is in a
directory GitHub reads (**F-49**). Note that the
script compares *tokens*, not lines, which the loop above does not: rustfmt
wraps by crate-name length, so two ports can differ by whitespace with no code
difference (`X15.1a`, **F-48**). It currently carries **no exemptions**: `fhir-postgresql-map`
had no `canon.rs` until **F-07** was closed, and that was the only one. Keep the
list empty if you can; an entry there is a divergence that survived review, and
must cite the finding or `M14.x` departure that allows it.

## The dialect surface

Exactly two places may differ per port:

- **`map/src/ddl.rs`** — which SQL the generator emits. `col_sql` is the one
  function that names engine types; everything above it speaks `ColTy`.
- **`store/`** — driver, transactions, placeholder syntax, search SQL.

If you find yourself needing a third place, you have found a missing
abstraction, not a missing exception. Say so before adding one.

## Comments

The house style is unusual and worth matching: comments explain **why the
obvious thing is wrong**, at the point where someone would otherwise do it.

```rust
// Not DECIMAL: M3.6 requires a decimal's original textual precision to
// survive round-trip, and `DECIMAL(65,30)` returns `1.50` as
// `1.500000000000000000000000000000` — a fixed declared scale cannot
// preserve a per-value lexical form.
ColTy::Numeric => "TEXT",
```

That comment is doing real work: without it the next person "fixes" a text
column that should hold a number, and silently breaks round-trip fidelity.

- Cite the requirement id when the reason is normative (`M3.6`, `L4`, `X15.2`).
- Prefer explaining the rejected alternative over describing the code.
- Do not narrate what the line already says.
- Module headers carry the design argument. Read `store/src/chain.rs`'s header
  before touching anything about hashing.

## Errors

- `thiserror` for library errors, `anyhow` at the edges.
- An error MUST name the path or identifier that failed, not just the resource
  (`R4.3`, `V9.3`). "Unknown element" is not actionable; "unknown element
  `Patient.foo.bar`" is.
- **An error message must not echo a submitted value** (`T11.7`). The value may
  be PHI, and an error is the one place it escapes into a log, a response, and a
  ticket at once.
- Unsupported operations return an explicit `Unsupported` with what is missing —
  never a silent no-op, never a stub returning success. `fhir-sqlite`'s
  `transact_audited` is the model: it refuses, and the refusal is the correct
  answer.

## Async

- `tokio`, multi-thread runtime.
- Async drivers where they exist (`tokio-postgres`, `mysql_async`, `tiberius`).
- `rusqlite` is synchronous, so `fhir-sqlite` wraps calls in `spawn_blocking`.
  Do not "simplify" that away.

## Formatting and lints

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

Both gate CI on both forges. MSRV is `rust-version` in the workspace
`Cargo.toml` (currently 1.90) and is a promise to downstream users — CI builds
on exactly that toolchain, because an unverified MSRV is a guess.

## Dependencies

- Add to `[workspace.dependencies]`, reference as `foo.workspace = true`.
- Every driver dependency carries a comment explaining **why that driver**, and
  a port with *no* driver says why not — `fhir-oracle` is Scaffold and choosing
  one is blocked on open annex questions. Two ports carried a comment describing
  a driver they did not depend on (**F-03**, fixed); do not add a third.
- New dependencies pass `cargo deny` (advisories, licenses, bans). A license
  that is not in the workspace's list is a conversation, not a commit.

## Generated artifacts

`assets/fhir-<engine>-relmap-{r3,r4,r5}.json.gz` and `CHECKSUMS.txt` are
committed on purpose (`G2.1`), so a build never needs the FHIR spec packages or
a network. Regeneration is deterministic (`G2.2`) — if the checksum moves and
the FHIR input did not, that is a generator bug and the diff is the evidence.
