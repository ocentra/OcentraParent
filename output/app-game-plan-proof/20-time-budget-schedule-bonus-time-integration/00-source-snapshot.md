# WP20 Source Snapshot

## Branch And Commit

- Branch: `codex/app-game-read-model-service-events`
- Base commit before WP20 edits: `a0e4280f`
- Latest verified `origin/main`: `49e4c1c`
- Lane: `codex-c`
- Workpack: `20-time-budget-schedule-bonus-time-integration`

## Source State

- `origin/main` was confirmed as an ancestor of `HEAD` before WP20 edits.
- `git status --short` showed pre-existing untracked local `.codex` and
  `.playwright-cli` proof/browser artifacts plus the new parent-domain
  time-budget files.
- Hub lock covered the new parent-domain time-budget source/test files,
  app-game and app-plan checklist/docs paths, and both proof roots.

## Docs Read

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/ai.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/20-time-budget-schedule-bonus-time-integration.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/19-time-budget-schedule-bonus-time-integration.md`
- `packages/parent-domain/README.md`
- `packages/activity-domain/README.md`

## Source Inspected

- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler-rules.ts`
- `packages/parent-domain/src/native-game-budget-policy.ts`
- `packages/parent-domain/src/native-game-budget-policy-rules.ts`
- `packages/parent-domain/tests/app-game-policy-target-compiler.test.ts`
- `packages/parent-domain/tests/native-game-budget-policy.test.ts`

## Scope Boundary

This slice adds TypeScript parent-domain contract proof for shared app/game
time budgets, schedule state, bonus-time approval/audit refs, dry-run actions,
and timer recovery refs. It does not add Rust protocol parity, service runtime
evaluation, portal budget authoring UI, notification delivery, adapter
execution, or broad app/game blocking.
