# Workpack 04 - Parent Android Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `04-parent-android-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the parent Android package, device install state, and store/manual-required state.

Current status: complete with explicit manual-required boundaries for device/emulator install proof and store/signing proof.

## Must prove

- `release:package:parent-android` is the package anchor
- device install or simulator proof is recorded explicitly
- store/manual-required status is visible
- no child-runtime distribution claims appear in the Android row

## Execution truth

- `cmd /c npm run release:package:parent-android` now builds the real parent Android debug APK from `platforms/android/parent` into `target/release-packages/parent-android/`.
- `cmd /c npm run test:parent-mobile-package-source-artifact-proof` proves the parent Android package id, launch activity, source tree, and release script remain separate from child Android runtime surfaces.
- `cmd /c npm run test:parent-android-package-proof` records the current Android package truth under `test-results/parent-android-package-proof/proof.json`: versioned APK, latest APK alias, checksums, explicit `ca.ocentra.parent.mobile/.MainActivity` launch target, and no child-runtime claim.
- Install/launch proof is explicit manual-required in this checkout because `adb devices` returned no attached device and `cmd /c emulator -list-avds` was unavailable on this host.
- Google Play/store and release-signing proof remain explicit manual-required states and are not implied by the debug APK artifact.

## Proof root

- `output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/`

## Failure conditions

- scaffold-only output is treated as distribution parity
- store proof is implied without a real artifact
- Android proof is used to claim iOS or desktop readiness
