# WP131 Timer Parent-Surface Child UX Parent Preference Setup Draft

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP131 Timer Parent-Surface Child UX Parent Preference Setup Draft`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
deriving parent-safe preference setup draft rows from child UX parent-surface
intent rows before any parent preference UI or mutation work.

## Scope

- Add a parent-domain schema-backed read model for parent preference setup
  drafts derived from child UX parent-surface intent rows.
- Preserve scheduler, outbox, provider-channel, parent-preference,
  quiet-hours, drill-in, and manual-proof refs in a parent-safe shape.
- Separate draft-ready rows from unavailable-visible rows so future parent UI
  can distinguish setup candidates from visible-but-disabled states.
- Prove schema honesty with focused tests over the real child UX local outbox
  provider/preference/parent-surface chain.

## Non-Goals

- No rendered parent preference UI or parent frequency controls.
- No parent preference mutation or notification rule mutation.
- No provider delivery, delivery receipts, receipt ingestion, retry workers, or
  quiet-hours runtime.
- No child runtime delivery.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, private diagnostics,
  screenshots, reports, or sensitive child evidence in the draft read model.
- No package export while package ownership remains elsewhere.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Focused parent-domain preference setup draft test.
- Parent-domain build.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
