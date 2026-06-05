# 65. Notification Preference Status Handoff

## Goal

Map app/game notification preference-preflight rows into V3 notification
preference and quiet-hours status entries so parent preference requirements are
visible in the shared notification boundary without claiming parent UI or
delivery.

## Scope

- Reuse WP62 `AppGameNotificationPreferencePreflightReadModel` rows.
- Convert parent-preference-required and manual-required rows into V3
  manual-required notification status entries.
- Convert unavailable rows into V3 disabled/not-sent notification status
  entries.
- Preserve scheduler, outbox, provider-channel, reason, parent preference,
  quiet-hours, and manual proof refs where the source row provides them.
- Preserve explicit false claims for parent preference UI, frequency controls,
  parent notification UI, provider delivery, receipt ingestion, credentials,
  cloud routing, child delivery, retry-worker execution, quiet-hours timer
  execution, production durable outbox storage, adapter dispatch, broad
  blocking, and platform support.

## Non-Goals

- Parent notification preferences UI, frequency controls, or history UI.
- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-preference-status-handoff.ts`
- `packages/parent-domain/tests/app-game-notification-preference-status-handoff.test.ts`
- `scripts/test/app-game-notification-preference-status-handoff-proof.mjs`
- `test-results/app-game-notification-preference-status-handoff-proof/proof.json`
- `output/app-game-plan-proof/65-notification-preference-status-handoff/`
- `output/app-plan-proof/65-notification-preference-status-handoff/`

## Validation

- [x] Preference-status handoff parses WP62 app/game preference preflight rows
      before mapping.
- [x] Parent-preference-required and manual-required rows become V3
      manual-required notification preference/quiet-hours status entries.
- [x] Unavailable rows become V3 disabled/not-sent status entries.
- [x] Proof pack records no parent preference UI, no frequency controls, no
      parent notification UI, no provider delivery, no receipt ingestion, no
      credentials, no retry-worker/quiet-hours timer runtime, no child delivery,
      no adapter dispatch, no broad blocking, and no durable production outbox
      claim.
- [x] Product checklist unchanged because this handoff does not move feature
      status and provider/runtime/UI/platform gaps remain.
