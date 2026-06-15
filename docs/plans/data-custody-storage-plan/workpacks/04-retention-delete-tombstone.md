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

