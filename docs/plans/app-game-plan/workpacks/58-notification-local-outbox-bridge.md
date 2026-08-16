# 58. Notification Local Outbox Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `58. Notification Local Outbox Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

## Historical Code Audit And Repair (2026-08-15)

- The audit found that `build_activity_app_game_notification_readiness_report`
  derived `local_outbox_runtime_claimed` from the separate WP125
  parent-preference setup outbox. That false coupling and its dead probe were
  removed in commit `b7c63a75b`; the service now keeps the runtime claim false
  until a real service composition owns the WP58 store.
- Commit `b7c63a75b` adds the Rust-owned WP58 bridge from validated
  notification-readiness rows to canonical `NotificationLocalOutboxRecord`
  values and reuses the atomic WP121 local-outbox store. Eligible rows persist,
  reopen, replay idempotently, and reject same-identity conflicts;
  manual-required and unavailable rows remain unqueued.
- The named proof script, generated proof root, and test-results artifact are
  absent from the tracked checkout. They cannot support the historical DONE
  wording.
- WP121 supplies a typed atomic child-UX local-outbox store and canonical
  `NotificationLocalOutboxRecord` use. WP58 now reuses that storage truth and
  also exposes deterministic JSONL serialization/parsing for the bounded bridge
  artifact; it does not create a second store format.

## Production boundary follow-up - 2026-08-16

- `crates/app-game-core/src/app_game_notification_local_outbox_bridge.rs` now
  validates the complete bridge read model before returning it and before any
  local-outbox persistence side effect.
- `crates/app-game-core/src/app_game_notification_local_outbox_bridge_read_model_validation.rs`
  now rejects malformed source rows, mismatched bridge/entry/alert identities,
  provider or scheduler claims, and non-local delivery record state. This keeps
  a caller-supplied read model from minting a local outbox record outside the
  WP58 bridge contract.
- This follow-up is production code drafted only in the global code-writing
  phase. Focused tests, validation, Enforcer gates, and retained proof remain
  deferred; provider delivery, receipts, scheduler runtime, UI, child delivery,
  adapter dispatch, and real service composition remain outside this workpack.

## Current Status - Production code drafted; tests and proof deferred

- The historical Phase 1/2 execution record remains retained in repository
  history, but it does not validate the production-boundary follow-up above.
- The historical proof script/output/test-results artifacts remain absent and
  must be regenerated only during Phase 3. Provider delivery, receipt
  ingestion, scheduler runtime, parent UI, child delivery, adapter dispatch,
  broad blocking, and platform support remain unclaimed.

## Proof

- `scripts/test/app-game-notification-local-outbox-bridge-proof.mjs`
- `output/app-game-plan-proof/58-notification-local-outbox-bridge`
- `output/app-plan-proof/58-notification-local-outbox-bridge`
- `test-results/app-game-notification-local-outbox-bridge-proof/proof.json`

## DONE Checklist

- [ ] Hub lock covers bridge source/test, proof harness, proof roots, product
      docs, and workpack docs.
- [x] Existing app/game notification intent contract and notification local
      outbox adapter proof inspected and reused.
- [x] Eligible app/game notification intents become existing local outbox
      records and round-trip through JSONL parsing.
- [x] Manual-required and unavailable app/game notification intents do not
      queue delivery records.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      scheduler runtime, no parent UI, no child delivery, no policy execution,
      no adapter dispatch, no broad blocking, and no platform support claim.
