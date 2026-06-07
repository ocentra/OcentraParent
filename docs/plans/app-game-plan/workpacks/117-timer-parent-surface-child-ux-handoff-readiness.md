# WP117 Timer Parent-Surface Child UX Handoff Readiness

## Goal

Advance the unified native app plus native game control path from child-facing
reason/status reference visibility into an auditable local child UX handoff
readiness state.

## Scope

- Extend the app/game timer parent-surface read model with child UX handoff
  ready/blocked counts and handoff result-reference ids.
- Derive readiness from real app/game approval action-result rows that contain
  both child reason and child status references.
- Add parent-domain validation that maps child-facing UX cards into ready or
  blocked local handoff rows.
- Render handoff readiness in the parent App/Game Sessions timer parent-surface
  summary.
- Preserve explicit no-claim boundaries for child runtime delivery,
  notification delivery, adapter dispatch, broad blocking, platform
  enforcement, private diagnostics, and raw private source rows.

## Non-Goals

- No child-device UI runtime.
- No notification provider delivery or receipt ingestion.
- No adapter execution.
- No broad app/game blocking support claim.
- No package export change while `packages/parent-domain/package.json` is owned
  by another lane.

## Validation

- Rust protocol and service tests for the timer parent-surface read model.
- Parent-domain child UX handoff tests.
- Agent-protocol-domain parser tests.
- Portal timer parent-surface intent tests.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard,
  and hub guard before commit.
