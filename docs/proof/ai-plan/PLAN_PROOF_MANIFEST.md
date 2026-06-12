# Proof Manifest - ai-plan

- plan route: docs/plans/ai-plan/PLAN_STATE.md
- assigned workpack: one or more entries from docs/plans/ai-plan/PLAN_EXECUTION_BLUEPRINT.md slice-to-workpack binding
- owner: active execution lane owner (AI/Developer)
- test boundary: unit/integration/e2e/security/non-functional as defined in plan HID floor

## Required proof bundle

- docs/proof/ai-plan/slice-01-contract-schema.md
- docs/proof/ai-plan/slice-02-provider-routing.md
- docs/proof/ai-plan/slice-03-safety-boundary.md
- docs/proof/ai-plan/slice-04-output-invariants.md
- tests/ or explicit test run transcript (`.log`, `.junit`, or Playwright report)
- logs/ (command output and transport-level traces)
- screenshots/ or traces/ (UI and runtime proof when applicable)

## Required test families for closed slice

- E2E: policy handoff path with evidence routing
- Integration: provider routing queue and dead-letter
- Non-functional: timeout/fallback and resource pressure
- Security: prompt-injection, hallucination boundary
- Unit: parser/schema + output invariants

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
