# 59. Notification Scheduler Bridge

## Goal

Bridge app/game local outbox records into the existing parent-domain local
notification scheduler row schema so app/game alerts can prove deterministic
scheduler handoff without claiming provider delivery or production runtime.

## Scope

- Reuse WP58 `AppGameNotificationLocalOutboxBridgeReadModel` rows.
- Schedule only linked local outbox records.
- Keep manual-required and unavailable app/game notification rows unscheduled
  with explicit proof requirements.
- Serialize and reread scheduler rows through the existing
  `NotificationLocalOutboxSchedulerRecordSchema`.
- Preserve explicit false claims for provider delivery, receipt ingestion,
  credentials, cloud routing, parent notification UI, child delivery,
  retry-worker execution, quiet-hours timer execution, production durable
  storage, adapter dispatch, broad blocking, and platform support.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, receipt webhooks, or receipt ingestion.
- Production retry workers, production quiet-hours timers, or durable outbox
  database runtime.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-scheduler-bridge.ts`
- `packages/parent-domain/tests/app-game-notification-scheduler-bridge.test.ts`
- `scripts/test/app-game-notification-scheduler-bridge-proof.mjs`
- `test-results/app-game-notification-scheduler-bridge-proof/proof.json`
- `output/app-game-plan-proof/59-notification-scheduler-bridge/`
- `output/app-plan-proof/59-notification-scheduler-bridge/`

## Validation

- [x] Bridge parses the WP58 app/game outbox bridge read model before
      scheduling.
- [x] Only linked local outbox records become scheduler JSONL rows.
- [x] Manual-required and unavailable rows remain unscheduled.
- [x] Scheduler JSONL rereads through the existing scheduler record parser.
- [x] Proof pack records no provider delivery, no receipt ingestion, no
      retry-worker/quiet-hours timer runtime, no parent UI, no child delivery,
      no adapter dispatch, no durable production outbox, and no platform claim.
