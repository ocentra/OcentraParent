# WP109 Timer Parent-Surface Service Read Model Proof

## Scope

- Added `agent.activity.app-game.timer-parent-surface.read-model.get`.
- Added `agent.activity.app-game.timer-parent-surface.read-model.reported`.
- Wired TypeScript protocol-domain parsing, Rust protocol transport/DTOs, and
  agent-service websocket command handling.
- Backed rows with the existing app-game ActivityStore service read model.

## No-Claim Boundary

The read model keeps these fields false:

- `timerRuntimeClaimed`
- `schedulerPersistenceClaimed`
- `durableSchedulerStorageClaimed`
- `auditRuntimeClaimed`
- `rollbackRuntimeClaimed`
- `adapterDispatchClaimed`
- `childDeliveryClaimed`
- `platformEnforcementClaimed`
- `rawPrivateSourceRowsIncluded`

## Validation

- `cmd /c npx prettier --check packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts packages/agent-protocol-domain/tests/app-game-timer-parent-surface-read-model.test.ts docs/plans/app-game-plan/unified-app-game-control-goal.md docs/plans/app-game-plan/workpacks/109-timer-parent-surface-service-read-model.md docs/plans/app-game-plan/workpacks/README.md docs/plans/app-game-plan/implementation-checklist.md docs/features/app-game-control.md` passed.
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model contracts` passed: 5 files, 35 tests.
- `cargo test -p ocentra-parent-agent-protocol app_game_timer_parent_surface --quiet` passed: 2 tests.
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface --quiet` passed: 2 tests.
