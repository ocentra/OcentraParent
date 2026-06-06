# WP12 iOS Background Region Significant-Change Adapter Source Snapshot

- Branch: codex/tracking-ios-location-manual-required-proof
- Base commit at generation: 30804cc6c93865f3afa9159b65b91863815a4fc9
- Git status at proof generation:

```text
M docs/features/location-geofence-device-status.md
 M docs/plans/tracking-plan/implementation-checklist.md
 M docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md
 M docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md
?? packages/parent-domain/src/tracking-ios-location-manual-required-proof.ts
?? packages/parent-domain/tests/tracking-ios-location-manual-required-proof.test.ts
?? scripts/test/tracking-ios-location-manual-required-proof.mjs
```

- Scope: parent-domain iOS Always authorization, region, significant-change, visits, and background relaunch gaps read model against existing simulator package/manual proof plans.
- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, WP11 workpack, and WP12 workpack.
- Boundary: this proof keeps Core Location authorization, sample, region, background, entitlement, notification, physical-device, authority, and product-ready behavior manual-required until matching artifacts exist.
