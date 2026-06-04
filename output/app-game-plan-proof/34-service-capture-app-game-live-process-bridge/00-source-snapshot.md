# WP34 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`

Base inspected: `origin/main` / `HEAD` at
`5cf8244c Merge pull request #267 from ocentra/codex/browser-enforcement-timer-recovery-proof`.

Current state: uncommitted C-lane WP29-WP34 continuation work in the codex-c
worktree. Existing unrelated local `.codex/` and `.playwright-cli/` artifacts
are intentionally excluded from this proof.

Source paths inspected or touched:

- `crates/agent-core/src/lib.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source_tests.rs`
- `crates/agent-service/src/activity_capture.rs`
- `crates/agent-service/src/activity_capture_tests.rs`
- `crates/agent-protocol/src/constants/activity_store.rs`
- `crates/agent-protocol/src/constants/value.rs`
- `crates/agent-core/README.md`
- `crates/agent-service/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/34-service-capture-app-game-live-process-bridge.md`

Before-state gap: WP33 proved a core live process source could append/replay
app/game runtime rows through the local journal and SQLite read model, but the
service capture path did not record those app/game runtime rows.
