# Parent Android Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_ANDROID_DISTRIBUTION.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

The parent Android package is a parent client artifact with device and store proof separated from setup and child runtime ownership.

## Boundary

- Owns the parent Android package, device install proof, signer state, and store/manual-required state.
- Does not own child Android runtime distribution, device-owner policy, or setup journey logic.

## Validation anchors

- `npm run release:package:parent-android`
- `npm run test:parent-android-package-proof`
- `npm run test:parent-mobile-shell-runtime-proof`
- `npm run test:parent-mobile-package-source-artifact-proof`
- `npm run test:parent-mobile-service-bridge`
- `npm run test:parent-mobile-controller-observer-handoff`

## Current packet truth

- The package anchor is the real `release:package:parent-android` build path and produces the parent APK under `target/release-packages/parent-android/`.
- The WP04 proof anchor is `test:parent-android-package-proof`, which records artifact presence, checksum files, explicit package id/activity, and current Android install/store/manual-required truth.
- In this checkout the install row remains manual-required because no attached Android device was available through `adb` and no local `emulator` command was available to boot a simulator.
- Store and signing rows remain manual-required and are not upgraded by the debug APK artifact.

## Negative cases that must exist

- scaffold-only package remains manual-required
- missing device proof does not become store proof
- parent Android install does not imply child Android runtime distribution
- package-source proof does not claim device-owner policy
