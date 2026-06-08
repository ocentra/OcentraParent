# WP162 App/game policy readiness parent-surface detail rows

- Branch: `codex/app-game-control-product-completion`
- Base commit before edits: `1eccf2587`
- Worktree status before edits: clean
- Locked paths: `packages/portal-domain/src/app-game-policy-readiness-panel.ts`,
  `packages/portal-domain/src/details.ts`,
  `apps/portal/tests/app-game-policy-readiness-panel.test.ts`,
  `docs/features/app-game-control.md`,
  `docs/plans/app-game-plan/implementation-checklist.md`,
  `docs/plans/app-game-plan/workpacks/README.md`,
  `docs/plans/app-game-plan/workpacks/162-app-game-policy-readiness-parent-surface-detail-rows.md`,
  `output/app-game-plan-proof/162-app-game-policy-readiness-parent-surface-detail-rows`,
  `test-results/app-game-policy-readiness-parent-surface-detail-rows`.

## Source inspected

- `docs/plans/app-game-plan/unified-app-game-control-goal.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `packages/portal-domain/README.md`
- `apps/portal/README.md`
- `packages/portal-domain/src/app-game-policy-readiness-panel.ts`
- `packages/portal-domain/src/details.ts`
- `apps/portal/tests/app-game-policy-readiness-panel.test.ts`

## Result

The App/Game Sessions policy-readiness route intent now surfaces the service
read model's evidence-claim, identity, approval authority, approval
action-result, platform authority, and AI classifier row counts, plus row-level
ready/manual-required/missing reasons. This keeps missing and manual-required
policy inputs visible to parents without claiming policy execution or adapter
dispatch.
