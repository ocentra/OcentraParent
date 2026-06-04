# WP38 Source Snapshot

## Scope

Agent-service app-use/games read-model rows now carry refs for staged app/game
boundary rows already projected by `AppGameServiceReadModel`.

## Touched Source

- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_game_boundary_evidence_tests.rs`
- `crates/agent-service/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/38-service-authority-classifier-surface-evidence.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/38-service-authority-classifier-surface-evidence.md`

## Explicit Non-Touched Scope

- No portal UI files.
- No policy evaluator or adapter execution files.
- No `docs/product-capability-checklist.md` edit.
