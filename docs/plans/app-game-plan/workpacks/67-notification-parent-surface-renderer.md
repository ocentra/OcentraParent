# 67. Notification Parent Surface Renderer

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `67. Notification Parent Surface Renderer`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `crates/parent-runtime-core/src/parent_ui_bridge/app_game_notification.rs`
- `crates/parent-runtime-core/tests/integration/parent_ui_bridge/runtime_and_activity_tests.rs`
- `apps/portal/tests/unit/app-game-notification-parent-surface-panel.test.ts`
- `scripts/test/app-game-notification-parent-surface-ui-proof.mjs`
- `test-results/app-game-notification-parent-surface-ui-proof/proof.json`
- `output/app-game-plan-proof/67-notification-parent-surface-renderer/`
- `output/app-plan-proof/67-notification-parent-surface-renderer/`

## Validation

- [ ] Schema-backed parent-surface intent rows render status, drill-in,
      scheduler/outbox, preference, quiet-hours, and manual-proof refs.
- [ ] Missing or invalid service input renders an explicit empty state instead
      of inventing rows.
- [ ] The panel is gated to the App/Game Sessions route.
- [ ] Proof pack records no live service event, no parent preference mutation,
      no provider delivery, no receipt ingestion, no credentials, no production
      runtime, no child delivery, no adapter dispatch, no broad blocking, no
      mobile UI, and no platform support.
- [ ] Product checklist unchanged because this renderer proof does not move
      feature status and runtime/provider/platform gaps remain.
