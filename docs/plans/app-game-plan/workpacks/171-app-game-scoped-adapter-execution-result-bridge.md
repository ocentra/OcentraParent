# WP171 App/Game Scoped Adapter Execution Result Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP171 App/Game Scoped Adapter Execution Result Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Bridge the WP170 app/game dispatch-result row to real
`agent.enforcement.execute` audit evidence for the single scoped Windows
owned-process app/game time-limit boundary. The service read-model getter stays
side-effect-free: it reports execution evidence as missing until a real
`agent.enforcement.audit.reported` payload is attached by the proof/runtime
handoff.

## Implementation

- Extend the app/game adapter dispatch result contract with adapter execution
  state, decision, result id, status, adapter result code, audit event id, refs,
  and aggregate counts.
- Extend Rust protocol structs/constants with the same adapter execution result
  fields.
- Add a service bridge that parses real enforcement audit payload fields and
  attaches them to the scoped owned-process app/game timer row.
- Keep the default read-model command side-effect-free: it marks the accepted
  scoped row as execution-evidence-missing until execution evidence is supplied.
- Keep broad installed-app blocking, degraded adapter dependency, unavailable,
  unsupported, Android/iOS manual, provider delivery, child delivery, platform
  enforcement, raw source rows, raw target values, and private diagnostics
  unclaimed.
- Render adapter execution result/status/refs in the existing portal-domain
  app/game adapter dispatch result panel.

## Proof

- `scripts/test/app-game-scoped-adapter-execution-result-bridge-proof.mjs`
- `test-results/app-game-scoped-adapter-execution-result-bridge-proof/proof.json`

The proof runs the real `agent.enforcement.execute` service test that records
enforcement audit events to journal and store, plus the app/game contract,
Rust protocol, service payload, and portal-domain panel tests.

## Non-Claims

- Broad installed-app blocking execution remains unclaimed.
- Platform enforcement outside the scoped Windows owned-process time-limit
  boundary remains unclaimed.
- Provider delivery and provider receipt ingestion remain unclaimed.
- Child-device runtime delivery remains unclaimed.
- Raw private source rows, raw target values, and private diagnostics remain
  unclaimed.

## Product Doc Decision

`docs/features/app-game-control.md`,
`docs/plans/app-game-plan/implementation-checklist.md`, and this workpack index
record the scoped adapter execution result bridge. The central product
capability checklist remains intentionally untouched because another lane owns
checklist churn.
