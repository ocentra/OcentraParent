# WP119 Timer Parent-Surface Child UX Local Artifact Visibility

## Summary

WP119 makes the WP118 child UX local handoff artifact boundary visible in the
live app/game timer parent-surface read model and portal summary. The service
now reports artifact record counts, skipped blocked-row counts, and artifact
reference ids derived from ready child UX handoff rows.

## Scope

- `crates/agent-protocol` adds child UX local artifact fields to the timer
  parent-surface read model.
- `crates/agent-service` derives artifact visibility from ready child UX
  handoff action-result rows and emits those fields in the parent-surface
  payload.
- `packages/agent-protocol-domain` parses the new fields from the service
  event.
- `packages/portal-domain` and `apps/portal` render and test the artifact
  record/skipped/ref details.

## No-Claim Boundary

- No child-device UI runtime.
- No child runtime delivery.
- No notification provider delivery.
- No adapter dispatch.
- No broad blocking or platform enforcement.
- No private diagnostics or raw private source rows.
- No package manifest/export update.
