# WP01 Source Snapshot

Workpack: `01-contract-boundary-and-effect-schemas`
Lane: `codex-c`
Branch: `codex/app-plan-work`
Base head before WP01 edits: `3d581e9`

## Scope

- Extended `packages/activity-domain/src/app-game-primitives.ts`.
- Extended `packages/activity-domain/src/app-game.ts`.
- Extended `packages/activity-domain/tests/app-game.test.ts`.
- Added `packages/activity-domain/tests/app-game-evidence-claim.test.ts`.
- Added `packages/parent-domain/src/app-game-control-authority.ts`.
- Added `packages/parent-domain/src/app-game-control-authority-rules.ts`.
- Added `packages/parent-domain/tests/app-game-control-authority.test.ts`.
- Exported the parent authority module from `packages/parent-domain/package.json`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.

## Hub Locks

Exact paths locked in `codex-c` for this slice:

- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/tests/app-game.test.ts`
- `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-authority-rules.ts`
- `packages/parent-domain/tests/app-game-control-authority.test.ts`
- `packages/parent-domain/package.json`
- `docs/plans/app-game-plan/implementation-checklist.md`
- exact proof files under `output/app-game-plan-proof/01-contract-boundary-and-effect-schemas/`

## Current Status Note

Pre-commit exact commit SHA is recorded in the final hub report after commit.
Unrelated pre-existing `.codex/` and `.playwright-cli/` proof artifacts remain
untracked and untouched.
