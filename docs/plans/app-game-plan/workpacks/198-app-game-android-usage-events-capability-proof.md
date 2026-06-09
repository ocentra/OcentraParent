# WP198 App/game Android UsageEvents capability proof

## Scope

Add an Android package-local bridge for app/game UsageEvents capability status
and bind it to a parent-domain proof contract.

This workpack keeps UsageStats as a settings-grant-required capability and
records only package-local readiness states, commands, events, proof refs, and
open gaps. It does not declare `PACKAGE_USAGE_STATS`, store raw UsageEvents
rows, store package names, or claim adapter dispatch, platform enforcement, or
child-device delivery.

## Non-Goals

- No UsageStats settings grant proof.
- No runtime UsageEvents collection from the Android package.
- No raw UsageEvents rows, package names, class names, or activity custody.
- No Device Owner/Profile Owner authority proof.
- No hide, suspend, uninstall-block, lock task, managed configuration, adapter
  dispatch, platform enforcement, provider delivery, or child delivery claim.

## Files

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsCapabilityProof.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
- `packages/parent-domain/src/app-game-android-usage-events-capability-proof.ts`
- `packages/parent-domain/tests/app-game-android-usage-events-capability-proof.test.ts`
- `scripts/test/app-game-android-usage-events-capability-proof.mjs`

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-capability-proof`
- `cmd /c node --check scripts/test/app-game-android-usage-events-capability-proof.mjs`
- `cmd /c node scripts/test/app-game-android-usage-events-capability-proof.mjs`

## Done Criteria

- Android package source contains a package-local app/game UsageEvents capability
  bridge.
- MainActivity surfaces the bridge state without raw UsageEvents data.
- Parent-domain rejects rows that promote raw storage, adapter dispatch,
  platform enforcement, or child delivery.
- Android package build proof confirms the bridge compiles while UsageStats
  remains settings-grant-required.
