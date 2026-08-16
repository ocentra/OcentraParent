<!-- agent-capsule -->

> Agent Capsule
> Doc: Sync And Export Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Sync And Export Expectations

Sync/export features move family data across boundaries and need privacy
discipline. Ocentra-hosted infrastructure is not the default sync destination.
The normal remote-continuity target is parent-owned storage such as Google Drive,
OneDrive, iCloud Drive, Dropbox, a NAS, or a local folder chosen by the parent.

## Parent Outcome

A parent can intentionally move scoped family data for backup, migration,
support, or remote continuity and can understand what moved, when it moved, where
it went, who owns the destination, and how to delete or retain it.

## Child-Device Outcome

The child-device agent remains the source of truth for local evidence. Sync and export read from validated journal/query data, preserve schema versions, and never corrupt or replace local evidence when a transfer fails.

## Platform Scope

- Windows proves the first local export and sync queue behavior.
- Parent-owned storage sync is allowed only after connector identity, storage
  scope, encryption, retention, and conflict boundaries are explicit.
- Ocentra-hosted cloud may coordinate account/auth/subscription, connector
  status, and stateless report compilation, but it must not become the default
  family-data store.
- Android, iOS, macOS, and Linux can claim sync/export support only for the data classes and storage permissions they actually implement.
- Web surfaces initiate parent-authorized export, import, delete, or sync operations through typed service/cloud contracts; web code does not read local evidence files directly.

## Data Scope

Sync/export must distinguish raw encrypted journal segments, derived SQLite query
rows, parent rules, approval decisions, device registry entries, notification
history, audit events, and generated summaries. Each export format must declare
whether it is encrypted machine-readable data, encrypted support bundle, or
intentionally human-readable parent report. Each sync target must declare whether
it is child-local, parent-device local, parent-owned external storage, provider
notification state, or Ocentra-hosted non-activity metadata.

## Trust Boundary

Exports require explicit parent action or a preconfigured sync policy.
Cross-device sync requires authenticated family and device identity.
Parent-owned storage connectors require least-privilege scopes, visible provider
account/folder status, and revocation behavior. Support bundles must minimize
sensitive details by default and make any included child activity evidence
obvious before export. Delete and retention behavior must be tied to the same
family/device/storage scope as the data.

## Contract Boundary

Expected contracts include export manifest, export item descriptor, encryption
metadata, schema version, retention policy, storage provider reference, sync
cursor, sync batch, conflict record, report compile request, report compile
result, import result, delete request, delete result, and audit event. Importers
must validate schema versions before applying data. Remote sync and report
compilation must reuse the cloud route, storage connector, and identity contracts
rather than introducing a second trust model.

Current endpoint proof: `sync-export-endpoint-contract-proof` defines
versioned endpoint-domain route ids, API paths, headers, query params, and
contract-version labels for parent-owned export manifest/status, sync cursor,
sync batch status, import preview, delete status, and remote connector status
boundaries. It is contract-only proof and does not implement connector OAuth,
upload/download, Ocentra-hosted family data custody, account/subscription
backend, or portal UI.

Current parent-domain proof:
`scripts/test/parent-owned-sync-export-manifest-proof.mjs` validates a
parent-owned sync/export manifest read model covering data classes, export
formats, encryption metadata, retention/delete policy, connector status, sync
cursor state, conflict records, import results, and delete results. This is a
contract proof only: it does not implement transfer runtime, connector OAuth,
provider API calls, portal UI, report compiler runtime, account/subscription
backend, raw child evidence upload by default, or Ocentra-hosted child evidence
custody.

Current local export/delete runtime proof:
`scripts/test/parent-owned-local-export-runtime-proof.mjs` validates a
parent-domain local runtime read model for parent-authorized Windows export and
delete jobs. It covers export queued/running/written, delete
requested/confirmed/failed, offline queued, and manual-required states; scoped
data classes; encrypted local output metadata; delete confirmation; source
evidence retention for local safety; support-safe audit refs; and failure
behavior that does not mutate local evidence or parent-owned output. This proof
does not implement cloud transfer, connector OAuth, provider API calls, portal
UI, remote report compilation, child-device mutation, raw evidence upload, or
Ocentra-hosted family-data custody.

