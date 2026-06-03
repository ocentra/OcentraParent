# WP08 Windows Process Runtime Evidence Adapter Source Snapshot

Recorded: 2026-06-03.

## Branch And Base

- Lane: `codex-c`
- Branch: `codex/app-game-windows-process-runtime`
- Base after required main refresh: `origin/main` at `c044a72`
- Pre-WP08 commit head when this snapshot was recorded: `f164fe3`
- PR238 duplicate commits were skipped/dropped during rebase because the app-game
  plan and WP01 baseline are now on `main`.

## Lock State

Locked paths:

- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/app-game-runtime.ts`
- `packages/activity-domain/tests/app-game-runtime.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime_tests.rs`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `output/app-game-plan-proof/08-windows-process-runtime-evidence-adapter`

## Source Files Inspected

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-rust-service.mdc`
- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/enforcement.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/08-windows-process-runtime-evidence-adapter.md`
- `docs/plans/app-game-plan/v0-5-app-game-shared-evidence-spine-plan.md`
- `docs/plans/app-game-plan/v0-5-native-apps-product-slice-plan.md`
- `docs/plans/app-game-plan/v0-5-native-games-product-slice-plan.md`
- `docs/plans/app-game-plan/v0-5-app-game-platform-deep-dive.md`
- `docs/plans/app-game-plan/v0-5-app-game-test-blueprint.md`
- `docs/plans/app-game-plan/ui-ux-requirements-guide.md`
- `crates/agent-core/README.md`
- `crates/agent-protocol/README.md`
- `packages/activity-domain/README.md`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/tests/app-game.test.ts`
- `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_store_inventory.rs`

## Scope Boundary

This workpack adds contract, protocol, and staged parser proof only. It does not
touch D-owned `crates/agent-core/src/process_capture.rs`,
`crates/agent-core/src/process_capture_tests.rs`, `crates/agent-service/**`,
portal UI, browser-plan files, live process polling, journal ingest, SQLite
replay, foreground evidence, content evidence, policy execution, or enforcement
adapters.
