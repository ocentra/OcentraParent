# 64. Notification Provider Status Handoff

## Goal

Cross-record the shared app/game provider-status handoff boundary for native
app notifications so native app provider-preflight rows become V0.8
provider-status manual-required or unavailable rows without claiming delivery.

## Scope

- Reuse the shared app/game WP64 provider-status handoff proof.
- Map native app/game provider preflight rows into V0.8 provider-status
  manual-required and unavailable boundary rows.
- Preserve scheduler, outbox, provider-channel, readiness, and manual proof refs
  where available.
- Preserve all provider delivery, receipt, credential, runtime, UI, child
  delivery, adapter, broad blocking, and platform non-claims.

## Non-Goals

- Native app notification provider delivery.
- Provider credentials, webhooks, templates, receipts, or receipt ingestion.
- Production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- Parent notification UI/history/preferences or child-device delivery.
- Policy evaluator execution, broad app blocking, adapter dispatch, or platform
  support.

## Code-pass status

The agent-service notification-readiness report now emits a typed provider
status-boundary read model derived from the existing app/game readiness rows.
parent-runtime-core consumes that payload and the existing portal notification
panel renders the provider status, proof state, delivery claim, and manual-proof
requirements. The app-game-core preflight/handoff builders remain the upstream
typed ownership; this service projection does not claim their native runtime
execution. Provider delivery remains manual-required or unavailable, and the
workpack stays unvalidated with tests/proof/checklist deferred.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-provider-status-handoff.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-provider-status-handoff.test.ts`
- Harness:
  `scripts/test/app-game-notification-provider-status-handoff-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/64-notification-provider-status-handoff/`

## Validation

- [ ] Cross-recorded from shared app/game WP64 proof.
- [ ] Native app rows map provider preflight status into existing V0.8
      provider-status manual-required/unavailable rows.
- [ ] Delivery/provider credential/runtime/UI/history/child/adapter/platform
      claims remain false.
- [ ] Product checklist unchanged because this handoff does not move feature
      status and provider/runtime/UI/platform gaps remain.
