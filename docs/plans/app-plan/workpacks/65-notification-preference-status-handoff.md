# 65. Notification Preference Status Handoff

## Goal

Cross-record the shared app/game preference-status handoff boundary for native
app notifications so native app preference-preflight rows become V3 notification
preference and quiet-hours status entries without claiming parent UI or
delivery.

## Scope

- Reuse the shared app/game WP65 preference-status handoff proof.
- Map native app/game preference preflight rows into V3 manual-required and
  disabled/not-sent notification status entries.
- Preserve scheduler, outbox, provider-channel, reason, parent preference,
  quiet-hours, and manual proof refs where available.
- Preserve all parent preference UI, frequency-control, provider delivery,
  receipt, credential, runtime, UI, child delivery, adapter, broad blocking, and
  platform non-claims.

## Non-Goals

- Native app notification preference UI, provider delivery, or history UI.
- Provider credentials, webhooks, templates, receipts, or receipt ingestion.
- Production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- Child-device delivery, policy evaluator execution, broad app blocking,
  adapter dispatch, or platform support.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-preference-status-handoff.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-preference-status-handoff.test.ts`
- Harness:
  `scripts/test/app-game-notification-preference-status-handoff-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/65-notification-preference-status-handoff/`

## Validation

- [ ] Cross-recorded from shared app/game WP65 proof.
- [ ] Native app rows map preference preflight status into V3 manual-required or
      disabled/not-sent notification status entries.
- [ ] Parent preference UI, delivery/provider credential/runtime/UI/history/
      child/adapter/platform claims remain false.
- [ ] Product checklist unchanged because this handoff does not move feature
      status and provider/runtime/UI/platform gaps remain.
