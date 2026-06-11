# WP133 Timer Parent-Surface Child UX Parent Preference Setup Service Records

## Goal

Advance the unified native app plus native game child UX evidence spine by
moving parent preference setup card inputs into the live timer parent-surface
service read model.

## Scope

- Extend the agent-protocol-domain timer parent-surface read-model contract with
  dedicated child UX parent preference setup counts, reference ids, and records.
- Mirror those records in the Rust protocol and agent-service timer
  parent-surface payload.
- Derive setup records from replayed action-result rows and structured child UX
  parent-surface intent records without adding parent UI, mutation, delivery, or
  platform enforcement claims.
- Render App/Game Sessions parent preference setup cards from service-emitted
  setup records instead of deriving cards from parent-surface intent rows in the
  portal-domain layer.
- Extend focused protocol, Rust service, and portal tests to prove the records
  are service-backed and no-claim flags remain false.

## Non-Goals

- No interactive parent preference UI controls.
- No frequency controls.
- No parent preference mutation or notification rule mutation.
- No provider delivery, delivery receipts, receipt ingestion, retry workers, or
  quiet-hours runtime.
- No child runtime delivery.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, private diagnostics,
  screenshots, reports, or sensitive child evidence in the setup records.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Agent-protocol-domain build and focused timer parent-surface parser test.
- Portal-domain build and focused App/Game Sessions timer parent-surface portal
  test.
- Rust protocol and agent-service focused timer parent-surface tests.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
