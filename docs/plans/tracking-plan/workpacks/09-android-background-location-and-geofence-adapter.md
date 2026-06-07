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
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Prove Android 10+ background permission where claimed.
- [ ] Prove Android 11+ settings-page background permission flow.
- [ ] Prove enter, exit, and dwell transitions.
- [ ] Represent active geofence limit.
- [ ] Add battery/background degraded proof before claims.
- [x] Record emulator package permission-grant proof for
      `ACCESS_BACKGROUND_LOCATION` while preserving no geofence-transition and
      no physical-device claims.
- [x] Record emulator package/service scaffold proof and explicit
      no-geofence-transition state before physical-device work.
- [x] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.
- [x] Record parent-domain manual-required proof rows for missing background
      permission grant and geofence transition runtime before device/runtime work.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local and physical-device plans list the permission, geofence definition,
location event, transition, alert decision, screenshot, and logcat artifacts
required before any Android background/geofence claim. Background runtime and
product-complete behavior are still not claimed.

`npm run test:tracking-plan-android-emulator-proof` now fills the local
emulator background-permission layer for this workpack. The generated
`05-geofence-transition-proof.json` records package launch, foreground service
observation, declared `ACCESS_BACKGROUND_LOCATION`, emulator grant state, app UI
text for `background-location-permission-granted`, and
`background-geofence-transition-manual-required` while keeping
`geofenceTransitionCount` at `0`. This proves emulator background permission
state only; it is not background sample collection, geofence transition,
physical-device, authority, or product-ready Android tracking proof.

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
      `node scripts/test/tracking-android-permission-background-proof.mjs`
      passed locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`,
      including `02-platform-permission-proof.md`,
      `05-geofence-transition-proof.json`, `15-manual-platform-proof.md`, and
      `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the local manual-required
      background permission/geofence transition proof; central capability row
      update remains a hub/primary-owned doc delta.
- [x] Known gaps/manual-required states: Android 11+ settings-page background
      permission flow, background sample collection, enter/exit/dwell
      transition delivery, active geofence-limit runtime, physical-device
      proof, authority, provider delivery, notification delivery, and
      product-ready Android tracking remain unclaimed.
