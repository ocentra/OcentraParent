# Workpack 04: Retention Delete Tombstone

Goal: define retention and deletion as a cross-device protocol, not a UI checkbox.

Context to read:

- `docs/expectations/evidence-storage.md`
- `docs/expectations/data-custody.md`
- `docs/features/evidence-store-query.md`
- `docs/features/screen-visibility-live-view.md`

In scope:

- Retention classes by data type.
- Delete request, tombstone, propagation, replay, audit, and recovery.
- Remote/offline devices and provider sync after delete.
- Legal/support exceptions if any.
- Redaction versus hard delete versus minimal audit references.
- Delete semantics across reports, exports, sync bundles, local caches, parent cache, and remote relay metadata.

Out of scope:

- Domain-specific capture adapter details.
- Payment provider records that must be retained externally by Stripe.
- Hiding a row in UI without storage/query/sync proof.

Decision tree:

| If the assignment touches...      | Route                                                                |
| --------------------------------- | -------------------------------------------------------------------- |
| Evidence capture domain retention | owning domain plan plus this workpack for custody rules              |
| Provider sync/delete propagation  | WP03 parent-owned cloud sync                                         |
| Export/import after delete        | WP05 export/import backup recovery                                   |
| Reports/query after delete        | WP06 report/query custody                                            |
| Billing/account/legal records     | payment/account plans; do not delete external statutory records here |

Required delete states:

- `deleteRequested`: authorized parent action or retention policy requests deletion.
- `deleteValidated`: role, household, device, and data class allow deletion.
- `tombstoneWritten`: local deletion marker written before propagation.
- `localRedacted`: sensitive payload removed or redacted locally.
- `propagationPending`: offline device/provider/relay still needs tombstone.
- `propagated`: known stores acknowledged tombstone.
- `replayProtected`: old sync/import/replay cannot resurrect deleted data.
- `auditRetained`: minimal allowed audit ref remains with no sensitive payload.
- `hardDeleted`: payload is gone and cannot be restored by normal app flows.

Decisions required:

- Default retention per data category.
- Parent override limits.
- Tombstone lifetime and replay rules.
- What can be hard-deleted versus redacted versus retained as minimal audit.
- Which data types are never remotely synced, never exported, or never included in notifications.

Expected artifacts:

- Retention matrix.
- Delete/tombstone state machine.
- Audit and export impact.
- Adjacent-plan update list.
- Data-class table for config, account metadata, policy, evidence, logs, screenshots, network artifacts, AI outputs, reports, notifications, and billing references.

Expected proof:

- Delete propagation across local store, sync bundle, report/query surfaces, and offline replay.
- Tombstone idempotency and ordering.
- Deleted data no longer appears in reports or exports except allowed audit references.
- Retention expiry boundary proof.
- Wrong-role delete denial proof.
- Backup/import cannot resurrect deleted payload proof.

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
- Report, notification, assistant answer, backup, or export leaks deleted/expired evidence.
- Audit references retain sensitive payload when only minimal audit should remain.
