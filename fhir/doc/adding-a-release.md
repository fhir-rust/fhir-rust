# Adding a FHIR® release

Each FHIR release is its own crate: `fhir-r2`, `fhir-r3`, `fhir-r4`, `fhir-r5`,
`fhir-r6`, all siblings of `fhir-core` and re-exported by the `fhir` facade
behind a feature. Adding one is mostly mechanical. This is the procedure that
produced `fhir-r6`, written down while it was fresh.

Budget roughly an hour, most of it waiting for the generator and the test
suite.

## 1. Get the definitions

The generator reads the official JSON bundles:

```sh
mkdir -p doc/fhir-specifications/rN/fhir-definitions-json
curl -sL https://hl7.org/fhir/<VERSION>/definitions.json.zip -o /tmp/defs.zip
unzip -qo /tmp/defs.zip -d doc/fhir-specifications/rN/fhir-definitions-json
```

Check `version.info` in the unpacked bundle and use the `FhirVersion` value
it reports verbatim. For a ballot that is something like `6.0.0-ballot3`, not
`6.0.0`. This string reaches `CapabilityStatement.fhirVersion`, so rounding
it to a release number would misstate what a server speaks.

These bundles are committed — around 70–90 MB per release. That is
deliberate: builds and installs must never require fetching a specification.

## 2. Teach the generator

In `src/codegen/version.rs`, add the variant and extend five things:
`ALL`, `parse`, `module`, `label`, `version_string`, `spec_url`. The compiler
finds every match arm you miss.

In `fhir-derive-macros/src/lib.rs`, add the token to `KNOWN_VERSIONS`.
Forgetting this fails late and confusingly, with `unknown FHIR version` on
every generated type.

> **Known trap.** `version_path` in that file still defaults to `crate::r5`
> when `#[fhir_version]` is absent — a leftover from when R5 was the only
> release. Generated code always carries the attribute, so the default is
> invisible until hand-written code omits it. If a hand-written test in a new
> release crate fails with *cannot find `r5` in `crate`*, that is this.

## 3. Generate

```sh
cargo run --release -- rN     # writes fhir-rN/src
```

This emits the model only: `types/`, `resources/`, `codes.rs`,
`extension_ext.rs`, `meta/generated.rs`, and the two module files. Roughly
200k lines.

## 4. Add the support modules

Twelve modules are per-release and hand-maintained, not generated:

```
builder  bundle_util  choice  client  coded  lib
meta     prelude      summary temporal validate xml
```

Copy them from the closest existing release and rewrite the release token.
**Use a real regex, not `sed`** — BSD `sed` on macOS does not support `\b`,
so `s/\br5\b/r6/g` silently changes nothing while the command still succeeds.

Base `lib.rs` on a release with the same module set. R5 carries extra
hand-written modules (`abstract_types`, `properties`, `resource`, `todo`)
that other releases do not, so copying R5's `lib.rs` declares modules whose
files do not exist.

Each release crate's `lib.rs` needs:

```rust
pub use crate as rN;                          // keeps `crate::rN::…` resolving
impl ::fhir_core::release::Release for RN { … }
```

The self-alias is what lets thousands of generated `crate::rN::…` paths work
unchanged inside a crate that *is* `rN`, and lets the derive macros keep
emitting `crate::rN` without knowing about the split.

## 5. Adapt what the release actually changed

This is the only step needing judgement, and it is small. R5 → R6 was one
function: `Bundle.link.relation` became `Coded<IanaLinkRelations>` where it
had been a bare string, so `Release::next_link` no longer compiled.

Match both `Known` and `Unknown` when comparing coded values. `Coded` exists
so a code outside the value set still round-trips; a paging link should not
become unfollowable because a build did not parse it as `Known`.

## 6. Wire it up

- `Cargo.toml`: add the workspace member, the optional dependency, and the
  feature (`rN = ["dep:fhir-rN"]`).
- `src/lib.rs`: `#[cfg(feature = "rN")] pub use ::fhir_rN as rN;`
- Ballot releases: `publish = false` on the crate, feature **off** by
  default, and say plainly in the crate docs that the model tracks a draft
  and is outside the semver promise.

## 7. Check it

```sh
cargo test -p fhir-rN
cargo build --all-targets --features "r3 r4 rN xml client"
cargo clippy --all-targets --features "r3 r4 rN xml client" -- -D warnings
```

Paths built from strings are the ones that break. The compiler verifies
module paths exhaustively and verifies nothing about `.join("src").join("r5")`
— so after moving or adding a release, grep for hardcoded paths in the
generator, in `src/r5/parse/`, and in `tests/`.

## Reserved names

`fhir-r7`, `fhir-r8` and `fhir-r9` exist as name reservations only. There is
no R7, R8 or R9 specification; R6 is the newest release HL7® has published in
any form, and it is still in ballot. Those crates hold their names and this
procedure, and contain no model — a placeholder type would be a guess about a
specification nobody has written.

They are at `0.0.0`, are not dependencies of `fhir`, and enable no feature.
When a specification arrives, follow the steps above and give the crate a
real version; nothing about the reservation needs undoing first.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
