# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base: `86214bb294a0a8dc5f9a79bb72410bc3a5c36f31`
- Workpack: WP42 service Windows inventory capture bridge
- Primary source files:
  - `crates/agent-protocol/src/constants/activity_capture.rs`
  - `crates/agent-core/src/activity_store_app_game.rs`
  - `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_source.rs`
  - `crates/agent-core/src/lib.rs`
  - `crates/agent-service/src/activity_capture.rs`
  - `crates/agent-service/src/activity_capture/app_game.rs`
  - `crates/agent-service/src/activity_capture_tests.rs`
  - `crates/agent-service/src/activity_capture_tests/inventory.rs`
  - `crates/agent-service/src/activity_capture_tests/freshness.rs`
- Product docs:
  - `docs/features/app-game-control.md`
  - `docs/plans/app-game-plan/current-app-game-snapshot.md`
  - `docs/plans/app-game-plan/implementation-checklist.md`
  - `docs/plans/app-game-plan/workpacks/42-service-windows-inventory-capture-bridge.md`
- Checklist decision: `docs/product-capability-checklist.md` intentionally unchanged.

## Scope

Service capture bridge for bounded live Windows shortcut inventory journal
events. The existing encrypted journal, ActivityStore, and app/game read-model
path project those rows as inventory-only evidence.

## No-Claim Scope

No registry crawling, Store package enumeration, portal UI, policy consumption,
adapter execution, broad app blocking, or platform support claim.
