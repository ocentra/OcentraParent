# Proof Manifest - remote-access-plan

- plan route: docs/plans/remote-access-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/remote-access-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/remote-access-plan/slice-01-authz-grant.md
- docs/proof/remote-access-plan/slice-02-relay-transport.md
- docs/proof/remote-access-plan/slice-03-view-control-separation.md
- docs/proof/remote-access-plan/slice-04-retention-audit.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: remote-control establishment and teardown
- Integration: relay separation and control/view lanes
- Non-functional: retry storm and partial-outage behavior
- Security: authN/authZ, replay and log redaction tests
- Unit: grant/session contracts

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
