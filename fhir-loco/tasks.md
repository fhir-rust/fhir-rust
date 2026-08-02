
## Remaining work (as of the port commit)

Read, vread, create, update, delete, search and history work against SQLite,
verified end to end.

- [ ] **Conditional create/delete** (`If-None-Exist`). The SQLite store supports
  it; only the HTTP wiring is missing.
- [ ] **`_include` / `_revinclude`.** `refs_of` exists in the store.
- [ ] **Transaction Bundles.** Blocked on the store: `transact_audited` returns
  `Unsupported` deliberately, because a compensating implementation would claim
  an atomicity it does not have.
- [ ] **Type- and system-level `_history`.**
- [ ] **Other backends.** Only SQLite is wired. MySQL and MariaDB have native
  stores now, but not the HTTP-facing surface (`status`, `get_versioned`,
  `get_all`, `put_audited`) this layer calls — see their task lists.
- [ ] **`store::init` uses a process-global `OnceLock`**, which is why the tests
  share one database and coexist by using distinct resource ids. If a second
  backend is mounted this wants revisiting.

### Cross-cutting, all repos

- [ ] **Git remotes are wrong.** Every database repo still has `origin` =
  `git@github.com:fhirpg/fhirpg.git`, correct for at most one of them. Pushing
  any `port/*` branch as-is would send that port to the upstream project. Set
  each remote before pushing. Nothing has been pushed.
- [ ] **Shared history.** All six database repos descend from `688641a` of the
  original `fhirpg` project. Whether five separate products should keep that
  history, be squashed, or be re-rooted is a decision to make deliberately
  rather than discover after a push.
- [ ] **T70 Accent folding misses Nordic letters.** `fold("Ærø")` is `"ærø"`, so
  a search for `aero` misses it, while `Muñoz` → `munoz` works: `ñ` decomposes
  into a base letter plus a combining mark under NFD, which the fold strips,
  whereas `æ`/`ø`/`å` are distinct letters with nothing to strip. `fold.rs` is
  byte-identical across all repos, so this affects every one, including
  fhir-postgresql. It matters because the function's own doc comment cites Ærø
  as the motivating example. Needs a scope decision (Nordic only, or a full
  `unaccent` equivalent) and a `_norm` backfill.
