# WP09 Android Background Location And Geofence Adapter

## Purpose

Proof-gate Android background location and geofence enter/exit/dwell behavior.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- Android geofencing and background location docs
- `docs/expectations/platforms.md`

## Target State

Background tracking and geofencing are represented only when permission,
settings-page flow, active geofence limit, transition delivery, battery, and
degraded states have proof.

## Tests And Proof

Proof root: `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`

- `02-platform-permission-proof.md`
- `05-geofence-transition-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Prove Android 10+ background permission where claimed.
- [x] Prove Android 11+ settings-page background permission flow.
- [ ] Prove Android system enter, exit, and dwell transitions.
- [x] Represent active geofence limit.
- [x] Add battery/background degraded proof before claims.
- [x] Record emulator package permission-grant proof for
      `ACCESS_BACKGROUND_LOCATION` while preserving no geofence-transition and
      no physical-device claims.
- [x] Record emulator package/service scaffold proof and explicit
      no-geofence-transition state before physical-device work.
- [x] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.
- [x] Record parent-domain manual-required proof rows for missing background
      permission grant and geofence transition runtime before device/runtime work.
- [x] Record emulator local-geofence enter/exit transition rows from an
      app-owned `LocationManager` GPS listener while preserving no Android
      system geofencing, no dwell, and no physical-device claims.
- [x] Record emulator foreground-service-backed background-activity sample rows
      from app-owned `LocationManager` GPS listener proof storage while
      preserving no Android system background delivery, no physical-device, and
      no product-ready tracking claims.
- [x] Record active app-owned local geofence count against Android's documented
      100 geofences per app per device user limit while preserving no Android
      system geofencing, no dwell, and no physical-device claims.
- [x] Record Android `LocationManager.addProximityAlert` registration separately
      from app-owned local listener transitions while preserving no Android
      system delivery, no dwell, and no physical-device claims.
- [x] Split Android proximity-alert broadcast delivery counters from app-owned
      local listener transition counters so emulator proof cannot treat local
      GPS-listener enter/exit rows as Android system geofence delivery.
- [x] Cross-reference WP10 low-power, app-killed/restarted, pending-upload, and
      manual-required status-gap rows from the WP09 geofence proof while
      preserving no Android system geofencing, no dwell, and no physical-device
      background claims.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local and physical-device plans list the permission, geofence definition,
location event, transition, alert decision, screenshot, and logcat artifacts
required before any Android background/geofence claim. Background runtime and
product-complete behavior are still not claimed.

`npm run test:tracking-plan-android-emulator-proof` now fills the local
emulator background-permission, foreground-service-backed background-activity
sample, and local-geofence enter/exit layer for this workpack. The generated
`05-geofence-transition-proof.json` records package launch, foreground service
observation, declared `ACCESS_BACKGROUND_LOCATION`, emulator grant state,
Android app Settings page routing, activity backgrounding through
`input keyevent 3`, an emulator `geo fix` sample while the foreground service
keeps a `LocationManager` GPS listener active, app-owned background sample proof
storage with provider/timestamp/accuracy, and app-owned local-geofence proof
storage with `geofenceTransitionCount: 3`, `geofenceEnterCount: 2`, and
`geofenceExitCount: 1`. The final app UI can still render the manual-required
boundary while the proof storage preserves the local emulator sample/geofence
rows. This proves emulator foreground-service/backgrounded activity sample rows
and emulator `LocationManager` GPS-listener local-geofence enter/exit rows only.
The same proof writes
`test-results/tracking-plan-android-emulator-proof/25-active-geofence-limit-proof.json`
and compares the one app-owned local geofence row against Android's documented
100-geofence per-app/per-device-user limit. That is an active limit
representation artifact only, not Android system geofence delivery proof.
The same proof now preserves Android `LocationManager.addProximityAlert`
registration metadata separately as `systemProximityRegistration` in
`05-geofence-transition-proof.json`. That proves the emulator registration call
and source metadata only. The Android runtime bundle and generated proof now
also expose `systemProximityTransitionCount`,
`systemProximityEnterCount`, and `systemProximityExitCount` separately from the
app-owned local listener `geofenceTransitionCount`, `geofenceEnterCount`, and
`geofenceExitCount`. A zero system-proximity transition count keeps Android
system geofence delivery unclaimed even when app-owned emulator local listener
rows exist; a future nonzero system count must come from
`TrackingAndroidGeofenceTransitionReceiver`, not the local GPS listener. Android
system geofence delivery, dwell transition, physical-device background
behavior, authority, provider delivery, production upload worker, and
product-ready Android tracking remain unclaimed.
The same proof now embeds a `backgroundDegradedStatusProof` bridge to
`output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/17-status-gap-proof.json`.
That bridge covers low-power degraded, app-killed/restarted, pending-upload,
and manual-required status rows from WP10 only. It is not Android system
geofencing, dwell transition, physical-device background, authority, provider
delivery, production upload worker, or product-ready Android tracking proof.
The same proof now launches the Android app details
settings page for `ca.ocentra.parent.agent` and records Settings
activity/window routing as the Android 11+ background-location settings-page
flow artifact. It is not Android
system background delivery, Android system geofencing, dwell transition,
physical-device, authority, provider delivery, production upload worker, or
product-ready Android tracking proof.

`node scripts/test/tracking-android-permission-background-proof.mjs` now records
WP09 parent-domain manual-required rows for the background permission grant and
geofence transition gaps. The proof writes `02-platform-permission-proof.md`,
refreshes `05-geofence-transition-proof.json`, and keeps background permission,
background runtime, geofence runtime, physical-device behavior, authority,
notification/provider delivery, and product-ready Android tracking unclaimed.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/09-android-background-location-and-geofence-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: Android background/geofence workpack doc, tracking
      implementation checklist, owning tracking feature doc, WP09 generated
      proof artifacts, and focused Android permission/background proof results.
- [x] Validation commands and results:
      `npm run test:tracking-plan-android-emulator-proof` passed locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`,
      including `02-platform-permission-proof.md`,
      `05-geofence-transition-proof.json`, `15-manual-platform-proof.md`, and
      `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the local manual-required
      background permission/geofence transition proof; central capability row
      update remains a hub/primary-owned doc delta.
- [x] Known gaps/manual-required states: Android app settings-page routing,
      background permission grant, foreground-service-backed background sample,
      app-owned local geofence enter/exit, and active geofence-limit
      representation are locally observed. Android proximity-alert registration
      and system broadcast transition counters are separated from those local
      rows. Android system geofencing delivery remains unclaimed unless the
      system counter is nonzero; dwell transition delivery, physical-device
      proof, authority, provider delivery, notification delivery, production
      upload workers, and product-ready Android tracking remain unclaimed.
