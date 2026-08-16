# 59. Notification Scheduler Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `59. Notification Scheduler Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Bridge app/game local outbox records into the existing parent-domain local
notification scheduler row schema so app/game alerts can prove deterministic
scheduler handoff without claiming provider delivery or production runtime.

## Scope

- Reuse WP58 `AppGameNotificationLocalOutboxBridgeReadModel` rows.
- Schedule only linked local outbox records.
- Keep manual-required and unavailable app/game notification rows unscheduled
  with explicit proof requirements.
- Serialize and reread scheduler rows through the existing
  `NotificationLocalOutboxSchedulerRecordSchema`.
- Preserve explicit false claims for provider delivery, receipt ingestion,
  credentials, cloud routing, parent notification UI, child delivery,
  retry-worker execution, quiet-hours timer execution, production durable
  storage, adapter dispatch, broad blocking, and platform support.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, receipt webhooks, or receipt ingestion.
- Production retry workers, production quiet-hours timers, or durable outbox
  database runtime.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Current Code Audit (2026-08-15)

- `app_game_child_ux_scheduler` already validates one canonical
  `NotificationLocalOutboxRecord` and maps an honest queued-local record to a
  `DueLocal` scheduler record while rejecting unsafe delivery/private-data
  claims.
- `AppGameChildUxSchedulerProofStore` already provides atomic persistence,
  reopen, exact-replay idempotency, and same-identity conflict rejection. Its
  persisted rows deliberately keep production durable-outbox, provider,
  receipt, cloud, UI, and private-metadata claims false.
- Existing child-UX contract tests exercise due, manual, unavailable,
  persistence, reopen, replay, and conflict behavior for individual canonical
  records.
- `build_app_game_notification_scheduler_bridge` now validates and consumes the
  WP58 `AppGameNotificationLocalOutboxBridgeReadModel`, schedules only linked
  records, and retains manual-required/unavailable rows as explicit blocked
  WP59 rows.
- The bridge serializes and parses deterministic scheduler JSONL and persists
  scheduled rows through `AppGameChildUxSchedulerProofStore`; focused tests
  cover reopen, exact replay idempotency, same-identity conflict rejection,
  tampered source counts/claims/identities, and the non-claim boundary.
- The implementation and focused Phase 2 gates are committed at `4cf6a11c9`.
- The historical parent-domain source/test/proof paths named below are absent;
  current ownership is Rust `app-game-core` plus the canonical agent-protocol
  scheduler schema.

## Implementation and proof routes

- `crates/app-game-core/src/app_game_notification_scheduler_bridge.rs`
- `crates/app-game-core/src/app_game_notification_scheduler_bridge_types.rs`
- `crates/app-game-core/tests/contract/app_game_notification_scheduler_bridge.rs`
- Historical `packages/parent-domain/...` and
  `scripts/test/app-game-notification-scheduler-bridge-proof.mjs` routes are
  absent and are not current implementation authority.
- `test-results/app-game-notification-scheduler-bridge-proof/proof.json`
- `output/app-game-plan-proof/59-notification-scheduler-bridge/`
- `output/app-plan-proof/59-notification-scheduler-bridge/`

## Validation

- [x] Bridge parses the WP58 app/game outbox bridge read model before
      scheduling.
- [x] Only linked local outbox records become scheduler JSONL rows.
- [x] Manual-required and unavailable rows remain unscheduled.
- [x] Scheduler JSONL rereads through the existing scheduler record parser.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      retry-worker/quiet-hours timer runtime, no parent UI, no child delivery,
      no adapter dispatch, no durable production outbox, and no platform claim.
