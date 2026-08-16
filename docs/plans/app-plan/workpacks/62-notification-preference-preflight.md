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

## Current Code Audit (2026-08-15)

- `app_game_child_ux_preference_preflight` validates one scheduler row against
  its persisted local-outbox source record and rejects identity, evidence, and
  unsafe-claim mismatches.
- Due-local rows require distinct parent-preference, notification-frequency,
  and quiet-hours refs; manual and unavailable states remain blocked.
- Focused Rust contract tests cover due/manual/unavailable, unpersisted,
  mismatched, claimed, and duplicate-requirement paths.
- The shared WP62 bridge now consumes the complete WP59 read model, verifies
  exact scheduled rows against the durable scheduler store, generates
  deterministic requirements, and retains blocked rows. This production code
  was drafted at `a93b45f33`; dedicated bridge tests and all execution/
  validation are intentionally deferred to later global phases.

## Proof

- Current implementation:
  `crates/app-game-core/src/app_game_child_ux_preference_preflight.rs`
- Current types:
  `crates/app-game-core/src/app_game_child_ux_preference_preflight_types.rs`
- Shared bridge:
  `crates/app-game-core/src/app_game_notification_preference_preflight_bridge.rs`
- Shared bridge types:
  `crates/app-game-core/src/app_game_notification_preference_preflight_bridge_types.rs`
- Current focused tests:
  `crates/app-game-core/tests/contract/app_game_child_ux_outbox.rs`
- Native app proof pack:
  `output/app-plan-proof/62-notification-preference-preflight/`

## Validation

- [ ] Cross-recorded from shared app/game WP62 proof.
- [ ] Native app rows require parent preference/frequency/quiet-hours proof only
      after scheduler proof.
- [ ] Manual-required and unavailable rows remain blocked before preference
      preflight.
- [ ] UI/delivery/provider credential/runtime/child/adapter/platform claims
      remain false.
