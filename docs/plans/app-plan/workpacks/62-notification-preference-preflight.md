# 62. Notification Preference Preflight

## Goal

Cross-record the shared app/game parent-preference preflight boundary for native
app notification handoff so native app alerts expose parent preference,
frequency, and quiet-hours requirements after WP59 scheduler linking without
claiming UI or delivery.

## Scope

- Reuse the shared app/game WP62 preference preflight bridge.
- Map scheduled native app/game scheduler rows into parent-preference-required
  preflight rows.
- Keep manual-required and unavailable native app notification rows blocked
  before preference preflight.
- Preserve all parent preference UI, frequency-control UI, provider delivery,
  receipt, credential, runtime, child delivery, adapter, broad blocking, and
  platform non-claims.

## Non-Goals

- Native app notification parent preference UI or frequency controls.
- Native app notification provider delivery.
- Provider credentials, webhooks, templates, receipts, or receipt ingestion.
- Production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- Child-device delivery, policy evaluator execution, broad app blocking,
  adapter dispatch, or platform support.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-preference-preflight.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-preference-preflight.test.ts`
- Harness:
  `scripts/test/app-game-notification-preference-preflight-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/62-notification-preference-preflight/`

## Validation

- [x] Cross-recorded from shared app/game WP62 proof.
- [x] Native app rows require parent preference/frequency/quiet-hours proof only
      after scheduler proof.
- [x] Manual-required and unavailable rows remain blocked before preference
      preflight.
- [x] UI/delivery/provider credential/runtime/child/adapter/platform claims
      remain false.
