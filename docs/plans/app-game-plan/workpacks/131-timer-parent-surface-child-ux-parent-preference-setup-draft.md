# WP131 Timer Parent-Surface Child UX Parent Preference Setup Draft

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
