# 9. Validation

- **V9.1** Structural validation (element existence, cardinality, primitive
  lexical rules, choice exclusivity, required bindings) always runs — it is
  inherent to shredding against the map.
- **V9.2** **[service]** A strict mode additionally deserializes through the
  typed [`fhir`](https://crates.io/crates/fhir) crate model for the resource's
  version and rejects on any mismatch. In this monorepo the mode is a library
  option; the header and configuration flag this requirement historically named
  belong to a service that does not exist here (`C0.17`).
- **V9.3** **[service]** Validation failure at a service interface returns 422
  with an OperationOutcome listing each issue with a FHIRPath-style location. At
  the library interface the equivalent obligation is that the error names the
  failing path (`R4.3`), not merely the resource.
- **V9.4** Terminology validation is **out of scope** (`S1`). A `required`
  binding is enforced as the literal set of codes the generator extracted
  (`M3.7`); no code system is expanded, no subsumption is computed, and no
  `$validate-code` is performed. A deployment that needs those must place a
  terminology service in front. Stating this is the requirement: an unstated gap
  in validation reads, to an auditor, as a validation claim.

---

Part of the [fhir-databases specification](index.md).
