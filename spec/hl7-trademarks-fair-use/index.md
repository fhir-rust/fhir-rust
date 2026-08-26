# HL7® trademarks fair use

Fair Use of HL7 Word Marks: Anyone may use HL7 word marks in fair use ways. Examples of acceptable fair uses of HL7 word mark are provided at http://www.hl7.org/legal/fairuse.cfm. When using HL7 word marks (e.g., "HL7", "FHIR®", "CDA®", etc.) for fair use:

Always include the trademark registration mark® after the first use of word marks each page

Include the following disclaimer on the webpages, material and other locations where such marks are used: "HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their use of these trademarks does not constitute an endorsement by HL7."

Please refer to the Fast Healthcare Interoperability Resources as the "HL7® FHIR® standard". When referencing the HL7® FHIR® standard in a website, document, presentation, or otherwise in a place of prominence, refer to it as the "HL7® FHIR® standard". In subsequent uses, please refer to it as the "HL7® FHIR® standard" or "HL7® FHIR®", using the ® symbol as often as is practical, at least once on each page of printed matter, generally in connection with the first or dominant usage.

Assurance: create automatic tests to verify this works.

**Done 2026-08-26.** `scripts/check-trademarks.sh` verifies the first two rules
on every markdown document at the repository root and under `help/`: the
registration mark follows the first prose use of each word mark, and any page
using a mark carries the disclaimer verbatim. It runs in
`.github/workflows/gates.yml` alongside the shared-core and doc-example gates.

Code, link targets and URLs are masked before checking, so `fhir-sqlite-store`
and `FHIR_POSTGRESQL_BENCH` are correctly not treated as uses of the mark.

The third rule — the "HL7® FHIR® standard" full form in a place of prominence —
is editorial and is applied in `README.md` and `NEWS.md` rather than gated; a
script cannot tell which usage is dominant.

**Also covered since 2026-08-26:** the rustdoc of the nine top-level crate
roots (`fhir`, `fhir-store`, `fhir-loco`, and the six ports' `-store`
crates) — the script extracts doc-comment prose, `er7-rust`'s
`prose_rust_docs` approach, and applies the same two rules to what docs.rs
renders.

**Covered tree-wide since 2026-08-26:** every markdown file in the
repository is in the script's scope and compliant — root, `help/`, `doc/`,
the specs, the four families' own markdown including every port's `book/`
chapters — plus the nine top-level crate roots' rustdoc. The only
exemptions are structural, named in the script with their reasons:
`fhir/fhir.md` (a generated transcript — its ® is the generator's job),
`book/src/SUMMARY.md` files (mdbook navigation manifests, where appended
prose breaks the build), and `.github/` issue templates (a footer there
would inject itself into every filed issue). All seven mdbooks build with
the footers in place.
