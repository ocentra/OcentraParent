# Source Snapshot

WP40 adds a dedicated backend/protocol app/game boundary read-model
command/event.

Touched source:

- `packages/agent-protocol-domain/src/contracts.ts`
- `packages/agent-protocol-domain/src/defaults.ts`
- `packages/agent-protocol-domain/src/app-game-boundary-read-model.ts`
- `packages/agent-protocol-domain/tests/contracts.test.ts`
- `packages/agent-protocol-domain/tests/app-game-boundary-read-model.test.ts`
- `packages/agent-protocol-domain/package.json`
- `crates/agent-protocol/src/transport.rs`
- `crates/agent-protocol/src/constants.rs`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-protocol/src/app_game_boundary_read_model.rs`
- `crates/agent-protocol/src/app_game_boundary_read_model_tests.rs`
- `crates/agent-service/src/app_game_boundary_read_model_payload.rs`
- `crates/agent-service/src/app_game_boundary_read_model_payload_tests.rs`
- `crates/agent-service/src/app_game_boundary_read_model_service_tests.rs`
- `crates/agent-service/src/activity_api.rs`
- `crates/agent-service/src/websocket.rs`

The new `agent.activity.app-game.boundary.read-model.get` command reports a
service-backed `appGameBoundaryReadModel` payload with staged evidence-claim,
identity, approval authority/action-result, platform authority matrix/rows, and
AI classifier result counts plus citation refs.
