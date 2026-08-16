# 56. Notification Service Read Model

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-service-read-model`
- Scope: native-app cross-record of the shared app/game notification readiness
  service read model.

## Goal

Cross-record the shared app/game WP56 service read model for native app control:
the Rust service exposes a dedicated notification readiness command/event that
derives native app time-limit, approval-request, suspicious-unknown,
manual-required, and unavailable notification intent readiness from the existing
app/game service read model without claiming provider delivery, parent UI, child
delivery, policy execution, broad app blocking, or platform support.

## In Scope

- Cross-record the TypeScript agent-protocol parser and Rust protocol DTOs for
  native app notification readiness rows.
- Cross-record the agent-service WebSocket command/report path backed by the
  existing app/game ActivityStore read model.
- Record proof outputs under the native app proof root.
- Preserve explicit false claims for provider delivery, provider receipts,
  local outbox runtime, scheduler runtime, adapter dispatch, parent UI, and
  child delivery.

## Out Of Scope

- Provider delivery or provider receipt ingestion.
- Parent notification UI, preference UI, or history UI.
- Durable production outbox/scheduler runtime.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `crates/agent-service/README.md`; that file is locked by `codex-d` during
  this workpack and remains a follow-up doc update if primary wants it.

## Proof

- `scripts/test/app-game-notification-service-read-model-proof.mjs`
- `output/app-plan-proof/56-notification-service-read-model`
- `output/app-game-plan-proof/56-notification-service-read-model`
- `test-results/app-game-notification-service-read-model-proof/proof.json`

## DONE Checklist

- [ ] Cross-recorded from shared app/game WP56 without creating a separate
      native-app notification protocol.
- [ ] Native app notification readiness rows stay backed by the shared
      app/game service read model and protocol constants.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no local
      outbox runtime, no scheduler runtime, no parent UI, no child delivery, no
      policy execution, no broad app blocking, and no platform support claim.
