# WP06 Source Snapshot

Workpack: `06-windows-installed-inventory-adapter`
Lane: `codex-c`
Branch: `codex/app-game-windows-installed-inventory`
Base head before WP06 edits: `463366d`

## Source Docs Read

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/skills/ocentra-parent-rule-router/SKILL.md`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-rust-service.mdc`
- `docs/plans/app-game-plan/workpacks/06-windows-installed-inventory-adapter.md`
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
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game_rows.rs`
- `crates/agent-core/Cargo.toml`

## Lock State

Locked for `codex-c`: Rust app/game protocol source/tests, the app-game store
module, the staged Windows inventory parser submodule, app-game plan docs, and
WP06 proof files. D still owns crate root and browser/runtime paths, so WP06
avoids `crates/agent-core/src/lib.rs`, `crates/agent-protocol/src/lib.rs`,
service endpoints, portal paths, and D-owned browser-plan files.

## Scope Boundary

This workpack mirrors WP05 inventory evidence rows into Rust protocol and adds a
typed `agent-core` parser for Windows installed app/game inventory records. It
proves registry-like installed records, Start Menu shortcut records, launcher
manifest game records, strong-identity dedupe, display-only non-merge, and
inventory-is-not-use guards.

It does not claim live registry enumeration, shell-link parsing, executable
metadata, publisher signature, file hash collection, launcher manifest crawling,
journal ingest, service events, portal inventory rows, product checklist status
movement, or broad app/game blocking.
