# 57. Policy Evaluator Service Read Model

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-evaluator-service`
- Scope: service-backed app/game policy evaluator read model.

## Goal

Expose a dedicated app/game policy evaluation command/event from the Rust
service so downstream consumers can see dry-run policy decisions derived from
the existing app/game policy readiness rows before any portal, notification,
timer, persistence, or adapter path depends on them.

## In Scope

- Add the TypeScript agent-protocol schema/parser for
  `agent.activity.app-game.policy-evaluation.read-model.reported`.
- Add Rust protocol DTOs/constants for dry-run policy evaluation rows.
- Add an agent-service payload/report builder backed by the existing
  app/game policy readiness read model and
  `ActivityStore::app_game_service_read_model`.
- Add focused TS/Rust/service tests and proof outputs.
- Keep `dryRun=true`, `enforcementHandoffState=disabled`, and
  `adapterDispatchClaimed=false`.
- Keep block-launch and missing classifier/platform states manual-required.

## Out Of Scope

- Portal UI rendering.
- Parent rule authoring or durable policy persistence.
- Notification delivery or child request UX.
- Timer execution, bonus time, rollback, or enforcement handoff.
- Adapter dispatch, broad installed-app blocking, or platform support claims.

## Proof

- `scripts/test/app-game-policy-evaluator-service-proof.mjs`
- `output/app-game-plan-proof/57-policy-evaluator-service-read-model`
- `output/app-plan-proof/57-policy-evaluator-service-read-model`
- `test-results/app-game-policy-evaluator-service-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] Existing policy target compiler, policy readiness read model, service API,
      WebSocket routing, and app/game service-store source inspected.
- [x] TypeScript contract/parser rejects wrong events, invalid JSON, invalid
      payloads, `adapterDispatchClaimed=true`, and `dryRun=false`.
- [x] Rust protocol DTO serializes evaluation rows and keeps
      `adapterDispatchClaimed=false`.
- [x] Agent-service WebSocket command reports dry-run evaluation rows from the
      real app/game activity-store read model through the existing readiness
      model.
- [x] Proof pack records no portal UI, no parent rule persistence, no
      notification delivery, no timer/enforcement handoff, no adapter dispatch,
      and no platform support claim.
