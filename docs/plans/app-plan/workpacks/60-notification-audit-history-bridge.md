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

## Proof

- Shared source:
  `packages/logging-domain/src/notification-audit-history-handoff.ts`
- Shared test:
  `packages/logging-domain/tests/notification-audit-history-handoff.test.ts`
- Harness:
  `scripts/test/app-game-notification-audit-history-bridge-proof.mjs`
- Native app proof pack:
  `output/app-plan-proof/60-notification-audit-history-bridge/`

## Validation

- [x] Cross-recorded from shared app/game WP60 proof.
- [x] Native app rows become audit-history entries only after local outbox
      eligibility or manual/unavailable status is parsed.
- [x] Manual-required and unavailable rows remain blocked/manual without
      provider sends.
- [x] Runtime/provider/UI/child/adapter/platform claims remain false.
