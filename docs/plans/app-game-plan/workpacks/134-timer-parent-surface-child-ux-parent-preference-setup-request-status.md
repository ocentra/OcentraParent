# WP134 Timer Parent-Surface Child UX Parent Preference Setup Request Boundary

## Goal

Advance the unified native app plus native game parent surface from read-only
setup records to an honest parent preference setup request/status and command
boundary.

## Scope

- Extend the app/game timer parent-surface read-model contract with
  parent-preference setup request status counts and reference ids.
- Add the schema-backed
  `agent.activity.app-game.timer-parent-surface.parent-preference-setup.request`
  command and matching reported event so a parent surface can request setup
  without claiming a durable preference mutation.
- Mirror those request status fields in the Rust protocol and agent-service
  timer parent-surface payload, and mirror the request command/result in Rust
  protocol plus the agent-service WebSocket handler.
- Derive request-ready setup records from existing service-emitted child UX
  parent preference setup records, using only parent-safe refs.
- Return an accepted command-boundary result that keeps preference mutation,
  notification rule mutation, provider delivery, durable outbox storage,
  adapter dispatch, and platform enforcement unclaimed.
- Render request-ready setup records in the App/Game Sessions portal intent as
  parent preference setup UI-ready.
- Keep parent preference mutation, notification rule writes, delivery,
  adapter dispatch, platform enforcement, raw private rows, raw target values,
  and private diagnostics unclaimed.

## Non-Goals

- No parent preference mutation.
- No notification rule mutation.
- No provider delivery, provider receipts, receipt ingestion, retry workers, or
  quiet-hours runtime.
- No child runtime delivery.
- No durable service outbox storage.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, screenshots, reports, or
  private diagnostics in parent-visible request refs.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Agent-protocol-domain build and focused timer parent-surface/request parser
  tests.
- Portal-domain build and focused App/Game Sessions timer parent-surface portal
  test.
- Rust protocol and agent-service focused timer parent-surface/request command
  tests.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
