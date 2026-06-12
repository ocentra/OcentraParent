# Workpack 05: Export Import Backup Recovery

Goal: define parent-controlled portability and recovery.

Context to read:

- `docs/expectations/sync-export.md`
- `docs/expectations/data-custody.md`
- `docs/expectations/platform-deliverables.md`

In scope:

- Export bundle contents, encryption, manifest, version, checksum, provenance, and redaction.
- Import validation, schema migration, conflict, duplicate, and partial restore.
- Backup cadence, manual backup, restore, and corruption recovery.
- Parent-readable summary without exposing sensitive raw payload by accident.
- Household binding, device binding, key verification, provider-neutral portability, and support recovery boundaries.

Out of scope:

- Cloud provider adapter implementation.
- UI styling.
- Plain JSON dumps of sensitive child/family data.

Decision tree:

| If the assignment touches...   | Route                                                       |
| ------------------------------ | ----------------------------------------------------------- |
| Export bundle data classes     | WP01 custody source of truth and WP04 retention rules       |
| Encryption/key handling        | WP02 encryption/key custody                                 |
| Cloud backup provider          | WP03 parent-owned cloud sync                                |
| Restore into account/household | account-identity-family-plan for household/device authority |
| Report export                  | WP06 report/query custody                                   |

Required bundle properties:

- Versioned manifest with schema version, created-at, source, household binding, data classes, proof tier, and retention notes.
- Encrypted payload sections by data class; sensitive sections are not readable without the parent-held key path.
- Checksums or signatures for manifest/payload integrity.
- Redacted human summary that is safe for support and parent review.
- Import preflight that validates version, household binding, key availability, tombstones, duplicates, and migration path before restore.
- Partial restore state when some data classes are rejected or unavailable.

Decisions required:

- Bundle shape and versioning.
- Migration and rollback expectations.
- Import authority and household binding.
- Human support recovery limits.
- Whether support can ever help recover encrypted payloads; default should be no unless parent-owned keys permit it.

Expected artifacts:

- Export/import contract.
- Backup/restore state machine.
- Corruption recovery proof plan.
- Schema migration expectations.
- Wrong-household, wrong-key, expired-retention, duplicate-device, and partial-restore handling matrix.

Expected proof:

- Export decrypt/verify/restore proof.
- Corrupt bundle rejection.
- Wrong household import rejection.
- Migration/backward compatibility proof.
- Backup cadence and rollback proof.
- Redacted summary proof.
- Tombstone preservation proof.

Expected proof names:

- `data-custody.export.bundle-contract`
- `data-custody.export.encrypted-payload-proof`
- `data-custody.import.verify-restore`
- `data-custody.import.corrupt-bundle-negative`
- `data-custody.import.wrong-household-negative`
- `data-custody.import.migration-rollback`
- `data-custody.backup.partial-restore`
- `data-custody.export.redacted-summary-proof`

Failure conditions:

- Export cannot be imported.
- Export is readable by anyone who obtains the file.
- Restore creates duplicate child/device/policy truth.
- Restore ignores tombstones or retention expiry.
- Support workflow requires Ocentra to possess parent decrypt keys for child evidence by default.
