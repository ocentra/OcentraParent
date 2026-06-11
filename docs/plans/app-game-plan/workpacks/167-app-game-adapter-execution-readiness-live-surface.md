# WP167 App/Game Adapter Execution Readiness Live Surface

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
