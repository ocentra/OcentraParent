# WP118 Timer Parent-Surface Child UX Local Handoff Artifact

## Goal

Advance the unified native app plus native game control path from child UX
handoff readiness into a schema-validated local handoff artifact boundary that
future child runtime code can consume without claiming delivery.

## Scope

- Add a parent-domain local handoff artifact bridge that consumes the WP117 child
  UX handoff read model.
- Serialize only ready app/game child UX handoff rows into JSONL artifact
  records.
- Parse JSONL artifact rows back through Effect Schema contracts.
- Keep blocked missing-ref rows out of the local artifact and counted as skipped.
- Preserve explicit no-claim boundaries for child runtime delivery,
  notification delivery, adapter dispatch, platform enforcement, private
  diagnostics, and raw private source rows.

## Non-Goals

- No child-device UI runtime.
- No notification provider delivery or receipt ingestion.
- No adapter execution.
- No platform enforcement or broad blocking claim.
- No package manifest/export change while shared package ownership is held by
  another lane.
- No central product checklist update while `docs/product-capability-checklist.md`
  is locked by another lane.

## Validation

- Parent-domain child UX local handoff artifact tests.
- Parent-domain package build.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
