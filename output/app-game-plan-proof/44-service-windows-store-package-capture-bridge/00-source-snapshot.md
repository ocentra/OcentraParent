# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base during proof: `origin/main` at `3c0d90f6`
- Scope: service bridge for bounded Windows packaged-app manifest inventory.
- Touched source:
  - `crates/agent-core/src/activity_store_app_game.rs`
  - `crates/agent-core/src/lib.rs`
  - `crates/agent-service/src/activity_capture.rs`
  - `crates/agent-service/src/activity_capture/app_game.rs`
  - `crates/agent-service/src/activity_capture/capture_events.rs`
  - `crates/agent-service/src/activity_capture_tests.rs`
  - `crates/agent-service/src/activity_capture_tests/freshness.rs`
  - `crates/agent-service/src/activity_capture_tests/inventory.rs`
  - `crates/agent-protocol/src/constants/activity_store.rs`
- Existing boundary: WP43 already proved core `AppxManifest.xml` parsing and
  inventory-only journal events.

## Current State

Agent-service activity capture now includes bounded Windows packaged-app
manifest inventory events in the same encrypted journal/store/read-model path
used for process, foreground, network, and shortcut inventory evidence.
