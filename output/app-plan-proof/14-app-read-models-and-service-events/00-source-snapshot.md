# Source Snapshot

- Generated: 2026-06-03.
- Lane: `codex-c`.
- Thread: `app-plan-c`.
- Branch: `codex/app-game-read-model-service-events`.
- Pre-commit head: `9c8423a`.
- `origin/main`: `cbd8e2a`.
- Merge base: `cbd8e2a`.
- Shared proof root: `output/app-game-plan-proof/15-read-models-and-service-events`.

## Before-State Gap

Native app activity rows were not backed by the typed app-game service projection.
The service could expose older summary state, but app-use rows did not carry
separate inventory, running, foreground, daily rollup, capability, and evidence
source counts from the staged app-game journal/SQLite projection.

## App-Specific Source Surface

- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-protocol/src/activity_surface.rs`
- `crates/agent-protocol/src/activity_surface_tests.rs`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store.rs`
- `crates/agent-core/src/activity_store/internals.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`
- `crates/agent-service/src/activity_surface_adapter.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use/source.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_store.rs`

The app plan intentionally reuses the shared app/game evidence spine instead of
creating a parallel native-app-only projection.
