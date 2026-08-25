# Workpack 04: Retention Delete Tombstone

Goal: define retention and deletion as a cross-device protocol, not a UI checkbox.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/plans/data-custody-storage-plan/EVENT_MODEL.md`
- `docs/expectations/evidence-storage.md`
- `docs/features/evidence-store-query.md`
- `docs/features/screen-visibility-live-view.md`

In scope:

- Retention classes by data type.
- Delete request, tombstone, propagation, replay, audit, and recovery.
- Remote and offline devices and provider sync after delete.
- Minimal audit references versus hard delete versus redaction.
- Delete semantics across reports, exports, sync bundles, local caches, parent cache, and relay metadata.

Out of scope:

- Capture adapter details.
- Payment-provider records that must be retained externally.
- Hiding a row in UI without storage, query, or sync proof.

Acceptance:

- Delete creates a tombstone state before propagation.
- Replay protection blocks resurrection from old sync or import flows.
- Expired delete requests fail with explicit expiration handling.
- Audit retention stays minimal and does not leak deleted payloads.

Required delete states:

- `deleteRequested`
- `deleteValidated`
- `tombstoneWritten`
- `localRedacted`
- `propagationPending`
- `propagated`
- `replayProtected`
- `auditRetained`
- `hardDeleted`

Expected artifacts:

- Retention matrix.
- Delete and tombstone state machine.
- Audit and export impact.
- Adjacent-plan update list.
- Data-class table for config, account metadata, policy, evidence, logs, screenshots, network artifacts, AI outputs, reports, notifications, and billing references.

Expected proof names:

- `data-custody.delete.retention-matrix`
- `data-custody.delete.tombstone-state-machine`
- `data-custody.delete.local-redaction-hard-delete`
- `data-custody.delete.offline-replay-protection`
- `data-custody.delete.report-export-no-leak`
- `data-custody.delete.wrong-role-denied`
- `data-custody.delete.retention-expiry-boundary`

Failure conditions:

- Delete only hides UI rows.
- Offline child or cloud sync resurrects deleted evidence.
- Report, notification, assistant answer, backup, or export leaks deleted or expired evidence.
- Audit references retain sensitive payload when only minimal audit should remain.

## Completion

- Status: production source accepted for the shared WP04 state machine and child-runtime custody execution boundary; current expected tests, focused execution, and proof refresh remain open.
- Proof root: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/`
- Canonical owners: `crates/schema` for the shared retention/delete contract, `crates/storage-custody-core` for generic delete/tombstone state derivation, and `crates/child-runtime` for the durable child-side tombstone/effect store and execution lifecycle.
- TS/shared edge note: no new `packages/schema-domain` surface was needed for WP04. TS ownership was not widened.

## Source-wave checkpoint (2026-08-17)

- The durable tombstone store moved from `storage-custody-core` to its actual child-runtime owner, with symlink rejection, atomic intent/terminal mutation, replay protection, and effect-ledger reconciliation kept together.
- The old `crates/storage-custody-core/tests/unit/retention_delete_tombstone_store.rs` owner is stale and must be migrated or rewritten in the expected-test wave; the deleted core module must not be restored as a re-export.
- No tests were written or run in this source wave, so the recorded proof/validation list below is historical rather than acceptance of the moved implementation.

## Required states proved

- `deleteRequested`, `deleteValidated`, and `tombstoneWritten` are covered by the Rust derivation/state-machine tests.
- `localRedacted`, `propagationPending`, and `propagated` are covered by the runtime redaction/propagation tests.
- `replayProtected`, `auditRetained`, and `hardDeleted` are covered by the replay-protection, minimal-audit, and hard-delete-chain tests.

## Proof artifacts

- `00-retention-matrix-proof.md`
- `01-delete-state-machine-proof.md`
- `02-tombstone-idempotency-proof.md`
- `03-offline-retry-proof.md`
- `04-derived-output-boundary-proof.md`
- `05-wrong-role-denied-proof.md`
- `06-retention-expiry-boundary-proof.md`
- `07-restore-cannot-revive-deleted-state-proof.md`
- `16-validation-commands.log`

## Focused validations

- `cargo test -p ocentra-schema --test contract retention_delete_tombstone`
- `cargo test -p ocentra-storage-custody-core retention_delete_tombstone`
- `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/retention_delete_tombstone.rs crates/schema/tests/contract.rs crates/schema/tests/contract/retention_delete_tombstone.rs crates/storage-custody-core/src/lib.rs crates/storage-custody-core/src/retention_delete_tombstone.rs crates/storage-custody-core/tests/unit.rs crates/storage-custody-core/tests/unit/retention_delete_tombstone.rs`

## Adjacent handoffs

- Eventing remains the replay/journal spine owner; WP04 proves replay-safe delete semantics at the shared Rust state-machine layer without re-owning the event bus.
- Report, notification, assistant, export, and sync producers remain sibling consumers of the delete boundary; this workpack proves shared retention/delete truth and leak-blocking flags only.

## No-claim boundary

- No provider delete execution claim is made.
- No portal rendering claim is made.
- No LAN claim is made.
- No restore/apply completion claim is made outside the WP04 resurrection-block boundary.
