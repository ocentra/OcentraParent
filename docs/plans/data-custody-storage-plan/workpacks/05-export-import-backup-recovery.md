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

## Completion

- Status: complete for WP05 only; no broader plan, provider-runtime, or PR readiness claim is made.
- Proof root: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/`
- Canonical owners: `crates/schema` for the shared export/import/restore contract and `crates/storage-custody-core` for bundle derivation, import preflight, and restore/apply state derivation.
- TS/shared edge note: no new `packages/schema-domain` surface was needed for WP05. TS ownership was not widened.

## Required acceptance proved

- Versioned manifest with schema version, created-at, source household/device binding, data classes, proof tier, and retention notes is covered by the Rust contract proof.
- Encrypted payload sections by data class plus manifest/payload integrity refs are covered by the Rust contract and runtime derivation tests.
- Redacted human summary safe for support and parent review is enforced by the runtime builder and covered by contract/runtime tests.
- Import preflight validates version, household binding, key availability, tombstones, duplicates, and migration path before apply.
- Partial restore is explicit with accepted and rejected section lists.
- Wrong-household, wrong-key, corrupt-bundle, expired-retention, duplicate-device, and unsupported migration cases fail closed in the shared Rust preflight state machine.
- Default support recovery of child evidence decryption remains blocked.

## Proof artifacts

- `00-export-bundle-contract-proof.md`
- `01-encrypted-payload-proof.md`
- `02-import-preview-non-mutating-proof.md`
- `03-wrong-household-key-bundle-proof.md`
- `04-tombstone-preserved-proof.md`
- `05-restore-apply-idempotent-proof.md`
- `06-partial-restore-proof.md`
- `16-validation-commands.log`

## Focused validations

- `cargo test -p ocentra-schema --test contract export_import_backup_recovery`
- `cargo test -p ocentra-storage-custody-core export_import_backup_recovery`
- `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/export_import_backup_recovery.rs crates/schema/tests/contract.rs crates/schema/tests/contract/export_import_backup_recovery.rs crates/storage-custody-core/src/lib.rs crates/storage-custody-core/src/export_import_backup_recovery.rs crates/storage-custody-core/tests/unit.rs crates/storage-custody-core/tests/unit/export_import_backup_recovery.rs`

## Adjacent handoffs

- Provider adapters remain sibling owners for actual cloud/local-folder retrieval and upload runtime.
- Portal surfaces remain sibling owners for preview and confirmation UI only.
- Eventing remains the replay/idempotency spine; WP05 consumes tombstone ordering without re-owning the event bus.

## No-claim boundary

- No provider adapter runtime claim is made.
- No portal rendering claim is made.
- No LAN claim is made.
- No default support decrypt path is claimed.
