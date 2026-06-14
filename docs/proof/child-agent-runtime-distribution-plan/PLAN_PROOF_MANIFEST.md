# Proof Manifest - child-agent-runtime-distribution-plan

- plan route: docs/plans/child-agent-runtime-distribution-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/child-agent-runtime-distribution-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/child-agent-runtime-distribution-plan/slice-01-checklist-snapshot.md
- docs/proof/child-agent-runtime-distribution-plan/slice-02-windows-service.md
- docs/proof/child-agent-runtime-distribution-plan/slice-03-macos-service.md
- docs/proof/child-agent-runtime-distribution-plan/slice-04-linux-service.md
- docs/proof/child-agent-runtime-distribution-plan/slice-05-android-package.md
- docs/proof/child-agent-runtime-distribution-plan/slice-06-ios-capability.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- Package lifecycle and service start/stop
- Integration: package smoke + runtime handoff
- Non-functional: startup latency and smoke reproducibility
- Security: signing checks and uninstall/tamper checks
- Unit: package/bootstrap contract checks

## Negative-case evidence required

- respawn unavailable or manual-required state
- replay/idempotency and stale-timing failures
- uninstall/revocation failures
- manual-required state and bypass limitations
- rollback/teardown evidence for each failure path

## Slice close gate

1. Test run attached for the slice
2. At least one negative-case proof captured
3. Teardown/rollback or uninstall evidence included
4. Cross-layer proof references updated in `PLAN_EXECUTION_BLUEPRINT.md`

