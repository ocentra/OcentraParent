# Source Snapshot

WP39 adds explicit staged boundary count fields to the existing app-use and
games activity-surface read-model rows.

Touched source:

- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-protocol/src/activity_surface.rs`
- `crates/agent-protocol/src/activity_surface_tests.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models/app_game_boundary_evidence_tests.rs`

The count fields cover evidence claim, identity, approval authority, approval
action result, platform authority matrix, platform authority row, and AI
classifier result rows already present in `AppGameServiceReadModel`.
