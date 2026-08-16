# WP209 App/Game Child Runtime Transport Receipt Service

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP209 App/Game Child Runtime Transport Receipt Service`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Expose the app/game child runtime transport and receipt boundary through the
agent protocol and Rust service.

This makes the child runtime boundary service-visible without claiming runtime
delivery execution.

## Implementation

- Added
  `packages/agent-protocol-domain/src/app-game-child-runtime-transport-receipt.ts`.
- Added
  `packages/agent-protocol-domain/tests/app-game-child-runtime-transport-receipt.test.ts`.
- Added Rust protocol structs and command/event registration in
  `crates/agent-protocol`.
- Added Rust service payload and WebSocket command handling in
  `crates/agent-service`.
- Added `scripts/test/app-game-child-runtime-transport-receipt-service-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-child-runtime-transport-receipt-service-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-child-runtime-transport-receipt
cargo test -p ocentra-parent-agent-protocol app_game_child_runtime_transport_receipt
cargo test -p ocentra-parent-agent-service app_game_child_runtime_transport_receipt
cmd /c node scripts/test/app-game-child-runtime-transport-receipt-service-proof.mjs
```

## Proof

- `test-results/app-game-child-runtime-transport-receipt-service-proof/proof.json`
- `output/app-game-plan-proof/209-app-game-child-runtime-transport-receipt-service/proof.json`
- `output/app-game-plan-proof/209-app-game-child-runtime-transport-receipt-service/00-source-snapshot.md`
- `output/app-game-plan-proof/209-app-game-child-runtime-transport-receipt-service/10-validation-commands.log`

## Boundaries

Proved:

- The command
  `agent.activity.app-game.child-runtime-transport-receipt.read-model.get`
  is registered in TypeScript and Rust protocol contracts.
- The service reports
  `agent.activity.app-game.child-runtime-transport-receipt.read-model.reported`
  through the WebSocket command handler.
- The payload field
  `appGameChildRuntimeTransportReceiptReadModel` is schema-validated by the
  TypeScript protocol parser.
- Runtime transport execution, receipt ingestion, provider delivery, platform
  delivery channel, adapter dispatch, platform enforcement, and raw private
  source rows remain unclaimed.

Not proved:

- Child runtime transport execution.
- Child runtime receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
