# WP164 App/game dashboard readiness blocker cards

- Branch: `codex/app-game-control-product-completion`
- Base commit before edits: `3aba686be`
- Worktree status before edits: clean
- Locked paths: `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`,
  `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`,
  `docs/features/app-game-control.md`,
  `docs/plans/app-game-plan/implementation-checklist.md`,
  `docs/plans/app-game-plan/workpacks/README.md`,
  `docs/plans/app-game-plan/workpacks/164-app-game-dashboard-readiness-blocker-cards.md`,
  `output/app-game-plan-proof/164-app-game-dashboard-readiness-blocker-cards`,
  `test-results/app-game-dashboard-readiness-blocker-cards`.

## Source inspected

- `docs/plans/app-game-plan/unified-app-game-control-goal.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`
- `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`

## Result

The main App/Game Sessions dashboard intent now turns existing boundary counts
and row states into parent-readable readiness blocker cards. Parents can see
missing approval action results, AI classifier evidence-only review,
manual-required capability rows, and unknown approval review rows in the
existing evidence drawer, with an aggregate readiness blocker metric, without
claiming policy execution, adapter dispatch, or platform enforcement.
