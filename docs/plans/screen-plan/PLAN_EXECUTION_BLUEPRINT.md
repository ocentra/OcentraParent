# Screen Plan � HID Execution Blueprint

## Execution objective

Make capture/storage/inference path explicit with retention safety and policy handoff proof.

## Slice 01 � Capture Permission and Custody

### Acceptance

- Permission gating and raw artifact custody are explicit and test-covered.

### Tests

- `screen.capture.permission-authn`

### Proof

- `docs/proof/screen-plan/slice-01-capture-custody.md`

## Slice 02 � OCR and Redaction

### Acceptance

- OCR/VLM outputs pass schema checks and do not leak private text.

### Tests

- `screen.ocr.output-invariants`
- `screen.storage.redaction`

### Proof

- `docs/proof/screen-plan/slice-02-ocr-redaction.md`

## Slice 03 � Retention, Deletion, and Read-Model

### Acceptance

- Deletion and retention tombstones are proven in read-model and journal.

### Tests

- `screen.storage.retention-tombstone`
- `screen.read-model.replay-ordering`

### Proof

- `docs/proof/screen-plan/slice-03-retention-journal.md`

## Slice 04 � Integration to Policy and Alerts

### Acceptance

- Handoff to downstream policy/audit is deterministic and non-authoritative from AI layer.

### Tests

- `screen.policy-handoff.state-machine`

### Proof

- `docs/proof/screen-plan/slice-04-policy-handoff.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/screen-plan/workpacks/01-source-index-and-doc-reconciliation.md
- Slice 02: docs/plans/screen-plan/workpacks/02-current-screen-snapshot-and-gap-map.md
- Slice 03: docs/plans/screen-plan/workpacks/03-contract-boundary-and-effect-schemas.md
- Slice 04: docs/plans/screen-plan/workpacks/04-parent-opt-in-settings-contract.md

## PR-ready gate

- No screen claim with unproven deletion or custody behavior.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: capture/session schema checks
- Integration: custody, deletion, and handoff plumbing
- E2E: child-parent visibility and consent branches
- Security: permission, auth lifecycle, replay ordering
- Non-functional: event sequencing and retention timing

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
