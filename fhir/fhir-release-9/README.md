# fhir-release-9

Reserved for a future FHIR Release 9 data model.

**There is no FHIR R9 specification.** R6 is the newest release HL7 has
published in any form, and it is still in ballot. R9 is simply the number
that would come next; HL7 has not announced it, and nothing here should be
read as implying otherwise.

This crate holds the name so the model can be published here if such a
release exists, and so the crate family stays contiguous: one crate per
release, all siblings of
[`fhir-core`](https://crates.io/crates/fhir-core). A gap in the sequence is
an invitation for an unrelated crate to occupy a name in the scheme — which
has already happened once, to `fhir-r4`.

It contains no types, deliberately. A placeholder `Patient` would be a guess
about a specification nobody has written, and a wrong guess is worse than an
absent one for anything that touches clinical data.

## What is actually available

For a real FHIR model today, use [`fhir`](https://crates.io/crates/fhir),
which covers DSTU2, STU3, R4 and R5, with R6 available from the repository as
a ballot draft.

## Filling this in

The procedure is `doc/adding-a-release.md` in the repository. It is short,
because the generator already knows how: adding R6 needed a definition
bundle, a `Version` entry, a `KNOWN_VERSIONS` entry, one generator run, and
one hand-written change (`Bundle.link.relation` became a coded value).

## License

MIT. FHIR® is a registered trademark of HL7, used with permission.
