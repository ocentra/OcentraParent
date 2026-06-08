# WP163 App/game dashboard boundary count visibility

- Branch: `codex/app-game-control-product-completion`
- Base commit before edits: `d329eca18`
- Worktree status before edits: clean
- Locked paths: `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`,
  `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`,
  `docs/features/app-game-control.md`,
  `docs/plans/app-game-plan/implementation-checklist.md`,
  `docs/plans/app-game-plan/workpacks/README.md`,
  `docs/plans/app-game-plan/workpacks/163-app-game-dashboard-boundary-count-visibility.md`,
  `output/app-game-plan-proof/163-app-game-dashboard-boundary-count-visibility`,
  `test-results/app-game-dashboard-boundary-count-visibility`.

## Source inspected

- `docs/plans/app-game-plan/unified-app-game-control-goal.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`
- `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`
- `packages/activity-domain/src/activity-surface.ts`

## Result

The main App/Game Sessions dashboard intent now surfaces the service-backed
boundary row counts already present on app-use/game rows. Parents can see
aggregate boundary and AI classifier counts plus evidence-drawer boundary
summaries without the dashboard claiming policy execution, adapter dispatch, or
platform enforcement.
