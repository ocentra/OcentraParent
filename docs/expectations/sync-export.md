# Sync And Export Expectations

Sync/export features move family data across boundaries and need privacy discipline.

## Parent Outcome

A parent can intentionally move scoped family data for backup, migration, support, or remote continuity and can understand what moved, when it moved, where it went, and how to delete or retain it.

## Child-Device Outcome

The child-device agent remains the source of truth for local evidence. Sync and export read from validated journal/query data, preserve schema versions, and never corrupt or replace local evidence when a transfer fails.

## Platform Scope

- Windows proves the first local export and sync queue behavior.
- Cloud sync is allowed only after the cloud identity and relay boundaries are explicit.
- Android, iOS, macOS, and Linux can claim sync/export support only for the data classes and storage permissions they actually implement.
- Web surfaces initiate parent-authorized export, import, delete, or sync operations through typed service/cloud contracts; web code does not read local evidence files directly.

## Data Scope

Sync/export must distinguish raw encrypted journal segments, derived SQLite query rows, parent rules, approval decisions, device registry entries, notification history, audit events, and generated summaries. Each export format must declare whether it is encrypted machine-readable data, encrypted support bundle, or intentionally human-readable parent report.

## Trust Boundary

Exports require explicit parent action or a preconfigured sync policy. Cross-device sync requires authenticated family and device identity. Support bundles must minimize sensitive details by default and make any included child activity evidence obvious before export. Delete and retention behavior must be tied to the same family/device scope as the data.

## Contract Boundary

Expected contracts include export manifest, export item descriptor, encryption metadata, schema version, retention policy, sync cursor, sync batch, conflict record, import result, delete request, delete result, and audit event. Importers must validate schema versions before applying data. Cloud sync must reuse the cloud route and identity contracts rather than introducing a second trust model.

## Failure Behavior

- Failed sync or export leaves local journal and query store intact.
- Partial uploads are retryable, resumable, or explicitly abandoned without ambiguous state.
- Import failures report the exact rejected schema/version/scope and do not partially apply untrusted data.
- Conflict handling is deterministic and parent-visible when parent-owned settings differ.
- Delete failures report the affected data class and leave an audit record.

## Expected Deliverables

- Export contract.
- Encryption boundary.
- Retention policy.
- Import/replay behavior.
- Sync status.
- Conflict model.
- Parent-visible export/delete controls before paid production.
- Import validation and replay behavior.
- Sync cursor and retry queue.
- Audit trail for export, import, sync, retention, and delete actions.

## Acceptance

- Exported data is encrypted or intentionally human-readable with explicit parent action.
- Import validates schema versions.
- Sync failures do not corrupt local evidence.
- Parent can understand what data moved where.
- Delete/retention behavior is explicit.
- Raw evidence is not silently uploaded before privacy and cloud boundaries are approved.
- Query stores remain rebuildable from journal data after import or restore.
- Parent rules and approval decisions resolve conflicts without silently overwriting newer local state.
- Exported files declare product version, schema version, family/device scope, and data classes.

## Validation Gates

- Contract tests for manifests, data classes, encryption metadata, sync cursor, conflict records, and import/delete results.
- Real export/import tests using the encrypted journal and SQLite rebuild path.
- Retry/conflict tests covering interruption, duplicate batch, stale cursor, and newer local state.
- Portal or CLI smoke for parent-visible export status, sync status, and delete/retention controls when those surfaces exist.
- Security/static-analysis review because export, sync, deletion, and retention move sensitive family data.

## Non-Goals

- Do not silently upload raw evidence before cloud privacy decisions are made.
- Do not make sync the only way to preserve local evidence.
- Do not create export formats without versioning.

## Done Signal

The parent can intentionally export, import, or sync scoped data with clear status, schema validation, and no corruption of local evidence.
