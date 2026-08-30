# 16. Repository and release

## Layout

- **W16.1** The monorepo root holds what is shared; each port is one directory
  holding a self-contained Cargo workspace.

  ```
  spec/databases/            the normative core (§0–§16)          ← shared
  AGENTS.md  agents/         contributor and agent guidance       ← shared
  CLAUDE.md                  pointer to AGENTS.md                 ← shared
  doc/                       tutorials, examples, comparisons     ← shared
  README.md  index.md        entry points                         ← shared

  fhir-<engine>/
    Cargo.toml               workspace: -map, -gen, -store
    crates/fhir-<engine>-map/     model, shred, reconstruct, fold, canon, ddl
    crates/fhir-<engine>-gen/     FHIR spec packages → map + DDL
    crates/fhir-<engine>-store/   the driver, transactions, search, chain
    assets/                  committed generated artifacts + CHECKSUMS.txt
    spec/index.md            port index
    spec/14-<engine>-dialect.md   the port's departures
    book/                    the port's mdBook
    doc/                     benchmarks, ci, containers
    fuzz/                    fuzz targets and seed corpora
    scripts/db.sh            local engine container
    plan.md  tasks.md        decisions and work breakdown
  ```

- **W16.2** Crate names follow `fhir-<engine>-{map,gen,store}`, and the engine
  segment matches the directory. A crate whose name says one engine and whose
  code targets another is the failure this rule exists to catch.
- **W16.3** Every crate's `description` MUST name the engine the crate actually
  targets — a string published to crates.io, shown on docs.rs, and read by
  someone choosing a dependency. The rule's originating defect (six store
  crates calling themselves "PostgreSQL storage layer", **F-02**) is fixed;
  the same rule caught four "SCAFFOLD" descriptions that outlived the
  scaffolds (**F-76**, fixed 2026-08-06).
- **W16.4** A port's workspace MUST declare only the drivers it uses, and every
  dependency comment MUST describe the dependency it sits above. Originating
  defect **F-03**, fixed.

## The single source of truth

- **W16.5** Normative text lives in `/spec` **once**. A port's `spec/` directory
  MUST contain only its own `index.md` and its dialect annex. Copying a core
  section into a port is prohibited, whatever the intention: the six copies of
  §1–§13 that this rule replaces were identical apart from a product name, and
  the risk they carried — an amendment landing in one and not the others — is
  the entire reason for the rule.
- **W16.6** CI MUST verify `X15.1`: normalize the crate-name substitution and
  diff the shared modules across ports; any difference fails the build. Without
  this the shared core is shared by convention, and convention is what produced
  six diverging spec directories. Satisfied: `scripts/check-shared-core.sh`
  runs in `gates.yml` on every push (**F-10** fixed; **F-49** first half
  closed).
- **W16.7** A change to shared code MUST be applied to every port in the same
  commit. A port left behind is not "pending"; it is a divergence that `W16.6`
  will report as a defect.

## Documentation

- **W16.8** Documentation MUST NOT be text-substituted from another port. The
  README, book, and `doc/` of a port describe **that port**: its engine, its
  conformance level (`C0.11`), its measured numbers, and its own limitations.

  The originating defect — every port's README carrying the PostgreSQL
  reference's status paragraph with the engine name swapped, including a
  `serve` command and figures never measured there — was **F-01**, fixed;
  the corpus round-trip has since been run in every port (**F-42**), and the
  search-compilation figure is 92.4% since **F-38** (an earlier revision of
  this section quoted the superseded 94.8%).

- **W16.9** A code example in documentation MUST be runnable against the code as
  shipped. An example that cannot run is worse than none: it costs a reader
  the time to find out. The originating defect (every README documenting a
  `cargo install` of a CLI crate that exists in no workspace, `C0.18`) is
  fixed — READMEs show library usage, and `scripts/check-doc-examples.sh`
  compiles every documented example in CI (**F-60**).
- **W16.10** Measured numbers MUST name what measured them and when. A
  throughput figure inherited by substitution is not a measurement of the port
  that now carries it.

## Versioning and release

- **W16.11** Ports version independently. They currently share `0.4.0`, which is
  a fact about a common ancestor rather than a promise; a fix to one port MUST
  NOT require a version bump in the other five.
- **W16.12** A port's `CHANGELOG.md` MUST describe changes to **that port**. An
  entry inherited from another port's history describes work that was not done
  here.
- **W16.13** Release gates are `O10.10` (supply-chain evidence) and `O10.11`
  (published version matches source), per port.
- **W16.14** A port MUST NOT be published to a registry above its conformance
  level (`C0.8`). Publishing a Scaffold-level port under a name that implies a
  working store is a claim about clinical software made to people who cannot
  check it.
- **W16.15** Each port's git remote MUST match the port. The originating
  defect — all six sharing the ancestor project's `origin` — was resolved
  outright by the monorepo merge (**F-11**): the ports are directories in one
  repository with one remote, and none has a `.git` of its own.

---

Part of the [fhir-databases specification](index.md).
