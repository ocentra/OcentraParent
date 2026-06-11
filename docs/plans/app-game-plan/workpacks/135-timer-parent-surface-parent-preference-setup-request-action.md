# WP135 Timer Parent-Surface Parent Preference Setup Request Action

## Goal

Turn the WP134 parent preference setup request boundary into a real App/Game
Sessions portal action for request-ready setup rows.

## Scope

- Add a portal-domain request action to service-emitted timer parent-surface
  child UX parent preference setup rows when the setup request status is
  request-ready.
- Build the schema-backed parent preference setup request payload from
  parent-safe refs, the setup reference, the source parent-surface intent ref,
  and the click timestamp.
- Render a parent setup request button on request-ready setup cards in the
  App/Game Sessions route.
- Select the
  `agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested`
  result event so the existing command-result panel can display the accepted
  service response.
- Keep unavailable setup rows read-only.

## Non-Goals

- No durable parent preference mutation.
- No notification rule mutation.
- No provider delivery, receipt ingestion, retry execution, or quiet-hours
  runtime.
- No child runtime delivery.
- No durable service outbox storage.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, screenshots, reports, or
  private diagnostics in the request action.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Portal-domain build.
- Focused App/Game Sessions timer parent-surface portal test.
- Portal build.
- `git diff --check`, lane guard, and hub guard before commit.
