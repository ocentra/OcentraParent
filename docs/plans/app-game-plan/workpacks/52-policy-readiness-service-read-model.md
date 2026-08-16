# 52. Policy Readiness Service Read Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `52. Policy Readiness Service Read Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [ ] Existing app/game boundary read-model and service-store source inspected.
- [ ] TypeScript contract/parser rejects wrong events, invalid JSON, invalid
      payloads, and `adapterDispatchClaimed=true`.
- [ ] Rust protocol DTO serializes readiness rows and keeps
      `adapterDispatchClaimed=false`.
- [ ] Agent-service WebSocket command reports readiness from the real app/game
      activity-store read model.
- [ ] Proof pack records no portal UI, no product checklist change, no policy
      execution, no adapter dispatch, and no platform support claim.
