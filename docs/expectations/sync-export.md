# Sync And Export Expectations

Sync/export features move family data across boundaries and need privacy discipline.

## Expected Deliverables

- Export contract.
- Encryption boundary.
- Retention policy.
- Import/replay behavior.
- Sync status.
- Conflict model.
- Parent-visible export/delete controls before paid production.

## Acceptance

- Exported data is encrypted or intentionally human-readable with explicit parent action.
- Import validates schema versions.
- Sync failures do not corrupt local evidence.
- Parent can understand what data moved where.
- Delete/retention behavior is explicit.

## Non-Goals

- Do not silently upload raw evidence before cloud privacy decisions are made.
- Do not make sync the only way to preserve local evidence.
- Do not create export formats without versioning.

## Done Signal

The parent can intentionally export, import, or sync scoped data with clear status, schema validation, and no corruption of local evidence.
