# WP134 Timer Parent-Surface Child UX Parent Preference Setup Request Boundary

## Scope

- Added parent preference setup request-ready/unavailable status fields to the
  agent-protocol-domain timer parent-surface read-model schema.
- Mirrored the fields in Rust protocol and agent-service timer-parent-surface
  payload construction.
- Added the schema-backed
  `agent.activity.app-game.timer-parent-surface.parent-preference-setup.request`
  command and matching requested event with TypeScript and Rust protocol parity.
- Added an agent-service WebSocket command response that accepts the
  parent-safe setup request as a command-boundary proof while leaving durable
  preference mutation and downstream delivery unclaimed.
- Derived request-ready status for service-emitted child UX parent preference
  setup records from existing parent-safe setup records.
- Updated the App/Game Sessions portal intent so request-ready setup records
  render as parent preference setup UI-ready.
- Extended focused TypeScript, Rust, and portal tests for setup request status,
  command/request result parsing, request refs, UI readiness, and no-claim
  boundaries.

## No-Claim Boundary

- Parent preference mutation, notification rule mutation, provider delivery,
  receipt ingestion, quiet-hours runtime, child delivery, durable service
  outbox storage, adapter dispatch, broad blocking, platform enforcement, raw
  private source rows, raw target values, and private diagnostics remain
  unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane
  owns production-support checklist churn.

## Validation

See `10-validation-commands.log`.
