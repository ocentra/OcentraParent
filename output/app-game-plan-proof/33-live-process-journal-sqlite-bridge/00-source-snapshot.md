# WP33 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`

Base inspected: `origin/main` / `HEAD` at
`5cf8244c Merge pull request #267 from ocentra/codex/browser-enforcement-timer-recovery-proof`.

Current state: uncommitted C-lane WP29-WP33 continuation work in the codex-c
worktree. Existing unrelated local `.codex/` and `.playwright-cli/` artifacts
are intentionally excluded from this proof.

Source paths inspected or touched:

- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_source_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`
- `crates/agent-core/README.md`
- `crates/agent-protocol/src/constants.rs`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/33-live-process-journal-sqlite-bridge.md`

Before-state gap: WP32 proved the live process source could create runtime
records, but did not prove those live records could append to the encrypted
journal or replay into SQLite read-model rows.
