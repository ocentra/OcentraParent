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
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [x] Prove foreground permission UX.
- [x] Prove fused sample.
- [x] Prove emulator current `LocationManager` sample with raw coordinate proof
      export.
- [x] Prove emulator foreground permission grant and app-reported last-known
      sample state.
- [x] Preserve provider, accuracy, and timestamp metadata in emulator
      foreground proof without exporting raw coordinates.
- [x] Document emulator device, OS version, app build, foreground service
      scaffold, and proof commands without claiming fused/current,
      physical-device, or product-ready location behavior.
- [x] Generate the Android Studio/local proof artifact plan before device work.
- [x] Record parent-domain manual-required proof rows for missing foreground
      permission grant and foreground location sample before device/runtime work.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local plan lists the APK build, `adb`, permission-state, runtime
location-evidence, and status artifacts that must be collected before any
foreground Android location claim. Runtime/product-complete behavior is still
not claimed.

`npm run test:tracking-plan-android-emulator-proof` now fills the local
emulator foreground proof layer: it builds the Android debug APK with Google
Play Services location, installs and launches it on `Pixel_9_Pro_XL_API_35`,
grants the declared foreground location runtime permissions, captures
foreground-service state, UI tree, headless screenshot inspection, logcat,
package permission state, device metadata, battery/connectivity dumps, and
writes
`03-runtime-location-evidence.json` as a fused foreground sample observed status
when the Android permission-controller foreground location UX dialog is captured
before the scripted grant and the app UI reports
`foreground-location-permission-granted`,
`current-location-sample-observed-emulator-location-manager`, provider `gps`,
source `android-location-manager-current-listener-emulator`, and Google Play
Services fused foreground sample metadata with provider `fused`, observed epoch
millis, accuracy meters, source `google-play-services-fused-current-emulator`
or `google-play-services-fused-last-known`, and raw latitude/longitude values.
This proves emulator foreground permission UX readiness plus app-emitted current
`LocationManager` and fused sample metadata with raw coordinate proof export; it
does not prove physical Android device behavior, background/geofence
transitions, authority, provider delivery, notification delivery, or
product-ready Android tracking.

`node scripts/test/tracking-android-permission-background-proof.mjs` now records
WP08 parent-domain manual-required rows for the foreground permission grant and
foreground location sample gaps. The proof writes
`02-platform-permission-proof.md`, refreshes `03-runtime-location-evidence.json`,
and keeps foreground permission/location sample, physical-device behavior,
authority, notification/provider delivery, and product-ready Android tracking
unclaimed.

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
- `output/tracking-plan-proof/08-android-foreground-location-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: Android foreground workpack doc, tracking
      implementation checklist, owning tracking feature doc, WP08 generated
      proof artifacts, and focused Android permission/background proof results.
- [x] Validation commands and results:
      `node scripts/test/tracking-android-permission-background-proof.mjs`
      passed locally for manual-required rows;
      `npm run test:tracking-plan-android-emulator-proof` passed locally for
      the emulator foreground permission and last-known sample metadata proof.
- [x] Proof artifacts under
      `output/tracking-plan-proof/08-android-foreground-location-adapter/`,
      including `02-platform-permission-proof.md`,
      `03-runtime-location-evidence.json`, `15-manual-platform-proof.md`, and
      `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the local manual-required
      foreground permission/sample proof; central capability row update remains
      a hub/primary-owned doc delta.
- [x] Known gaps/manual-required states: foreground permission grant,
      foreground permission UX dialog, app-reported current `LocationManager`
      sample state, fused current sample state, provider, timestamp, accuracy
      metadata, source, and raw latitude/longitude proof export are now observed
      on emulator. Samsung S9 physical proof now observes foreground/background
      permission grants and foreground-service-backed background GPS samples;
      authority, provider delivery, notification delivery, geofence
      transition/dwell delivery, Android system geofence delivery, and
      product-ready Android tracking remain unclaimed.
- [x] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: Android foreground proof bundle, Android status UI, Android
      emulator proof harness, owning tracking feature doc, implementation
      checklist, this workpack doc, regenerated WP08/WP09/WP10 Android proof
      artifacts, and hub doc delta queue.
- [x] Validation commands and results:
      `npm run test:tracking-plan-android-emulator-proof` passed after adding
      current foreground sample source plus raw latitude/longitude proof export.
- [x] Proof artifacts:
      `output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json`
      and `test-results/tracking-plan-android-emulator-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [x] Known gaps/manual-required states: this remains emulator P3 local proof
      plus Samsung S9 physical permission/background-sample evidence. Android
      physical geofence transition/dwell delivery, Android system geofence
      delivery, authority, provider delivery, notification delivery, and
      product-ready Android tracking remain unclaimed.
