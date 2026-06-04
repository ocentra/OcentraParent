# WP31 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`

Base inspected: `origin/main` / `HEAD` at
`5cf8244c main advanced after PR267 browser/enforcement timer recovery merge`.

Current state: uncommitted C-lane WP29, WP30, and WP31 continuation work in the
codex-c worktree. Existing unrelated local `.codex/` and `.playwright-cli/`
artifacts are intentionally excluded from this proof.

Source paths inspected or touched:

- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_authority_classifier.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/protocol_rows.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_protocol_rows_tests.rs`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/31-journal-sqlite-authority-classifier-storage.md`

Before-state gap: WP29/WP30 gave Rust protocol parity for evidence, identity,
authority, action-result, platform authority, and classifier rows, but the
existing app/game journal and SQLite projection stored only inventory, runtime,
foreground, launcher, and daily rollup rows.
