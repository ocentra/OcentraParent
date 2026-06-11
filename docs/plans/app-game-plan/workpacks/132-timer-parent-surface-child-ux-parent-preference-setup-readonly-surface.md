# WP132 Timer Parent-Surface Child UX Parent Preference Setup Read-Only Surface

## Goal

Advance the unified native app plus native game child UX evidence spine by
surfacing WP131 parent preference setup draft state in the live App/Game
Sessions parent-surface route as read-only setup cards.

## Scope

- Export the parent-domain parent preference setup draft boundary for
  cross-package consumption.
- Derive read-only parent preference setup rows in the portal-domain timer
  parent-surface intent from live child UX parent-surface records.
- Render those setup rows on the App/Game Sessions route as separate cards with
  target meaning, draft status, parent-safe refs, and explicit no-claim UI,
  mutation, rule-write, delivery, adapter, child-delivery, and platform states.
- Extend the focused portal test to prove the service-backed event produces the
  setup cards and absent service input does not invent rows.

## Non-Goals

- No interactive parent preference UI controls.
- No parent preference mutation or notification rule mutation.
- No provider delivery, delivery receipts, receipt ingestion, retry workers, or
  quiet-hours runtime.
- No child runtime delivery.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, private diagnostics,
  screenshots, reports, or sensitive child evidence in the setup cards.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Parent-domain build.
- Portal-domain build.
- Focused App/Game Sessions timer parent-surface portal test.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
