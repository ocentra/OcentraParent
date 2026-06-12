# Native Apps Plan � HID Execution Blueprint

## Execution objective

Convert app ownership docs into executable boundaries with idempotent state transitions and family authorization.

## Slice 01 � Domain and Contract Baseline

### Acceptance

- Shared app-domain schemas and contracts are explicit and invalid input is rejected.

### Tests

- `app-plan.contract.schema-negative`
- `app-plan.authz.family-device-boundary`

### Proof

- `docs/proof/app-plan/slice-01-domain-contract.md`

## Slice 02 � Replay and Ordering

### Acceptance

- App actions are safe under duplicate, stale, and out-of-order inputs.

### Tests

- `app-plan.replay.idempotency-ordering`

### Proof

- `docs/proof/app-plan/slice-02-state-ordering.md`

## Slice 03 � Observability and PR Gate

### Acceptance

- Logs/traces cover action denial/allow decisions without sensitive payload leakage.

### Tests

- `app-plan.observability`

### Proof

- `docs/proof/app-plan/slice-03-observability.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/app-plan/workpacks/01-contract-boundary-and-effect-schemas.md
- Slice 02: docs/plans/app-plan/workpacks/02-source-index-and-doc-reconciliation.md
- Slice 03: docs/plans/app-plan/workpacks/03-current-app-snapshot-and-gap-map.md

## PR-ready gate

- No app-plan checkbox flips to checked without a linked proof manifest and negative-case execution.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: domain contract and family boundary assertions
- Integration: app state transitions and replay/idempotency
- E2E: ownership and family flow visibility
- Security: authZ and sensitive-action denial paths
- Non-functional: logging and tracing completeness

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
