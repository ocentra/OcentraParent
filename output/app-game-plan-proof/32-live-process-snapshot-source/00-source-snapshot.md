# WP32 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`

Base inspected: `origin/main` / `HEAD` at
`5cf8244c Merge pull request #267 from ocentra/codex/browser-enforcement-timer-recovery-proof`.

Current state: uncommitted C-lane WP29-WP32 continuation work in the codex-c
worktree. Existing unrelated local `.codex/` and `.playwright-cli/` artifacts
are intentionally excluded from this proof.

Source paths inspected or touched:

- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source_tests.rs`
- `crates/agent-core/README.md`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/32-live-process-snapshot-source.md`

Before-state gap: WP08 had a staged process-runtime record adapter, but no real
process-table source produced those records from the host OS.
