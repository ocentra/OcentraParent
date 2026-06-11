# WP168 App/Game Adapter Dispatch Preflight Live Handoff

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
