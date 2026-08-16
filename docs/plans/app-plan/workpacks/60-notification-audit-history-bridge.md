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

The former graph roots point at the later app/game notification parent-surface
intent, which does not consume WP58 or create audit-history entries. The named
`packages/logging-domain` source/test owner and proof harness are absent. No
current notification-specific queued/manual/unavailable audit-history read
model, ref-preservation test, or deterministic JSONL handoff exists. Generic
logging and enforcement history are separate owners and do not close WP60.

## Proof

- Planned shared source:
  `crates/app-game-core/src/app_game_notification_audit_history_bridge.rs`
- Planned shared test:
  `crates/app-game-core/tests/contract/app_game_notification_audit_history_bridge.rs`
- Historical `packages/logging-domain/...` and script harness routes are absent.
- Native app proof pack:
  `output/app-plan-proof/60-notification-audit-history-bridge/`

## Validation

- [ ] Cross-recorded from shared app/game WP60 proof.
- [ ] Native app rows become audit-history entries only after local outbox
      eligibility or manual/unavailable status is parsed.
- [ ] Manual-required and unavailable rows remain blocked/manual without
      provider sends.
- [ ] Runtime/provider/UI/child/adapter/platform claims remain false.
