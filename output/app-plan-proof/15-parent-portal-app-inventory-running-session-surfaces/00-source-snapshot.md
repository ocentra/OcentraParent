# Source Snapshot

Date: 2026-06-03

Branch: `codex/app-game-read-model-service-events`

Native app scope:

- App-plan WP15 parent portal app inventory/running/session surfaces.
- This is mirrored through app-game WP16 because the service read models expose
  shared app-use and games dashboard rows.

Source docs read:

- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/v0-5-native-apps-full-scope-plan.md`
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- `docs/plans/app-plan/v0-5-native-apps-test-blueprint.md`
- `docs/plans/app-plan/ui-ux-requirements-guide.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/15-parent-portal-app-inventory-running-session-surfaces.md`

Changed native-app-relevant source:

- `packages/portal-domain/src/parent-portal-data.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`
- `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`
- `apps/portal/e2e/portal-route-scaffold-assertions.ts`