Current stateless report compiler proof:
`scripts/test/stateless-report-compiler-status-proof.mjs` validates a
parent-domain request/status/result read model for parent-authorized report
compilation from parent-owned storage. It covers request id, family/account/
device scope, source connector and cursor refs, requested data classes and time
window, parent-owned output destination, queued/running/succeeded/failed/
expired/manual-required lifecycle states, temporary input/output TTL and
deletion confirmation, redaction/minimization flags, audit refs, and failure
behavior that does not mutate local evidence or parent-owned storage. This is a
contract proof only: it does not implement report compiler runtime, cloud
worker, connector OAuth/provider API, upload/download, portal UI,
account/subscription backend, retained temp child evidence, child-device
mutation, or Ocentra-hosted family-data custody.

## Failure Behavior

- Failed sync or export leaves local journal and query store intact.
- Partial uploads are retryable, resumable, or explicitly abandoned without ambiguous state.
- Import failures report the exact rejected schema/version/scope and do not partially apply untrusted data.
- Conflict handling is deterministic and parent-visible when parent-owned settings differ.
- Delete failures report the affected data class and leave an audit record.
- External storage outage or revoked access leaves local safety behavior intact
  and reports connector status to the parent.
- Ocentra report compiler failure leaves source data and parent-owned storage
  unchanged.

## Expected Deliverables

- Export contract.
- Encryption boundary.
- Retention policy.
- Import/replay behavior.
- Sync status.
- Parent-owned storage connector status.
- Conflict model.
- Stateless report compilation status where remote reporting exists.
- Parent-visible export/delete controls before paid production.
- Import validation and replay behavior.
- Sync cursor and retry queue.
- Audit trail for export, import, sync, retention, and delete actions.

## Acceptance

- Exported data is encrypted or intentionally human-readable with explicit parent action.
- Import validates schema versions.
- Sync failures do not corrupt local evidence.
- Parent can understand what data moved where.
- Parent can distinguish local device data, parent-owned storage data, and
  Ocentra-hosted non-activity metadata.
- Delete/retention behavior is explicit.
- Raw evidence is not silently uploaded anywhere, including parent-owned storage,
  before parent sync settings and privacy boundaries are explicit.
- Ocentra-hosted infrastructure does not retain synced child evidence or
  generated reports by default.
- Query stores remain rebuildable from journal data after import or restore.
- Parent rules and approval decisions resolve conflicts without silently overwriting newer local state.
- Exported files declare product version, schema version, family/device scope, and data classes.

## Validation Gates

- Contract tests for manifests, data classes, encryption metadata, sync cursor, conflict records, and import/delete results.
- Endpoint-domain contract tests and `sync-export-endpoint-contract-proof` for
  parent-owned sync/export route/header/query/version boundaries.
- Parent-domain contract tests and
  `parent-owned-sync-export-manifest-proof` for manifest data classes,
  encryption metadata, connector status, sync cursor, conflict, import/delete,
  retention/delete, and no-default-Ocentra-custody boundaries.
- Parent-domain contract tests and `parent-owned-local-export-runtime-proof`
  for local export queue/write/delete/failure/offline/manual states, encrypted
  local output metadata, delete confirmation, source evidence retention for
  local safety, and no cloud/provider/UI/custody overclaims.
- Parent-domain contract tests and
  `stateless-report-compiler-status-proof` for parent-authorized compiler
  request scope, source connector/cursor refs, requested data classes/time
  window, output destination ownership, queued/running/succeeded/failed/
  expired/manual-required states, temp TTL/deletion confirmation,
  redaction/minimization flags, audit refs, and non-mutating failure behavior.
- Real export/import tests using the encrypted journal and SQLite rebuild path.
- Retry/conflict tests covering interruption, duplicate batch, stale cursor, and newer local state.
- Parent-owned storage connector tests for revoked grants, wrong account/folder,
  partial upload, restore, delete, and retry.
- Portal or CLI smoke for parent-visible export status, sync status, and delete/retention controls when those surfaces exist.
- Security/static-analysis review because export, sync, deletion, and retention move sensitive family data.

## Non-Goals

- Do not silently upload raw evidence before parent-owned storage, privacy, and
  data-custody decisions are made.
- Do not make sync the only way to preserve local evidence.
- Do not create export formats without versioning.
- Do not use Ocentra-hosted storage as the default sync destination for family
  activity data.
- Do not retain generated reports in Ocentra systems unless a future explicit
  retention feature is designed and approved.

## Done Signal

The parent can intentionally export, import, sync, or compile reports from scoped
data with clear status, schema validation, explicit destination ownership, and no
corruption of local evidence or default Ocentra custody of family activity data.
