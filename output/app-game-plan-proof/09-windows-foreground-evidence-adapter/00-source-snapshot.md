# WP09 Windows Foreground App/Game Evidence Adapter Source Snapshot

- Lane: `codex-c`
- Branch: `codex/app-game-windows-foreground-evidence`
- Base before this workpack: `94e78f3 Add Windows process app game runtime proof`
- Commit for this workpack: pending local commit after validation
- Worktree note: tracked edits are limited to the locked WP09 foreground
  contract/protocol/parser/docs/proof paths. Existing untracked `.codex/` and
  `.playwright-cli/` proof artifacts were preserved and not staged.

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
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/09-windows-foreground-evidence-adapter.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/v0-5-native-apps-full-scope-plan.md`
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- `docs/plans/app-plan/v0-5-native-apps-test-blueprint.md`
- `docs/plans/app-plan/ui-ux-requirements-guide.md`
- `docs/plans/app-plan/workpacks/09-windows-foreground-app-evidence-adapter.md`
- `packages/activity-domain/README.md`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-runtime.ts`
- `packages/activity-domain/tests/app-game-runtime.test.ts`
- `crates/agent-protocol/README.md`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/README.md`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime.rs`
- `crates/agent-core/src/window_capture.rs`
- `crates/agent-core/src/window_capture_event.rs`
- `crates/agent-core/src/window_capture_tests.rs`

## Before-State Gap

WP08 proved process runtime evidence but intentionally kept
`foregroundState = notClaimed`. The missing WP09 gap was a separate foreground
evidence contract/protocol/parser proof that can represent active focus, closed
foreground intervals, permission-limited title/window metadata, and
foreground-is-not-content guards.
