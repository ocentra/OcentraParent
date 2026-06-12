# Tracking Plan � HID Execution Blueprint

## Execution objective

Convert tracking claims into reproducible location/session/geofence behavior with strict ordering, privacy, and alert control.

## Slice 01 � Location and Session Contracts

### Acceptance

- Location/session schema boundaries and family/device authorization are explicit.

### Tests

- `tracking.location.schema-negative-decode`
- `tracking.authz.family-isolation`

### Proof

- `docs/proof/tracking-plan/slice-01-location-contract.md`

## Slice 02 � Platform Adapters and Power/Permissions

### Acceptance

- Adapter capability, permission, and degraded modes are captured.

### Tests

- `tracking.adapter.platform`
- `tracking.platform.permission`

### Proof

- `docs/proof/tracking-plan/slice-02-adapter-matrix.md`

## Slice 03 � Geofence and Session Invariants

### Acceptance

- Geofence transitions handle jitter, duplicates, stale samples, and stale transitions.

### Tests

- `tracking.geofence.transition-invariants`
- `tracking.session.idempotency-replay`

### Proof

- `docs/proof/tracking-plan/slice-03-geofence-invariants.md`

## Slice 04 � Alerts, Escalation, and Rollback

### Acceptance

- Alert severity/rollover/rate-limit and rollback behavior are explicit.

### Tests

- `tracking.alert.rate-limit-escalation`
- `tracking.alert.acknowledge-rollback`

### Proof

- `docs/proof/tracking-plan/slice-04-alert-escalation.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/tracking-plan/workpacks/01-source-index-and-repo-reconciliation.md
- Slice 02: docs/plans/tracking-plan/workpacks/02-current-tracking-snapshot-and-gap-map.md
- Slice 03: docs/plans/tracking-plan/workpacks/03-contract-boundary-and-effect-schemas.md
- Slice 04: docs/plans/tracking-plan/workpacks/04-location-evidence-model.md

## PR-ready gate

- No location/geofence claim without replay/order/privacy and platform limitation proof.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: tracking schema and location contracts
- Integration: adapter, permission, and policy transitions
- E2E: consent, geofence, and alert flows
- Security: geofence/replay/role isolation and escalation
- Non-functional: ordering, rollback, and canary checks

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
