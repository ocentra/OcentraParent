# Workpack 05: Export Import Backup Recovery

Goal: define parent-controlled portability and recovery.

Context to read:

- `docs/plans/data-custody-storage-plan/BUNDLE_PROTOCOL.md`
- `docs/plans/data-custody-storage-plan/PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
- `docs/plans/data-custody-storage-plan/DECISIONS.md`
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
- Plain JSON dumps of sensitive child or family data.

Acceptance:

- Retrieve, preview, and apply are separate steps.
- Wrong-household, wrong-key, expired-retention, duplicate-device, and corrupt-bundle cases fail closed.
- Partial restore states are explicit.
- Support cannot recover encrypted payloads by default.

Required bundle properties:

- Versioned manifest with schema version, created-at, source, household binding, data classes, proof tier, and retention notes.
- Encrypted payload sections by data class.
- Checksums or signatures for manifest and payload integrity.
- Redacted human summary safe for support and parent review.
- Import preflight that validates version, household binding, key availability, tombstones, duplicates, and migration path before restore.
- Partial restore state when some data classes are rejected or unavailable.

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
- Restore creates duplicate child, device, or policy truth.
- Restore ignores tombstones or retention expiry.
- Support workflow requires Ocentra to possess parent decrypt keys for child evidence by default.

