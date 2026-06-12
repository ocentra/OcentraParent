# WP117 Timer Parent-Surface Child UX Handoff Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP117 Timer Parent-Surface Child UX Handoff Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
