# Proof Manifest - portal-ux-household-surfaces-plan

- plan route: docs/plans/portal-ux-household-surfaces-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/portal-ux-household-surfaces-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/portal-ux-household-surfaces-plan/slice-01-service-shell.md
- docs/proof/portal-ux-household-surfaces-plan/slice-02-setup-state-machine.md
- docs/proof/portal-ux-household-surfaces-plan/slice-03-policy-logging.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: visible state/error transitions
- Integration: data flow from service to household UI
- Non-functional: render and accessibility checks under failure
- Security: authZ leaks and unauthorized state transitions
- Unit: portal contract schema checks

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
