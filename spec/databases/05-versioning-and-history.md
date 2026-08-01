# 5. Versioning and history

- **H5.1** Every create/update/delete increments `version_id` and appends one
  row to `<resource>_history(id, version_id, last_updated, op, resource)` where
  `op` ∈ C/U/D and `resource` is a `Jsonb` column (`M3.6c`). History is an
  immutable audit archive; a whole-resource snapshot is acceptable there because
  it is written once and read only by vread, history, and audit.

  This is the sanctioned exception to fully relational storage: normalizing
  every historical version would multiply the hardest part of the system for no
  query benefit. The "not merely JSON" constraint governs live queryable data,
  which stays fully relational.

- **H5.2** Delete is soft at the interface level (history row with `op = D`;
  base and child rows removed); a deleted id's history remains readable.
- **H5.3** vread serves any historical version from history; read serves the
  current version reconstructed from the relational tables. A checksum
  comparison between the two paths is part of the test suite, not runtime.
- **H5.4** `version_id` MUST be assigned under a lock that serializes writers
  for a given resource id, so two concurrent writers cannot be handed the same
  version. The lock is the same one the chain append relies on (`M3.16`): the
  digest of version *n* commits to the digest of version *n−1*, so a race that
  interleaved two appends would produce two rows claiming the same predecessor
  and a chain that verifies for neither.

---

Part of the [fhir-databases specification](index.md).
