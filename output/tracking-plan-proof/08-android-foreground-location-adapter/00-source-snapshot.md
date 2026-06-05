# WP08 Android Foreground Location Adapter Source Snapshot

- Branch: codex/tracking-android-permission-background-proof
- Base commit at generation: f3578df8a61e1b96854416da3f6e45a2d789aafb
- Git status at proof generation:

```text
M docs/feature-list.md
 M docs/features/location-geofence-device-status.md
 M docs/plans/tracking-plan/implementation-checklist.md
 M docs/plans/tracking-plan/workpacks/08-android-foreground-location-adapter.md
 M docs/plans/tracking-plan/workpacks/09-android-background-location-and-geofence-adapter.md
 M output/tracking-plan-proof/08-android-foreground-location-adapter/00-source-snapshot.md
 M output/tracking-plan-proof/08-android-foreground-location-adapter/02-platform-permission-proof.md
 M output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json
 M output/tracking-plan-proof/08-android-foreground-location-adapter/15-manual-platform-proof.md
 M output/tracking-plan-proof/08-android-foreground-location-adapter/16-validation-commands.log
 M output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/00-source-snapshot.md
 M output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/02-platform-permission-proof.md
 M output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json
 M output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/15-manual-platform-proof.md
 M output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/16-validation-commands.log
?? packages/parent-domain/src/tracking-android-permission-background-proof.ts
?? packages/parent-domain/tests/tracking-android-permission-background-proof.test.ts
?? scripts/test/tracking-android-permission-background-proof.mjs
```

- Scope: parent-domain Android foreground permission and sample gaps read model against existing emulator scaffold/manual proof plans.
- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, WP08 workpack, and WP09 workpack.
- Boundary: this proof keeps Android foreground permission, foreground sample, background permission, and geofence transitions manual-required until device/runtime artifacts exist.
