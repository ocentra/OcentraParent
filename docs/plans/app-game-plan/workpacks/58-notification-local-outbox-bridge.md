# 58. Notification Local Outbox Bridge

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-local-outbox-bridge`
- Scope: parent-domain app/game notification intent to local outbox bridge.

## Goal

Prove that validated app/game notification intents can be bridged into the
existing parent-owned local notification outbox record schema and deterministic
JSONL artifact without creating a second outbox truth or claiming provider
delivery/runtime support.

## In Scope

- Add a parent-domain bridge from `AppGameNotificationIntent` to the existing
  `NotificationLocalOutboxRecord` schema.
- Write and reread deterministic JSONL records for local-outbox-eligible
  time-limit and suspicious-unknown app/game notification intents.
- Keep manual-required and unavailable app/game intents visible in the bridge
  read model without queueing delivery records.
- Preserve explicit false claims for provider delivery, receipts, scheduler
  runtime, cloud routing, parent UI, child delivery, adapter dispatch, broad
  blocking, and platform support.
- Add focused contract tests and proof packs under both shared app/game and
  native app proof roots.

## Out Of Scope

- Durable production local outbox storage.
- Provider delivery, credentials, webhook receipts, or receipt ingestion.
- Quiet-hours timer execution, retry workers, or production scheduler runtime.
- Parent notification UI, preference UI, or history UI.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `packages/parent-domain/package.json` and `packages/parent-domain/README.md`
  updates, because E-B owns those locks during this slice.

## Proof

- `scripts/test/app-game-notification-local-outbox-bridge-proof.mjs`
- `output/app-game-plan-proof/58-notification-local-outbox-bridge`
- `output/app-plan-proof/58-notification-local-outbox-bridge`
- `test-results/app-game-notification-local-outbox-bridge-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers bridge source/test, proof harness, proof roots, product
      docs, and workpack docs.
- [x] Existing app/game notification intent contract and notification local
      outbox adapter proof inspected and reused.
- [x] Eligible app/game notification intents become existing local outbox
      records and round-trip through JSONL parsing.
- [x] Manual-required and unavailable app/game notification intents do not
      queue delivery records.
- [x] Proof pack records no provider delivery, no receipt ingestion, no
      scheduler runtime, no parent UI, no child delivery, no policy execution,
      no adapter dispatch, no broad blocking, and no platform support claim.
