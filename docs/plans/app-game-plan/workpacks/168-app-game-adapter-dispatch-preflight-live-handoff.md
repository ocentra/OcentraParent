# WP168 App/Game Adapter Dispatch Preflight Live Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP168 App/Game Adapter Dispatch Preflight Live Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Connect the live app/game adapter execution readiness surface to a
parent-visible dispatch preflight model without claiming broad adapter
execution.

## Implementation

- Add a TypeScript agent-protocol-domain parser and package export for
  `agent.activity.app-game.adapter-dispatch-preflight.read-model.reported`.
- Add Rust protocol command/event names and dispatch-preflight read-model
  structs.
- Add an agent-service WebSocket command handler for
  `agent.activity.app-game.adapter-dispatch-preflight.read-model.get`.
- Derive eight dispatch preflight rows from the adapter execution readiness
  model and the existing V0.8 policy dispatch spine.
- Mark only the scoped Windows owned-process app/game time-limit row as
  dispatch eligible; keep all broad, degraded, unavailable, unsupported, and
  manual-required rows blocked before dispatch.
- Add a portal-domain parent-safe panel intent, command button contract, and
  portal live-state parser for the reported event.

## Proof

- `scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`
- `test-results/app-game-adapter-dispatch-preflight-live-handoff-proof/proof.json`

## Non-Claims

- Adapter dispatch execution remains unclaimed; the row is dispatch eligible
  only.
- Broad installed-app blocking execution remains unclaimed.
- Platform enforcement outside scoped Windows owned-process time-limit remains
  unclaimed.
- Provider delivery and provider receipt ingestion remain unclaimed.
- Child-device runtime delivery remains unclaimed.
- Raw private source rows, raw target values, and private diagnostics remain
  unclaimed.

## Product Doc Decision

`docs/features/app-game-control.md`,
`docs/plans/app-game-plan/implementation-checklist.md`, and this workpack index
record the dispatch-preflight progress. The central product capability
checklist remains intentionally untouched because another lane owns checklist
churn.
