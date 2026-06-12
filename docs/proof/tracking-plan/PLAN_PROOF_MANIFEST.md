# Proof Manifest - tracking-plan

- plan route: docs/plans/tracking-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/tracking-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/tracking-plan/slice-01-location-contract.md
- docs/proof/tracking-plan/slice-02-adapter-matrix.md
- docs/proof/tracking-plan/slice-03-geofence-invariants.md
- docs/proof/tracking-plan/slice-04-alert-escalation.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: consent, geofence, and alert flows
- Integration: adapter, permission, and policy transitions
- Non-functional: ordering, rollback, and canary checks
- Security: geofence/replay/role isolation and escalation
- Unit: tracking schema and location contracts

## Run log template

- date: YYYY-MM-DD
- command: ...
- test-set: unit/integration/e2e/security
- result: pass | fail
- failure: ... and correction made
- follow-up command(s): ...

## Negative-case evidence required

- authN/authZ boundary failures
- replay/idempotency and stale-timing failures
- stale/ordering/fault-path failures
- manual-required state and bypass limitations
- rollback/teardown evidence for each failure path

## Slice close gate

1. Test run attached for the slice
2. At least one negative-case proof captured
3. Teardown/rollback evidence included
4. Cross-layer proof references updated in `PLAN_EXECUTION_BLUEPRINT.md`
