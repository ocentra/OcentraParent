# WP37 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`
Base main: `1a7edd7e5f89bcbe7c930c66657a734245801798`

Scope inspected:

- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_source.rs`
- `crates/agent-core/src/lib.rs`
- `crates/agent-service/src/activity_capture.rs`
- `crates/agent-service/src/activity_capture_tests.rs`
- `crates/agent-service/src/activity_capture_tests/freshness.rs`
- `docs/plans/app-game-plan/workpacks/37-service-foreground-capture-bridge.md`

WP37 wires the WP36 core foreground source into the existing bounded
`agent-service` activity-capture event list. The bridge appends an app/game
foreground journal event only when the active-window source is available.
