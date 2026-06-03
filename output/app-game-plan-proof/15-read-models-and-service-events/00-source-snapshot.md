# Source Snapshot

- Generated: 2026-06-03.
- Lane: `codex-c`.
- Thread: `app-plan-c`.
- Branch: `codex/app-game-read-model-service-events`.
- Pre-commit head: `9c8423a`.
- `origin/main`: `cbd8e2a`.
- Merge base: `cbd8e2a`.
- Working state at snapshot: tracked implementation, docs, and proof changes pending; unrelated untracked `.codex/` and `.playwright-cli/` artifacts were left untouched.

## Hub And Lock State

The slice was reported `STARTED` to the hub before edits. Locks covered the app-game read-model contracts, Rust protocol/core/service projection paths, the WP15 and WP14 plan docs, and the two proof roots.

Additional touched cleanup paths were locked before doc/proof completion:

- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-service/src/activity_surface_adapter.rs`
- `crates/agent-service/src/activity_surface_read_model_states.rs`

## Source Files Inspected

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundary-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-protocol-websocket-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-rust-service-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation-rules.mdc`
- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/15-read-models-and-service-events.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/14-app-read-models-and-service-events.md`
- `packages/activity-domain/README.md`
- `crates/agent-protocol/README.md`
- `crates/agent-core/README.md`
- `crates/agent-service/README.md`

## Before-State Gap

WP14 projected typed app/game inventory, runtime, foreground, launcher, and daily rollup rows from staged encrypted journal replay into SQLite. The service still mapped app-use and games tabs from older summary/report shapes, so portal consumers did not receive typed inventory/running/foreground/launcher/session/capability row state from the app-game service projection.

## Changed Source Surface

- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-protocol/src/activity_surface.rs`
- `crates/agent-protocol/src/activity_surface_tests.rs`
- `crates/agent-core/src/activity_store.rs`
- `crates/agent-core/src/activity_store/internals.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-service/src/activity_surface_adapter.rs`
- `crates/agent-service/src/activity_surface_read_model_states.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use/source.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_store.rs`
