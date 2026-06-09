# WP199 App/game Android UsageEvents runtime preflight

## Scope

Add an Android package-local runtime preflight for app/game UsageEvents
readiness.

The Android package now checks UsageStats AppOps state and UsageStats service
visibility from package code, then exposes only readiness states through
MainActivity. Parent-domain models the preflight as blocked until runtime sample
proof exists. No raw UsageEvents rows, package names, adapter dispatch, platform
enforcement, or child-device delivery are claimed.

## Non-Goals

- No `PACKAGE_USAGE_STATS` manifest declaration.
- No automatic settings grant or runtime permission claim.
- No runtime UsageEvents sample collection claim.
- No raw UsageEvents rows, package names, class names, or activity custody.
- No Device Owner/Profile Owner, hide/suspend/uninstall-block, lock task,
  managed configuration, adapter dispatch, platform enforcement, provider
  delivery, or child-device delivery claim.

## Files

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
- `packages/parent-domain/src/app-game-android-usage-events-runtime-preflight.ts`
- `packages/parent-domain/tests/app-game-android-usage-events-runtime-preflight.test.ts`
- `scripts/test/app-game-android-usage-events-runtime-preflight-proof.mjs`

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-runtime-preflight`
- `cmd /c node --check scripts/test/app-game-android-usage-events-runtime-preflight-proof.mjs`
- `cmd /c node scripts/test/app-game-android-usage-events-runtime-preflight-proof.mjs`

## Done Criteria

- Android package source checks UsageStats AppOps and service visibility.
- MainActivity surfaces the preflight state without raw UsageEvents data.
- Parent-domain keeps runtime collection blocked until runtime sample proof
  exists.
- Android package build proof confirms the preflight compiles while UsageStats
  remains settings-grant-required.
