# WP46 Source Snapshot

Branch: `codex/app-plan-evidence-control-continuation`

Scope:

- `crates/agent-service/src/activity_capture/app_game.rs`
- `crates/agent-service/src/activity_capture/capture_events.rs`
- `crates/agent-service/src/activity_capture/errors.rs`
- `crates/agent-service/src/activity_capture_tests.rs`
- `crates/agent-service/src/activity_capture_tests/freshness.rs`
- `crates/agent-service/src/activity_capture_tests/inventory.rs`
- `crates/agent-protocol/src/constants/activity_store.rs`
- `crates/agent-service/README.md`
- `crates/agent-core/README.md`
- `crates/agent-protocol/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/46-service-windows-registry-capture-bridge.md`

Summary:

WP46 cross-records the shared app/game service capture bridge for native app
inventory. Windows Uninstall registry rows enter the service journal/store path
as inventory-only evidence with hashed source/path refs.
