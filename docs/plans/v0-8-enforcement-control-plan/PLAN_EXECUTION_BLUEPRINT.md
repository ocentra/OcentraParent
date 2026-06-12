# V0.8 Enforcement Control Plan � HID Execution Blueprint

## Execution objective

Define enforcement control as a provable authority pipeline: policy input, adapter capability, execution, and rollback.

## Slice 01 � Policy Input and Contract

### Acceptance

- Policy inputs validated, schema-safe, and reject unauthorized commands.

### Tests

- `enforcement.policy-input.schema-negative`

### Proof

- `docs/proof/v0-8-enforcement-control-plan/slice-01-policy-input.md`

## Slice 02 � Adapter Authority Matrix

### Acceptance

- Execution adapters expose capability map and manual-required fallbacks.

### Tests

- `enforcement.authz.privilege-escalation`
- `enforcement.adapter.capability-matrix`

### Proof

- `docs/proof/v0-8-enforcement-control-plan/slice-02-adapter-matrix.md`

## Slice 03 � Replay, Concurrency, and Rollback

### Acceptance

- Concurrent/replay/out-of-order actions are deterministic and reversible.

### Tests

- `enforcement.replay.idempotency-race`
- `enforcement.canary.rollback-validation`

### Proof

- `docs/proof/v0-8-enforcement-control-plan/slice-03-replay-rollback.md`

## Slice 04 � UI/Status and Evidence

### Acceptance

- Child/family visible reasons match enforcement results and include audit trace refs.

### Tests

- `enforcement.no-ai-direct-action`
- `enforcement.ui-status.audit`

### Proof

- `docs/proof/v0-8-enforcement-control-plan/slice-04-ui-audit.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/v0-8-enforcement-control-plan/workpacks/01-contract-boundary-and-effect-schemas.md
- Slice 02: docs/plans/v0-8-enforcement-control-plan/workpacks/02-policy-decision-evidence-references.md
- Slice 03: docs/plans/v0-8-enforcement-control-plan/workpacks/03-adapter-capability-matrix.md
- Slice 04: docs/plans/v0-8-enforcement-control-plan/workpacks/04-owned-process-time-limit.md

## PR-ready gate

- No enforcement claim until privilege boundaries and rollback validation are proven.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: enforcement input/adapter schema checks
- Integration: policy execution + portal state consumption
- E2E: parent-facing status and rollback path
- Security: privilege boundary, replay/race, bypass probes
- Non-functional: canary rollout and observability

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
