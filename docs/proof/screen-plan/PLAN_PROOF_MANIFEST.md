# Proof Manifest - screen-plan

- plan route: docs/plans/screen-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/screen-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/screen-plan/slice-01-capture-custody.md
- docs/proof/screen-plan/slice-02-ocr-redaction.md
- docs/proof/screen-plan/slice-03-retention-journal.md
- docs/proof/screen-plan/slice-04-policy-handoff.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: child-parent visibility and consent branches
- Integration: custody, deletion, and handoff plumbing
- Non-functional: event sequencing and retention timing
- Security: permission, auth lifecycle, replay ordering
- Unit: capture/session schema checks

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
