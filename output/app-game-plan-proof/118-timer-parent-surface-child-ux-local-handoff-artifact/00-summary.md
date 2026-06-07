# WP118 Timer Parent-Surface Child UX Local Handoff Artifact

## Summary

WP118 adds a parent-domain local handoff artifact bridge for the unified native
app plus native game child UX path. It consumes WP117 child UX handoff readiness
rows, writes only ready rows to schema-validated JSONL artifact records, and
parses the artifact back through the same Effect Schema boundary.

## Scope

- `packages/parent-domain/src/app-game-child-facing-ux-local-handoff.ts`
  defines local handoff artifact record/read-model contracts plus JSONL
  serialize/parse helpers.
- `packages/parent-domain/tests/app-game-child-facing-ux-local-handoff.test.ts`
  proves native-app and native-game ready rows are written, blocked rows are
  skipped, and delivery/adapter/platform/diagnostics overclaims are rejected.
- App-game feature and implementation docs record the new local artifact bridge
  while keeping live child UI, notifications, service persistence, adapter
  dispatch, broad blocking, platform enforcement, and package exports as gaps.

## No-Claim Boundary

- No child-device UI runtime.
- No child runtime delivery.
- No notification provider delivery or receipt ingestion.
- No adapter dispatch.
- No broad blocking or platform enforcement.
- No private diagnostics or raw private source rows.
- No package manifest/export update.
