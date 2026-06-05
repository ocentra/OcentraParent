# 63. Notification Payload Preflight

## Goal

Cross-record the shared app/game notification payload preflight for native app
notification handoff so native app alerts prove minimal provider-facing payload
requirements after WP59 scheduling.

## Scope

- Reuse the shared app/game WP63 payload preflight.
- Require scheduled native app/game notification rows to cite minimal payload
  fields and sensitive-detail exclusions.
- Keep manual-required and unavailable native app notification rows blocked
  without provider payload refs.
- Preserve all provider, receipt, runtime, UI, child delivery, adapter, broad
  blocking, and platform non-claims.

## Non-Goals

- Native app notification provider delivery.
- Provider-specific payload template runtime.
- Production local scheduler runtime, retry workers, quiet-hours timers, or
  durable outbox storage.
- Parent notification UI or child-device delivery.
- Policy evaluator execution, broad app blocking, adapter dispatch, or
  platform support.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-payload-preflight.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-payload-preflight.test.ts`
- Harness:
  `scripts/test/app-game-notification-payload-preflight-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/63-notification-payload-preflight/`

## Validation

- [x] Cross-recorded from shared app/game WP63 proof.
- [x] Native app rows require minimal payload fields after scheduling.
- [x] Sensitive child detail stays excluded from provider-facing payloads.
- [x] Manual-required and unavailable rows remain blocked.
- [x] Runtime/provider/UI/child/adapter/platform claims remain false.
