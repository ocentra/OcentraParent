# WP40 - App/Game Boundary Read-Model Event

## Scope

Add a dedicated backend/protocol app/game boundary read-model command/event for
staged authority and classifier rows already present in `AppGameServiceReadModel`.

The event reports evidence claim, identity, approval authority, approval
action-result, platform authority matrix, platform authority row, and AI
classifier result counts plus citation refs.

This workpack does not add portal rows, policy consumption, live
classifier/provider execution, adapter execution, broad blocking, or platform
support claims.

## Implementation

- Add TypeScript command/event names and `appGameBoundaryReadModel` payload
  parsing in `@ocentra-parent/agent-protocol-domain`.
- Mirror the command/event, payload field, and serde read-model structs in
  `crates/agent-protocol`.
- Add an `agent-service` payload builder that derives rows from the existing
  `AppGameServiceReadModel`.
- Wire `agent.activity.app-game.boundary.read-model.get` through the real
  WebSocket command dispatcher.
- Add focused TypeScript parser, Rust protocol, payload, and service WebSocket
  tests.

## Proof

- `cmd /c npm exec --workspace @ocentra-parent/agent-protocol-domain -- vitest run tests/contracts.test.ts tests/app-game-boundary-read-model.test.ts`
- `cargo test -p ocentra-parent-agent-protocol app_game_boundary_read_model`
- `cargo test -p ocentra-parent-agent-service app_game_boundary`
- `cargo fmt --check`

Proof artifacts live in:

```text
output/app-game-plan-proof/40-app-game-boundary-read-model-event
```

## No-Claim Boundaries

- The event is backend/protocol read-model proof only.
- It does not prove live model/provider execution.
- It does not render portal authority/classifier rows.
- It does not trigger policy decisions, adapter execution, broad app blocking,
  or platform support.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP40 adds a
dedicated backend event for staged boundary row counts and citations only;
product status should not move until live classifier/provider execution, policy
consumption, portal rows, adapter proof, and platform support exist.
