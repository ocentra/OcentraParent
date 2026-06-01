# V6 Mobile Agents Expectations

This is the milestone-specific expectation file for V6 in `docs/product-roadmap.md`.

Supporting expectation files: [platforms](platforms.md),
[platform deliverables](platform-deliverables.md), [capture](capture.md),
[evidence storage](evidence-storage.md),
[location and geofence](location-geofence.md),
[tamper and uninstall protection](tamper-uninstall-protection.md), and
[release installer](release-installer.md).

## Outcome

- Android, iOS, macOS, and Linux claims match real OS permissions, APIs, packaging, and store constraints.
- Mobile agents reuse shared contracts and journal/query shapes where practical.
- Platform limits are visible instead of disguised as parity.
- Parent mobile app claims and child mobile agent claims are tracked separately.
- Location, notification, integrity, permission-loss, and uninstall/tamper
  claims are tracked per platform instead of bundled under "mobile support."

## Acceptance

- Android foreground/device-admin paths and iOS approved APIs are documented and tested before product claims.
- Platform-specific capture/enforcement adapters stay behind typed boundaries.
- Location/geofence, notification, app-activity, network-filtering, and
  integrity capabilities each have their own permission/capability status.
- Mobile packaging and install/update/store-readiness evidence is available for each claimed platform.
- CI/emulator/simulator evidence is paired with real-device, entitlement, or
  managed-device notes where OS policy requires it.

## Validation

- Run `npm run validate`.
- Include platform-specific smoke tests, permission/capability evidence,
  location/integrity proof where claimed, contract parity, and release/package
  checks for each supported platform.
