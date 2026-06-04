# WP38 Native App Source Snapshot

## Scope

Cross-record of shared app/game WP38 for the native app plan. Existing
app-use/games read-model rows now preserve staged app/game boundary refs through
their evidence vectors.

## Touched Source

- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_game_boundary_evidence_tests.rs`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/38-service-authority-classifier-surface-evidence.md`

## Explicit Non-Touched Scope

- No native-app portal UI.
- No policy evaluator or adapter execution files.
- No `docs/product-capability-checklist.md` edit.
