# 56. Notification Service Read Model

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-service-read-model`
- Scope: service-backed app/game notification readiness read model.

## Goal

Expose a dedicated app/game notification readiness command/event from the Rust
service so downstream notification, policy, and parent surfaces can see whether
the existing app/game service read model has enough local evidence to form
parent notification intents, without claiming provider delivery, receipt
ingestion, child delivery, UI, adapter dispatch, broad blocking, or platform
support.

## In Scope

- Add the TypeScript agent-protocol schema/parser for
  `agent.activity.app-game.notification-readiness.read-model.reported`.
- Add Rust protocol DTOs/constants for the notification readiness read model.
- Add an agent-service payload/report builder backed by the existing
  `ActivityStore::app_game_service_read_model` path.
- Derive readiness rows for time-limit exceeded, approval request, suspicious
  unknown, manual-required, and unavailable app/game states.
- Add focused TS/Rust/service tests and deterministic proof packs under both
  shared app/game and native app proof roots.
- Keep provider delivery, provider receipts, local outbox runtime, scheduler
  runtime, adapter dispatch, parent UI, and child delivery claims pinned false.

## Out Of Scope

- Provider delivery, credentials, webhook receipts, or receipt ingestion.
- Durable production outbox storage, scheduler workers, or quiet-hours timers.
- Parent notification UI, preference UI, or history UI.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `crates/agent-service/README.md`; that file is locked by `codex-d` during
  this workpack and remains a follow-up doc update if primary wants it.

## Proof

- `scripts/test/app-game-notification-service-read-model-proof.mjs`
- `output/app-game-plan-proof/56-notification-service-read-model`
- `output/app-plan-proof/56-notification-service-read-model`
- `test-results/app-game-notification-service-read-model-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths except the D-owned service README.
- [x] Existing notification intent contract, policy readiness service model,
      app/game service read model, protocol-domain, Rust protocol, and
      WebSocket command routing inspected.
- [x] TypeScript parser rejects wrong events, invalid JSON, invalid payloads,
      invalid readiness states, and true delivery/runtime/UI/adapter claims.
- [x] Rust protocol DTO serializes readiness rows and preserves all no-claim
      booleans.
- [x] Agent-service WebSocket command reports notification readiness from the
      real app/game activity-store read model.
- [x] Proof pack records no provider delivery, no receipt ingestion, no local
      outbox runtime, no scheduler runtime, no parent UI, no child delivery, no
      policy execution, no adapter dispatch, and no platform support claim.
