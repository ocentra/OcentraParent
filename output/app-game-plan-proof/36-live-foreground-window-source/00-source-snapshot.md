# WP36 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`
Base main: `1a7edd7e5f89bcbe7c930c66657a734245801798`

Scope inspected:

- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_source.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_source_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/constants.rs`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`

Before WP36, live process runtime capture was proved through core and service
runtime rows, but foreground evidence still lacked a live active-window source.

WP36 adds a bounded Rust core source that maps active-window foreground metadata
into existing app/game foreground evidence rows and journal events. Window
identity and title evidence are represented as opaque refs, not raw titles,
paths, or content.
