# WP08 Android Foreground Location Adapter

## Purpose

Prove Android foreground current, fused, and last-known location evidence with
permission, accuracy, freshness, and degraded state.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- Android official location update docs

## Target State

Android foreground location produces contract-valid location evidence or an
explicit unavailable/degraded state.

## Tests And Proof

Proof root: `output/tracking-plan-proof/08-android-foreground-location-adapter/`

- `02-platform-permission-proof.md`
- `03-runtime-location-evidence.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Prove foreground permission UX.
- [ ] Prove fused/current sample.
- [ ] Prove last-known sample or unavailable state.
- [ ] Preserve accuracy and timestamp from provider.
- [ ] Document device, OS version, app build, and proof commands.
- [x] Generate the Android Studio/local proof artifact plan before device work.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local plan lists the APK build, `adb`, permission-state, runtime
location-evidence, and status artifacts that must be collected before any
foreground Android location claim. Runtime/product-complete behavior is still
not claimed.
`node scripts/test/tracking-plan-platform-local-proof.mjs` now proves the first
Android Studio/emulator scaffold runtime pass on `Pixel_9_Pro_XL_API_35`: debug
APK build/install, app launch, status surface, process observation, screenshot,
logcat tail, and foreground service visibility. This does not satisfy the
foreground location sample requirement; fused/current/last-known location
evidence and permission UX remain pending.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/08-android-foreground-location-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/platform-local-proof/android-emulator-local-proof.json`
- `output/tracking-plan-proof/08-android-foreground-location-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/08-android-foreground-location-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
