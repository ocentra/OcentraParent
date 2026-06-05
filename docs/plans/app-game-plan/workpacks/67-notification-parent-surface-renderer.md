# 67. Notification Parent Surface Renderer

## Goal

Render the WP66 app/game notification parent-surface intent read model in the
Portal App/Game Sessions route so parent-visible setup and unavailable states
are visible without claiming notification delivery or preference mutation.

## Scope

- Reuse the WP66 `AppGameNotificationParentSurfaceIntentReadModel` contract.
- Render schema-backed parent-surface intent rows only after validation.
- Show an explicit missing-service empty state when the route has no supplied
  read model.
- Display redacted status, drill-in refs, scheduler/outbox refs,
  provider/preference status, quiet-hours status, manual-proof requirements,
  and explicit no-runtime claims.
- Keep the renderer mounted only on the App/Game Sessions route.

## Non-Goals

- Live service event ingestion for parent-surface intent rows.
- Parent preference mutation, frequency controls, or quiet-hours editor
  behavior.
- Provider push, email, SMS, WhatsApp, in-app delivery, credentials, webhooks,
  delivery receipts, or receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox/history storage, or cloud routing.
- Child-device delivery, mobile UI, policy evaluator execution, adapter
  dispatch, broad app/game blocking, or platform support.

## Proof

- `apps/portal/src/AppGameNotificationParentSurfaceRoutePanel.tsx`
- `apps/portal/src/app-game-notification-parent-surface-panel.ts`
- `apps/portal/tests/app-game-notification-parent-surface-panel.test.ts`
- `scripts/test/app-game-notification-parent-surface-ui-proof.mjs`
- `test-results/app-game-notification-parent-surface-ui-proof/proof.json`
- `output/app-game-plan-proof/67-notification-parent-surface-renderer/`
- `output/app-plan-proof/67-notification-parent-surface-renderer/`

## Validation

- [x] Schema-backed parent-surface intent rows render status, drill-in,
      scheduler/outbox, preference, quiet-hours, and manual-proof refs.
- [x] Missing or invalid service input renders an explicit empty state instead
      of inventing rows.
- [x] The panel is gated to the App/Game Sessions route.
- [x] Proof pack records no live service event, no parent preference mutation,
      no provider delivery, no receipt ingestion, no credentials, no production
      runtime, no child delivery, no adapter dispatch, no broad blocking, no
      mobile UI, and no platform support.
- [x] Product checklist unchanged because this renderer proof does not move
      feature status and runtime/provider/platform gaps remain.
