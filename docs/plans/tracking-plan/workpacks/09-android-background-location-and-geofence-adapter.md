# WP09 Android Background Location And Geofence Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP09 Android Background Location And Geofence Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `26-android-system-geofence-blocker-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Prove Android 10+ background permission where claimed.
- [ ] Prove Android 11+ settings-page background permission flow on an image
      with a resolvable Settings activity.
- [ ] Prove Android system enter, exit, and dwell transitions.
- [ ] Represent active geofence limit.
- [ ] Add battery/background degraded proof before claims.
- [ ] Record emulator package permission-grant proof for
      `ACCESS_BACKGROUND_LOCATION` while preserving no geofence-transition and
      no physical-device claims.
- [ ] Record emulator package/service scaffold proof and explicit
      no-geofence-transition state before physical-device work.
- [ ] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.
- [ ] Record parent-domain manual-required proof rows for missing background
      permission grant and geofence transition runtime before device/runtime work.
- [ ] Record emulator local-geofence enter/exit transition rows from an
      app-owned `LocationManager` GPS listener while preserving no Android
      system geofencing, no dwell, and no physical-device claims.
- [ ] Record emulator foreground-service-backed background-activity sample rows
      from app-owned `LocationManager` GPS listener proof storage while
      preserving no Android system background delivery, no physical-device, and
      no product-ready tracking claims.
- [ ] Record active app-owned local geofence count against Android's documented
      100 geofences per app per device user limit while preserving no Android
      system geofencing, no dwell, and no physical-device claims.
- [ ] Record Android `LocationManager.addProximityAlert` registration separately
      from app-owned local listener transitions while preserving no Android
      system delivery, no dwell, and no physical-device claims.
- [ ] Split Android proximity-alert broadcast delivery counters from app-owned
      local listener transition counters so emulator proof cannot treat local
      GPS-listener enter/exit rows as Android system geofence delivery.
- [ ] Add a parent-domain system-geofence blocker proof that consumes the
      emulator proof and fails if zero system proximity broadcast counters are
      treated as Android system geofence delivery.
- [ ] Classify local emulator evidence separately from the missing Android
      system runtime artifact set for system proximity broadcasts, dwell
      transitions, physical-device background geofencing, and authority-enrolled
      runtime proof.
- [ ] Cross-reference WP10 low-power, app-killed/restarted, pending-upload, and
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
Android app Settings route-attempt artifact, activity backgrounding through
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
system geofence blocker proof now writes
`26-android-system-geofence-blocker-proof.json` and
`test-results/tracking-android-system-geofence-blocker-proof/proof.json` for
that exact blocker. The blocker row now carries local evidence refs separately
from the required runtime artifact refs for system-proximity broadcast
transitions, dwell-transition observations, physical-device background geofence
results, and authority-enrolled geofence runtime proof; all runtime refs remain
missing until a real Android system/physical/authority execution artifact fills
them. Android system geofence delivery, dwell transition, physical-device
background behavior, authority, provider delivery, production upload worker,
and product-ready Android tracking remain unclaimed.
The same proof now embeds a `backgroundDegradedStatusProof` bridge to
`output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/17-status-gap-proof.json`.
That bridge covers low-power degraded, app-killed/restarted, pending-upload,
and manual-required status rows from WP10 only. It is not Android system
geofencing, dwell transition, physical-device background, authority, provider
delivery, production upload worker, or product-ready Android tracking proof.
The same proof now attempts to launch the Android app details settings page for
`ca.ocentra.parent.agent`; on the current ATD emulator image that route is
unresolvable because no Settings activity is present, so the harness records the
activity/window route-attempt artifact without claiming Android 11+ Settings
page proof. It is not Android
system background delivery, Android system geofencing, dwell transition,
physical-device, authority, provider delivery, production upload worker, or
product-ready Android tracking proof.

`node scripts/test/tracking-android-physical-device-runtime-proof.mjs` now
records Samsung S9 physical-device foreground/background permission grants,
4,177 foreground-service-backed background GPS samples, app-owned geofence
registration, and Android proximity registration metadata after debug APK push
plus on-device package-manager install. The physical proof also records
`shellLocationInjectionAvailable=false`, so route injection cannot be driven
from this Windows host; physical geofence transition/dwell counters and Android
system proximity broadcast counters remain zero/unclaimed until a real
outside/inside/dwell route or authority-backed run produces nonzero counters.

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

- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: Android background/geofence workpack doc, tracking
      implementation checklist, owning tracking feature doc, WP09 generated
      proof artifacts, and focused Android permission/background proof results.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-android-emulator-proof` passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`,
      including `02-platform-permission-proof.md`,
      `05-geofence-transition-proof.json`, `15-manual-platform-proof.md`, and
      `16-validation-commands.log`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the local manual-required
      background permission/geofence transition proof; central capability row
      update remains a hub/primary-owned doc delta.
- [ ] Known gaps/manual-required states: Android app settings-page route
      attempt, background permission grant, foreground-service-backed background sample,
      app-owned local geofence enter/exit, and active geofence-limit
      representation are locally observed. Android proximity-alert registration
      and system broadcast transition counters are separated from those local
      rows. The blocker proof also names the missing Android runtime artifact
      set for system broadcast delivery, dwell delivery, physical-device proof,
      and authority proof. Android system geofencing delivery remains unclaimed
      unless the system counter is nonzero; dwell transition delivery,
      physical geofence transition/dwell delivery, authority, provider delivery,
      notification delivery, production upload workers, and product-ready
      Android tracking remain unclaimed. Samsung S9 physical proof now observes
      permission grants, 4,177 background GPS samples, app-owned geofence
      registration, and system proximity registration metadata only.
