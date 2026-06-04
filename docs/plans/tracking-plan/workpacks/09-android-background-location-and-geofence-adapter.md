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
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Prove Android 10+ background permission where claimed.
- [ ] Prove Android 11+ settings-page background permission flow.
- [ ] Prove enter, exit, and dwell transitions.
- [ ] Represent active geofence limit.
- [ ] Add battery/background degraded proof before claims.
- [x] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local and physical-device plans list the permission, geofence definition,
location event, transition, alert decision, screenshot, and logcat artifacts
required before any Android background/geofence claim. Background runtime and
product-complete behavior are still not claimed.
`node scripts/test/tracking-plan-platform-local-proof.mjs` now proves only the
Android emulator package scaffold runtime and foreground service visibility.
It does not prove Android background permission, geofence transition delivery,
Doze, killed-app, reboot, OEM background reliability, or physical-device
behavior.

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

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
