# 60. Notification Audit-History Bridge

## Goal

Cross-record the shared app/game audit-history bridge for native app
notification handoff so native app alerts can prove metadata-only audit rows
after local outbox linking.

## Scope

- Reuse the shared app/game WP60 audit-history handoff.
- Map linked local outbox rows from native app/game notification intents into
  queued logging-domain audit-history entries.
- Keep manual-required and unavailable native app notification rows blocked and
  audit-visible without queued provider sends.
- Preserve all provider, receipt, runtime, UI, child delivery, adapter, broad
  blocking, and platform non-claims.

## Non-Goals

- Native app notification provider delivery.
- Production retry workers, quiet-hours timers, or durable outbox/history
  runtime.
- Parent notification history/preferences UI or child-device delivery.
- Policy evaluator execution, broad app blocking, adapter dispatch, or platform
  support.

## Current Code Audit (2026-08-15)

The shared Rust owner now validates and consumes WP58, records ordered
queued/manual/unavailable metadata entries, preserves audit/evidence/policy
refs for linked and blocked rows, and round-trips deterministic JSONL. Focused
tests reject tampered refs/claims/identities and preserve explicit runtime,
provider, UI, child, adapter, and platform non-claims. Code and focused Phase 2
gates are committed at `bae505ce8`; durable production history/query remains a
later boundary.

## Proof

- Shared source:
  `crates/app-game-core/src/app_game_notification_audit_history_bridge.rs`
- Shared test:
  `crates/app-game-core/tests/contract/app_game_notification_audit_history_bridge.rs`
- Historical `packages/logging-domain/...` and script harness routes are absent.
- Native app proof pack:
  `output/app-plan-proof/60-notification-audit-history-bridge/`

## Validation

- [ ] Cross-recorded from shared app/game WP60 proof.
- [x] Native app rows become audit-history entries only after local outbox
      eligibility or manual/unavailable status is parsed.
- [x] Manual-required and unavailable rows remain blocked/manual without
      provider sends.
- [x] Runtime/provider/UI/child/adapter/platform claims remain false.
