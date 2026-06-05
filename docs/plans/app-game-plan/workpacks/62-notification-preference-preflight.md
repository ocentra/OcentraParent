# 62. Notification Preference Preflight

## Goal

Bridge app/game notification scheduler rows into an explicit parent-preference
preflight read model so scheduled alerts expose parent preference, frequency,
and quiet-hours proof requirements before any provider delivery can be claimed.

## Scope

- Reuse WP59 `AppGameNotificationSchedulerBridgeReadModel` rows.
- Convert scheduled local scheduler rows into `parent-preference-required`
  preflight rows with scheduler, outbox, provider-channel, and reason refs
  preserved.
- Require parent preference, frequency-control, and quiet-hours policy proof
  refs before delivery.
- Keep manual-required and unavailable app/game notification rows blocked before
  preference preflight with manual proof requirements.
- Preserve explicit false claims for parent preference UI, frequency-control UI,
  quiet-hours timer runtime, provider delivery, receipt ingestion, credentials,
  cloud routing, child delivery, retry-worker execution, durable production
  outbox storage, and adapter dispatch.

## Non-Goals

- Parent notification history, preferences, or frequency-control UI.
- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-preference-preflight.ts`
- `packages/parent-domain/tests/app-game-notification-preference-preflight.test.ts`
- `scripts/test/app-game-notification-preference-preflight-proof.mjs`
- `test-results/app-game-notification-preference-preflight-proof/proof.json`
- `output/app-game-plan-proof/62-notification-preference-preflight/`
- `output/app-plan-proof/62-notification-preference-preflight/`

## Validation

- [x] Preference preflight parses the WP59 app/game scheduler bridge read model
      before mapping rows.
- [x] Scheduled local rows become parent-preference-required preflight rows with
      source scheduler/outbox/provider/reason refs.
- [x] Manual-required and unavailable rows remain blocked before preference
      preflight.
- [x] Proof pack records no parent preference UI, no frequency-control UI, no
      quiet-hours timer runtime, no provider delivery, no receipt ingestion, no
      credentials, no child delivery, no adapter dispatch, and no durable
      production outbox claim.
