# WP21 Source Snapshot

## Branch And Commit

- Branch: `codex/app-game-read-model-service-events`
- Base commit before WP21 edits: `ead13c6`
- Latest verified `origin/main`: `49e4c1c`
- Lane: `codex-c`
- Workpack: `21-child-facing-warning-and-request-ux`

## Source State

- `origin/main` was confirmed as an ancestor of `HEAD` before WP21 edits.
- `git status --short` showed only pre-existing untracked local `.codex` and
  `.playwright-cli` proof/browser artifacts.
- Hub lock covered parent-domain child UX contract files, text-domain child copy
  files, app-game and app-plan checklist/docs paths, and both proof roots.

## Docs Read

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/21-child-facing-warning-and-request-ux.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/20-child-facing-app-warning-block-request-ux.md`
- `docs/plans/app-plan/ui-ux-requirements-guide.md`
- `packages/parent-domain/README.md`
- `packages/text-domain/README.md`

## Source Inspected

- `packages/parent-domain/src/app-game-control-approval-flow.ts`
- `packages/parent-domain/src/app-game-time-budget-policy.ts`
- `packages/parent-domain/src/app-game-time-budget-policy-rules.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `packages/text-domain/src/contracts.ts`
- `packages/text-domain/src/portal-dev.ts`
- `packages/text-domain/src/portal-product-text.ts`
- `packages/text-domain/src/portal-product-text-tokens.ts`

## Scope Boundary

This slice adds TypeScript parent-domain and text-domain contract proof for
child-facing warning, approval-needed, time-limit, request, manual-required,
and unavailable states. It does not add live child UI, portal screenshots,
service persistence, notification delivery, child-device runtime wiring, or
adapter execution.
