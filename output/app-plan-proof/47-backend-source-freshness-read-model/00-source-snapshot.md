# WP47 Source Snapshot

- Lane: `codex-c`
- Branch: `codex/app-plan-evidence-control-continuation`
- Pre-commit head: `5af23f08`
- Workpack: backend source freshness read-model rows
- Cross-recorded from shared app/game WP47 for the native app plan.

## Source Files

- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `crates/agent-protocol/src/activity_surface.rs`
- `crates/agent-protocol/src/activity_surface_tests.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/app_game_source_status_tests.rs`

## Boundary

This slice exposes typed backend `sourceStatusRows` for existing app-use
read-model rows. It does not add portal rendering, policy consumption, adapter
execution, broad app blocking, or platform support.
