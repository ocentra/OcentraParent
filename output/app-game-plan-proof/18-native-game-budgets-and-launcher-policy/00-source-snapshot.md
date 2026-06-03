# WP18 Source Snapshot

## Branch And Commit

- Branch: `codex/app-game-read-model-service-events`
- Base commit before WP18 edits: `b297405`
- Lane: `codex-c`
- Workpack: `18-native-game-budgets-and-launcher-policy`

## Source State

- `git status --short` before WP18 source edits showed only pre-existing
  untracked local proof/browser artifacts plus the new WP18 files after the
  contract files were created.
- Hub lock refreshed for:
  `packages/parent-domain/src/native-game-budget-policy-rules.ts`,
  `packages/parent-domain/src/native-game-budget-policy.ts`,
  `packages/parent-domain/tests/native-game-budget-policy.test.ts`,
  `docs/features/app-game-control.md`,
  `docs/plans/app-game-plan/current-app-game-snapshot.md`,
  `docs/plans/app-game-plan/implementation-checklist.md`,
  `docs/plans/app-game-plan/source-index.md`,
  `docs/plans/app-game-plan/workpacks/18-native-game-budgets-and-launcher-policy.md`,
  and this proof root.

## Docs Read

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/18-native-game-budgets-and-launcher-policy.md`
- `packages/parent-domain/README.md`

## Source Inspected

- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/src/references.ts`
- `packages/parent-domain/src/reference-primitives.ts`
- `packages/parent-domain/src/game-control-catalog-schema.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-session-primitives.ts`
- `packages/activity-domain/src/app-game-category-risk.ts`

## Scope Boundary

This slice adds TypeScript parent-domain contract proof only. It does not add
Rust protocol parity, service read models, portal authoring UI, live launcher
crawling, notification delivery, adapter execution, or broad game/app blocking.
