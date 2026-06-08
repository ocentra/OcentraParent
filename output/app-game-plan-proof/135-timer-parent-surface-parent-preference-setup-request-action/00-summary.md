# WP135 Timer Parent-Surface Parent Preference Setup Request Action

## Scope

- Added a portal-domain request action for service-emitted timer parent-surface
  child UX parent preference setup records whose request status is
  `request-ready`.
- Added a schema-backed payload builder that serializes the
  `appGameTimerParentPreferenceSetupRequest` field with parent-safe request
  refs, source parent-surface intent ref, setup ref, request id, and click
  timestamp.
- Wired the App/Game Sessions timer parent-surface setup card to send
  `agent.activity.app-game.timer-parent-surface.parent-preference-setup.request`
  and select
  `agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested`
  for the command-result panel.
- Added the accepted setup request event to the portal command-result event
  whitelist.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
- `cmd /c npm run build --workspace @ocentra-parent/portal`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/app-game-timer-parent-surface-panel.test.ts`

## No-Claim Boundaries

- No durable parent preference mutation.
- No notification rule mutation.
- No provider delivery or receipt ingestion.
- No child runtime delivery.
- No durable outbox storage.
- No adapter dispatch, broad blocking, or platform enforcement.
- No raw private source rows, raw target values, screenshots, reports, or
  private diagnostics in the request action.
- `docs/product-capability-checklist.md` was not updated because another lane
  owns that checklist churn.
