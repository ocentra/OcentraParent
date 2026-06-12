# WP169 App/Game Scoped Adapter Dispatch Command Result

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP169 App/Game Scoped Adapter Dispatch Command Result`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Move the WP168 dispatch-eligible preflight row into a parent-visible
dispatch command-result handoff without claiming broad app blocking or adapter
execution.

## Implementation

- Add a TypeScript agent-protocol-domain parser and package export for
  `agent.activity.app-game.adapter-dispatch-result.read-model.reported`.
- Add Rust protocol command/event names and dispatch-result read-model structs.
- Add an agent-service WebSocket command handler for
  `agent.activity.app-game.adapter-dispatch-result.read-model.get`.
- Derive dispatch-result rows from the dispatch preflight read model.
- Mark only the scoped Windows owned-process app/game time-limit row as an
  accepted command-result handoff to `agent.enforcement.execute` /
  `agent.enforcement.audit.reported`.
- Keep all broad, degraded, unavailable, unsupported, and manual-required rows
  blocked before command handoff.
- Add a portal-domain parent-safe panel intent, command button contract, and
  portal live-state parser for the reported event.

## Proof

- `scripts/test/app-game-scoped-adapter-dispatch-command-result-proof.mjs`
- `test-results/app-game-scoped-adapter-dispatch-command-result-proof/proof.json`

## Non-Claims

- Adapter dispatch execution remains unclaimed; this workpack only proves the
  scoped command-result handoff.
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
record the scoped command-result progress. The central product capability
checklist remains intentionally untouched because another lane owns checklist
churn.
