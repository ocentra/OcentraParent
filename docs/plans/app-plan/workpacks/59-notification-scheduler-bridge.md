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

The shared Rust owner now validates and consumes the WP58 bridge read model,
schedules only linked rows, preserves blocked manual/unavailable rows, and
round-trips deterministic scheduler JSONL. It reuses the atomic scheduler store
and has focused tests for reopen, idempotent replay, identity conflict, source
tampering, and explicit runtime/provider/UI non-claims. The code and focused
Phase 2 gates are committed at `4cf6a11c9`; historical parent-domain routes are
absent and are not implementation authority.

2026-08-16 production-code follow-up: the public scheduler persistence entry
point now invokes the canonical scheduler read-model validator before writing
any scheduler records. Invalid counts, row identities, blocked-row shapes, or
runtime/provider claims fail as invalid data; no scheduler/provider runtime
authority is added.

## Proof

- Shared source:
  `crates/app-game-core/src/app_game_notification_scheduler_bridge.rs`
- Shared test:
  `crates/app-game-core/tests/contract/app_game_notification_scheduler_bridge.rs`
- Historical `packages/parent-domain/...` and script harness routes are absent.
- Native app proof pack:
  `output/app-plan-proof/59-notification-scheduler-bridge/`

## Validation

- [ ] Cross-recorded from shared app/game WP59 proof.
- [x] Native app rows schedule only after local outbox eligibility is proved.
- [x] Manual-required and unavailable rows remain unscheduled.
- [x] Runtime/provider/UI/child/adapter/platform claims remain false.
