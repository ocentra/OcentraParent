# WP221 App/Game Android Child Runtime Local Notification Action Proof

## Scope

Add Android child-app package-local notification action proof for the app/game
ask-parent path.

This proves the Android child package can attach a local request action to the
app/game notification, route that action to a non-exported in-package receiver,
and write a package-local request marker for debug readback. It does not claim
provider delivery, platform delivery outside the package, service request
ingestion, parent approval round trip, adapter dispatch, broad blocking,
platform enforcement, or raw private source-row custody.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeLocalNotificationProof.java`
  with a local notification request action and proof state.
- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationActionReceiver.java`.
- Registered the non-exported receiver in
  `platforms/android/agent/app/src/main/AndroidManifest.xml`.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  with local notification action proof trigger and status rendering.
- Added the child notification action label in
  `platforms/android/agent/app/src/main/res/values/strings.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-notification-action-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-notification-action-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-notification-action-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-notification-action-proof/proof.json`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/proof.json`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child package exposes a package-local ask-parent action on the
  app/game local notification.
- The Android child package records and reads back a package-local request
  action marker.
- Parent-domain accepts only channel, post, action, marker readback, and opaque
  proof refs while rejecting service ingestion, approval round trip, provider,
  external platform, dispatch, enforcement, and raw private source-row claims.

Not proved:

- Provider notification delivery.
- Platform delivery outside the child package.
- Service request ingestion.
- Parent approval round trip.
- Adapter dispatch, broad blocking, or platform enforcement.
- Raw private source row custody.
