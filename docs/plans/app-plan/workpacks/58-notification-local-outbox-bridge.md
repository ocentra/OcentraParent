# 58. Notification Local Outbox Bridge

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-local-outbox-bridge`
- Scope: native-app cross-record of the shared app/game notification local
  outbox bridge.

## Goal

Cross-record the shared app/game WP58 bridge for native app control: validated
native app/game notification intents can become existing parent-owned local
outbox JSONL records only when local-outbox eligible, while manual-required and
unavailable intents remain unqueued and all provider/runtime claims stay false.

## In Scope

- Cross-record the parent-domain bridge from app/game notification intents to
  existing local outbox records.
- Record proof outputs under the native app proof root.
- Preserve explicit false claims for provider delivery, provider receipts,
  scheduler runtime, cloud routing, parent UI, child delivery, adapter dispatch,
  broad app blocking, and platform support.

## Out Of Scope

- Provider delivery or provider receipt ingestion.
- Durable production outbox/scheduler runtime.
- Parent notification UI, preference UI, or history UI.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `packages/parent-domain/package.json` and `packages/parent-domain/README.md`
  updates, because E-B owns those locks during this slice.

## Current Status - Phase 1/2 Complete; Phase 3 Open

Shared App/Game commit `b7c63a75b` supplies the Rust-owned native app/game
readiness-row bridge to canonical `NotificationLocalOutboxRecord` values. It
reuses the atomic local-outbox store, proves append/reopen/idempotent replay and
conflict rejection, keeps manual/unavailable rows unqueued, and round-trips only
linked rows through deterministic JSONL. The agent-service false positive that
treated the unrelated parent-preference setup outbox as WP58 runtime was removed.
Focused bridge/service tests, the complete app-game-core suites, Clippy,
architecture, Enforcer, and pre-commit passed. Phase 3 proof remains open.

## Proof

- `scripts/test/app-game-notification-local-outbox-bridge-proof.mjs`
- `output/app-plan-proof/58-notification-local-outbox-bridge`
- `output/app-game-plan-proof/58-notification-local-outbox-bridge`
- `test-results/app-game-notification-local-outbox-bridge-proof/proof.json`

## DONE Checklist

- [x] Cross-recorded from shared app/game WP58 without creating a separate
      native-app notification outbox schema.
- [x] Native app notification local-outbox rows stay backed by the shared
      app/game notification intent contract and existing parent-domain local
      outbox record parser.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      scheduler runtime, no parent UI, no child delivery, no policy execution,
      no broad app blocking, and no platform support claim.
