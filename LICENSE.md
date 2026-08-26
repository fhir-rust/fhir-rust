# License

```
SPDX-License-Identifier: MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only
```

That expression is the authoritative, machine-readable statement of the terms,
and it is the licence every publishable crate declares (measured 2026-08-26:
all 34 of the repository's 41 `[package]` manifests that publish — 16
verbatim, 18 via `license.workspace = true` from a workspace root carrying
the same string; the seven `publish = false` internal crates declare no
licence field). `OR` means **the recipient chooses**: take any one of the
five, and no obligation from any other one applies to you.

License is any of these or contact us for custom license options.

* [MIT](https://opensource.org/license/mit) ([SPDX: MIT](https://spdx.org/licenses/MIT.html))

* [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([SPDX: Apache-2.0](https://spdx.org/licenses/Apache-2.0.html))

* [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) ([SPDX: GPL-2.0-only](https://spdx.org/licenses/GPL-2.0-only.html))

* [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) ([SPDX: GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html))

* [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) ([SPDX: BSD-Clause](https://spdx.org/licenses/BSD-3-Clause.html))

## Scope

This applies to every crate in the repository — all **four** families: the
model, the persistence core, the six database ports, and the HTTP surface. The
SPDX expression `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR
GPL-3.0-only` is declared identically in all **33** `[package]` manifests, and
`OR` means the recipient chooses; no obligation from one option carries into
another.

The seven `fuzz` crates are `publish = false` and carry no manifest licence;
they are build tooling, not distributed artifacts.

Six ports also carry `LICENSE-APACHE` and `LICENSE-MIT`, the full texts of two
of the five. They are the same grant as above, not a narrower one.

## Already-published versions

Versions already on crates.io keep the licence they were published under —
a published version is immutable, including its metadata. `fhir`, `fhir-core`,
`fhir-derive-macros`, and the release crates under their pre-2026-08-10
names (`fhir-release-1` through `-10` — since renamed `fhir-r1`…`fhir-r10`
in this tree) have releases that predate this harmonization; the terms
above apply from the next published version of each.

## The five texts

Full text of every option is in [`LICENSES/`](LICENSES/), one file per SPDX
identifier — the [REUSE](https://reuse.software/) convention, which exists for
repositories that offer more than one licence:

| | |
| --- | --- |
| [`LICENSES/MIT.txt`](LICENSES/MIT.txt) | MIT |
| [`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt) | Apache License 2.0 |
| [`LICENSES/BSD-3-Clause.txt`](LICENSES/BSD-3-Clause.txt) | BSD 3-Clause |
| [`LICENSES/GPL-2.0-only.txt`](LICENSES/GPL-2.0-only.txt) | GNU GPL v2.0 only |
| [`LICENSES/GPL-3.0-only.txt`](LICENSES/GPL-3.0-only.txt) | GNU GPL v3.0 only |

Added 2026-08-26. Before then this file offered five licences and the repository
shipped the text of none of them at the root — only links. MIT, Apache-2.0 and
BSD-3-Clause each require the licence text to travel with the software, so a URL
was not sufficient.

**The six ports' `LICENSE-APACHE` files are not the Apache licence.** Each
contains the fifteen-line boilerplate notice Apache publishes for *source file
headers*, ending in a pointer to `apache.org`. It is not a copy of the License,
and §4 of Apache-2.0 requires recipients be given one. Use
[`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt) as the authoritative text;
replacing the per-port files is outstanding work.

## The five-way choice is deliberate

**Decided 2026-08-26: the five-way expression stays.** The alternative
considered was narrowing the manifest field to the ecosystem-standard
`MIT OR Apache-2.0` and keeping the wider offer available on request. It was
rejected: the point of the grant is that the recipient chooses, and narrowing it
to suit scanning tools would take a real freedom away from users to buy a
cosmetic improvement in a metadata field.

The known cost of that decision, recorded so nobody has to rediscover it: some
enterprise licence scanners do not evaluate `OR` expressions and will route
anything naming `GPL-2.0-only` or `GPL-3.0-only` to legal review. If that
happens to you, the answer is in the paragraph below — you may take MIT alone,
and doing so is sufficient. `PM-75` in
[`help/outreach/index.md`](help/outreach/index.md) tracks it as a known
adoption friction rather than a defect to fix.

## For automated tooling

`LICENSE.md` is prose, and a licence detector reading only this file will report
the repository as unlicensed rather than as any of the five — which is what
GitHub currently does (`license.spdx_id: "NOASSERTION"`). The authoritative
answers a tool should read are, in order:

1. the `license` field of any publishable `[package]` manifest — the SPDX
   expression above in all 34 of them, 16 verbatim and 18 by
   `license.workspace = true` inheritance;
2. the `SPDX-License-Identifier` line at the top of this file;
3. the full texts in [`LICENSES/`](LICENSES/), one file per SPDX identifier.
   (The six ports also carry `LICENSE-MIT`, the full MIT text; their
   `LICENSE-APACHE` siblings are the header boilerplate described above, not
   the licence — use `LICENSES/Apache-2.0.txt`.)

If your licence policy evaluates SPDX expressions but not `OR` — a common
limitation in enterprise scanners, which may route anything naming
`GPL-2.0-only` or `GPL-3.0-only` to legal review regardless of the disjunction —
note that the grant permits you to take `MIT` alone, and that doing so is
sufficient. Say so to your reviewer and cite this paragraph.
[`help/outreach/index.md`](help/outreach/index.md) `PM-75` records this as a
known adoption friction.

## FHIR® itself

FHIR® is a registered trademark of Health Level Seven International. The
specification material these crates are generated from is HL7®'s, under HL7's
terms; this licence covers the Rust source in this repository, not the standard.
See [`fhir/spec/trademarks.md`](https://github.com/fhir-rust/fhir-rust/blob/main/fhir/spec/trademarks.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
