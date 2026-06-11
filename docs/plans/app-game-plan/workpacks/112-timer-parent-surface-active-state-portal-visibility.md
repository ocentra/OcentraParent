# WP112 Timer Parent-Surface Active-State Portal Visibility

## Scope

- Render WP111 active timer state-store flags in the parent portal
  timer parent-surface intent.
- Keep the display unified for native apps and native games.
- Show timer runtime, scheduler persistence, durable scheduler storage, audit
  runtime, and rollback runtime as separate parent-visible details.
- Keep adapter dispatch, child delivery, broad blocking, platform enforcement,
  live scheduling, audit/rollback execution, and raw private source rows
  unclaimed.

## Implementation

- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` now maps
  `timerRuntimeClaimed`, `schedulerPersistenceClaimed`,
  `durableSchedulerStorageClaimed`, `auditRuntimeClaimed`, and
  `rollbackRuntimeClaimed` into summary details.
- The product-claim copy changes when a service-backed active timer state-store
  is visible.
- `packages/text-domain/src/portal-dev.ts` no longer states that active timer
  state-store visibility is always unclaimed.
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts` covers both
  inactive and active state-store read models.

## Validation

- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`
- `cmd /c npx prettier --check packages/portal-domain/src/app-game-timer-parent-surface-panel.ts apps/portal/tests/app-game-timer-parent-surface-panel.test.ts packages/text-domain/src/portal-dev.ts docs/features/app-game-control.md docs/plans/app-game-plan/implementation-checklist.md docs/plans/app-game-plan/workpacks/README.md docs/plans/app-game-plan/workpacks/112-timer-parent-surface-active-state-portal-visibility.md output/app-game-plan-proof/112-timer-parent-surface-active-state-portal-visibility/00-summary.md output/app-game-plan-proof/112-timer-parent-surface-active-state-portal-visibility/10-validation-commands.log test-results/app-game-timer-parent-surface-active-state-portal-visibility/handoff.json`

## Remaining Gaps

- Live timer scheduling execution remains unimplemented.
- Audit runtime/log read-model storage and rollback execution remain
  unimplemented.
- Adapter dispatch, child delivery, broad blocking, platform enforcement, and
  raw private source row access remain unclaimed.
