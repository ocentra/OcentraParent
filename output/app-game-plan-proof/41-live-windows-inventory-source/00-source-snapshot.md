# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base: `86214bb294a0a8dc5f9a79bb72410bc3a5c36f31`
- Workpack: WP41 live Windows inventory source
- Primary source files:
  - `crates/agent-protocol/src/app_game.rs`
  - `crates/agent-protocol/src/constants.rs`
  - `crates/agent-core/src/activity_store_app_game.rs`
  - `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_source.rs`
  - `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_source_tests.rs`
- Product docs:
  - `docs/features/app-game-control.md`
  - `docs/plans/app-game-plan/current-app-game-snapshot.md`
  - `docs/plans/app-game-plan/implementation-checklist.md`
  - `docs/plans/app-game-plan/workpacks/41-live-windows-inventory-source.md`
- Checklist decision: `docs/product-capability-checklist.md` intentionally unchanged.

## Scope

Core-only bounded live Windows Start Menu shortcut inventory source. It emits
inventory-only records and journal events through the existing app/game
inventory parser and journal/SQLite path.

## No-Claim Scope

No registry crawling, Store package enumeration, service capture, portal UI,
policy consumption, adapter execution, broad app blocking, or platform support
claim.
