# WP34 Source Snapshot

Cross-recorded from shared app/game WP34.

Branch: `codex/app-plan-evidence-control-continuation`

Base inspected: `origin/main` / `HEAD` at
`5cf8244c Merge pull request #267 from ocentra/codex/browser-enforcement-timer-recovery-proof`.

Native app source paths inspected or touched:

- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source.rs`
- `crates/agent-service/src/activity_capture.rs`
- `crates/agent-service/src/activity_capture_tests.rs`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/34-service-capture-app-game-live-process-bridge.md`

Before-state gap: native app runtime process rows could replay through core
journal/SQLite proof, but service capture did not yet store those runtime rows
for app-use/games read models.
