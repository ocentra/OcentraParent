# 61. Notification Provider Preflight

## Goal

Cross-record the shared app/game provider-preflight boundary for native app
notification handoff so native app alerts expose provider adapter requirements
after WP59 scheduler linking without claiming delivery.

## Scope

- Reuse the shared app/game WP61 provider preflight bridge.
- Map scheduled native app/game scheduler rows into provider-adapter-required
  preflight rows.
- Keep manual-required and unavailable native app notification rows blocked
  before provider preflight.
- Preserve all provider delivery, receipt, credential, runtime, UI, child
  delivery, adapter, broad blocking, and platform non-claims.

## Non-Goals

- Native app notification provider delivery.
- Provider credentials, webhooks, templates, receipts, or receipt ingestion.
- Production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- Parent notification UI or child-device delivery.
- Policy evaluator execution, broad app blocking, adapter dispatch, or
  platform support.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-provider-preflight.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-provider-preflight.test.ts`
- Harness:
  `scripts/test/app-game-notification-provider-preflight-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/61-notification-provider-preflight/`

## Validation

- [x] Cross-recorded from shared app/game WP61 proof.
- [x] Native app rows require provider adapter setup only after scheduler proof.
- [x] Manual-required and unavailable rows remain blocked before provider
      preflight.
- [x] Delivery/provider credential/runtime/UI/child/adapter/platform claims
      remain false.
