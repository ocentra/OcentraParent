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

- Status: production source packet strengthened but incomplete; expected tests, focused execution, proof refresh, external owner mounting, and runtime composition remain open. Scheduled backup and unavailable-provider paths remain manual-required.
- Proof root: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/`
- Canonical owners: `crates/schema` for the shared export/import/restore contract and durable backup/schedule/job/migration/rollback shapes; `crates/storage-custody-core` for pure bundle derivation, import preflight, restore/migration orchestration, fail-closed compensation, and provider-neutral ports; `crates/parent-runtime-core` for durable scheduler/job persistence, restore/migration ledgers, restart reconciliation, executor/rollback mounting, and real Eventing journal/outbox composition.
- TS/shared edge note: no new `packages/schema-domain` surface was needed for WP05. TS ownership was not widened.

## Reviewed source ownership route (2026-08-18)

WP05 is the READY source slice for the remaining production packet. The route
is deliberately split by authority and durability; no crate may mint a
caller-selected Account/family authority, key/decrypt capability, integrity
decision, or provider identity.

### Canonical schema contract owner

- `crates/schema/src/export_import_backup_recovery.rs`
- `crates/schema/src/export_import_backup_recovery/`
- `crates/schema/tests/contract/export_import_backup_recovery.rs`

These shapes own durable backup cadence and schedule identity, backup/job
lifecycle, idempotency and execution references, provider-neutral operation
references, and migration apply/rollback/reconciliation results and receipts
bound to bundle and plan identity. The schema is the source of contract truth;
it does not persist jobs or execute provider/producer work.

### Pure storage-custody-core owner

- `crates/storage-custody-core/src/export_import_backup_recovery.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_build.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import_integrity.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import_logic.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import_rejection.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import_response.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_import_sections.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_restore.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_migration.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_backup_schedule.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_backup_job_state.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_restore_execution_plan.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_restore_execution_plan_validation.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_migration_execution.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding_custody_port.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding_execution.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding_execution_metadata.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding_execution_metadata_identity.rs`
- `crates/storage-custody-core/src/export_import_backup_recovery_compensation.rs`
- `crates/storage-custody-core/tests/unit/export_import_backup_recovery.rs`

These modules remain pure decisions/orchestration. They derive safe plans from
owner-bound opaque inputs, bind bundle/preflight identity, enforce no
resurrection, and describe partial-write compensation; they do not own durable
job/receipt storage, filesystem/provider SDKs, or producer mutation.

### Parent-runtime-core durable owner

- `crates/parent-runtime-core/src/data_custody_backup_runtime.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_persistence.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_schedule.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_schedule_execute.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_job_ledger.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_job_ledger_apply.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_job_ledger_event_apply.rs`
- `crates/parent-runtime-core/src/data_custody_backup_runtime_reconciliation.rs`
- `crates/parent-runtime-core/src/data_custody_runtime_eventing.rs`
- `crates/parent-runtime-core/src/data_custody_runtime_eventing_identity.rs`
- `crates/parent-runtime-core/src/data_custody_runtime_eventing_identity_backup.rs`
- `crates/parent-runtime-core/src/data_custody_runtime_eventing_identity_kind.rs`
- `crates/parent-runtime-core/src/data_custody_runtime_eventing_validation.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_stage.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_recovery.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_dispatch.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_dispatch_apply.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_dispatch_preflight.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_ledger.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_ledger_event_apply.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_ledger_event_stage.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_reconciliation.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_reconciliation_section_partition.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_reconciliation_sections.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_reconciliation_validation.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_executor.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_executor_receipts.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_rollback.rs`
- `crates/parent-runtime-core/src/data_custody_restore_runtime_rollback_dispatch.rs`

This is the missing legal parent-runtime owner. It persists scheduler/job and
restore/migration state, reconciles interrupted work on restart, mounts only
opaque provider/key/Account ports, and composes the real Eventing journal and
outbox seam. It must fail closed when an external authority, key/decrypt
capability, provider-neutral adapter, or producer handoff is unavailable; no
provider SDK, OAuth flow, local filesystem adapter, or authority minting is
added here. The current source keeps the mount constructors and dispatch
ports owner-private until the dependency-owned Account/key/provider composer
supplies a trusted implementation; this is an explicit external mounting
blocker, not an authorization or source-completion claim.

### Expected test roots (deferred)

- `crates/schema/tests/contract/export_import_backup_recovery_runtime.rs`
- `crates/storage-custody-core/tests/unit/export_import_backup_recovery_runtime.rs`
- `crates/parent-runtime-core/tests/unit/data_custody_backup_runtime.rs`
- `crates/parent-runtime-core/tests/unit/data_custody_restore_runtime.rs`
- `crates/parent-runtime-core/tests/integration/data_custody_runtime.rs`

The expected-test wave remains deferred. These paths are routing obligations,
not evidence that tests exist or passed.

## Source-wave checkpoint (2026-08-17)

- `crates/storage-custody-core/src/export_import_backup_recovery_import_integrity.rs` now rejects dishonest import bundles before restore/apply derivation.
- The current source packet now includes the schema-owned lifecycle contracts, manual-required scheduled-cadence gate, parent durable job/restore/migration ledgers, migration-before-restore ordering, and fenced rollback boundary; external provider/custody mounting remains blocked.
- No tests were written or run in this source wave. Integrity, backup, migration, and rollback tests remain deferred to the expected-test wave, and source completion is not claimed.

The 2026-08-18 route correction assigns the durable scheduler, ledgers,
restart reconciliation, executor/rollback mount, and Eventing/outbox
composition to `crates/parent-runtime-core`; `storage-custody-core` remains a
pure decision/orchestration owner. This changes routing only and does not mark
any source, test, proof, or runtime composition complete.

## Candidate source acceptance (tests and proof unresolved)

The following are candidate source outcomes for the current implementation wave;
none is acceptance evidence until the expected test roots, focused validation,
runtime composition, and retained proof artifacts are completed and reviewed:

- Versioned manifest with schema version, created-at, source household/device binding, data classes, proof tier, and retention notes.
- Encrypted payload sections by data class plus manifest/payload integrity refs.
- Redacted human summary safe for support and parent review.
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

- WP09 remains the downstream source-only route for provider-neutral byte
  custody/adapter-port composition after this contract and runtime-owner
  slice; it must not duplicate the parent scheduler/job ledger.
- WP10 remains the downstream source-only route for producer handoffs after
  this contract and runtime-owner slice; it must not duplicate the parent
  restore/migration ledger or fabricate producer receipts.
- Provider adapters remain sibling owners for actual cloud/provider SDK or
  local-folder execution; WP05 supplies only opaque provider-neutral ports.
- Portal surfaces remain sibling owners for preview and confirmation UI only.
- Eventing remains the replay/idempotency spine. Parent-runtime-core composes
  its real journal/outbox seam; WP05 does not re-own event-bus internals.

## No-claim boundary

- No provider adapter runtime claim is made.
- No portal rendering claim is made.
- No LAN claim is made.
- No default support decrypt path is claimed.
