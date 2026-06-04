# WP36 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`
Base main: `1a7edd7e5f89bcbe7c930c66657a734245801798`

This native app proof cross-records shared app/game WP36.

Source inspected:

- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_source.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_source_tests.rs`
- `crates/agent-protocol/src/app_game.rs`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/36-live-foreground-window-source.md`

WP36 proves core active-window foreground evidence production for native
app/game rows with opaque window/title refs. It does not prove service capture,
portal foreground freshness, policy execution, adapter execution, or platform
support.
