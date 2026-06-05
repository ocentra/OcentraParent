# 63. Notification Payload Preflight

## Goal

Bridge app/game scheduler rows into a minimal notification payload preflight
read model before any provider template, provider delivery, or parent UI claim
is allowed.

## Scope

- Reuse WP59 `AppGameNotificationSchedulerBridgeReadModel` rows.
- Require scheduled rows to carry the expected minimal alert id,
  family/device scope, severity, reason code, evidence ref, policy ref, and
  parent action link fields.
- Require sensitive-detail exclusions for raw child evidence, raw URL/title,
  raw message text, screenshots/reports, and sensitive provider metadata.
- Keep manual-required and unavailable scheduler rows blocked with their source
  proof requirements.
- Preserve explicit false claims for provider payload template runtime,
  provider delivery, receipt ingestion, credentials, cloud routing, parent
  notification UI, child delivery, retry workers, quiet-hours timers, durable
  production outbox storage, adapter dispatch, broad blocking, and platform
  support.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider-specific template rendering runtime.
- Provider credentials, receipt webhooks, or receipt ingestion.
- Production retry workers, production quiet-hours timers, or durable outbox
  database runtime.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-payload-preflight.ts`
- `packages/parent-domain/tests/app-game-notification-payload-preflight.test.ts`
- `scripts/test/app-game-notification-payload-preflight-proof.mjs`
- `test-results/app-game-notification-payload-preflight-proof/proof.json`
- `output/app-game-plan-proof/63-notification-payload-preflight/`
- `output/app-plan-proof/63-notification-payload-preflight/`

## Validation

- [x] Payload preflight parses the WP59 scheduler bridge read model first.
- [x] Scheduled app/game rows require the seven minimal payload fields.
- [x] Scheduled app/game rows require five sensitive-detail exclusion refs.
- [x] Manual-required and unavailable rows remain blocked without scheduler,
      channel, reason, payload, template, or provider refs.
- [x] Schema tests reject provider template runtime and sensitive-detail
      overclaims.
- [x] Proof pack records no provider delivery, no receipt ingestion, no
      retry-worker/quiet-hours timer runtime, no parent UI, no child delivery,
      no adapter dispatch, no durable production outbox, and no platform claim.
