# WP218 App/Game Android UsageEvents Granted Sample Proof

## Scope

Prove the Android child-agent package can observe a granted, count-only
UsageEvents sample on the physical Android proof target.

This closes the WP201 gap where the package could install, launch, and report
UsageEvents permission/sample state, but did not actively prove a granted
settings/AppOps state with a live sample count. It keeps raw UsageEvents rows,
package names, activity rows, UI XML, adapter dispatch, child-device delivery,
provider delivery, platform enforcement, Device Owner/Profile Owner authority,
and Play policy proof unclaimed.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java`
  with named count-only sample fields.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  so the child app activity renders `sampleEventCount` and
  `foregroundEventCount` beside the existing UsageEvents permission and sample
  states.
- Added
  `packages/parent-domain/src/app-game-android-usage-events-granted-sample-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-usage-events-granted-sample-proof.test.ts`.
- Added
  `scripts/test/app-game-android-usage-events-granted-sample-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-usage-events-granted-sample-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-granted-sample-proof
cmd /c node scripts/test/app-game-android-usage-events-granted-sample-proof.mjs
```

## Proof

- `test-results/app-game-android-usage-events-granted-sample-proof/proof.json`
- `output/app-game-plan-proof/218-app-game-android-usage-events-granted-sample-proof/proof.json`
- `output/app-game-plan-proof/218-app-game-android-usage-events-granted-sample-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/218-app-game-android-usage-events-granted-sample-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android debug package installs and launches on the physical Samsung Galaxy
  S9 proof target.
- The Android shell AppOps path can grant `GET_USAGE_STATS` for the debug
  child-agent package on that target.
- The package-local UsageEvents runtime preflight can observe a count-only
  sample after launch.
- Parent-domain accepts only granted AppOps, activity-visible count-only sample
  evidence, opaque proof refs, and false raw-custody/enforcement claims.

Not proved:

- Device Owner or Profile Owner authority.
- Play policy or store distribution proof.
- Raw UsageEvents rows, package names, activity rows, raw UI XML, or raw device
  serial custody.
- Provider delivery, child-device delivery outside the package, adapter
  dispatch, broad blocking, or platform enforcement.
