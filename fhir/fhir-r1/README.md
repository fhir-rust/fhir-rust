# fhir-r1

Reserved for a FHIR Release 1 (DSTU1, 0.0.82) data model.

**The specification is real; the model is not.** DSTU1 was published in 2012
and superseded by DSTU2 in 2015. A DSTU1 model was built in this workspace
and withdrawn before release — carrying a trial model of a 2012 draft was not
worth its maintenance.

DSTU1 shares less with its successors than any other pair of releases:

| | DSTU1 | DSTU2 onwards |
| --- | --- | --- |
| Collections | Atom feeds | `Bundle` resource |
| Summary view | none (`isSummary` arrived in DSTU2) | `_summary` |
| Medication ordering | `MedicationPrescription` | `MedicationOrder` |
| `OperationOutcome.issue` | `severity`, `details` | adds `code` |

Because there is no `Bundle`, DSTU1 cannot implement the `Release` trait the
REST client is generic over — so a DSTU1 model would ship without a client.

This crate holds the name so the model can be published here if it is ever
wanted, and so the crate family stays contiguous: one crate per release, all
siblings of [`fhir-core`](https://crates.io/crates/fhir-core).

It contains no types, deliberately.

## What is actually available

For a real FHIR model today, use [`fhir`](https://crates.io/crates/fhir),
which covers DSTU2, STU3, R4 and R5, with R6 available from the repository as
a ballot draft. DSTU2 is the closest modelled release to DSTU1, though the
two differ more than any other adjacent pair.

## License

MIT. FHIR® is a registered trademark of HL7, used with permission.
