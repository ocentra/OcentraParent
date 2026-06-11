# WP220 App/Game Android Child Runtime Local Notification Proof

## Scope

Add Android child-app package-local notification proof for app/game warning and
request UX handoff.

This proves the Android child package can create a local notification channel,
post a package-local app/game child notice through Android `NotificationManager`,
and write a package-local marker for readback. It does not claim provider
delivery, platform delivery outside the package, child request approval round
trip, adapter dispatch, broad blocking, platform enforcement, or raw private
source-row custody.

## Implementation

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeLocalNotificationProof.java`.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  with local notification proof trigger and status rendering.
- Added app/game child notification strings in
  `platforms/android/agent/app/src/main/res/values/strings.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-notification-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-notification-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-notification-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-notification-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-notification-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-notification-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-notification-proof/proof.json`
- `output/app-game-plan-proof/220-app-game-android-child-runtime-local-notification-proof/proof.json`
- `output/app-game-plan-proof/220-app-game-android-child-runtime-local-notification-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/220-app-game-android-child-runtime-local-notification-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child package creates a dedicated app/game local notification
  channel.
- The Android child package posts a package-local app/game notification after a
  proof trigger.
- The Android child package writes and reads back a package-local notification
  marker.
- Parent-domain accepts only channel, notification post, marker readback, and
  opaque proof refs while rejecting provider, external platform, dispatch,
  enforcement, and raw private source-row claims.

Not proved:

- Provider notification delivery.
- Platform delivery outside the child package.
- Child request approval round trip.
- Adapter dispatch, broad blocking, or platform enforcement.
- Raw private source row custody.
