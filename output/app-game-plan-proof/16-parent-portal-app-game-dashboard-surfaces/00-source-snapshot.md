# Source Snapshot

Date: 2026-06-03

Branch: `codex/app-game-read-model-service-events`

Scope:

- App-game WP16 parent portal dashboard surfaces.
- Native app-plan WP15 mirror for app inventory/running/foreground rows.

Source docs read:

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/portal.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/16-parent-portal-app-game-dashboard-surfaces.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/15-parent-portal-app-inventory-running-session-surfaces.md`
- app-plan full scope, platform deep dive, test blueprint, and UI/UX guide.

Before state:

- Service-backed app-use and games activity-surface read models existed.
- `activity-ui-intent.ts` exposed raw `appUseReadModel` and `gamesReadModel`.
- The App/Game Sessions route did not have a dedicated app/game dashboard intent
  or route-specific surface assertions.
- `/#/app-game-sessions` still selected the generic reports control route
  context instead of the app/game sessions control.

Changed source:

- `packages/portal-domain/src/parent-portal-data.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`
- `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`
- `apps/portal/e2e/portal-route-scaffold-assertions.ts`
