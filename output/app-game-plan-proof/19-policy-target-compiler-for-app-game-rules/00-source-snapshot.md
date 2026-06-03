# WP19 Source Snapshot

## Branch And Commit

- Branch: `codex/app-game-read-model-service-events`
- Base commit before WP19 edits: `cb364e3`
- Lane: `codex-c`
- Workpack: `19-policy-target-compiler-for-app-game-rules`

## Source State

- `origin/main` was confirmed as an ancestor of `HEAD` before WP19 edits.
- `git status --short` showed only pre-existing untracked local `.codex` and
  `.playwright-cli` proof/browser artifacts.
- Hub lock covered the new parent-domain compiler source/test files, app-game
  and app-plan checklist/docs paths, and both proof roots.

## Docs Read

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/19-policy-target-compiler-for-app-game-rules.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/18-policy-target-compiler-for-app-rules.md`
- `packages/parent-domain/README.md`

## Source Inspected

- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/native-game-budget-policy.ts`
- `packages/parent-domain/src/native-game-budget-policy-rules.ts`
- `packages/parent-domain/tests/native-game-budget-policy.test.ts`

## Scope Boundary

This slice adds TypeScript parent-domain compiler contract proof. It does not
add Rust protocol parity, service runtime evaluation, portal rule authoring UI,
notification delivery, adapter execution, timers, rollback, or broad app/game
blocking.
