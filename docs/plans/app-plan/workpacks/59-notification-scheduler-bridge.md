# 59. Notification Scheduler Bridge

## Goal

Cross-record the shared app/game scheduler bridge for native app notification
handoff so native app alerts can prove deterministic local scheduler rows after
WP58 outbox linking.

## Scope

- Reuse the shared app/game WP59 scheduler bridge.
- Schedule only linked local outbox records from native app/game notification
  intents.
- Keep manual-required and unavailable native app notification rows
  unscheduled.
- Preserve all provider, receipt, runtime, UI, child delivery, adapter, broad
  blocking, and platform non-claims.

## Non-Goals

- Native app notification provider delivery.
- Production local scheduler runtime, retry workers, or quiet-hours timers.
- Parent notification UI or child-device delivery.
- Policy evaluator execution, broad app blocking, adapter dispatch, or
  platform support.

## Current Code Audit (2026-08-15)

The shared Rust owner already has a safe per-record canonical scheduler route,
an atomic scheduler proof store, and tests for due/manual/unavailable mapping,
reopen, idempotent replay, and identity conflicts. It does not yet consume the
WP58 bridge read model, preserve blocked WP58 rows in a WP59 result, or
round-trip scheduler JSONL. Historical parent-domain and proof paths are absent.

## Proof

- Shared source:
  `packages/parent-domain/src/app-game-notification-scheduler-bridge.ts`
- Shared test:
  `packages/parent-domain/tests/app-game-notification-scheduler-bridge.test.ts`
- Harness:
  `scripts/test/app-game-notification-scheduler-bridge-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/59-notification-scheduler-bridge/`

## Validation

- [ ] Cross-recorded from shared app/game WP59 proof.
- [ ] Native app rows schedule only after local outbox eligibility is proved.
- [ ] Manual-required and unavailable rows remain unscheduled.
- [ ] Runtime/provider/UI/child/adapter/platform claims remain false.
