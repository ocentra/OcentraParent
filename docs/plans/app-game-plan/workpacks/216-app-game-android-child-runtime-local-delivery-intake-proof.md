# WP216 App/Game Android Child Runtime Local Delivery Intake Proof

## Scope

Add Android child-app package-local delivery intake proof for the app/game child
runtime delivery path.

This proves the Android child package can trigger a non-exported in-package
delivery receiver from `MainActivity`, record a package-local delivery intake
marker, and tie that local intake to package-local receipt channel, receipt,
and receipt-ack markers. It does not claim service delivery or receipt
ingestion, provider delivery, platform delivery outside the package, adapter
dispatch, platform enforcement, or raw private row custody.

## Implementation

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryProof.java`.
- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryReceiver.java`.
- Updated Android `MainActivity` to trigger package-local delivery intake proof
  through an explicit extra and render delivery intake/readback state.
- Added the non-exported delivery receiver declaration in
  `platforms/android/agent/app/src/main/AndroidManifest.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-delivery-intake-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-delivery-intake-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-delivery-intake-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-delivery-intake-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-delivery-intake-physical-proof/proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/physical-proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/01-physical-android-delivery-snapshot.md`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/10-validation-commands.log`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/11-physical-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with a non-exported package-local
  delivery intake receiver.
- The Android activity can trigger the in-package delivery proof without
  exposing an external broadcast receiver.
- The delivery intake path records local delivery, receipt channel, receipt,
  and receipt-ack marker custody.
- The physical Samsung Galaxy S9 proof target installs and launches the debug
  APK and exposes the delivery, channel, receipt, and ack internal marker files
  through debug `run-as`.

Not proved:

- Service delivery or receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution outside the child package.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
