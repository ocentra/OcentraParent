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

## Current Code and Test State (2026-08-15)

- `app_game_child_ux_provider_preflight` validates one canonical scheduler row
  against its persisted local-outbox source record and rejects identity,
  evidence, and unsafe-claim mismatches.
- Due-local rows become provider-adapter-required only when adapter,
  credential, and smoke-proof requirement refs are present; manual and
  unavailable scheduler rows remain blocked.
- Focused Rust contract tests cover provider-required, manual, unavailable,
  unpersisted, mismatched, claimed, and missing-requirement paths.
- The shared WP61 bridge consumes and validates the complete WP59 read model,
  requires each scheduled row to match the actual durable scheduler store,
  generates deterministic adapter/credential/smoke requirement refs, and
  retains manual/unavailable rows as blocked.
- Focused tests reject unpersisted, tampered, duplicate, claimed, and mismatched
  input. Historical `packages/parent-domain` and proof-harness paths remain
  absent.

## Proof

- Current implementation:
  `crates/app-game-core/src/app_game_child_ux_provider_preflight.rs`
- Current types:
  `crates/app-game-core/src/app_game_child_ux_provider_preflight_types.rs`
- Current focused tests:
  `crates/app-game-core/tests/contract/app_game_child_ux_outbox.rs`
- Shared bridge:
  `crates/app-game-core/src/app_game_notification_provider_preflight_bridge.rs`
- Shared bridge tests:
  `crates/app-game-core/tests/contract/app_game_notification_provider_preflight_bridge.rs`
- Native app proof pack:
  `output/app-plan-proof/61-notification-provider-preflight/`

## Validation

- [x] Shared app/game WP61 code/test checkpoint cross-recorded; retained proof remains open.
- [x] Native app rows require provider adapter setup only after scheduler proof.
- [x] Manual-required and unavailable rows remain blocked before provider
      preflight.
- [ ] Delivery/provider credential/runtime/UI/child/adapter/platform claims
      remain false.
