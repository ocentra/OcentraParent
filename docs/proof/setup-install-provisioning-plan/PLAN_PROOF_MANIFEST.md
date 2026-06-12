# Proof Manifest - setup-install-provisioning-plan

- plan route: docs/plans/setup-install-provisioning-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/setup-install-provisioning-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/setup-install-provisioning-plan/slice-01-handoff.md
- docs/proof/setup-install-provisioning-plan/slice-02-state-machine.md
- docs/proof/setup-install-provisioning-plan/slice-03-platform-install.md
- docs/proof/setup-install-provisioning-plan/slice-04-recovery.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: first-run install + recovery paths
- Integration: website-to-runtime handoff
- Non-functional: platform matrix and manual fallback behavior
- Security: onboarding abuse and unauthorized state entry
- Unit: identity/site/setup contract checks

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
