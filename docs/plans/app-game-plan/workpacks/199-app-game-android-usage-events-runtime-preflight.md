# WP199 App/game Android UsageEvents runtime preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP199 App/game Android UsageEvents runtime preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
