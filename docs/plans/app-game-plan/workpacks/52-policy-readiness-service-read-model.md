# 52. Policy Readiness Service Read Model

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-readiness-service`
- Scope: service-backed app/game policy readiness read model.

## Goal

Expose a dedicated app/game policy readiness command/event from the Rust service
so downstream policy consumers can see whether the existing app/game service
model contains the required evidence, approval authority, and platform authority
rows before any policy evaluator or adapter path is allowed to consume it.

## In Scope

- Add the TypeScript agent-protocol schema/parser for
  `agent.activity.app-game.policy-readiness.read-model.reported`.
- Add Rust protocol DTOs/constants for the readiness read model.
- Add an agent-service payload/report builder backed by
  `ActivityStore::app_game_service_read_model`.
- Add focused TS/Rust/service tests and proof outputs.
- Keep missing prerequisites visible as `missing` or `manual-required` rows.

## Out Of Scope

- Portal UI rendering.
- Product capability checklist edits.
- Policy evaluator execution.
- Notifications or child request UX.
- Adapter dispatch, broad installed-app blocking, or platform support claims.

## Proof

- `scripts/test/app-game-policy-readiness-service-proof.mjs`
- `output/app-game-plan-proof/52-policy-readiness-service-read-model`
- `output/app-plan-proof/52-policy-readiness-service-read-model`
- `test-results/app-game-policy-readiness-service-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] Existing app/game boundary read-model and service-store source inspected.
- [x] TypeScript contract/parser rejects wrong events, invalid JSON, invalid
      payloads, and `adapterDispatchClaimed=true`.
- [x] Rust protocol DTO serializes readiness rows and keeps
      `adapterDispatchClaimed=false`.
- [x] Agent-service WebSocket command reports readiness from the real app/game
      activity-store read model.
- [x] Proof pack records no portal UI, no product checklist change, no policy
      execution, no adapter dispatch, and no platform support claim.
