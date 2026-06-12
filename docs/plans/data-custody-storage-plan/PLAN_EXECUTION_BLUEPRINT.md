# Data Custody Storage Plan — HID Execution Blueprint

## Execution objective

Turn custody promises into concrete schema, deletion, retention, export, and deletion-tombstone behavior.

## Slice 01 — Evidence Schema and Access Contract

### Acceptance

- Evidence schema boundary and data authority are enforced with invalid input rejection.

### Tests

- `data-custody.contract.schema-negative`

### Proof

- `docs/proof/data-custody-storage-plan/slice-01-evidence-schema.md`

## Slice 02 — Retention and Deletion

### Acceptance

- Deletion creates a `deleted` tombstone state with a bounded retention marker (`deleteExpiry`, `evidenceHash`, `policyVersion`) persisted for the configured retention window; delete requests older than window must fail with explicit expiration response while keeping the tombstone discoverable for proof audits.

### Tests

- `data-custody.replay.idempotency-ordering`
- `data-custody.retention-delete-tombstone`

### Proof

- `docs/proof/data-custody-storage-plan/slice-02-retention-delete.md`

## Slice 03 — Export/Sync Integrity

### Acceptance

- Export and sync produce checksummed payloads with ownership boundaries.

### Tests

- `data-custody.sync.integrity`

### Proof

- `docs/proof/data-custody-storage-plan/slice-03-export-sync.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/data-custody-storage-plan/workpacks/01-custody-source-of-truth.md
- Slice 02: docs/plans/data-custody-storage-plan/workpacks/02-encryption-key-custody.md
- Slice 03: docs/plans/data-custody-storage-plan/workpacks/03-parent-owned-cloud-sync.md

## PR-ready gate

- No storage claim without retention/delete evidence and negative-case replay path.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: schema validation and ownership models
- Integration: retention/delete and export sync flows
- E2E: restore/rebuild and ownership handoff
- Security: corruption/replay detection and deletion tamper checks
- Non-functional: checksum and cleanup timing

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
