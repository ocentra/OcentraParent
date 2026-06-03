# WP10 Launcher Evidence And Game Candidate Model Source Snapshot

- Lane: `codex-c`
- Branch: `codex/app-game-launcher-candidate-model`
- Base before this workpack: `e106c47 Reconcile native app proof packs with app game spine`
- Commit for this workpack: pending local commit after validation
- Worktree note: tracked edits are limited to the locked WP10
  contract/protocol/parser/docs/proof paths. Existing untracked `.codex/` and
  `.playwright-cli/` proof artifacts were preserved and not staged.

## Source Files Inspected

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-protocol-websocket.mdc`
- `.ocentra-ai/rules/ocentra-parent-rust-service.mdc`
- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/10-launcher-evidence-and-game-candidate-model.md`
- `packages/activity-domain/readme.md`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-runtime.ts`
- `packages/activity-domain/src/app-game-foreground.ts`
- `packages/activity-domain/src/app-game-inventory.ts`
- `packages/activity-domain/tests/app-game-runtime.test.ts`
- `packages/activity-domain/tests/app-game-foreground.test.ts`
- `crates/agent-protocol/README.md`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/README.md`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground.rs`

## Before-State Gap

WP08 and WP09 proved process runtime and foreground evidence, and both carried
launcher no-claim guards. The missing WP10 gap was a dedicated launcher evidence
row that can represent launcher-only, launcher foreground, launcher-game
candidate, and proved child-game states without treating a launcher as a game.
