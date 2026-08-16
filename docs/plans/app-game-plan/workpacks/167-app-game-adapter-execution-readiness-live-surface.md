# WP167 App/Game Adapter Execution Readiness Live Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP167 App/Game Adapter Execution Readiness Live Surface`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Expose the WP166 app/game adapter execution readiness model through the live
agent/portal surface without upgrading any unsupported control claims.

## Implementation

- Add a TypeScript agent-protocol-domain parser and package export for
  `agent.activity.app-game.adapter-execution-readiness.read-model.reported`.
- Add Rust protocol command/event names and read-model structs.
- Add an agent-service WebSocket command handler for
  `agent.activity.app-game.adapter-execution-readiness.read-model.get`.
- Add a portal-domain parent-safe panel intent and command button contract.
- Add portal live-state parsing for the reported event.

## Proof

- `scripts/test/app-game-adapter-execution-readiness-live-surface-proof.mjs`
- `test-results/app-game-adapter-execution-readiness-live-surface-proof/proof.json`

## Non-Claims

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
record the live-surface progress. The central product capability checklist is
intentionally untouched because another lane owns that checklist churn.
