# WP07 Source Snapshot

Workpack: `07-windows-store-uwp-appx-inventory-adapter`
Lane: `codex-c`
Branch: `codex/app-game-windows-store-inventory`
Base head before WP07 edits: `3831f5a`

## Source Docs Read

- `docs/plans/app-game-plan/workpacks/07-windows-store-uwp-appx-inventory-adapter.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `crates/agent-core/README.md`
- `crates/agent-protocol/README.md`

## Source Files Inspected

- `packages/activity-domain/src/app-game-inventory.ts`
- `packages/activity-domain/src/app-game-inventory-primitives.ts`
- `packages/activity-domain/src/app-game-identity-primitives.ts`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/tests/app-game-inventory.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_tests.rs`

## Lock State

Locked for `codex-c`: Rust app/game protocol constants/tests, the app-game store
module, the staged Store/UWP parser/test files, app-game plan docs, and WP07
proof files. The slice avoids crate roots, service endpoints, portal paths,
product checklist, and D-owned browser-plan paths.

## Scope Boundary

This workpack adds typed parser proof for Microsoft Store, UWP, AppX, and MSIX
package inventory rows. It proves that Store package rows remain separate from
Win32 executable identity, that store game rows can carry a game category
candidate, that runtime merge checks require deterministic package/AUMID
identity, and that AppUserModelId can be handed to later policy-target compiler
work.

It does not claim live package enumeration, Store API integration, install or
purchase approval, journal ingest, service events, portal inventory rows,
product checklist status movement, or broad package blocking.
