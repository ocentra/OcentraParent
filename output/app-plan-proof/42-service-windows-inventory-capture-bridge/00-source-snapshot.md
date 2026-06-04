# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base: `86214bb294a0a8dc5f9a79bb72410bc3a5c36f31`
- Workpack: App-plan WP42 service Windows inventory capture bridge
- Shared source files:
  - `crates/agent-protocol/src/constants/activity_capture.rs`
  - `crates/agent-core/src/activity_store_app_game.rs`
  - `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_source.rs`
  - `crates/agent-core/src/lib.rs`
  - `crates/agent-service/src/activity_capture.rs`
  - `crates/agent-service/src/activity_capture/app_game.rs`
  - `crates/agent-service/src/activity_capture_tests/inventory.rs`
- Product docs:
  - `docs/plans/app-plan/current-app-snapshot.md`
  - `docs/plans/app-plan/implementation-checklist.md`
  - `docs/plans/app-plan/workpacks/42-service-windows-inventory-capture-bridge.md`
- Checklist decision: `docs/product-capability-checklist.md` intentionally unchanged.

## Scope

Native app cross-record of the shared app/game service inventory capture bridge.

## No-Claim Scope

No registry crawling, Store package enumeration, portal UI, policy consumption,
adapter execution, broad app blocking, or platform support claim.
